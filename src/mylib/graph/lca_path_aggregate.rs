use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use lca_path_aggregate::*;")]
pub mod lca_path_aggregate {
    use ac_library::Monoid;
    use std::mem::swap;

    pub struct LcaPathAggregate<M: Monoid> {
        dist: Vec<i64>,                 // dist[v]: ルートから v までの距離 (深さ)
        ancestor: Vec<Vec<usize>>,      // ancestor[i][v]: v の 2^i 先の祖先
        path_aggregate: Vec<Vec<M::S>>, // path_aggregate[i][v]: v から 2^i 先の祖先へのパス上の辺の重みの総和
    }
    impl<M: Monoid> LcaPathAggregate<M>
    where
        M::S: Clone,
    {
        /// パス上の値をマージ可能な LCA 構造体を構築する。
        ///
        /// # Arguments
        /// * `adj` - 隣接リスト表現の木。`adj[u]` は `(v, w)` のリスト。`v` は隣接ノード、`w` は辺 `u-v` の重み。
        /// * `root` - 木の根
        ///
        /// # 計算量
        /// O(V log V) (V は頂点数)
        pub fn new(adj: &[Vec<(usize, M::S)>], root: usize) -> Self {
            let nv = adj.len();
            let mut dist = vec![-1; nv];
            let mut parent = vec![root; nv];
            let mut parent_edge_weight = vec![M::identity(); nv];

            let mut q = std::collections::VecDeque::new();
            q.push_back(root);
            dist[root] = 0;

            while let Some(u) = q.pop_front() {
                for &(v, ref w) in &adj[u] {
                    if v != parent[u] {
                        dist[v] = dist[u] + 1;
                        parent[v] = u;
                        parent_edge_weight[v] = w.clone();
                        q.push_back(v);
                    }
                }
            }

            let k = (usize::BITS - nv.leading_zeros()) as usize;

            let mut ancestor = vec![vec![0; nv]; k];
            ancestor[0] = parent;

            for i in 1..k {
                for v in 0..nv {
                    let p = ancestor[i - 1][v];
                    ancestor[i][v] = ancestor[i - 1][p];
                }
            }

            let mut path_aggregate = vec![vec![M::identity(); nv]; k];
            path_aggregate[0] = parent_edge_weight;

            for i in 1..k {
                for v in 0..nv {
                    let p = ancestor[i - 1][v];
                    path_aggregate[i][v] =
                        M::binary_operation(&path_aggregate[i - 1][v], &path_aggregate[i - 1][p]);
                }
            }

            Self {
                dist,
                ancestor,
                path_aggregate,
            }
        }

        /// u と v の LCA を求める
        ///
        /// # 計算量
        /// O(log V)
        pub fn lca(&self, u: usize, v: usize) -> usize {
            let mut u = u;
            let mut v = v;
            if self.dist[u] < self.dist[v] {
                swap(&mut u, &mut v);
            }

            let dist_diff = self.dist[u] - self.dist[v];
            for k in 0..self.ancestor.len() {
                if (dist_diff >> k) & 1 == 1 {
                    u = self.ancestor[k][u];
                }
            }

            if u == v {
                return u;
            }

            for k in (0..self.ancestor.len()).rev() {
                if self.ancestor[k][u] != self.ancestor[k][v] {
                    u = self.ancestor[k][u];
                    v = self.ancestor[k][v];
                }
            }
            self.ancestor[0][u]
        }

        /// u から v へのパス上の辺の重みの総和（モノイド積）を求める
        ///
        /// このクエリはモノイドの可換性を要求します。
        ///
        /// # 計算量
        /// O(log V)
        pub fn path_aggregate(&self, u: usize, v: usize) -> M::S {
            let lca = self.lca(u, v);
            let agg_u = self.query_path_up(u, self.dist[u] - self.dist[lca]);
            let agg_v = self.query_path_up(v, self.dist[v] - self.dist[lca]);
            M::binary_operation(&agg_u, &agg_v)
        }

        // u から dist だけ親方向に遡ったパスの重みの総和
        fn query_path_up(&self, u: usize, dist: i64) -> M::S {
            let mut res = M::identity();
            let mut current = u;
            for k in 0..self.ancestor.len() {
                if (dist >> k) & 1 == 1 {
                    res = M::binary_operation(&res, &self.path_aggregate[k][current]);
                    current = self.ancestor[k][current];
                }
            }
            res
        }

        /// u と v の距離 (辺の数)
        ///
        /// # 計算量
        /// O(log V)
        pub fn dist(&self, u: usize, v: usize) -> i64 {
            let lca = self.lca(u, v);
            self.dist[u] + self.dist[v] - 2 * self.dist[lca]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::lca_path_aggregate::LcaPathAggregate;
    use ac_library::Additive;
    use itertools::Itertools;
    use rand::SeedableRng;
    use rand::prelude::*;
    use rand::rngs::StdRng;

    fn build_tree(n: usize, edges: &[(usize, usize, i64)]) -> Vec<Vec<(usize, i64)>> {
        let mut adj = vec![vec![]; n];
        for &(u, v, w) in edges {
            adj[u].push((v, w));
            adj[v].push((u, w));
        }
        adj
    }

    #[test]
    fn test_path_aggregate() {
        // 0 --1-- 1 --2-- 3 --3-- 6
        // |
        // 1 --4-- 4 --1-- 7
        // |       |--1-- 8
        // |       '--1-- 9
        // |
        // 2 --1-- 5 --1-- 10
        //         |
        //         '--1-- 11
        let n = 12;
        let edges = [
            (0, 1, 1),
            (1, 3, 2),
            (3, 6, 3),
            (1, 4, 4),
            (4, 7, 1),
            (4, 8, 1),
            (4, 9, 1),
            (0, 2, 1),
            (2, 5, 1),
            (5, 10, 1),
            (5, 11, 1),
        ];
        let adj = build_tree(n, &edges);
        let lca_agg = LcaPathAggregate::<Additive<i64>>::new(&adj, 0);

        // path: 6-3-1-4-9, weights: 3, 2, 4, 1. sum = 10
        assert_eq!(lca_agg.path_aggregate(6, 9), 3 + 2 + 4 + 1);
        // path: 9-4-1-0-2-5-10, weights: 1, 4, 1, 1, 1, 1. sum = 9
        assert_eq!(lca_agg.path_aggregate(9, 10), 1 + 4 + 1 + 1 + 1 + 1);
        // path: 1-3-6, weights: 2, 3. sum = 5
        assert_eq!(lca_agg.path_aggregate(1, 6), 2 + 3);
        // path: 3-3, weights: 0
        assert_eq!(lca_agg.path_aggregate(3, 3), 0);

        // Naive check
        let mut parent = vec![0; n];
        let mut weights = vec![0; n];
        let mut q = std::collections::VecDeque::new();
        q.push_back(0);
        let mut visited = vec![false; n];
        visited[0] = true;

        while let Some(u) = q.pop_front() {
            for &(v, w) in &adj[u] {
                if !visited[v] {
                    visited[v] = true;
                    parent[v] = u;
                    weights[v] = w;
                    q.push_back(v);
                }
            }
        }

        let query_naive = |mut u: usize, mut v: usize| {
            let lca = lca_agg.lca(u, v);
            let mut sum = 0;
            while u != lca {
                sum += weights[u];
                u = parent[u];
            }
            while v != lca {
                sum += weights[v];
                v = parent[v];
            }
            sum
        };

        for u in 0..n {
            for v in 0..n {
                assert_eq!(
                    lca_agg.path_aggregate(u, v),
                    query_naive(u, v),
                    "u={}, v={}",
                    u,
                    v
                );
            }
        }
    }

    #[test]
    fn test_dist() {
        // 0 --1-- 1 --2-- 3 --3-- 6
        // |
        // 1 --4-- 4 --1-- 7
        // |       |--1-- 8
        // |       '--1-- 9
        // |
        // 2 --1-- 5 --1-- 10
        //         |
        //         '--1-- 11
        let n = 12;
        let edges = [
            (0, 1, 1),
            (1, 3, 2),
            (3, 6, 3),
            (1, 4, 4),
            (4, 7, 1),
            (4, 8, 1),
            (4, 9, 1),
            (0, 2, 1),
            (2, 5, 1),
            (5, 10, 1),
            (5, 11, 1),
        ];
        let adj = build_tree(n, &edges);
        let lca_agg = LcaPathAggregate::<Additive<i64>>::new(&adj, 0);

        // path: 6-3-1-4-9, edges: 6-3, 3-1, 1-4, 4-9. len = 4
        assert_eq!(lca_agg.dist(6, 9), 4);
        // path: 9-4-1-0-2-5-10, edges: 9-4, 4-1, 1-0, 0-2, 2-5, 5-10. len = 6
        assert_eq!(lca_agg.dist(9, 10), 6);
        // path: 1-3-6, edges: 1-3, 3-6. len = 2
        assert_eq!(lca_agg.dist(1, 6), 2);
        // path: 3-3, len = 0
        assert_eq!(lca_agg.dist(3, 3), 0);
    }

    #[test]
    fn test_path_aggregate_line_graph() {
        for n in 1..=10 {
            if n == 1 {
                let adj = build_tree(1, &[]);
                let lca_agg = LcaPathAggregate::<Additive<i64>>::new(&adj, 0);
                assert_eq!(lca_agg.path_aggregate(0, 0), 0);
                continue;
            }
            let edges = (0..n - 1).map(|i| (i, i + 1, (i + 1) as i64)).collect_vec();
            let adj = build_tree(n, &edges);
            let lca_agg = LcaPathAggregate::<Additive<i64>>::new(&adj, 0);

            for u in 0..n {
                for v in 0..n {
                    let mut expected = 0;
                    let start = u.min(v);
                    let end = u.max(v);
                    for i in start..end {
                        expected += (i + 1) as i64;
                    }
                    assert_eq!(
                        lca_agg.path_aggregate(u, v),
                        expected,
                        "n={}, u={}, v={}",
                        n,
                        u,
                        v
                    );
                }
            }
        }
    }

    #[test]
    #[ignore]
    fn test_path_aggregate_random() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..1000 {
            let n = rng.random_range(2..=50);
            let mut edges = Vec::new();
            for i in 1..n {
                let p = rng.random_range(0..i);
                let w = rng.random_range(0..1_000_000_000);
                edges.push((p, i, w));
            }

            let adj = build_tree(n, &edges);
            let lca_agg = LcaPathAggregate::<Additive<i64>>::new(&adj, 0);

            let (parent, weights) = {
                let mut parent = vec![0; n];
                let mut weights = vec![0; n]; // weights[v]: v と parent[v] を繋ぐ辺の重み
                let mut q = std::collections::VecDeque::new();
                q.push_back(0);
                let mut visited = vec![false; n];
                visited[0] = true;

                while let Some(u) = q.pop_front() {
                    for &(v, w) in &adj[u] {
                        if !visited[v] {
                            visited[v] = true;
                            parent[v] = u;
                            weights[v] = w;
                            q.push_back(v);
                        }
                    }
                }
                (parent, weights)
            };

            let query_naive = |mut u: usize, mut v: usize| {
                let lca = lca_agg.lca(u, v);
                let mut sum = 0;
                while u != lca {
                    sum += weights[u];
                    u = parent[u];
                }
                while v != lca {
                    sum += weights[v];
                    v = parent[v];
                }
                sum
            };

            for _ in 0..100 {
                let u = rng.random_range(0..n);
                let v = rng.random_range(0..n);
                assert_eq!(
                    lca_agg.path_aggregate(u, v),
                    query_naive(u, v),
                    "n={}, u={}, v={}",
                    n,
                    u,
                    v
                );
            }
        }
    }
}
