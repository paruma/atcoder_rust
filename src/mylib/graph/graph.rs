use cargo_snippet::snippet;

use super::super::data_structure::queue::mod_queue::Queue;

#[snippet]
/// 有向グラフの辺集合から隣接リストを作成します。
///
/// # 計算量
/// O(V + E)
pub fn make_adj_from_directed(n_vertex: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n_vertex];

    for &(from, to) in edges {
        adj[from].push(to);
    }

    adj
}

#[snippet]
/// 無向グラフの辺集合から隣接リストを作成します。
///
/// # 計算量
/// O(V + E)
pub fn make_adj_from_undirected(n_vertex: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n_vertex];

    for &(from, to) in edges {
        adj[from].push(to);
        adj[to].push(from);
    }

    adj
}

#[snippet]
/// 重み付き有向グラフの辺集合から隣接リストを作成します。
///
/// # 計算量
/// O(V + E)
pub fn make_adj_from_weighted_directed<T>(
    n_vertex: usize,
    edges: &[(usize, usize, T)],
) -> Vec<Vec<(usize, T)>>
where
    T: Clone,
{
    let mut adj = vec![vec![]; n_vertex];

    for (from, to, weight) in edges {
        adj[*from].push((*to, weight.clone()));
    }

    adj
}

#[snippet]
/// 重み付き無向グラフの辺集合から隣接リストを作成します。
///
/// # 計算量
/// O(V + E)
pub fn make_adj_from_weighted_undirected<T>(
    n_vertex: usize,
    edges: &[(usize, usize, T)],
) -> Vec<Vec<(usize, T)>>
where
    T: Clone,
{
    let mut adj = vec![vec![]; n_vertex];

    for (from, to, weight) in edges {
        adj[*from].push((*to, weight.clone()));
        adj[*to].push((*from, weight.clone()));
    }

    adj
}

#[snippet]
/// 根付き木の隣接リスト `adj` と根 `root` から、各頂点の親頂点を求めます。
///
/// # 計算量
/// O(V + E)
pub fn make_tree_parent(adj: &[Vec<usize>], root: usize) -> Vec<Option<usize>> {
    let n = adj.len();
    let mut parent = vec![None; n];
    let mut visited = vec![false; n];
    let mut queue = std::collections::VecDeque::new();

    visited[root] = true;
    queue.push_back(root);

    while let Some(v) = queue.pop_front() {
        for &u in &adj[v] {
            if !visited[u] {
                visited[u] = true;
                parent[u] = Some(v);
                queue.push_back(u);
            }
        }
    }

    parent
}

#[snippet]
/// 根付き木の隣接リスト `adj` と根 `root` から、各頂点の子頂点リストを求めます。
///
/// # 計算量
/// O(V + E)
pub fn make_tree_children(adj: &[Vec<usize>], root: usize) -> Vec<Vec<usize>> {
    let n = adj.len();
    let mut children = vec![vec![]; n];
    let mut visited = vec![false; n];
    let mut queue = std::collections::VecDeque::new();

    visited[root] = true;
    queue.push_back(root);

    while let Some(v) = queue.pop_front() {
        for &u in &adj[v] {
            if !visited[u] {
                visited[u] = true;
                children[v].push(u);
                queue.push_back(u);
            }
        }
    }

    children
}

#[snippet(include = "mod_queue")]
/// 幅優先探索 (BFS) を行い、頂点の訪問順序を返します。
///
/// # 計算量
/// O(V + E)
pub fn bfs_order(adj: &[Vec<usize>], init: usize) -> Vec<usize> {
    let nv = adj.len();
    let mut order = vec![];
    let mut visited = vec![false; nv];
    let mut open = Queue::new();
    open.push(init);
    order.push(init);
    visited[init] = true;
    while let Some(current) = open.pop() {
        for &next in &adj[current] {
            if !visited[next] {
                order.push(next);
                visited[next] = true;
                open.push(next);
            }
        }
    }
    order
}

