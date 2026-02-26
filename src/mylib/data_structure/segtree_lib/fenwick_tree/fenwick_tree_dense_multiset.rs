use cargo_snippet::snippet;

#[snippet(prefix = "use fenwick_tree_dense_multiset::*;")]
pub mod fenwick_tree_dense_multiset {
    use std::ops::{Bound, RangeBounds};

    /// Fenwick Tree を基盤としたマルチセット。
    ///
    /// 要素は `0` から `size - 1` までの `usize` 値に限定されます。
    /// BTreeMultiSet と違って、任意の値を挿入することはできませんが、そのかわりk番目の値が k に依らず $O(\log N)$ で取得できます。
    #[derive(Clone, Debug)]
    pub struct FenwickTreeDenseMultiset {
        ft: InternalFenwickTree,
        length: usize,
        set_length: usize,
    }

    impl FenwickTreeDenseMultiset {
        /// 指定された範囲 `[0, size)` の値を管理する空のマルチセットを作成します。
        ///
        /// # 計算量
        /// $O(N)$ ($N$ は `size`)
        pub fn new(size: usize) -> Self {
            Self {
                ft: InternalFenwickTree::new(size),
                length: 0,
                set_length: 0,
            }
        }

        /// 指定した値を追加します。
        ///
        /// # Panics
        /// `value >= size` の場合にパニックします。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn insert(&mut self, value: usize) {
            self.insert_many(value, 1);
        }

