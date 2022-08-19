pub mod iter;

use std::array;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;

pub(super) type NodePtr<K, V> = Option<NonNull<Node<K, V>>>;

pub(super) struct Node<K, V> {
    pub(super) key: K,
    pub(super) value: V,
    pub(super) height: isize,
    left: NodePtr<K, V>,
    right: NodePtr<K, V>,
}

macro_rules! rotate {
    ($node: expr, $side: ident, $opposite: ident) => {
        // 拔下正侧节点
        let $side = unsafe { $node.$side.take().unwrap().as_mut() };

        // 中结点链接到当前节点邻侧
        $node.$side = $side.$opposite.take();

        // 更新一下当前节点的高
        $node.update_height();

        // 交换节点指针所指堆空间的内容
        mem::swap($node, $side);

        // 衔接节点
        $node.$opposite = Some($side.into());

        // 更新原正侧节点、现父节点的高
        $node.update_height();
    };
}

macro_rules! side_insert {
    ($node: expr, $side: ident, $key: expr, $value: expr) => {
        match $node.$side {
            Some(mut $side) => unsafe { $side.as_mut().insert($key, $value) },
            None => {
                $node.$side = Some(Box::leak(Node::new($key, $value)).into());
                None
            }
        }
    };
}

impl<K, V> Node<K, V> {
    fn update_height(&mut self) {
        self.height = self
            .children()
            .map(|child| unsafe { child.as_ref().height })
            .fold(-1, Ord::max)
            + 1
    }

    // 平衡因子
    fn bal_fct(&self) -> isize {
        unsafe {
            self.left.map_or(-1, |left| left.as_ref().height)
                - self.right.map_or(-1, |right| right.as_ref().height)
        }
    }

    fn rebalance(&mut self) {
        if self.bal_fct() > 1 {
            if unsafe { self.left.unwrap().as_ref().bal_fct() } >= 0 {
                self.rot_right();
            } else {
                self.rot_left_right();
            }
        } else if self.bal_fct() < -1 {
            if unsafe { self.right.unwrap().as_ref().bal_fct() } <= 0 {
                self.rot_left();
            } else {
                self.rot_right_left();
            }
        }

        self.update_height();
    }

    fn rot_right(&mut self) {
        /*         / ->  /\
         *        /
         */

        rotate!(self, left, right);
    }

    fn rot_left(&mut self) {
        /*         \  ->  /\
         *          \
         */

        rotate!(self, right, left);
    }

    fn rot_left_right(&mut self) {
        /*         / ->   /  -> /\
         *         \     /
         */

        unsafe {
            self.left.unwrap().as_mut().rot_left();
        }
        self.rot_right();
    }

    fn rot_right_left(&mut self) {
        /*         \  ->  \   -> /\
         *         /       \
         */

        unsafe {
            self.right.unwrap().as_mut().rot_right();
        }
        self.rot_left();
    }

    fn swap_successor(&mut self) {
        // 函数入口上下文：
        // - 删除键已匹配
        // - 传入节点必有右节点
        // - 右节点与传入节点交换键值后，后继结点会立刻删除

        // 寻找传入节点右子树的最小节点
        let mut successor: &mut Self = unsafe { self.right.unwrap().as_mut() };
        while let Some(mut left) = successor.left {
            successor = unsafe { left.as_mut() };
        }

        // 交换键、值
        mem::swap(&mut self.key, &mut successor.key);
        mem::swap(&mut self.value, &mut successor.value);
    }
}

impl<K, V> Node<K, V>
where
    K: Ord,
{
    pub(super) fn new(key: K, value: V) -> Box<Self> {
        Box::new(Self {
            key,
            value,
            height: 0,
            left: None,
            right: None,
        })
    }

    pub(super) fn into_value(node: Box<Self>) -> V {
        node.value
    }

    pub(super) fn insert(&mut self, key: K, value: V) -> Option<V> {
        let res = match self.key.cmp(&key) {
            Ordering::Equal => Some(mem::replace(&mut self.value, value)),
            Ordering::Less => side_insert!(self, right, key, value),
            Ordering::Greater => side_insert!(self, left, key, value),
        };

        self.rebalance();

        res
    }

    pub(super) fn pop_min_node(opt_node: &mut NodePtr<K, V>) -> Box<Self> {
        let node = unsafe { opt_node.unwrap().as_mut() };
        // 节点 只含有右子节点 或 为叶子节点;
        // 无论哪种，都可以尝试拔下右子节点，接到当前节点位
        match node.left {
            None => unsafe {
                Box::from_raw(mem::replace(opt_node, node.right.take()).unwrap().as_ptr())
            },

            Some(_) => {
                let removal = Self::pop_min_node(&mut node.left);

                node.rebalance();

                removal
            }
        }
    }

    pub(super) fn remove_node<Q>(opt_node: &mut NodePtr<K, V>, key: &Q) -> Option<Box<Self>>
    where
        Q: ?Sized + Ord,
        K: Borrow<Q>,
    {
        // 一直不相等会导致比无可比，即查找未命中
        opt_node.and_then(|mut node| {
            let node = unsafe { node.as_mut() };
            let removal = match node.key.borrow().cmp(key) {
                Ordering::Less => Self::remove_node(&mut node.right, key),

                Ordering::Greater => Self::remove_node(&mut node.left, key),

                // 查找命中, 含右子节点
                Ordering::Equal if node.right.is_some() => {
                    node.swap_successor();
                    Some(Self::pop_min_node(&mut node.right))
                }

                // 查找命中, 只含左子节点 或 为叶子节点;
                // 必有以上两种情况，但无论哪种，
                // 总体平衡性保证，可能出现的子节点必为叶子，
                // 所以直接返回，不用更新高度;
                // 若节点本身为叶子，则替换等于拔叶子，遑论更新，哈哈
                _ => {
                    return mem::replace(opt_node, node.left.take())
                        .map(|res| unsafe { Box::from_raw(res.as_ptr()) });
                }
            };

            node.rebalance();
            removal
        })
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
