#![feature(let_chains)]

mod mst;

#[cfg(test)]
mod tests;

use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use self::mst::{kruskal, lazy_prim, prim};

pub struct WeiGraph<V = ()> {
    adj_table: BTreeMap<u32, Node<V>>,
}

struct Node<V> {
    element: V,
    adj_edges: HashSet<Rc<WeiEdge>>,
}

#[derive(Debug, Eq)]
pub struct WeiEdge {
    weight: i32,
    edge: Edge,
}

pub type Edge = (u32, u32);

impl Hash for WeiEdge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.weight.to_ne_bytes());
    }
}

impl PartialEq for WeiEdge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl PartialOrd for WeiEdge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for WeiEdge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl<V> Node<V> {
    fn new(element: V) -> Self {
        Self {
            element,
            adj_edges: HashSet::new(),
        }
    }
}

impl WeiEdge {
    fn other(&self, id: u32) -> u32 {
        self.edge.0 + self.edge.1 - id
    }
}

impl<V> Default for WeiGraph<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V> WeiGraph<V> {
    fn add_node(&mut self, id: u32, elt: V) {
        if let Some(node) = self.adj_table.get_mut(&id) {
            node.element = elt;
        } else {
            self.adj_table.insert(id, Node::new(elt));
        }
    }

    fn adj_edges(&self, id: u32) -> impl Iterator<Item = &WeiEdge> {
        self.adj_table
            .get(&id)
            .map(|node| node.adj_edges.iter().map(|rc| rc.as_ref()))
            .unwrap()
    }
}

impl<V> WeiGraph<V> {
    pub fn new() -> Self {
        Self {
            adj_table: BTreeMap::new(),
        }
    }

    pub fn node_count(&self) -> usize {
        self.adj_table.len()
    }

    pub fn add_edge(&mut self, weight: i32, edge: Edge, start: V, end: V) {
        self.add_node(edge.0, start);
        self.add_node(edge.1, end);

        let wei_edge = Rc::new(WeiEdge { weight, edge });

        self.adj_table.entry(edge.0).and_modify(|node| {
            node.adj_edges.insert(Rc::clone(&wei_edge));
        });

        self.adj_table.entry(edge.1).and_modify(|node| {
            node.adj_edges.insert(Rc::clone(&wei_edge));
        });
    }

    pub fn ids(&self) -> impl Iterator<Item = u32> + '_ {
        self.adj_table.keys().copied()
    }

    pub fn edges(&self) -> impl Iterator<Item = &WeiEdge> {
        let mut edges = HashSet::new();

        for node in self.adj_table.values() {
            for edge in &node.adj_edges {
                edges.insert(edge.as_ref());
            }
        }

        edges.into_iter()
    }
}

impl<V> WeiGraph<V> {
    pub fn lazy_prim_mst(&self, root: u32) -> Result<Vec<&'_ WeiEdge>, u32> {
        self.adj_table
            .contains_key(&root)
            .then(|| lazy_prim::span(self, root))
            .ok_or(root)
    }

    pub fn prim_mst(&self, root: u32) -> Result<Vec<&'_ WeiEdge>, u32> {
        self.adj_table
            .contains_key(&root)
            .then(|| prim::span(self, root))
            .ok_or(root)
    }

    pub fn kruskal_mst(&self) -> Option<Vec<&'_ WeiEdge>> {
        (!self.adj_table.is_empty()).then(|| kruskal::span(self))
    }
}
