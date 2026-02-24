use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use btree_multiset::*;")]
pub mod btree_multiset {
    use std::{
        borrow::Borrow,
        collections::{BTreeMap, btree_map::Range},
        ops::RangeBounds,
    };

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct BTreeMultiSet<T> {
        map: BTreeMap<T, usize>,
        length: usize,
    }

    impl<T> Default for BTreeMultiSet<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T> BTreeMultiSet<T> {
        /// 新しい空のマルチセットを作成する。
        ///
        /// 計算量は $O(1)$。
        pub const fn new() -> BTreeMultiSet<T> {
            BTreeMultiSet {
                map: BTreeMap::new(),
                length: 0,
            }
        }

        /// 指定した範囲の要素とその個数のイテレータを返す。
        ///
        /// 計算量は $O(\log K)$ ($K$ は種類数)。
        pub fn range<R>(&self, range: R) -> Range<'_, T, usize>
        where
            T: Ord,
            R: RangeBounds<T>,
        {
            self.map.range(range)
        }

        /// 内部の BTreeMap のイテレータを返す。
        ///
        /// 要素とその個数のペア `(&T, &usize)` を巡回する。
        pub fn iter(&self) -> std::collections::btree_map::Iter<'_, T, usize> {
            self.map.iter()
        }

        /// 最小の要素を返す。
        ///
        /// 空の場合は `None` を返す。計算量は $O(\log K)$ ($K$ は種類数)。
        pub fn min(&self) -> Option<&T>
        where
            T: Ord,
        {
            self.map.first_key_value().map(|(k, _)| k)
        }

        /// 最大の要素を返す。
        ///
        /// 空の場合は `None` を返す。計算量は $O(\log K)$ ($K$ は種類数)。
        pub fn max(&self) -> Option<&T>
        where
            T: Ord,
        {
            self.map.last_key_value().map(|(k, _)| k)
        }

        /// 重複を考慮して、$n$ 番目に小さい要素を返す（0-indexed）。
        ///
        /// $n$ が全体の要素数（`len()`）以上の場合は `None` を返す。
        /// 計算量は $O(\log K + \min(m, K))$ ($m$ は走査したユニークな要素数、$K$ は種類数)。
        pub fn nth_min(&self, n: usize) -> Option<&T>
        where
            T: Ord,
        {
            let mut sum = 0;
            for (val, &cnt) in self.iter() {
                if sum + cnt > n {
                    return Some(val);
                }
                sum += cnt;
            }
            None
        }

        /// 重複を考慮して、$n$ 番目に大きい要素を返す（0-indexed）。
        ///
        /// $n$ が全体の要素数（`len()`）以上の場合は `None` を返す。
        /// 計算量は $O(\log K + \min(m, K))$ ($m$ は走査したユニークな要素数、$K$ は種類数)。
        pub fn nth_max(&self, n: usize) -> Option<&T>
        where
            T: Ord,
        {
            let mut sum = 0;
            for (val, &cnt) in self.iter().rev() {
                if sum + cnt > n {
                    return Some(val);
                }
                sum += cnt;
            }
            None
        }

        /// 指定した範囲内での最小の要素を返す。
        ///
        /// 範囲内に要素がない場合は `None` を返す。計算量は $O(\log K)$ ($K$ は種類数)。
        pub fn min_in_range<R>(&self, range: R) -> Option<&T>
        where
            T: Ord,
            R: RangeBounds<T>,
        {
            self.range(range).next().map(|(k, _)| k)
        }

        /// 指定した範囲内での最大の要素を返す。
        ///
        /// 範囲内に要素がない場合は `None` を返す。計算量は $O(\log K)$ ($K$ は種類数)。
        pub fn max_in_range<R>(&self, range: R) -> Option<&T>
        where
            T: Ord,
            R: RangeBounds<T>,
        {
            self.range(range).next_back().map(|(k, _)| k)
        }

        /// 指定した範囲内で、重複を考慮して $n$ 番目に小さい要素を返す（0-indexed）。
        ///
        /// $n$ が範囲内の要素数以上の場合は `None` を返す。
        /// 計算量は $O(\log K + \min(m, K))$ ($m$ は範囲内で走査したユニークな要素数、$K$ は種類数)。
        pub fn nth_min_in_range<R>(&self, n: usize, range: R) -> Option<&T>
        where
            T: Ord,
            R: RangeBounds<T>,
        {
            let mut sum = 0;
            for (val, &cnt) in self.range(range) {
                if sum + cnt > n {
                    return Some(val);
                }
                sum += cnt;
            }
            None
        }

