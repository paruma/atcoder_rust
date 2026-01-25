use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_affine_range_minmax::*;")]
pub mod range_affine_range_minmax {
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use std::{cmp::Ordering, convert::Infallible, ops::RangeBounds};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeMinMax {
        pub min: i64,
        pub max: i64,
        pub len: i64,
    }

    impl RangeMinMax {
        pub fn new(min: i64, max: i64, len: i64) -> Self {
            Self { min, max, len }
        }

        pub fn unit(x: i64) -> Self {
            Self {
                min: x,
                max: x,
                len: 1,
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Affine {
        pub slope: i64,
        pub intercept: i64,
    }

    impl Affine {
        /// 区間変更用（定数関数）
        pub fn constant_func(x: i64) -> Affine {
            Affine {
                slope: 0,
                intercept: x,
            }
        }

        /// 区間加算用
        pub fn addition_func(x: i64) -> Affine {
            Affine {
                slope: 1,
                intercept: x,
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeMinMaxMonoid(Infallible);
    impl Monoid for RangeMinMaxMonoid {
        type S = RangeMinMax;
        fn identity() -> RangeMinMax {
            RangeMinMax {
                // INF, -INF は len == 0のときだけ使う。
                min: i64::MAX,
                max: i64::MIN,
                len: 0,
            }
        }
        fn binary_operation(a: &RangeMinMax, b: &RangeMinMax) -> RangeMinMax {
            RangeMinMax {
                min: i64::min(a.min, b.min),
                max: i64::max(a.max, b.max),
                len: a.len + b.len,
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeAffineRangeMinMax(Infallible);
    impl MapMonoid for RangeAffineRangeMinMax {
        type M = RangeMinMaxMonoid;
        type F = Affine;

        fn identity_map() -> Affine {
            Affine {
                slope: 1,
                intercept: 0,
            }
        }
        fn composition(a: &Affine, b: &Affine) -> Affine {
            Affine {
                slope: a.slope * b.slope,
                intercept: a.slope * b.intercept + a.intercept,
            }
        }

        fn mapping(f: &Affine, x: &RangeMinMax) -> RangeMinMax {
            if x.len == 0 {
                return RangeMinMaxMonoid::identity();
            }

            match f.slope.cmp(&0) {
                Ordering::Equal => RangeMinMax {
                    min: f.intercept,
                    max: f.intercept,
                    len: x.len,
                },
                Ordering::Greater => RangeMinMax {
                    min: f.intercept + f.slope * x.min,
                    max: f.intercept + f.slope * x.max,
                    len: x.len,
                },
                Ordering::Less => RangeMinMax {
                    min: f.intercept + f.slope * x.max,
                    max: f.intercept + f.slope * x.min,
                    len: x.len,
                },
            }
        }
    }

    #[derive(Clone)]
    pub struct RangeAffineRangeMinMaxSegtree {
        segtree: LazySegtree<RangeAffineRangeMinMax>,
        len: usize,
    }

    impl RangeAffineRangeMinMaxSegtree {
        pub fn new(n: usize) -> Self {
            let xs = vec![0; n];
            Self::from_slice(&xs)
        }

        pub fn from_slice(xs: &[i64]) -> RangeAffineRangeMinMaxSegtree {
            let initial_data = xs.iter().copied().map(RangeMinMax::unit).collect_vec();
            let len = initial_data.len();
            RangeAffineRangeMinMaxSegtree {
                segtree: LazySegtree::from(initial_data),
                len,
            }
        }

        pub fn set(&mut self, p: usize, x: i64) {
            self.segtree.set(p, RangeMinMax::unit(x));
        }

        pub fn get(&mut self, p: usize) -> i64 {
            // min でも max でも同じ
            self.segtree.get(p).min
        }

        pub fn range_min<R>(&mut self, range: R) -> i64
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).min
        }

        pub fn range_max<R>(&mut self, range: R) -> i64
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).max
        }

        pub fn all_min(&self) -> i64 {
            self.segtree.all_prod().min
        }

        pub fn all_max(&self) -> i64 {
            self.segtree.all_prod().max
        }

        pub fn apply_affine(&mut self, p: usize, slope: i64, intercept: i64) {
            self.segtree.apply(p, Affine { slope, intercept })
        }

        pub fn apply_update(&mut self, p: usize, x: i64) {
            // set と同じはず
            self.segtree.apply(p, Affine::constant_func(x))
        }

        pub fn apply_add(&mut self, p: usize, x: i64) {
            self.segtree.apply(p, Affine::addition_func(x))
        }

        pub fn apply_range_affine<R>(&mut self, range: R, slope: i64, intercept: i64)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, Affine { slope, intercept })
        }

        pub fn apply_range_update<R>(&mut self, range: R, x: i64)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, Affine::constant_func(x))
        }

        pub fn apply_range_add<R>(&mut self, range: R, x: i64)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, Affine::addition_func(x))
        }

        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            self.len
        }
        pub fn to_vec(&mut self) -> Vec<i64> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}

