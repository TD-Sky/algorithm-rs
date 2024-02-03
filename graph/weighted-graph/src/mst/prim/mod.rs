mod end;

use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

use self::end::End;
use crate::{WeiEdge, WeiGraph};

struct PrimMST<'a, V> {
    graph: &'a WeiGraph<V>,
    marked: BTreeSet<u32>,
    stretchs: BTreeMap<u32, &'a WeiEdge>,
    pq: BinaryHeap<End>,
}

impl<'a, V> PrimMST<'a, V> {
    fn new(graph: &'a WeiGraph<V>) -> Self {
        Self {
            graph,
            marked: BTreeSet::new(),
            stretchs: BTreeMap::new(),
            pq: BinaryHeap::new(),
        }
    }

    fn visit(&mut self, start: u32) {
        for edge in self.graph.adj_edges(start) {
            let end = edge.other(start);

            // 跳过生成树内边
            if self.marked.contains(&end) {
                continue;
            }

            // 替换成权重更小的边
            self.stretchs
                .entry(end)
                .and_modify(|this| {
                    if this.weight > edge.weight {
                        *this = edge;

                        // 记录到终点的权重
                        // 每个节点都关联唯一最小权重，失效边不会留存
                        self.pq.push(End::new(edge.weight, end));
                    }
                })
                .or_insert_with(|| {
                    self.pq.push(End::new(edge.weight, end));

                    edge
                });
        }
    }
}

pub(crate) fn span<V>(graph: &WeiGraph<V>, root: u32) -> Vec<&WeiEdge> {
    let mut mst = PrimMST::new(graph);

    mst.pq.push(End::forge(root));

    // 不断访问最近节点
    while let Some(closest_end) = mst.pq.pop() {
        let id = closest_end.id();

        mst.marked.insert(id);
        mst.visit(id);
    }

    mst.stretchs.into_values().collect()
}
