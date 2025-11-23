use cargo_snippet::snippet;

use super::abstract_segtree_beats::abstract_segtree_beats::{MapMonoidBeats, MonoidBeats};

#[snippet(
    prefix = "use range_chmax_range_sum::*;",
    include = "abstract_segtree_beats"
)]
pub mod range_chmax_range_sum {
    use super::{MapMonoidBeats, MonoidBeats};
    use std::{
        cmp::{max, min},
        convert::Infallible,
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
}

#[cfg(test)]
mod test_range_chmax_range_sum {
    use itertools::Itertools;
    use rand::Rng;
    use rand::SeedableRng;
    use rand::rngs::SmallRng;

    use super::super::abstract_segtree_beats::abstract_segtree_beats::*;
    use super::range_chmax_range_sum::*;

    #[test]
    fn test_range_chmax_range_sum() {
        let xs = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let f = ChmaxFunc::new(6);

        let mut seg = SegtreeBeats::<RangeChmaxRangeSum>::from(
            xs.iter().copied().map(RangeSum::unit).collect_vec(),
        );
        seg.apply_range(3..8, f); // [0,1,2,6,6,6,6,7,8,9]
        assert_eq!(seg.prod(2..5).unwrap().sum, 14); // [2,6,6]
        assert_eq!(
            seg.to_vec()
                .iter()
                .copied()
                .map(|x| x.unwrap().sum)
                .collect_vec(),
            vec![0, 1, 2, 6, 6, 6, 6, 7, 8, 9]
        );
        assert_eq!(
            (0..xs.len()).map(|i| seg.get(i).unwrap().sum).collect_vec(),
            vec![0, 1, 2, 6, 6, 6, 6, 7, 8, 9]
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
                let chmax_func = ChmaxFunc::new(chmax_val);

                let mut seg = SegtreeBeats::<RangeChmaxRangeSum>::from(
                    xs.iter().copied().map(RangeSum::unit).collect_vec(),
                );

                seg.apply_range(begin..end, chmax_func);

                for i in begin..end {
                    xs[i] = i64::max(xs[i], chmax_val);
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
