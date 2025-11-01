use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_affine_range_sum_of_square::*;")]
pub mod range_affine_range_sum_of_square {
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::marker::PhantomData;
    use std::ops::{Add, Mul, RangeBounds};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeSum<T> {
        pub sum_sq: T,
        pub sum: T,
        pub len: i64,
    }
    impl<T> RangeSum<T>
    where
        T: Copy + Mul<Output = T>,
    {
        pub fn unit(x: T) -> RangeSum<T> {
            RangeSum {
                sum_sq: x * x,
                sum: x,
                len: 1,
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Affine<T> {
        pub slope: T,
        pub intercept: T,
    }

    impl<T> Affine<T>
    where
        T: From<i64>,
    {
        /// 区間変更用（定数関数）
        pub fn constant_func(x: T) -> Affine<T> {
            Affine {
                slope: 0.into(),
                intercept: x,
            }
        }

        /// 区間加算用
        pub fn addition_func(x: T) -> Affine<T> {
            Affine {
                slope: 1.into(),
                intercept: x,
            }
        }
    }

    pub struct ValueLenSum<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for ValueLenSum<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        type S = RangeSum<T>;
        fn identity() -> RangeSum<T> {
            RangeSum {
                sum_sq: 0.into(),
                sum: 0.into(),
                len: 0,
            }
        }
        fn binary_operation(a: &RangeSum<T>, b: &RangeSum<T>) -> RangeSum<T> {
            RangeSum {
                sum_sq: a.sum_sq + b.sum_sq,
                sum: a.sum + b.sum,
                len: a.len + b.len,
            }
        }
    }

    pub struct RangeAffineRangeSumOfSquare<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for RangeAffineRangeSumOfSquare<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        type M = ValueLenSum<T>;
        type F = Affine<T>;

        fn identity_map() -> Affine<T> {
            Affine {
                slope: 1.into(),
                intercept: 0.into(),
            }
        }
        fn composition(a: &Affine<T>, b: &Affine<T>) -> Affine<T> {
            Affine {
                slope: a.slope * b.slope,
                intercept: a.slope * b.intercept + a.intercept,
            }
        }

        fn mapping(f: &Affine<T>, x: &RangeSum<T>) -> RangeSum<T> {
            let a = f.slope;
            let b = f.intercept;
            let len_t: T = x.len.into();

            let new_sum_sq = a * a * x.sum_sq + (a + a) * b * x.sum + b * b * len_t;
            let new_sum = a * x.sum + b * len_t;

            RangeSum {
                sum_sq: new_sum_sq,
                sum: new_sum,
                len: x.len,
            }
        }
    }

    pub struct RangeAffineRangeSumOfSquareSegtree<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        segtree: LazySegtree<RangeAffineRangeSumOfSquare<T>>,
        len: usize,
    }

    impl<T> RangeAffineRangeSumOfSquareSegtree<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        pub fn new(xs: &[T]) -> RangeAffineRangeSumOfSquareSegtree<T> {
            let xs = xs.iter().copied().map(RangeSum::unit).collect_vec();
            let len = xs.len();
            RangeAffineRangeSumOfSquareSegtree {
                segtree: LazySegtree::from(xs),
                len,
            }
        }

        pub fn set(&mut self, p: usize, x: T) {
            self.segtree.set(p, RangeSum::unit(x));
        }

        pub fn get(&mut self, p: usize) -> T {
            self.segtree.get(p).sum
        }

        pub fn range_sum_of_square<R>(&mut self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).sum_sq
        }

        pub fn all_sum_of_square(&self) -> T {
            self.segtree.all_prod().sum_sq
        }

        pub fn range_sum<R>(&mut self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).sum
        }

        pub fn all_sum(&self) -> T {
            self.segtree.all_prod().sum
        }

        pub fn apply_affine(&mut self, p: usize, slope: T, intercept: T) {
            self.segtree.apply(p, Affine { slope, intercept })
        }

        pub fn apply_update(&mut self, p: usize, x: T) {
            self.segtree.apply(p, Affine::constant_func(x))
        }

        pub fn apply_add(&mut self, p: usize, x: T) {
            self.segtree.apply(p, Affine::addition_func(x))
        }

        pub fn apply_range_affine<R>(&mut self, range: R, slope: T, intercept: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, Affine { slope, intercept })
        }

        pub fn apply_range_update<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, Affine::constant_func(x))
        }

        pub fn apply_range_add<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, Affine::addition_func(x))
        }

        pub fn to_vec(&mut self) -> Vec<T> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}

