#![allow(dead_code)]

use crate::mylib::stack0::mod_stack::Stack;

struct DfsGraph<'a> {
    adj: &'a [Vec<usize>],
}

impl DfsGraph<'_> {
    fn new(adj: &[Vec<usize>]) -> DfsGraph<'_> {
        // adj.len() は グラフの頂点の数
        DfsGraph { adj }
    }

    fn exec_init(&mut self, v: usize) -> Vec<bool> {
        let n_vertex = self.adj.len();
        let mut visited = vec![false; n_vertex];
        self.exec(v, &mut visited);
        visited
    }
    /// 計算量: O(頂点の数 + 辺の数)
    fn exec(&mut self, v: usize, visited: &mut Vec<bool>) {
        // 行きがけ
        visited[v] = true;

        for &next in &self.adj[v] {
            if !visited[next] {
                self.exec(next, visited);
            }
        }
        // 帰りがけ
    }
}

fn dfs_by_stack(adj: &[Vec<usize>]) -> Vec<bool> {
    enum State {
        Pre(usize),
        Post(usize),
    }

    let nv = adj.len();
    let mut visited = vec![false; nv];
    let mut open = Stack::new();
    open.push(State::Post(0));
    open.push(State::Pre(0));
    while let Some(current) = open.pop() {
        match current {
            State::Pre(v) => {
                visited[v] = true;
                for &edge in &adj[v] {
                    if !visited[edge] {
                        open.push(State::Post(edge));
                        open.push(State::Pre(edge));
                    }
                }
            }
            State::Post(_v) => {}
        }
    }
    visited
}

#[cfg(test)]
mod tests {
    use crate::mylib::graph::make_adj_from_directed;

    use super::*;

    #[allow(clippy::bool_assert_comparison)]
    #[test]
    fn test_dfs_graph() {
        // 0 ← 1
        // ↓ ↗
        // 2
        // 3 → 4
        let n_vertex = 5;
        let edges = [(0, 1), (1, 2), (2, 0), (3, 4)];
        let adj = make_adj_from_directed(n_vertex, &edges);
        let mut dfs = DfsGraph::new(&adj);
        let visited = dfs.exec_init(0);
        assert_eq!(visited[0], true);
        assert_eq!(visited[1], true);
        assert_eq!(visited[2], true);
        assert_eq!(visited[3], false);
        assert_eq!(visited[4], false);
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test]
    fn test_dfs_by_stack() {
        // 0 ← 1
        // ↓ ↗
        // 2
        // 3 → 4
        let n_vertex = 5;
        let edges = [(0, 1), (1, 2), (2, 0), (3, 4)];
        let adj = make_adj_from_directed(n_vertex, &edges);
        let visited = dfs_by_stack(&adj);
        assert_eq!(visited[0], true);
        assert_eq!(visited[1], true);
        assert_eq!(visited[2], true);
        assert_eq!(visited[3], false);
        assert_eq!(visited[4], false);
    }
}
