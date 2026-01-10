use crate::data_structure::ix::{Bounds, Ix, IxVec};
use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use lowlink::*;")]
pub mod lowlink {
    /// LowLink (橋、関節点の検出)
    ///
    /// 計算量: O(V + E)
    #[derive(Clone, Debug)]
    pub struct LowLink {
        /// 各頂点の訪問順序 (0-indexed)
        pub ord: Vec<usize>,
        /// 各頂点から DFS 木の辺と後退辺を最大 1 回通って到達できる頂点の最小 ord
        pub low: Vec<usize>,
        /// 橋 (u, v) のリスト。常に u < v となるように正規化されている。
        pub bridges: Vec<(usize, usize)>,
        /// 関節点のリスト。昇順にソートされている。
        pub articulation_points: Vec<usize>,
    }

    impl LowLink {
        /// LowLink を構築する
        ///
        /// # Arguments
        /// * `nv` - 頂点数
        /// * `adj` - 頂点を受け取り、隣接する頂点のイテレータを返すクロージャー
        pub fn new<F, It>(nv: usize, mut adj: F) -> Self
        where
            F: FnMut(usize) -> It,
            It: IntoIterator<Item = usize>,
        {
            let mut ord = vec![usize::MAX; nv];
            let mut low = vec![usize::MAX; nv];
            let mut bridges = Vec::new();
            let mut articulation_points = Vec::new();
            let mut k = 0;

            // adj を何度も呼ぶので、事前にグラフを構築したほうが効率的かもしれないが、
            // ここでは汎用性を重視してクロージャを受け取る形を維持する。
            // ただし、再帰呼び出しの中でクロージャを呼ぶのは難しいため、
            // 内部で隣接リストを作成する。
            let mut g = vec![vec![]; nv];
            for i in 0..nv {
                for v in adj(i) {
                    if v != i {
                        // 自己ループは無視 (橋や関節点の定義上、影響しないことが多いが、文脈による。
                        // ここでは単純グラフを想定)
                        g[i].push(v);
                    }
                }
            }

            for i in 0..nv {
                if ord[i] == usize::MAX {
                    Self::dfs(
                        i,
                        usize::MAX,
                        &g,
                        &mut k,
                        &mut ord,
                        &mut low,
                        &mut bridges,
                        &mut articulation_points,
                    );
                }
            }

            bridges.sort();
            articulation_points.sort();
            articulation_points.dedup();

            Self {
                ord,
                low,
                bridges,
                articulation_points,
            }
        }

        #[allow(clippy::too_many_arguments)]
        fn dfs(
            u: usize,
            p: usize,
            g: &[Vec<usize>],
            k: &mut usize,
            ord: &mut Vec<usize>,
            low: &mut Vec<usize>,
            bridges: &mut Vec<(usize, usize)>,
            articulation_points: &mut Vec<usize>,
        ) {
            ord[u] = *k;
            low[u] = *k;
            *k += 1;

            let mut is_articulation = false;
            let mut child_count = 0;

            for &v in &g[u] {
                if v == p {
                    continue;
                }
                if ord[v] != usize::MAX {
                    low[u] = low[u].min(ord[v]);
                } else {
                    child_count += 1;
                    Self::dfs(v, u, g, k, ord, low, bridges, articulation_points);
                    low[u] = low[u].min(low[v]);
                    if p != usize::MAX && low[v] >= ord[u] {
                        is_articulation = true;
                    }
                    if low[v] > ord[u] {
                        if u < v {
                            bridges.push((u, v));
                        } else {
                            bridges.push((v, u));
                        }
                    }
                }
            }

            if p == usize::MAX && child_count >= 2 {
                is_articulation = true;
            }

            if is_articulation {
                articulation_points.push(u);
            }
        }
    }
}

#[snippet(prefix = "use lowlink_ix::*;")]
pub mod lowlink_ix {
    use super::lowlink::LowLink;
    use super::{Bounds, Ix, IxVec};

    /// LowLink の結果 (Ix版)
    #[derive(Clone, Debug)]
    pub struct LowLinkIxResult<I: Ix> {
        pub ord: IxVec<I, usize>,
        pub low: IxVec<I, usize>,
        pub bridges: Vec<(I, I)>,
        pub articulation_points: Vec<I>,
    }

    /// Bounds を用いた任意の型 I に対する LowLink
    pub fn lowlink_arbitrary<I, F, It>(bounds: Bounds<I>, mut adj: F) -> LowLinkIxResult<I>
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

        let res = LowLink::new(nv, &mut adj_usize);

        LowLinkIxResult {
            ord: IxVec::from_vec(bounds, res.ord),
            low: IxVec::from_vec(bounds, res.low),
            bridges: res
                .bridges
                .into_iter()
                .map(|(u, v)| (bounds.from_index(u), bounds.from_index(v)))
                .collect(),
            articulation_points: res
                .articulation_points
                .into_iter()
                .map(|u| bounds.from_index(u))
                .collect(),
        }
    }
}

