#![allow(dead_code)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    from: usize,
    to: usize,
}
#[allow(dead_code)]
// 辺のリストから隣接リストを作る
fn make_adj(n_vertex: usize, edges: &[Edge]) -> Vec<Vec<Edge>> {
    let mut adj = vec![vec![]; n_vertex];

    for &e in edges {
        adj[e.from].push(e);
    }

    adj
}

struct DfsGraph<'a> {
    adj: &'a Vec<Vec<Edge>>,
    visited: Vec<bool>,
}

impl DfsGraph<'_> {
    fn new(adj: &Vec<Vec<Edge>>) -> DfsGraph<'_> {
        // adj.len() は グラフの頂点の数
        DfsGraph { adj, visited: vec![false; adj.len()] }
    }
    fn exec(&mut self, v: usize) {
        // 行きがけ
        self.visited[v] = true;

        for &edge in &self.adj[v] {
            if !self.visited[edge.to] {
                self.exec(edge.to);
            }
        }
        // 帰りがけ
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[allow(clippy::bool_assert_comparison)]
    #[test]
    fn test_dfs_graph() {
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
        let mut dfs = DfsGraph::new(&adj);
        dfs.exec(0);
        assert_eq!(dfs.visited[0], true);
        assert_eq!(dfs.visited[1], true);
        assert_eq!(dfs.visited[2], true);
        assert_eq!(dfs.visited[3], false);
        assert_eq!(dfs.visited[4], false);
    }
}
