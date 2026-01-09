use cargo_snippet::snippet;

#[snippet(prefix = "use dijkstra::*;")]
#[allow(clippy::module_inception)]
pub mod dijkstra {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    /// ダイクストラ法の結果（距離と復元情報）
    #[derive(Clone, Debug)]
    pub struct DijkstraResult {
        pub dist: Vec<Option<i64>>,
        pub prev: Vec<Option<usize>>,
    }

    impl DijkstraResult {
        /// 頂点 `t` への最短経路を復元する（始点 -> ... -> t）
        pub fn restore(&self, t: usize) -> Option<Vec<usize>> {
            self.dist[t]?;
            let mut path: Vec<_> = std::iter::successors(Some(t), |&curr| self.prev[curr]).collect();
            path.reverse();
            Some(path)
        }
    }

    /// 標準的なダイクストラ法
    ///
    /// # Arguments
    /// * `nv` - 頂点数
    /// * `adj` - 頂点を受け取り、隣接する頂点とそのコストのペアのイテレータを返すクロージャー
    /// * `init` - 始点となる頂点集合のイテレータ
    ///
    /// # Returns
    /// 始点集合 `init` からの最短距離を格納した `Vec<Option<i64>>`。到達不可能な頂点は `None`。
    pub fn dijkstra<F, It>(
        nv: usize,
        mut adj: F,
        init: impl IntoIterator<Item = usize>,
    ) -> Vec<Option<i64>>
    where
        F: FnMut(usize) -> It,
        It: IntoIterator<Item = (usize, i64)>,
    {
        let mut dist = vec![None; nv];
        let mut pq = BinaryHeap::new();
        for s in init {
            if dist[s].is_none() {
                dist[s] = Some(0);
                pq.push(Reverse((0, s)));
            }
        }
        while let Some(Reverse((d, u))) = pq.pop() {
            if dist[u].is_some_and(|cur| cur < d) {
                continue;
            }
            for (v, cost) in adj(u) {
                assert!(cost >= 0, "cost must be non-negative");
                let next_d = d + cost;
                if dist[v].is_none_or(|cur| cur > next_d) {
                    dist[v] = Some(next_d);
                    pq.push(Reverse((next_d, v)));
                }
            }
        }
        dist
    }

    /// 経路復元可能なダイクストラ法
    ///
    /// # Returns
    /// 最短距離 `dist` と、復元用配列 `prev` を含む `DijkstraResult`。
    pub fn dijkstra_with_restore<F, It>(
        nv: usize,
        mut adj: F,
        init: impl IntoIterator<Item = usize>,
    ) -> DijkstraResult
    where
        F: FnMut(usize) -> It,
        It: IntoIterator<Item = (usize, i64)>,
    {
        let mut dist = vec![None; nv];
        let mut prev = vec![None; nv];
        let mut pq = BinaryHeap::new();
        for s in init {
            if dist[s].is_none() {
                dist[s] = Some(0);
                pq.push(Reverse((0, s)));
            }
        }
        while let Some(Reverse((d, u))) = pq.pop() {
            if dist[u].is_some_and(|cur| cur < d) {
                continue;
            }
            for (v, cost) in adj(u) {
                assert!(cost >= 0, "cost must be non-negative");
                let next_d = d + cost;
                if dist[v].is_none_or(|cur| cur > next_d) {
                    dist[v] = Some(next_d);
                    prev[v] = Some(u);
                    pq.push(Reverse((next_d, v)));
                }
            }
        }
        DijkstraResult { dist, prev }
    }
}

#[snippet(prefix = "use dijkstra_ix::*;")]
pub mod dijkstra_ix {
    use super::dijkstra::{dijkstra, dijkstra_with_restore};
    use crate::data_structure::ix::{Bounds, Ix, IxVec};

    /// ダイクストラ法の結果（Ix版）
    #[derive(Clone, Debug)]
    pub struct DijkstraIxResult<I: Ix> {
        pub dist: IxVec<I, Option<i64>>,
        pub prev: IxVec<I, Option<I>>,
    }

    impl<I: Ix> DijkstraIxResult<I> {
        pub fn restore(&self, t: I) -> Option<Vec<I>> {
            self.dist[t]?;
            let mut path: Vec<_> = std::iter::successors(Some(t), |&curr| self.prev[curr]).collect();
            path.reverse();
            Some(path)
        }
    }

