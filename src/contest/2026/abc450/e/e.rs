fn ctoi(ch: char) -> usize {
    (ch as u8 - b'a') as usize
}

// こうの方がいいかも
// fn ctoi(ch: char) -> usize {
//     ch as usize - 'a' as usize
// }

struct Rec<'a> {
    fibs: &'a [usize],
    dp2: &'a [Vec<i64>],
    yx_counts: &'a [CumSum],
    yx_len: usize,
}

impl<'a> Rec<'a> {
    // [0, n) での ch の個数を求める。
    fn rec(&self, ch: usize, n: usize) -> i64 {
        if n == 0 {
            return 0;
        }
        if n <= self.yx_len {
            return self.yx_counts[ch].range_sum(0..n);
        }
        // fibs が 2 3 5 8 ..
        // 9 だったら8がほしい
        // 8 だったら8がほしい
        let prefix_i = self.fibs.range_max_index(..=n).unwrap();
        let suffix = n - self.fibs[prefix_i];

        let prefix_ans = self.dp2[ch][prefix_i];
        let suffix_ans = self.rec(ch, suffix);

        prefix_ans + suffix_ans
    }
}

// 問題文と制約は読みましたか？
#[fastout]
fn main() {
    input! {
        x: Chars,
        y: Chars,
        q: usize,
        qs: [(Usize1, Usize1, char); q],
    }
    let yx = chain!(&y, &x).copied().collect_vec();
    let yx_counts = ('a'..='z')
        .map(|target| {
            CumSum::new(
                &yx.iter()
                    .copied()
                    .map(|ch| (ch == target) as i64)
                    .collect_vec(),
            )
        })
        .collect_vec();
    // y + x をベースにして考える
    let fibs = {
        let mut fibs = vec![yx.len(), y.len() + yx.len()];
        for i in 2..100 {
            let added = fibs[i - 1].saturating_add(fibs[i - 2]);
            if added > 1_000_000_000_000_000_001 {
                break;
            }
            fibs.push(added);
        }
        fibs
    };

    // dp2[ch][i] = f(ch, fibs[i])) つまり [0, fibs[i]) までにある ch の数

    let mut dp2 = vec![vec![i64::MIN; fibs.len()]; 26];

    for ch in 0..26 {
        dp2[ch][0] = yx_counts[ch].range_sum(..);
        dp2[ch][1] = dp2[ch][0] + yx_counts[ch].range_sum(..y.len());
        for i in 2..fibs.len() {
            dp2[ch][i] = dp2[ch][i - 1] + dp2[ch][i - 2];
        }
    }
    let rec = Rec {
        fibs: &fibs,
        dp2: &dp2,
        yx_counts: &yx_counts,
        yx_len: yx.len(),
    };

    for (l, r, ch) in qs {
        let ch = ctoi(ch);
        // [l, r] = [l, r + 1)
        let sub_ans = rec.rec(ch, r + 1) - rec.rec(ch, l);
        println!("{}", sub_ans);
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
        /// # 計算量
        /// O(1)
        pub fn range_sum(&self, range: impl RangeBounds<usize>) -> i64 {
            let range = self.open(range);
            self.cumsum[range.end] - self.cumsum[range.start]
        }
        /// 区間 `[0, end)` での和を計算します。
        /// # 計算量
        /// O(1)
        pub fn prefix_sum(&self, end: usize) -> i64 {
            self.cumsum[end]
        }
        /// 区間 `[begin, n)` の要素の和を計算します。（`n` は元の配列の長さ）
        /// # 計算量
        /// O(1)
        pub fn suffix_sum(&self, begin: usize) -> i64 {
            self.cumsum[self.cumsum.len() - 1] - self.cumsum[begin]
        }
        /// `f(sum(l..r))` が `true` となる最大の `r in [l, n]` を見つける。
        /// `n` は元の配列の長さ。
        /// `f` は単調でなければならない。
        /// `f(sum(l..i))` が `true` => `f(sum(l..j))` が `true` for all `l <= j <= i`.
        /// # Panics
        /// `l > n` の場合にパニックする。
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
        /// `f` は単調でなければならない。
        /// `f(sum(i..r))` が `true` => `f(sum(j..r))` が `true` for all `i <= j <= r`.
        /// `r > n` の場合にパニックする。
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
