use cargo_snippet::snippet;

#[snippet(prefix = "use range_affine_range_sum::*;")]
pub mod range_affine_range_sum {
    use ac_library::{MapMonoid, Monoid};
    use std::convert::Infallible;
    use std::marker::PhantomData;
    use std::ops::{Add, Mul};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ValueLen<T> {
        pub value: T,
        pub len: i64,
    }
    impl<T> ValueLen<T> {
        pub fn unit(x: T) -> ValueLen<T> {
            ValueLen { value: x, len: 1 }
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
        type S = ValueLen<T>;
        fn identity() -> ValueLen<T> {
            ValueLen {
                value: 0.into(),
                len: 0,
            }
        }
        fn binary_operation(a: &ValueLen<T>, b: &ValueLen<T>) -> ValueLen<T> {
            ValueLen {
                value: a.value + b.value,
                len: a.len + b.len,
            }
        }
    }

    pub struct AffineSum<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for AffineSum<T>
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

        fn mapping(f: &Affine<T>, x: &ValueLen<T>) -> ValueLen<T> {
            ValueLen {
                value: f.slope * x.value + f.intercept * x.len.into(),
                len: x.len,
            }
        }
    }
}

#[snippet(prefix = "use range_affine_range_minmax::*;")]
pub mod range_affine_range_minmax {
    use std::{cmp::Ordering, convert::Infallible};

