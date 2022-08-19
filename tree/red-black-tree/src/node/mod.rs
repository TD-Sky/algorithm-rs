use std::{array, borrow::Borrow, cmp::Ordering, marker::PhantomData, mem, ptr::NonNull};

pub(super) enum Color {
    Red,
    Black,
}

pub(super) type NodePtr<K, V> = Option<NonNull<Node<K, V>>>;

pub(super) struct Node<K, V> {
    pub(super) key: K,
    pub(super) value: V,
    color: Color,
    left: NodePtr<K, V>,
    right: NodePtr<K, V>,
}

impl Color {
    fn is_red(&self) -> bool {
        match self {
            Color::Red => true,
            Color::Black => false,
        }
    }

    fn rev(&mut self) {
        match self {
            Color::Red => *self = Color::Black,
            Color::Black => *self = Color::Red,
        };
    }

    fn replce(&mut self, src: Color) -> Self {
        mem::replace(self, src)
    }
}

macro_rules! side_insert {
    ($node: expr, $side: ident, $key: expr, $value: expr) => {
        match $node.$side {
            Some(mut $side) => unsafe { $side.as_mut().insert($key, $value) },
            None => {
                $node.$side = Some(Box::leak(Node::new($key, $value, Color::Red)).into());
                None
            }
        }
    };
}

impl<K, V> Node<K, V> {
    /* 链接颜色判定方法 */

    fn is_red(opt_node: NodePtr<K, V>) -> bool {
        opt_node.map_or(false, |node| unsafe { node.as_ref().color.is_red() })
    }

    fn red_right(&self) -> bool {
        Self::is_red(self.right)
    }

    fn red_left(&self) -> bool {
        Self::is_red(self.left)
    }

    fn red_double_left(&self) -> bool {
        self.left.map_or(false, |left| unsafe {
            left.as_ref().color.is_red() && left.as_ref().red_left()
        })
    }

    fn red_left_of_right(&self) -> bool {
        self.right
            .map_or(false, |right| unsafe { right.as_ref().red_left() })
    }

    fn red_left_of_left(&self) -> bool {
        self.left
            .map_or(false, |left| unsafe { left.as_ref().red_left() })
    }

    /* 局部变换 */

    unsafe fn rot_left(&mut self) {
        // 拔下右节点
        let right = { self.right.take().unwrap().as_mut() };

        // 中结点链接到当前节点右侧
        self.right = right.left.take();

        // 链接颜色旋转
        right.color = self.color.replce(Color::Red);

        // 交换节点指针所指堆空间的内容
        mem::swap(self, right);

        // 衔接节点
        self.left = Some(right.into());
    }

    unsafe fn rot_right(&mut self) {
        // 拔下左节点
        let left = { self.left.take().unwrap().as_mut() };

        // 中结点链接到当前节点左侧
        self.left = left.right.take();

        // 链接颜色旋转
        left.color = self.color.replce(Color::Red);

        // 交换节点指针所指堆空间的内容
        mem::swap(self, left);

        // 衔接节点
        self.right = Some(left.into());
    }

    fn flip_color(&mut self) {
        self.color.rev();
        unsafe {
            self.left.map(|mut left| left.as_mut().color.rev());
            self.right.map(|mut right| right.as_mut().color.rev());
        }
    }

    fn restruct_left(&mut self) {
        // 局部重构，使左子节点粘连
        self.flip_color();

        // 向下的树是完好的，故只可能存在红色左链接，
        // 若是如此，重构后节点会发生粘连，
        // 在2-3-4树中表现为上溢。
        if self.red_left_of_right() {
            unsafe {
                self.right.unwrap().as_mut().rot_right();
                self.rot_left();
            }
            // 消除粘连
            self.flip_color();
        }
    }

    fn restruct_right(&mut self) {
        self.flip_color();

        if self.red_left_of_left() {
            unsafe {
                self.rot_right();
            }
            self.flip_color();
        }
    }

    /* 交换键、值 */
    fn swap_successor(&mut self) {
        // 函数入口上下文：
        // - 删除键已匹配
        // - 传入节点必有右节点
        // - 右节点与传入节点交换键值后，后继结点会立刻删除

        // 寻找传入节点右子树的最小节点
        let mut successor: NonNull<Self> = self.right.unwrap();
        while let Some(left) = unsafe { successor.as_mut().left } {
            successor = left;
        }

        unsafe {
            mem::swap(&mut self.key, &mut successor.as_mut().key);
            mem::swap(&mut self.value, &mut successor.as_mut().value);
        }
    }

