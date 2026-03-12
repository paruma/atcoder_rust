#![allow(dead_code)]
// 全方位木DP の具体的な実装（DistMaxReroot を使わずに）

use mylib::data_structure::queue::mod_queue::Queue;

/// 全方位木DP を使って、各頂点から最も遠い頂点までの距離を求める
fn reroot(adj: &[Vec<usize>]) -> Vec<u64> {
    let n = adj.len();
    if n == 0 {
        return vec![];
    }
    if n == 1 {
        return vec![0];
    }

    // 1. 木の構造を整理（根を 0 とする）
    let (children, parent, bfs_order) = {
        let mut children = vec![vec![]; n];
        let mut parent = vec![None; n];
        let mut bfs_order = Vec::with_capacity(n);
        let mut queue = Queue::new();

        let mut visited = vec![false; n];
        visited[0] = true;
        queue.push(0);

        while let Some(cur) = queue.pop() {
            bfs_order.push(cur);
            for (cur_to_next, &next) in adj[cur].iter().enumerate() {
                if !visited[next] {
                    visited[next] = true;
                    let next_to_cur = adj[next]
                        .iter()
                        .position(|&back| back == cur)
                        .expect("Edge must be bidirectional");
                    children[cur].push((next, cur_to_next, next_to_cur));
                    parent[next] = Some((cur, next_to_cur, cur_to_next));
                    queue.push(next);
                }
            }
        }
        (children, parent, bfs_order)
    };

    // dp[u][i]: u から見て i 番目の隣接頂点方向にある部分木の集約値（Max距離）
    let mut dp: Vec<Vec<u64>> = adj
        .iter()
        .map(|next_list| vec![0; next_list.len()])
        .collect();

    // 2. 下向きの部分木の集約値を決定
    for &u in bfs_order.iter().rev() {
        if let Some((p, _u_to_p, p_to_u)) = parent[u] {
            let res = children[u]
                .iter()
                .map(|&(_c, u_to_c, _c_to_u)| dp[u][u_to_c] + 1)
                .fold(0u64, |acc, val| acc.max(val));
            dp[p][p_to_u] = res;
        }
    }

    // 3. 上向きの部分木の集約値を決定
    for &u in &bfs_order {
        if children[u].is_empty() {
            continue;
        }

        let edge_values: Vec<_> = dp[u].iter().map(|&x| x + 1).collect();

        let cum = CumMax::new(&edge_values);

        for &(c, u_to_c, c_to_u) in &children[u] {
            let res_without_c = cum.max_without1(u_to_c);
            dp[c][c_to_u] = res_without_c;
        }
    }

    // 4. 各頂点を根とした最終集計
    (0..n)
        .map(|u| {
            dp[u]
                .iter()
                .map(|&x| x + 1)
                .fold(0u64, |acc, val| acc.max(val))
        })
        .collect()
}

/// 累積 max を効率的に計算するための構造体
struct CumMax {
    prefix: Vec<u64>,
    suffix: Vec<u64>,
}

impl CumMax {
    fn new(xs: &[u64]) -> Self {
        let n = xs.len();
        let mut prefix = vec![0; n + 1];
        let mut suffix = vec![0; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i].max(xs[i]);
        }
        for i in (0..n).rev() {
            suffix[i] = xs[i].max(suffix[i + 1]);
        }
        Self { prefix, suffix }
    }

    /// インデックス `i` の要素を除いた全体の max を求める
    fn max_without1(&self, i: usize) -> u64 {
        self.prefix[i].max(self.suffix[i + 1])
    }
}

fn tree_diam(adj: &[Vec<usize>]) -> u64 {
    if adj.is_empty() {
        return 0;
    }
    reroot(adj).into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_diam_single_node() {
        let adj = vec![vec![]];
        let result = tree_diam(&adj);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_tree_diam_two_nodes() {
        // 0 - 1
        let adj = vec![vec![1], vec![0]];
        let result = tree_diam(&adj);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_tree_diam_linear_three() {
        // 0 - 1 - 2
        let adj = vec![vec![1], vec![0, 2], vec![1]];
        let result = tree_diam(&adj);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_tree_diam_linear_four() {
        // 0 - 1 - 2 - 3
        let adj = vec![vec![1], vec![0, 2], vec![1, 3], vec![2]];
        let result = tree_diam(&adj);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_tree_diam_star_five() {
        // Center is 0, four leaves around it
        let adj = vec![vec![1, 2, 3, 4], vec![0], vec![0], vec![0], vec![0]];
        let result = tree_diam(&adj);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_tree_diam_binary_tree() {
        //       0
        //      / \
        //     1   2
        //    / \
        //   3   4
        let adj = vec![vec![1, 2], vec![0, 3, 4], vec![0], vec![1], vec![1]];
        let result = tree_diam(&adj);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_tree_diam_balanced_tree() {
        //       0
        //      / \
        //     1   2
        //    / \ / \
        //   3  4 5  6
        let adj = vec![
            vec![1, 2],
            vec![0, 3, 4],
            vec![0, 5, 6],
            vec![1],
            vec![1],
            vec![2],
            vec![2],
        ];
        let result = tree_diam(&adj);
        assert_eq!(result, 4);
    }
}
