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

// Super Redidue Ring
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
struct SRR<const MOD: i64> {
    rep: i64,
}
impl<const MOD: i64> SRR<MOD> {
    fn new(rep: i64) -> SRR<MOD> {
        SRR {
            rep: rep.rem_euclid(MOD),
        }
    }
}

impl<const MOD: i64> Ring for SRR<MOD> {
    fn zero() -> Self {
        SRR { rep: 0 }
    }

    fn one() -> Self {
        SRR { rep: 1 }
    }

    fn mul(&self, other: &Self) -> Self {
        SRR::new(self.rep * other.rep)
    }

    fn add(&self, other: &Self) -> Self {
        SRR::new(self.rep + other.rep)
    }

    fn neg(&self) -> Self {
        SRR::new(-self.rep)
    }
}

impl<const MOD: i64> std::ops::Add for SRR<MOD> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Ring::add(&self, &rhs)
    }
}

impl<const MOD: i64> std::ops::Sub for SRR<MOD> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Ring::sub(&self, &rhs)
    }
}

impl<const MOD: i64> std::ops::Mul for SRR<MOD> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Ring::mul(&self, &rhs)
    }
}

impl<const MOD: i64> std::ops::Neg for SRR<MOD> {
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
        const MOD: i64 = 1_000_000_007;
        type RR = SRR<MOD>;
        let x = RR::new(3);
        let y = RR::new(7);
        assert_eq!(x + y, RR::new(10));
        assert_eq!(x - y, RR::new(MOD - 4));
        assert_eq!(x * y, RR::new(21));
    }
}