        /// 指定した範囲内で、重複を考慮して $n$ 番目に大きい要素を返す（0-indexed）。
        ///
        /// $n$ が範囲内の要素数以上の場合は `None` を返す。
        /// 計算量は $O(\log K + \min(m, K))$ ($m$ は範囲内で走査したユニークな要素数、$K$ は種類数)。
        pub fn nth_max_in_range<R>(&self, n: usize, range: R) -> Option<&T>
        where
            T: Ord,
            R: RangeBounds<T>,
        {
            let mut sum = 0;
            for (val, &cnt) in self.range(range).rev() {
                if sum + cnt > n {
                    return Some(val);
                }
                sum += cnt;
            }
            None
        }

        /// 要素を1つ追加する。
        ///
        /// 計算量は $O(\log K)$ ($K$ は種類数)。
        pub fn insert(&mut self, value: T)
        where
            T: Ord,
        {
            *self.map.entry(value).or_insert(0) += 1;
            self.length += 1;
        }

        /// 要素を指定した個数分追加する。
        ///
        /// 計算量は $O(\log K)$ ($K$ は種類数)。
        pub fn insert_many(&mut self, value: T, count: usize)
        where
            T: Ord,
        {
            if count == 0 {
                return;
            }
            *self.map.entry(value).or_insert(0) += count;
            self.length += count;
        }

        /// 要素を1つ削除する。
        ///
        /// 要素が存在した場合は `true`、存在しなかった場合は `false` を返す。
        /// 計算量は $O(\log K)$ ($K$ は種類数)。
        pub fn remove1<Q>(&mut self, value: &Q) -> bool
        where
            T: Borrow<Q> + Ord,
            Q: ?Sized + Ord,
        {
            if let Some(cnt) = self.map.get_mut(value) {
                *cnt -= 1;
                if *cnt == 0 {
                    self.map.remove(value);
                }
                self.length -= 1;
                return true;
            }
            false
        }

        /// 要素を最大 `count` 個削除する。
        ///
        /// 実際に削除した個数を返す。計算量は $O(\log K)$ ($K$ は種類数)。
        pub fn remove_up_to<Q>(&mut self, value: &Q, count: usize) -> usize
        where
            T: Borrow<Q> + Ord,
            Q: ?Sized + Ord,
        {
            if count == 0 {
                return 0;
            }
            if let Some(cnt) = self.map.get_mut(value) {
                let removed = (*cnt).min(count);
                *cnt -= removed;
                if *cnt == 0 {
                    self.map.remove(value);
                }
                self.length -= removed;
                removed
            } else {
                0
            }
        }

        /// 指定した要素をすべて削除する。
        ///
        /// 要素が存在した場合は `true`、存在しなかった場合は `false` を返す。
        /// 計算量は $O(\log K)$ ($K$ は種類数)。
        pub fn remove_all<Q>(&mut self, value: &Q) -> bool
        where
            T: Borrow<Q> + Ord,
            Q: ?Sized + Ord,
        {
            if let Some(cnt) = self.map.get(value) {
                self.length -= cnt;
                self.map.remove(value);
                return true;
            }
            false
        }

        /// 最小の要素を1つ取り出して削除する。
        ///
        /// 空の場合は `None` を返す。計算量は $O(\log K)$ ($K$ は種類数)。
        pub fn pop_min(&mut self) -> Option<T>
        where
            T: Ord + Clone,
        {
            let mut entry = self.map.first_entry()?;
            self.length -= 1;
            if *entry.get() > 1 {
                *entry.get_mut() -= 1;
                Some(entry.key().clone())
            } else {
                let (key, _) = entry.remove_entry();
                Some(key)
            }
        }

        /// 最大の要素を1つ取り出して削除する。
        ///
        /// 空の場合は `None` を返す。計算量は $O(\log K)$ ($K$ は種類数)。
        pub fn pop_max(&mut self) -> Option<T>
        where
            T: Ord + Clone,
        {
            let mut entry = self.map.last_entry()?;
            self.length -= 1;
            if *entry.get() > 1 {
                *entry.get_mut() -= 1;
                Some(entry.key().clone())
            } else {
                let (key, _) = entry.remove_entry();
                Some(key)
            }
        }

        /// マルチセットの全要素を削除し、空にする。
        ///
        /// 計算量は $O(N)$ ($N$ は要素の種類数)。
        pub fn clear(&mut self) {
            self.map.clear();
            self.length = 0;
        }