        /// 指定した値を `count` 個追加します。
        ///
        /// # Panics
        /// `value >= size` の場合にパニックします。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn insert_many(&mut self, value: usize, count: usize) {
            if count == 0 {
                return;
            }
            if self.count(value) == 0 {
                self.set_length += 1;
            }
            self.ft.add(value, count as i64);
            self.length += count;
        }

        /// 要素を1つ削除します。
        ///
        /// 要素が存在した場合は `true`、存在しなかった場合は `false` を返します。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn remove1(&mut self, value: usize) -> bool {
            if self.count(value) > 0 {
                self.ft.add(value, -1);
                self.length -= 1;
                if self.count(value) == 0 {
                    self.set_length -= 1;
                }
                true
            } else {
                false
            }
        }

        /// 要素を最大 `count` 個削除します。
        ///
        /// 実際に削除した個数を返します。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn remove_up_to(&mut self, value: usize, count: usize) -> usize {
            let current = self.count(value);
            let removed = current.min(count);
            if removed > 0 {
                self.ft.add(value, -(removed as i64));
                self.length -= removed;
                if current == removed {
                    self.set_length -= 1;
                }
            }
            removed
        }

        /// 指定した要素をすべて削除します。
        ///
        /// 要素が存在した場合は `true`、存在しなかった場合は `false` を返します。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn remove_all(&mut self, value: usize) -> bool {
            let current = self.count(value);
            if current > 0 {
                self.ft.add(value, -(current as i64));
                self.length -= current;
                self.set_length -= 1;
                true
            } else {
                false
            }
        }

        /// 最小の要素を1つ取り出して削除します。
        ///
        /// 空の場合は `None` を返します。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn pop_min(&mut self) -> Option<usize> {
            let val = self.min()?;
            self.remove1(val);
            Some(val)
        }

        /// 最大の要素を1つ取り出して削除します。
        ///
        /// 空の場合は `None` を返します。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn pop_max(&mut self) -> Option<usize> {
            let val = self.max()?;
            self.remove1(val);
            Some(val)
        }

        /// マルチセットの全要素を削除し、空にします。
        ///
        /// # 計算量
        /// $O(N)$ ($N$ は `size`)
        pub fn clear(&mut self) {
            self.ft = InternalFenwickTree::new(self.ft.len());
            self.length = 0;
            self.set_length = 0;
        }

        /// マルチセットに含まれる全要素数（重複を含む）を返します。
        ///
        /// # 計算量
        /// $O(1)$
        pub fn len(&self) -> usize {
            self.length
        }

        /// マルチセットに含まれるユニークな要素の種類数を返します。
        ///
        /// # 計算量
        /// $O(1)$
        pub fn set_len(&self) -> usize {
            self.set_length
        }

        /// マルチセットが空かどうかを返します。
        ///
        /// # 計算量
        /// $O(1)$
        pub fn is_empty(&self) -> bool {
            self.length == 0
        }

        /// 指定した要素の個数を返します。
        ///
        /// # 計算量
        /// $O(1)$
        pub fn count(&self, value: usize) -> usize {
            self.ft.get(value) as usize
        }

        /// 指定した要素が含まれているかを返します。
        ///
        /// # 計算量
        /// $O(1)$
        pub fn contains(&self, value: usize) -> bool {
            self.count(value) > 0
        }

        /// 重複を考慮して、$n$ 番目に小さい要素を返します（0-indexed）。
        ///
        /// $n$ が全体の要素数（`len()`）以上の場合は `None` を返します。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn nth_min(&self, n: usize) -> Option<usize> {
            let idx = self.ft.max_right(0, |&s| s <= n as i64);
            if idx < self.ft.len() { Some(idx) } else { None }
        }

        /// 重複を考慮して、$n$ 番目に大きい要素を返します（0-indexed）。
        ///
        /// $n$ が全体の要素数（`len()`）以上の場合は `None` を返します。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn nth_max(&self, n: usize) -> Option<usize> {
            let length = self.length;
            if n < length {
                let target_prefix_sum = (length - 1 - n) as i64;
                Some(self.ft.max_right(0, |&s| s <= target_prefix_sum))
            } else {
                None
            }
        }

        /// 最小の要素を返します。
        ///
        /// 空の場合は `None` を返します。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn min(&self) -> Option<usize> {
            let idx = self.ft.max_right(0, |&s| s == 0);
            if idx < self.ft.len() { Some(idx) } else { None }
        }

        /// 最大の要素を返します。
        ///
        /// 空の場合は `None` を返します。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn max(&self) -> Option<usize> {
            let idx = self.ft.min_left(self.ft.len(), |&s| s == 0);
            if idx > 0 { Some(idx - 1) } else { None }
        }

        fn resolve_range<R: RangeBounds<usize>>(&self, range: R) -> (usize, usize) {
            let n = self.ft.len();
            let l = match range.start_bound() {
                Bound::Included(&l) => l,
                Bound::Excluded(&l) => l + 1,
                Bound::Unbounded => 0,
            };
            let r = match range.end_bound() {
                Bound::Included(&r) => r + 1,
                Bound::Excluded(&r) => r,
                Bound::Unbounded => n,
            };
            assert!(
                l <= r && r <= n,
                "FenwickTreeDenseMultiset::resolve_range: invalid range. l: {}, r: {}, n: {}",
                l,
                r,
                n
            );
            (l, r)
        }

        /// 指定された範囲内での最小の要素を返します。
        ///
        /// 範囲内に要素がない場合は `None` を返します。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn min_in_range<R: RangeBounds<usize>>(&self, range: R) -> Option<usize> {
            let (l, r) = self.resolve_range(range);
            let idx = self.ft.max_right(l, |&s| s == 0);
            if idx < r { Some(idx) } else { None }
        }

        /// 指定された範囲内での最大の要素を返します。
        ///
        /// 範囲内に要素がない場合は `None` を返します。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn max_in_range<R: RangeBounds<usize>>(&self, range: R) -> Option<usize> {
            let (l, r) = self.resolve_range(range);
            let idx = self.ft.min_left(r, |&s| s == 0);
            if idx > l { Some(idx - 1) } else { None }
        }

        /// 指定された範囲内で、重複を考慮して $n$ 番目に小さい要素を返す（0-indexed）。
        ///
        /// $n$ が範囲内の要素数以上の場合は `None` を返す。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn nth_min_in_range<R: RangeBounds<usize>>(&self, n: usize, range: R) -> Option<usize> {
            let (l, r) = self.resolve_range(range);
            let idx = self.ft.max_right(l, |&s| s <= n as i64);
            if idx < r { Some(idx) } else { None }
        }

        /// 指定された範囲内で、重複を考慮して $n$ 番目に大きい要素を返す（0-indexed）。
        ///
        /// $n$ が範囲内の要素数以上の場合は `None` を返す。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn nth_max_in_range<R: RangeBounds<usize>>(&self, n: usize, range: R) -> Option<usize> {
            let (l, r) = self.resolve_range(range);
            let range_count = self.ft.range_sum(l..r) as usize;
            if n < range_count {
                let target_prefix_sum = (range_count - 1 - n) as i64;
                Some(self.ft.max_right(l, |&s| s <= target_prefix_sum))
            } else {
                None
            }
        }

        /// 指定された範囲内の要素数（重複を含む）を返します。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn count_in_range<R: RangeBounds<usize>>(&self, range: R) -> usize {
            let (l, r) = self.resolve_range(range);
            if l >= r {
                return 0;
            }
            self.ft.range_sum(l..r) as usize
        }

        /// 指定した範囲内に要素が含まれているかを返します。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は `size`)
        pub fn contains_in_range<R: RangeBounds<usize>>(&self, range: R) -> bool {
            self.count_in_range(range) > 0
        }
    }

    /// Fenwick Tree の基本操作を提供する補助構造体。
    ///
    /// `range_sum_fenwick_tree.rs` に準拠した実装。
    #[derive(Clone, Debug)]
    struct InternalFenwickTree {
        n: usize,
        ary: Vec<i64>,
        vals: Vec<i64>,
    }

    impl InternalFenwickTree {
        fn new(n: usize) -> Self {
            Self {
                n,
                ary: vec![0; n],
                vals: vec![0; n],
            }
        }

        fn prefix_sum(&self, mut idx: usize) -> i64 {
            let mut sum = 0;
            while idx > 0 {
                sum += self.ary[idx - 1];
                idx &= idx - 1;
            }
            sum
        }

        fn add(&mut self, mut idx: usize, val: i64) {
            assert!(idx < self.n);
            self.vals[idx] += val;
            idx += 1;
            while idx <= self.n {
                self.ary[idx - 1] += val;
                idx += idx & idx.wrapping_neg();
            }
        }

        fn range_sum(&self, range: std::ops::Range<usize>) -> i64 {
            let l = range.start;
            let r = range.end;
            assert!(l <= r && r <= self.n);
            self.prefix_sum(r) - self.prefix_sum(l)
        }

        fn get(&self, idx: usize) -> i64 {
            assert!(idx < self.n);
            self.vals[idx]
        }

        fn max_right<F: FnMut(&i64) -> bool>(&self, l: usize, mut f: F) -> usize {
            assert!(l <= self.n);
            assert!(f(&0));
            let val_l = self.prefix_sum(l);
            let mut r = 0;
            let mut current_val = 0;
            let mut k = if self.n == 0 {
                0
            } else {
                1 << (usize::BITS - 1 - self.n.leading_zeros())
            };

            while k > 0 {
                if r + k <= self.n {
                    let next_val = current_val + self.ary[r + k - 1];
                    if r + k <= l || f(&(next_val - val_l)) {
                        r += k;
                        current_val = next_val;
                    }
                }
                k >>= 1;
            }
            r
        }

        fn min_left<F: FnMut(&i64) -> bool>(&self, r: usize, mut f: F) -> usize {
            assert!(r <= self.n);
            assert!(f(&0));
            let val_r = self.prefix_sum(r);
            if f(&val_r) {
                return 0;
            }
            let mut idx = 0;
            let mut current_val = 0;
            let mut k = if self.n == 0 {
                0
            } else {
                1 << (usize::BITS - 1 - self.n.leading_zeros())
            };

            while k > 0 {
                if idx + k <= r {
                    let next_val = current_val + self.ary[idx + k - 1];
                    if !f(&(val_r - next_val)) {
                        idx += k;
                        current_val = next_val;
                    }
                }
                k >>= 1;
            }
            idx + 1
        }

        fn len(&self) -> usize {
            self.n
        }
    }
}

