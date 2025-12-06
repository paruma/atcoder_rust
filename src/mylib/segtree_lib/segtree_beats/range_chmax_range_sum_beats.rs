use cargo_snippet::snippet;

use super::abstract_segtree_beats::abstract_segtree_beats::{MapMonoidBeats, MonoidBeats};

#[snippet(
    prefix = "use range_chmax_range_sum::*;",
    include = "abstract_segtree_beats"
)]
pub mod range_chmax_range_sum {
    use super::{MapMonoidBeats, MonoidBeats};
    use crate::mylib::segtree_lib::segtree_beats::abstract_segtree_beats::abstract_segtree_beats::SegtreeBeats;
    use itertools::Itertools;
    use std::{
        cmp::{max, min},
        convert::Infallible,
        ops::RangeBounds,
    };

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct RangeSum {
        pub sum: i64,
        pub len: usize,
        pub min: i64,
        pub min_cnt: usize,
        pub min_2nd: i64,
    }

    impl RangeSum {
        pub fn unit(x: i64) -> Option<RangeSum> {
            Some(RangeSum {
                sum: x,
                len: 1,
                min: x,
                min_cnt: 1,
                min_2nd: i64::MAX,
            })
        }
    }

    fn second_smallest(a0: i64, a1: i64, b0: i64, b1: i64) -> i64 {
        // a0 < a1, b0 < b1 のとき、{a0, a1, b0, b1} で2番目に小さい値
        if a0 == b0 {
            min(a1, b1)
        } else if a1 <= b0 {
            a1
        } else if b1 <= a0 {
            b1
        } else {
            max(a0, b0)
        }
    }

    pub struct RangeSumMonoid(Infallible);
    impl MonoidBeats for RangeSumMonoid {
        type S = Option<RangeSum>;

        fn identity() -> Self::S {
            Some(RangeSum {
                sum: 0,
                len: 0,
                min: i64::MAX,
                min_cnt: 0,
                min_2nd: i64::MAX,
            })
        }

        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            match (a, b) {
                (None, None) => None,
                (None, Some(_)) => None,
                (Some(_), None) => None,
                (Some(a), Some(b)) => Some(RangeSum {
                    sum: a.sum + b.sum,
                    len: a.len + b.len,
                    min: min(a.min, b.min),
                    min_cnt: a.min_cnt * (a.min <= b.min) as usize
                        + b.min_cnt * (b.min <= a.min) as usize,
                    min_2nd: second_smallest(a.min, a.min_2nd, b.min, b.min_2nd),
                }),
            }
        }

        fn fails(a: &Self::S) -> bool {
            a.is_none()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ChmaxFunc {
        pub chmax_val: i64,
    }

    impl ChmaxFunc {
        pub fn new(x: i64) -> Self {
            ChmaxFunc { chmax_val: x }
        }
    }

    pub struct RangeChmaxRangeSum(Infallible);

    impl MapMonoidBeats for RangeChmaxRangeSum {
        type F = ChmaxFunc;
        type M = RangeSumMonoid;

        fn identity_map() -> Self::F {
            ChmaxFunc {
                chmax_val: i64::MIN,
            }
        }

        #[allow(clippy::if_same_then_else)]
        fn mapping(f: &Self::F, x: &<Self::M as MonoidBeats>::S) -> <Self::M as MonoidBeats>::S {
            match x {
                None => None,
                Some(x) => {
                    if x.len == 0 {
                        Some(*x)
                    } else if f.chmax_val <= x.min {
                        Some(*x)
                    } else if f.chmax_val < x.min_2nd {
                        Some(RangeSum {
                            sum: x.sum + (f.chmax_val - x.min) * x.min_cnt as i64,
                            len: x.len,
                            min: f.chmax_val,
                            min_cnt: x.min_cnt,
                            min_2nd: x.min_2nd,
                        })
                    } else {
                        // 計算失敗
                        None
                    }
                }
            }
        }

        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            ChmaxFunc {
                chmax_val: max(f.chmax_val, g.chmax_val),
            }
        }
    }

