//! 整数の集合を隣り合わない半開区間の直和で管理するデータ構造。
//!
//! # 機能
//! - 区間の追加 (`insert_range`)
//! - 区間の削除 (`remove_range`)
//! - 点が区間集合に含まれるかの判定 (`contains`)
//! - 区間が完全にカバーされているかの判定 (`covered`)
//! - 全区間の長さの合計 (`len`)

use std::collections::BTreeMap;
use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct RangeSet2 {
    map: BTreeMap<i64, i64>, // 半開区間 [l, r) を map.insert(l, r) で管理
    total_length: i64,
}

impl Default for RangeSet2 {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for RangeSet2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(self.map.iter().map(|(&l, &r)| (l, r)))
            .finish()
    }
}

impl RangeSet2 {
    /// 新しい `RangeSet2` を作成する。
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            total_length: 0,
        }
    }

    /// 区間 `[l, r)` を挿入する。
    ///
    /// 重複または隣接する区間はマージされる。
    ///
    /// # 計算量
    /// amortized O(log N)
    pub fn insert_range(&mut self, l: i64, r: i64) {
        if l >= r {
            return;
        }

        let mut start = l;
        let mut end = r;
        let mut removed_len = 0;

        // --- l より前に開始し、[l, r) とマージ可能な区間を探す ---
        if let Some((&prev_l, &prev_r)) = self.map.range(..=l).max() {
            if prev_r >= l {
                start = prev_l;
                end = end.max(prev_r);
                removed_len += prev_r - prev_l;
                self.map.remove(&prev_l);
            }
        }

        // --- 新しい区間 [start, end) と重なる後続の区間をマージする ---
        let mut to_remove = Vec::new();
        for (&l_i, &r_i) in self.map.range(start..) {
            if l_i > end {
                break;
            }
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

    /// 区間 `[l, r)` を削除する。
    ///
    /// # 計算量
    /// amortized O(log N)
    pub fn remove_range(&mut self, l: i64, r: i64) {
        if l >= r {
            return;
        }

        let mut to_add = Vec::new();
        let mut to_remove = Vec::new();
        let mut len_change = 0;

        // --- [l, r) と重なる区間をすべて探し、処理する ---
        // l の直前に開始し、l と重なる可能性のある区間を探す
        if let Some((&prev_l, &prev_r)) = self.map.range(..=l).max() {
            if prev_r > l {
                to_remove.push(prev_l);
                len_change -= prev_r - prev_l; // 元の区間を削除

                // [prev_l, l) の部分は残す
                if prev_l < l {
                    to_add.push((prev_l, l));
                    len_change += l - prev_l; // 新しい区間を追加
                }
                // [r, prev_r) の部分が残る可能性
                if prev_r > r {
                    to_add.push((r, prev_r));
                    len_change += prev_r - r; // 新しい区間を追加
                }
            }
        }

        // l 以降に開始し、[l, r) と重なる区間を列挙
        for (&l_i, &r_i) in self.map.range(l..).take_while(|&(&l_i, _)| l_i < r) {
            to_remove.push(l_i);
            len_change -= r_i - l_i; // 元の区間を削除

            // [r, r_i) の部分が残る可能性
            if r_i > r {
                to_add.push((r, r_i));
                len_change += r_i - r; // 新しい区間を追加
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

    /// 点 `x` がいずれかの区間に含まれているかを返す。
    pub fn contains(&self, x: i64) -> bool {
        // x を含みうる区間は、x 以前に始まったものだけ
        // そのような区間 [l, r) のうち、l が最大のものを探す
        if let Some((&l, &r)) = self.map.range(..=x).max() {
            // l <= x < r であれば含まれる
            l <= x && x < r
        } else {
            false
        }
    }

    /// 区間 `[l, r)` が完全に区間集合にカバーされているかを返す。
    pub fn covered(&self, l: i64, r: i64) -> bool {
        if l >= r {
            return true;
        }
        if let Some((&start, &end)) = self.map.range(..=l).max() {
            start <= l && r <= end
        } else {
            false
        }
    }

    /// 区間集合が空かどうかを返す。
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// 管理している区間の総数を返す。
    pub fn count_ranges(&self) -> usize {
        self.map.len()
    }

    /// 全区間の長さの合計を返す。O(1)で計算できる。
    pub fn len(&self) -> i64 {
        self.total_length
    }

    /// 管理しているすべての区間 `[l, r)` のイテレータを返す。
    pub fn ranges(&self) -> impl Iterator<Item = (i64, i64)> + '_ {
        self.map.iter().map(|(&l, &r)| (l, r))
    }

    /// x 以上で self に入っていない値の最小値を返す (いわゆる mex)
    pub fn min_exclusive_geq(&self, x: i64) -> i64 {
        if let Some((&l, &r)) = self.map.range(..=x).max() {
            if l <= x && x < r { r } else { x }
        } else {
            x
        }
    }

    /// x 以下で self に入っていない値の最大値を返す
    pub fn max_exclusive_leq(&self, x: i64) -> i64 {
        if let Some((&l, &r)) = self.map.range(..=x).max() {
            if l <= x && x < r { l - 1 } else { x }
        } else {
            x
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{Rng, SeedableRng, rngs::StdRng};

    #[test]
    fn test_insert_non_overlapping() {
        let mut set = RangeSet2::new();
        set.insert_range(0, 5);
        set.insert_range(10, 15);
        assert_eq!(set.map.len(), 2);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(0, 5), (10, 15)]);
    }

    #[test]
    fn test_insert_overlapping_merge() {
        let mut set = RangeSet2::new();
        set.insert_range(0, 5);
        set.insert_range(3, 8); // [0, 5) と [3, 8) -> [0, 8)
        assert_eq!(set.map.len(), 1);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(0, 8)]);

        set.insert_range(10, 15);
        set.insert_range(-2, 1); // [-2, 1) と [0, 8) -> [-2, 8)
        assert_eq!(set.map.len(), 2);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(-2, 8), (10, 15)]);
    }

    #[test]
    fn test_insert_adjacent_merge() {
        let mut set = RangeSet2::new();
        set.insert_range(0, 5);
        set.insert_range(5, 10); // [0, 5) と [5, 10) -> [0, 10)
        assert_eq!(set.map.len(), 1);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(0, 10)]);

        set.insert_range(-5, 0); // [-5, 0) と [0, 10) -> [-5, 10)
        assert_eq!(set.map.len(), 1);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(-5, 10)]);
    }

    #[test]
    fn test_insert_containing_merge() {
        let mut set = RangeSet2::new();
        set.insert_range(0, 10);
        set.insert_range(3, 7); // [3, 7) は [0, 10) に含まれる
        assert_eq!(set.map.len(), 1);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(0, 10)]);

        set.insert_range(-5, 15); // [0, 10) は [-5, 15) に含まれる
        assert_eq!(set.map.len(), 1);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(-5, 15)]);
    }

    #[test]
    fn test_insert_multiple_merge() {
        let mut set = RangeSet2::new();
        set.insert_range(0, 2);
        set.insert_range(4, 6);
        set.insert_range(8, 10);
        set.insert_range(1, 9); // [1, 9) が [0,2), [4,6), [8,10) をマージ
        assert_eq!(set.map.len(), 1);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(0, 10)]);
    }

    #[test]
    fn test_remove_no_overlap() {
        let mut set = RangeSet2::new();
        set.insert_range(10, 20);
        set.remove_range(0, 5);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(10, 20)]);
    }

    #[test]
    fn test_remove_total() {
        let mut set = RangeSet2::new();
        set.insert_range(10, 20);
        set.remove_range(10, 20);
        assert!(set.is_empty());

        set.insert_range(10, 20);
        set.remove_range(5, 25);
        assert!(set.is_empty());
    }

    #[test]
    fn test_remove_split() {
        let mut set = RangeSet2::new();
        set.insert_range(10, 20);
        set.remove_range(13, 17); // [10, 20) -> [10, 13) と [17, 20)
        assert_eq!(set.map.len(), 2);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(10, 13), (17, 20)]);
    }

    #[test]
    fn test_remove_left_side() {
        let mut set = RangeSet2::new();
        set.insert_range(10, 20);
        set.remove_range(5, 15);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(15, 20)]);
    }

    #[test]
    fn test_remove_right_side() {
        let mut set = RangeSet2::new();
        set.insert_range(10, 20);
        set.remove_range(15, 25);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(10, 15)]);
    }

    #[test]
    fn test_remove_multiple_ranges() {
        let mut set = RangeSet2::new();
        set.insert_range(0, 5);
        set.insert_range(10, 15);
        set.insert_range(20, 25);
        set.remove_range(3, 22);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(0, 3), (22, 25)]);
    }

    #[test]
    fn test_contains() {
        let mut set = RangeSet2::new();
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
    fn test_covered() {
        let mut set = RangeSet2::new();
        set.insert_range(0, 10);
        assert!(set.covered(2, 8));
        assert!(set.covered(0, 10));
        assert!(!set.covered(0, 11));
        assert!(!set.covered(-1, 10));
    }

    #[test]
    fn test_len() {
        let mut set = RangeSet2::new();
        set.insert_range(0, 5);
        set.insert_range(10, 15);
        assert_eq!(set.len(), 10);
        set.insert_range(4, 11);
        assert_eq!(set.len(), 15);
    }

    #[test]
    #[ignore]
    fn test_random_operations() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut set = RangeSet2::new();
        let mut reference = vec![]; // bool 配列で区間を管理

        const MAX_POS: i64 = 200;
        reference.resize(MAX_POS as usize, false);

        for _ in 0..10000 {
            let l = rng.random_range(0..MAX_POS);
            let r = rng.random_range(l..=MAX_POS);

            if rng.random_bool(0.5) {
                // Insert
                set.insert_range(l, r);
                for i in l..r {
                    reference[i as usize] = true;
                }
            } else {
                // Remove
                set.remove_range(l, r);
                for i in l..r {
                    reference[i as usize] = false;
                }
            }

            // 検証
            let mut current_pos = 0;
            for (start, end) in set.ranges() {
                // ギャップの部分が false であることを確認
                for i in current_pos..start {
                    assert!(!reference[i as usize], "pos {} should be false", i);
                }
                // 区間の部分が true であることを確認
                for i in start..end {
                    assert!(reference[i as usize], "pos {} should be true", i);
                }
                current_pos = end;
            }
            // 最後の区間以降が false であることを確認
            for i in current_pos..MAX_POS {
                assert!(!reference[i as usize], "pos {} should be false", i);
            }
        }
    }

    #[test]
    fn test_mex() {
        let mut set = RangeSet2::new();
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
        let empty_set = RangeSet2::new();
        assert_eq!(empty_set.min_exclusive_geq(100), 100);
        assert_eq!(empty_set.max_exclusive_leq(100), 100);

        let mut set_neg = RangeSet2::new();
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
}
