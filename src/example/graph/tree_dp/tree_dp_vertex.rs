#![allow(dead_code)]

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

/// 根付き木において、各頂点を根とする部分木の頂点重みの総和を計算します。
///
/// - `adj`: 隣接リスト
/// - `xs`: 各頂点の重み
/// - `root`: 根となる頂点番号
///
/// 返り値: 各頂点 `i` について、その部分木の頂点重みの総和を `dp[i]` に格納したベクタ
fn tree_dp_vertex(adj: &[Vec<usize>], xs: &[i64], root: usize) -> Vec<i64> {
    let (children, order) = tree_children_and_order(adj, root);

    let nv = adj.len();

    let mut dp = vec![0; nv];
    for cur in order {
        let children_sum = children[cur]
            .iter()
            .copied()
            .map(|child| dp[child])
            .sum::<i64>();
        dp[cur] = children_sum + xs[cur];
    }
    dp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_dp_vertex() {
        // 木構造:
        //     0(10)
        //    / \
        //   1(2) 2(3)
        //  / \
        // 3(4) 4(5)
        let n_vertex = 5;
        let edges = [(0, 1), (0, 2), (1, 3), (1, 4)];
        let adj = edges
            .iter()
            .fold(vec![vec![]; n_vertex], |mut adj, &(u, v)| {
                adj[u].push(v);
                adj[v].push(u);
                adj
            });
        let xs = vec![10, 2, 3, 4, 5];
        let dp = tree_dp_vertex(&adj, &xs, 0);

        assert_eq!(dp[3], 4);
        assert_eq!(dp[4], 5);
        assert_eq!(dp[1], 2 + 4 + 5);
        assert_eq!(dp[2], 3);
        assert_eq!(dp[0], 10 + 11 + 3);
    }
}