    /// Bounds を用いた任意の型 I に対するダイクストラ法
    pub fn dijkstra_arbitrary<I, F, It>(
        bounds: Bounds<I>,
        mut adj: F,
        init: impl IntoIterator<Item = I>,
    ) -> IxVec<I, Option<i64>>
    where
        I: Ix,
        F: FnMut(I) -> It,
        It: IntoIterator<Item = (I, i64)>,
    {
        let nv = bounds.range_size();
        let mut adj_usize = |u_idx: usize| {
            let u = bounds.from_index(u_idx);
            adj(u)
                .into_iter()
                .map(move |(v, cost)| (bounds.to_index(v), cost))
        };
        let init_usize = init.into_iter().map(|s| bounds.to_index(s));
        let res_vec = dijkstra(nv, &mut adj_usize, init_usize);
        IxVec::from_vec(bounds, res_vec)
    }

    /// Bounds を用いた任意の型 I に対するダイクストラ法 (経路復元付き)
    pub fn dijkstra_with_restore_arbitrary<I, F, It>(
        bounds: Bounds<I>,
        mut adj: F,
        init: impl IntoIterator<Item = I>,
    ) -> DijkstraIxResult<I>
    where
        I: Ix,
        F: FnMut(I) -> It,
        It: IntoIterator<Item = (I, i64)>,
    {
        let nv = bounds.range_size();
        let mut adj_usize = |u_idx: usize| {
            let u = bounds.from_index(u_idx);
            adj(u)
                .into_iter()
                .map(move |(v, cost)| (bounds.to_index(v), cost))
        };
        let init_usize = init.into_iter().map(|s| bounds.to_index(s));
        let res = dijkstra_with_restore(nv, &mut adj_usize, init_usize);

        DijkstraIxResult {
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

pub use dijkstra::*;
pub use dijkstra_ix::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_structure::ix::Bounds;

    #[test]
    fn test_dijkstra_basic() {
        let adj = [vec![(1, 10), (2, 3)], vec![(2, 1)], vec![(1, 5)]];
        let res = dijkstra(3, |u| adj[u].iter().copied(), [0]);
        assert_eq!(res, vec![Some(0), Some(8), Some(3)]);
    }

    #[test]
    fn test_dijkstra_arbitrary() {
        let bounds = Bounds::new(0, 2);
        let adj = [vec![(1, 10)], vec![(2, 5)], vec![]];
        let res = dijkstra_arbitrary(bounds, |u| adj[u].iter().copied(), [0]);
        assert_eq!(res[0], Some(0));
        assert_eq!(res[1], Some(10));
        assert_eq!(res[2], Some(15));
    }

    #[test]
    fn test_dijkstra_restore() {
        // 0 -(10)-> 1 -(5)-> 2
        // |         ^
        // (4)       | (2)
        // v         |
        // 3 --------+
        let adj = [vec![(1, 10), (3, 4)], vec![(2, 5)], vec![], vec![(1, 2)]];
        let res = dijkstra_with_restore(4, |u| adj[u].iter().copied(), [0]);

        assert_eq!(res.dist, vec![Some(0), Some(6), Some(11), Some(4)]);
        // 0 -> 3 (cost 4) -> 1 (cost 2) = total 6
        // 0 -> 1 (cost 10)
        assert_eq!(res.restore(2), Some(vec![0, 3, 1, 2]));
        assert_eq!(res.restore(1), Some(vec![0, 3, 1]));
        assert_eq!(res.restore(3), Some(vec![0, 3]));
        assert_eq!(res.restore(0), Some(vec![0]));
    }

    #[test]
    fn test_dijkstra_restore_arbitrary() {
        let bounds = Bounds::new(0, 2);
        // 0 -(10)-> 1 -(5)-> 2
        let adj = [vec![(1, 10)], vec![(2, 5)], vec![]];
        let res = dijkstra_with_restore_arbitrary(bounds, |u| adj[u].iter().copied(), [0]);

        assert_eq!(res.dist[2], Some(15));
        assert_eq!(res.restore(2), Some(vec![0, 1, 2]));
    }
}
