// https://github.com/rust-lang-ja/ac-library-rs/blob/72fe2a19cf6efcb225327912d4da332425d1a37d/src/modint.rs#L513
// From<i32>とかあるのか。なるほどなぁ。

//const MOD: i64 = 1_000_000_007;

trait Ring {
    fn zero() -> Self;
    fn one() -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn add(&self, other: &Self) -> Self;
    fn neg(&self) -> Self;
    fn sub(&self, other: &Self) -> Self
    where
        Self: std::marker::Sized,
    {
        self.add(&other.neg())
    }
}

const MOD: i64 = 1_000_000_007;

// Super Redidue Ring
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
struct RR {
    rep: i64,
}

impl RR {
    fn new(rep: i64) -> RR {
        RR {
            rep: rep.rem_euclid(MOD),
        }
    }
}

impl Ring for RR {
    fn zero() -> Self {
        RR { rep: 0 }
    }

    fn one() -> Self {
        RR { rep: 1 }
    }

    fn mul(&self, other: &Self) -> Self {
        RR::new(self.rep * other.rep)
    }

    fn add(&self, other: &Self) -> Self {
        RR::new(self.rep + other.rep)
    }

    fn neg(&self) -> Self {
        RR::new(-self.rep)
    }
}

impl std::ops::Add for RR {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Ring::add(&self, &rhs)
    }
}

impl std::ops::Sub for RR {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Ring::sub(&self, &rhs)
    }
}

impl std::ops::Mul for RR {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Ring::mul(&self, &rhs)
    }
}

impl std::ops::Neg for RR {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Ring::neg(&self)
    }
}

mod exam {
    use super::*;
    impl Ring for i32 {
        fn zero() -> Self {
            0
        }

        fn one() -> Self {
            1
        }

        fn mul(&self, other: &Self) -> Self {
            self * other
        }

        fn add(&self, other: &Self) -> Self {
            self + other
        }

        fn neg(&self) -> Self {
            -self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring_i32() {
        let x = 3;
        let y = 4;
        assert_eq!(x.mul(&y), 12);
        assert_eq!(x.add(&y), 7);
        assert_eq!(x.sub(&y), -1);
        assert_eq!(x.neg(), -3);
        assert_eq!(i32::one(), 1);
        assert_eq!(i32::zero(), 0);
    }

    #[test]
    fn test_rr() {
        let x = RR::new(3);
        let y = RR::new(7);
        assert_eq!(x + y, RR::new(10));
        assert_eq!(x - y, RR::new(MOD - 4));
        assert_eq!(y - x, RR::new(4));
        assert_eq!(x * y, RR::new(21));
    }
}
