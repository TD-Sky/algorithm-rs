use std::collections::VecDeque;
use std::collections::{BTreeMap, BTreeSet};

#[cfg(test)]
mod tests;

pub struct DiGraph<V = ()> {
    adj_table: BTreeMap<u32, Node<V>>,
}

struct Node<V> {
    element: V,
    neighbours: BTreeSet<u32>,
}

pub type Edge = (u32, u32);

impl<V> Default for DiGraph<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V> DiGraph<V> {
    fn add_node(&mut self, id: u32, elt: V) {
        if let Some(node) = self.adj_table.get_mut(&id) {
            node.element = elt; // 替换值
        } else {
            self.adj_table.insert(id, Node::new(elt)); // 建立新节点
        }
    }

    fn neighbours(&self, id: u32) -> impl Iterator<Item = &u32> {
        self.adj_table
            .get(&id)
            .map(|node| node.neighbours.iter())
            .unwrap()
    }

    fn bfs(&self, start: u32) -> BTreeMap<u32, u32> {
        let mut stretchs = BTreeMap::new();
        let mut marked = BTreeSet::new();
        let mut queue = VecDeque::new();

        marked.insert(start);
        queue.push_front(start);

        // 搜索会遍历所有节点
        while let Some(id) = queue.pop_back() {
            for &neighbour in self.neighbours(id) {
                // 若相邻点未标记，则压入队列
                if marked.insert(neighbour) {
                    // 核心功能：构建邻接边表
                    stretchs.insert(neighbour, id);
                    // 立即探索邻接点
                    queue.push_front(neighbour);
                }
            }
        }

        stretchs
    }
}

impl<V> DiGraph<V> {
    pub fn new() -> Self {
        Self {
            adj_table: BTreeMap::new(),
        }
    }

    pub fn node_count(&self) -> usize {
        self.adj_table.len()
    }

    pub fn edge_count(&self) -> usize {
        self.adj_table
            .values()
            .map(|node| node.neighbours.len())
            .sum()
    }

    pub fn add_edge(&mut self, edge: Edge, start: V, end: V) {
        self.add_node(edge.0, start);
        self.add_node(edge.1, end);

        self.adj_table.entry(edge.0).and_modify(|node| {
            node.neighbours.insert(edge.1);
        });
    }

    pub fn shortest_path(&self, start: u32, end: u32) -> Result<Vec<u32>, u32> {
        if !self.adj_table.contains_key(&start) {
            return Err(start);
        } else if !self.adj_table.contains_key(&end) {
            return Err(end);
        }

        let mut path = VecDeque::new();
        let mut id = end;
        let stretchs = self.bfs(start);

        // 在广搜得到的路径上，回溯至起点
        while id != start {
            path.push_front(id);
            id = *stretchs.get(&id).unwrap();
        }

        path.push_front(start);

        Ok(path.into())
    }
}

impl<V> Node<V> {
    fn new(element: V) -> Self {
        Self {
            element,
            neighbours: BTreeSet::new(),
        }
    }
}
