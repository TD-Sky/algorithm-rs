use std::collections::HashSet;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::ptr::NonNull;

use super::Node;

pub struct Preorder<'a, K, V> {
    stack: VecDeque<NonNull<Node<K, V>>>,
    marked: HashSet<NonNull<Node<K, V>>>,
    marker: PhantomData<(&'a K, &'a V)>,
}

impl<'a, K, V> Preorder<'a, K, V> {
    pub(crate) fn new() -> Self {
        Self {
            stack: VecDeque::new(),
            marked: HashSet::new(),
            marker: PhantomData,
        }
    }

    pub(crate) fn with_root(root: NonNull<Node<K, V>>) -> Self {
        let mut iter = Self::new();
        iter.stack.push_back(root);
        iter
    }
}

impl<'a, K, V> Iterator for Preorder<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        // 栈空即完成遍历
        self.stack.back().cloned().and_then(|node_raw| {
            let node = unsafe { node_raw.as_ref() };

            match node.children().find(|child| !self.marked.contains(child)) {
                // 仍可往下，父节点留在栈中
                Some(child) => self.stack.push_back(child),
                // 遍历到叶子，重返父节点
                None => {
                    self.stack.pop_back();
                }
            };

            match self.marked.insert(node_raw) {
                true => Some((&node.key, &node.value)), // 第一次遇到，操作
                false => self.next(),                   // 重复遇到，跳过
            }
        })
    }
}

pub struct Inorder<'a, K, V> {
    stack: VecDeque<NonNull<Node<K, V>>>,
    marked: HashSet<NonNull<Node<K, V>>>,
    marker: PhantomData<(&'a K, &'a V)>,
}

impl<'a, K, V> Inorder<'a, K, V> {
    pub(crate) fn new() -> Self {
        Self {
            stack: VecDeque::new(),
            marked: HashSet::new(),
            marker: PhantomData,
        }
    }

    pub(crate) fn with_root(root: NonNull<Node<K, V>>) -> Self {
        let mut iter = Self::new();
        iter.stack.push_back(root);
        iter
    }
}

impl<'a, K, V> Iterator for Inorder<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        // 栈空即完成遍历
        self.stack.back().cloned().and_then(|node_raw| {
            let node = unsafe { node_raw.as_ref() };

            match node.left {
                Some(left) if !self.marked.contains(&left) => {
                    self.stack.push_back(left);
                    self.next()
                }

                _ => match self.marked.insert(node_raw) {
                    true => {
                        match node.right {
                            Some(right) => self.stack.push_back(right),
                            None => {
                                self.stack.pop_back();
                            }
                        }

                        Some((&node.key, &node.value))
                    }

                    false => {
                        self.stack.pop_back();
                        self.next()
                    }
                },
            }
        })
    }
}