#[cfg(test)]
mod test_range_affine_range_sum_of_square {
    use ac_library::{MapMonoid, ModInt998244353, Monoid};

    use super::range_affine_range_sum_of_square::*;

    type Mint = ModInt998244353;
    type DataM = ValueLenSum<Mint>;
    type ActionM = RangeAffineRangeSumOfSquare<Mint>;

    #[test]
    fn test_value_len_sum() {
        let x1 = RangeSum::unit(5.into()); // sum_sq=25, sum=5, len=1
        let x2 = RangeSum::unit(9.into()); // sum_sq=81, sum=9, len=1

        assert_eq!(
            DataM::binary_operation(&x1, &x2),
            RangeSum {
                sum_sq: 106.into(), // 25 + 81
                sum: 14.into(),
                len: 2,
            }
        );

        assert_eq!(DataM::binary_operation(&x1, &DataM::identity()), x1);
        assert_eq!(DataM::binary_operation(&DataM::identity(), &x1), x1);
    }

    #[test]
    fn test_affine_constant_func() {
        let f = Affine::constant_func(5.into()); // ax+b where a=0, b=5

        // [1, 2] -> sum_sq=1+4=5, sum=3, len=2
        let x1 = RangeSum {
            sum_sq: 5.into(),
            sum: 3.into(),
            len: 2,
        };
        let empty = RangeSum {
            sum_sq: 0.into(),
            sum: 0.into(),
            len: 0,
        };

        // [1, 2] becomes [5, 5].
        // new sum_sq = 5^2 + 5^2 = 50
        // new sum = 5 + 5 = 10
        assert_eq!(
            ActionM::mapping(&f, &x1),
            RangeSum {
                sum_sq: 50.into(),
                sum: 10.into(),
                len: 2
            }
        );
        assert_eq!(ActionM::mapping(&f, &empty), empty);
    }

    #[test]
    fn test_affine_addition_func() {
        let f = Affine::addition_func(5.into()); // ax+b where a=1, b=5

        // [1, 2] -> sum_sq=5, sum=3, len=2
        let x1 = RangeSum {
            sum_sq: 5.into(),
            sum: 3.into(),
            len: 2,
        };
        let empty = RangeSum {
            sum_sq: 0.into(),
            sum: 0.into(),
            len: 0,
        };

        // [1, 2] becomes [6, 7]
        // new sum_sq = 6^2 + 7^2 = 36 + 49 = 85
        // new sum = 6 + 7 = 13
        assert_eq!(
            ActionM::mapping(&f, &x1),
            RangeSum {
                sum_sq: 85.into(),
                sum: 13.into(),
                len: 2
            }
        );
        assert_eq!(ActionM::mapping(&f, &empty), empty);
    }

    #[test]
    fn test_affine_sum_composition() {
        // This is the same as before.
        let f1 = Affine {
            slope: 3.into(),
            intercept: 5.into(),
        };

        let f2 = Affine {
            slope: 5.into(),
            intercept: 2.into(),
        };

        let f3 = Affine {
            slope: 0.into(),
            intercept: 2.into(),
        };

        // 3(5x + 2) + 5 = 15x + 11
        assert_eq!(
            ActionM::composition(&f1, &f2),
            Affine {
                slope: 15.into(),
                intercept: 11.into()
            }
        );

        // 3*(0x + 2) + 5 = 11
        assert_eq!(
            ActionM::composition(&f1, &f3),
            Affine {
                slope: 0.into(),
                intercept: 11.into()
            }
        );

        // 0(3x + 5) + 2 = 2
        assert_eq!(
            ActionM::composition(&f3, &f1),
            Affine {
                slope: 0.into(),
                intercept: 2.into()
            }
        );
    }