    use ac_library::{MapMonoid, Monoid};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct MinMaxLen {
        pub min: i64,
        pub max: i64,
        pub len: i64,
    }
    impl MinMaxLen {
        pub fn unit(x: i64) -> MinMaxLen {
            MinMaxLen {
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

    pub struct MinMaxLenMonoid(Infallible);
    impl Monoid for MinMaxLenMonoid {
        type S = MinMaxLen;
        fn identity() -> MinMaxLen {
            MinMaxLen {
                // INF, -INF は len == 0のときだけ使う。
                min: INF,
                max: -INF,
                len: 0,
            }
        }
        fn binary_operation(a: &MinMaxLen, b: &MinMaxLen) -> MinMaxLen {
            MinMaxLen {
                min: Ord::min(a.min, b.min),
                max: Ord::max(a.max, b.max),
                len: a.len + b.len,
            }
        }
    }

    const INF: i64 = i64::MAX;

    pub struct AffineMinMax(Infallible);
    impl MapMonoid for AffineMinMax {
        type M = MinMaxLenMonoid;
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

        fn mapping(f: &Affine, x: &MinMaxLen) -> MinMaxLen {
            if x.len == 0 {
                return MinMaxLenMonoid::identity();
            }

            match f.slope.cmp(&0) {
                Ordering::Equal => MinMaxLen {
                    min: f.intercept,
                    max: f.intercept,
                    len: x.len,
                },
                Ordering::Greater => MinMaxLen {
                    min: f.intercept + f.slope * x.min,
                    max: f.intercept + f.slope * x.max,
                    len: x.len,
                },
                Ordering::Less => MinMaxLen {
                    min: f.intercept + f.slope * x.max,
                    max: f.intercept + f.slope * x.min,
                    len: x.len,
                },
            }
        }
    }
}

#[cfg(test)]
mod test_range_affine_range_sum {
    use ac_library::{LazySegtree, MapMonoid, ModInt998244353, Monoid};

    use super::range_affine_range_sum::*;

    type Mint = ModInt998244353;
    type DataM = ValueLenSum<Mint>;
    type ActionM = AffineSum<Mint>;

    #[test]
    fn test_value_len_sum() {
        let x1 = ValueLen::unit(5.into());
        let x2 = ValueLen::unit(9.into());

        assert_eq!(
            DataM::binary_operation(&x1, &x2),
            ValueLen {
                value: 14.into(),
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
        let x1 = ValueLen {
            value: 3.into(),
            len: 2,
        };
        let empty = ValueLen {
            value: 0.into(),
            len: 0,
        };

        assert_eq!(
            // [1, 2] を [5, 5] に変換したときの区間和とその長さ
            ActionM::mapping(&f, &x1),
            ValueLen {
                value: 10.into(),
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
        let x1 = ValueLen {
            value: 3.into(),
            len: 2,
        };
        let empty = ValueLen {
            value: 0.into(),
            len: 0,
        };

        assert_eq!(
            // [1, 2] を [6, 7] に変換したときの区間和とその長さ
            ActionM::mapping(&f, &x1),
            ValueLen {
                value: 13.into(),
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
            .map(|x| ValueLen::unit(Mint::new(x)))
            .fold(DataM::identity(), |acc, x| {
                DataM::binary_operation(&acc, &x)
            });

        let x2 = ValueLen {
            value: 0.into(),
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
            ValueLen {
                value: 33.into(), // 8 + 11 + 14 = 33
                len: 3,
            }
        );

        assert_eq!(
            // sum {3x + 5 | x in []}
            ActionM::mapping(&f1, &x2),
            ValueLen {
                value: 0.into(),
                len: 0,
            }
        );

        assert_eq!(
            // sum {0x + 10 | x in [1, 2, 3]}
            ActionM::mapping(&f2, &x1),
            ValueLen {
                value: 30.into(), // 10 + 10 + 10
                len: 3,
            }
        );
    }

    #[test]
    fn test_monoid_map_axiom() {
        let axiom_of_id = |x: ValueLen<Mint>| {
            // id(x) = x
            assert_eq!(ActionM::mapping(&ActionM::identity_map(), &x), x);
        };

        let axiom_of_prod_act = |f1: Affine<Mint>, f2: Affine<Mint>, x: ValueLen<Mint>| {
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

        let axiom_of_keeping_prod = |f: Affine<Mint>, x1: ValueLen<Mint>, x2: ValueLen<Mint>| {
            // f(x1*x2) = f(x1)*f(x2)
            assert_eq!(
                ActionM::mapping(&f, &DataM::binary_operation(&x1, &x2)),
                DataM::binary_operation(&ActionM::mapping(&f, &x1), &ActionM::mapping(&f, &x2))
            );
        };

        let x1 = ValueLen {
            value: 6.into(),
            len: 2,
        };
        let x2 = ValueLen {
            value: 10.into(),
            len: 3,
        };

        let empty = ValueLen {
            value: 0.into(),
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
    fn test_sample_of_lazy_segtree() {
        // range affine range sum の遅延セグ木の使用例
        let xs = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].map(ValueLen::unit).to_vec();

        let mut segtree: LazySegtree<AffineSum<i64>> = LazySegtree::from(xs);

        let f1 = Affine::constant_func(5.into());
        let f2 = Affine::addition_func(3.into());
        let f3 = Affine {
            slope: 2.into(),
            intercept: 7.into(),
        };

        segtree.apply_range(3..6, f1); // [0, 1, 2, 5, 5, 5, 6, 7, 8, 9]
        segtree.apply_range(0..2, f2); // [3, 4, 2, 5, 5, 5, 6, 7, 8, 9]
        segtree.apply_range(8..10, f3); // [3, 4, 2, 5, 5, 5, 6, 7, 23, 25]

        assert_eq!(segtree.all_prod().value, 85);
    }
}

#[cfg(test)]
mod test_range_affine_range_minmax {
    use ac_library::{LazySegtree, MapMonoid, Monoid};

    use super::range_affine_range_minmax::*;

    type DataM = MinMaxLenMonoid;
    type ActionM = AffineMinMax;

    #[test]
    fn test_minmax_len_monoid() {
        let x1 = MinMaxLen::unit(5.into());
        let x2 = MinMaxLen::unit(9.into());

        assert_eq!(
            DataM::binary_operation(&x1, &x2),
            MinMaxLen {
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
        let f = Affine::constant_func(5.into());

        // 例えば [1, 2] に対する区間和とその長さ
        let x1 = MinMaxLen {
            min: 1,
            max: 2,
            len: 2,
        };
        let empty = MinMaxLenMonoid::identity();

        assert_eq!(
            // [1, 2] を [5, 5] に変換したときの区間和とその長さ
            ActionM::mapping(&f, &x1),
            MinMaxLen {
                min: 5.into(),
                max: 5.into(),
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
        let x1 = MinMaxLen {
            min: 1,
            max: 2,
            len: 2,
        };
        let empty = MinMaxLenMonoid::identity();

        assert_eq!(
            // [1, 2] を [6, 7] に変換したときの区間和とその長さ
            ActionM::mapping(&f, &x1),
            MinMaxLen {
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
            .map(MinMaxLen::unit)
            .fold(DataM::identity(), |acc, x| {
                DataM::binary_operation(&acc, &x)
            });

        let x2 = MinMaxLenMonoid::identity();

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
            MinMaxLen {
                min: 8,
                max: 14,
                len: 3,
            }
        );

        assert_eq!(
            // [3x + 5 | x in []] = []
            ActionM::mapping(&f1, &x2),
            MinMaxLenMonoid::identity()
        );

        assert_eq!(
            // [0x + 10 | x in [1, 2, 3]] = [10, 10, 10]
            ActionM::mapping(&f2, &x1),
            MinMaxLen {
                min: 10,
                max: 10,
                len: 3,
            }
        );

        assert_eq!(
            // [-3x - 5 | x in [1, 2, 3]] = [-8, -11, -14]
            ActionM::mapping(&f3, &x1),
            MinMaxLen {
                min: -14,
                max: -8,
                len: 3,
            }
        );
    }

    #[test]
    fn test_monoid_map_axiom() {
        let axiom_of_id = |x: MinMaxLen| {
            // id(x) = x
            assert_eq!(ActionM::mapping(&ActionM::identity_map(), &x), x);
        };

        let axiom_of_prod_act = |f1: Affine, f2: Affine, x: MinMaxLen| {
            // (f1*f2)(x) = f1(f2(x))
            assert_eq!(
                ActionM::mapping(&ActionM::composition(&f1, &f2), &x),
                ActionM::mapping(&f1, &ActionM::mapping(&f2, &x))
            );
        };

        let axiom_of_keeping_unit = |f: Affine| {
            // これは必須ではない？
            // f(e) = e
            assert_eq!(
                ActionM::mapping(&f, &ActionM::mapping(&f, &DataM::identity())),
                DataM::identity()
            );
        };

        let axiom_of_keeping_prod = |f: Affine, x1: MinMaxLen, x2: MinMaxLen| {
            // f(x1*x2) = f(x1)*f(x2)
            assert_eq!(
                ActionM::mapping(&f, &DataM::binary_operation(&x1, &x2)),
                DataM::binary_operation(&ActionM::mapping(&f, &x1), &ActionM::mapping(&f, &x2))
            );
        };

        let x1 = MinMaxLen {
            min: 3,
            max: 6,
            len: 2,
        };
        let x2 = MinMaxLen {
            min: -3,
            max: 9,
            len: 3,
        };

        let empty = MinMaxLenMonoid::identity();

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
    fn test_sample_of_lazy_segtree() {
        // range affine range min/max の遅延セグ木の使用例
        let xs = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].map(MinMaxLen::unit).to_vec();

        let mut segtree: LazySegtree<AffineMinMax> = LazySegtree::from(xs);

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
    }
}