    pub struct RangeChmaxRangeSumSegtree {
        segtree: SegtreeBeats<RangeChmaxRangeSum>,
        len: usize,
    }

    impl RangeChmaxRangeSumSegtree {
        pub fn new(n: usize) -> Self {
            let segtree = SegtreeBeats::<RangeChmaxRangeSum>::new(n);
            Self { segtree, len: n }
        }

        pub fn from(xs: &[i64]) -> Self {
            let len = xs.len();
            let segtree = SegtreeBeats::<RangeChmaxRangeSum>::from(
                xs.iter().copied().map(RangeSum::unit).collect_vec(),
            );
            Self { segtree, len }
        }

        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            self.len
        }

        pub fn set(&mut self, p: usize, x: i64) {
            self.segtree.set(p, RangeSum::unit(x));
        }

        pub fn get(&mut self, p: usize) -> i64 {
            self.segtree.get(p).unwrap().sum
        }

        pub fn sum<R: RangeBounds<usize>>(&mut self, range: R) -> i64 {
            self.segtree.prod(range).unwrap().sum
        }

        pub fn all_sum(&mut self) -> i64 {
            self.segtree.all_prod().unwrap().sum
        }

        pub fn chmax<R: RangeBounds<usize>>(&mut self, range: R, x: i64) {
            self.segtree.apply_range(range, ChmaxFunc::new(x));
        }

        pub fn to_vec(&mut self) -> Vec<i64> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}

#[cfg(test)]
mod test_range_chmax_range_sum {
    use itertools::Itertools;

    use super::range_chmax_range_sum::*;

    #[test]
    fn test_range_chmax_range_sum() {
        let xs = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut seg = RangeChmaxRangeSumSegtree::from(&xs);
        seg.chmax(3..8, 6); // [0,1,2,6,6,6,6,7,8,9]
        assert_eq!(seg.sum(2..5), 14); // [2,6,6]
        assert_eq!(seg.to_vec(), vec![0, 1, 2, 6, 6, 6, 6, 7, 8, 9]);
        assert_eq!(
            (0..xs.len()).map(|i| seg.get(i)).collect_vec(),
            vec![0, 1, 2, 6, 6, 6, 6, 7, 8, 9]
        )
    }

    #[ignore]
    #[test]
    fn test_random_range_chmax_range_sum() {
        use rand::{Rng, SeedableRng, rngs::SmallRng};

        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.random_range(1..=30);
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
            let mut segtree = RangeChmaxRangeSumSegtree::from(&naive_vec);

            for _ in 0..100 {
                let op_type = rng.random_range(0..5);

                match op_type {
                    0 => {
                        // set(p, x)
                        let p = rng.random_range(0..n);
                        let x = rng.random_range(-100..=100);
                        naive_vec[p] = x;
                        segtree.set(p, x);
                    }
                    1 => {
                        // chmax(range, x)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let x = rng.random_range(-100..=100);
                        for i in l..r {
                            naive_vec[i] = naive_vec[i].max(x);
                        }
                        segtree.chmax(l..r, x);
                    }
                    2 => {
                        // get(p)
                        let p = rng.random_range(0..n);
                        assert_eq!(segtree.get(p), naive_vec[p], "get({}) failed", p);
                    }
                    3 => {
                        // sum(range)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let expected_sum: i64 = naive_vec[l..r].iter().sum();
                        assert_eq!(segtree.sum(l..r), expected_sum, "sum({}..{}) failed", l, r);
                    }
                    4 => {
                        // all_sum()
                        let expected_sum: i64 = naive_vec.iter().sum();
                        assert_eq!(segtree.all_sum(), expected_sum, "all_sum() failed");
                    }
                    _ => unreachable!(),
                }
            }
            assert_eq!(segtree.to_vec(), naive_vec, "final to_vec() check failed");
        }
    }
}