    #[test]
    fn test_affine_sum_mapping() {
        // [1, 2, 3]
        let x1 = DataM::binary_operation(
            &DataM::binary_operation(&RangeSum::unit(1.into()), &RangeSum::unit(2.into())),
            &RangeSum::unit(3.into()),
        );
        // sum_sq = 1+4+9=14, sum=6, len=3

        let x2 = RangeSum {
            sum_sq: 0.into(),
            sum: 0.into(),
            len: 0,
        };

        let f1 = Affine {
            slope: 3.into(),
            intercept: 5.into(),
        };

        let f2 = Affine {
            slope: 0.into(),
            intercept: 10.into(),
        };

        // apply f1 to [1, 2, 3] -> [8, 11, 14]
        // new sum_sq = 64 + 121 + 196 = 381
        // new sum = 8 + 11 + 14 = 33
        assert_eq!(
            ActionM::mapping(&f1, &x1),
            RangeSum {
                sum_sq: 381.into(),
                sum: 33.into(),
                len: 3,
            }
        );

        assert_eq!(
            ActionM::mapping(&f1, &x2),
            RangeSum {
                sum_sq: 0.into(),
                sum: 0.into(),
                len: 0,
            }
        );

        // apply f2 to [1, 2, 3] -> [10, 10, 10]
        // new sum_sq = 100 + 100 + 100 = 300
        // new sum = 10 + 10 + 10 = 30
        assert_eq!(
            ActionM::mapping(&f2, &x1),
            RangeSum {
                sum_sq: 300.into(),
                sum: 30.into(),
                len: 3,
            }
        );
    }

    #[test]
    fn test_range_sum_range_affine_segtree() {
        let xs: Vec<i64> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let mut segtree = RangeAffineRangeSumOfSquareSegtree::<i64>::new(&xs);

        segtree.apply_range_update(3..6, 5); // [0, 1, 2, 5, 5, 5, 6, 7, 8, 9]
        segtree.apply_range_add(0..2, 3); // [3, 4, 2, 5, 5, 5, 6, 7, 8, 9]
        segtree.apply_range_affine(8..10, 2, 7); // [3, 4, 2, 5, 5, 5, 6, 7, 23, 25]
        // vec is [3, 4, 2, 5, 5, 5, 6, 7, 23, 25]
        // sum is 85
        // sum_sq is 9+16+4+25+25+25+36+49+529+625 = 1343

        assert_eq!(segtree.all_sum_of_square(), 1343);
        assert_eq!(segtree.all_sum(), 85);
        assert_eq!(segtree.range_sum_of_square(1..4), 16 + 4 + 25); // [4, 2, 5] -> 16+4+25=45
        assert_eq!(segtree.range_sum(1..4), 4 + 2 + 5); // 11
        assert_eq!(segtree.to_vec(), vec![3, 4, 2, 5, 5, 5, 6, 7, 23, 25]);
    }

    #[ignore]
    #[test]
    fn test_random_affine_sum_of_square() {
        use rand::{Rng, SeedableRng, rngs::SmallRng};

        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.random_range(1..=20);
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
            let mut segtree = RangeAffineRangeSumOfSquareSegtree::<i64>::new(&naive_vec);

            for _ in 0..100 {
                // 100 random operations per set
                let op_type = rng.random_range(0..11); // 11 operations

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
                        // range_sum(range)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);

                        let expected_sum: i64 = naive_vec[l..r].iter().sum();
                        assert_eq!(
                            segtree.range_sum(l..r),
                            expected_sum,
                            "range_sum({}..{}) failed",
                            l,
                            r
                        );
                    }
                    9 => {
                        // all_sum()
                        let expected_sum: i64 = naive_vec.iter().sum();
                        assert_eq!(segtree.all_sum(), expected_sum, "all_sum() failed");
                    }
                    10 => {
                        // range_sum_of_square(range)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);

                        let expected_sum_sq: i64 = naive_vec[l..r].iter().map(|x| x * x).sum();
                        assert_eq!(
                            segtree.range_sum_of_square(l..r),
                            expected_sum_sq,
                            "range_sum_of_square({}..{}) failed",
                            l,
                            r
                        );
                    }
                    _ => unreachable!(),
                }
            }

            // Final check
            assert_eq!(segtree.to_vec(), naive_vec, "final to_vec() check failed");
        }
    }
}
