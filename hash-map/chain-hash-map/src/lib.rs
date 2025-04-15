#[cfg(test)]
mod tests;

use std::borrow::Borrow;
use std::collections::LinkedList;
use std::mem;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub struct ChainHashMap<K, V> {
    base: Vec<LinkedList<Node<K, V>>>,
    modulus: usize,
    len: usize,
}

struct Node<K, V> {
    key: K,
    value: V,
}

impl<K, V> Default for ChainHashMap<K, V> {
    fn default() -> Self {
        Self::new()
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
                self.base[bucket].push_back(Node { key: k, value: v });
                self.len += 1;
                None
            }
        }
    }

    pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let bucket = self.hash(k);

        self.base[bucket]
            .extract_if(|node| node.key.borrow().eq(k))
            .next()
            .map(|node| {
                self.len -= 1;
                node.value
            })
    }

    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let bucket = self.hash(k);

        self.base[bucket]
            .iter()
            .find_map(|node| node.key.borrow().eq(k).then_some(&node.value))
    }

    pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let bucket = self.hash(k);

        self.base[bucket]
            .iter_mut()
            .find_map(|node| node.key.borrow().eq(k).then_some(&mut node.value))
    }
}
