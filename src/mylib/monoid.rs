use cargo_snippet::snippet;

#[snippet(prefix = "use monoid_bitwise::*;")]
pub mod monoid_bitwise {
    use std::{
        convert::Infallible,
        marker::PhantomData,
        ops::{BitAnd, BitOr, BitXor, Not},
    };

    use ac_library::Monoid;
    use num_traits::Zero;

    pub struct BitwiseOr<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for BitwiseOr<S>
    where
        S: Copy + BitOr<Output = S> + Zero,
    {
        type S = S;
        fn identity() -> Self::S {
            S::zero()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a | *b
        }
    }

    pub struct BitwiseAnd<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for BitwiseAnd<S>
    where
        S: Copy + BitAnd<Output = S> + Not<Output = S> + Zero,
    {
        type S = S;
        fn identity() -> Self::S {
            !S::zero()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a & *b
        }
    }

    pub struct BitwiseXor<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for BitwiseXor<S>
    where
        S: Copy + BitXor<Output = S> + Zero,
    {
        type S = S;
        fn identity() -> Self::S {
            S::zero()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a ^ *b
        }
    }
}

#[snippet(prefix = "use monoid_gcd_lcm::*;")]
pub mod monoid_gcd_lcm {
    use std::{convert::Infallible, marker::PhantomData};

    use ac_library::Monoid;
    use num_integer::Integer;

    pub struct Gcd<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for Gcd<S>
    where
        S: Integer + Clone,
    {
        type S = S;
        fn identity() -> Self::S {
            S::zero()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a.gcd(b)
        }
    }

    pub struct Lcm<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for Lcm<S>
    where
        S: Integer + Clone,
    {
        type S = S;
        fn identity() -> Self::S {
            S::one()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a.lcm(b)
        }
    }
}

#[snippet(prefix = "use monoid_modint::*;")]
pub mod monoid_modint {
    use std::{convert::Infallible, marker::PhantomData};

    use ac_library::{Modulus, Monoid, StaticModInt};

    pub struct MintAdditive<Mod>(Infallible, PhantomData<fn() -> Mod>);
    impl<Mod> Monoid for MintAdditive<Mod>
    where
        Mod: Modulus,
    {
        type S = StaticModInt<Mod>;
        fn identity() -> Self::S {
            StaticModInt::raw(0)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a + b
        }
    }

    pub struct MintMultiplicative<Mod>(Infallible, PhantomData<fn() -> Mod>);
    impl<Mod> Monoid for MintMultiplicative<Mod>
    where
        Mod: Modulus,
    {
        type S = StaticModInt<Mod>;
        fn identity() -> Self::S {
            StaticModInt::raw(1)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a * b
        }
    }
}

#[snippet(prefix = "use monoid_affine::*;")]
pub mod monoid_affine {
    use std::{
        convert::Infallible,
        marker::PhantomData,
        ops::{Add, Mul},
    };

    use ac_library::Monoid;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct AffineTransform<T> {
        slope: T,
        intercept: T,
    }
    impl<T> AffineTransform<T> {
        pub fn new(slope: T, intercept: T) -> Self {
            Self { slope, intercept }
        }

        pub fn apply(&self, x: T) -> T
        where
            T: Copy + Mul<Output = T> + Add<Output = T>,
        {
            self.slope * x + self.intercept
        }

        pub fn identity() -> Self
        where
            T: From<i64>,
        {
            Self { slope: 1.into(), intercept: 0.into() }
        }

        pub fn composite(&self, rhs: &Self) -> Self
        where
            T: Copy + Mul<Output = T> + Add<Output = T>,
        {
            Self {
                slope: self.slope * rhs.slope,
                intercept: self.slope * rhs.intercept + self.intercept,
            }
        }
    }

    pub struct AffineComposition<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for AffineComposition<T>
    where
        T: Copy + From<i64> + Add<Output = T> + Mul<Output = T>,
    {
        type S = AffineTransform<T>;
        fn identity() -> Self::S {
            AffineTransform::identity()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a.composite(b)
        }
    }
}

#[cfg(test)]
mod test {

    use crate::mylib::monoid::monoid_affine::AffineComposition;
    use crate::mylib::monoid::monoid_affine::AffineTransform;

    use super::monoid_bitwise::*;
    use super::monoid_gcd_lcm::*;
    use super::monoid_modint::*;
    use ac_library::Mod998244353;
    use ac_library::ModInt998244353;
    use ac_library::Monoid;

    #[test]
    fn test_monoid_additive() {
        type Mint = ModInt998244353;
        type M = MintAdditive<Mod998244353>;
        assert_eq!(M::binary_operation(&Mint::new(3), &Mint::new(4)), Mint::new(7));
        assert_eq!(M::binary_operation(&Mint::new(3), &M::identity()), Mint::new(3));
    }

    #[test]
    fn test_monoid_multiplicative() {
        type Mint = ModInt998244353;
        type M = MintMultiplicative<Mod998244353>;
        assert_eq!(M::binary_operation(&Mint::new(3), &Mint::new(4)), Mint::new(12));
        assert_eq!(M::binary_operation(&Mint::new(3), &M::identity()), Mint::new(3));
    }

    #[test]
    fn test_monoid_bitwise_or() {
        type M = BitwiseOr<i64>;
        assert_eq!(M::binary_operation(&0b0110, &0b0011), 0b0111);
        assert_eq!(M::binary_operation(&0b0110, &M::identity()), 0b0110);
    }
    #[test]
    fn test_monoid_bitwise_and() {
        type M = BitwiseAnd<i64>;
        assert_eq!(M::binary_operation(&0b0110, &0b0011), 0b0010);
        assert_eq!(M::binary_operation(&0b0110, &M::identity()), 0b0110);
    }
    #[test]
    fn test_monoid_bitwise_xor() {
        type M = BitwiseXor<i64>;
        assert_eq!(M::binary_operation(&0b0110, &0b0011), 0b0101);
        assert_eq!(M::binary_operation(&0b0110, &M::identity()), 0b0110);
    }

    #[test]
    fn test_monoid_gcd() {
        type M = Gcd<i64>;
        assert_eq!(M::binary_operation(&12, &8), 4);
        assert_eq!(M::binary_operation(&12, &M::identity()), 12);
    }

    #[test]
    fn test_monoid_lcm() {
        type M = Lcm<i64>;
        assert_eq!(M::binary_operation(&12, &8), 24);
        assert_eq!(M::binary_operation(&12, &M::identity()), 12);
    }

    #[test]
    fn test_monoid_affine() {
        type Mint = ModInt998244353;
        type M = AffineComposition<Mint>;
        let affine1: AffineTransform<Mint> = AffineTransform::new(3.into(), 5.into());
        let affine2: AffineTransform<Mint> = AffineTransform::new(5.into(), 2.into());
        assert_eq!(
            M::binary_operation(&affine1, &affine2),
            AffineTransform::new(15.into(), 11.into())
        );
        assert_eq!(M::binary_operation(&affine1, &M::identity()), affine1)
    }
}
