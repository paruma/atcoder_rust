use cargo_snippet::snippet;

#[snippet(prefix = "use fenwick_tree_sparse_multiset::*;")]
pub mod fenwick_tree_sparse_multiset {
    use std::ops::RangeBounds;

    /// Fenwick Tree を基盤とした座標圧縮付きマルチセット。
    ///
    /// BTreeMultiSet と違って、`entries` として指定した値以外を挿入することはできませんが、そのかわりk番目の値が k に依らず $O(\log N)$ で取得できます。
    #[derive(Clone, Debug)]
    pub struct FenwickTreeSparseMultiset {
        ft: InternalFenwickTree,
        cc: CoordinateCompression,
        length: usize,
        set_length: usize,
    }

    impl FenwickTreeSparseMultiset {
        /// 構築時に指定された `entries` に基づいて座標圧縮空間を構築し、空のマルチセットを作成します。
        ///
        /// # 計算量
        /// $O(N \log N)$ ($N$ は `entries.len()`)
        pub fn new(entries: &[i64]) -> Self {
            let cc = CoordinateCompression::new(entries);
            let size = cc.space_size();
            Self {
                ft: InternalFenwickTree::new(size),
                cc,
                length: 0,
                set_length: 0,
            }
        }

        /// 指定した値を追加します。
        ///
        /// # Panics
        /// 構築時に指定された `entries` に含まれない値が渡された場合にパニックします。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn insert(&mut self, value: i64) {
            self.insert_many(value, 1);
        }

