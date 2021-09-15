// マクロで遊んだ。

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

    macro_rules! bi_ops {
        ($stdops: ident, $fn: ident) => {
            impl std::ops::$stdops for RR {
                type Output = Self;

                fn $fn(self, rhs: Self) -> Self::Output {
                    Ring::$fn(self, rhs)
                }
            }
        };
    }
    bi_ops!(Add, add);
    bi_ops!(Sub, sub);
    bi_ops!(Mul, mul);

    impl std::ops::Neg for RR {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Ring::neg(self)
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
        use num;
        let x = vec![RR::new(3), RR::new(4)];
        let y = x.iter();
        y.fold(RR::zero(), |acc, x| acc + *x);

        assert_eq!(num::pow(RR::new(2), 10), RR::new(1024));
    }
}
