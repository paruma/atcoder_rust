#![allow(dead_code)]

use itertools::Itertools;
use petgraph::unionfind::UnionFind;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    u: usize,
    v: usize,
    cost: i64,
}

// 最小全域木を求める
fn kruskal(nv: usize, edges: &[Edge]) -> i64 {
    let mut uf = UnionFind::new(nv);
    let edges = edges
        .iter()
        .copied()
        .sorted_by_key(|e| e.cost)
        .collect_vec();

    let mut cost_sum = 0;

    for e in edges {
        if !uf.equiv(e.u, e.v) {
            uf.union(e.u, e.v);
            cost_sum += e.cost;
        }
    }

    cost_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nv = 6;
        let edges = [
            (0, 1, 1),
            (0, 2, 3),
            (1, 2, 1),
            (1, 3, 7),
            (2, 4, 1),
            (1, 4, 3),
            (3, 4, 1),
            (3, 5, 1),
            (4, 5, 6),
        ]
        .into_iter()
        .map(|(u, v, cost)| Edge { u, v, cost })
        .collect_vec();
        assert_eq!(kruskal(nv, &edges), 5);
    }
}
