use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_set::*;")]
// 参考: 要素の追加・削除と mex を対数時間で処理するよ - えびちゃんの日記
// https://rsk0315.hatenablog.com/entry/2020/10/11/125049
pub mod range_set {
    use std::collections::BTreeSet;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct RangeSet {
        set: BTreeSet<(i64, i64)>,
        count: usize,
    }
    impl Default for RangeSet {
        fn default() -> Self {
            Self::new()
        }
    }

    impl RangeSet {
        pub fn new() -> RangeSet {
            RangeSet {
                set: vec![(i64::MIN, i64::MIN), (i64::MAX, i64::MAX)] // 番兵
                    .into_iter()
                    .collect(),
                count: 0,
            }
        }

        pub fn iter(&self) -> impl Iterator<Item = i64> + '_ {
            self.set
                .iter()
                .copied()
                .filter(|&(l, r)| (l, r) != (i64::MIN, i64::MIN) && (l, r) != (i64::MAX, i64::MAX)) // 番兵は除く
                .flat_map(|(left, right)| left..=right)
        }

        pub fn insert(&mut self, x: i64) -> bool {
            if self.contains(x) {
                return false;
            }

            // 番兵がいるので unwrap 可能。
            let &(prev_l, prev_r) = self.set.range(..(x + 1, x + 1)).max().unwrap();
            let &(next_l, next_r) = self.set.range((x + 1, x + 1)..).min().unwrap();

            // 以下の4パターンがある ('x' が insert する値。"[ ]" が既存の区間 )
            // [ ]x[ ]
            // [ ]x  [ ]
            // [ ]  x[ ]
            // [ ]  x  [ ]

            if prev_r + 1 == x && x == next_l - 1 {
                self.set.remove(&(prev_l, prev_r));
                self.set.remove(&(next_l, next_r));
                self.set.insert((prev_l, next_r));
            } else if prev_r + 1 == x {
                self.set.remove(&(prev_l, prev_r));
                self.set.insert((prev_l, x));
            } else if x == next_l - 1 {
                self.set.remove(&(next_l, next_r));
                self.set.insert((x, next_r));
            } else {
                self.set.insert((x, x));
            }

            self.count += 1;

            true
        }

        pub fn remove(&mut self, x: i64) -> bool {
            if !self.contains(x) {
                return false;
            }

            let &(current_l, current_r) = self.set.range(..(x + 1, x + 1)).max().unwrap();

            // 削除のパターンは以下の4通り
            //  [x]
            // → (消滅)
            //
            //  [x    ]
            // →  [   ]
            //
            //  [    x]
            //→ [   ]
            //
            //  [  x  ]
            // →[ ] [ ]

            if current_l == x && x == current_r {
                self.set.remove(&(current_l, current_r));
            } else if current_l == x {
                self.set.remove(&(current_l, current_r));
                self.set.insert((x + 1, current_r));
            } else if x == current_r {
                self.set.remove(&(current_l, current_r));
                self.set.insert((current_l, x - 1));
            } else {
                self.set.remove(&(current_l, current_r));
                self.set.insert((current_l, x - 1));
                self.set.insert((x + 1, current_r));
            }

            self.count -= 1;
            true
        }

        pub fn len(&self) -> usize {
            self.count
        }

        pub fn is_empty(&self) -> bool {
            self.count == 0
        }

        pub fn contains(&self, x: i64) -> bool {
            let &(l, r) = self.set.range(..(x + 1, x + 1)).max().unwrap();
            (l..=r).contains(&x)
        }

        /// x 以上で self に入っていない値の最小値を返す (いわゆる mex)
        pub fn min_exclusive_geq(&self, x: i64) -> i64 {
            let &(l, r) = self.set.range(..(x + 1, x + 1)).max().unwrap();
            if (l..=r).contains(&x) {
                r + 1
            } else {
                x
            }
        }
        /// x 以下で self に入っていない値の最大値を返す
        pub fn max_exclusive_leq(&self, x: i64) -> i64 {
            let &(l, r) = self.set.range(..(x + 1, x + 1)).max().unwrap();
            if (l..=r).contains(&x) {
                l - 1
            } else {
                x
            }
        }
    }
    impl FromIterator<i64> for RangeSet {
        fn from_iter<I: IntoIterator<Item = i64>>(iter: I) -> RangeSet {
            let mut set = RangeSet::new();
            for x in iter {
                set.insert(x);
            }
            set
        }
    }
}

