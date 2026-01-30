use cargo_snippet::snippet;

#[snippet(prefix = "use min_max_monoid::*;")]
#[allow(clippy::module_inception)]
pub mod min_max_monoid {
    use ac_library::Monoid;
    use std::convert::Infallible;
    use std::marker::PhantomData;

    pub trait BoundedBelow {
        fn min_value() -> Self;
    }

    pub trait BoundedAbove {
        fn max_value() -> Self;
    }

    macro_rules! impl_bounded {
        ($($ty:ty),*) => {
            $(
                impl BoundedBelow for $ty {
                    #[inline]
                    fn min_value() -> Self {
                        Self::MIN
                    }
                }

                impl BoundedAbove for $ty {
                    #[inline]
                    fn max_value() -> Self {
                        Self::MAX
                    }
                }
            )*
        };
    }

    impl_bounded!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

    impl<T: BoundedAbove> BoundedBelow for std::cmp::Reverse<T> {
        #[inline]
        fn min_value() -> Self {
            std::cmp::Reverse(T::max_value())
        }
    }

    impl<T: BoundedBelow> BoundedAbove for std::cmp::Reverse<T> {
        #[inline]
        fn max_value() -> Self {
            std::cmp::Reverse(T::min_value())
        }
    }

    macro_rules! impl_bounded_tuples {
        ($head:ident) => {};
        ($head:ident, $($tail:ident),*) => {
            impl<$head, $($tail),*> BoundedBelow for ($head, $($tail),*)
            where
                $head: BoundedBelow,
                $($tail: BoundedBelow),*
            {
                #[inline]
                fn min_value() -> Self {
                    ($head::min_value(), $($tail::min_value()),*)
                }
            }

            impl<$head, $($tail),*> BoundedAbove for ($head, $($tail),*)
            where
                $head: BoundedAbove,
                $($tail: BoundedAbove),*
            {
                #[inline]
                fn max_value() -> Self {
                    ($head::max_value(), $($tail::max_value()),*)
                }
            }

            impl_bounded_tuples!($($tail),*);
        };
        () => {};
    }

    impl_bounded_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);

    /// 辞書式順序で最小の要素を管理するモノイド (単位元は最大値)
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct TupleMin<T>(Infallible, PhantomData<fn() -> T>);

    impl<T> Monoid for TupleMin<T>
    where
        T: BoundedAbove + Ord + Clone,
    {
        type S = T;
        fn identity() -> Self::S {
            T::max_value()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            std::cmp::min(a, b).clone()
        }
    }

    /// 辞書式順序で最大の要素を管理するモノイド (単位元は最小値)
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct TupleMax<T>(Infallible, PhantomData<fn() -> T>);

    impl<T> Monoid for TupleMax<T>
    where
        T: BoundedBelow + Ord + Clone,
    {
        type S = T;
        fn identity() -> Self::S {
            T::min_value()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            std::cmp::max(a, b).clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::min_max_monoid::*;
    use ac_library::Monoid;
    use std::cmp::Reverse;

    #[test]
    fn test_tuple_min_monoid() {
        type M = TupleMin<(i64, i64, i64)>;
        let identity = M::identity();
        assert_eq!(identity, (i64::MAX, i64::MAX, i64::MAX));

        let a = (10, 20, 30);
        let b = (10, 15, 100);
        let c = (5, 100, 100);

        assert_eq!(M::binary_operation(&a, &b), (10, 15, 100));
        assert_eq!(M::binary_operation(&b, &c), (5, 100, 100));
        assert_eq!(M::binary_operation(&a, &identity), a);
    }

    #[test]
    fn test_tuple_max_monoid() {
        type M = TupleMax<(i64, i64)>;
        let identity = M::identity();
        assert_eq!(identity, (i64::MIN, i64::MIN));

        let a = (10, 20);
        let b = (10, 25);
        let c = (5, 100);

        assert_eq!(M::binary_operation(&a, &b), (10, 25));
        assert_eq!(M::binary_operation(&b, &c), (10, 25));
        assert_eq!(M::binary_operation(&a, &identity), a);
    }

    #[test]
    fn test_tuple_long() {
        type M = TupleMin<(i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64)>;
        let identity = M::identity();
        assert_eq!(identity.0, i64::MAX);
        assert_eq!(identity.11, i64::MAX);
    }

    #[test]
    fn test_tuple_min_monoid_primitive() {
        type M = TupleMin<i64>;
        assert_eq!(M::identity(), i64::MAX);
        assert_eq!(M::binary_operation(&10, &20), 10);
    }

    #[test]
    fn test_reverse_monoid() {
        type M = TupleMin<Reverse<i64>>;
        let identity = M::identity();
        assert_eq!(identity, Reverse(i64::MIN));

        let a = Reverse(10);
        let b = Reverse(20);
        let c = Reverse(5);

        assert_eq!(M::binary_operation(&a, &b), Reverse(20));
        assert_eq!(M::binary_operation(&a, &c), Reverse(10));
    }

    #[test]
    fn test_tuple_with_reverse() {
        type M = TupleMin<(i64, Reverse<i64>)>;
        let identity = M::identity();
        assert_eq!(identity, (i64::MAX, Reverse(i64::MIN)));

        let a = (10, Reverse(100));
        let b = (10, Reverse(200));
        let c = (5, Reverse(50));

        assert_eq!(M::binary_operation(&a, &b), (10, Reverse(200)));
        assert_eq!(M::binary_operation(&a, &c), (5, Reverse(50)));
    }

    #[test]
    fn test_tuple_with_ext_int() {
        use crate::math::ext_int::mod_ext_int::ExtInt;
        type M = TupleMin<(ExtInt, i64)>;
        let identity = M::identity();
        assert_eq!(identity, (ExtInt::INF, i64::MAX));

        let a = (ExtInt::fin(10), 100);
        let b = (ExtInt::INF, 200);
        assert_eq!(M::binary_operation(&a, &b), a);
    }

    #[test]
    fn test_tuple_with_neg_ext_int() {
        use crate::math::neg_ext_int::mod_neg_ext_int::NegExtInt;
        type M = TupleMax<(NegExtInt, i64)>;
        let identity = M::identity();
        assert_eq!(identity, (NegExtInt::NEG_INF, i64::MIN));

        let a = (NegExtInt::fin(10), 100);
        let b = (NegExtInt::NEG_INF, 200);
        assert_eq!(M::binary_operation(&a, &b), a);
    }
}