#[cfg(test)]
mod test_range_affine_range_minmax {
    use super::range_affine_range_minmax::*;
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;

    type DataM = RangeMinMaxMonoid;
    type ActionM = RangeAffineRangeMinMax;

    #[test]
    fn test_minmax_len_monoid() {
        let x1 = RangeMinMax::unit(5);
        let x2 = RangeMinMax::unit(9);

        assert_eq!(
            DataM::binary_operation(&x1, &x2),
            RangeMinMax {
                min: 5,
                max: 9,
                len: 2,
            }
        );

        assert_eq!(DataM::binary_operation(&x1, &DataM::identity()), x1);
        assert_eq!(DataM::binary_operation(&DataM::identity(), &x1), x1);
    }

    #[test]
    fn test_affine_constant_func() {
        let f = Affine::constant_func(5);

        // 例えば [1, 2] に対する区間和とその長さ
        let x1 = RangeMinMax {
            min: 1,
            max: 2,
            len: 2,
        };
        let empty = RangeMinMaxMonoid::identity();

        assert_eq!(
            // [1, 2] を [5, 5] に変換したときの区間和とその長さ
            ActionM::mapping(&f, &x1),
            RangeMinMax {
                min: 5,
                max: 5,
                len: 2
            }
        );
        assert_eq!(
            // 空列の場合は空列のまま
            ActionM::mapping(&f, &empty),
            empty
        );
    }

    #[test]
    fn test_affine_addition_func() {
        let f = Affine::addition_func(5);

        // 例えば [1, 2] に対する区間和とその長さ
        let x1 = RangeMinMax {
            min: 1,
            max: 2,
            len: 2,
        };
        let empty = RangeMinMaxMonoid::identity();

        assert_eq!(
            // [1, 2] を [6, 7] に変換したときの区間和とその長さ
            ActionM::mapping(&f, &x1),
            RangeMinMax {
                min: 6,
                max: 7,
                len: 2
            }
        );
        assert_eq!(
            // 空列の場合は空列のまま
            ActionM::mapping(&f, &empty),
            empty
        );
    }

    #[test]
    fn test_affine_min_max_composition() {
        let f1 = Affine {
            slope: 3,
            intercept: 5,
        };

        let f2 = Affine {
            slope: 5,
            intercept: 2,
        };

        let f3 = Affine {
            slope: 0,
            intercept: 2,
        };

        // 3(5x + 2) + 5 = 15x + 11
        assert_eq!(
            ActionM::composition(&f1, &f2),
            Affine {
                slope: 15,
                intercept: 11
            }
        );

        // 3*(0x + 2) + 5 = 11
        assert_eq!(
            ActionM::composition(&f1, &f3),
            Affine {
                slope: 0,
                intercept: 11
            }
        );

        // 0(3x + 5) + 2 = 2
        assert_eq!(
            ActionM::composition(&f3, &f1),
            Affine {
                slope: 0,
                intercept: 2
            }
        );
    }
    #[test]
    fn test_affine_sum_mapping() {
        let x1 = [1, 2, 3]
            .iter()
            .copied()
            .map(RangeMinMax::unit)
            .fold(DataM::identity(), |acc, x| {
                DataM::binary_operation(&acc, &x)
            });

        let x2 = RangeMinMaxMonoid::identity();

        let f1 = Affine {
            slope: 3,
            intercept: 5,
        };

        let f2 = Affine {
            slope: 0,
            intercept: 10,
        };

        let f3 = Affine {
            slope: -3,
            intercept: -5,
        };

        assert_eq!(
            // [3x + 5 | x in [1, 2, 3]] = [8, 11, 14]
            ActionM::mapping(&f1, &x1),
            RangeMinMax {
                min: 8,
                max: 14,
                len: 3,
            }
        );

        assert_eq!(
            // [3x + 5 | x in []] = []
            ActionM::mapping(&f1, &x2),
            RangeMinMaxMonoid::identity()
        );

        assert_eq!(
            // [0x + 10 | x in [1, 2, 3]] = [10, 10, 10]
            ActionM::mapping(&f2, &x1),
            RangeMinMax {
                min: 10,
                max: 10,
                len: 3,
            }
        );

        assert_eq!(
            // [-3x - 5 | x in [1, 2, 3]] = [-8, -11, -14]
            ActionM::mapping(&f3, &x1),
            RangeMinMax {
                min: -14,
                max: -8,
                len: 3,
            }
        );
    }