        /// 指定した値を `count` 個追加します。
        ///
        /// # Panics
        /// 構築時に指定された `entries` に含まれない値が渡された場合にパニックします。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn insert_many(&mut self, value: i64, count: usize) {
            if count == 0 {
                return;
            }
            let idx = self.cc.compress(value);
            if self.ft.get(idx) == 0 {
                self.set_length += 1;
            }
            self.ft.add(idx, count as i64);
            self.length += count;
        }

        /// 要素を1つ削除します。
        ///
        /// 要素が存在した場合は `true`、存在しなかった場合は `false` を返します。
        ///
        /// # Panics
        /// 構築時に指定された `entries` に含まれない値が渡された場合にパニックします。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn remove1(&mut self, value: i64) -> bool {
            let idx = self.cc.compress(value);
            if self.ft.get(idx) > 0 {
                self.ft.add(idx, -1);
                self.length -= 1;
                if self.ft.get(idx) == 0 {
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
        /// # Panics
        /// 構築時に指定された `entries` に含まれない値が渡された場合にパニックします。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn remove_up_to(&mut self, value: i64, count: usize) -> usize {
            let idx = self.cc.compress(value);
            let current = self.ft.get(idx) as usize;
            let removed = current.min(count);
            if removed > 0 {
                self.ft.add(idx, -(removed as i64));
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
        /// # Panics
        /// 構築時に指定された `entries` に含まれない値が渡された場合にパニックします。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn remove_all(&mut self, value: i64) -> bool {
            let idx = self.cc.compress(value);
            let current = self.ft.get(idx) as usize;
            if current > 0 {
                self.ft.add(idx, -(current as i64));
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
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn pop_min(&mut self) -> Option<i64> {
            let val = self.min()?;
            self.remove1(val);
            Some(val)
        }

        /// 最大の要素を1つ取り出して削除します。
        ///
        /// 空の場合は `None` を返します。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn pop_max(&mut self) -> Option<i64> {
            let val = self.max()?;
            self.remove1(val);
            Some(val)
        }

        /// マルチセットの全要素を削除し、空にします。
        ///
        /// # 計算量
        /// $O(N)$ ($N$ は一意な要素数)
        pub fn clear(&mut self) {
            self.ft.clear();
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
        /// # Panics
        /// 構築時に指定された `entries` に含まれない値が渡された場合にパニックします。
        ///
        /// # 計算量
        /// $O(1)$
        pub fn count(&self, value: i64) -> usize {
            let idx = self.cc.compress(value);
            self.ft.get(idx) as usize
        }

        /// 指定した要素が含まれているかを返します。
        ///
        /// # Panics
        /// 構築時に指定された `entries` に含まれない値が渡された場合にパニックします。
        ///
        /// # 計算量
        /// $O(1)$
        pub fn contains(&self, value: i64) -> bool {
            self.count(value) > 0
        }

        /// 重複を考慮して、$n$ 番目に小さい要素を返します（0-indexed）。
        ///
        /// $n$ が全体の要素数（`len()`）以上の場合は `None` を返します。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn nth_min(&self, n: usize) -> Option<i64> {
            let idx = self.ft.max_right(0, |&s| s <= n as i64);
            if idx < self.ft.len() {
                Some(self.cc.decompress(idx))
            } else {
                None
            }
        }

        /// 重複を考慮して、$n$ 番目に大きい要素を返します（0-indexed）。
        ///
        /// $n$ が全体の要素数（`len()`）以上の場合は `None` を返します。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn nth_max(&self, n: usize) -> Option<i64> {
            let length = self.length;
            if n < length {
                let target_prefix_sum = (length - 1 - n) as i64;
                let idx = self.ft.max_right(0, |&s| s <= target_prefix_sum);
                Some(self.cc.decompress(idx))
            } else {
                None
            }
        }

        /// 最小の要素を返します。
        ///
        /// 空の場合は `None` を返します。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn min(&self) -> Option<i64> {
            let idx = self.ft.max_right(0, |&s| s == 0);
            if idx < self.ft.len() {
                Some(self.cc.decompress(idx))
            } else {
                None
            }
        }

        /// 最大の要素を返します。
        ///
        /// 空の場合は `None` を返します。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn max(&self) -> Option<i64> {
            let idx = self.ft.min_left(self.ft.len(), |&s| s == 0);
            if idx > 0 {
                Some(self.cc.decompress(idx - 1))
            } else {
                None
            }
        }

        /// 指定された範囲内での最小の要素を返します。
        ///
        /// 範囲内に要素がない場合は `None` を返します。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn min_in_range<R: RangeBounds<i64>>(&self, range: R) -> Option<i64> {
            let r_idx = self.cc.compress_range(range);
            let idx = self.ft.max_right(r_idx.start, |&s| s == 0);
            if idx < r_idx.end {
                Some(self.cc.decompress(idx))
            } else {
                None
            }
        }

        /// 指定された範囲内での最大の要素を返します。
        ///
        /// 範囲内に要素がない場合は `None` を返します。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn max_in_range<R: RangeBounds<i64>>(&self, range: R) -> Option<i64> {
            let r_idx = self.cc.compress_range(range);
            let idx = self.ft.min_left(r_idx.end, |&s| s == 0);
            if idx > r_idx.start {
                Some(self.cc.decompress(idx - 1))
            } else {
                None
            }
        }

        /// 指定された範囲内で、重複を考慮して $n$ 番目に小さい要素を返す（0-indexed）。
        ///
        /// $n$ が範囲内の要素数以上の場合は `None` を返す。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn nth_min_in_range<R: RangeBounds<i64>>(&self, n: usize, range: R) -> Option<i64> {
            let r_idx = self.cc.compress_range(range);
            let idx = self.ft.max_right(r_idx.start, |&s| s <= n as i64);
            if idx < r_idx.end {
                Some(self.cc.decompress(idx))
            } else {
                None
            }
        }

        /// 指定された範囲内で、重複を考慮して $n$ 番目に大きい要素を返す（0-indexed）。
        ///
        /// $n$ が範囲内の要素数以上の場合は `None` を返す。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn nth_max_in_range<R: RangeBounds<i64>>(&self, n: usize, range: R) -> Option<i64> {
            let r_idx = self.cc.compress_range(range);
            let range_count = self.ft.range_sum(r_idx.start..r_idx.end) as usize;
            if n < range_count {
                let target_prefix_sum = (range_count - 1 - n) as i64;
                let idx = self.ft.max_right(r_idx.start, |&s| s <= target_prefix_sum);
                Some(self.cc.decompress(idx))
            } else {
                None
            }
        }

        /// 指定された範囲内の要素数（重複を含む）を返します。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn count_in_range<R: RangeBounds<i64>>(&self, range: R) -> usize {
            let r_idx = self.cc.compress_range(range);
            if r_idx.start >= r_idx.end {
                return 0;
            }
            self.ft.range_sum(r_idx.start..r_idx.end) as usize
        }

        /// 指定した範囲内に要素が含まれているかを返します。
        ///
        /// # 計算量
        /// $O(\log N)$ ($N$ は一意な要素数)
        pub fn contains_in_range<R: RangeBounds<i64>>(&self, range: R) -> bool {
            self.count_in_range(range) > 0
        }
    }

    /// 座標圧縮構造体。
    #[derive(Debug, Clone)]
    struct CoordinateCompression {
        space: Vec<i64>,
    }

    impl CoordinateCompression {
        fn new(space: &[i64]) -> Self {
            let mut space = space.to_vec();
            space.sort_unstable();
            space.dedup();
            Self { space }
        }

        fn compress(&self, x: i64) -> usize {
            self.space
                .binary_search(&x)
                .expect("Value not in coordinate compression space")
        }

        fn decompress(&self, i: usize) -> i64 {
            self.space[i]
        }

        fn space_size(&self) -> usize {
            self.space.len()
        }

        fn compress_range(&self, range: impl RangeBounds<i64>) -> std::ops::Range<usize> {
            use std::ops::Bound::*;
            let begin = match range.start_bound() {
                Unbounded => 0,
                Included(&x) => self.space.binary_search(&x).unwrap_or_else(|e| e),
                Excluded(&x) => match self.space.binary_search(&x) {
                    Ok(i) => i + 1,
                    Err(i) => i,
                },
            };
            let end = match range.end_bound() {
                Unbounded => self.space.len(),
                Included(&x) => match self.space.binary_search(&x) {
                    Ok(i) => i + 1,
                    Err(i) => i,
                },
                Excluded(&x) => self.space.binary_search(&x).unwrap_or_else(|e| e),
            };
            begin..end
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

        fn clear(&mut self) {
            self.ary.fill(0);
            self.vals.fill(0);
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
    use super::fenwick_tree_sparse_multiset::*;
    use crate::data_structure::btree_multiset::btree_multiset::BTreeMultiSet;
    use rand::{Rng, SeedableRng, rngs::SmallRng};
    use std::ops::Bound::*;

    #[test]
    fn test_basic_sparse() {
        let entries = vec![10, 20, 30, 40, 50];
        let mut ms = FenwickTreeSparseMultiset::new(&entries);
        assert!(ms.is_empty());
        assert_eq!(ms.set_len(), 0);

        ms.insert(30);
        ms.insert(30);
        ms.insert(50);
        ms.insert_many(10, 2);

        assert_eq!(ms.len(), 5);
        assert_eq!(ms.count(10), 2);
        assert_eq!(ms.count(30), 2);
        assert_eq!(ms.count(50), 1);
        assert_eq!(ms.count(20), 0);
        assert_eq!(ms.set_len(), 3);

        assert_eq!(ms.min(), Some(10));
        assert_eq!(ms.max(), Some(50));

        assert_eq!(ms.nth_min(0), Some(10));
        assert_eq!(ms.nth_min(2), Some(30));
        assert_eq!(ms.nth_min(4), Some(50));
        assert_eq!(ms.nth_min(5), None);

        assert_eq!(ms.nth_max(0), Some(50));
        assert_eq!(ms.nth_max(1), Some(30));
        assert_eq!(ms.nth_max(3), Some(10));
        assert_eq!(ms.nth_max(5), None);

        assert!(ms.contains(30));
        assert!(!ms.contains(20));
    }

    #[test]
    fn test_remove_sparse() {
        let entries = vec![10, 20, 30];
        let mut ms = FenwickTreeSparseMultiset::new(&entries);
        ms.insert_many(10, 3);
        ms.insert(20);

        assert_eq!(ms.remove_up_to(10, 2), 2);
        assert_eq!(ms.count(10), 1);
        assert!(ms.remove1(10));
        assert_eq!(ms.count(10), 0);
        assert!(!ms.remove1(10));
        assert!(ms.remove_all(20));
        assert_eq!(ms.len(), 0);
    }

    #[test]
    fn test_range_queries_sparse() {
        let entries = vec![10, 20, 30, 40, 50];
        let mut ms = FenwickTreeSparseMultiset::new(&entries);
        ms.insert(10);
        ms.insert(20);
        ms.insert(30);
        ms.insert(40);
        ms.insert(50);

        assert_eq!(ms.min_in_range(15..45), Some(20));
        assert_eq!(ms.max_in_range(15..45), Some(40));
        assert_eq!(ms.nth_min_in_range(1, 15..45), Some(30));
        assert_eq!(ms.nth_max_in_range(1, 15..45), Some(30));

        assert_eq!(ms.count_in_range(15..45), 3);
        assert_eq!(ms.count_in_range(15..20), 0);
        assert_eq!(ms.count_in_range(..), 5);

        assert!(ms.contains_in_range(15..25));
        assert!(!ms.contains_in_range(15..20));

        // Edge case: empty range
        assert_eq!(ms.min_in_range(20..20), None);

        // Bound types
        assert_eq!(ms.min_in_range((Excluded(10), Included(30))), Some(20));
    }

    #[test]
    fn test_pop_sparse() {
        let mut ms = FenwickTreeSparseMultiset::new(&[10, 20, 30]);
        ms.insert(20);
        ms.insert(10);
        assert_eq!(ms.pop_min(), Some(10));
        assert_eq!(ms.pop_max(), Some(20));
        assert_eq!(ms.pop_min(), None);
    }

    #[test]
    fn test_clear_sparse() {
        let mut ms = FenwickTreeSparseMultiset::new(&[10, 20]);
        ms.insert(10);
        ms.clear();
        assert!(ms.is_empty());
    }

    #[test]
    #[should_panic]
    fn test_invalid_insert_sparse() {
        let ms = FenwickTreeSparseMultiset::new(&[1, 2, 3]);
        let mut ms = ms;
        ms.insert(4);
    }

    #[test]
    fn test_insert_zero_count_sparse() {
        let mut ms = FenwickTreeSparseMultiset::new(&[10]);
        ms.insert_many(10, 0);
        assert!(ms.is_empty());
    }

    #[test]
    fn test_coverage_edge_cases() {
        // RangeBounds Unbounded patterns
        let entries = vec![10, 20, 30];
        let mut ms = FenwickTreeSparseMultiset::new(&entries);
        ms.insert(10);
        ms.insert(30);
        assert_eq!(ms.min_in_range(..), Some(10));
        assert_eq!(ms.max_in_range(..), Some(30));
        assert_eq!(ms.min_in_range(15..), Some(30));
        assert_eq!(ms.max_in_range(..25), Some(10));
        assert!(ms.contains_in_range(..));

        // Binary search Err cases in compress_range
        assert_eq!(ms.min_in_range(15..25), None);
        assert_eq!(ms.max_in_range(15..25), None);
        assert_eq!(ms.min_in_range((Excluded(10), Excluded(30))), None);

        // size 0 Fenwick Tree
        let ms_zero = FenwickTreeSparseMultiset::new(&[]);
        assert!(ms_zero.is_empty());
        assert_eq!(ms_zero.min(), None);
        assert_eq!(ms_zero.max(), None);
    }

    #[test]
    #[ignore]
    fn test_random_sparse_vs_btree() {
        let mut rng = SmallRng::from_os_rng();
        let mut entries = Vec::new();
        for _ in 0..10 {
            entries.push(rng.random_range(-20..20));
        }
        let mut ft_ms = FenwickTreeSparseMultiset::new(&entries);
        let mut bt_ms = BTreeMultiSet::new();

        entries.sort_unstable();
        entries.dedup();

        for _ in 0..2000 {
            match rng.random_range(0..14) {
                0 => {
                    let v = entries[rng.random_range(0..entries.len())];
                    ft_ms.insert(v);
                    bt_ms.insert(v);
                }
                1 => {
                    let v = entries[rng.random_range(0..entries.len())];
                    let c = rng.random_range(0..5);
                    ft_ms.insert_many(v, c);
                    bt_ms.insert_many(v, c);
                }
                2 => {
                    let v = entries[rng.random_range(0..entries.len())];
                    assert_eq!(ft_ms.remove1(v), bt_ms.remove1(&v));
                }
                3 => {
                    let v = entries[rng.random_range(0..entries.len())];
                    let v_btree = v;
                    let c = rng.random_range(0..5);
                    assert_eq!(ft_ms.remove_up_to(v, c), bt_ms.remove_up_to(&v_btree, c));
                }
                4 => {
                    let v = entries[rng.random_range(0..entries.len())];
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
                    let l = rng.random_range(-30..30);
                    let r = rng.random_range(l..30);
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
                    let l = rng.random_range(-30..30);
                    let r = rng.random_range(l..30);
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
                    let v = entries[rng.random_range(0..entries.len())];
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
