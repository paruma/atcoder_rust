#![allow(dead_code)]

/// 計算量: O(頂点の数 + 辺の数)
fn dfs_graph(adj: &[Vec<usize>], v: usize) -> Vec<bool> {
    // adj.len() は グラフの頂点の数
    fn rec(adj: &[Vec<usize>], v: usize, visited: &mut Vec<bool>) {
        // 行きがけ
        visited[v] = true;

        for &next in &adj[v] {
            if !visited[next] {
                rec(adj, next, visited);
            }
        }
        // 帰りがけ
    }

    let n_vertex = adj.len();
    let mut visited = vec![false; n_vertex];
    rec(adj, v, &mut visited);
    visited
}

fn dfs_order_by_stack(adj: &[Vec<usize>], start: usize) -> (Vec<usize>, Vec<usize>) {
    enum State {
        Pre(usize),  // 行きがけ（初めて訪れる）
        Post(usize), // 帰りがけ（子の処理が終わった後）
    }

    let mut visited = vec![false; adj.len()];
    let mut stack = vec![State::Pre(start)];
    let mut pre_order = Vec::new();
    let mut post_order = Vec::new();

    while let Some(state) = stack.pop() {
        match state {
            State::Pre(current) => {
                if visited[current] {
                    continue;
                }
                visited[current] = true;
                pre_order.push(current); // 行きがけ順

                stack.push(State::Post(current)); // 帰りがけ用に後で処理する

                for &next in adj[current].iter().rev() {
                    if !visited[next] {
                        stack.push(State::Pre(next));
                    }
                }
            }
            State::Post(current) => {
                post_order.push(current); // 帰りがけ順
            }
        }
    }
    (pre_order, post_order)
}
#[cfg(test)]
mod tests {
    use mylib::graph::graph::make_adj_from_directed;

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
        let visited = dfs_graph(&adj, 0);
        assert_eq!(visited[0], true);
        assert_eq!(visited[1], true);
        assert_eq!(visited[2], true);
        assert_eq!(visited[3], false);
        assert_eq!(visited[4], false);
    }

    #[test]
    fn test_dfs_pre_order_not_tree() {
        // 0 → 1
        // ↓   ↑
        // 2 → 3

        let n_vertex = 4;
        let edges = [(0, 1), (0, 2), (2, 3), (3, 1)].map(|(from, to)| (from, to));
        let adj = make_adj_from_directed(n_vertex, &edges);
        let (pre_order, post_order) = dfs_order_by_stack(&adj, 0);
        assert_eq!(pre_order, vec![0, 1, 2, 3]);
        assert_eq!(post_order, vec![1, 3, 2, 0]);
    }
}
