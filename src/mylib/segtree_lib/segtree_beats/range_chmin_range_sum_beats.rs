use cargo_snippet::snippet;

use super::abstract_segtree_beats::abstract_segtree_beats::{MapMonoidBeats, MonoidBeats};
#[snippet(
    prefix = "use range_chmin_range_sum::*;",
    include = "abstract_segtree_beats"
)]
pub mod range_chmin_range_sum {
    use std::{
        cmp::{max, min},
        convert::Infallible,
    };

    use super::{MapMonoidBeats, MonoidBeats};

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
}

#[cfg(test)]
mod test_range_chmin_range_sum {
    use itertools::Itertools;
    use rand::Rng;
    use rand::SeedableRng;
    use rand::rngs::SmallRng;

    use super::super::abstract_segtree_beats::abstract_segtree_beats::*;
    use super::range_chmin_range_sum::*;

    #[test]
    fn test_range_chmax_range_sum() {
        let xs = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let f = ChminFunc::new(4);

        let mut seg = SegtreeBeats::<RangeChminRangeSum>::from(
            xs.iter().copied().map(RangeSum::unit).collect_vec(),
        );
        seg.apply_range(3..8, f); // [0,1,2,3,4,4,4,4,8,9]
        assert_eq!(seg.prod(3..6).unwrap().sum, 11); // [3,4,4]
        assert_eq!(
            (0..xs.len()).map(|i| seg.get(i).unwrap().sum).collect_vec(),
            vec![0, 1, 2, 3, 4, 4, 4, 4, 8, 9]
        )
    }

    #[test]
    fn test_range_chmax_range_sum_random() {
        let mut rng = SmallRng::seed_from_u64(42);

        // テスト回数を大きくしすぎると実行時間がかかるため控えめに
        let n_tests = 20;
        let n_changes = 3;

        for _ in 0..n_tests {
            let n = rng.random_range(0..15);
            let mut xs = (0..n).map(|_| rng.random_range(0..10)).collect_vec();

            for _ in 0..n_changes {
                let begin = rng.random_range(0..=n);
                let end = rng.random_range(begin..=n);

                let chmax_val = rng.random_range(0..10);
                let chmax_func = ChminFunc::new(chmax_val);

                let mut seg = SegtreeBeats::<RangeChminRangeSum>::from(
                    xs.iter().copied().map(RangeSum::unit).collect_vec(),
                );

                seg.apply_range(begin..end, chmax_func);

                for i in begin..end {
                    xs[i] = i64::min(xs[i], chmax_val);
                }

                assert_eq!(
                    seg.prod(begin..end).unwrap().sum,
                    xs[begin..end].iter().sum::<i64>()
                );
                assert_eq!((0..n).map(|i| seg.get(i).unwrap().sum).collect_vec(), xs)
            }
        }
    }
}
