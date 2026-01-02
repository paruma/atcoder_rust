use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use pos_f64::*;")]
pub mod pos_f64 {
    use std::iter::Sum;
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

    #[derive(Clone, Copy, PartialEq, PartialOrd)]
    pub struct PosF64 {
        pub x: f64,
        pub y: f64,
    }

    impl PosF64 {
        pub fn new(x: f64, y: f64) -> PosF64 {
            PosF64 { x, y }
        }

        pub fn scala_mul(self, rhs: f64) -> PosF64 {
            self * rhs
        }

        pub fn inner_product(self, rhs: Self) -> f64 {
            self.x * rhs.x + self.y * rhs.y
        }

        pub fn outer_product(self, rhs: Self) -> f64 {
            self.x * rhs.y - self.y * rhs.x
        }

        pub fn norm_square(self) -> f64 {
            self.inner_product(self)
        }

        pub fn norm(self) -> f64 {
            self.norm_square().sqrt()
        }

        pub fn dist(self, rhs: Self) -> f64 {
            (self - rhs).norm()
        }

        pub fn dist_square(self, rhs: Self) -> f64 {
            (self - rhs).norm_square()
        }

        pub fn rotate(self, theta: f64) -> PosF64 {
            let (s, c) = theta.sin_cos();
            PosF64::new(self.x * c - self.y * s, self.x * s + self.y * c)
        }

        pub fn normalize(self) -> PosF64 {
            self / self.norm()
        }
    }

    impl Add for PosF64 {
        type Output = PosF64;

        fn add(self, rhs: Self) -> Self::Output {
            PosF64::new(self.x + rhs.x, self.y + rhs.y)
        }
    }

    impl Sub for PosF64 {
        type Output = PosF64;

        fn sub(self, rhs: Self) -> Self::Output {
            PosF64::new(self.x - rhs.x, self.y - rhs.y)
        }
    }

    impl Neg for PosF64 {
        type Output = Self;

        fn neg(self) -> Self::Output {
            PosF64::new(-self.x, -self.y)
        }
    }

    impl Sum for PosF64 {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(PosF64::new(0.0, 0.0), |acc, x| acc + x)
        }
    }

    impl<'a> Sum<&'a PosF64> for PosF64 {
        fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
            iter.fold(PosF64::new(0.0, 0.0), |a, b| a + *b)
        }
    }

    impl num_traits::Zero for PosF64 {
        fn zero() -> Self {
            PosF64::new(0.0, 0.0)
        }

        fn is_zero(&self) -> bool {
            self.x.is_zero() && self.y.is_zero()
        }
    }

    impl AddAssign for PosF64 {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }

    impl SubAssign for PosF64 {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }

    impl Mul<f64> for PosF64 {
        type Output = PosF64;

        fn mul(self, rhs: f64) -> Self::Output {
            PosF64::new(self.x * rhs, self.y * rhs)
        }
    }

    impl MulAssign<f64> for PosF64 {
        fn mul_assign(&mut self, rhs: f64) {
            *self = *self * rhs;
        }
    }

    impl Div<f64> for PosF64 {
        type Output = PosF64;

        fn div(self, rhs: f64) -> Self::Output {
            PosF64::new(self.x / rhs, self.y / rhs)
        }
    }

    impl DivAssign<f64> for PosF64 {
        fn div_assign(&mut self, rhs: f64) {
            *self = *self / rhs;
        }
    }

    use std::fmt::{Debug, Error, Formatter};

    impl Debug for PosF64 {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            f.write_fmt(format_args!("({}, {})", self.x, self.y))?;
            Ok(())
        }
    }

    use proconio::source::{Readable, Source};
    use std::io::BufRead;

    impl Readable for PosF64 {
        type Output = PosF64;
        fn read<R: BufRead, S: Source<R>>(source: &mut S) -> PosF64 {
            let x = f64::read(source);
            let y = f64::read(source);
            PosF64::new(x, y)
        }
    }
}

#[cfg(test)]
mod tests_pos_f64 {
    use super::pos_f64::*;
    use num::Zero;
    use proconio::source::Readable;
    use proconio::source::once::OnceSource;

    #[test]
    fn test_read() {
        let mut source = OnceSource::from("1.5 2.5");
        let p = PosF64::read(&mut source);
        assert_eq!(p, PosF64::new(1.5, 2.5));
    }

    #[test]
    fn test_pos_add() {
        let p1: PosF64 = PosF64::new(2.0, 3.0);
        let p2: PosF64 = PosF64::new(4.0, 7.0);
        assert_eq!(p1 + p2, PosF64::new(6.0, 10.0));
    }

