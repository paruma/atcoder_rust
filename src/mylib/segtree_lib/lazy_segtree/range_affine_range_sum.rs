use cargo_snippet::snippet;

#[snippet(prefix = "use range_affine_range_sum::*;")]
pub mod range_affine_range_sum {
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::marker::PhantomData;
    use std::ops::{Add, Mul, RangeBounds};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeSum<T> {
        pub sum: T,
        pub len: i64,
    }
    impl<T> RangeSum<T> {
        pub fn unit(x: T) -> RangeSum<T> {
            RangeSum { sum: x, len: 1 }
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
                sum: 0.into(),
                len: 0,
            }
        }
        fn binary_operation(a: &RangeSum<T>, b: &RangeSum<T>) -> RangeSum<T> {
            RangeSum {
                sum: a.sum + b.sum,
                len: a.len + b.len,
            }
        }
    }

    pub struct RangeAffineRangeSum<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for RangeAffineRangeSum<T>
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
            RangeSum {
                sum: f.slope * x.sum + f.intercept * x.len.into(),
                len: x.len,
            }
        }
    }

    pub struct RangeAffineRangeSumSegtree<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        segtree: LazySegtree<RangeAffineRangeSum<T>>,
        len: usize,
    }

    impl<T> RangeAffineRangeSumSegtree<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        pub fn new(xs: &[T]) -> RangeAffineRangeSumSegtree<T> {
            let xs = xs.iter().copied().map(RangeSum::unit).collect_vec();
            let len = xs.len();
            RangeAffineRangeSumSegtree {
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
            // set と同じはず
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
mod test_range_affine_range_sum {
    use ac_library::{LazySegtree, MapMonoid, ModInt998244353, Monoid};

    use super::range_affine_range_sum::*;

    type Mint = ModInt998244353;
    type DataM = ValueLenSum<Mint>;
    type ActionM = RangeAffineRangeSum<Mint>;

    #[test]
    fn test_value_len_sum() {
        let x1 = RangeSum::unit(5.into());
        let x2 = RangeSum::unit(9.into());

        assert_eq!(
            DataM::binary_operation(&x1, &x2),
            RangeSum {
                sum: 14.into(),
                len: 2,
            }
        );

        assert_eq!(DataM::binary_operation(&x1, &DataM::identity()), x1);
        assert_eq!(DataM::binary_operation(&DataM::identity(), &x1), x1);
    }

    #[test]
    fn test_affine_constant_func() {
        let f = Affine::constant_func(5.into());

        // 例えば [1, 2] に対する区間和とその長さ
        let x1 = RangeSum {
            sum: 3.into(),
            len: 2,
        };
        let empty = RangeSum {
            sum: 0.into(),
            len: 0,
        };

        assert_eq!(
            // [1, 2] を [5, 5] に変換したときの区間和とその長さ
            ActionM::mapping(&f, &x1),
            RangeSum {
                sum: 10.into(),
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
        let f = Affine::addition_func(5.into());

        // 例えば [1, 2] に対する区間和とその長さ
        let x1 = RangeSum {
            sum: 3.into(),
            len: 2,
        };
        let empty = RangeSum {
            sum: 0.into(),
            len: 0,
        };

        assert_eq!(
            // [1, 2] を [6, 7] に変換したときの区間和とその長さ
            ActionM::mapping(&f, &x1),
            RangeSum {
                sum: 13.into(),
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
    fn test_affine_sum_composition() {
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
        let x1 = [1, 2, 3]
            .iter()
            .copied()
            .map(|x| RangeSum::unit(Mint::new(x)))
            .fold(DataM::identity(), |acc, x| {
                DataM::binary_operation(&acc, &x)
            });

        let x2 = RangeSum {
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

        assert_eq!(
            // sum {3x + 5 | x in [1, 2, 3]}
            ActionM::mapping(&f1, &x1),
            RangeSum {
                sum: 33.into(), // 8 + 11 + 14 = 33
                len: 3,
            }
        );

        assert_eq!(
            // sum {3x + 5 | x in []}
            ActionM::mapping(&f1, &x2),
            RangeSum {
                sum: 0.into(),
                len: 0,
            }
        );

        assert_eq!(
            // sum {0x + 10 | x in [1, 2, 3]}
            ActionM::mapping(&f2, &x1),
            RangeSum {
                sum: 30.into(), // 10 + 10 + 10
                len: 3,
            }
        );
    }

    #[test]
    fn test_monoid_map_axiom() {
        let axiom_of_id = |x: RangeSum<Mint>| {
            // id(x) = x
            assert_eq!(ActionM::mapping(&ActionM::identity_map(), &x), x);
        };

        let axiom_of_prod_act = |f1: Affine<Mint>, f2: Affine<Mint>, x: RangeSum<Mint>| {
            // (f1*f2)(x) = f1(f2(x))
            assert_eq!(
                ActionM::mapping(&ActionM::composition(&f1, &f2), &x),
                ActionM::mapping(&f1, &ActionM::mapping(&f2, &x))
            );
        };

        let axiom_of_keeping_unit = |f: Affine<Mint>| {
            // これは必須ではない？
            // f(e) = e
            assert_eq!(
                ActionM::mapping(&f, &ActionM::mapping(&f, &DataM::identity())),
                DataM::identity()
            );
        };

        let axiom_of_keeping_prod = |f: Affine<Mint>, x1: RangeSum<Mint>, x2: RangeSum<Mint>| {
            // f(x1*x2) = f(x1)*f(x2)
            assert_eq!(
                ActionM::mapping(&f, &DataM::binary_operation(&x1, &x2)),
                DataM::binary_operation(&ActionM::mapping(&f, &x1), &ActionM::mapping(&f, &x2))
            );
        };

        let x1 = RangeSum {
            sum: 6.into(),
            len: 2,
        };
        let x2 = RangeSum {
            sum: 10.into(),
            len: 3,
        };

        let empty = RangeSum {
            sum: 0.into(),
            len: 0,
        };

        let f1 = Affine {
            slope: 3.into(),
            intercept: 5.into(),
        };

        let f2 = Affine {
            slope: (-5).into(),
            intercept: 2.into(),
        };

        let id = Affine {
            slope: 0.into(),
            intercept: 2.into(),
        };

        axiom_of_id(x1);
        axiom_of_id(empty);

        axiom_of_prod_act(f1, f2, x1);
        axiom_of_prod_act(f1, id, x1);
        axiom_of_prod_act(id, f2, x1);
        axiom_of_prod_act(id, id, x1);

        axiom_of_keeping_unit(f1);
        axiom_of_keeping_unit(id);

        axiom_of_keeping_prod(f1, x1, x2);
        axiom_of_keeping_prod(f1, empty, x2);
        axiom_of_keeping_prod(f1, x1, empty);
        axiom_of_keeping_prod(f1, empty, empty);
        axiom_of_keeping_prod(id, x1, x2);
        axiom_of_keeping_prod(id, empty, x2);
        axiom_of_keeping_prod(id, x1, empty);
        axiom_of_keeping_prod(id, empty, empty);
    }

    #[test]
    fn test_range_sum_range_affine_segtree() {
        let xs = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let mut segtree: RangeAffineRangeSumSegtree<i64> =
            RangeAffineRangeSumSegtree::<i64>::new(&xs);

        segtree.apply_range_update(3..6, 5); // [0, 1, 2, 5, 5, 5, 6, 7, 8, 9]
        segtree.apply_range_add(0..2, 3); // [3, 4, 2, 5, 5, 5, 6, 7, 8, 9]
        segtree.apply_range_affine(8..10, 2, 7); // [3, 4, 2, 5, 5, 5, 6, 7, 23, 25]

        assert_eq!(segtree.all_sum(), 85);
        assert_eq!(segtree.range_sum(1..4), 11); // [4, 2, 5]
        assert_eq!(segtree.to_vec(), vec![3, 4, 2, 5, 5, 5, 6, 7, 23, 25]);
    }
    #[test]
    fn test_sample_of_lazy_segtree() {
        // range affine range sum の遅延セグ木の使用例
        let xs = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].map(RangeSum::unit).to_vec();

        let mut segtree: LazySegtree<RangeAffineRangeSum<i64>> = LazySegtree::from(xs);

        let f1 = Affine::constant_func(5.into());
        let f2 = Affine::addition_func(3.into());
        let f3 = Affine {
            slope: 2.into(),
            intercept: 7.into(),
        };

        segtree.apply_range(3..6, f1); // [0, 1, 2, 5, 5, 5, 6, 7, 8, 9]
        segtree.apply_range(0..2, f2); // [3, 4, 2, 5, 5, 5, 6, 7, 8, 9]
        segtree.apply_range(8..10, f3); // [3, 4, 2, 5, 5, 5, 6, 7, 23, 25]

        assert_eq!(segtree.all_prod().sum, 85);
    }
}