#[cfg(test)]
mod tests {
    use super::fenwick_tree_dense_multiset::*;
    use crate::data_structure::btree_multiset::btree_multiset::BTreeMultiSet;
    use rand::{Rng, SeedableRng, rngs::SmallRng};
    use std::ops::Bound::*;

    #[test]
    fn test_basic() {
        let mut ms = FenwickTreeDenseMultiset::new(10);
        assert!(ms.is_empty());
        assert_eq!(ms.len(), 0);
        assert_eq!(ms.set_len(), 0);

        ms.insert(3);
        ms.insert(3);
        ms.insert(5);
        ms.insert_many(1, 2);

        assert_eq!(ms.len(), 5);
        assert_eq!(ms.count(1), 2);
        assert_eq!(ms.count(3), 2);
        assert_eq!(ms.count(5), 1);
        assert_eq!(ms.count(0), 0);
        assert_eq!(ms.set_len(), 3);

        assert_eq!(ms.min(), Some(1));
        assert_eq!(ms.max(), Some(5));

        assert_eq!(ms.nth_min(0), Some(1));
        assert_eq!(ms.nth_min(1), Some(1));
        assert_eq!(ms.nth_min(2), Some(3));
        assert_eq!(ms.nth_min(3), Some(3));
        assert_eq!(ms.nth_min(4), Some(5));
        assert_eq!(ms.nth_min(5), None);

        assert_eq!(ms.nth_max(0), Some(5));
        assert_eq!(ms.nth_max(1), Some(3));
        assert_eq!(ms.nth_max(2), Some(3));
        assert_eq!(ms.nth_max(3), Some(1));
        assert_eq!(ms.nth_max(4), Some(1));
        assert_eq!(ms.nth_max(5), None);

        assert!(ms.contains(3));
        assert!(!ms.contains(0));
    }

