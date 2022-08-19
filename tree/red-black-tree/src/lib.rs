mod node;

#[cfg(test)]
mod tests;

use self::node::{Color, Node, NodePtr};
use std::{borrow::Borrow, ptr::NonNull};

pub struct RBTreeMap<K, V> {
    root: NodePtr<K, V>,
    len: usize,
}

impl<K, V> Drop for RBTreeMap<K, V> {
    fn drop(&mut self) {
        // 后序遍历销毁树
        unsafe fn postorder<K, V>(mut node: NonNull<Node<K, V>>) {
            for child in node.as_mut().children() {
                postorder(child);
            }

            Box::from_raw(node.as_ptr());
        }

        self.root.take().map(|tree| unsafe { postorder(tree) });
    }
}

impl<K, V> RBTreeMap<K, V>
where
    K: Ord,
{
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.len += 1;

        match self.root {
            None => {
                self.root = Some(Box::leak(Node::new(key, value, Color::Black)).into());
                None
            }

            Some(mut root) => {
                let old = unsafe { root.as_mut().insert(key, value) };

                unsafe {
                    root.as_mut().blacken();
                }

                old.map(|val| {
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
            let removal = Node::remove_node(&mut self.root, key);

            self.root.map(|mut root| unsafe { root.as_mut().blacken() });

            removal.map(|res| {
                self.len -= 1;
                Node::into_value(res)
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
