// コンテスト中の解法
// 二分探索を2回するため、計算量に log が2つついて TL ギリギリ
// super_slice ではなく、標準ライブラリの二分探索のラッパーを利用
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a_s: [i64; n],
    }

    a_s.sort();

    for _ in 0..q {
        input! {
            x: i64,
            y: i64,
        }
        // x..=cand で a_s に含まれている数を cnt とする。
        // x..=cand で a_s に含まれていない数は cand - x + 1 - cnt である。
        // cand - x + 1 - cnt >= y となる最小の cand を求める
        let ans = bin_search(10_000_000_000, x - 1, |cand| {
            let cnt = a_s.range_count(x..=cand) as i64;
            cand - x + 1 - cnt >= y
        });
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
use superslice::Ext;
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
        collections::{BinaryHeap, HashMap, HashSet},
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
        let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Display>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{}", x)).join(" ");
            println!("{}", msg);
        }
    }
    pub fn print_bytes(bytes: &[u8]) {
        let msg = String::from_utf8(bytes.to_vec()).unwrap();
        println!("{}", msg);
    }
    pub fn print_chars(chars: &[char]) {
        let msg = chars.iter().collect::<String>();
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec_bytes(vec_bytes: &[Vec<u8>]) {
        for row in vec_bytes {
            let msg = String::from_utf8(row.to_vec()).unwrap();
            println!("{}", msg);
        }
    }
    #[fastout]
    pub fn print_vec_chars(vec_chars: &[Vec<char>]) {
        for row in vec_chars {
            let msg = row.iter().collect::<String>();
            println!("{}", msg);
        }
    }
    pub fn print_yesno(ans: bool) {
        let msg = if ans { "Yes" } else { "No" };
        println!("{}", msg);
    }
}

// ====== snippet ======
/// 二分探索をする。
/// ```text
/// ng ng ng ok ok ok
///          ↑ここの引数の値を返す
/// ```
/// # 計算量
/// O(log(|ok - ng|))
/// ## Arguments
/// * ok != ng
/// * |ok - ng| <= 2^63 - 1, |ok + ng| <= 2^63 - 1
/// * p の定義域について
///     * ng < ok の場合、p は区間 ng..ok で定義されている。
///     * ok < ng の場合、p は区間 ok..ng で定義されている。
/// * p の単調性について
///     * ng < ok の場合、p は単調増加
///     * ok < ng の場合、p は単調減少
/// ## Return
/// * ng < ok の場合: I = { i in ng..ok | p(i) == true } としたとき
///     * I が空でなければ、min I を返す。
///     * I が空ならば、ok を返す。
/// * ok < ng の場合: I = { i in ok..ng | p(i) == true } としたとき
///     * I が空でなければ、max I を返す。
///     * I が空ならば、ok を返す。
pub fn bin_search<F>(mut ok: i64, mut ng: i64, mut p: F) -> i64
where
    F: FnMut(i64) -> bool,
{
    debug_assert!(ok != ng);
    debug_assert!(ok.checked_sub(ng).is_some());
    debug_assert!(ok.checked_add(ng).is_some());
    while num::abs(ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        debug_assert!(mid != ok);
        debug_assert!(mid != ng);
        if p(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}
use sorted_slice::*;
#[allow(clippy::module_inception)]
pub mod sorted_slice {
    use std::ops::{Bound::*, Range, RangeBounds};
    /// ソート済みスライスに対する区間クエリを提供するトレイト。
    pub trait SortedSliceExt<T: Ord> {
        fn range_indices<R: RangeBounds<T>>(&self, range: R) -> Range<usize>;
        fn range_count<R: RangeBounds<T>>(&self, range: R) -> usize;
        fn range_min_index<R: RangeBounds<T>>(&self, range: R) -> Option<usize>;
        fn range_max_index<R: RangeBounds<T>>(&self, range: R) -> Option<usize>;
    }
    impl<T: Ord> SortedSliceExt<T> for [T] {
        /// `range` に含まれる要素のインデックス範囲 `[begin, end)` を返す。
        /// # 前提条件
        /// * `self`: 広義単調増加（ソート済み）であること
        /// # 計算量
        /// $O(\log N)$（$N$ は `self` の長さ）
        fn range_indices<R: RangeBounds<T>>(&self, range: R) -> Range<usize> {
            let begin = match range.start_bound() {
                Included(lo) => self.partition_point(|x| x < lo),
                Excluded(lo) => self.partition_point(|x| x <= lo),
                Unbounded => 0,
            };
            let end = match range.end_bound() {
                Included(hi) => self.partition_point(|x| x <= hi),
                Excluded(hi) => self.partition_point(|x| x < hi),
                Unbounded => self.len(),
            };
            begin..end.max(begin)
        }
        /// `range` に含まれる要素の個数を返す。
        /// # 前提条件
        /// * `self`: 広義単調増加（ソート済み）であること
        /// # 計算量
        /// $O(\log N)$（$N$ は `self` の長さ）
        fn range_count<R: RangeBounds<T>>(&self, range: R) -> usize {
            self.range_indices(range).len()
        }
        /// `self[i] ∈ range` を満たす最小の `i` を返す。存在しない場合は `None`。
        /// # 前提条件
        /// * `self`: 広義単調増加（ソート済み）であること
        /// # 計算量
        /// $O(\log N)$（$N$ は `self` の長さ）
        fn range_min_index<R: RangeBounds<T>>(&self, range: R) -> Option<usize> {
            let r = self.range_indices(range);
            if r.is_empty() { None } else { Some(r.start) }
        }
        /// `self[i] ∈ range` を満たす最大の `i` を返す。存在しない場合は `None`。
        /// # 前提条件
        /// * `self`: 広義単調増加（ソート済み）であること
        /// # 計算量
        /// $O(\log N)$（$N$ は `self` の長さ）
        fn range_max_index<R: RangeBounds<T>>(&self, range: R) -> Option<usize> {
            let r = self.range_indices(range);
            if r.is_empty() { None } else { Some(r.end - 1) }
        }
    }
}
