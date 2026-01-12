use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use pos3d_f64::*;")]
pub mod pos3d_f64 {
    use std::iter::Sum;
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

    #[derive(Clone, Copy, PartialEq, PartialOrd)]
    pub struct Pos3dF64 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    impl Pos3dF64 {
        pub fn new(x: f64, y: f64, z: f64) -> Pos3dF64 {
            Pos3dF64 { x, y, z }
        }

        pub fn scala_mul(self, rhs: f64) -> Pos3dF64 {
            self * rhs
        }

        pub fn inner_product(self, rhs: Self) -> f64 {
            self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
        }

        pub fn outer_product(self, rhs: Self) -> Pos3dF64 {
            Pos3dF64::new(
                self.y * rhs.z - self.z * rhs.y,
                self.z * rhs.x - self.x * rhs.z,
                self.x * rhs.y - self.y * rhs.x,
            )
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

        pub fn normalize(self) -> Pos3dF64 {
            self / self.norm()
        }
    }

    impl Add for Pos3dF64 {
        type Output = Pos3dF64;

        fn add(self, rhs: Self) -> Self::Output {
            Pos3dF64::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
        }
    }

    impl Sub for Pos3dF64 {
        type Output = Pos3dF64;

        fn sub(self, rhs: Self) -> Self::Output {
            Pos3dF64::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
        }
    }

    impl Neg for Pos3dF64 {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Pos3dF64::new(-self.x, -self.y, -self.z)
        }
    }

    impl Sum for Pos3dF64 {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Pos3dF64::new(0.0, 0.0, 0.0), |acc, x| acc + x)
        }
    }

    impl<'a> Sum<&'a Pos3dF64> for Pos3dF64 {
        fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
            iter.fold(Pos3dF64::new(0.0, 0.0, 0.0), |a, b| a + *b)
        }
    }

    impl num_traits::Zero for Pos3dF64 {
        fn zero() -> Self {
            Pos3dF64::new(0.0, 0.0, 0.0)
        }

        fn is_zero(&self) -> bool {
            self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
        }
    }

    impl AddAssign for Pos3dF64 {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }

    impl SubAssign for Pos3dF64 {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }

    impl Mul<f64> for Pos3dF64 {
        type Output = Pos3dF64;

        fn mul(self, rhs: f64) -> Self::Output {
            Pos3dF64::new(self.x * rhs, self.y * rhs, self.z * rhs)
        }
    }

    impl MulAssign<f64> for Pos3dF64 {
        fn mul_assign(&mut self, rhs: f64) {
            *self = *self * rhs;
        }
    }

    impl Div<f64> for Pos3dF64 {
        type Output = Pos3dF64;

        fn div(self, rhs: f64) -> Self::Output {
            Pos3dF64::new(self.x / rhs, self.y / rhs, self.z / rhs)
        }
    }

    impl DivAssign<f64> for Pos3dF64 {
        fn div_assign(&mut self, rhs: f64) {
            *self = *self / rhs;
        }
    }

    use std::fmt::{Debug, Error, Formatter};

    impl Debug for Pos3dF64 {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            f.write_fmt(format_args!("({}, {}, {})", self.x, self.y, self.z))?;
            Ok(())
        }
    }

    use proconio::source::{Readable, Source};
    use std::io::BufRead;

    impl Readable for Pos3dF64 {
        type Output = Pos3dF64;
        fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Pos3dF64 {
            let x = f64::read(source);
            let y = f64::read(source);
            let z = f64::read(source);
            Pos3dF64::new(x, y, z)
        }
    }
}

#[cfg(test)]
mod tests_pos3d_f64 {
    use super::pos3d_f64::*;
    use num::Zero;
    use proconio::source::Readable;
    use proconio::source::once::OnceSource;

    #[test]
    fn test_read() {
        let mut source = OnceSource::from("1.5 2.5 3.5");
        let p = Pos3dF64::read(&mut source);
        assert_eq!(p, Pos3dF64::new(1.5, 2.5, 3.5));
    }

    #[test]
    fn test_pos3d_add() {
        let p1: Pos3dF64 = Pos3dF64::new(2.0, 3.0, 4.0);
        let p2: Pos3dF64 = Pos3dF64::new(4.0, 7.0, 10.0);
        assert_eq!(p1 + p2, Pos3dF64::new(6.0, 10.0, 14.0));
    }

