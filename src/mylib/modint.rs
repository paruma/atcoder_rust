#[allow(dead_code)]

// https://github.com/rust-lang-ja/ac-library-rs/blob/72fe2a19cf6efcb225327912d4da332425d1a37d/src/modint.rs#L513
// From<i32>とかあるのか。なるほどなぁ。

mod rr {
    pub const MOD: i64 = 1_000_000_007;

    #[allow(clippy::upper_case_acronyms)]
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
    use num::{One, Zero};

    //use super::*;

    #[test]
    fn test_rr() {
        use super::rr::*;
        let x = RR::new(3);
        let y = RR::new(7);

        assert_eq!(x.rep(), 3);
        assert_eq!(RR::one(), RR::new(1));
        assert_eq!(RR::zero(), RR::new(0));
        assert_eq!(x + y, RR::new(10));
        assert_eq!(x - y, RR::new(MOD - 4));
        assert_eq!(y - x, RR::new(4));
        assert_eq!(-x, RR::new(MOD - 3));
        assert_eq!((-x).rep(), MOD - 3);
        assert_eq!(x * y, RR::new(21));
    }

    #[test]
    fn test_rr_pow() {
        use super::rr::*;
        assert_eq!(num::pow(RR::new(2), 10), RR::new(1024));
    }
}
