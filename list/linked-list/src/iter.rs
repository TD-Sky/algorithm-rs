use std::marker::PhantomData;

use crate::LinkedList;
use crate::NodePtr;

pub struct Iter<'a, T> {
    pub(super) front: NodePtr<T>,
    pub(super) back: NodePtr<T>,
    pub(super) marker: PhantomData<&'a LinkedList<T>>,
}

pub struct IterMut<'a, T> {
    pub(super) front: NodePtr<T>,
    pub(super) back: NodePtr<T>,
    pub(super) marker: PhantomData<&'a mut LinkedList<T>>,
}

pub struct IntoIter<T> {
    pub(super) head: NodePtr<T>,
    pub(super) tail: NodePtr<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            self.back.take();
            unsafe { Some(&self.front.take()?.as_ref().element) }
        } else {
            let node = unsafe { self.front?.as_ref() };
            self.front = node.next;
            Some(&node.element)
        }
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back == self.front {
            self.front.take();
            unsafe { Some(&self.back.take()?.as_ref().element) }
        } else {
            let node = unsafe { self.back?.as_ref() };
            self.back = node.prev;
            Some(&node.element)
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            self.back.take();
            unsafe { Some(&mut self.front.take()?.as_mut().element) }
        } else {
            let node = unsafe { self.front?.as_mut() };
            self.front = node.next;
            Some(&mut node.element)
        }
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back == self.front {
            self.front.take();
            unsafe { Some(&mut self.back.take()?.as_mut().element) }
        } else {
            let node = unsafe { self.back?.as_mut() };
            self.back = node.prev;
            Some(&mut node.element)
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.head == self.tail {
            self.tail.take();
            unsafe { Some(Box::from_raw(self.head.take()?.as_ptr()).element) }
        } else {
            let node = unsafe { Box::from_raw(self.head?.as_ptr()) };
            self.head = node.next;
            Some(node.element)
        }
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.tail == self.head {
            self.head.take();
            unsafe { Some(Box::from_raw(self.tail.take()?.as_ptr()).element) }
        } else {
            let node = unsafe { Box::from_raw(self.tail?.as_ptr()) };
            self.tail = node.prev;
            Some(node.element)
        }
    }
}
