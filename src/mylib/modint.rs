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
        pub fn new(x: i64) -> RR {
            RR {
                rep: x.rem_euclid(MOD),
            }
        }

        pub fn rep(self) -> i64 {
            self.rep
        }
    }

    impl num_traits::Zero for RR {
        fn zero() -> Self {
            RR::new(0)
        }

        fn is_zero(&self) -> bool {
            self.rep == 0
        }
    }

    impl num_traits::One for RR {
        fn one() -> Self {
            RR::new(1)
        }
    }

    macro_rules! bi_ops_impl {
        ($std_ops: ident, $fn: ident, $op: tt) => {
            impl std::ops::$std_ops for RR {
                type Output = Self;

                fn $fn (self, rhs: Self) -> Self::Output {
                    RR::new(self.rep $op rhs.rep)
                }
            }
        };
    }

    bi_ops_impl!(Add, add, +);
    bi_ops_impl!(Sub, sub, -);
    bi_ops_impl!(Mul, mul, *);

    macro_rules! bi_ops_assign_impl {
        ($std_ops_assign: ident, $fn_assign: ident, $op: tt) => {
            impl std::ops::$std_ops_assign for RR {
                fn $fn_assign(&mut self, rhs: Self) {
                    *self = *self $op rhs
                }
            }
        };
    }

    bi_ops_assign_impl!(AddAssign, add_assign, +);
    bi_ops_assign_impl!(SubAssign, sub_assign, -);
    bi_ops_assign_impl!(MulAssign, mul_assign, *);

    impl std::ops::Neg for RR {
        type Output = Self;

        fn neg(self) -> Self::Output {
            RR::new(-self.rep)
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
