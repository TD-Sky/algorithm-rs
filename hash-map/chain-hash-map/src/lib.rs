#![feature(drain_filter)]

#[cfg(test)]
mod tests;

use std::{
    borrow::Borrow,
    collections::hash_map::DefaultHasher,
    collections::LinkedList,
    hash::{Hash, Hasher},
    mem,
};

struct Node<K, V> {
    key: K,
    value: V,
}

pub struct ChainHashMap<K, V> {
    base: Vec<LinkedList<Node<K, V>>>,
    modulus: usize,
    len: usize,
}

impl<K, V> Node<K, V> {
    fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

impl<K, V> ChainHashMap<K, V> {
    fn hash<H>(&self, k: H) -> usize
    where
        H: Hash,
    {
        let mut hasher = DefaultHasher::new();
        k.hash(&mut hasher);
        (hasher.finish() as usize) % self.modulus
    }
}

impl<K, V> ChainHashMap<K, V> {
    pub fn new() -> Self {
        let modulus = 97;
        let mut base = Vec::with_capacity(modulus);

        for _ in 0..modulus {
            base.push(LinkedList::new());
        }

        Self {
            base,
            modulus,
            len: 0,
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        let mut base = Vec::with_capacity(cap);

        for _ in 0..cap {
            base.push(LinkedList::new());
        }

        Self {
            base,
            modulus: cap,
            len: 0,
        }
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V>
    where
        K: Hash + Eq,
    {
        let bucket = self.hash(&k);

        match self.base[bucket]
            .iter_mut()
            .find_map(|node| (node.key == k).then_some(&mut node.value))
        {
            Some(value) => Some(mem::replace(value, v)),
            None => {
                self.base[bucket].push_back(Node::new(k, v));
                self.len += 1;
                None
            }
        }
    }

    pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let bucket = self.hash(k);

        self.base[bucket]
            .drain_filter(|node| node.key.borrow().eq(k))
            .next()
            .map(|node| {
                self.len -= 1;
                node.value
            })
    }

    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let bucket = self.hash(k);

        self.base[bucket]
            .iter()
            .find_map(|node| node.key.borrow().eq(k).then_some(&node.value))
    }

    pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let bucket = self.hash(k);

        self.base[bucket]
            .iter_mut()
            .find_map(|node| node.key.borrow().eq(k).then_some(&mut node.value))
    }
}