pub use lowlink::*;
pub use lowlink_ix::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_structure::ix::Bounds;

    #[test]
    fn test_lowlink_basic() {
        // 0 - 1 - 2
        // |   |
        // 3 - 4
        //
        // 5 (isolated)
        let adj = [
            vec![1, 3],    // 0
            vec![0, 2, 4], // 1
            vec![1],       // 2
            vec![0, 4],    // 3
            vec![1, 3],    // 4
            vec![],        // 5
        ];

        let res = LowLink::new(6, |u| adj[u].iter().copied());

        // Bridges: (1, 2) is the only bridge
        assert_eq!(res.bridges, vec![(1, 2)]);

        // Articulation points: 1 is the only articulation point (connecting {2} to others)
        // Wait, removing 1 disconnects 2.
        // 0-3-4 is a cycle. 0-1-4-3-0 is a cycle.
        // {0, 1, 3, 4} is biconnected if we ignore 2.
        // Removing 1 leaves 2 isolated. So 1 is articulation.
        // Removing others doesn't increase connected components count (5 is already isolated).
        assert_eq!(res.articulation_points, vec![1]);
    }

    #[test]
    fn test_lowlink_arbitrary() {
        let bounds = Bounds::new((0, 0), (1, 2)); // 2x3 grid
        // (0,0)-(0,1)-(0,2)
        //   |     |
        // (1,0)-(1,1)  (1,2) <- isolated from (1,1) but connected to (0,2)? let's make a specific graph.

        // Graph:
        // (0,0) - (0,1) - (0,2)
        //           |
        //         (1,1)
        //
        // (1,0) and (1,2) are isolated for simplicity, or just not used in edges.

        let adj = |u: (usize, usize)| -> Vec<(usize, usize)> {
            match u {
                (0, 0) => vec![(0, 1)],
                (0, 1) => vec![(0, 0), (0, 2), (1, 1)],
                (0, 2) => vec![(0, 1)],
                (1, 1) => vec![(0, 1)],
                _ => vec![],
            }
        };

        let res = lowlink_arbitrary(bounds, adj);

        // Bridges: all edges are bridges because it's a tree.
        // Edges: ((0,0), (0,1)), ((0,1), (0,2)), ((0,1), (1,1))
        // The output of bridges is sorted by index.
        // (0,0) -> 0
        // (0,1) -> 1
        // (0,2) -> 2
        // (1,0) -> 3
        // (1,1) -> 4
        // (1,2) -> 5

        // Bridges indices: (0,1), (1,2), (1,4)
        // Coords: ((0,0),(0,1)), ((0,1),(0,2)), ((0,1),(1,1))

        // Note: internal sort is by index. The output conversion maintains order?
        // IxVec map doesn't, but bridges is a Vec.
        // In `lowlink_arbitrary`, we map indices back to coords.
        // Since indices are sorted (u < v, and pairs sorted), the coords might not be sorted "visually" but will correspond to sorted indices.

        let mut bridges = res.bridges;
        // Normalize for comparison (sort pairs, then sort vector)
        bridges.iter_mut().for_each(|(u, v)| {
            if u > v {
                std::mem::swap(u, v);
            }
        });
        bridges.sort();

        let expected = vec![((0, 0), (0, 1)), ((0, 1), (0, 2)), ((0, 1), (1, 1))];
        assert_eq!(bridges, expected);

        // Articulation points: (0,1) is the center.
        assert_eq!(res.articulation_points, vec![(0, 1)]);
    }

    fn find_bridges_brute_force(nv: usize, edges: &[(usize, usize)]) -> Vec<(usize, usize)> {
        use crate::data_structure::dsu::dsu_core::DsuCore;

        edges
            .iter()
            .filter(|&&(u, v)| {
                let mut dsu_orig = DsuCore::new(nv);
                let mut dsu_removed = DsuCore::new(nv);
                for &(nu, nv) in edges {
                    dsu_orig.merge(nu, nv);
                    if (u, v) != (nu, nv) {
                        dsu_removed.merge(nu, nv);
                    }
                }
                dsu_removed.count_group() > dsu_orig.count_group()
            })
            .copied()
            .collect()
    }

    fn find_articulation_points_brute_force(nv: usize, edges: &[(usize, usize)]) -> Vec<usize> {
        use crate::data_structure::dsu::dsu_core::DsuCore;
        use itertools::Itertools;

        (0..nv)
            .filter(|&i| {
                let mut dsu_orig = DsuCore::new(nv);
                for &(u, v) in edges {
                    dsu_orig.merge(u, v);
                }

                let i_leader = dsu_orig.leader(i);
                let count_before = (0..nv)
                    .filter(|&v| v != i)
                    .map(|v| dsu_orig.leader(v))
                    .filter(|&l| l != i_leader)
                    .unique()
                    .count();

                let mut dsu_after = DsuCore::new(nv);
                for &(u, v) in edges {
                    if u != i && v != i {
                        dsu_after.merge(u, v);
                    }
                }
                let count_after = (0..nv)
                    .filter(|&v| v != i)
                    .map(|v| dsu_after.leader(v))
                    .unique()
                    .count();

                count_after > count_before + 1
            })
            .collect()
    }

    #[test]
    #[ignore]
    fn test_lowlink_random() {
        use itertools::iproduct;
        use rand::prelude::*;

        let mut rng = StdRng::seed_from_u64(42);

        for _ in 0..50 {
            let nv = rng.random_range(1..=20);
            let edges: Vec<_> = iproduct!(0..nv, 0..nv)
                .filter(|&(u, v)| u < v && rng.random_bool(0.2))
                .collect();

            let adj = edges.iter().fold(vec![vec![]; nv], |mut acc, &(u, v)| {
                acc[u].push(v);
                acc[v].push(u);
                acc
            });

            let res = LowLink::new(nv, |u| adj[u].iter().copied());

            let expected_bridges = find_bridges_brute_force(nv, &edges);
            let expected_articulation_points = find_articulation_points_brute_force(nv, &edges);

            assert_eq!(res.bridges, expected_bridges, "Bridges mismatch");
            assert_eq!(
                res.articulation_points, expected_articulation_points,
                "Articulation points mismatch"
            );
        }
    }
}
