use cargo_snippet::snippet;

#[snippet(prefix = "use bucket_multiset::*;")]
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

#[cfg(test)]
mod tests {
    use std::ops::Bound::{Excluded, Included, Unbounded};

    use rand::{Rng, SeedableRng, rngs::SmallRng};

    use super::bucket_multiset::BucketMultiSet;
    use crate::data_structure::btree_multiset::btree_multiset::BTreeMultiSet;

    #[test]
    fn split_and_remove_duplicates_across_buckets() {
        let mut set = BucketMultiSet::default();
        assert!(set.is_empty());
        assert!(!set.remove1(&7));
        for _ in 0..100 {
            set.insert(7);
        }
        set.insert(6);
        set.insert(8);
        assert_eq!(set.set_len(), 3);
        assert_eq!(set.count(&7), 100);
        assert_eq!(set.remove_up_to(&7, 60), 60);
        assert_eq!(set.count(&7), 40);
        assert!(set.remove1(&7));
        assert!(set.remove_all(&7));
        assert!(!set.contains(&7));
        assert_eq!(set.len(), 2);
        assert_eq!(set.set_len(), 2);
        assert_eq!(set.pop_min(), Some(6));
        assert_eq!(set.pop_max(), Some(8));
        assert_eq!(set.pop_min(), None);
        assert_eq!(set.pop_max(), None);
        set.insert(9);
        set.clear();
        assert_eq!(set.set_len(), 0);
        assert!(set.is_empty());
    }

    #[test]
    fn rank_and_bounds() {
        let set: BucketMultiSet<_> = [8, 4, 1, 4, 2, 8].into_iter().collect();
        assert_eq!(set.nth_min(2), Some(&4));
        assert_eq!(set.nth_max(2), Some(&4));
        assert_eq!(set.nth_min(6), None);
        assert_eq!(set.nth_max(6), None);
        assert_eq!(set.min(), Some(&1));
        assert_eq!(set.max(), Some(&8));
        assert_eq!(set.count_in_range(2..8), 3);
        assert_eq!(set.min_in_range((Excluded(2), Included(8))), Some(&4));
        assert_eq!(set.max_in_range(..8), Some(&4));
        assert_eq!(set.nth_min_in_range(1, 4..), Some(&4));
        assert_eq!(set.nth_max_in_range(2, (Unbounded, Included(4))), Some(&2));
        assert_eq!(set.nth_min_in_range(usize::MAX, ..), None);
        assert_eq!(set.nth_max_in_range(usize::MAX, ..), None);
        assert!(!set.contains_in_range(5..8));
        assert!(set.contains_in_range(8..=8));
        assert_eq!(set.count_in_range((Excluded(4), Excluded(4))), 0);
    }

    #[test]
    fn test_random_bucket_vs_btree() {
        let mut rng = SmallRng::seed_from_u64(42);
        for initial_len in [0, 256] {
            let values: Vec<i32> = (0..initial_len).map(|_| rng.random_range(-5..=5)).collect();
            let mut bucket: BucketMultiSet<_> = values.iter().copied().collect();
            let mut btree = BTreeMultiSet::new();
            for value in values {
                btree.insert(value);
            }

            for step in 0..2000 {
                let value = rng.random_range(-5..=5);
                let count = rng.random_range(0..=5);
                let op = rng.random_range(0..16);
                let context = format!(
                    "initial_len={initial_len}, step={step}, op={op}, value={value}, count={count}"
                );
                match op {
                    0..=2 => {
                        bucket.insert(value);
                        btree.insert(value);
                    }
                    3..=5 => {
                        for _ in 0..count {
                            bucket.insert(value);
                        }
                        btree.insert_many(value, count);
                    }
                    6 => assert_eq!(bucket.remove1(&value), btree.remove1(&value), "{context}"),
                    7 => assert_eq!(
                        bucket.remove_up_to(&value, count),
                        btree.remove_up_to(&value, count),
                        "{context}"
                    ),
                    8 => assert_eq!(
                        bucket.remove_all(&value),
                        btree.remove_all(&value),
                        "{context}"
                    ),
                    9 => assert_eq!(bucket.pop_min(), btree.pop_min(), "{context}"),
                    10 => assert_eq!(bucket.pop_max(), btree.pop_max(), "{context}"),
                    _ if step % 97 == 0 => {
                        bucket.clear();
                        btree.clear();
                    }
                    _ => {}
                }

                assert_eq!(bucket.len(), btree.len(), "{context}");
                assert_eq!(bucket.set_len(), btree.set_len(), "{context}");
                assert_eq!(bucket.is_empty(), btree.is_empty(), "{context}");
                assert_eq!(bucket.min(), btree.min(), "{context}");
                assert_eq!(bucket.max(), btree.max(), "{context}");
                assert_eq!(bucket.count(&value), btree.count(&value), "{context}");
                assert_eq!(bucket.contains(&value), btree.contains(&value), "{context}");
                let n = rng.random_range(0..bucket.len() + 3);
                assert_eq!(bucket.nth_min(n), btree.nth_min(n), "{context}, n={n}");
                assert_eq!(bucket.nth_max(n), btree.nth_max(n), "{context}, n={n}");

                let start = rng.random_range(-7..=7);
                let end = rng.random_range(start..=7);
                let lower = match rng.random_range(0..3) {
                    0 => Included(start),
                    1 => Excluded(start),
                    _ => Unbounded,
                };
                let upper = match rng.random_range(0..3) {
                    0 => Included(end),
                    // BTreeMap は上下端が同じ値で両方とも除外の場合に panic します。
                    1 if lower == Excluded(end) => Excluded(end + 1),
                    1 => Excluded(end),
                    _ => Unbounded,
                };
                let range = (lower, upper);
                let range_context = format!("{context}, {range:?}");
                let expected_count: usize = btree.range(range).map(|(_, count)| count).sum();
                assert_eq!(
                    bucket.count_in_range(range),
                    expected_count,
                    "{range_context}"
                );
                assert_eq!(
                    bucket.contains_in_range(range),
                    expected_count != 0,
                    "{range_context}"
                );
                assert_eq!(
                    bucket.min_in_range(range),
                    btree.min_in_range(range),
                    "{range_context}"
                );
                assert_eq!(
                    bucket.max_in_range(range),
                    btree.max_in_range(range),
                    "{range_context}"
                );
                let n = rng.random_range(0..expected_count + 3);
                assert_eq!(
                    bucket.nth_min_in_range(n, range),
                    btree.nth_min_in_range(n, range),
                    "{range_context}, n={n}"
                );
                assert_eq!(
                    bucket.nth_max_in_range(n, range),
                    btree.nth_max_in_range(n, range),
                    "{range_context}, n={n}"
                );
            }
        }
    }
}
