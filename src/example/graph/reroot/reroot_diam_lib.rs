#![allow(dead_code)]
// src/mylib/graph/reroot.rs を使って木の直径を求める

use mylib::graph::reroot::reroot::*;

fn tree_diam(adj: &[Vec<usize>]) -> u64 {
    if adj.is_empty() {
        return 0;
    }
    let result = DistMaxReroot.reroot(adj);
    result.into_iter().max().unwrap()
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
