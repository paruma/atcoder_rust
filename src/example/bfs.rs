#![allow(dead_code)]

use crate::mylib::queue0::mod_queue::Queue;

/// init から行ける頂点を全探索する
/// 計算量: O(頂点の数 + 辺の数)
fn bfs(adj: &[Vec<usize>], init: usize) -> Vec<bool> {
    let n_vertex = adj.len();
    let mut open: Queue<usize> = Queue::new();
    let mut visited = vec![false; n_vertex];
    open.push(init);
    visited[init] = true;

    while let Some(current) = open.pop() {
        for &to in &adj[current] {
            if !visited[to] {
                visited[to] = true;
                open.push(to);
            }
        }
    }
    visited.clone()
}

// === test ===

#[cfg(test)]
mod tests {

    use crate::mylib::graph::make_adj_from_directed;

    use super::*;

    #[allow(clippy::bool_assert_comparison)]
    #[test]
    fn test_dfs() {
        // 0 ← 1
        // ↓ ↗
        // 2
        // 3 → 4
        let n_vertex = 5;
        let edges = [(0, 1), (1, 2), (2, 0), (3, 4)];
        let adj = make_adj_from_directed(n_vertex, &edges);
        let visited = bfs(&adj, 0);
        assert_eq!(visited[0], true);
        assert_eq!(visited[1], true);
        assert_eq!(visited[2], true);
        assert_eq!(visited[3], false);
        assert_eq!(visited[4], false);
    }
}
