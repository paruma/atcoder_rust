use crate::data_structure::ix::{Bounds, Ix, IxVec};
use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use lowlink::*;")]
pub mod lowlink {
    /// LowLink (無向グラフの橋、関節点の検出)
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
        /// * `adj_fn` - 頂点を受け取り、隣接する頂点のイテレータを返すクロージャー。
        ///   **無向グラフ**として隣接頂点を返す必要があります。
        pub fn new<F, It>(nv: usize, mut adj_fn: F) -> Self
        where
            F: FnMut(usize) -> It,
            It: IntoIterator<Item = usize>,
        {
            let mut ord = vec![usize::MAX; nv];
            let mut low = vec![usize::MAX; nv];
            let mut bridges = Vec::new();
            let mut articulation_points = Vec::new();
            let mut k = 0;

            // adj_fn を何度も呼ぶので、事前にグラフを構築したほうが効率的かもしれないが、
            // ここでは汎用性を重視してクロージャを受け取る形を維持する。
            // ただし、再帰呼び出しの中でクロージャを呼ぶのは難しいため、
            // 内部で隣接リストを作成する。
            let adj = {
                let mut adj = vec![vec![]; nv];
                for i in 0..nv {
                    for v in adj_fn(i) {
                        if v != i {
                            // 自己ループは無視 (橋や関節点の定義上、影響しないことが多いが、文脈による。
                            // ここでは単純グラフを想定)
                            adj[i].push(v);
                        }
                    }
                }
                adj
            };

            for i in 0..nv {
                if ord[i] == usize::MAX {
                    Self::dfs(
                        i,
                        usize::MAX,
                        &adj,
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

        /// DFS を行い、ord, low, bridges, articulation_points を計算する
        ///
        /// # Arguments
        /// * `u` - 現在の頂点
        /// * `p` - 親頂点
        /// * `adj` - 隣接リスト
        /// * `k` - 訪問順序のカウンター
        /// * `ord` - 各頂点の訪問順序
        /// * `low` - 各頂点の lowlink 値
        /// * `bridges` - 橋のリスト
        /// * `articulation_points` - 関節点のリスト
        #[allow(clippy::too_many_arguments)]
        fn dfs(
            u: usize,
            p: usize,
            adj: &[Vec<usize>],
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

            for &v in &adj[u] {
                if v == p {
                    continue;
                }
                if ord[v] != usize::MAX {
                    low[u] = low[u].min(ord[v]);
                } else {
                    child_count += 1;
                    Self::dfs(v, u, adj, k, ord, low, bridges, articulation_points);
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
        /// 各頂点の訪問順序 (0-indexed)
        pub ord: IxVec<I, usize>,
        /// 各頂点から DFS 木の辺と後退辺を最大 1 回通って到達できる頂点の最小 ord
        pub low: IxVec<I, usize>,
        /// 橋 (u, v) のリスト。
        pub bridges: Vec<(I, I)>,
        /// 関節点のリスト。
        pub articulation_points: Vec<I>,
    }

    /// Bounds を用いた任意の型 I に対する LowLink (無向グラフ)
    ///
    /// # Arguments
    /// * `bounds` - 頂点のインデックス範囲
    /// * `adj` - 頂点を受け取り、隣接する頂点のイテレータを返すクロージャー。
    ///   **無向グラフ**として隣接頂点を返す必要があります。
    ///
    /// # Returns
    /// LowLink の結果を格納した `LowLinkIxResult<I>`。
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

#[cfg(test)]
mod tests {
    use super::lowlink::*;
    use super::lowlink_ix::*;
    use crate::data_structure::ix::Bounds;

    #[test]
    fn test_lowlink_basic() {
        // 0 - 1 - 2
        // |   |
        // 3 - 4
        //
        // 5 (独立)
        let adj = [
            vec![1, 3],    // 0
            vec![0, 2, 4], // 1
            vec![1],       // 2
            vec![0, 4],    // 3
            vec![1, 3],    // 4
            vec![],        // 5
        ];

        let res = LowLink::new(6, |u| adj[u].iter().copied());

        // 橋: (1, 2) が唯一の橋
        assert_eq!(res.bridges, vec![(1, 2)]);

        // 関節点: 1 が唯一の関節点（2 を他の頂点から切り離す）
        // 1 を取り除くと 2 が孤立する。
        // 0-3-4-1-0 はサイクルを形成しており、2 を除けば {0, 1, 3, 4} は二重連結。
        // 1 を取り除くと 2 が孤立するため、1 は関節点となる。
        // 他の頂点を取り除いても（5 は元々孤立しているため）連結成分の数は増えない。
        assert_eq!(res.articulation_points, vec![1]);
    }

    #[test]
    fn test_lowlink_arbitrary() {
        let bounds = Bounds::new((0, 0), (1, 2)); // 2x3 グリッド
        // (0,0)-(0,1)-(0,2)
        //   |     |
        // (1,0)-(1,1)  (1,2) <- (1,1)から孤立しているが、(0,2)に接続されている？
        // ここでは特定のグラフを構築する。

        // グラフ構造:
        // (0,0) - (0,1) - (0,2)
        //           |
        //         (1,1)
        //
        // 簡略化のため (1,0) と (1,2) は孤立点とする。

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

        // 橋: 木構造なので、すべての辺が橋となる。
        // 辺: ((0,0), (0,1)), ((0,1), (0,2)), ((0,1), (1,1))
        // 橋の出力はインデックス順にソートされる。
        // (0,0) -> 0
        // (0,1) -> 1
        // (0,2) -> 2
        // (1,0) -> 3
        // (1,1) -> 4
        // (1,2) -> 5

        // 橋のインデックス: (0,1), (1,2), (1,4)
        // 座標: ((0,0),(0,1)), ((0,1),(0,2)), ((0,1),(1,1))

        // 注意: 内部的なソートはインデックス順。座標変換後も順序は維持されるか？
        // IxVec の map は維持しないが、bridges は Vec である。
        // `lowlink_arbitrary` では、インデックスを座標に戻している。
        // インデックスがソートされている（u < v かつペア間でソート）ため、
        // 座標は「見た目的」にはソートされていないかもしれないが、ソートされたインデックスに対応する。

        let mut bridges = res.bridges;
        // 比較のために正規化（ペア内をソートし、その後ベクトル全体をソート）
        bridges.iter_mut().for_each(|(u, v)| {
            if u > v {
                std::mem::swap(u, v);
            }
        });
        bridges.sort();

        let expected = vec![((0, 0), (0, 1)), ((0, 1), (0, 2)), ((0, 1), (1, 1))];
        assert_eq!(bridges, expected);

        // 関節点: (0,1) が中心。
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
