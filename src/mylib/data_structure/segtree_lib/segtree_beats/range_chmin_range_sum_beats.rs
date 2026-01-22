use cargo_snippet::snippet;

use super::abstract_segtree_beats::abstract_segtree_beats::{
    MapMonoidBeats, MonoidBeats, SegtreeBeats,
};

#[snippet(
    prefix = "use range_chmin_range_sum::*;",
    include = "abstract_segtree_beats"
)]
pub mod range_chmin_range_sum {
    use super::{MapMonoidBeats, MonoidBeats, SegtreeBeats};
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
        pub max: i64,
        pub max_cnt: usize,
        pub max_2nd: i64,
    }

    impl RangeSum {
        pub fn unit(x: i64) -> Option<RangeSum> {
            Some(RangeSum {
                sum: x,
                len: 1,
                max: x,
                max_cnt: 1,
                max_2nd: i64::MIN,
            })
        }
    }

    fn second_largest(a0: i64, a1: i64, b0: i64, b1: i64) -> i64 {
        // a0 > a1, b0 > b1 のとき、{a0, a1, b0, b1} で2番目に大きい値
        if a0 == b0 {
            max(a1, b1)
        } else if a1 >= b0 {
            a1
        } else if b1 >= a0 {
            b1
        } else {
            min(a0, b0)
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeSumMonoid(Infallible);
    impl MonoidBeats for RangeSumMonoid {
        type S = Option<RangeSum>;

        fn identity() -> Self::S {
            Some(RangeSum {
                sum: 0,
                len: 0,
                max: i64::MIN,
                max_cnt: 0,
                max_2nd: i64::MIN,
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
                    max: max(a.max, b.max),
                    max_cnt: a.max_cnt * (a.max >= b.max) as usize
                        + b.max_cnt * (b.max >= a.max) as usize,
                    max_2nd: second_largest(a.max, a.max_2nd, b.max, b.max_2nd),
                }),
            }
        }

        fn fails(a: &Self::S) -> bool {
            a.is_none()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ChminFunc {
        pub chmin_val: i64,
    }

    impl ChminFunc {
        pub fn new(x: i64) -> Self {
            ChminFunc { chmin_val: x }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeChminRangeSum(Infallible);

    impl MapMonoidBeats for RangeChminRangeSum {
        type F = ChminFunc;
        type M = RangeSumMonoid;

        fn identity_map() -> Self::F {
            ChminFunc {
                chmin_val: i64::MAX,
            }
        }

        #[allow(clippy::if_same_then_else)]
        fn mapping(f: &Self::F, x: &<Self::M as MonoidBeats>::S) -> <Self::M as MonoidBeats>::S {
            match x {
                None => None,
                Some(x) => {
                    if x.len == 0 {
                        Some(*x)
                    } else if f.chmin_val >= x.max {
                        Some(*x)
                    } else if f.chmin_val > x.max_2nd {
                        Some(RangeSum {
                            sum: x.sum + (f.chmin_val - x.max) * x.max_cnt as i64,
                            len: x.len,
                            max: f.chmin_val,
                            max_cnt: x.max_cnt,
                            max_2nd: x.max_2nd,
                        })
                    } else {
                        // 計算失敗
                        None
                    }
                }
            }
        }

        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            ChminFunc {
                chmin_val: min(f.chmin_val, g.chmin_val),
            }
        }
    }

    #[derive(Clone)]
    pub struct RangeChminRangeSumSegtree {
        segtree: SegtreeBeats<RangeChminRangeSum>,
        len: usize,
    }

    impl RangeChminRangeSumSegtree {
        pub fn new(n: usize) -> Self {
            let segtree = SegtreeBeats::<RangeChminRangeSum>::new(n);
            Self { segtree, len: n }
        }

        pub fn from(xs: &[i64]) -> Self {
            let len = xs.len();
            let segtree = SegtreeBeats::<RangeChminRangeSum>::from(
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

        pub fn chmin<R: RangeBounds<usize>>(&mut self, range: R, x: i64) {
            self.segtree.apply_range(range, ChminFunc::new(x));
        }

        pub fn to_vec(&mut self) -> Vec<i64> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}

#[cfg(test)]
mod test_range_chmin_range_sum {
    use itertools::Itertools;

    use super::range_chmin_range_sum::*;

    #[test]
    fn test_range_chmin_range_sum() {
        let xs = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut seg = RangeChminRangeSumSegtree::from(&xs);
        seg.chmin(3..8, 4); // [0,1,2,3,4,4,4,4,8,9]
        assert_eq!(seg.sum(3..6), 11); // [3,4,4]
        assert_eq!(seg.to_vec(), vec![0, 1, 2, 3, 4, 4, 4, 4, 8, 9]);
        assert_eq!(
            (0..xs.len()).map(|i| seg.get(i)).collect_vec(),
            vec![0, 1, 2, 3, 4, 4, 4, 4, 8, 9]
        )
    }

    #[ignore]
    #[test]
    fn test_random_range_chmin_range_sum() {
        use rand::{Rng, SeedableRng, rngs::SmallRng};

        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.random_range(1..=30);
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
            let mut segtree = RangeChminRangeSumSegtree::from(&naive_vec);

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
                        // chmin(range, x)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let x = rng.random_range(-100..=100);
                        for i in l..r {
                            naive_vec[i] = naive_vec[i].min(x);
                        }
                        segtree.chmin(l..r, x);
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