    #[test]
    fn test_pos_sub() {
        let p1: PosF64 = PosF64::new(2.0, 3.0);
        let p2: PosF64 = PosF64::new(4.0, 7.0);
        assert_eq!(p2 - p1, PosF64::new(2.0, 4.0));
    }

    #[test]
    fn test_pos_neg() {
        let p1: PosF64 = PosF64::new(2.0, -3.0);
        assert_eq!(-p1, PosF64::new(-2.0, 3.0));
    }

    #[test]
    fn test_pos_zero() {
        let zero: PosF64 = PosF64::new(0.0, 0.0);
        assert_eq!(PosF64::zero(), zero);
        assert!(zero.is_zero());
    }

    #[test]
    fn test_pos_add_assign() {
        let p1: PosF64 = PosF64::new(2.0, 3.0);
        let mut p2: PosF64 = PosF64::new(4.0, 7.0);
        p2 += p1;
        assert_eq!(p2.x, 6.0);
        assert_eq!(p2.y, 10.0);
    }

    #[test]
    fn test_pos_sub_assign() {
        let p1: PosF64 = PosF64::new(2.0, 3.0);
        let mut p2: PosF64 = PosF64::new(4.0, 7.0);
        p2 -= p1;
        assert_eq!(p2.x, 2.0);
        assert_eq!(p2.y, 4.0);
    }

    #[test]
    fn test_sum() {
        let ps = [
            PosF64::new(1.0, 2.0),
            PosF64::new(3.0, 4.0),
            PosF64::new(5.0, 6.0),
        ];
        assert_eq!(ps.iter().copied().sum::<PosF64>(), PosF64::new(9.0, 12.0));
        assert_eq!(ps.iter().sum::<PosF64>(), PosF64::new(9.0, 12.0));

        let empty: [PosF64; 0] = [];
        assert_eq!(empty.iter().copied().sum::<PosF64>(), PosF64::new(0.0, 0.0));
        assert_eq!(empty.iter().sum::<PosF64>(), PosF64::new(0.0, 0.0));
    }

    #[test]
    fn test_pos_scala_mul() {
        let p: PosF64 = PosF64::new(2.0, 3.0);
        assert_eq!(p.scala_mul(4.0), PosF64::new(8.0, 12.0));
        assert_eq!(p * 4.0, PosF64::new(8.0, 12.0));

        let mut p2 = p;
        p2 *= 4.0;
        assert_eq!(p2, PosF64::new(8.0, 12.0));
    }

    #[test]
    fn test_pos_div() {
        let p = PosF64::new(8.0, 12.0);
        assert_eq!(p / 4.0, PosF64::new(2.0, 3.0));

        let mut p2 = p;
        p2 /= 4.0;
        assert_eq!(p2, PosF64::new(2.0, 3.0));
    }

    #[test]
    fn test_pos_inner_product() {
        let p1: PosF64 = PosF64::new(2.0, 3.0);
        let p2: PosF64 = PosF64::new(4.0, 5.0);
        assert_eq!(p1.inner_product(p2), 23.0);
    }

    #[test]
    fn test_pos_outer_product() {
        let p1: PosF64 = PosF64::new(2.0, 3.0);
        let p2: PosF64 = PosF64::new(4.0, 5.0);
        assert_eq!(p1.outer_product(p2), -2.0);
    }

    #[test]
    fn test_pos_norm() {
        let p: PosF64 = PosF64::new(3.0, 4.0);
        assert_eq!(p.norm_square(), 25.0);
        assert_eq!(p.norm(), 5.0);
    }

    #[test]
    fn test_pos_dist() {
        let p1 = PosF64::new(1.0, 2.0);
        let p2 = PosF64::new(4.0, 6.0);
        assert_eq!(p1.dist(p2), 5.0);
        assert_eq!(p1.dist_square(p2), 25.0);
    }

    #[test]
    fn test_pos_rotate() {
        let p = PosF64::new(1.0, 0.0);
        let p_rotated = p.rotate(std::f64::consts::FRAC_PI_2);
        assert!((p_rotated.x - 0.0).abs() < 1e-10);
        assert!((p_rotated.y - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_pos_normalize() {
        let p = PosF64::new(3.0, 4.0);
        let p_normed = p.normalize();
        assert!((p_normed.norm() - 1.0).abs() < 1e-10);
        assert_eq!(p_normed.x, 0.6);
        assert_eq!(p_normed.y, 0.8);
    }

    #[test]
    fn test_pos_debug() {
        let p = PosF64::new(2.5, 3.5);
        assert_eq!(format!("{:?}", p), "(2.5, 3.5)");
    }
}
