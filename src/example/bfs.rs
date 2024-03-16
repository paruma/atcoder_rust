#![allow(dead_code)]

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
    let mut open: Queue<usize> = Queue::new();
    let mut visited = vec![false; n_vertex];
    open.push(init);
    visited[init] = true;

    while let Some(current) = open.pop() {
        for &e in &adj[current] {
            if !visited[e.to] {
                visited[e.to] = true;
                open.push(e.to);
            }
        }
    }
    visited.clone()
}

// === library ===

use mod_queue::*;
pub mod mod_queue {
    use std::collections::VecDeque;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Queue<T> {
        raw: VecDeque<T>,
    }
    impl<T> Queue<T> {
        pub fn new() -> Self {
            Queue {
                raw: VecDeque::new(),
            }
        }
        pub fn push(&mut self, value: T) {
            self.raw.push_front(value)
        }
        pub fn pop(&mut self) -> Option<T> {
            self.raw.pop_back()
        }
        pub fn peek(&self) -> Option<&T> {
            self.raw.back()
        }
        pub fn is_empty(&self) -> bool {
            self.raw.is_empty()
        }
    }
    impl<T> Default for Queue<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}

// === test ===

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