    #[test]
    fn test_monoid_map_axiom() {
        let axiom_of_id = |x: RangeMinMax| {
            // id(x) = x
            assert_eq!(ActionM::mapping(&ActionM::identity_map(), &x), x);
        };

        let axiom_of_prod_act = |f1: Affine, f2: Affine, x: RangeMinMax| {
            // (f1*f2)(x) = f1(f2(x))
            assert_eq!(
                ActionM::mapping(&ActionM::composition(&f1, &f2), &x),
                ActionM::mapping(&f1, &ActionM::mapping(&f2, &x))
            );
        };

        let axiom_of_keeping_unit = |f: Affine| {
            // f(e) = e
            assert_eq!(ActionM::mapping(&f, &DataM::identity()), DataM::identity());
        };

        let axiom_of_keeping_prod = |f: Affine, x1: RangeMinMax, x2: RangeMinMax| {
            // f(x1*x2) = f(x1)*f(x2)
            assert_eq!(
                ActionM::mapping(&f, &DataM::binary_operation(&x1, &x2)),
                DataM::binary_operation(&ActionM::mapping(&f, &x1), &ActionM::mapping(&f, &x2))
            );
        };

        let x1 = RangeMinMax {
            min: 3,
            max: 6,
            len: 2,
        };
        let x2 = RangeMinMax {
            min: -3,
            max: 9,
            len: 3,
        };

        let empty = RangeMinMaxMonoid::identity();

        let f1 = Affine {
            slope: 3,
            intercept: 5,
        };

        let f2 = Affine {
            slope: (-5),
            intercept: 2,
        };

        let id = Affine {
            slope: 0,
            intercept: 2,
        };

        axiom_of_id(x1);
        axiom_of_id(empty);

        axiom_of_prod_act(f1, f2, x1);
        axiom_of_prod_act(f1, id, x1);
        axiom_of_prod_act(id, f2, x1);
        axiom_of_prod_act(id, id, x1);

        axiom_of_keeping_unit(f1);
        axiom_of_keeping_unit(f2);
        axiom_of_keeping_unit(id);

        axiom_of_keeping_prod(f1, x1, x2);
        axiom_of_keeping_prod(f1, empty, x2);
        axiom_of_keeping_prod(f1, x1, empty);
        axiom_of_keeping_prod(f1, empty, empty);
        axiom_of_keeping_prod(f2, x1, x2);
        axiom_of_keeping_prod(f2, empty, x2);
        axiom_of_keeping_prod(f2, x1, empty);
        axiom_of_keeping_prod(f2, empty, empty);
        axiom_of_keeping_prod(id, x1, x2);
        axiom_of_keeping_prod(id, empty, x2);
        axiom_of_keeping_prod(id, x1, empty);
        axiom_of_keeping_prod(id, empty, empty);
    }

    #[test]
    fn test_range_affine_range_minmax_segtree() {
        let xs = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let mut segtree = RangeAffineRangeMinMaxSegtree::from_slice(&xs);

        segtree.apply_range_update(3..6, 5); // [0, 1, 2, 5, 5, 5, 6, 7, 8, 9]
        segtree.apply_range_add(0..2, 3); // [3, 4, 2, 5, 5, 5, 6, 7, 8, 9]
        segtree.apply_range_affine(8..10, 2, 7); // [3, 4, 2, 5, 5, 5, 6, 7, 23, 25]

        assert_eq!(segtree.all_min(), 2);
        assert_eq!(segtree.all_max(), 25);

        assert_eq!(segtree.range_max(1..4), 5); // [4, 2, 5]
        assert_eq!(segtree.range_min(1..4), 2); // [4, 2, 5]
        assert_eq!(segtree.to_vec(), vec![3, 4, 2, 5, 5, 5, 6, 7, 23, 25]);
    }