    #[test]
    fn test_pos3d_sub() {
        let p1: Pos3dF64 = Pos3dF64::new(2.0, 3.0, 4.0);
        let p2: Pos3dF64 = Pos3dF64::new(4.0, 7.0, 10.0);
        assert_eq!(p2 - p1, Pos3dF64::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_pos3d_neg() {
        let p1: Pos3dF64 = Pos3dF64::new(2.0, -3.0, 4.0);
        assert_eq!(-p1, Pos3dF64::new(-2.0, 3.0, -4.0));
    }

    #[test]
    fn test_pos3d_zero() {
        let zero: Pos3dF64 = Pos3dF64::new(0.0, 0.0, 0.0);
        assert_eq!(Pos3dF64::zero(), zero);
        assert!(zero.is_zero());
    }

    #[test]
    fn test_pos3d_add_assign() {
        let p1: Pos3dF64 = Pos3dF64::new(2.0, 3.0, 4.0);
        let mut p2: Pos3dF64 = Pos3dF64::new(4.0, 7.0, 10.0);
        p2 += p1;
        assert_eq!(p2.x, 6.0);
        assert_eq!(p2.y, 10.0);
        assert_eq!(p2.z, 14.0);
    }

    #[test]
    fn test_pos3d_sub_assign() {
        let p1: Pos3dF64 = Pos3dF64::new(2.0, 3.0, 4.0);
        let mut p2: Pos3dF64 = Pos3dF64::new(4.0, 7.0, 10.0);
        p2 -= p1;
        assert_eq!(p2.x, 2.0);
        assert_eq!(p2.y, 4.0);
        assert_eq!(p2.z, 6.0);
    }

    #[test]
    fn test_sum() {
        let ps = [
            Pos3dF64::new(1.0, 2.0, 3.0),
            Pos3dF64::new(3.0, 4.0, 5.0),
            Pos3dF64::new(5.0, 6.0, 7.0),
        ];
        assert_eq!(ps.iter().copied().sum::<Pos3dF64>(), Pos3dF64::new(9.0, 12.0, 15.0));
        assert_eq!(ps.iter().sum::<Pos3dF64>(), Pos3dF64::new(9.0, 12.0, 15.0));

        let empty: [Pos3dF64; 0] = [];
        assert_eq!(empty.iter().copied().sum::<Pos3dF64>(), Pos3dF64::new(0.0, 0.0, 0.0));
        assert_eq!(empty.iter().sum::<Pos3dF64>(), Pos3dF64::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_pos3d_scala_mul() {
        let p: Pos3dF64 = Pos3dF64::new(2.0, 3.0, 4.0);
        assert_eq!(p.scala_mul(4.0), Pos3dF64::new(8.0, 12.0, 16.0));
        assert_eq!(p * 4.0, Pos3dF64::new(8.0, 12.0, 16.0));

        let mut p2 = p;
        p2 *= 4.0;
        assert_eq!(p2, Pos3dF64::new(8.0, 12.0, 16.0));
    }

    #[test]
    fn test_pos3d_div() {
        let p = Pos3dF64::new(8.0, 12.0, 16.0);
        assert_eq!(p / 4.0, Pos3dF64::new(2.0, 3.0, 4.0));

        let mut p2 = p;
        p2 /= 4.0;
        assert_eq!(p2, Pos3dF64::new(2.0, 3.0, 4.0));
    }

    #[test]
    fn test_pos3d_inner_product() {
        let p1: Pos3dF64 = Pos3dF64::new(2.0, 3.0, 4.0);
        let p2: Pos3dF64 = Pos3dF64::new(4.0, 5.0, 6.0);
        assert_eq!(p1.inner_product(p2), 2.0 * 4.0 + 3.0 * 5.0 + 4.0 * 6.0);
    }

    #[test]
    fn test_pos3d_outer_product() {
        let p1: Pos3dF64 = Pos3dF64::new(1.0, 0.0, 0.0);
        let p2: Pos3dF64 = Pos3dF64::new(0.0, 1.0, 0.0);
        assert_eq!(p1.outer_product(p2), Pos3dF64::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_pos3d_norm() {
        let p: Pos3dF64 = Pos3dF64::new(3.0, 4.0, 12.0);
        assert_eq!(p.norm_square(), 9.0 + 16.0 + 144.0);
        assert_eq!(p.norm(), 13.0);
    }

    #[test]
    fn test_pos3d_dist() {
        let p1 = Pos3dF64::new(1.0, 2.0, 3.0);
        let p2 = Pos3dF64::new(4.0, 6.0, 15.0);
        // diff: (3, 4, 12)
        assert_eq!(p1.dist(p2), 13.0);
        assert_eq!(p1.dist_square(p2), 169.0);
    }

    #[test]
    fn test_pos3d_normalize() {
        let p = Pos3dF64::new(3.0, 4.0, 12.0);
        let p_normed = p.normalize();
        assert!((p_normed.norm() - 1.0).abs() < 1e-10);
        assert_eq!(p_normed.x, 3.0 / 13.0);
        assert_eq!(p_normed.y, 4.0 / 13.0);
        assert_eq!(p_normed.z, 12.0 / 13.0);
    }

    #[test]
    fn test_pos3d_debug() {
        let p = Pos3dF64::new(2.5, 3.5, 4.5);
        assert_eq!(format!("{:?}", p), "(2.5, 3.5, 4.5)");
    }
}
