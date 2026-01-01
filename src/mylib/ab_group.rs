use cargo_snippet::snippet;
use std::{
    convert::Infallible,
    iter::Sum,
    marker::PhantomData,
    ops::{Add, Neg, Sub},
};

#[snippet(prefix = "use ab_group::*;")]
#[allow(clippy::module_inception)]
pub mod ab_group {
    use super::*;

    /// 可換群 (Abelian Group)
    pub trait AbGroup {
        type S: Clone;
        fn zero() -> Self::S;
        fn add(a: &Self::S, b: &Self::S) -> Self::S;
        fn neg(a: &Self::S) -> Self::S;
        fn sub(a: &Self::S, b: &Self::S) -> Self::S {
            Self::add(a, &Self::neg(b))
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct AdditiveAbGroup<T>(Infallible, PhantomData<fn() -> T>);
    impl<T: Sum + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy> AbGroup
        for AdditiveAbGroup<T>
    {
        type S = T;
        fn zero() -> Self::S {
            std::iter::empty().sum()
        }
        fn add(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
        fn neg(a: &Self::S) -> Self::S {
            -(*a)
        }
        fn sub(a: &Self::S, b: &Self::S) -> Self::S {
            *a - *b
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct XorAbGroup(Infallible);

    impl AbGroup for XorAbGroup {
        type S = u64;
        fn zero() -> Self::S {
            0
        }
        fn add(a: &Self::S, b: &Self::S) -> Self::S {
            *a ^ *b
        }
        fn neg(a: &Self::S) -> Self::S {
            *a
        }
        fn sub(a: &Self::S, b: &Self::S) -> Self::S {
            *a ^ *b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ab_group::*;
    use rand::prelude::*;

    fn test_ab_group_properties<G, R, F>(name: &str, mut rng: R, generator: F)
    where
        G: AbGroup,
        G::S: PartialEq + std::fmt::Debug,
        R: Rng,
        F: Fn(&mut R) -> G::S,
    {
        for _ in 0..100 {
            let a = generator(&mut rng);
            let b = generator(&mut rng);
            let c = generator(&mut rng);
            let zero = G::zero();

            // Associativity: (a + b) + c = a + (b + c)
            let lhs = G::add(&G::add(&a, &b), &c);
            let rhs = G::add(&a, &G::add(&b, &c));
            assert_eq!(lhs, rhs, "{}: Associativity failed", name);

            // Identity: a + 0 = a, 0 + a = a
            assert_eq!(G::add(&a, &zero), a, "{}: Identity (right) failed", name);
            assert_eq!(G::add(&zero, &a), a, "{}: Identity (left) failed", name);

            // Inverse: a + (-a) = 0, (-a) + a = 0
            let neg_a = G::neg(&a);
            assert_eq!(G::add(&a, &neg_a), zero, "{}: Inverse (right) failed", name);
            assert_eq!(G::add(&neg_a, &a), zero, "{}: Inverse (left) failed", name);

            // Commutativity: a + b = b + a
            assert_eq!(
                G::add(&a, &b),
                G::add(&b, &a),
                "{}: Commutativity failed",
                name
            );

            // Subtraction consistency: a - b = a + (-b)
            assert_eq!(
                G::sub(&a, &b),
                G::add(&a, &G::neg(&b)),
                "{}: Subtraction failed",
                name
            );
        }
    }

    #[test]
    fn test_additive_ab_group_i32() {
        let rng = rand::rng();
        test_ab_group_properties::<AdditiveAbGroup<i32>, _, _>(
            "AdditiveAbGroup<i32>",
            rng,
            |rng| rng.random_range(-10..=10),
        );
    }

    #[test]
    fn test_additive_ab_group_i64() {
        let rng = rand::rng();
        test_ab_group_properties::<AdditiveAbGroup<i64>, _, _>(
            "AdditiveAbGroup<i64>",
            rng,
            |rng| rng.random_range(-10..=10),
        );
    }

    #[test]
    fn test_xor_ab_group() {
        let rng = rand::rng();
        test_ab_group_properties::<XorAbGroup, _, _>("XorAbGroup", rng, |rng| {
            rng.random_range(0..=16)
        });
    }
}
