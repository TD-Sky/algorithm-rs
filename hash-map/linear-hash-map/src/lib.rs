#![feature(extract_if)]

#[cfg(test)]
mod tests;

use std::borrow::Borrow;
use std::mem;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub struct LinearHashMap<K, V> {
    base: Vec<Option<Node<K, V>>>,
    capacity: usize,
    len: usize,
}

pub struct Node<K, V> {
    key: K,
    value: V,
}

impl<K, V> Default for LinearHashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> LinearHashMap<K, V> {
    fn hash<H>(&self, k: H) -> usize
    where
        H: Hash,
    {
        let mut hasher = DefaultHasher::new();
        k.hash(&mut hasher);
        (hasher.finish() as usize) % self.capacity
    }

    fn find_none(&self, mut i: usize) -> usize {
        while self.base[i].is_some() {
            i = (i + 1) % self.capacity;
        }

        i
    }

    fn resize(&mut self, cap: usize)
    where
        K: Hash,
    {
        // 只有两种情况：乘二 或 除二
        self.capacity = cap;

        // 提取已有节点，collect 后才能跟 base 脱开关系
        let nodes: Vec<_> = self
            .base
            .extract_if(|opt| opt.is_some())
            .map(Option::unwrap)
            .collect();

        // 扩容
        self.base.resize_with(cap, || None);

        // 重新分配已有节点
        for node in nodes {
            let start = self.hash(&node.key);
            let slot = self.find_none(start);

            self.base[slot] = Some(node);
        }
    }
}

impl<K, V> LinearHashMap<K, V> {
    pub fn new() -> Self {
        let capacity = 16;
        let mut base = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            base.push(None);
        }

        Self {
            base,
            capacity,
            len: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut base = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            base.push(None);
        }

        Self {
            base,
            capacity,
            len: 0,
        }
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V>
    where
        K: Hash + Eq,
    {
        if self.len >= self.capacity / 2 {
            self.resize(self.capacity * 2);
        }

        let mut i = self.hash(&k);

        while let Some(node) = &mut self.base[i] {
            if node.key == k {
                return Some(mem::replace(&mut node.value, v));
            }

            i = (i + 1) % self.capacity;
        }

        self.len += 1;
        self.base[i] = Some(Node { key: k, value: v });

        None
    }

    pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q> + Hash,
        Q: Hash + Eq,
    {
        let mut i = self.hash(k);

        // 找到删除位
        while let Some(node) = &self.base[i] {
            if node.key.borrow().eq(k) {
                break;
            }

            i = (i + 1) % self.capacity;
        }

        self.base[i].take().map(|node| {
            // 从删除位的下一位开始，
            i = (i + 1) % self.capacity;

            // 遍历删除节点所在键簇。
            while self.base[i].is_some() {
                // 寻找空位
                let slot = self.find_none(i);

                // 无论是否找到空位，都尝试交换
                self.base.swap(i, slot);

                i = (i + 1) % self.capacity;
            }

            self.len -= 1;

            if self.len > 0 && self.len == self.capacity / 8 {
                self.resize(self.capacity / 2);
            }

            node.value
        })
    }

    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let mut i = self.hash(k);

        while let Some(node) = &self.base[i] {
            if node.key.borrow().eq(k) {
                return Some(&node.value);
            }

            i = (i + 1) % self.capacity;
        }

        None
    }

    pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let mut i = self.hash(k);

        while let Some(node) = &self.base[i] {
            if node.key.borrow().eq(k) {
                break;
            }

            i = (i + 1) % self.capacity;
        }

        self.base[i].as_mut().map(|node| &mut node.value)
    }
}
