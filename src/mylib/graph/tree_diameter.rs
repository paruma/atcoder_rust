use cargo_snippet::snippet;

use crate::mylib::data_structure::queue::mod_queue::Queue;

/// 木の直径を求める(直径の長さと直径を構成する頂点のリストを返す)
///
/// edges: 辺の情報 (頂点, 頂点, コスト) のリスト
///
/// # 計算量
/// O(n) (n が頂点の数のとき)
#[snippet(include = "mod_queue")]
pub fn tree_diameter(edges: &[(usize, usize, i64)]) -> (i64, Vec<usize>) {
    let nv = edges.len() + 1;
    let adj = edges
        .iter()
        .copied()
        .fold(vec![vec![]; nv], |mut acc, (u, v, cost)| {
            acc[u].push((v, cost));
            acc[v].push((u, cost));
            acc
        });
    // init から最も遠い点までの距離と、init から最も遠い点までいくのに訪問する頂点のリストを返す
    fn bfs(adj: &[Vec<(usize, i64)>], init: usize) -> (i64, Vec<usize>) {
        let n = adj.len(); // 頂点の数

        let mut dist = vec![0; n];
        let mut prev = vec![None; n];
        let mut visited = vec![false; n];
        let mut open = Queue::new();
        open.push(init);
        visited[init] = true;

        while let Some(current) = open.pop() {
            for &(next, cost) in &adj[current] {
                if !visited[next] {
                    dist[next] = dist[current] + cost;
                    prev[next] = Some(current);
                    visited[next] = true;
                    open.push(next);
                }
            }
        }
        let (furthest, max_dist) = dist
            .iter()
            .copied()
            .enumerate()
            .max_by_key(|(_, d)| *d)
            .unwrap();
        // 復元
        let path: Vec<usize> = {
            let mut path: Vec<usize> =
                std::iter::successors(Some(furthest), |&i| prev[i]).collect();
            path.reverse();
            path
        };

        (max_dist, path)
    }

    // 頂点 0 から最も遠い点 x を求める
    let x = *bfs(&adj, 0).1.last().unwrap();

    // 頂点 x から最も遠い点 y までの距離を求める
    bfs(&adj, x)
}

#[snippet(include = "tree_diameter")]
pub fn tree_diameter_no_weight(edges: &[(usize, usize)]) -> (i64, Vec<usize>) {
    let edges: Vec<(usize, usize, i64)> = edges.iter().copied().map(|(u, v)| (u, v, 1)).collect();

    tree_diameter(&edges)
}

#[cfg(test)]
mod tests {
    use super::tree_diameter;

    #[test]
    fn test_tree_diameter() {
        let edges = vec![
            (0, 1, 5),
            (1, 2, 3),
            (2, 3, 1),
            (1, 4, 2),
            (4, 7, 4),
            (1, 5, 7),
            (2, 6, 5),
        ];
        let (diam_len, _diam_path) = tree_diameter(&edges);
        assert_eq!(diam_len, 15);
        // path の方は library checker でチェックする
    }

    #[test]
    fn test_tree_diameter_no_weight() {
        use super::tree_diameter_no_weight;
        let edges = vec![(0, 1), (1, 2), (2, 3), (1, 4), (4, 5)];
        let (diam_len, _diam_path) = tree_diameter_no_weight(&edges);
        // 5-4-1-2-3 (length 4) or 3-2-1-4-5 (length 4)
        assert_eq!(diam_len, 4);
    }
}