    #[test]
    fn test_remove() {
        let mut ms = FenwickTreeDenseMultiset::new(10);
        ms.insert_many(3, 3);
        ms.insert(5);

        assert_eq!(ms.remove_up_to(3, 2), 2);
        assert_eq!(ms.count(3), 1);
        assert_eq!(ms.len(), 2);
        assert_eq!(ms.set_len(), 2);

        assert!(ms.remove1(3));
        assert_eq!(ms.count(3), 0);
        assert_eq!(ms.set_len(), 1);

        assert!(!ms.remove1(3));
        assert_eq!(ms.remove_up_to(3, 1), 0);
        assert!(!ms.remove_all(3));

        ms.insert_many(7, 5);
        assert!(ms.remove_all(7));
        assert_eq!(ms.count(7), 0);
        assert_eq!(ms.len(), 1); // only 5 remains
    }

    #[test]
    fn test_range_queries() {
        let mut ms = FenwickTreeDenseMultiset::new(20);
        ms.insert_many(5, 2);
        ms.insert_many(10, 3);
        ms.insert_many(15, 2);

        assert_eq!(ms.min_in_range(6..16), Some(10));
        assert_eq!(ms.max_in_range(6..16), Some(15));
        assert_eq!(ms.min_in_range(11..15), None);
        assert_eq!(ms.min_in_range(10..10), None);

        assert_eq!(ms.nth_min_in_range(0, 6..16), Some(10));
        assert_eq!(ms.nth_min_in_range(2, 6..16), Some(10));
        assert_eq!(ms.nth_min_in_range(3, 6..16), Some(15));
        assert_eq!(ms.nth_min_in_range(5, 6..16), None);

        assert_eq!(ms.nth_max_in_range(0, 6..16), Some(15));
        assert_eq!(ms.nth_max_in_range(2, 6..16), Some(10));

        assert_eq!(ms.count_in_range(6..16), 5);
        assert_eq!(ms.count_in_range(11..15), 0);
        assert_eq!(ms.count_in_range(..), 7);

        assert!(ms.contains_in_range(14..16));
        assert!(!ms.contains_in_range(11..15));

        // Range Bound types
        assert_eq!(ms.min_in_range((Excluded(5), Included(10))), Some(10));
        assert_eq!(ms.min_in_range(..), Some(5));
        assert_eq!(ms.max_in_range(..), Some(15));
        assert_eq!(ms.min_in_range(16..), None);
    }

    #[test]
    fn test_pop() {
        let mut ms = FenwickTreeDenseMultiset::new(10);
        ms.insert(3);
        ms.insert(1);
        ms.insert(5);

        assert_eq!(ms.pop_min(), Some(1));
        assert_eq!(ms.pop_max(), Some(5));
        assert_eq!(ms.pop_min(), Some(3));
        assert_eq!(ms.pop_min(), None);
        assert_eq!(ms.pop_max(), None);
    }

    #[test]
    fn test_clear_and_empty() {
        let mut ms = FenwickTreeDenseMultiset::new(10);
        ms.insert(1);
        ms.clear();
        assert!(ms.is_empty());
        assert_eq!(ms.len(), 0);
        assert_eq!(ms.set_len(), 0);
        assert_eq!(ms.min(), None);
        assert_eq!(ms.max(), None);

        let _ms_empty = FenwickTreeDenseMultiset::new(0);
    }

    #[test]
    fn test_edge_values() {
        let mut ms = FenwickTreeDenseMultiset::new(10);
        ms.insert(0);
        ms.insert(9);
        assert_eq!(ms.min(), Some(0));
        assert_eq!(ms.max(), Some(9));
    }

    #[test]
    fn test_insert_zero_count() {
        let mut ms = FenwickTreeDenseMultiset::new(10);
        ms.insert_many(3, 0);
        assert!(ms.is_empty());
    }

    #[test]
    fn test_coverage_edge_cases_dense() {
        let mut ms = FenwickTreeDenseMultiset::new(10);
        ms.insert(3);
        ms.insert(7);
        // Unbounded patterns
        assert_eq!(ms.min_in_range(..), Some(3));
        assert_eq!(ms.max_in_range(..), Some(7));
        assert_eq!(ms.min_in_range(4..), Some(7));
        assert_eq!(ms.max_in_range(..6), Some(3));
        assert!(ms.contains_in_range(..));

        // Excluded pattern
        assert_eq!(ms.min_in_range((Excluded(3), Unbounded)), Some(7));

        // size 0
        let ms_zero = FenwickTreeDenseMultiset::new(0);
        assert_eq!(ms_zero.min(), None);
        assert_eq!(ms_zero.max(), None);
    }

