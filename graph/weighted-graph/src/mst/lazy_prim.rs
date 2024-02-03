use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap},
};

use crate::{WeiEdge, WeiGraph};

struct LazyPrimMST<'a, V> {
    graph: &'a WeiGraph<V>,
    marked: BTreeSet<u32>,
    pq: BinaryHeap<Reverse<&'a WeiEdge>>,
}

impl<'a, V> LazyPrimMST<'a, V> {
    fn new(graph: &'a WeiGraph<V>) -> Self {
        Self {
            graph,
            marked: BTreeSet::new(),
            pq: BinaryHeap::new(),
        }
    }

    fn visit(&mut self, start: u32) {
        // 记录不属于生成树的边
        for edge in self.graph.adj_edges(start) {
            if !self.marked.contains(&edge.other(start)) {
                self.pq.push(Reverse(edge));
            }
        }
    }
}

pub(crate) fn span<V>(graph: &WeiGraph<V>, root: u32) -> Vec<&WeiEdge> {
    let mut mst = LazyPrimMST::new(graph);
    let mut res = Vec::with_capacity(graph.node_count() - 1);

    mst.marked.insert(root);
    mst.visit(root); // 将根节点的所有邻接边加入优先队列

    while let Some(Reverse(wei_edge)) = mst.pq.pop() {
        // 边优先队列的入队操作由标记点引发，
        // 故每次循环最多只有一个未标记点；
        // 标记最小边新端点，记录它们的邻接边；
        if mst.marked.insert(wei_edge.edge.0) {
            mst.visit(wei_edge.edge.0);
        } else if mst.marked.insert(wei_edge.edge.1) {
            mst.visit(wei_edge.edge.1);
        } else {
            // 没有新端点，说明是生成树内边，跳过
            continue;
        }

        // 将这条最小边加入生成树
        res.push(wei_edge);
    }

    res
}
