use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_set2::*;")]
pub mod range_set2 {

    use std::collections::BTreeMap;

    /// 整数の集合を隣り合わない半開区間の直和で管理するデータ構造。
    ///
    /// # 機能
    /// - 区間内の整数の追加 (`insert_range`)
    /// - 区間内の整数の削除 (`remove_range`)
    /// - 点が区間集合に含まれるかの判定 (`contains`)
    /// - 区間が完全にカバーされているかの判定 (`covers`)
    /// - 全区間の長さの合計 (`len`)
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct RangeSet2 {
        map: BTreeMap<i64, i64>, // key: l, value: r で半開区間 [l, r) を管理
        total_length: i64,
    }

    impl Default for RangeSet2 {
        fn default() -> Self {
            Self::new()
        }
    }

    impl RangeSet2 {
        /// 空の `RangeSet2` を作成する。
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
    use super::range_set2::*;
    use rand::{Rng, SeedableRng, rngs::StdRng};

    #[test]
    fn test_insert_non_overlapping() {
        let mut set = RangeSet2::new();
        set.insert_range(0, 5);
        set.insert_range(10, 15);
        assert_eq!(set.ranges().count(), 2);
        assert_eq!(set.ranges().collect::<Vec<_>>(), vec![(0, 5), (10, 15)]);
    }

    #[test]
    fn test_insert_overlapping_merge() {
        let mut set = RangeSet2::new();
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
        let mut set = RangeSet2::new();
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
        let mut set = RangeSet2::new();
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
        let mut set = RangeSet2::new();
        set.insert_range(0, 2);
        set.insert_range(4, 6);
        set.insert_range(8, 10);
        set.insert_range(1, 9); // [1, 9) が [0,2), [4,6), [8,10) をマージ
        assert_eq!(set.ranges().count(), 1);
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
        assert_eq!(set.ranges().count(), 2);
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
    fn test_covers() {
        let mut set = RangeSet2::new();
        set.insert_range(0, 10);
        assert!(set.covers(2, 8));
        assert!(set.covers(0, 10));
        assert!(!set.covers(0, 11));
        assert!(!set.covers(-1, 10));
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
    fn test_is_empty() {
        let mut set = RangeSet2::new();
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

    #[test]
    #[ignore]
    fn test_random_ops_against_naive() {
        let mut rng = StdRng::seed_from_u64(42);

        for _ in 0..500 {
            let len = rng.random_range(1..=30);
            let mut set = RangeSet2::new();
            let mut naive = NaiveRangeSet::new(len);

            let num_ops = 200;

            for op_idx in 0..num_ops {
                let l = rng.random_range(0..=len as i64);
                let r = rng.random_range(l..=len as i64);

                let op_type = rng.random_range(0..2);

                let (op_name, l_op, r_op) = if op_type == 0 {
                    set.insert_range(l, r);
                    naive.insert_range(l, r);
                    ("insert", l, r)
                } else {
                    set.remove_range(l, r);
                    naive.remove_range(l, r);
                    ("remove", l, r)
                };

                // Assertion
                let base_context = format!(
                    "Operation[{}]: {}({}, {}), LEN: {}",
                    op_idx, op_name, l_op, r_op, len
                );

                assert_eq!(
                    set.len(),
                    naive.len(),
                    "Failed len check: {}\nset: {:?}\nnaive: {:?}",
                    base_context,
                    set,
                    naive
                );

                assert_eq!(
                    set.is_empty(),
                    naive.is_empty(),
                    "Failed is_empty check: {}\nset: {:?}\nnaive: {:?}",
                    base_context,
                    set,
                    naive
                );

                for i in -2..=len as i64 + 2 {
                    assert_eq!(
                        set.contains(i),
                        naive.contains(i),
                        "Failed contains({}) check: {}\nset: {:?}\nnaive: {:?}",
                        i,
                        base_context,
                        set,
                        naive
                    );
                    assert_eq!(
                        set.min_exclusive_geq(i),
                        naive.min_exclusive_geq(i),
                        "Failed min_exclusive_geq({}) check: {}\nset: {:?}\nnaive: {:?}",
                        i,
                        base_context,
                        set,
                        naive
                    );
                    assert_eq!(
                        set.max_exclusive_leq(i),
                        naive.max_exclusive_leq(i),
                        "Failed max_exclusive_leq({}) check: {}\nset: {:?}\nnaive: {:?}",
                        i,
                        base_context,
                        set,
                        naive
                    );
                }

                for _ in 0..10 {
                    let l_cover = rng.random_range(-2..=len as i64 + 2);
                    let r_cover = rng.random_range(l_cover..=len as i64 + 2);
                    assert_eq!(
                        set.covers(l_cover, r_cover),
                        naive.covers(l_cover, r_cover),
                        "Failed covers({}, {}) check: {}\nset: {:?}\nnaive: {:?}",
                        l_cover,
                        r_cover,
                        base_context,
                        set,
                        naive
                    );
                }
            }
        }
    }

    /// `Vec<bool>` を使った RangeSet のナイーブな実装 (テスト用)
    #[derive(Debug)]
    struct NaiveRangeSet {
        vec: Vec<bool>,
        len: usize,
    }

    impl NaiveRangeSet {
        fn new(len: usize) -> Self {
            Self {
                vec: vec![false; len],
                len,
            }
        }

        fn insert_range(&mut self, l: i64, r: i64) {
            assert!(l >= 0 && r <= self.len as i64);
            for i in l..r {
                self.vec[i as usize] = true;
            }
        }

        fn remove_range(&mut self, l: i64, r: i64) {
            assert!(l >= 0 && r <= self.len as i64);
            for i in l..r {
                self.vec[i as usize] = false;
            }
        }

        fn contains(&self, x: i64) -> bool {
            if x >= 0 && x < self.len as i64 {
                self.vec[x as usize]
            } else {
                false
            }
        }

        fn covers(&self, l: i64, r: i64) -> bool {
            assert!(l <= r);
            if l == r {
                return true;
            }
            (l..r).all(|i| self.contains(i))
        }

        fn len(&self) -> i64 {
            self.vec.iter().filter(|&&b| b).count() as i64
        }

        fn is_empty(&self) -> bool {
            self.len() == 0
        }

        fn min_exclusive_geq(&self, x: i64) -> i64 {
            if !self.contains(x) {
                return x;
            }
            (x + 1..self.len as i64)
                .find(|&i| !self.contains(i))
                .unwrap_or(self.len as i64)
        }

        fn max_exclusive_leq(&self, x: i64) -> i64 {
            if !self.contains(x) {
                return x;
            }
            (0..x).rev().find(|&i| !self.contains(i)).unwrap_or(-1)
        }
    }
}
