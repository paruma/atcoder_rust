use cargo_snippet::snippet;

#[snippet(prefix = "use rf_fake::*;")]
#[snippet(prefix = "use num::{One, Zero};")]
pub mod rf_fake {
    #[derive(Clone, Copy, Debug, PartialEq, Default)]
    pub struct RF {
        rep: f64,
    }

    impl RF {
        pub fn new(x: f64) -> RF {
            RF { rep: x }
        }

        pub fn rep(self) -> f64 {
            self.rep
        }
    }

    impl RF {
        pub fn inv(self) -> Self {
            RF::new(1.0 / self.rep)
        }
    }

    impl num_traits::Zero for RF {
        fn zero() -> Self {
            RF::new(0.0)
        }

        fn is_zero(&self) -> bool {
            f64::is_zero(&self.rep)
        }
    }

    impl num_traits::One for RF {
        fn one() -> Self {
            RF::new(1.0)
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
    bi_ops_impl!(Div, div, /);

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
    use super::rf_fake::*;

    fn approx_eq(x: f64, y: f64) -> bool {
        #[allow(clippy::unreadable_literal)]
        #[allow(clippy::excessive_precision)]
        const EPS: f64 = 2.2204460492503131e-16_f64;
        (x - y).abs() < EPS
    }

    #[test]
    fn test_rf() {
        let x = RF::new(3.0);
        let y = RF::new(7.0);

        assert!(approx_eq(x.rep(), 3.0_f64));
        assert_eq!(RF::one(), RF::new(1.0));
        assert_eq!(RF::zero(), RF::new(0.0));
        assert_eq!(x + y, RF::new(10.0));
        assert_eq!(x - y, RF::new(-4.0));
        assert_eq!(y - x, RF::new(4.0));
        assert_eq!(-x, RF::new(-3.0));
        assert!(approx_eq((-x).rep(), -3.0));
        assert_eq!(x * y, RF::new(21.0));
        assert_eq!((x / y) * y, x);
        assert_eq!((y.inv()) * y, RF::one());
    }

    #[test]
    fn test_rf_assign() {
        let mut x = RF::new(3.0);
        let y = RF::new(4.0);

        x += y;
        assert_eq!(x, RF::new(7.0));

        x -= y;
        assert_eq!(x, RF::new(3.0));

        x *= y;
        assert_eq!(x, RF::new(12.0));

        x /= y;
        assert_eq!(x, RF::new(3.0));
    }

    #[test]
    fn test_rf_vec() {
        let xs = vec![RF::new(3.0), RF::new(4.0)];
        let z = xs.iter().fold(RF::zero(), |sum, x| sum + *x);
        assert_eq!(z, RF::new(7.0));
    }
}

// https://github.com/rust-lang-ja/ac-library-rs/blob/72fe2a19cf6efcb225327912d4da332425d1a37d/src/modint.rs#L513
// From<i32>とかあるのか。なるほどなぁ。