    #[test]
    #[should_panic(expected = "invalid range")]
    fn test_out_of_bounds_range() {
        let ms = FenwickTreeDenseMultiset::new(10);
        ms.min_in_range(0..11);
    }

    #[test]
    #[ignore]
    fn test_random_dense_vs_btree() {
        let mut rng = SmallRng::from_os_rng();
        let size = 100;
        let mut ft_ms = FenwickTreeDenseMultiset::new(size);
        let mut bt_ms = BTreeMultiSet::new();

        for _ in 0..2000 {
            match rng.random_range(0..14) {
                0 => {
                    let v = rng.random_range(0..size);
                    ft_ms.insert(v);
                    bt_ms.insert(v);
                }
                1 => {
                    let v = rng.random_range(0..size);
                    let c = rng.random_range(0..5);
                    ft_ms.insert_many(v, c);
                    bt_ms.insert_many(v, c);
                }
                2 => {
                    let v = rng.random_range(0..size);
                    assert_eq!(ft_ms.remove1(v), bt_ms.remove1(&v));
                }
                3 => {
                    let v = rng.random_range(0..size);
                    let v_btree = v;
                    let c = rng.random_range(0..5);
                    assert_eq!(ft_ms.remove_up_to(v, c), bt_ms.remove_up_to(&v_btree, c));
                }
                4 => {
                    let v = rng.random_range(0..size);
                    let v_btree = v;
                    assert_eq!(ft_ms.remove_all(v), bt_ms.remove_all(&v_btree));
                }
                5 => {
                    assert_eq!(ft_ms.pop_min(), bt_ms.pop_min());
                }
                6 => {
                    assert_eq!(ft_ms.pop_max(), bt_ms.pop_max());
                }
                7 => {
                    assert_eq!(ft_ms.min(), bt_ms.min().copied());
                    assert_eq!(ft_ms.max(), bt_ms.max().copied());
                }
                8 => {
                    let n = rng.random_range(0..ft_ms.len() + 10);
                    assert_eq!(ft_ms.nth_min(n), bt_ms.nth_min(n).copied());
                    assert_eq!(ft_ms.nth_max(n), bt_ms.nth_max(n).copied());
                }
                9 => {
                    let l = rng.random_range(0..=size);
                    let r = rng.random_range(l..=size);
                    let range = l..r;
                    assert_eq!(
                        ft_ms.min_in_range(range.clone()),
                        bt_ms.min_in_range(range.clone()).copied()
                    );
                    assert_eq!(
                        ft_ms.max_in_range(range.clone()),
                        bt_ms.max_in_range(range.clone()).copied()
                    );
                    assert_eq!(
                        ft_ms.count_in_range(range.clone()),
                        bt_ms.range(range.clone()).map(|(_, &c)| c).sum::<usize>()
                    );
                    assert_eq!(
                        ft_ms.contains_in_range(range.clone()),
                        bt_ms.contains_in_range(range)
                    );
                }
                10 => {
                    let l = rng.random_range(0..=size);
                    let r = rng.random_range(l..=size);
                    let range = l..r;
                    let count = bt_ms.range(range.clone()).map(|(_, &c)| c).sum::<usize>();
                    let n = rng.random_range(0..count + 10);
                    assert_eq!(
                        ft_ms.nth_min_in_range(n, range.clone()),
                        bt_ms.nth_min_in_range(n, range.clone()).copied()
                    );
                    assert_eq!(
                        ft_ms.nth_max_in_range(n, range.clone()),
                        bt_ms.nth_max_in_range(n, range).copied()
                    );
                }
                11 => {
                    let v = rng.random_range(0..size);
                    let v_btree = v;
                    assert_eq!(ft_ms.count(v), bt_ms.count(&v_btree));
                    assert_eq!(ft_ms.contains(v), bt_ms.contains(&v_btree));
                }
                12 => {
                    assert_eq!(ft_ms.len(), bt_ms.len());
                    assert_eq!(ft_ms.set_len(), bt_ms.set_len());
                }
                13 => {
                    ft_ms.clear();
                    bt_ms.clear();
                }
                _ => unreachable!(),
            }
        }
    }
}