#[snippet(include = "mod_stack")]
/// 深さ優先探索 (DFS) を行い、行きがけ順 (pre-order) での頂点順序を返します。
///
/// # 計算量
/// O(V + E)
pub fn dfs_pre_order(adj: &[Vec<usize>], init: usize) -> Vec<usize> {
    fn dfs(
        adj: &[Vec<usize>],
        current: usize,
        visited: &mut Vec<bool>,
        pre_order: &mut Vec<usize>,
    ) {
        // 行きがけ
        visited[current] = true;
        pre_order.push(current);

        for &next in &adj[current] {
            if !visited[next] {
                dfs(adj, next, visited, pre_order);
            }
        }
    }
    let nv = adj.len();
    let mut visited = vec![false; nv];
    let mut pre_order = vec![];
    dfs(adj, init, &mut visited, &mut pre_order);

    pre_order
}

#[snippet(include = "mod_stack")]
/// 深さ優先探索 (DFS) を行い、帰りがけ順 (post-order) での頂点順序を返します。
///
/// # 計算量
/// O(V + E)
pub fn dfs_post_order(adj: &[Vec<usize>], init: usize) -> Vec<usize> {
    fn dfs(
        adj: &[Vec<usize>],
        current: usize,
        visited: &mut Vec<bool>,
        post_order: &mut Vec<usize>,
    ) {
        // 行きがけ
        visited[current] = true;

        for &next in &adj[current] {
            if !visited[next] {
                dfs(adj, next, visited, post_order);
            }
        }
        // 帰りがけ
        post_order.push(current);
    }
    let nv = adj.len();
    let mut visited = vec![false; nv];
    let mut post_order = vec![];
    dfs(adj, init, &mut visited, &mut post_order);

    post_order
}

#[snippet(include = "mod_queue")]
/// 指定した始点から各頂点への最短距離（枝数）を求めます。
///
/// # 計算量
/// O(V + E)
pub fn calc_dist(adj: &[Vec<usize>], init: usize) -> Vec<usize> {
    let nv = adj.len();
    let mut visited = vec![false; nv];
    let mut dist = vec![usize::MAX; nv];
    let mut open = Queue::new();
    visited[init] = true;
    dist[init] = 0;
    open.push(init);
    while let Some(current) = open.pop() {
        for &next in &adj[current] {
            if !visited[next] {
                visited[next] = true;
                dist[next] = dist[current] + 1;
                open.push(next);
            }
        }
    }
    dist
}

#[snippet(include = "mod_queue")]
#[allow(clippy::collapsible_else_if)]
/// グラフが二部グラフであるかを判定します。
///
/// # 計算量
/// O(V + E)
pub fn is_bipartite_graph(adj: &[Vec<usize>]) -> bool {
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
            for &next in &adj[current] {
                if !visited[next] {
                    visited[next] = true;
                    open.push(next);
                    odd_even_list[next] = 1 - odd_even_list[current];
                } else {
                    // 偶奇チェックをする
                    if odd_even_list[current] == odd_even_list[next] {
                        return false;
                    }
                }
            }
        }
    }
    true
}

#[snippet]
/// Union-Find を用いて、グラフが二部グラフであるかを判定します。
///
/// # 計算量
/// O(E α(V))
pub fn is_bipartite_graph_by_uf(n_vertex: usize, edges: &[(usize, usize)]) -> bool {
    use petgraph::unionfind::UnionFind;
    let mut uf = UnionFind::new(2 * n_vertex);
    for &(from, to) in edges {
        uf.union(from, to + n_vertex);
        uf.union(from + n_vertex, to);
    }
    (0..n_vertex).all(|i| !uf.equiv(i, i + n_vertex))
}

#[snippet]
/// 無向グラフに閉路が含まれるかを判定します。
///
/// # 計算量
/// O(E α(V))
pub fn has_cycle_undirected(n_vertex: usize, edges: &[(usize, usize)]) -> bool {
    use petgraph::unionfind::UnionFind;
    let mut uf = UnionFind::new(n_vertex);
    for &(from, to) in edges {
        if uf.equiv(from, to) {
            return true;
        }
        uf.union(from, to);
    }
    false
}

