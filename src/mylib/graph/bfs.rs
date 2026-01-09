use cargo_snippet::snippet;

#[snippet(prefix = "use bfs::*;")]
#[allow(clippy::module_inception)]
pub mod bfs {
    use std::collections::VecDeque;

    /// BFS の結果（距離と復元情報）
    #[derive(Clone, Debug)]
    pub struct BfsResult {
        pub dist: Vec<Option<i64>>,
        pub prev: Vec<Option<usize>>,
    }

    impl BfsResult {
        /// 頂点 `t` への最短経路を復元する（始点 -> ... -> t）
        pub fn restore(&self, t: usize) -> Option<Vec<usize>> {
            self.dist[t]?;
            let mut path: Vec<_> = std::iter::successors(Some(t), |&curr| self.prev[curr]).collect();
            path.reverse();
            Some(path)
        }
    }

    /// 標準的な usize インデックスを用いた幅優先探索 (BFS)
    ///
    /// # Arguments
    /// * `nv` - 頂点数
    /// * `adj` - 頂点を受け取り、隣接する頂点のイテレータを返すクロージャー
    /// * `init` - 始点となる頂点集合のイテレータ
    ///
    /// # Returns
    /// 始点集合 `init` からの最短距離を格納した `Vec<Option<i64>>`。到達不可能な頂点は `None`。
    pub fn bfs<F, It>(
        nv: usize,
        mut adj: F,
        init: impl IntoIterator<Item = usize>,
    ) -> Vec<Option<i64>>
    where
        F: FnMut(usize) -> It,
        It: IntoIterator<Item = usize>,
    {
        let mut dist = vec![None; nv];
        let mut q = VecDeque::new();
        for s in init {
            if dist[s].is_none() {
                dist[s] = Some(0);
                q.push_back(s);
            }
        }
        while let Some(u) = q.pop_front() {
            let d = dist[u].unwrap();
            for v in adj(u) {
                if dist[v].is_none() {
                    dist[v] = Some(d + 1);
                    q.push_back(v);
                }
            }
        }
        dist
    }

    /// 経路復元可能な BFS
    ///
    /// # Returns
    /// 最短距離 `dist` と、復元用配列 `prev` を含む `BfsResult`。
    pub fn bfs_with_restore<F, It>(
        nv: usize,
        mut adj: F,
        init: impl IntoIterator<Item = usize>,
    ) -> BfsResult
    where
        F: FnMut(usize) -> It,
        It: IntoIterator<Item = usize>,
    {
        let mut dist = vec![None; nv];
        let mut prev = vec![None; nv];
        let mut q = VecDeque::new();

        for s in init {
            if dist[s].is_none() {
                dist[s] = Some(0);
                // 始点の prev は None
                q.push_back(s);
            }
        }

        while let Some(u) = q.pop_front() {
            let d = dist[u].unwrap();
            for v in adj(u) {
                if dist[v].is_none() {
                    dist[v] = Some(d + 1);
                    prev[v] = Some(u);
                    q.push_back(v);
                }
            }
        }
        BfsResult { dist, prev }
    }
}

#[snippet(prefix = "use bfs_ix::*;")]
pub mod bfs_ix {
    use super::bfs::{bfs, bfs_with_restore};
    use crate::data_structure::ix::{Bounds, Ix, IxVec};

    /// BFS の結果（Ix版）
    #[derive(Clone, Debug)]
    pub struct BfsIxResult<I: Ix> {
        pub dist: IxVec<I, Option<i64>>,
        pub prev: IxVec<I, Option<I>>,
    }

    impl<I: Ix> BfsIxResult<I> {
        pub fn restore(&self, t: I) -> Option<Vec<I>> {
            self.dist[t]?;
            let mut path: Vec<_> = std::iter::successors(Some(t), |&curr| self.prev[curr]).collect();
            path.reverse();
            Some(path)
        }
    }

    /// Bounds を用いた任意の型 I に対する BFS
    pub fn bfs_arbitrary<I, F, It>(
        bounds: Bounds<I>,
        mut adj: F,
        init: impl IntoIterator<Item = I>,
    ) -> IxVec<I, Option<i64>>
    where
        I: Ix,
        F: FnMut(I) -> It,
        It: IntoIterator<Item = I>,
    {
        let nv = bounds.range_size();
        let mut adj_usize = |u_idx: usize| {
            let u = bounds.from_index(u_idx);
            adj(u).into_iter().map(move |v| bounds.to_index(v))
        };
        let init_usize = init.into_iter().map(|s| bounds.to_index(s));
        let res_vec = bfs(nv, &mut adj_usize, init_usize);
        IxVec::from_vec(bounds, res_vec)
    }

    /// Bounds を用いた任意の型 I に対する BFS (経路復元付き)
    pub fn bfs_with_restore_arbitrary<I, F, It>(
        bounds: Bounds<I>,
        mut adj: F,
        init: impl IntoIterator<Item = I>,
    ) -> BfsIxResult<I>
    where
        I: Ix,
        F: FnMut(I) -> It,
        It: IntoIterator<Item = I>,
    {
        let nv = bounds.range_size();
        let mut adj_usize = |u_idx: usize| {
            let u = bounds.from_index(u_idx);
            adj(u).into_iter().map(move |v| bounds.to_index(v))
        };
        let init_usize = init.into_iter().map(|s| bounds.to_index(s));

        let res = bfs_with_restore(nv, &mut adj_usize, init_usize);

        BfsIxResult {
            dist: IxVec::from_vec(bounds, res.dist),
            prev: IxVec::from_vec(
                bounds,
                res.prev
                    .into_iter()
                    .map(|p| p.map(|idx| bounds.from_index(idx)))
                    .collect(),
            ),
        }
    }
}

