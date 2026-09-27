// 問題文と制約は読みましたか？

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

#[fastout]
fn main() {
    input! {
        n: usize,
        nq: usize,
        a_s: [i64; n],
        b_s: [i64; n],
        qs: [(Usize1, Usize1); nq],
    }

    let nv = n + 1;

    let mut es = vec![];

    for i in 0..n {
        es.push((i, (i + 1) % n, a_s[i]));
        es.push((i, n, b_s[i]));
    }

    let adj = make_adj_from_weighted_undirected(nv, &es);

    let dist_from_n = dijkstra(nv, |u| adj[u].iter().copied(), [n])
        .iter()
        .copied()
        .map(|d| d.unwrap())
        .collect_vec();

    let a_s2 = chain!(&a_s, &a_s).copied().collect_vec();
    let as_cumsum = CumSum::new(&a_s2);

    for (from, to) in qs {
        // n を経由

        let dist1 = dist_from_n[from] + dist_from_n[to];
        let mut dist2 = i64::MAX;
        if to != n {
            let cand1 = as_cumsum.range_sum(from..to); // 時計
            let cand2 = as_cumsum.range_sum(to..from + n); // 反時計
            dist2 = cand1.min(cand2);
        }

        let ans = dist1.min(dist2);
        println!("{}", ans);
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }

    /// 間違っていたら false を返す
    fn process_one_test(rng: &mut SmallRng) -> bool {
        // ==== 問題を作る ====
        let n = rng.random_range(1..=10);
        let xs = (0..n).map(|_| rng.random_range(0..10)).collect_vec();

        // ==== 解く ====
        let main_ans = xs.len();
        let naive_ans = 1;

        // ==== 間違っていたら報告をする ====
        if main_ans != naive_ans {
            // 問題を出力
            println!("{:?}", (n, xs));
            println!("main ans : {:?}", main_ans);
            println!("naive ans: {:?}", naive_ans);
            return false;
        }
        true
    }

    #[allow(unreachable_code)]
    #[test]
    #[ignore]
    fn test_with_naive() {
        let num_tests = 100;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut cnt_wrong = 0;
        let mut rng = SmallRng::seed_from_u64(42);
        // let mut rng = SmallRng::from_os_rng();
        for _ in 0..num_tests {
            let is_ok = process_one_test(&mut rng);
            if !is_ok {
                cnt_wrong += 1;
            }
            if cnt_wrong >= max_wrong_case {
                break;
            }
        }
        if cnt_wrong > 0 {
            println!("{} cases are wrong.", cnt_wrong);
            panic!();
        }
    }
}

// ====== import ======
#[allow(unused_imports)]
use {
    itertools::{Itertools, chain, iproduct, izip},
    proconio::{
        derive_readable, fastout, input,
        marker::{Bytes, Chars, Usize1},
    },
    rand::{Rng, SeedableRng, rngs::SmallRng, seq::SliceRandom},
    std::{
        cmp::Reverse,
        collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet},
    },
};

// ====== output func ======
#[allow(unused_imports)]
use print_util::*;
pub mod print_util {
    use itertools::Itertools;
    use proconio::fastout;

    #[fastout]
    pub fn print_vec<T: std::fmt::Display>(arr: &[T]) {
        for a in arr {
            println!("{}", a);
        }
    }

    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
        println!("{}", arr.iter().join(" "));
    }

    #[fastout]
    pub fn print_vec2<T: std::fmt::Display, R: AsRef<[T]>>(arr: &[R]) {
        for row in arr {
            println!("{}", row.as_ref().iter().join(" "));
        }
    }

    pub fn print_bytes(bytes: &[u8]) {
        println!("{}", std::str::from_utf8(bytes).unwrap());
    }

    pub fn print_chars(chars: &[char]) {
        println!("{}", chars.iter().collect::<String>());
    }

    #[fastout]
    pub fn print_vec_bytes<R: AsRef<[u8]>>(vec_bytes: &[R]) {
        for row in vec_bytes {
            println!("{}", std::str::from_utf8(row.as_ref()).unwrap());
        }
    }

    #[fastout]
    pub fn print_vec_chars<R: AsRef<[char]>>(vec_chars: &[R]) {
        for row in vec_chars {
            println!("{}", row.as_ref().iter().collect::<String>());
        }
    }

    pub fn print_yesno(ans: bool) {
        println!("{}", if ans { "Yes" } else { "No" });
    }
}

