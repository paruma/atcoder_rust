#![allow(dead_code)]

use mylib::graph::graph::dfs_post_order;
use mylib::graph::tree::tree::make_tree_children_weighted;

/// 根付き木において、各頂点を根とする部分木の辺重みの総和を計算します。
///
/// - `adj`: 隣接リスト(隣の点と重み)
/// - `root`: 根となる頂点番号
///
/// 返り値: 各頂点 `i` について、その部分木の重みの総和を `dp[i]` に格納したベクタ
fn tree_dp_edge(adj: &[Vec<(usize, i64)>], root: usize) -> Vec<i64> {
    let children = make_tree_children_weighted(adj, root);
    let order = dfs_post_order(
        &adj.iter()
            .map(|v| v.iter().map(|(u, _)| *u).collect())
            .collect::<Vec<_>>(),
        root,
    );

    let nv = adj.len();

    let mut dp = vec![0; nv];
    for cur in order {
        dp[cur] = children[cur]
            .iter()
            .copied()
            .map(|(child, edge_cost)| dp[child] + edge_cost)
            .sum::<i64>();
    }
    dp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::identity_op)]
    #[test]
    fn test_tree_dp_edge() {
        // 木構造 (辺の重み):
        //     0
        //  (1)/ \(2)
        //    1   2
        // (3)/ \(4)
        //   3   4
        let n_vertex = 5;
        let edges = [(0, 1, 1), (0, 2, 2), (1, 3, 3), (1, 4, 4)];
        let adj = edges
            .iter()
            .fold(vec![vec![]; n_vertex], |mut adj, &(u, v, w)| {
                adj[u].push((v, w));
                adj[v].push((u, w));
                adj
            });
        let dp = tree_dp_edge(&adj, 0);

        assert_eq!(dp[3], 0);
        assert_eq!(dp[4], 0);
        assert_eq!(dp[1], 3 + 4);
        assert_eq!(dp[2], 0);
        assert_eq!(dp[0], (3 + 4 + 1) + (0 + 2));
    }
}
