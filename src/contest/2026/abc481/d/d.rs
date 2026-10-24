// 問題文と制約は読みましたか？
// #[fastout]
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
fn main() {
    input! {
        nv: usize,
        ne: usize,
        src: Usize1,
        dst: Usize1,
        es: [(Usize1, Usize1); ne],
    }

    let mut adj = make_adj_from_directed(nv, &es);

    for nexts in &mut adj {
        nexts.sort();
    }

    let adjc = adj
        .iter()
        .map(|nexts| {
            nexts
                .iter()
                .copied()
                .enumerate()
                .map(|(i, v)| {
                    let cost = if i == 0 { 0 } else { 1 };
                    (v, cost)
                })
                .collect_vec()
            //
        })
        .collect_vec();

    let dist = bfs01(nv, |u| adjc[u].iter().copied(), [src])[dst];
    if let Some(ans) = dist {
        println!("{}", ans);
    } else {
        println!("-1");
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
// 使用したテンプレート・スニペットの公開元:
// https://github.com/paruma/atcoder_rust
use bfs01::*;
#[allow(clippy::module_inception)]
pub mod bfs01 {
    use std::collections::VecDeque;

    /// 01-BFS の結果（距離と復元情報）
    #[derive(Clone, Debug)]
    pub struct Bfs01Result {
        pub dist: Vec<Option<i64>>,
        pub prev: Vec<Option<usize>>,
    }

    impl Bfs01Result {
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

    /// 01-BFS を使って各頂点への最短距離を求める
    ///
    /// # Arguments
    /// * `nv` - 頂点数
    /// * `adj` - 頂点を受け取り、隣接する頂点とそのコストのペアのイテレータを返す `usize -> impl IntoIterator<Item = (usize, i64)>` のクロージャー。コストは 0 または 1 のみ
    /// * `init` - 始点となる頂点集合のイテレータ。1点のみの場合は `[v]` のように指定する
    ///
    /// # Returns
    /// 始点集合 `init` からの最短距離を格納した `Vec<Option<i64>>`。到達不可能な頂点は `None`。
    ///
    /// # 計算量
    /// O(V + E)
    ///
    /// # Examples
    /// ```ignore
    /// let adj = vec![vec![(1, 1), (3, 0)], vec![(2, 0)], vec![], vec![(2, 1)]];
    ///
    /// // 1点を始点にする場合
    /// let dist = bfs01(4, |u| adj[u].iter().copied(), [0]);
    /// assert_eq!(dist, vec![Some(0), Some(1), Some(1), Some(0)]);
    /// ```
    pub fn bfs01<F, It>(
        nv: usize,
        mut adj: F,
        init: impl IntoIterator<Item = usize>,
    ) -> Vec<Option<i64>>
    where
        F: FnMut(usize) -> It,
        It: IntoIterator<Item = (usize, i64)>,
    {
        let mut dist = vec![None; nv];
        let mut q = VecDeque::new();
        for s in init {
            if dist[s].is_none() {
                dist[s] = Some(0);
                q.push_back((s, 0));
            }
        }
        while let Some((u, d)) = q.pop_front() {
            if dist[u].is_some_and(|cur| cur < d) {
                continue;
            }
            for (v, cost) in adj(u) {
                assert!(cost == 0 || cost == 1, "cost must be 0 or 1");
                let next_d = d + cost;
                if dist[v].is_none_or(|cur| cur > next_d) {
                    dist[v] = Some(next_d);
                    if cost == 0 {
                        q.push_front((v, next_d));
                    } else {
                        q.push_back((v, next_d));
                    }
                }
            }
        }
        dist
    }

    /// 01-BFS を使って各頂点への最短距離と経路復元情報を求める
    ///
    /// # Arguments
    /// * `nv` - 頂点数
    /// * `adj` - 頂点を受け取り、隣接する頂点とそのコストのペアのイテレータを返す `usize -> impl IntoIterator<Item = (usize, i64)>` のクロージャー。コストは 0 または 1 のみ
    /// * `init` - 始点となる頂点集合のイテレータ。1点のみの場合は `[v]` のように指定する
    ///
    /// # Returns
    /// 最短距離 `dist` と、復元用配列 `prev` を含む `Bfs01Result`。
    ///
    /// # 計算量
    /// O(V + E)
    ///
    /// # Examples
    /// ```ignore
    /// let adj = vec![vec![(1, 0), (3, 1)], vec![(2, 0)], vec![], vec![(1, 0)]];
    /// let res = bfs01_with_restore(4, |u| adj[u].iter().copied(), [0]);
    ///
    /// assert_eq!(res.dist, vec![Some(0), Some(0), Some(0), Some(1)]);
    /// assert_eq!(res.restore(2), Some(vec![0, 1, 2]));
    /// assert_eq!(res.restore(3), Some(vec![0, 3]));
    /// assert_eq!(res.restore(1), Some(vec![0, 1]));
    /// ```
    pub fn bfs01_with_restore<F, It>(
        nv: usize,
        mut adj: F,
        init: impl IntoIterator<Item = usize>,
    ) -> Bfs01Result
    where
        F: FnMut(usize) -> It,
        It: IntoIterator<Item = (usize, i64)>,
    {
        let mut dist = vec![None; nv];
        let mut prev = vec![None; nv];
        let mut q = VecDeque::new();
        for s in init {
            if dist[s].is_none() {
                dist[s] = Some(0);
                q.push_back((s, 0));
            }
        }
        while let Some((u, d)) = q.pop_front() {
            if dist[u].is_some_and(|cur| cur < d) {
                continue;
            }
            for (v, cost) in adj(u) {
                assert!(cost == 0 || cost == 1, "cost must be 0 or 1");
                let next_d = d + cost;
                if dist[v].is_none_or(|cur| cur > next_d) {
                    dist[v] = Some(next_d);
                    prev[v] = Some(u);
                    if cost == 0 {
                        q.push_front((v, next_d));
                    } else {
                        q.push_back((v, next_d));
                    }
                }
            }
        }
        Bfs01Result { dist, prev }
    }
}
