mod sum {
    pub trait Sum2<A>: Sized {
        fn sum2<I: Iterator<Item = A>>(iter: I) -> Self;
    }

    impl<'a, T: num::Zero + std::ops::Add<T, Output = T> + Copy> Sum2<&'a T> for T {
        fn sum2<I: Iterator<Item = &'a T>>(iter: I) -> Self {
            iter.fold(Self::zero(), |acc, x| acc + *x)
        }
    }

    impl<T: num::Zero + std::ops::Add<T, Output = T> + Copy> Sum2<T> for T {
        fn sum2<I: Iterator<Item = T>>(iter: I) -> Self {
            iter.fold(Self::zero(), |acc, x| acc + x)
        }
    }

    pub trait IteratorExtSum2: Iterator + Sized {
        fn sum2<S>(self) -> S
        where
            Self: Sized,
            S: Sum2<Self::Item>,
        {
            Sum2::sum2(self)
        }
    }

    impl<T: Iterator> IteratorExtSum2 for T {}
}

// ModInt
mod rr {
    pub const MOD: i64 = 1_000_000_007;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct RR {
        rep: i64,
    }

    impl RR {
        pub fn new(rep: i64) -> RR {
            RR {
                rep: rep.rem_euclid(MOD),
            }
        }

        #[allow(dead_code)]
        pub fn rep(self) -> i64 {
            self.rep
        }
    }

    trait Ring {
        fn zero() -> Self;
        fn one() -> Self;
        fn mul(self, rhs: Self) -> Self;
        fn add(self, rhs: Self) -> Self;
        fn neg(self) -> Self;
        fn sub(self, rhs: Self) -> Self
        where
            Self: std::marker::Sized,
        {
            self.add(rhs.neg())
        }
    }

    impl Ring for RR {
        fn zero() -> Self {
            RR::new(0)
        }

        fn one() -> Self {
            RR::new(1)
        }

        fn mul(self, rhs: Self) -> Self {
            RR::new(self.rep * rhs.rep)
        }

        fn add(self, rhs: Self) -> Self {
            RR::new(self.rep + rhs.rep)
        }

        fn neg(self) -> Self {
            RR::new(-self.rep)
        }
    }

    impl num_traits::Zero for RR {
        fn zero() -> Self {
            Ring::zero()
        }

        fn is_zero(&self) -> bool {
            self.rep == 0
        }
    }

    impl num_traits::One for RR {
        fn one() -> Self {
            Ring::one()
        }
    }

    impl std::ops::Add for RR {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Ring::add(self, rhs)
        }
    }
    impl std::ops::Neg for RR {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Ring::neg(self)
        }
    }
    impl std::ops::Sub for RR {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Ring::sub(self, rhs)
        }
    }

    impl std::ops::Mul for RR {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Ring::mul(self, rhs)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::rr::*;
    use super::sum::*;

    #[allow(clippy::redundant_clone)]
    #[test]
    fn test_sum() {
        let xs = vec![1, 2, 3];

        let s1: i32 = xs.iter().sum();
        let s2: i32 = xs.clone().into_iter().sum();
        let s3: i32 = xs.iter().sum2();
        let s4: i32 = xs.clone().into_iter().sum2();

        assert_eq!(s1, 6);
        assert_eq!(s2, 6);
        assert_eq!(s3, 6);
        assert_eq!(s4, 6);
    }

    #[test]
    fn test_sum_rr() {
        let xs = vec![RR::new(2), RR::new(3), RR::new(4)];

        let s1: RR = xs.iter().sum2();
        let s2: RR = xs.into_iter().sum2();

        assert_eq!(s1, RR::new(9));
        assert_eq!(s2, RR::new(9));
    }
}
