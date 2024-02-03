#[cfg(test)]
mod tests;

mod node;

use std::borrow::Borrow;
use std::ptr::NonNull;

pub use self::node::iter;
use self::node::{Node, NodePtr};

pub struct AVLTreeMap<K, V> {
    root: NodePtr<K, V>,
    len: usize,
}

impl<K, V> Drop for AVLTreeMap<K, V> {
    fn drop(&mut self) {
        // 后序遍历销毁树
        unsafe fn postorder<K, V>(mut node: NonNull<Node<K, V>>) {
            for child in node.as_mut().children() {
                postorder(child);
            }

            drop(Box::from_raw(node.as_ptr()));
        }

        if let Some(tree) = self.root.take() {
            unsafe { postorder(tree) }
        }
    }
}

impl<K, V> Default for AVLTreeMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> AVLTreeMap<K, V> {
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }
}

impl<K, V> AVLTreeMap<K, V>
where
    K: Ord,
{
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.len += 1;

        match self.root {
            None => {
                self.root = Some(Box::leak(Node::new(key, value)).into());
                None
            }

            Some(mut root) => {
                let res = unsafe { root.as_mut().insert(key, value) };

                res.map(|val| {
                    self.len -= 1;
                    val
                })
            }
        }
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        Q: ?Sized + Ord,
        K: Borrow<Q>,
    {
        self.root.and_then(|_| {
            Node::remove_node(&mut self.root, key).map(|res| {
                self.len -= 1;
                Node::into_value(*res)
            })
        })
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: ?Sized + Ord,
        K: Borrow<Q>,
    {
        self.root
            .and_then(|root| unsafe { root.as_ref().get_node(key).map(|node| &node.value) })
    }
}

impl<K, V> AVLTreeMap<K, V> {
    pub fn preorder(&self) -> iter::Preorder<'_, K, V> {
        self.root.map_or_else(
            || iter::Preorder::new(),
            |root| iter::Preorder::with_root(root),
        )
    }

    pub fn inorder(&self) -> iter::Inorder<'_, K, V> {
        self.root.map_or_else(
            || iter::Inorder::new(),
            |root| iter::Inorder::with_root(root),
        )
    }
}
