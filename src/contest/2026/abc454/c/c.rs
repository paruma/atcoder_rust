// 問題文と制約は読みましたか？
// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abs: [(Usize1, Usize1); m],
    }

    let adj = abs
        .iter()
        .copied()
        .fold(vec![vec![]; n], |mut acc, (a, b)| {
            acc[a].push(b);
            acc
        });

    let reachable = bfs_reachable(n, |u| adj[u].iter().copied(), [0]);
    let ans: usize = reachable.iter().copied().filter(|p| *p).count();
    println!("{}", ans);
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
use bfs::*;
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
        /// # Returns
        /// 始点から `t` までの頂点列。`t` に到達不可能な場合は `None`。
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
    /// 幅優先探索 (BFS) で、各頂点への最短距離を求める
    /// # Arguments
    /// * `nv` - 頂点数
    /// * `adj` - 頂点を受け取り、隣接する頂点のイテレータを返す `usize -> impl IntoIterator<Item = usize>` のクロージャー
    /// * `init` - 始点となる頂点集合のイテレータ。1点のみの場合は `[v]` のように指定する
    /// # Returns
    /// 始点集合 `init` からの最短距離を格納した `Vec<Option<i64>>`。到達不可能な頂点は `None`。
    /// # 計算量
    /// O(V + E)
    /// # Examples
    /// ```ignore
    /// let adj = vec![vec![1], vec![0, 2, 3], vec![1], vec![1]];
    /// // 1点を始点にする場合
    /// let dist = bfs(4, |u| adj[u].iter().copied(), [0]);
    /// assert_eq!(dist, vec![Some(0), Some(1), Some(2), Some(2)]);
    /// // 複数点を始点にする場合
    /// let starts = vec![0, 3];
    /// let dist = bfs(4, |u| adj[u].iter().copied(), starts.iter().copied());
    /// assert_eq!(dist, vec![Some(0), Some(1), Some(2), Some(0)]);
    /// ```
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
    /// 幅優先探索 (BFS) で、各頂点への最短距離と経路復元情報を求める
    /// # Arguments
    /// * `nv` - 頂点数
    /// * `adj` - 頂点を受け取り、隣接する頂点のイテレータを返す `usize -> impl IntoIterator<Item = usize>` のクロージャー
    /// * `init` - 始点となる頂点集合のイテレータ。1点のみの場合は `[v]` のように指定する
    /// # Returns
    /// 最短距離 `dist` と、復元用配列 `prev` を含む `BfsResult`。
    /// # 計算量
    /// O(V + E)
    /// # Examples
    /// ```ignore
    /// let adj = vec![vec![1, 3], vec![2], vec![], vec![]];
    /// let res = bfs_with_restore(4, |u| adj[u].iter().copied(), [0]);
    /// assert_eq!(res.dist, vec![Some(0), Some(1), Some(2), Some(1)]);
    /// assert_eq!(res.restore(2), Some(vec![0, 1, 2]));
    /// assert_eq!(res.restore(3), Some(vec![0, 3]));
    /// assert_eq!(res.restore(0), Some(vec![0]));
    /// ```
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
    /// 幅優先探索 (BFS) での訪問順序（キューに入れた順）を求める
    /// # Arguments
    /// * `nv` - 頂点数
    /// * `adj` - 頂点を受け取り、隣接する頂点のイテレータを返す `usize -> impl IntoIterator<Item = usize>` のクロージャー
    /// * `init` - 始点となる頂点集合のイテレータ。1点のみの場合は `[v]` のように指定する
    /// # Returns
    /// 到達可能な頂点を訪問順に格納した `Vec<usize>`
    /// # 計算量
    /// O(V + E)
    /// # Examples
    /// ```ignore
    /// let adj = vec![vec![1, 3], vec![2], vec![], vec![]];
    /// let order = bfs_order(4, |u| adj[u].iter().copied(), [0]);
    /// assert_eq!(order, vec![0, 1, 3, 2]);
    /// ```
    pub fn bfs_order<F, It>(
        nv: usize,
        mut adj: F,
        init: impl IntoIterator<Item = usize>,
    ) -> Vec<usize>
    where
        F: FnMut(usize) -> It,
        It: IntoIterator<Item = usize>,
    {
        let mut visited = vec![false; nv];
        let mut order = Vec::new();
        let mut q = VecDeque::new();
        for s in init {
            if !visited[s] {
                visited[s] = true;
                order.push(s);
                q.push_back(s);
            }
        }
        while let Some(u) = q.pop_front() {
            for v in adj(u) {
                if !visited[v] {
                    visited[v] = true;
                    order.push(v);
                    q.push_back(v);
                }
            }
        }
        order
    }
    /// 標準的な usize インデックスを用いた幅優先探索 (BFS) で、各頂点への到達可能性を判定する
    /// # Arguments
    /// * `nv` - 頂点数
    /// * `adj` - 頂点を受け取り、隣接する頂点のイテレータを返す `usize -> impl IntoIterator<Item = usize>` のクロージャー
    /// * `init` - 始点となる頂点集合のイテレータ。1点のみの場合は `[v]` のように指定する
    /// # Returns
    /// 各頂点への到達可能性を格納した `Vec<bool>`
    /// # 計算量
    /// O(V + E)
    /// # Examples
    /// ```ignore
    /// let adj = vec![vec![1], vec![2], vec![], vec![4], vec![]];
    /// let reachable = bfs_reachable(5, |u| adj[u].iter().copied(), [0]);
    /// assert_eq!(reachable, vec![true, true, true, false, false]);
    /// ```
    pub fn bfs_reachable<F, It>(
        nv: usize,
        mut adj: F,
        init: impl IntoIterator<Item = usize>,
    ) -> Vec<bool>
    where
        F: FnMut(usize) -> It,
        It: IntoIterator<Item = usize>,
    {
        let mut visited = vec![false; nv];
        let mut q = VecDeque::new();
        for s in init {
            if !visited[s] {
                visited[s] = true;
                q.push_back(s);
            }
        }
        while let Some(u) = q.pop_front() {
            for v in adj(u) {
                if !visited[v] {
                    visited[v] = true;
                    q.push_back(v);
                }
            }
        }
        visited
    }
}
