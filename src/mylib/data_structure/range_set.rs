use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_set::*;")]
pub mod range_set {

    use std::collections::BTreeMap;

    /// 整数の集合を隣り合わない半開区間の直和で管理するデータ構造。
    ///
    /// # 機能
    /// - 区間内の整数の追加 (`insert_range`)
    /// - 区間内の整数の削除 (`remove_range`)
    /// - 点が区間集合に含まれるかの判定 (`contains`)
    /// - 区間が完全にカバーされているかの判定 (`covers`)
    /// - 全区間の長さの合計 (`len`)
    /// - x 以上で集合に含まれない最小値 (`min_exclusive_geq`、いわゆる mex)
    /// - x 以下で集合に含まれない最大値 (`max_exclusive_leq`)
    /// - など
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct RangeSet {
        map: BTreeMap<i64, i64>, // key: l, value: r で半開区間 [l, r) を管理
        total_length: i64,
    }

    impl Default for RangeSet {
        fn default() -> Self {
            Self::new()
        }
    }

    impl RangeSet {
        /// 空の `RangeSet` を作成する。
        pub fn new() -> Self {
            Self {
                map: BTreeMap::new(),
                total_length: 0,
            }
        }

        /// 区間 `[l, r)` の各値を集合に追加する。
        ///
        /// # 計算量
        /// amortized O(log N)
        pub fn insert_range(&mut self, l: i64, r: i64) {
            assert!(l <= r);
            if r == l {
                return;
            }

            // マージ後の区間の開始点と終了点
            let mut start = l;
            let mut end = r;

            let mut to_remove = Vec::new();
            let mut removed_len = 0;

            // [l, r) と重なる区間、あるいは隣接する区間に対して処理をする。
            // 具体的には l <= r_i と l_i <= r を満たす [l_i, r_i) に対して処理をする
            for (&l_i, &r_i) in self.map.range(..=r).rev().take_while(|&(_, r_i)| l <= *r_i) {
                start = start.min(l_i);
                end = end.max(r_i);
                to_remove.push(l_i);
                removed_len += r_i - l_i;
            }

            for l_i in to_remove {
                self.map.remove(&l_i);
            }

            let added_len = end - start;
            self.total_length += added_len - removed_len;

            self.map.insert(start, end);
        }

        /// 区間 `[l, r)` の各値を集合から削除する。
        ///
        /// # 計算量
        /// amortized O(log N)
        pub fn remove_range(&mut self, l: i64, r: i64) {
            assert!(l <= r);
            if r == l {
                return;
            }

            let mut to_add = Vec::new();
            let mut to_remove = Vec::new();
            let mut len_change = 0;

            // [l, r) と重なる区間に対して処理をする
            // 具体的には l < r_i と l_i < r を満たす [l_i, r_i) に対して処理をする
            for (&l_i, &r_i) in self.map.range(..r).rev().take_while(|&(_, r_i)| l < *r_i) {
                to_remove.push(l_i);
                len_change -= r_i - l_i;

                // [l_i, l) の部分が残る可能性 (削除範囲の左側にはみ出している部分)
                if l_i < l {
                    to_add.push((l_i, l));
                    len_change += l - l_i;
                }

                // [r, r_i) の部分が残る可能性 (削除範囲の右側にはみ出している部分)
                if r < r_i {
                    to_add.push((r, r_i));
                    len_change += r_i - r;
                }
            }

            for l_i in to_remove {
                self.map.remove(&l_i);
            }
            for (l_add, r_add) in to_add {
                self.map.insert(l_add, r_add);
            }

            self.total_length += len_change;
        }

        /// 集合に `x` を追加する。
        ///
        /// # 計算量
        /// amortized O(log N)
        pub fn insert(&mut self, x: i64) {
            self.insert_range(x, x + 1);
        }

        /// 集合から `x` を削除する。
        ///
        /// # 計算量
        /// amortized O(log N)
        pub fn remove(&mut self, x: i64) {
            self.remove_range(x, x + 1);
        }

        /// 集合が `x` を含んでいるかを返す。
        ///
        /// # 計算量
        /// O(log N)
        pub fn contains(&self, x: i64) -> bool {
            self.find_range(x).is_some()
        }

