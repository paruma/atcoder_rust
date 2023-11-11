#![allow(dead_code)]

use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    from: usize,
    to: usize,
}
#[allow(dead_code)]
/// 辺のリストから隣接リストを作る
/// 計算量: O(頂点の数 + 辺の数)
fn make_adj(n_vertex: usize, edges: &[Edge]) -> Vec<Vec<Edge>> {
    let mut adj = vec![vec![]; n_vertex];

    for &e in edges {
        adj[e.from].push(e);
    }

    adj
}

/// init から行ける頂点を全探索する
/// 計算量: O(頂点の数 + 辺の数)
fn bfs(adj: &Vec<Vec<Edge>>, init: usize) -> Vec<bool> {
    let n_vertex = adj.len();
    let mut open: VecDeque<usize> = VecDeque::new();
    let mut visited = vec![false; n_vertex];
    open.push_front(init);
    visited[init] = true;

    while let Some(current) = open.pop_back() {
        for &e in &adj[current] {
            if !visited[e.to] {
                visited[e.to] = true;
                open.push_front(e.to);
            }
        }
    }
    visited.clone()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[allow(clippy::bool_assert_comparison)]
    #[test]
    fn test_dfs() {
        // 0 ← 1
        // ↓ ↗
        // 2
        // 3 → 4
        let n_vertex = 5;
        let edges = [(0, 1), (1, 2), (2, 0), (3, 4)]
            .into_iter()
            .map(|(from, to)| Edge { from, to })
            .collect_vec();
        let adj = make_adj(n_vertex, &edges);
        let visited = bfs(&adj, 0);
        assert_eq!(visited[0], true);
        assert_eq!(visited[1], true);
        assert_eq!(visited[2], true);
        assert_eq!(visited[3], false);
        assert_eq!(visited[4], false);
    }
}