        /// マルチセットに含まれる全要素数（重複を含む）を返す。
        ///
        /// 計算量は $O(1)$。
        pub fn len(&self) -> usize {
            self.length
        }

        /// マルチセットに含まれるユニークな要素の種類数を返す。
        ///
        /// 計算量は $O(1)$。
        pub fn set_len(&self) -> usize {
            self.map.len()
        }

        /// マルチセットが空かどうかを返す。
        ///
        /// 計算量は $O(1)$。
        pub fn is_empty(&self) -> bool {
            self.length == 0
        }

        /// 指定した要素の個数を返す。
        ///
        /// 計算量は $O(\log K)$ ($K$ は種類数)。
        pub fn count<Q>(&self, value: &Q) -> usize
        where
            T: Borrow<Q> + Ord,
            Q: ?Sized + Ord,
        {
            self.map.get(value).copied().unwrap_or(0)
        }

        /// 指定した要素が含まれているかを返す。
        ///
        /// 計算量は $O(\log K)$ ($K$ は種類数)。
        pub fn contains<Q>(&self, value: &Q) -> bool
        where
            T: Borrow<Q> + Ord,
            Q: ?Sized + Ord,
        {
            self.map.contains_key(value)
        }

        /// 指定した範囲内に要素が含まれているかを返す。
        ///
        /// 計算量は $O(\log K)$ ($K$ は種類数)。
        pub fn contains_in_range<R>(&self, range: R) -> bool
        where
            T: Ord,
            R: RangeBounds<T>,
        {
            self.range(range).next().is_some()
        }
    }
    impl<T: Ord> FromIterator<T> for BTreeMultiSet<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> BTreeMultiSet<T> {
            let mut set = BTreeMultiSet::new();
            for x in iter {
                set.insert(x);
            }
            set
        }
    }
}

#[cfg(test)]
mod tests {
    use super::btree_multiset::BTreeMultiSet;

