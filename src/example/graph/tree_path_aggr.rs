// 根付き木の根からのパスの集約値を求めるサンプルコード

#![allow(dead_code)]
use mylib::graph::{graph::dfs_pre_order, tree::tree::make_tree_children};

fn tree_length_from_root(adj: &[Vec<usize>], root: usize) -> Vec<usize> {
    let nv = adj.len();
    let children = make_tree_children(adj, root);
    let ord = dfs_pre_order(adj, root);
    let mut dp = vec![usize::MAX; nv];
    dp[root] = 0;

    for cur in ord {
        for &child in &children[cur] {
            dp[child] = dp[cur] + 1;
        }
    }
    dp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_node() {
        let adj = vec![vec![]];
        let result = tree_length_from_root(&adj, 0);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_linear_tree() {
        // 0 -> 1 -> 2 -> 3
        let adj = vec![vec![1], vec![0, 2], vec![1, 3], vec![2]];
        let result = tree_length_from_root(&adj, 0);
        assert_eq!(result, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_linear_tree_from_middle() {
        // 0 -> 1 -> 2 -> 3, root from 1
        let adj = vec![vec![1], vec![0, 2], vec![1, 3], vec![2]];
        let result = tree_length_from_root(&adj, 1);
        assert_eq!(result, vec![1, 0, 1, 2]);
    }

    #[test]
    fn test_binary_tree() {
        //     0
        //    / \
        //   1   2
        //  / \
        // 3   4
        let adj = vec![
            vec![1, 2],    // 0
            vec![0, 3, 4], // 1
            vec![0],       // 2
            vec![1],       // 3
            vec![1],       // 4
        ];
        let result = tree_length_from_root(&adj, 0);
        assert_eq!(result, vec![0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_binary_tree_from_leaf() {
        //     0
        //    / \
        //   1   2
        //  / \
        // 3   4
        let adj = vec![
            vec![1, 2],    // 0
            vec![0, 3, 4], // 1
            vec![0],       // 2
            vec![1],       // 3
            vec![1],       // 4
        ];
        let result = tree_length_from_root(&adj, 3);
        assert_eq!(result, vec![2, 1, 3, 0, 2]);
    }

    #[test]
    fn test_star_tree() {
        // Center is 0, all others connected directly
        //     1
        //     |
        // 4 - 0 - 2
        //     |
        //     3
        let adj = vec![
            vec![1, 2, 3, 4], // 0
            vec![0],          // 1
            vec![0],          // 2
            vec![0],          // 3
            vec![0],          // 4
        ];
        let result = tree_length_from_root(&adj, 0);
        assert_eq!(result, vec![0, 1, 1, 1, 1]);
    }

    #[test]
    fn test_star_tree_from_leaf() {
        let adj = vec![
            vec![1, 2, 3, 4], // 0
            vec![0],          // 1
            vec![0],          // 2
            vec![0],          // 3
            vec![0],          // 4
        ];
        let result = tree_length_from_root(&adj, 1);
        assert_eq!(result, vec![1, 0, 2, 2, 2]);
    }
}
