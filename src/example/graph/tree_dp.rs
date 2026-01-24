#![allow(dead_code)]

/// 根付き木において、各頂点を根とする部分木の頂点重みの総和を計算します。
///
/// - `adj`: 隣接リスト
/// - `xs`: 各頂点の重み
/// - `root`: 根となる頂点番号
///
/// 返り値: 各頂点 `i` について、その部分木の頂点重みの総和を `dp[i]` に格納したベクタ
fn tree_dp_vertex(adj: &[Vec<usize>], xs: &[i64], root: usize) -> Vec<i64> {
    fn rec(
        adj: &[Vec<usize>],
        xs: &[i64],
        cur: usize,
        visited: &mut Vec<bool>,
        dp: &mut Vec<i64>,
    ) -> i64 {
        visited[cur] = true;
        let mut sum = 0;
        for &next in &adj[cur] {
            if !visited[next] {
                let next_sum = rec(adj, xs, next, visited, dp);
                sum += next_sum;
            }
        }
        sum += xs[cur];
        dp[cur] = sum;
        sum
    }

    let nv = adj.len();

    let mut dp = vec![0; nv];
    let mut visited = vec![false; nv];
    rec(adj, xs, root, &mut visited, &mut dp);
    dp
}

/// 根付き木において、各頂点を根とする部分木の辺重みの総和を計算します。
///
/// - `adj`: 隣接リスト(隣の点と重み)
/// - `root`: 根となる頂点番号
///
/// 返り値: 各頂点 `i` について、その部分木の重みの総和を `dp[i]` に格納したベクタ
fn tree_dp_edge(adj: &[Vec<(usize, i64)>], root: usize) -> Vec<i64> {
    fn rec(
        adj: &[Vec<(usize, i64)>],
        cur: usize,
        visited: &mut Vec<bool>,
        dp: &mut Vec<i64>,
    ) -> i64 {
        visited[cur] = true;
        let mut sum = 0;
        for &(next, w) in &adj[cur] {
            if !visited[next] {
                let next_sum = rec(adj, next, visited, dp);
                sum += next_sum + w;
            }
        }
        dp[cur] = sum;
        sum
    }

    let nv = adj.len();

    let mut dp = vec![0; nv];
    let mut visited = vec![false; nv];
    rec(adj, root, &mut visited, &mut dp);
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