    fn rebalance(&mut self) {
        // 其实直接判定右红也可以
        // 但这会转换成双左情况
        if self.red_right() && !self.red_left() {
            unsafe {
                self.rot_left();
            }
        }

        if self.red_double_left() {
            unsafe {
                self.rot_right();
            }
        }

        if self.red_left() && self.red_right() {
            self.flip_color();
        }
    }
}

impl<K, V> Node<K, V>
where
    K: Ord,
{
    pub(super) fn new(key: K, value: V, color: Color) -> Box<Self> {
        Box::new(Self {
            color,
            key,
            value,
            left: None,
            right: None,
        })
    }

    pub(super) fn into_value(node: Box<Self>) -> V {
        node.value
    }

    pub(super) fn blacken(&mut self) {
        self.color = Color::Black;
    }

    pub(super) fn insert(&mut self, key: K, value: V) -> Option<V> {
        let old = match self.key.cmp(&key) {
            Ordering::Equal => Some(mem::replace(&mut self.value, value)),
            Ordering::Less => side_insert!(self, right, key, value),
            Ordering::Greater => side_insert!(self, left, key, value),
        };

        self.rebalance();

        old
    }

    pub(super) fn get_node<Q>(&self, key: &Q) -> Option<&Self>
    where
        Q: ?Sized + Ord,
        K: Borrow<Q>,
    {
        match self.key.borrow().cmp(key) {
            Ordering::Equal => Some(self),

            Ordering::Less => self
                .right
                .and_then(|right| unsafe { right.as_ref().get_node(key) }),

            Ordering::Greater => self
                .left
                .and_then(|left| unsafe { left.as_ref().get_node(key) }),
        }
    }

    pub(super) fn pop_min_node(opt_node: &mut NodePtr<K, V>) -> Box<Self> {
        // 删除节点不能破坏树的平衡，因此只要删除红节点即可。
        // 为了保证总是删除红节点，我们先进行局部重构，令
        // 当前节点 或 当前节点之左 为红。
        let node = unsafe { opt_node.unwrap().as_mut() };

        // 从指针调出左子节点的引用不违反容斥法则
        match node.left {
            None => unsafe {
                Box::from_raw(mem::replace(opt_node, node.right.take()).unwrap().as_ptr())
            },

            Some(left) => {
                // 捏红节点
                // 若节点的左和左之左都为黑，
                // 则借取节点以拼接，使子节点形成3/4-节点。
                // 若左之左为红，说明仍有更小的节点存在，可以直接往下；
                // 同时，这也说明左键在一个3-节点内，无需再借用拼接。
                if !(node.red_left() || unsafe { left.as_ref().red_left() }) {
                    // 2-3-4树左旋式局部重整
                    node.restruct_left();
                }

                let min_node = Self::pop_min_node(&mut node.left);

                node.rebalance();

                min_node
            }
        }
    }

    pub(super) fn remove_node<Q>(opt_node: &mut NodePtr<K, V>, key: &Q) -> Option<Box<Self>>
    where
        Q: ?Sized + Ord,
        K: Borrow<Q>,
    {
        opt_node.and_then(|mut node| {
            let node = unsafe { node.as_mut() };

            let removal = if node.key.borrow().gt(key) {
                if !(node.red_left() || node.red_left_of_left()) {
                    node.restruct_left();
                }
                Self::remove_node(&mut node.left, key)
            } else {
                if node.red_left() {
                    unsafe {
                        node.rot_right();
                    }
                }

                if node.key.borrow().eq(key) && node.right.is_none() {
                    return unsafe { Some(Box::from_raw(opt_node.take().unwrap().as_ptr())) };
                }

                if !(node.red_right() || node.red_left_of_right()) {
                    node.restruct_right();
                }

                if node.key.borrow().eq(key) {
                    node.swap_successor();
                    Some(Self::pop_min_node(&mut node.right))
                } else {
                    Self::remove_node(&mut node.right, key)
                }
            };

            node.rebalance();
            removal
        })
    }
}

pub(super) struct Children<'a, K, V> {
    inner: array::IntoIter<Option<NonNull<Node<K, V>>>, 2>,
    marker: PhantomData<&'a Node<K, V>>,
}

impl<K, V> Node<K, V> {
    pub(super) fn children(&self) -> Children<'_, K, V> {
        Children {
            inner: [self.left, self.right].into_iter(),
            marker: PhantomData,
        }
    }
}

impl<'a, K, V> Iterator for Children<'a, K, V> {
    type Item = NonNull<Node<K, V>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().and_then(|node_ptr| match node_ptr {
            None => self.next(),
            Some(_) => node_ptr,
        })
    }
}