        /// 集合が区間 `[l, r)` を含んでいるかを返す。
        ///
        /// # 計算量
        /// O(log N)
        pub fn covers(&self, l: i64, r: i64) -> bool {
            assert!(l <= r);
            if r == l {
                return true;
            }
            if let Some((_start, end)) = self.find_range(l) {
                r <= end
            } else {
                false
            }
        }

        /// 集合が空かどうかを返す。
        ///
        /// # 計算量
        /// O(1)
        pub fn is_empty(&self) -> bool {
            self.map.is_empty()
        }

        /// 集合の要素数を返す。
        ///
        /// # 計算量
        /// O(1)
        pub fn len(&self) -> i64 {
            self.total_length
        }

        /// 集合に含まれる最小値を返す。
        /// 集合が空の場合は `None` を返す。
        ///
        /// # 計算量
        /// O(1)
        pub fn min(&self) -> Option<i64> {
            self.map.keys().next().copied()
        }

        /// 集合に含まれる最大値を返す。
        /// 集合が空の場合は `None` を返す。
        ///
        /// # 計算量
        /// O(1)
        pub fn max(&self) -> Option<i64> {
            self.map.iter().next_back().map(|(_, &r)| r - 1)
        }

        /// 指定した区間 `[l, r)` と集合が共通部分を持たない（重ならない）かを返す。
        ///
        /// # 計算量
        /// O(log N)
        pub fn is_disjoint(&self, l: i64, r: i64) -> bool {
            if l >= r {
                return true;
            }
            // l_i < r を満たす最後の区間 [l_i, r_i) を取得
            // この区間が [l, r) と重ならなければ、これより左にある区間も重ならない
            if let Some((_, &r_i)) = self.map.range(..r).next_back() {
                // [l_i, r_i) と [l, r) が重なる条件は l < r_i
                // (l_i < r は range(..r) により保証されているため)
                // よって、重ならない条件は r_i <= l
                r_i <= l
            } else {
                // l_i < r を満たす区間がない -> すべての区間は r 以上 -> [l, r) とは重ならない
                true
            }
        }

        /// 集合全体が指定した区間 `[l, r)` に完全に含まれているかを返す。
        ///
        /// # 計算量
        /// O(1)
        pub fn is_covered_by(&self, l: i64, r: i64) -> bool {
            if self.is_empty() {
                return true;
            }
            if l >= r {
                return false;
            }

            // 最小値が l 以上か
            if let Some(min_val) = self.min() {
                if min_val < l {
                    return false;
                }
            }

            // 最大値が r 未満か (最大値 < r  <=> 最大値 <= r - 1)
            if let Some(max_val) = self.max() {
                if max_val >= r {
                    return false;
                }
            }

            true
        }

        /// x 以上で self に入っていない値の最小値を返す (いわゆる mex)
        ///
        /// # 計算量
        /// O(log N)
        pub fn min_exclusive_geq(&self, x: i64) -> i64 {
            if let Some((_, r)) = self.find_range(x) {
                r
            } else {
                x
            }
        }

        /// x 以下で self に入っていない値の最大値を返す
        ///
        /// # 計算量
        /// O(log N)
        pub fn max_exclusive_leq(&self, x: i64) -> i64 {
            if let Some((l, _)) = self.find_range(x) {
                l - 1
            } else {
                x
            }
        }

        /// x 以上で集合に含まれる最小の値を返す。
        /// 集合に含まれる値が存在しない場合は None を返す。
        ///
        /// # 計算量
        /// O(log N)
        pub fn min_inclusive_geq(&self, x: i64) -> Option<i64> {
            if self.contains(x) {
                return Some(x);
            }
            // x より右にある最初の区間の開始点を探す
            self.map.range(x..).next().map(|(&l, _)| l)
        }

        /// x 以下で集合に含まれる最大の値を返す。
        /// 集合に含まれる値が存在しない場合は None を返す。
        ///
        /// # 計算量
        /// O(log N)
        pub fn max_inclusive_leq(&self, x: i64) -> Option<i64> {
            if self.contains(x) {
                return Some(x);
            }
            // x より左にある最初の区間の終了点 - 1 を探す
            self.map.range(..x).last().map(|(_, &r)| r - 1)
        }