pub use bfs::*;
pub use bfs_ix::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_structure::ix::Bounds;

    #[test]
    fn test_bfs_basic() {
        let adj = [vec![1], vec![0, 2], vec![1]];
        let res = bfs(3, |u| adj[u].iter().copied(), [0]);
        assert_eq!(res, vec![Some(0), Some(1), Some(2)]);
    }

    #[test]
    fn test_bfs_arbitrary() {
        let bounds = Bounds::new((0, 0), (1, 1));
        let res = bfs_arbitrary(
            bounds,
            |(r, c)| {
                if (r, c) == (0, 0) {
                    vec![(0, 1), (1, 0)]
                } else {
                    vec![]
                }
            },
            [(0, 0)],
        );
        assert_eq!(res[(0, 0)], Some(0));
        assert_eq!(res[(0, 1)], Some(1));
        assert_eq!(res[(1, 0)], Some(1));
        assert_eq!(res[(1, 1)], None);
    }

    #[test]
    fn test_bfs_restore() {
        // 0 -> 1 -> 2
        // |
        // v
        // 3
        let adj = [vec![1, 3], vec![2], vec![], vec![]];
        let res = bfs_with_restore(4, |u| adj[u].iter().copied(), [0]);

        assert_eq!(res.dist, vec![Some(0), Some(1), Some(2), Some(1)]);
        assert_eq!(res.restore(2), Some(vec![0, 1, 2]));
        assert_eq!(res.restore(3), Some(vec![0, 3]));
        assert_eq!(res.restore(0), Some(vec![0]));
    }

    #[test]
    fn test_bfs_restore_arbitrary() {
        let bounds = Bounds::new((0, 0), (1, 1));
        // (0,0) -> (0,1) -> (1,1)
        //   |
        //   v
        // (1,0)
        let res = bfs_with_restore_arbitrary(
            bounds,
            |(r, c)| {
                let mut ret = vec![];
                if r + 1 <= 1 {
                    ret.push((r + 1, c));
                }
                if c + 1 <= 1 {
                    ret.push((r, c + 1));
                }
                ret
            },
            [(0, 0)],
        );

        assert_eq!(res.dist[(1, 1)], Some(2));
        // (0,0) -> (0,1) -> (1,1) OR (0,0) -> (1,0) -> (1,1)
        let path = res.restore((1, 1)).unwrap();
        assert_eq!(path.first(), Some(&(0, 0)));
        assert_eq!(path.last(), Some(&(1, 1)));
        assert_eq!(path.len(), 3);
    }

    fn solve_bellman_ford(nv: usize, adj: &[Vec<usize>], starts: &[usize]) -> Vec<Option<i64>> {
        let mut dist = vec![None; nv];
        for &s in starts {
            dist[s] = Some(0);
        }
        for _ in 0..nv {
            let mut updated = false;
            for u in 0..nv {
                if let Some(d) = dist[u] {
                    for &v in &adj[u] {
                        if dist[v].is_none_or(|cur| cur > d + 1) {
                            dist[v] = Some(d + 1);
                            updated = true;
                        }
                    }
                }
            }
            if !updated {
                break;
            }
        }
        dist
    }

    #[test]
    #[ignore]
    fn test_bfs_random() {
        use itertools::iproduct;
        use rand::prelude::*;
        let mut rng = StdRng::seed_from_u64(42);

        for _ in 0..100 {
            let nv = rng.random_range(1..=20);
            let adj = iproduct!(0..nv, 0..nv)
                .filter_map(|(u, v)| (u != v && rng.random_bool(0.3)).then_some((u, v)))
                .fold(vec![vec![]; nv], |mut acc, (u, v)| {
                    acc[u].push(v);
                    acc
                });

            // Multiple starts
            let num_starts = rng.random_range(0..=3.min(nv));
            let starts = (0..nv).choose_multiple(&mut rng, num_starts);

            // Expected
            let expected_dist = solve_bellman_ford(nv, &adj, &starts);

            // Test bfs
            let res_dist = bfs(nv, |u| adj[u].iter().copied(), starts.iter().copied());
            assert_eq!(res_dist, expected_dist, "bfs dist mismatch");

            // Test bfs_with_restore
            let res = bfs_with_restore(nv, |u| adj[u].iter().copied(), starts.iter().copied());
            assert_eq!(res.dist, expected_dist, "bfs_with_restore dist mismatch");

            // Path Check
            for i in 0..nv {
                if let Some(path) = res.restore(i) {
                    assert!(starts.contains(&path[0]), "Path must start from one of the sources");
                    assert_eq!(*path.last().unwrap(), i);
                    assert_eq!(path.len() as i64 - 1, res.dist[i].unwrap());

                    // Path check
                    for win in path.windows(2) {
                        let u = win[0];
                        let v = win[1];
                        assert!(adj[u].contains(&v), "Invalid edge in path: {} -> {}", u, v);
                    }
                } else {
                    assert!(res.dist[i].is_none());
                }
            }
        }
    }
}