    #[test]
    fn test_sample_of_lazy_segtree() {
        // range affine range min/max の遅延セグ木の使用例
        let xs = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
            .map(RangeMinMax::unit)
            .to_vec();

        let mut segtree: LazySegtree<RangeAffineRangeMinMax> = LazySegtree::from(xs);

        let f1 = Affine::constant_func(5);
        let f2 = Affine::addition_func(3);
        let f3 = Affine {
            slope: 2,
            intercept: 7,
        };

        segtree.apply_range(3..6, f1); // [0, 1, 2, 5, 5, 5, 6, 7, 8, 9]
        segtree.apply_range(0..2, f2); // [3, 4, 2, 5, 5, 5, 6, 7, 8, 9]
        segtree.apply_range(8..10, f3); // [3, 4, 2, 5, 5, 5, 6, 7, 23, 25]

        let all_prod = segtree.all_prod();
        assert_eq!(all_prod.min, 2);
        assert_eq!(all_prod.max, 25);

        let segtree_as_vec = (0..10).map(|i| segtree.get(i).max).collect_vec();
        assert_eq!(segtree_as_vec, vec![3, 4, 2, 5, 5, 5, 6, 7, 23, 25]);
    }

    #[ignore]
    #[test]
    fn test_random_affine_minmax() {
        use rand::{Rng, SeedableRng, rngs::SmallRng};

        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.random_range(1..=20);
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
            let mut segtree = RangeAffineRangeMinMaxSegtree::from_slice(&naive_vec);

            for _ in 0..100 {
                // 100 random operations per set
                let op_type = rng.random_range(0..10); // More operations

                match op_type {
                    0 => {
                        // set(p, x)
                        let p = rng.random_range(0..n);
                        let x = rng.random_range(-100..=100);
                        naive_vec[p] = x;
                        segtree.set(p, x);
                    }
                    1 => {
                        // apply_affine(p, slope, intercept)
                        let p = rng.random_range(0..n);
                        let slope = rng.random_range(-2..=2); // Keep slope small
                        let intercept = rng.random_range(-50..=50);
                        naive_vec[p] = naive_vec[p] * slope + intercept;
                        segtree.apply_affine(p, slope, intercept);
                    }
                    2 => {
                        // apply_update(p, x)
                        let p = rng.random_range(0..n);
                        let x = rng.random_range(-100..=100);
                        naive_vec[p] = x;
                        segtree.apply_update(p, x);
                    }
                    3 => {
                        // apply_add(p, x)
                        let p = rng.random_range(0..n);
                        let x = rng.random_range(-50..=50);
                        naive_vec[p] += x;
                        segtree.apply_add(p, x);
                    }
                    4 => {
                        // apply_range_affine(range, slope, intercept)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);

                        let slope = rng.random_range(-2..=2); // Keep slope small
                        let intercept = rng.random_range(-50..=50);

                        for i in l..r {
                            naive_vec[i] = naive_vec[i] * slope + intercept;
                        }
                        segtree.apply_range_affine(l..r, slope, intercept);
                    }
                    5 => {
                        // apply_range_update(range, x)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);

                        let x = rng.random_range(-100..=100);

                        for i in l..r {
                            naive_vec[i] = x;
                        }
                        segtree.apply_range_update(l..r, x);
                    }
                    6 => {
                        // apply_range_add(range, x)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);

                        let x = rng.random_range(-50..=50);

                        for i in l..r {
                            naive_vec[i] += x;
                        }
                        segtree.apply_range_add(l..r, x);
                    }
                    7 => {
                        // get(p)
                        let p = rng.random_range(0..n);
                        assert_eq!(segtree.get(p), naive_vec[p], "get({}) failed", p);
                    }
                    8 => {
                        // range_min(range)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);

                        let expected_min =
                            naive_vec[l..r].iter().copied().min().unwrap_or(i64::MAX);
                        assert_eq!(
                            segtree.range_min(l..r),
                            expected_min,
                            "range_min({}..{}) failed",
                            l,
                            r
                        );
                    }
                    9 => {
                        // range_max(range)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);

                        let expected_max =
                            naive_vec[l..r].iter().copied().max().unwrap_or(i64::MIN);
                        assert_eq!(
                            segtree.range_max(l..r),
                            expected_max,
                            "range_max({}..{}) failed",
                            l,
                            r
                        );
                    }
                    _ => unreachable!(),
                }
            }

            // Final check
            let final_expected_min = naive_vec.iter().copied().min().unwrap_or(i64::MAX);
            let final_expected_max = naive_vec.iter().copied().max().unwrap_or(i64::MIN);
            assert_eq!(
                segtree.all_min(),
                final_expected_min,
                "final all_min() check failed"
            );
            assert_eq!(
                segtree.all_max(),
                final_expected_max,
                "final all_max() check failed"
            );
            assert_eq!(segtree.to_vec(), naive_vec, "final to_vec() check failed");
        }
    }
}