        /// `x` が含まれる区間 `[l, r)` を検索し、`Some((l, r))` で返す。
        /// `x` を含む区間が見つからない場合は `None` を返す。
        fn find_range(&self, x: i64) -> Option<(i64, i64)> {
            if let Some((&l, &r)) = self.map.range(..=x).last() {
                if x < r {
                    // l <= x < r
                    Some((l, r))
                } else {
                    None
                }
            } else {
                None
            }
        }

        /// 管理しているすべての区間 `[l, r)` のイテレータを返す。
        #[cfg(test)]
        pub(crate) fn ranges(&self) -> impl Iterator<Item = (i64, i64)> + '_ {
            self.map.iter().map(|(&l, &r)| (l, r))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::range_set::*;
    use rand::{Rng, SeedableRng, rngs::StdRng};

    #[test]
    fn test_default() {
        let set = RangeSet::default();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_insert_non_overlapping() {
        let mut set = RangeSet::new();
        set.insert_range(0, 5);
        set.insert_range(10, 15);
        assert_eq!(set.ranges().count(), 2);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(0, 5), (10, 15)]);
    }

    #[test]
    fn test_insert_overlapping_merge() {
        let mut set = RangeSet::new();
        set.insert_range(0, 5);
        set.insert_range(3, 8); // [0, 5) と [3, 8) -> [0, 8)
        assert_eq!(set.ranges().count(), 1);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(0, 8)]);

        set.insert_range(10, 15);
        set.insert_range(-2, 1); // [-2, 1) と [0, 8) -> [-2, 8)
        assert_eq!(set.ranges().count(), 2);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(-2, 8), (10, 15)]);
    }

    #[test]
    fn test_insert_adjacent_merge() {
        let mut set = RangeSet::new();
        set.insert_range(0, 5);
        set.insert_range(5, 10); // [0, 5) と [5, 10) -> [0, 10)
        assert_eq!(set.ranges().count(), 1);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(0, 10)]);

        set.insert_range(-5, 0); // [-5, 0) と [0, 10) -> [-5, 10)
        assert_eq!(set.ranges().count(), 1);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(-5, 10)]);
    }

    #[test]
    fn test_insert_containing_merge() {
        let mut set = RangeSet::new();
        set.insert_range(0, 10);
        set.insert_range(3, 7); // [3, 7) は [0, 10) に含まれる
        assert_eq!(set.ranges().count(), 1);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(0, 10)]);

        set.insert_range(-5, 15); // [0, 10) は [-5, 15) に含まれる
        assert_eq!(set.ranges().count(), 1);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(-5, 15)]);
    }

    #[test]
    fn test_insert_multiple_merge() {
        let mut set = RangeSet::new();
        set.insert_range(0, 2);
        set.insert_range(4, 6);
        set.insert_range(8, 10);
        set.insert_range(1, 9); // [1, 9) が [0,2), [4,6), [8,10) をマージ
        assert_eq!(set.ranges().count(), 1);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(0, 10)]);
    }

    #[test]
    fn test_remove_no_overlap() {
        let mut set = RangeSet::new();
        set.insert_range(10, 20);
        set.remove_range(0, 5);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(10, 20)]);
    }

    #[test]
    fn test_remove_total() {
        let mut set = RangeSet::new();
        set.insert_range(10, 20);
        set.remove_range(10, 20);
        assert!(set.is_empty());

        set.insert_range(10, 20);
        set.remove_range(5, 25);
        assert!(set.is_empty());
    }

    #[test]
    fn test_remove_split() {
        let mut set = RangeSet::new();
        set.insert_range(10, 20);
        set.remove_range(13, 17); // [10, 20) -> [10, 13) と [17, 20)
        assert_eq!(set.ranges().count(), 2);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(10, 13), (17, 20)]);
    }

    #[test]
    fn test_remove_left_side() {
        let mut set = RangeSet::new();
        set.insert_range(10, 20);
        set.remove_range(5, 15);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(15, 20)]);
    }

    #[test]
    fn test_remove_right_side() {
        let mut set = RangeSet::new();
        set.insert_range(10, 20);
        set.remove_range(15, 25);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(10, 15)]);
    }

    #[test]
    fn test_remove_multiple_ranges() {
        let mut set = RangeSet::new();
        set.insert_range(0, 5);
        set.insert_range(10, 15);
        set.insert_range(20, 25);
        set.remove_range(3, 22);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(0, 3), (22, 25)]);
    }

    #[test]
    fn test_contains() {
        let mut set = RangeSet::new();
        set.insert_range(0, 5);
        set.insert_range(10, 15);

        assert!(!set.contains(-1));
        assert!(set.contains(0));
        assert!(set.contains(4));
        assert!(!set.contains(5));
        assert!(!set.contains(9));
        assert!(set.contains(10));
        assert!(set.contains(14));
        assert!(!set.contains(15));
    }

    #[test]
    fn test_covers() {
        let mut set = RangeSet::new();
        set.insert_range(0, 10);
        assert!(set.covers(2, 8));
        assert!(set.covers(0, 10));
        assert!(!set.covers(0, 11));
        assert!(!set.covers(-1, 10));
    }

    #[test]
    fn test_len() {
        let mut set = RangeSet::new();
        set.insert_range(0, 5);
        set.insert_range(10, 15);
        assert_eq!(set.len(), 10);
        set.insert_range(4, 11);
        assert_eq!(set.len(), 15);
    }

    #[test]
    fn test_insert_remove_single_point() {
        let mut set = RangeSet::new();
        assert!(!set.contains(5));
        assert_eq!(set.len(), 0);

        set.insert(5);
        assert!(set.contains(5));
        assert_eq!(set.len(), 1);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(5, 6)]);

        set.insert(5); // Inserting existing should not change state
        assert!(set.contains(5));
        assert_eq!(set.len(), 1);

        set.insert(10);
        assert!(set.contains(10));
        assert_eq!(set.len(), 2);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(5, 6), (10, 11)]);

        set.remove(5);
        assert!(!set.contains(5));
        assert_eq!(set.len(), 1);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(10, 11)]);

        set.remove(5); // Deleting non-existing should not change state
        assert!(!set.contains(5));
        assert_eq!(set.len(), 1);

        set.remove(10);
        assert!(!set.contains(10));
        assert_eq!(set.len(), 0);
        assert!(set.is_empty());

        // Test with merging adjacent points
        set.insert(0);
        set.insert(1);
        set.insert(3);
        assert_eq!(set.len(), 3);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(0, 2), (3, 4)]);

        set.insert(2);
        assert_eq!(set.len(), 4);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(0, 4)]);

        set.remove(1);
        assert_eq!(set.len(), 3);
        assert!(!set.contains(1));
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(0, 1), (2, 4)]);
    }

    #[test]
    fn test_is_empty() {
        let mut set = RangeSet::new();
        assert!(set.is_empty());

        set.insert_range(0, 5);
        assert!(!set.is_empty());

        set.remove_range(0, 5);
        assert!(set.is_empty());

        set.insert_range(10, 15);
        set.insert_range(20, 25);
        assert!(!set.is_empty());

        set.remove_range(10, 25);
        assert!(set.is_empty());
    }

    #[test]
    fn test_mex() {
        let mut set = RangeSet::new();
        set.insert_range(1, 3); // [1, 3) -> {1, 2}
        set.insert_range(5, 7); // [5, 7) -> {5, 6}

        // min_exclusive_geq
        assert_eq!(set.min_exclusive_geq(0), 0);
        assert_eq!(set.min_exclusive_geq(1), 3); // 1 is in, next is 3
        assert_eq!(set.min_exclusive_geq(2), 3); // 2 is in, next is 3
        assert_eq!(set.min_exclusive_geq(3), 3); // 3 is not in
        assert_eq!(set.min_exclusive_geq(4), 4); // 4 is not in
        assert_eq!(set.min_exclusive_geq(5), 7); // 5 is in, next is 7
        assert_eq!(set.min_exclusive_geq(6), 7); // 6 is in, next is 7
        assert_eq!(set.min_exclusive_geq(7), 7); // 7 is not in

        // max_exclusive_leq
        assert_eq!(set.max_exclusive_leq(8), 8);
        assert_eq!(set.max_exclusive_leq(7), 7);
        assert_eq!(set.max_exclusive_leq(6), 4); // 6 is in, prev is 4
        assert_eq!(set.max_exclusive_leq(5), 4); // 5 is in, prev is 4
        assert_eq!(set.max_exclusive_leq(4), 4);
        assert_eq!(set.max_exclusive_leq(3), 3);
        assert_eq!(set.max_exclusive_leq(2), 0); // 2 is in, prev is 0
        assert_eq!(set.max_exclusive_leq(1), 0); // 1 is in, prev is 0
        assert_eq!(set.max_exclusive_leq(0), 0);

        // Edge cases
        let empty_set = RangeSet::new();
        assert_eq!(empty_set.min_exclusive_geq(100), 100);
        assert_eq!(empty_set.max_exclusive_leq(100), 100);

        let mut set_neg = RangeSet::new();
        set_neg.insert_range(-2, 0); // {-2, -1}
        assert_eq!(set_neg.min_exclusive_geq(-3), -3);
        assert_eq!(set_neg.min_exclusive_geq(-2), 0);
        assert_eq!(set_neg.min_exclusive_geq(-1), 0);
        assert_eq!(set_neg.min_exclusive_geq(0), 0);

        assert_eq!(set_neg.max_exclusive_leq(1), 1);
        assert_eq!(set_neg.max_exclusive_leq(0), 0);
        assert_eq!(set_neg.max_exclusive_leq(-1), -3);
        assert_eq!(set_neg.max_exclusive_leq(-2), -3);
        assert_eq!(set_neg.max_exclusive_leq(-3), -3);
    }

    #[test]
    fn test_inclusive_search() {
        let mut set = RangeSet::new();
        set.insert_range(1, 4); // [1, 4) -> {1, 2, 3}
        set.insert_range(6, 8); // [6, 8) -> {6, 7}

        // min_inclusive_geq
        assert_eq!(set.min_inclusive_geq(0), Some(1));
        assert_eq!(set.min_inclusive_geq(1), Some(1));
        assert_eq!(set.min_inclusive_geq(2), Some(2));
        assert_eq!(set.min_inclusive_geq(3), Some(3));
        assert_eq!(set.min_inclusive_geq(4), Some(6)); // 4, 5 はない
        assert_eq!(set.min_inclusive_geq(5), Some(6));
        assert_eq!(set.min_inclusive_geq(6), Some(6));
        assert_eq!(set.min_inclusive_geq(7), Some(7));
        assert_eq!(set.min_inclusive_geq(8), None);

        // max_inclusive_leq
        assert_eq!(set.max_inclusive_leq(9), Some(7));
        assert_eq!(set.max_inclusive_leq(8), Some(7));
        assert_eq!(set.max_inclusive_leq(7), Some(7));
        assert_eq!(set.max_inclusive_leq(6), Some(6));
        assert_eq!(set.max_inclusive_leq(5), Some(3)); // 5, 4 はない
        assert_eq!(set.max_inclusive_leq(4), Some(3));
        assert_eq!(set.max_inclusive_leq(3), Some(3));
        assert_eq!(set.max_inclusive_leq(2), Some(2));
        assert_eq!(set.max_inclusive_leq(1), Some(1));
        assert_eq!(set.max_inclusive_leq(0), None);

        // Empty set
        let empty = RangeSet::new();
        assert_eq!(empty.min_inclusive_geq(0), None);
        assert_eq!(empty.max_inclusive_leq(0), None);
    }

    #[test]
    fn test_min_max() {
        let mut set = RangeSet::new();
        assert_eq!(set.min(), None);
        assert_eq!(set.max(), None);

        set.insert_range(5, 10);
        assert_eq!(set.min(), Some(5));
        assert_eq!(set.max(), Some(9));

        set.insert_range(0, 3);
        assert_eq!(set.min(), Some(0));
        assert_eq!(set.max(), Some(9));

        set.insert_range(12, 15);
        assert_eq!(set.min(), Some(0));
        assert_eq!(set.max(), Some(14));

        set.remove_range(0, 3);
        assert_eq!(set.min(), Some(5));
        assert_eq!(set.max(), Some(14));
    }

    #[test]
    fn test_is_disjoint() {
        let mut set = RangeSet::new();
        set.insert_range(0, 5); // [0, 5)
        set.insert_range(10, 15); // [10, 15)

        // Disjoint ranges
        assert!(set.is_disjoint(5, 10)); // Between intervals
        assert!(set.is_disjoint(-5, 0)); // Before all
        assert!(set.is_disjoint(15, 20)); // After all
        assert!(set.is_disjoint(6, 9)); // Subset of gap

        // Overlapping ranges
        assert!(!set.is_disjoint(0, 5)); // Exact match
        assert!(!set.is_disjoint(4, 6)); // Overlap boundary
        assert!(!set.is_disjoint(-1, 1)); // Overlap start
        assert!(!set.is_disjoint(0, 1)); // Subset
        assert!(!set.is_disjoint(10, 15));
        assert!(!set.is_disjoint(14, 16));
        assert!(!set.is_disjoint(0, 15)); // Covers multiple

        // Edge cases
        assert!(set.is_disjoint(0, 0)); // Empty range is always disjoint
        assert!(set.is_disjoint(2, 2));
    }

    #[test]
    fn test_is_covered_by() {
        let mut set = RangeSet::new();
        set.insert_range(2, 5); // [2, 5)
        set.insert_range(8, 10); // [8, 10)

        // Covered
        assert!(set.is_covered_by(2, 10)); // Exact bounds (min=2, max=9 < 10)
        assert!(set.is_covered_by(0, 12)); // Larger bounds
        assert!(set.is_covered_by(2, 15));
        assert!(set.is_covered_by(-5, 10));

        // Not covered
        assert!(!set.is_covered_by(3, 10)); // Cut left (min is 2)
        assert!(!set.is_covered_by(2, 9)); // Cut right (max is 9, range ends at 9 i.e. max element 8) -> wait.
        // max() returns 9 (from [8, 10)). range [2, 9) includes 8 but not 9.
        // Elements are {2, 3, 4, 8, 9}.
        // [2, 9) contains {2, 3, 4, 5, 6, 7, 8}.
        // 9 is in set but not in [2, 9). So false. Correct.

        assert!(!set.is_covered_by(5, 8)); // Gap only

        // Empty set
        let empty_set = RangeSet::new();
        assert!(empty_set.is_covered_by(0, 10));
        assert!(empty_set.is_covered_by(0, 0)); // Empty set is covered by empty range
    }

    #[test]
    #[ignore]
    fn test_random_ops_against_naive() {
        let mut rng = StdRng::seed_from_u64(42);

        for _ in 0..300 {
            // テスト範囲を決定。狭い範囲で衝突を多く発生させる
            let range_min = -2;
            let range_max = rng.random_range(1..=20);

            let mut set = RangeSet::new();
            let mut naive = NaiveRangeSet::new();

            let num_ops = 100;

            for _ in 0..num_ops {
                // 操作の種類をランダムに選択
                // 0: insert_range, 1: remove_range, 2: insert, 3: remove
                let op_type = rng.random_range(0..4);

                let l = rng.random_range(range_min..=range_max);
                let r = rng.random_range(l..=range_max);
                let x = rng.random_range(range_min..range_max);

                match op_type {
                    0 => {
                        set.insert_range(l, r);
                        naive.insert_range(l, r);
                    }
                    1 => {
                        set.remove_range(l, r);
                        naive.remove_range(l, r);
                    }
                    2 => {
                        set.insert(x);
                        naive.insert(x);
                    }
                    3 => {
                        set.remove(x);
                        naive.remove(x);
                    }
                    _ => unreachable!(),
                };

                // --- 1. 状態の一貫性チェック ---
                assert_eq!(set.len(), naive.len(), "len mismatch. set: {:?}", set);
                assert_eq!(
                    set.is_empty(),
                    naive.is_empty(),
                    "is_empty mismatch. set: {:?}",
                    set
                );
                assert_eq!(set.min(), naive.min(), "min mismatch. set: {:?}", set);
                assert_eq!(set.max(), naive.max(), "max mismatch. set: {:?}", set);

                // --- 2. 点クエリのチェック (狭い範囲での全探索) ---
                for i in range_min..=range_max {
                    assert_eq!(
                        set.contains(i),
                        naive.contains(i),
                        "contains({}) mismatch. set: {:?}",
                        i,
                        set
                    );
                    assert_eq!(
                        set.min_exclusive_geq(i),
                        naive.min_exclusive_geq(i),
                        "min_exclusive_geq({}) mismatch. set: {:?}",
                        i,
                        set
                    );
                    assert_eq!(
                        set.max_exclusive_leq(i),
                        naive.max_exclusive_leq(i),
                        "max_exclusive_leq({}) mismatch. set: {:?}",
                        i,
                        set
                    );
                    assert_eq!(
                        set.min_inclusive_geq(i),
                        naive.min_inclusive_geq(i),
                        "min_inclusive_geq({}) mismatch. set: {:?}",
                        i,
                        set
                    );
                    assert_eq!(
                        set.max_inclusive_leq(i),
                        naive.max_inclusive_leq(i),
                        "max_inclusive_leq({}) mismatch. set: {:?}",
                        i,
                        set
                    );
                }

                // --- 3. 区間クエリのチェック (ランダムサンプリング) ---
                for _ in 0..5 {
                    let ql = rng.random_range(range_min..=range_max);
                    let qr = rng.random_range(ql..=range_max);

                    assert_eq!(
                        set.covers(ql, qr),
                        naive.covers(ql, qr),
                        "covers({}, {}) mismatch. set: {:?}",
                        ql,
                        qr,
                        set
                    );
                    assert_eq!(
                        set.is_disjoint(ql, qr),
                        naive.is_disjoint(ql, qr),
                        "is_disjoint({}, {}) mismatch. set: {:?}",
                        ql,
                        qr,
                        set
                    );
                    assert_eq!(
                        set.is_covered_by(ql, qr),
                        naive.is_covered_by(ql, qr),
                        "is_covered_by({}, {}) mismatch. set: {:?}",
                        ql,
                        qr,
                        set
                    );
                }
            }
        }
    }

    /// `BTreeSet` を使った RangeSet のナイーブな実装 (テスト用)
    #[derive(Debug, Clone)]
    struct NaiveRangeSet {
        set: std::collections::BTreeSet<i64>,
    }

    impl NaiveRangeSet {
        fn new() -> Self {
            Self {
                set: std::collections::BTreeSet::new(),
            }
        }

        fn insert_range(&mut self, l: i64, r: i64) {
            for i in l..r {
                self.set.insert(i);
            }
        }

        fn remove_range(&mut self, l: i64, r: i64) {
            for i in l..r {
                self.set.remove(&i);
            }
        }

        fn insert(&mut self, x: i64) {
            self.set.insert(x);
        }

        fn remove(&mut self, x: i64) {
            self.set.remove(&x);
        }

        fn contains(&self, x: i64) -> bool {
            self.set.contains(&x)
        }

        fn covers(&self, l: i64, r: i64) -> bool {
            assert!(l <= r);
            (l..r).all(|i| self.set.contains(&i))
        }

        fn len(&self) -> i64 {
            self.set.len() as i64
        }

        fn is_empty(&self) -> bool {
            self.set.is_empty()
        }

        fn min(&self) -> Option<i64> {
            self.set.iter().min().copied()
        }

        fn max(&self) -> Option<i64> {
            self.set.iter().max().copied()
        }

        fn is_disjoint(&self, l: i64, r: i64) -> bool {
            (l..r).all(|i| !self.set.contains(&i))
        }

        fn is_covered_by(&self, l: i64, r: i64) -> bool {
            self.set.iter().all(|x| (l..r).contains(x))
        }

        fn min_exclusive_geq(&self, x: i64) -> i64 {
            (x..).find(|&i| !self.set.contains(&i)).unwrap()
        }

        fn max_exclusive_leq(&self, x: i64) -> i64 {
            (i64::MIN..=x)
                .rev()
                .find(|&i| !self.set.contains(&i))
                .unwrap()
        }

        fn min_inclusive_geq(&self, x: i64) -> Option<i64> {
            self.set.range(x..).min().copied()
        }

        fn max_inclusive_leq(&self, x: i64) -> Option<i64> {
            self.set.range(..=x).max().copied()
        }
    }
}
