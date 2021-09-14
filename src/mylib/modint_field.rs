use cargo_snippet::snippet;

#[snippet]
pub mod rf {
    pub const MOD: i64 = 1_000_000_007;

    #[allow(clippy::upper_case_acronyms)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct RF {
        rep: i64,
    }

    impl RF {
        pub fn new(x: i64) -> RF {
            RF {
                rep: x.rem_euclid(MOD),
            }
        }

        pub fn rep(self) -> i64 {
            self.rep
        }
    }

    impl RF {
        pub fn inv(self) -> Self {
            num::pow(self, (MOD - 2) as usize)
        }
    }

    impl num_traits::Zero for RF {
        fn zero() -> Self {
            RF::new(0)
        }

        fn is_zero(&self) -> bool {
            self.rep == 0
        }
    }

    impl num_traits::One for RF {
        fn one() -> Self {
            RF::new(1)
        }
    }

    macro_rules! bi_ops_impl {
        ($std_ops: ident, $fn: ident, $op: tt) => {
            impl std::ops::$std_ops for RF {
                type Output = Self;

                fn $fn (self, rhs: Self) -> Self::Output {
                    RF::new(self.rep $op rhs.rep)
                }
            }
        };
    }

    bi_ops_impl!(Add, add, +);
    bi_ops_impl!(Sub, sub, -);
    bi_ops_impl!(Mul, mul, *);

    impl std::ops::Div for RF {
        type Output = Self;
        fn div(self, rhs: Self) -> Self::Output {
            // *はまだ使えない？
            std::ops::Mul::mul(self, rhs.inv())
        }
    }

    macro_rules! bi_ops_assign_impl {
        ($std_ops_assign: ident, $fn_assign: ident, $op: tt) => {
            impl std::ops::$std_ops_assign for RF {
                fn $fn_assign(&mut self, rhs: Self) {
                    *self = *self $op rhs
                }
            }
        };
    }

    bi_ops_assign_impl!(AddAssign, add_assign, +);
    bi_ops_assign_impl!(SubAssign, sub_assign, -);
    bi_ops_assign_impl!(MulAssign, mul_assign, *);
    bi_ops_assign_impl!(DivAssign, div_assign, /);

    impl std::ops::Neg for RF {
        type Output = Self;

        fn neg(self) -> Self::Output {
            RF::new(-self.rep)
        }
    }
}

#[cfg(test)]
mod tests {
    use num::{One, Zero};

    //use super::*;
    use super::rf::*;

    #[test]
    fn test_rf() {
        use super::rf::*;

        let x = RF::new(3);
        let y = RF::new(7);

        assert_eq!(x.rep(), 3);
        assert_eq!(RF::one(), RF::new(1));
        assert_eq!(RF::zero(), RF::new(0));
        assert_eq!(x + y, RF::new(10));
        assert_eq!(x - y, RF::new(MOD - 4));
        assert_eq!(y - x, RF::new(4));
        assert_eq!(-x, RF::new(MOD - 3));
        assert_eq!((-x).rep(), MOD - 3);
        assert_eq!(x * y, RF::new(21));
        assert_eq!((x / y) * y, x);
        assert_eq!((y.inv()) * y, RF::one());
    }

    #[test]
    fn test_rf_assign() {
        let mut x = RF::new(3);
        let y = RF::new(4);

        x += y;
        assert_eq!(x, RF::new(7));

        x -= y;
        assert_eq!(x, RF::new(3));

        x *= y;
        assert_eq!(x, RF::new(12));

        x /= y;
        assert_eq!(x, RF::new(3));
    }

    #[test]
    fn test_rf_vec() {
        let xs = vec![RF::new(3), RF::new(4)];
        let z = xs.iter().fold(RF::zero(), |sum, x| sum + *x);
        assert_eq!(z, RF::new(7));
    }
}

// https://github.com/rust-lang-ja/ac-library-rs/blob/72fe2a19cf6efcb225327912d4da332425d1a37d/src/modint.rs#L513
// From<i32>とかあるのか。なるほどなぁ。
