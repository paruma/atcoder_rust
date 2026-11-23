// 問題文と制約は読みましたか？
// #[fastout]
fn main() {
    input! {
        n: usize,
        L: i64,
        R: i64,
        xs: [i64; n],
    }
    let ys = prefix_sum(&xs);
    let mut bag = BucketMultiSet::new();

    let mut ans = 0;

    for r in 0..=n {
        let sub = bag.count_in_range(ys[r] - R..=ys[r] - L);
        ans += sub;

        bag.insert(ys[r]);
    }

    println!("{}", ans);
    assert!(proconio::is_stdin_empty());
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
                Unbounded => self.cumsum.len() - 1, // xs.len() == self.cumsum.len() - 1
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
use bucket_multiset::*;
#[allow(clippy::module_inception)]
pub mod bucket_multiset {
    use std::ops::{Bound, Range, RangeBounds};

    /// ソート済みバケットで管理する、挿入候補の事前登録が不要な多重集合。
    ///
    /// [tatyam の SortedMultiset](https://github.com/tatyam-prime/SortedSet/blob/main/SortedMultiset.py)
    /// と同様に、長くなったバケットを分割し、空になったバケットを取り除きます。
    /// バケット数を $B$、最長バケットの長さを $L$ とすると、
    /// 順位取得は $O(B)$、範囲検索は $O(B + \log L)$、1 要素の更新は $O(B + L)$ です。
    /// 大量削除後は、現在の要素数に対してバケット数が多く残ることがあります。
    ///
    /// 各メソッドの計算量でも $B$ と $L$ は同じ意味で使い、要素の比較・移動・破棄を定数時間とします。
    #[derive(Clone, Debug)]
    pub struct BucketMultiSet<T> {
        buckets: Vec<Vec<T>>,
        length: usize,
        set_length: usize,
    }

    impl<T> Default for BucketMultiSet<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T> BucketMultiSet<T> {
        /// 空の多重集合を作ります。
        ///
        /// # 計算量
        /// $O(1)$。
        pub fn new() -> Self {
            Self {
                buckets: Vec::new(),
                length: 0,
                set_length: 0,
            }
        }

        /// 重複を含む要素数を返します。
        ///
        /// # 計算量
        /// $O(1)$。
        pub fn len(&self) -> usize {
            self.length
        }

        /// 異なる要素の種類数を返します。
        ///
        /// # 計算量
        /// $O(1)$。
        pub fn set_len(&self) -> usize {
            self.set_length
        }

        /// 空かどうかを返します。
        ///
        /// # 計算量
        /// $O(1)$。
        pub fn is_empty(&self) -> bool {
            self.length == 0
        }

        /// すべての要素を削除します。
        ///
        /// # 計算量
        /// $O(N)$（$N$ は削除前の全要素数）。
        pub fn clear(&mut self) {
            *self = Self::new();
        }

        /// 最小の要素を返します。空の場合は `None` を返します。
        ///
        /// # 計算量
        /// $O(1)$。
        pub fn min(&self) -> Option<&T> {
            self.buckets.first()?.first()
        }

        /// 最大の要素を返します。空の場合は `None` を返します。
        ///
        /// # 計算量
        /// $O(1)$。
        pub fn max(&self) -> Option<&T> {
            self.buckets.last()?.last()
        }

        /// 重複を含めて、0 始まりで小さい方から `n` 番目の要素を返します。
        ///
        /// # 計算量
        /// $O(B)$。
        pub fn nth_min(&self, mut n: usize) -> Option<&T> {
            for bucket in &self.buckets {
                if n < bucket.len() {
                    return Some(&bucket[n]);
                }
                n -= bucket.len();
            }
            None
        }

        /// 重複を含めて、0 始まりで大きい方から `n` 番目の要素を返します。
        ///
        /// # 計算量
        /// $O(B)$。
        pub fn nth_max(&self, mut n: usize) -> Option<&T> {
            for bucket in self.buckets.iter().rev() {
                if n < bucket.len() {
                    return Some(&bucket[bucket.len() - 1 - n]);
                }
                n -= bucket.len();
            }
            None
        }
    }

    impl<T: Ord> BucketMultiSet<T> {
        const SPLIT_RATIO: usize = 24;

        /// 要素を 1 個追加します。
        ///
        /// # 計算量
        /// $O(B + L)$。
        pub fn insert(&mut self, value: T) {
            if self.is_empty() {
                self.buckets.push(vec![value]);
                self.length = 1;
                self.set_length = 1;
                return;
            }

            let bucket_index = self
                .buckets
                .iter()
                .position(|bucket| bucket.last().unwrap() >= &value)
                .unwrap_or_else(|| self.buckets.len() - 1);

            let split_limit = self.buckets.len().saturating_mul(Self::SPLIT_RATIO);
            let bucket = &mut self.buckets[bucket_index];
            let index = bucket.partition_point(|entry| entry < &value);
            self.set_length += usize::from(bucket.get(index) != Some(&value));
            bucket.insert(index, value);
            self.length += 1;

            if bucket.len() > split_limit {
                let right = bucket.split_off(bucket.len() / 2);
                self.buckets.insert(bucket_index + 1, right);
            }
        }

        /// 要素を 1 個削除し、削除できたかを返します。
        ///
        /// # 計算量
        /// $O(B + L)$。
        pub fn remove1(&mut self, value: &T) -> bool {
            self.remove_up_to(value, 1) != 0
        }

        /// 要素を最大 `count` 個削除し、削除した個数を返します。
        ///
        /// # 計算量
        /// $O(B + A \log L + L + M)$（$A$ は削除対象を含むバケット数、$M$ は実際に削除した個数）。
        pub fn remove_up_to(&mut self, value: &T, count: usize) -> usize {
            let mut remaining = count;
            let mut empty_buckets = 0..0;
            let mut index = 0;
            while remaining != 0 && index < self.buckets.len() {
                let bucket = &mut self.buckets[index];
                if bucket.last().unwrap() < value {
                    index += 1;
                    continue;
                }

                let start = bucket.partition_point(|entry| entry < value);
                let end = bucket.partition_point(|entry| entry <= value);
                let removed = remaining.min(end - start);
                let last_match_bucket = end < bucket.len();
                bucket.drain(start..start + removed);
                remaining -= removed;
                if bucket.is_empty() {
                    // 空になったバケットは連続するので、外側の配列をまとめて詰めます。
                    if empty_buckets.is_empty() {
                        empty_buckets.start = index;
                    }
                    empty_buckets.end = index + 1;
                }
                index += 1;
                if last_match_bucket {
                    break;
                }
            }

            if !empty_buckets.is_empty() {
                self.buckets.drain(empty_buckets);
            }
            let removed = count - remaining;
            self.length -= removed;
            if removed != 0 && !self.contains(value) {
                self.set_length -= 1;
            }
            removed
        }

        /// 指定した値をすべて削除し、削除できたかを返します。
        ///
        /// # 計算量
        /// $O(B + A \log L + L + M)$（$A$ は削除対象を含むバケット数、$M$ は実際に削除した個数）。
        pub fn remove_all(&mut self, value: &T) -> bool {
            self.remove_up_to(value, usize::MAX) != 0
        }

        /// 最小の要素を 1 個取り出します。
        ///
        /// # 計算量
        /// $O(B + L)$。
        pub fn pop_min(&mut self) -> Option<T> {
            let bucket = self.buckets.first_mut()?;
            let value = bucket.remove(0);
            if bucket.is_empty() {
                self.buckets.remove(0);
            }
            self.length -= 1;
            self.set_length -= usize::from(self.min() != Some(&value));
            Some(value)
        }

        /// 最大の要素を 1 個取り出します。
        ///
        /// # 計算量
        /// $O(1)$。
        pub fn pop_max(&mut self) -> Option<T> {
            let bucket = self.buckets.last_mut()?;
            let value = bucket.pop().unwrap();
            if bucket.is_empty() {
                self.buckets.pop();
            }
            self.length -= 1;
            self.set_length -= usize::from(self.max() != Some(&value));
            Some(value)
        }

        /// 指定した値がいくつあるかを返します。
        ///
        /// # 計算量
        /// $O(B + \log L)$。
        pub fn count(&self, value: &T) -> usize {
            self.rank(value, true) - self.rank(value, false)
        }

        /// 指定した値が含まれるかを返します。
        ///
        /// # 計算量
        /// $O(B + \log L)$。
        pub fn contains(&self, value: &T) -> bool {
            self.buckets
                .iter()
                .find(|bucket| bucket.last().unwrap() >= value)
                .is_some_and(|bucket| bucket.binary_search(value).is_ok())
        }

        /// 範囲内の最小の要素を返します。
        ///
        /// # 計算量
        /// $O(B + \log L)$。
        pub fn min_in_range<R: RangeBounds<T>>(&self, range: R) -> Option<&T> {
            self.nth_min_in_range(0, range)
        }

        /// 範囲内の最大の要素を返します。
        ///
        /// # 計算量
        /// $O(B + \log L)$。
        pub fn max_in_range<R: RangeBounds<T>>(&self, range: R) -> Option<&T> {
            self.nth_max_in_range(0, range)
        }

        /// 範囲内で、0 始まりで小さい方から `n` 番目の要素を返します。
        ///
        /// # 計算量
        /// $O(B + \log L)$。
        pub fn nth_min_in_range<R: RangeBounds<T>>(&self, n: usize, range: R) -> Option<&T> {
            let indices = self.rank_range(range);
            if n < indices.len() {
                self.nth_min(indices.start + n)
            } else {
                None
            }
        }

        /// 範囲内で、0 始まりで大きい方から `n` 番目の要素を返します。
        ///
        /// # 計算量
        /// $O(B + \log L)$。
        pub fn nth_max_in_range<R: RangeBounds<T>>(&self, n: usize, range: R) -> Option<&T> {
            let indices = self.rank_range(range);
            if n < indices.len() {
                self.nth_max(self.len() - indices.end + n)
            } else {
                None
            }
        }

        /// 範囲内に含まれる要素の個数を重複込みで返します。
        ///
        /// # 計算量
        /// $O(B + \log L)$。
        pub fn count_in_range<R: RangeBounds<T>>(&self, range: R) -> usize {
            self.rank_range(range).len()
        }

        /// 範囲内に要素があるかを返します。
        ///
        /// # 計算量
        /// $O(B + \log L)$。
        pub fn contains_in_range<R: RangeBounds<T>>(&self, range: R) -> bool {
            self.count_in_range(range) != 0
        }

        // 計算量は O(B + log L)。バケットを走査し、境界のバケット内を二分探索する。
        fn rank(&self, value: &T, inclusive: bool) -> usize {
            let precedes = |entry: &T| entry < value || (inclusive && entry == value);
            let mut rank = 0;
            for bucket in &self.buckets {
                if !precedes(bucket.last().unwrap()) {
                    return rank + bucket.partition_point(precedes);
                }
                rank += bucket.len();
            }
            rank
        }

        // 計算量は O(B + log L)。rank を高々 2 回呼ぶ。
        fn rank_range<R: RangeBounds<T>>(&self, range: R) -> Range<usize> {
            let start = match range.start_bound() {
                Bound::Unbounded => 0,
                Bound::Included(value) => self.rank(value, false),
                Bound::Excluded(value) => self.rank(value, true),
            };
            let end = match range.end_bound() {
                Bound::Unbounded => self.len(),
                Bound::Included(value) => self.rank(value, true),
                Bound::Excluded(value) => self.rank(value, false),
            };
            start..end.max(start)
        }
    }

    impl<T: Ord> FromIterator<T> for BucketMultiSet<T> {
        /// 要素を集めてソートし、ほぼ同じ長さのバケットに分けます。
        ///
        /// # 計算量
        /// $O(N \log N)$。入力が昇順の場合は $O(N)$（$N$ は入力の要素数）。
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut values: Vec<_> = iter.into_iter().collect();
            let length = values.len();
            if length == 0 {
                return Self::new();
            }
            if !values.is_sorted() {
                values.sort_unstable();
            }
            let set_length = 1 + values.windows(2).filter(|pair| pair[0] != pair[1]).count();
            let bucket_count = (length.div_ceil(16) - 1).isqrt() + 1;
            let mut values = values.into_iter();
            let buckets = (0..bucket_count)
                .map(|i| {
                    let size = length / bucket_count + usize::from(i < length % bucket_count);
                    values.by_ref().take(size).collect()
                })
                .collect();
            Self {
                buckets,
                length,
                set_length,
            }
        }
    }
}