#[cfg(test)]
mod tests {
    use super::range_set::RangeSet;

    #[test]
    fn test_new() {
        let set: RangeSet = RangeSet::new();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_insert() {
        let mut set = RangeSet::new();
        assert!(set.insert(1));
        assert!(!set.is_empty());
        assert_eq!(set.len(), 1);
        assert!(set.contains(1));

        assert!(!set.insert(1));
        assert_eq!(set.len(), 1);
        assert!(set.contains(1));

        assert!(set.insert(2));
        assert_eq!(set.len(), 2);
        assert!(set.contains(2));
    }

    #[test]
    fn test_remove() {
        let mut set: RangeSet = vec![1, 1, 2].into_iter().collect();
        dbg!(&set);

        assert!(set.remove(1));
        assert_eq!(set.len(), 1);

        dbg!(&set);

        assert!(set.remove(2));
        assert!(set.is_empty());

        // Test removing an element not in the set
        assert!(!set.remove(3));
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_contains() {
        let set: RangeSet = vec![1, 1, 2].into_iter().collect();

        assert!(!set.contains(0));
        assert!(set.contains(1));
        assert!(set.contains(2));
        assert!(!set.contains(3));
    }

    #[test]
    fn test_from_iter_and_iter() {
        let set: RangeSet = vec![1, 1, 2, 3].into_iter().collect();

        assert_eq!(set.len(), 3);

        let elements: Vec<_> = set.iter().collect();
        assert_eq!(elements, vec![1, 2, 3]);
    }

    #[test]
    fn test_is_empty() {
        let mut set: RangeSet = RangeSet::new();
        assert!(set.is_empty());

        set.insert(1);
        assert!(!set.is_empty());

        set.remove(1);
        assert!(set.is_empty());
    }

    #[test]
    fn test_min_exclusive() {
        let set: RangeSet = vec![1, 3, 4].into_iter().collect();

        assert_eq!(set.min_exclusive_geq(0), 0);
        assert_eq!(set.min_exclusive_geq(1), 2);
        assert_eq!(set.min_exclusive_geq(2), 2);
        assert_eq!(set.min_exclusive_geq(3), 5);
        assert_eq!(set.min_exclusive_geq(4), 5);
        assert_eq!(set.min_exclusive_geq(5), 5);
    }

    #[test]
    fn test_max_exclusive() {
        let set: RangeSet = vec![1, 3, 4].into_iter().collect();

        assert_eq!(set.max_exclusive_leq(0), 0);
        assert_eq!(set.max_exclusive_leq(1), 0);
        assert_eq!(set.max_exclusive_leq(2), 2);
        assert_eq!(set.max_exclusive_leq(3), 2);
        assert_eq!(set.max_exclusive_leq(4), 2);
        assert_eq!(set.max_exclusive_leq(5), 5);
    }

    #[test]
    fn test_edge_cases() {
        {
            // 空集合のテスト
            let mut empty: RangeSet = RangeSet::new();
            assert!(!empty.remove(1));
            assert!(!empty.contains(1));

            assert_eq!(empty.min_exclusive_geq(2), 2);
            assert_eq!(empty.max_exclusive_leq(2), 2);
        }

        {
            // 0 や 負の数でのテスト

            let mut set: RangeSet = RangeSet::new();
            set.insert(-1);
            set.insert(0);
            assert!(set.contains(-1));
            assert!(set.contains(0));

            assert_eq!(set.min_exclusive_geq(-2), -2);
            assert_eq!(set.min_exclusive_geq(-1), 1);
            assert_eq!(set.min_exclusive_geq(0), 1);
            assert_eq!(set.min_exclusive_geq(1), 1);

            assert_eq!(set.max_exclusive_leq(-2), -2);
            assert_eq!(set.max_exclusive_leq(-1), -2);
            assert_eq!(set.max_exclusive_leq(0), -2);
            assert_eq!(set.max_exclusive_leq(1), 1);
        }
    }

    // ランダムテスト。重いのでコメントアウト。
    // #[test]
    // fn random_insert_delete_test() {
    //     use rand::{rngs::SmallRng, Rng, SeedableRng};
    //     let mut rng = SmallRng::from_os_rng();
    //     let mut range_set = RangeSet::new();
    //     let mut reference_set = std::collections::BTreeSet::new();

    //     const TEST_CASES: usize = 10000; // ランダム操作の回数
    //     const MAX_VAL: i64 = 5; // 挿入する値の範囲

    //     for _ in 0..TEST_CASES {
    //         // 挿入または削除をランダムに選択
    //         if rng.gen_bool(0.5) {
    //             // insert操作
    //             let val = rng.random_range(0..MAX_VAL);
    //             let was_inserted = range_set.insert(val);
    //             let ref_was_inserted = reference_set.insert(val);

    //             // 挿入結果が正しいかチェック
    //             assert_eq!(
    //                 was_inserted, ref_was_inserted,
    //                 "Insert operation failed for value {}",
    //                 val
    //             );
    //         } else {
    //             // delete操作
    //             let val = rng.random_range(0..MAX_VAL);
    //             let was_removed = range_set.remove(val);
    //             let ref_was_removed = reference_set.remove(&val);

    //             assert_eq!(
    //                 was_removed, ref_was_removed,
    //                 "Remove operation failed for value {}",
    //                 val
    //             );
    //         }

    //         // range_setの要素がreference_setと一致しているかチェック
    //         let range_set_elements: Vec<i64> = range_set.iter().collect();
    //         let reference_elements: Vec<i64> = reference_set.iter().copied().collect();
    //         assert_eq!(range_set_elements, reference_elements, "Sets do not match");
    //     }
    // }

    // #[test]
    // fn random_min_max_exclusive_test() {
    //     use rand::{rngs::SmallRng, Rng, SeedableRng};
    //     let mut rng = SmallRng::from_os_rng(); // SmallRngを初期化
    //     let mut range_set = RangeSet::new();
    //     let mut reference_set = std::collections::BTreeSet::new(); // 正しい動作確認用のBTreeSet

    //     const TEST_CASES: usize = 1000; // ランダム操作の回数
    //     const MAX_VAL: i64 = 30; // 挿入する値の範囲

    //     for _ in 0..TEST_CASES {
    //         // 挿入または削除をランダムに選択
    //         if rng.gen_bool(0.5) {
    //             // insert操作
    //             let val = rng.random_range(0..MAX_VAL);
    //             range_set.insert(val);
    //             reference_set.insert(val);
    //         } else {
    //             // delete操作
    //             let val = rng.random_range(0..MAX_VAL);
    //             range_set.remove(val);
    //             reference_set.remove(&val);
    //         }

    //         // ランダムに値を選び、min_exclusive_geqの結果を検証
    //         let x = rng.random_range(0..MAX_VAL);
    //         let min_exclusive_geq = range_set.min_exclusive_geq(x);
    //         let mut expected_min_exclusive_geq = x;
    //         while reference_set.contains(&expected_min_exclusive_geq) {
    //             expected_min_exclusive_geq += 1;
    //         }
    //         assert_eq!(
    //             min_exclusive_geq, expected_min_exclusive_geq,
    //             "min_exclusive_geq failed for value {}: got {}, expected {}",
    //             x, min_exclusive_geq, expected_min_exclusive_geq
    //         );

    //         // ランダムに値を選び、max_exclusive_leqの結果を検証
    //         let x = rng.random_range(0..MAX_VAL);
    //         let max_exclusive_leq = range_set.max_exclusive_leq(x);
    //         let mut expected_max_exclusive_leq = x;
    //         while reference_set.contains(&expected_max_exclusive_leq) {
    //             expected_max_exclusive_leq -= 1;
    //         }
    //         assert_eq!(
    //             max_exclusive_leq, expected_max_exclusive_leq,
    //             "max_exclusive_leq failed for value {}: got {}, expected {}",
    //             x, max_exclusive_leq, expected_max_exclusive_leq
    //         );
    //     }
    // }
}