    #[test]
    fn test_new() {
        let set: BTreeMultiSet<i32> = BTreeMultiSet::new();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_default() {
        let set: BTreeMultiSet<i32> = Default::default();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_insert() {
        let mut set = BTreeMultiSet::new();
        set.insert(1);
        assert!(!set.is_empty());
        assert_eq!(set.len(), 1);
        assert!(set.contains(&1));
        assert_eq!(set.count(&1), 1);

        set.insert(1);
        assert_eq!(set.len(), 2);
        assert_eq!(set.count(&1), 2);

        set.insert(2);
        assert_eq!(set.len(), 3);
        assert!(set.contains(&2));
        assert_eq!(set.count(&2), 1);
    }

    #[test]
    fn test_remove1() {
        let mut set: BTreeMultiSet<_> = vec![1, 1, 2].into_iter().collect();

        assert!(set.remove1(&1));
        assert_eq!(set.len(), 2);
        assert_eq!(set.count(&1), 1);

        assert!(set.remove1(&1));
        assert_eq!(set.len(), 1);
        assert_eq!(set.count(&1), 0);
        assert!(!set.contains(&1));

        assert!(set.remove1(&2));
        assert!(set.is_empty());

        // Test removing an element not in the set
        assert!(!set.remove1(&3));
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_remove_all() {
        let mut set: BTreeMultiSet<_> = vec![1, 1, 2, 2, 2, 3].into_iter().collect();

        assert!(set.remove_all(&1));
        assert_eq!(set.len(), 4);
        assert_eq!(set.count(&1), 0);
        assert!(!set.contains(&1));

        assert!(set.remove_all(&2));
        assert_eq!(set.len(), 1);
        assert_eq!(set.count(&2), 0);
        assert!(!set.contains(&2));

        assert!(set.remove_all(&3));
        assert_eq!(set.len(), 0);
        assert!(set.is_empty());

        // Test removing an element not in the set
        assert!(!set.remove_all(&4));
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_count() {
        let set: BTreeMultiSet<_> = vec![1, 1, 2].into_iter().collect();

        assert_eq!(set.count(&1), 2);
        assert_eq!(set.count(&2), 1);
        assert_eq!(set.count(&3), 0);
    }

    #[test]
    fn test_set_len() {
        let mut set: BTreeMultiSet<_> = vec![1, 1, 2].into_iter().collect();

        assert_eq!(set.len(), 3);
        assert_eq!(set.set_len(), 2);

        set.remove1(&1);
        assert_eq!(set.len(), 2);
        assert_eq!(set.set_len(), 2);

        set.remove1(&1);
        assert_eq!(set.len(), 1);
        assert_eq!(set.set_len(), 1);
    }

    #[test]
    fn test_contains() {
        let set: BTreeMultiSet<_> = vec![1, 1, 2].into_iter().collect();

        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(!set.contains(&3));
    }

    #[test]
    fn test_range() {
        let set: BTreeMultiSet<_> = vec![1, 2, 3].into_iter().collect();

        let range: Vec<_> = set.range(2..).collect();
        assert_eq!(range, vec![(&2, &1), (&3, &1)]);

        let range: Vec<_> = set.range(..3).collect();
        assert_eq!(range, vec![(&1, &1), (&2, &1)]);

        // Test range with elements out of bounds
        let range: Vec<_> = set.range(10..).collect();
        assert!(range.is_empty());
    }

    #[test]
    fn test_from_iter_and_iter() {
        let vec = vec![1, 1, 2, 3];
        let set: BTreeMultiSet<_> = vec.into_iter().collect();

        assert_eq!(set.len(), 4);
        assert_eq!(set.count(&1), 2);
        assert_eq!(set.count(&2), 1);
        assert_eq!(set.count(&3), 1);
    }

    #[test]
    fn test_iter() {
        let vec = vec![1, 1, 2, 3];
        let set: BTreeMultiSet<_> = vec.into_iter().collect();
        let elements: Vec<_> = set.iter().map(|(k, v)| (k, *v)).collect();
        assert_eq!(elements, vec![(&1, 2), (&2, 1), (&3, 1)]);
    }

    #[test]
    fn test_min_max() {
        let mut set = BTreeMultiSet::new();
        assert_eq!(set.min(), None);
        assert_eq!(set.max(), None);

        set.insert(20);
        set.insert(10);
        set.insert(30);
        set.insert(20);

        assert_eq!(set.min(), Some(&10));
        assert_eq!(set.max(), Some(&30));
    }

    #[test]
    fn test_nth() {
        let set: BTreeMultiSet<_> = vec![10, 20, 20, 30].into_iter().collect();

        // nth_min
        assert_eq!(set.nth_min(0), Some(&10));
        assert_eq!(set.nth_min(1), Some(&20));
        assert_eq!(set.nth_min(2), Some(&20));
        assert_eq!(set.nth_min(3), Some(&30));
        assert_eq!(set.nth_min(4), None);

        // nth_max
        assert_eq!(set.nth_max(0), Some(&30));
        assert_eq!(set.nth_max(1), Some(&20));
        assert_eq!(set.nth_max(2), Some(&20));
        assert_eq!(set.nth_max(3), Some(&10));
        assert_eq!(set.nth_max(4), None);
    }

    #[test]
    fn test_range_min_max() {
        let set: BTreeMultiSet<_> = vec![10, 20, 20, 30, 40].into_iter().collect();

        assert_eq!(set.min_in_range(15..35), Some(&20));
        assert_eq!(set.max_in_range(15..35), Some(&30));

        assert_eq!(set.min_in_range(45..), None);
        assert_eq!(set.max_in_range(..5), None);
    }

    #[test]
    fn test_range_nth() {
        let set: BTreeMultiSet<_> = vec![10, 20, 20, 30, 40].into_iter().collect();

        // 15..35 contains {20, 20, 30}
        assert_eq!(set.nth_min_in_range(0, 15..35), Some(&20));
        assert_eq!(set.nth_min_in_range(1, 15..35), Some(&20));
        assert_eq!(set.nth_min_in_range(2, 15..35), Some(&30));
        assert_eq!(set.nth_min_in_range(3, 15..35), None);

        assert_eq!(set.nth_max_in_range(0, 15..35), Some(&30));
        assert_eq!(set.nth_max_in_range(1, 15..35), Some(&20));
        assert_eq!(set.nth_max_in_range(2, 15..35), Some(&20));
        assert_eq!(set.nth_max_in_range(3, 15..35), None);
    }

    #[test]
    fn test_is_empty() {
        let mut set: BTreeMultiSet<i32> = BTreeMultiSet::new();
        assert!(set.is_empty());

        set.insert(1);
        assert!(!set.is_empty());

        set.remove1(&1);
        assert!(set.is_empty());
    }

    #[test]
    fn test_edge_cases() {
        let mut set: BTreeMultiSet<i32> = BTreeMultiSet::new();

        // Test remove on an empty set
        assert!(!set.remove1(&1));

        // Test count on an empty set
        assert_eq!(set.count(&1), 0);

        // Test contains on an empty set
        assert!(!set.contains(&1));

        // Test range on an empty set
        let range: Vec<_> = set.range(1..).collect();
        assert!(range.is_empty());

        // Test insert with negative and zero values
        set.insert(-1);
        set.insert(0);
        assert!(set.contains(&-1));
        assert!(set.contains(&0));
    }

    #[test]
    fn test_insert_many() {
        let mut set = BTreeMultiSet::new();
        set.insert_many(1, 3);
        assert_eq!(set.len(), 3);
        assert_eq!(set.count(&1), 3);

        set.insert_many(1, 0); // 既に存在する値を 0 個追加
        assert_eq!(set.len(), 3);
        assert_eq!(set.count(&1), 3);

        set.insert_many(2, 0); // 存在しない値を 0 個追加（種類数が増えないことを確認）
        assert_eq!(set.len(), 3);
        assert_eq!(set.set_len(), 1);
        assert_eq!(set.count(&2), 0);

        set.insert_many(2, 2);
        assert_eq!(set.len(), 5);
        assert_eq!(set.count(&2), 2);
    }

    #[test]
    fn test_remove_up_to() {
        let mut set: BTreeMultiSet<_> = vec![1, 1, 1, 2, 2].into_iter().collect();

        assert_eq!(set.remove_up_to(&1, 2), 2);
        assert_eq!(set.len(), 3);
        assert_eq!(set.count(&1), 1);

        assert_eq!(set.remove_up_to(&1, 5), 1);
        assert_eq!(set.len(), 2);
        assert_eq!(set.count(&1), 0);
        assert!(!set.contains(&1));

        assert_eq!(set.remove_up_to(&3, 1), 0);
        assert_eq!(set.remove_up_to(&2, 0), 0);
        assert_eq!(set.remove_up_to(&3, 0), 0); // 存在しない値を 0 個削除
        assert_eq!(set.len(), 2);
        assert_eq!(set.set_len(), 1);
    }

    #[test]
    fn test_pop_min_max() {
        let mut set: BTreeMultiSet<_> = vec![10, 10, 20, 30, 30].into_iter().collect();

        assert_eq!(set.pop_min(), Some(10));
        assert_eq!(set.len(), 4);
        assert_eq!(set.count(&10), 1);

        assert_eq!(set.pop_max(), Some(30));
        assert_eq!(set.len(), 3);
        assert_eq!(set.count(&30), 1);

        assert_eq!(set.pop_max(), Some(30));
        assert_eq!(set.len(), 2);
        assert_eq!(set.count(&30), 0);

        assert_eq!(set.pop_min(), Some(10));
        assert_eq!(set.pop_min(), Some(20));
        assert_eq!(set.pop_min(), None);
        assert_eq!(set.pop_max(), None);
        assert!(set.is_empty());
    }

    #[test]
    fn test_contains_in_range() {
        let set: BTreeMultiSet<_> = vec![10, 20, 30].into_iter().collect();

        // 存在する要素を含む範囲
        assert!(set.contains_in_range(15..25));
        assert!(set.contains_in_range(..15));
        assert!(set.contains_in_range(25..));
        assert!(set.contains_in_range(10..=10));
        assert!(set.contains_in_range(30..=30));

        // 存在しない要素の範囲
        assert!(!set.contains_in_range(35..45));
        assert!(!set.contains_in_range(5..10)); // 10 は exclusive end
        assert!(!set.contains_in_range(10..10)); // 空の範囲
        assert!(!set.contains_in_range(30..30)); // 空の範囲

        // Excluded 境界
        use std::ops::Bound::*;
        assert!(set.contains_in_range((Excluded(10), Unbounded))); // 20, 30 が含まれる
        assert!(!set.contains_in_range((Excluded(30), Unbounded))); // 30 より大きい要素はない
        assert!(!set.contains_in_range((Excluded(10), Excluded(20)))); // 10 と 20 の間には要素がない

        let empty_set: BTreeMultiSet<i32> = BTreeMultiSet::new();
        assert!(!empty_set.contains_in_range(..));
    }

    #[test]
    fn test_clear() {
        let mut set: BTreeMultiSet<_> = vec![1, 2, 3].into_iter().collect();
        set.clear();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
        assert_eq!(set.set_len(), 0);
        assert!(!set.contains(&1));
    }
}
