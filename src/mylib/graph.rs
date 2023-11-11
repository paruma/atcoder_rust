use cargo_snippet::snippet;
use petgraph::unionfind::UnionFind;

use super::queue0::mod_queue::Queue;

// FIXME: 無向グラフと有向グラフがごっちゃになっている。いい感じに区別したい

#[snippet(name = "edge")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Edge {
    from: usize,
    to: usize,
}

#[snippet(name = "edge")]
impl Edge {
    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }
}

#[snippet(name = "edge")]
pub fn make_adj(n_vertex: usize, edges: &[Edge]) -> Vec<Vec<Edge>> {
    let mut adj = vec![vec![]; n_vertex];

    for &e in edges {
        adj[e.from].push(e);
    }

    adj
}
#[snippet]
#[allow(clippy::collapsible_else_if)]
pub fn is_bipartite_graph(adj: &Vec<Vec<Edge>>) -> bool {
    // 無向グラフに使うことを想定している。
    let n_vertex = adj.len();
    let mut visited = vec![false; n_vertex];
    let mut odd_even_list = vec![-1; n_vertex]; // 0 or 1 を入れる
    for init in 0..n_vertex {
        if visited[init] {
            continue;
        }
        let mut open: Queue<usize> = Queue::new();
        open.push(init);
        visited[init] = true;
        odd_even_list[init] = 0;

        while let Some(current) = open.pop() {
            for &e in &adj[current] {
                if !visited[e.to] {
                    visited[e.to] = true;
                    open.push(e.to);
                    odd_even_list[e.to] = (odd_even_list[e.from] + 1) % 2; // 1 - odd_even_list[e.from] で良かった
                } else {
                    if odd_even_list[e.from] == odd_even_list[e.to] {
                        return false;
                    }
                }
            }
        }
    }
    true
}

#[snippet]
pub fn is_bipartite_graph_by_uf(n_vertex: usize, edges: &[Edge]) -> bool {
    let mut uf = UnionFind::new(2 * n_vertex);
    for &e in edges {
        uf.union(e.from, e.to + n_vertex);
        uf.union(e.from + n_vertex, e.to);
    }
    (0..n_vertex).all(|i| !uf.equiv(i, i + n_vertex))
}

#[snippet]
pub fn has_cycle(n_vertex: usize, edges: &[Edge]) -> bool {
    let mut uf = UnionFind::new(n_vertex);
    for &e in edges {
        if uf.equiv(e.from, e.to) {
            return true;
        }
        uf.union(e.from, e.to);
    }
    false
}

mod tests {
    use super::*;

    fn edge1() -> (usize, Vec<Edge>) {
        // 0
        // ↓  ↘
        // 1 → 2
        (3, vec![Edge::new(0, 1), Edge::new(0, 2), Edge::new(1, 2)])
    }

    fn edge2() -> (usize, Vec<Edge>) {
        // 0 → 1 → 2
        (3, vec![Edge::new(0, 1), Edge::new(1, 2)])
    }

    fn edge3() -> (usize, Vec<Edge>) {
        // 0 → 1
        // ↓   ↓
        // 2 → 3
        (4, vec![Edge::new(0, 1), Edge::new(0, 2), Edge::new(1, 3), Edge::new(2, 3)])
    }

    #[test]
    fn test_make_adj() {
        {
            let (n_vertex, edges) = edge1();
            let adj = make_adj(n_vertex, &edges);
            let expected =
                vec![vec![Edge::new(0, 1), Edge::new(0, 2)], vec![Edge::new(1, 2)], vec![]];
            assert_eq!(adj, expected);
        }
        {
            let n_vertex = 3;
            let edges = [];
            let adj = make_adj(n_vertex, &edges);
            let expected = vec![vec![], vec![], vec![]];
            assert_eq!(adj, expected);
        }
        {
            let n_vertex = 0;
            let edges = [];
            let adj = make_adj(n_vertex, &edges);
            let expected: Vec<Vec<Edge>> = vec![];
            assert_eq!(adj, expected);
        }
    }

    #[test]
    fn test_is_bipartite_graph() {
        {
            let (n_vertex, edges) = edge1();
            let adj = make_adj(n_vertex, &edges);
            assert!(!is_bipartite_graph(&adj));
        }
        {
            let (n_vertex, edges) = edge2();
            let adj = make_adj(n_vertex, &edges);
            assert!(is_bipartite_graph(&adj));
        }
        {
            let (n_vertex, edges) = edge3();
            let adj = make_adj(n_vertex, &edges);
            assert!(is_bipartite_graph(&adj));
        }
    }

    #[test]
    fn test_is_bipartite_graph_uf() {
        {
            let (n_vertex, edges) = edge1();
            assert!(!is_bipartite_graph_by_uf(n_vertex, &edges));
        }
        {
            let (n_vertex, edges) = edge2();
            assert!(is_bipartite_graph_by_uf(n_vertex, &edges));
        }
        {
            let (n_vertex, edges) = edge3();
            assert!(is_bipartite_graph_by_uf(n_vertex, &edges));
        }
    }

    #[test]
    fn test_has_cycle() {
        {
            let (n_vertex, edges) = edge1();
            assert!(has_cycle(n_vertex, &edges));
        }
        {
            let (n_vertex, edges) = edge2();
            assert!(!has_cycle(n_vertex, &edges));
        }
        {
            let (n_vertex, edges) = edge3();
            assert!(has_cycle(n_vertex, &edges));
        }
    }
}