/// 有向グラフに閉路が含まれるかを DFS を用いて判定します。
///
/// # 計算量
/// O(V + E)
pub fn has_cycle_directed(adj: &[Vec<usize>]) -> bool {
    // DFS を使って有向グラフの閉路判定をする
    fn dfs(
        adj: &[Vec<usize>],
        current: usize,
        visited_pre: &mut Vec<bool>,
        visited_post: &mut Vec<bool>,
    ) -> bool {
        // 行きがけ
        visited_pre[current] = true;

        for &next in &adj[current] {
            if visited_pre[next] && !visited_post[next] {
                // next が行きがけで訪問済だが帰りがけで未訪問: next から current に到達可能(閉路がある)
                // 逆に next が帰りがけで訪問済の場合は、next から current に到達不可能
                return true;
            }
            if visited_pre[next] {
                continue;
            }
            let has_cycle = dfs(adj, next, visited_pre, visited_post);
            if has_cycle {
                return true;
            }
        }
        // 帰りがけ
        visited_post[current] = true;
        false
    }
    let nv = adj.len();
    let mut visited_pre = vec![false; nv];
    let mut visited_post = vec![false; nv];
    for start in 0..nv {
        if visited_pre[start] {
            continue;
        }
        let has_cycle = dfs(adj, start, &mut visited_pre, &mut visited_post);
        if has_cycle {
            return true;
        }
    }
    false
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_imports)]
    use itertools::Itertools;

    #[allow(dead_code)]
    fn sample_edges1() -> (usize, Vec<(usize, usize)>) {
        // 0
        // ↓  ↘
        // 1 → 2
        (3, vec![(0, 1), (0, 2), (1, 2)])
    }
    #[allow(dead_code)]
    fn sample_edge2() -> (usize, Vec<(usize, usize)>) {
        // 0 → 1 → 2
        (3, vec![(0, 1), (1, 2)])
    }
    #[allow(dead_code)]
    fn sample_edges3() -> (usize, Vec<(usize, usize)>) {
        // 0 → 1
        // ↓   ↓
        // 2 → 3
        (4, vec![(0, 1), (0, 2), (1, 3), (2, 3)])
    }

    #[test]
    fn test_make_adj_from_directed() {
        {
            // 0
            // ↓  ↘
            // 1 → 2
            let n_vertex = 3;
            let edges = [(0, 1), (0, 2), (1, 2)];
            let adj = make_adj_from_directed(n_vertex, &edges);
            let expected = vec![vec![1, 2], vec![2], vec![]];
            assert_eq!(adj, expected);
        }
        {
            let n_vertex = 3;
            let edges = [];
            let adj = make_adj_from_directed(n_vertex, &edges);
            let expected = vec![vec![], vec![], vec![]];
            assert_eq!(adj, expected);
        }
        {
            let n_vertex = 0;
            let edges = [];
            let adj = make_adj_from_directed(n_vertex, &edges);
            let expected: Vec<Vec<usize>> = vec![];
            assert_eq!(adj, expected);
        }
    }

    #[test]
    fn test_make_adj_from_undirected() {
        {
            // 0
            // | ＼
            // 1 - 2
            let n_vertex = 3;
            let edges = [(0, 1), (0, 2), (1, 2)];
            let adj = make_adj_from_undirected(n_vertex, &edges);
            let expected = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
            assert_eq!(adj, expected);
        }
        {
            let n_vertex = 3;
            let edges = [];
            let adj = make_adj_from_undirected(n_vertex, &edges);
            let expected = vec![vec![], vec![], vec![]];
            assert_eq!(adj, expected);
        }
        {
            let n_vertex = 0;
            let edges = [];
            let adj = make_adj_from_undirected(n_vertex, &edges);
            let expected: Vec<Vec<usize>> = vec![];
            assert_eq!(adj, expected);
        }
    }

    #[test]
    fn test_make_adj_from_weighted_directed() {
        {
            // 0
            // ↓  ↘
            // 1 → 2
            let n_vertex = 3;
            let edges = [(0, 1, 100), (0, 2, 200), (1, 2, 300)];
            let adj = make_adj_from_weighted_directed(n_vertex, &edges);
            let expected = vec![vec![(1, 100), (2, 200)], vec![(2, 300)], vec![]];
            assert_eq!(adj, expected);
        }
        {
            let n_vertex = 3;
            let edges: [(usize, usize, i64); 0] = [];
            let adj: Vec<Vec<(usize, i64)>> = make_adj_from_weighted_directed(n_vertex, &edges);
            let expected: Vec<Vec<(usize, i64)>> = vec![vec![], vec![], vec![]];
            assert_eq!(adj, expected);
        }
        {
            let n_vertex = 0;
            let edges: [(usize, usize, i64); 0] = [];
            let adj: Vec<Vec<(usize, i64)>> = make_adj_from_weighted_directed(n_vertex, &edges);
            let expected: Vec<Vec<(usize, i64)>> = vec![];
            assert_eq!(adj, expected);
        }
    }

    #[test]
    fn test_make_adj_from_weighted_undirected() {
        {
            // 0
            // | ＼
            // 1 - 2
            let n_vertex = 3;
            let edges = [(0, 1, 100), (0, 2, 200), (1, 2, 300)];
            let adj = make_adj_from_weighted_undirected(n_vertex, &edges);
            let expected = vec![
                vec![(1, 100), (2, 200)],
                vec![(0, 100), (2, 300)],
                vec![(0, 200), (1, 300)],
            ];
            assert_eq!(adj, expected);
        }
        {
            let n_vertex = 3;
            let edges: [(usize, usize, i64); 0] = [];
            let adj = make_adj_from_weighted_undirected(n_vertex, &edges);
            let expected = vec![vec![], vec![], vec![]];
            assert_eq!(adj, expected);
        }
        {
            let n_vertex = 0;
            let edges: [(usize, usize, i64); 0] = [];
            let adj: Vec<Vec<(usize, i64)>> = make_adj_from_weighted_undirected(n_vertex, &edges);
            let expected: Vec<Vec<(usize, i64)>> = vec![];
            assert_eq!(adj, expected);
        }
    }

    #[test]
    fn test_make_tree_parent() {
        // 木構造:
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
        let root = 0;
        let expected = vec![
            None,    // 0
            Some(0), // 1
            Some(0), // 2
            Some(1), // 3
            Some(1), // 4
        ];
        let result = make_tree_parent(&adj, root);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_make_tree_children() {
        // 木構造:
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
        let root = 0;
        let expected = vec![
            vec![1, 2], // 0
            vec![3, 4], // 1
            vec![],     // 2
            vec![],     // 3
            vec![],     // 4
        ];
        let result = make_tree_children(&adj, root);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bfs_order() {
        // グラフでテストしているが、グラフでなくても良い（余裕があったら一般のグラフでテストする）
        // 0 → 1
        // ↓
        // 2 → 3 → 4
        // ↓   ↓
        // 5   6 → 7

        let n_vertex = 8;
        let edges =
            [(0, 1), (0, 2), (2, 3), (2, 5), (3, 4), (3, 6), (6, 7)].map(|(from, to)| (from, to));
        let adj = make_adj_from_directed(n_vertex, &edges);
        let order = bfs_order(&adj, 0);
        assert_eq!(order, vec![0, 1, 2, 3, 5, 4, 6, 7]); // FIXME: 実装依存になっていてよくない
    }

    #[test]
    fn test_dfs_pre_order() {
        // 0 → 1
        // ↓
        // 2 → 3 → 4
        // ↓   ↓
        // 5   6 → 7

        let n_vertex = 8;
        let edges =
            [(0, 1), (0, 2), (2, 3), (2, 5), (3, 4), (3, 6), (6, 7)].map(|(from, to)| (from, to));
        let adj = make_adj_from_directed(n_vertex, &edges);
        let order = dfs_pre_order(&adj, 0);
        assert_eq!(order, vec![0, 1, 2, 3, 4, 6, 7, 5]); // FIXME: 実装依存になっていてよくない
    }

    #[test]
    fn test_dfs_post_order() {
        // 0 → 1
        // ↓
        // 2 → 3 → 4
        // ↓   ↓
        // 5   6 → 7

        let n_vertex = 8;
        let edges =
            [(0, 1), (0, 2), (2, 3), (2, 5), (3, 4), (3, 6), (6, 7)].map(|(from, to)| (from, to));
        let adj = make_adj_from_directed(n_vertex, &edges);
        let order = dfs_post_order(&adj, 0);
        assert_eq!(order, vec![1, 4, 7, 6, 3, 5, 2, 0]); // FIXME: 実装依存になっていてよくない
    }

    #[test]
    fn test_dfs_pre_order_not_tree() {
        // 0 → 1
        // ↓   ↑
        // 2 → 3

        let n_vertex = 4;
        let edges = [(0, 2), (0, 1), (2, 3), (3, 1)].map(|(from, to)| (from, to));
        let adj = make_adj_from_directed(n_vertex, &edges);
        let order = dfs_pre_order(&adj, 0);
        assert_eq!(order, vec![0, 2, 3, 1]); // FIXME: 実装依存になっていてよくない
    }

    #[test]
    fn test_dfs_post_order_not_tree() {
        // 0 → 1
        // ↓   ↑
        // 2 → 3

        let n_vertex = 4;
        let edges = [(0, 2), (0, 1), (2, 3), (3, 1)].map(|(from, to)| (from, to));
        let adj = make_adj_from_directed(n_vertex, &edges);
        let order = dfs_post_order(&adj, 0);
        assert_eq!(order, vec![1, 3, 2, 0]); // FIXME: 実装依存になっていてよくない
    }

    #[test]
    fn test_calc_dist() {
        // 0 → 1 → 2 → 3
        // ↓           ↓
        // 4 ----------5
        let n_vertex = 6;

        let edges = vec![(0, 1), (0, 4), (1, 2), (2, 3), (3, 5), (4, 5)];
        let adj = make_adj_from_directed(n_vertex, &edges);
        let dist = calc_dist(&adj, 0);
        assert_eq!(dist, vec![0, 1, 2, 3, 1, 2]);

        let dist = calc_dist(&adj, 1);
        let inf = usize::MAX;
        assert_eq!(dist, vec![inf, 0, 1, 2, inf, 3]);
    }

    #[test]
    fn test_is_bipartite_graph() {
        {
            let (n_vertex, edges) = sample_edges1();
            let adj = make_adj_from_directed(n_vertex, &edges);
            assert!(!is_bipartite_graph(&adj));
        }
        {
            let (n_vertex, edges) = sample_edge2();
            let adj = make_adj_from_directed(n_vertex, &edges);
            assert!(is_bipartite_graph(&adj));
        }
        {
            let (n_vertex, edges) = sample_edges3();
            let adj = make_adj_from_directed(n_vertex, &edges);
            assert!(is_bipartite_graph(&adj));
        }
    }

    #[test]
    fn test_is_bipartite_graph_uf() {
        {
            let (n_vertex, edges) = sample_edges1();
            assert!(!is_bipartite_graph_by_uf(n_vertex, &edges));
        }
        {
            let (n_vertex, edges) = sample_edge2();
            assert!(is_bipartite_graph_by_uf(n_vertex, &edges));
        }
        {
            let (n_vertex, edges) = sample_edges3();
            assert!(is_bipartite_graph_by_uf(n_vertex, &edges));
        }
    }
    #[test]
    fn test_has_cycle_undirected() {
        {
            let (n_vertex, edges) = sample_edges1();

            assert!(has_cycle_undirected(n_vertex, &edges));
        }
        {
            let (n_vertex, edges) = sample_edge2();
            assert!(!has_cycle_undirected(n_vertex, &edges));
        }
        {
            let (n_vertex, edges) = sample_edges3();
            assert!(has_cycle_undirected(n_vertex, &edges));
        }
    }
    #[test]
    fn test_has_cycle_directed() {
        {
            let (n_vertex, edges) = sample_edges1();
            let adj = make_adj_from_directed(n_vertex, &edges);
            assert!(!has_cycle_directed(&adj));
        }
        {
            let (n_vertex, edges) = sample_edge2();
            let adj = make_adj_from_directed(n_vertex, &edges);
            assert!(!has_cycle_directed(&adj));
        }
        {
            let (n_vertex, edges) = sample_edges3();
            let adj = make_adj_from_directed(n_vertex, &edges);
            assert!(!has_cycle_directed(&adj));
        }
        {
            // 0 → 1 → 2
            //     ↑   ↓
            //     4 → 3
            let n_vertex = 5;
            let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 1)];
            let adj = make_adj_from_directed(n_vertex, &edges);
            assert!(has_cycle_directed(&adj));
        }
        {
            // 0 ← 1 → 2
            //     ↑   ↓
            //     4 → 3
            let n_vertex = 5;
            let edges = vec![(1, 0), (1, 2), (2, 3), (3, 4), (4, 1)];
            let adj = make_adj_from_directed(n_vertex, &edges);
            assert!(has_cycle_directed(&adj));
        }
        {
            // 0 → 1 → 2 → 5
            //     ↓   ↓   ↑
            //     4 → 3 → 6
            let n_vertex = 7;
            let edges = vec![
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 4),
                (1, 4),
                (2, 5),
                (3, 6),
                (6, 5),
            ];
            let adj = make_adj_from_directed(n_vertex, &edges);
            assert!(!has_cycle_directed(&adj));
        }
    }
}
