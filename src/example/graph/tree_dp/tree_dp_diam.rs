#![allow(dead_code)]
// 木DPで木の直径を求める

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Top2<T> {
    t1: T,
    t2: T,
}

impl<T: Ord> Top2<T> {
    fn new(t1: T, t2: T) -> Self {
        Self { t1, t2 }
    }
    fn first(self) -> T {
        self.t1
    }
    fn second(self) -> T {
        self.t2
    }

    #[must_use]
    fn inserted(self, x: T) -> Self {
        if x >= self.t1 {
            Self { t1: x, t2: self.t1 }
        } else if x >= self.t2 {
            Self { t1: self.t1, t2: x }
        } else {
            self
        }
    }
}

/// 根付き木の隣接リストから、各頂点の子頂点リストと帰りがけ順（post-order）の訪問順序を求めます。
///
/// - `adj`: 隣接リスト
/// - `root`: 根となる頂点番号
///
/// 返り値: (各頂点の子頂点リスト, 帰りがけ順の頂点番号リスト)
fn tree_children_and_order(adj: &[Vec<usize>], root: usize) -> (Vec<Vec<usize>>, Vec<usize>) {
    fn rec(
        adj: &[Vec<usize>],
        cur: usize,
        parent: usize,
        children: &mut [Vec<usize>],
        order: &mut Vec<usize>,
    ) {
        for &next in &adj[cur] {
            if next != parent {
                children[cur].push(next);
                rec(adj, next, cur, children, order);
            }
        }
        order.push(cur);
    }
    let nv = adj.len();
    let mut children = vec![vec![]; nv];
    let mut order = vec![];
    rec(adj, root, root, &mut children, &mut order);

    (children, order)
}

/// 木の直径を求める
fn tree_diam(adj: &[Vec<usize>]) -> usize {
    let (children, order) = tree_children_and_order(adj, 0);
    let nv = adj.len();

    // dp[v] = v から葉へのパスの長さの top2
    let mut dp = vec![Top2::new(0, 0); nv];

    for &cur in &order {
        dp[cur] = children[cur]
            .iter()
            .copied()
            .map(|child| dp[child])
            .fold(Top2::new(0, 0), |acc, x| acc.inserted(x.first() + 1));
    }

    dp.iter()
        .copied()
        .map(|top2| top2.first() + top2.second())
        .max()
        .unwrap()
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
        let adj = vec![
            vec![1, 2],    // 0
            vec![0, 3, 4], // 1
            vec![0],       // 2
            vec![1],       // 3
            vec![1],       // 4
        ];
        let result = tree_diam(&adj);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_tree_diam_path_six() {
        // 0 - 1 - 2 - 3 - 4 - 5
        let adj = vec![
            vec![1],
            vec![0, 2],
            vec![1, 3],
            vec![2, 4],
            vec![3, 5],
            vec![4],
        ];
        let result = tree_diam(&adj);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_tree_diam_balanced_tree() {
        //       0
        //      / \
        //     1   2
        //    / \ / \
        //   3  4 5  6
        let adj = vec![
            vec![1, 2],    // 0
            vec![0, 3, 4], // 1
            vec![0, 5, 6], // 2
            vec![1],       // 3
            vec![1],       // 4
            vec![2],       // 5
            vec![2],       // 6
        ];
        let result = tree_diam(&adj);
        assert_eq!(result, 4);
    }
}
