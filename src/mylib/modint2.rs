// https://github.com/rust-lang-ja/ac-library-rs/blob/72fe2a19cf6efcb225327912d4da332425d1a37d/src/modint.rs#L513
// From<i32>とかあるのか。なるほどなぁ。

mod rf {
    pub const MOD: i64 = 1_000_000_007;

    #[allow(clippy::upper_case_acronyms)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct RF {
        rep: i64,
    }

    impl RF {
        pub fn new(rep: i64) -> RF {
            RF {
                rep: rep.rem_euclid(MOD),
            }
        }

        #[allow(dead_code)]
        pub fn rep(self) -> i64 {
            self.rep
        }
    }

    trait Ring: std::marker::Sized + Copy {
        fn zero() -> Self;
        fn one() -> Self;
        fn mul(self, rhs: Self) -> Self;
        fn add(self, rhs: Self) -> Self;
        fn neg(self) -> Self;
        fn sub(self, rhs: Self) -> Self {
            self.add(rhs.neg())
        }

        fn pow(self, n: i64) -> Self {
            if n == 0 {
                return Self::one();
            }
            let y = self.pow(n / 2);

            if n % 2 == 0 {
                y.mul(y)
            } else {
                self.mul(y.mul(y))
            }
        }
    }

    trait Field: Ring {
        fn inv(self) -> Self;
        fn div(self, rhs: Self) -> Self
        where
            Self: std::marker::Sized,
        {
            self.mul(rhs.inv())
        }
    }

    impl Ring for RF {
        fn zero() -> Self {
            RF::new(0)
        }

        fn one() -> Self {
            RF::new(1)
        }

        fn mul(self, rhs: Self) -> Self {
            RF::new(self.rep * rhs.rep)
        }

        fn add(self, rhs: Self) -> Self {
            RF::new(self.rep + rhs.rep)
        }

        fn neg(self) -> Self {
            RF::new(-self.rep)
        }
    }

    impl Field for RF {
        fn inv(self) -> Self {
            self.pow(MOD - 2)
        }
    }

    impl RF {
        #[allow(dead_code)]
        pub fn zero() -> Self {
            Ring::zero()
        }

        #[allow(dead_code)]
        pub fn one() -> Self {
            Ring::one()
        }

        #[allow(dead_code)]
        pub fn inv(self) -> Self {
            Field::inv(self)
        }
    }

    impl std::ops::Add for RF {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Ring::add(self, rhs)
        }
    }
    impl std::ops::Neg for RF {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Ring::neg(self)
        }
    }
    impl std::ops::Sub for RF {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Ring::sub(self, rhs)
        }
    }

    impl std::ops::Mul for RF {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Ring::mul(self, rhs)
        }
    }

    impl std::ops::Div for RF {
        type Output = Self;

        fn div(self, rhs: Self) -> Self::Output {
            Field::div(self, rhs)
        }
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

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
}