// ====== snippet ======
use dijkstra::*;
#[allow(clippy::module_inception)]
pub mod dijkstra {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    /// ダイクストラ法の実行結果（最短距離と経路復元情報）を保持する構造体です。
    #[derive(Clone, Debug)]
    pub struct DijkstraResult {
        /// 各頂点への最短距離です。到達不可能な場合は `None` となります。
        pub dist: Vec<Option<i64>>,
        /// 経路復元用の親頂点インデックスです。
        pub prev: Vec<Option<usize>>,
    }
    impl DijkstraResult {
        /// 頂点 `t` への最短経路を復元する（始点 -> ... -> t）
        ///
        /// # Returns
        /// 始点から `t` までの頂点列。`t` に到達不可能な場合は `None`。
        ///
        /// # 計算量
        /// O(経路の長さ)
        pub fn restore(&self, t: usize) -> Option<Vec<usize>> {
            self.dist[t]?;
            let mut path: Vec<_> =
                std::iter::successors(Some(t), |&curr| self.prev[curr]).collect();
            path.reverse();
            Some(path)
        }
    }
    /// ダイクストラ法を使って各頂点への最短距離を求める
    ///
    /// # Arguments
    /// * `nv` - 頂点数
    /// * `adj` - 頂点を受け取り、隣接する頂点とそのコストのペアのイテレータを返す `usize -> impl IntoIterator<Item = (usize, i64)>` のクロージャー。コストは非負
    /// * `init` - 始点となる頂点集合のイテレータ。1点のみの場合は `[v]` のように指定する
    ///
    /// # Returns
    /// 始点集合 `init` からの最短距離を格納した `Vec<Option<i64>>`。到達不可能な頂点は `None`。
    ///
    /// # 計算量
    /// O(V + E log V)
    ///
    /// # Examples
    /// ```ignore
    /// let adj = vec![vec![(1, 10), (2, 3)], vec![(2, 1)], vec![(1, 5)]];
    ///
    /// // 1点を始点にする場合
    /// let dist = dijkstra(3, |u| adj[u].iter().copied(), [0]);
    /// assert_eq!(dist, vec![Some(0), Some(8), Some(3)]);
    /// ```
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
    /// ダイクストラ法を使って各頂点への最短距離と経路復元情報を求める
    ///
    /// # Arguments
    /// * `nv` - 頂点数
    /// * `adj` - 頂点を受け取り、隣接する頂点とそのコストのペアのイテレータを返す `usize -> impl IntoIterator<Item = (usize, i64)>` のクロージャー。コストは非負
    /// * `init` - 始点となる頂点集合のイテレータ。1点のみの場合は `[v]` のように指定する
    ///
    /// # Returns
    /// 最短距離 `dist` と、復元用配列 `prev` を含む `DijkstraResult`。
    ///
    /// # 計算量
    /// O(V + E log V)
    ///
    /// # Examples
    /// ```ignore
    /// let adj = vec![vec![(1, 10), (3, 4)], vec![(2, 5)], vec![], vec![(1, 2)]];
    /// let res = dijkstra_with_restore(4, |u| adj[u].iter().copied(), [0]);
    ///
    /// assert_eq!(res.dist, vec![Some(0), Some(6), Some(11), Some(4)]);
    /// assert_eq!(res.restore(2), Some(vec![0, 3, 1, 2]));
    /// assert_eq!(res.restore(1), Some(vec![0, 3, 1]));
    /// assert_eq!(res.restore(3), Some(vec![0, 3]));
    /// assert_eq!(res.restore(0), Some(vec![0]));
    /// ```
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
use cumsum::*;
#[allow(clippy::module_inception)]
pub mod cumsum {
    pub fn prefix_sum(xs: &[i64]) -> Vec<i64> {
        let mut prefix_sum = vec![0; xs.len() + 1];
        for i in 1..xs.len() + 1 {
            prefix_sum[i] = prefix_sum[i - 1] + xs[i - 1];
        }
        prefix_sum
    }
    use std::ops::{Bound, Range, RangeBounds};
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct CumSum {
        pub cumsum: Vec<i64>,
    }
    impl CumSum {
        /// # 計算量
        /// O(|xs|)
        pub fn new(xs: &[i64]) -> CumSum {
            let mut cumsum = vec![0; xs.len() + 1];
            for i in 1..xs.len() + 1 {
                cumsum[i] = cumsum[i - 1] + xs[i - 1];
            }
            CumSum { cumsum }
        }
        fn open(&self, range: impl RangeBounds<usize>) -> Range<usize> {
            use Bound::Excluded;
            use Bound::Included;
            use Bound::Unbounded;
            let begin = match range.start_bound() {
                Unbounded => 0,
                Included(&x) => x,
                Excluded(&x) => x + 1,
            };
            let end = match range.end_bound() {
                Excluded(&x) => x,
                Included(&x) => x + 1,
                Unbounded => self.cumsum.len() - 1,
            };
            begin..end
        }
        /// 区間 `[begin, end)` の要素の和を計算します。
        ///
        /// # 計算量
        /// O(1)
        pub fn range_sum(&self, range: impl RangeBounds<usize>) -> i64 {
            let range = self.open(range);
            self.cumsum[range.end] - self.cumsum[range.start]
        }
        /// 区間 `[0, end)` での和を計算します。
        ///
        /// # 計算量
        /// O(1)
        pub fn prefix_sum(&self, end: usize) -> i64 {
            self.cumsum[end]
        }
        /// 区間 `[begin, n)` の要素の和を計算します。（`n` は元の配列の長さ）
        ///
        /// # 計算量
        /// O(1)
        pub fn suffix_sum(&self, begin: usize) -> i64 {
            self.cumsum[self.cumsum.len() - 1] - self.cumsum[begin]
        }
        /// `f(sum(l..r))` が `true` となる最大の `r in [l, n]` を見つける。
        ///
        /// `n` は元の配列の長さ。
        ///
        /// # 前提条件
        /// * `l <= n`
        /// * `f(0)` が `true`
        /// * `f` は単調である。
        ///     * `f(sum(l..i))` が `true` => `f(sum(l..j))` が `true` for all `l <= j <= i`.
        ///
        /// # 計算量
        /// O(log n)
        pub fn max_right<F>(&self, l: usize, mut f: F) -> usize
        where
            F: FnMut(i64) -> bool,
        {
            let n = self.cumsum.len() - 1;
            assert!(l <= n);
            assert!(f(0), "f(0) must be true");
            if f(self.range_sum(l..n)) {
                return n;
            }
            let mut ok = l;
            let mut ng = n + 1;
            while ng - ok > 1 {
                let mid = ok + (ng - ok) / 2;
                if f(self.range_sum(l..mid)) {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok
        }
        /// `f(sum(l..r))` が `true` となる最小の `l in [0, r]` を見つける。
        ///
        /// # 前提条件
        /// * `r <= n`
        /// * `f(0)` が `true`
        /// * `f` は単調である。
        ///     * `f(sum(i..r))` が `true` => `f(sum(j..r))` が `true` for all `i <= j <= r`.
        ///
        /// # 計算量
        /// O(log r)
        pub fn min_left<F>(&self, r: usize, mut f: F) -> usize
        where
            F: FnMut(i64) -> bool,
        {
            let n = self.cumsum.len() - 1;
            assert!(r <= n);
            assert!(f(0), "f(0) must be true");
            if f(self.range_sum(0..r)) {
                return 0;
            }
            let mut ok = r;
            let mut ng = 0;
            while ok - ng > 1 {
                let mid = ng + (ok - ng) / 2;
                if f(self.range_sum(mid..r)) {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok
        }
    }
}
