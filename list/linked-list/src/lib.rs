pub mod iter;

#[cfg(test)]
mod tests;

use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;

use self::iter::*;

#[derive(Default, Eq)]
pub struct LinkedList<T> {
    head: NodePtr<T>,
    tail: NodePtr<T>,
    len: usize,
}

type NodePtr<T> = Option<NonNull<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    element: T,
    prev: NodePtr<T>,
    next: NodePtr<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        drop(LinkedList {
            head: self.head.take(),
            tail: self.tail.take(),
            len: mem::take(&mut self.len),
        });
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        if index < self.len / 2 {
            self.iter().nth(index)
        } else {
            self.iter().rev().nth(self.len - index - 1)
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let len = self.len;

        if index >= len {
            return None;
        }

        if index < self.len / 2 {
            self.iter_mut().nth(index)
        } else {
            self.iter_mut().rev().nth(len - index - 1)
        }
    }

    pub fn push_back(&mut self, elt: T) {
        let node = Node {
            element: elt,
            prev: self.tail,
            next: None,
        };
        let node_ptr = Some(Box::leak(Box::new(node)).into());

        if let Some(mut tail) = self.tail {
            unsafe {
                tail.as_mut().next = node_ptr;
            }
        } else {
            self.head = node_ptr;
        }

        self.tail = node_ptr;
        self.len += 1;
    }

    pub fn push_front(&mut self, elt: T) {
        let node = Node {
            element: elt,
            prev: None,
            next: self.head,
        };
        let node_ptr = Some(Box::leak(Box::new(node)).into());

        if let Some(mut head) = self.head {
            unsafe {
                head.as_mut().prev = node_ptr;
            }
        } else {
            self.tail = node_ptr;
        }

        self.head = node_ptr;
        self.len += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.map(|node| {
            let node = unsafe { Box::from_raw(node.as_ptr()) };
            self.tail = node.prev;

            if let Some(mut tail) = self.tail {
                unsafe {
                    tail.as_mut().next = None;
                }
            } else {
                self.head = None;
            }

            self.len -= 1;
            node.element
        })
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.map(|node| {
            let node = unsafe { Box::from_raw(node.as_ptr()) };
            self.head = node.next;

            if let Some(mut head) = self.head {
                unsafe {
                    head.as_mut().prev = None;
                }
            } else {
                self.tail = None;
            }

            self.len -= 1;
            node.element
        })
    }

    pub fn insert(&mut self, mut index: usize, elt: T) {
        assert!(index <= self.len);

        let node = Box::leak(Box::new(Node {
            element: elt,
            prev: None,
            next: None,
        }));

        let (mut curr, strategy, opposite): (_, MoveStrategy<T>, _) =
            if index == 0 || index < self.len / 2 {
                (&mut self.head, MoveStrategy::forward_next(), &mut self.tail)
            } else {
                index = self.len.saturating_sub(index + 1);
                (&mut self.tail, MoveStrategy::forward_prev(), &mut self.head)
            };

        for _ in 0..index {
            unsafe {
                curr = (strategy.forward_ref_mut)(curr.unwrap().as_mut());
            }
        }

        if let Some(mut entry) = *curr {
            *(strategy.forward_ref_mut)(node) = Some(entry);
            *(strategy.backward_ref_mut)(node) = unsafe { (strategy.backward)(entry.as_ref()) };
            *curr = Some(node.into());
            unsafe {
                *(strategy.backward_ref_mut)(entry.as_mut()) = Some(node.into());
            }
        } else {
            // insert into empty list
            *curr = Some(node.into());
            *opposite = Some(node.into());
        }

        self.len += 1;
    }

    pub fn remove(&mut self, mut index: usize) -> Option<T> {
        let (mut curr, strategy, opposite): (_, MoveStrategy<T>, _) =
            if index == 0 || index < self.len / 2 {
                (&mut self.head, MoveStrategy::forward_next(), &mut self.tail)
            } else {
                index = self.len.saturating_sub(index + 1);
                (&mut self.tail, MoveStrategy::forward_prev(), &mut self.head)
            };

        let mut i = 0;
        while let Some(mut entry) = *curr {
            if i == index {
                let forward_node = unsafe { (strategy.forward)(entry.as_ref()) };
                *curr = forward_node;
                if let Some(mut forward_node) = forward_node {
                    unsafe {
                        *(strategy.backward_ref_mut)(forward_node.as_mut()) =
                            (strategy.backward)(entry.as_ref());
                    };
                }

                if self.len == 1 {
                    *opposite = None;
                }

                self.len -= 1;
                return unsafe { Some(Box::from_raw(entry.as_ptr()).element) };
            } else {
                i += 1;
                unsafe {
                    curr = (strategy.forward_ref_mut)(entry.as_mut());
                }
            }
        }

        None
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            front: self.head,
            back: self.tail,
            marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            front: self.head,
            back: self.tail,
            marker: PhantomData,
        }
    }
}

impl<T> std::fmt::Debug for LinkedList<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        for (lhs, rhs) in self.iter().zip(other.iter()) {
            if lhs != rhs {
                return false;
            }
        }
        true
    }
}

impl<T> std::ops::Index<usize> for LinkedList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> std::ops::IndexMut<usize> for LinkedList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = LinkedList::new();
        for elt in iter {
            list.push_back(elt);
        }
        list
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            head: self.head,
            tail: self.tail,
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let iter = IntoIter {
            head: self.head,
            tail: self.tail,
        };
        for _ in iter {}
    }
}

type Neighbour<T> = fn(&Node<T>) -> NodePtr<T>;
type NeighbourRefMut<T> = fn(&mut Node<T>) -> &mut NodePtr<T>;

struct MoveStrategy<T> {
    forward: Neighbour<T>,
    forward_ref_mut: NeighbourRefMut<T>,
    backward: Neighbour<T>,
    backward_ref_mut: NeighbourRefMut<T>,
}

impl<T> MoveStrategy<T> {
    fn forward_next() -> Self {
        Self {
            forward: |node| node.next,
            forward_ref_mut: |node| &mut node.next,
            backward: |node| node.prev,
            backward_ref_mut: |node| &mut node.prev,
        }
    }

    fn forward_prev() -> Self {
        Self {
            forward: |node| node.prev,
            forward_ref_mut: |node| &mut node.prev,
            backward: |node| node.next,
            backward_ref_mut: |node| &mut node.next,
        }
    }
}
