use cargo_snippet::snippet;

#[snippet(prefix = "use pos3d_i128::*;")]
#[allow(clippy::module_inception)]
pub mod pos3d_i128 {
    use std::io::BufRead;
    use std::iter::Sum;
    use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct Pos3DI128 {
        pub x: i128,
        pub y: i128,
        pub z: i128,
    }

    impl Pos3DI128 {
        pub fn new(x: i128, y: i128, z: i128) -> Pos3DI128 {
            Pos3DI128 { x, y, z }
        }

        pub fn scalar_mul(self, rhs: i128) -> Pos3DI128 {
            Pos3DI128::new(self.x * rhs, self.y * rhs, self.z * rhs)
        }

        pub fn inner_product(self, rhs: Self) -> i128 {
            self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
        }

        pub fn outer_product(self, rhs: Self) -> Pos3DI128 {
            Pos3DI128::new(
                self.y * rhs.z - self.z * rhs.y,
                self.z * rhs.x - self.x * rhs.z,
                self.x * rhs.y - self.y * rhs.x,
            )
        }

        pub fn norm_square(self) -> i128 {
            self.inner_product(self)
        }

        pub fn l1_norm(self) -> i128 {
            self.x.abs() + self.y.abs() + self.z.abs()
        }

        pub fn linf_norm(self) -> i128 {
            self.x.abs().max(self.y.abs()).max(self.z.abs())
        }

        pub fn dist_square(self, rhs: Self) -> i128 {
            (self - rhs).norm_square()
        }

        pub fn l1_dist(self, rhs: Self) -> i128 {
            (self - rhs).l1_norm()
        }

        pub fn linf_dist(self, rhs: Self) -> i128 {
            (self - rhs).linf_norm()
        }

        // ベクトルを正規化する（最大公約数で割る）。
        // (0,0,0)の場合は(0,0,0)を返す。
        //
        // 計算量: O(log(min(|x|, |y|, |z|)))
        pub fn normalize(self) -> Pos3DI128 {
            if self.x == 0 && self.y == 0 && self.z == 0 {
                return self;
            }
            let g = num::integer::gcd(self.x.abs(), num::integer::gcd(self.y.abs(), self.z.abs()));
            Pos3DI128::new(self.x / g, self.y / g, self.z / g)
        }

        pub fn around6_pos_iter(self) -> impl Iterator<Item = Pos3DI128> {
            DIR6_LIST.iter().copied().map(move |d| self + d)
        }

        pub fn around26_pos_iter(self) -> impl Iterator<Item = Pos3DI128> {
            DIR26_LIST.iter().copied().map(move |d| self + d)
        }
    }

    impl Add for Pos3DI128 {
        type Output = Pos3DI128;

        fn add(self, rhs: Self) -> Self::Output {
            Pos3DI128::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
        }
    }

    impl Sub for Pos3DI128 {
        type Output = Pos3DI128;

        fn sub(self, rhs: Self) -> Self::Output {
            Pos3DI128::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
        }
    }

    impl Neg for Pos3DI128 {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Pos3DI128::new(-self.x, -self.y, -self.z)
        }
    }

    impl Sum for Pos3DI128 {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Pos3DI128::new(0, 0, 0), |acc, x| acc + x)
        }
    }

    impl<'a> Sum<&'a Pos3DI128> for Pos3DI128 {
        fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
            iter.fold(Pos3DI128::new(0, 0, 0), |a, b| a + *b)
        }
    }

    impl num_traits::Zero for Pos3DI128 {
        fn zero() -> Self {
            Pos3DI128::new(0, 0, 0)
        }

        fn is_zero(&self) -> bool {
            self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
        }
    }

    impl AddAssign for Pos3DI128 {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }

    impl SubAssign for Pos3DI128 {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }

    impl Mul<i128> for Pos3DI128 {
        type Output = Pos3DI128;

        fn mul(self, rhs: i128) -> Self::Output {
            self.scalar_mul(rhs)
        }
    }

    impl MulAssign<i128> for Pos3DI128 {
        fn mul_assign(&mut self, rhs: i128) {
            *self = *self * rhs
        }
    }

    use std::fmt::{Debug, Error, Formatter};

    impl Debug for Pos3DI128 {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            f.write_fmt(format_args!("({}, {}, {})", self.x, self.y, self.z))?;
            Ok(())
        }
    }

    use proconio::source::{Readable, Source};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum PosXYZ {}
    impl Readable for PosXYZ {
        type Output = Pos3DI128;
        fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Pos3DI128 {
            let x = i128::read(source);
            let y = i128::read(source);
            let z = i128::read(source);
            Pos3DI128::new(x, y, z)
        }
    }

    pub const DIR6_LIST: [Pos3DI128; 6] = [
        Pos3DI128 { x: 1, y: 0, z: 0 },
        Pos3DI128 { x: -1, y: 0, z: 0 },
        Pos3DI128 { x: 0, y: 1, z: 0 },
        Pos3DI128 { x: 0, y: -1, z: 0 },
        Pos3DI128 { x: 0, y: 0, z: 1 },
        Pos3DI128 { x: 0, y: 0, z: -1 },
    ];

    #[rustfmt::skip]
    pub const DIR26_LIST: [Pos3DI128; 26] = [
        Pos3DI128 { x: 1, y: 0, z: 0 },
        Pos3DI128 { x: -1, y: 0, z: 0 },
        Pos3DI128 { x: 0, y: 1, z: 0 },
        Pos3DI128 { x: 0, y: -1, z: 0 },
        Pos3DI128 { x: 0, y: 0, z: 1 },
        Pos3DI128 { x: 0, y: 0, z: -1 },
        Pos3DI128 { x: 1, y: 1, z: 0 },
        Pos3DI128 { x: 1, y: -1, z: 0 },
        Pos3DI128 { x: -1, y: 1, z: 0 },
        Pos3DI128 { x: -1, y: -1, z: 0 },
        Pos3DI128 { x: 1, y: 0, z: 1 },
        Pos3DI128 { x: 1, y: 0, z: -1 },
        Pos3DI128 { x: -1, y: 0, z: 1 },
        Pos3DI128 { x: -1, y: 0, z: -1 },
        Pos3DI128 { x: 0, y: 1, z: 1 },
        Pos3DI128 { x: 0, y: 1, z: -1 },
        Pos3DI128 { x: 0, y: -1, z: 1 },
        Pos3DI128 { x: 0, y: -1, z: -1 },
        Pos3DI128 { x: 1, y: 1, z: 1 },
        Pos3DI128 { x: 1, y: 1, z: -1 },
        Pos3DI128 { x: 1, y: -1, z: 1 },
        Pos3DI128 { x: 1, y: -1, z: -1 },
        Pos3DI128 { x: -1, y: 1, z: 1 },
        Pos3DI128 { x: -1, y: 1, z: -1 },
        Pos3DI128 { x: -1, y: -1, z: 1 },
        Pos3DI128 { x: -1, y: -1, z: -1 },
    ];
}

#[cfg(test)]
mod tests_pos3d_i128 {
    use super::pos3d_i128::*;
    use num::Zero;
    use proconio::source::Readable;
    use proconio::source::once::OnceSource;
    use std::collections::HashSet;

    #[test]
    fn test_read() {
        let mut source = OnceSource::from("1 2 3");
        let p = PosXYZ::read(&mut source);
        assert_eq!(p, Pos3DI128::new(1, 2, 3));
    }

    #[test]
    fn test_pos3d_i128_add() {
        let p1 = Pos3DI128::new(1, 2, 3);
        let p2 = Pos3DI128::new(4, 5, 6);
        assert_eq!(p1 + p2, Pos3DI128::new(5, 7, 9));
    }

    #[test]
    fn test_pos3d_i128_sub() {
        let p1 = Pos3DI128::new(1, 2, 3);
        let p2 = Pos3DI128::new(4, 5, 6);
        assert_eq!(p2 - p1, Pos3DI128::new(3, 3, 3));
    }

    #[test]
    fn test_pos3d_i128_neg() {
        let p1 = Pos3DI128::new(1, -2, 3);
        assert_eq!(-p1, Pos3DI128::new(-1, 2, -3));
    }

    #[test]
    fn test_pos3d_i128_zero() {
        let zero = Pos3DI128::new(0, 0, 0);
        assert_eq!(Pos3DI128::zero(), zero);
        assert!(zero.is_zero());
    }

    #[test]
    fn test_pos3d_i128_scalar_mul() {
        let p = Pos3DI128::new(1, 2, 3);
        assert_eq!(p * 2, Pos3DI128::new(2, 4, 6));
    }

    #[test]
    fn test_pos3d_i128_inner_product() {
        let p1 = Pos3DI128::new(1, 2, 3);
        let p2 = Pos3DI128::new(4, 5, 6);
        assert_eq!(p1.inner_product(p2), 4 + 10 + 18);
    }

    #[test]
    fn test_pos3d_i128_outer_product() {
        let p1 = Pos3DI128::new(1, 0, 0);
        let p2 = Pos3DI128::new(0, 1, 0);
        assert_eq!(p1.outer_product(p2), Pos3DI128::new(0, 0, 1));
    }

    #[test]
    fn test_pos3d_i128_normalize() {
        assert_eq!(Pos3DI128::new(2, 4, 6).normalize(), Pos3DI128::new(1, 2, 3));
        assert_eq!(Pos3DI128::new(0, 0, 0).normalize(), Pos3DI128::new(0, 0, 0));
    }

    #[test]
    fn test_pos3d_i128_norms() {
        let p = Pos3DI128::new(2, -3, 6);
        assert_eq!(p.l1_norm(), 11);
        assert_eq!(p.linf_norm(), 6);
        assert_eq!(p.norm_square(), 49);
    }

    #[test]
    fn test_pos3d_i128_dists() {
        let p1 = Pos3DI128::new(1, 2, 3);
        let p2 = Pos3DI128::new(4, -2, 3);
        // diff: (3, -4, 0)
        assert_eq!(p1.l1_dist(p2), 7);
        assert_eq!(p1.linf_dist(p2), 4);
        assert_eq!(p1.dist_square(p2), 25);
    }

    #[test]
    fn test_around6_pos_iter() {
        let p = Pos3DI128::new(0, 0, 0);
        let actual = p.around6_pos_iter().collect::<HashSet<_>>();
        assert_eq!(actual.len(), 6);
        assert!(actual.contains(&Pos3DI128::new(1, 0, 0)));
        assert!(actual.contains(&Pos3DI128::new(-1, 0, 0)));
        assert!(actual.contains(&Pos3DI128::new(0, 1, 0)));
        assert!(actual.contains(&Pos3DI128::new(0, -1, 0)));
        assert!(actual.contains(&Pos3DI128::new(0, 0, 1)));
        assert!(actual.contains(&Pos3DI128::new(0, 0, -1)));
    }

    #[test]
    fn test_around26_pos_iter() {
        let p = Pos3DI128::new(0, 0, 0);
        let actual = p.around26_pos_iter().collect::<HashSet<_>>();
        assert_eq!(actual.len(), 26);
        assert!(actual.contains(&Pos3DI128::new(1, 1, 1)));
        assert!(!actual.contains(&Pos3DI128::new(0, 0, 0)));
    }

    #[test]
    fn test_sum() {
        let ps = [
            Pos3DI128::new(1, 2, 3),
            Pos3DI128::new(4, 5, 6),
            Pos3DI128::new(7, 8, 9),
        ];
        assert_eq!(
            ps.iter().copied().sum::<Pos3DI128>(),
            Pos3DI128::new(12, 15, 18)
        );
        assert_eq!(ps.iter().sum::<Pos3DI128>(), Pos3DI128::new(12, 15, 18));

        let empty: [Pos3DI128; 0] = [];
        assert_eq!(
            empty.iter().copied().sum::<Pos3DI128>(),
            Pos3DI128::new(0, 0, 0)
        );
        assert_eq!(empty.iter().sum::<Pos3DI128>(), Pos3DI128::new(0, 0, 0));
    }

    #[test]
    fn test_pos3d_i128_assign_ops() {
        let mut p = Pos3DI128::new(1, 2, 3);
        p += Pos3DI128::new(1, 1, 1);
        assert_eq!(p, Pos3DI128::new(2, 3, 4));

        p -= Pos3DI128::new(2, 2, 2);
        assert_eq!(p, Pos3DI128::new(0, 1, 2));

        p *= 10;
        assert_eq!(p, Pos3DI128::new(0, 10, 20));
    }

    #[test]
    fn test_pos3d_i128_debug() {
        let p = Pos3DI128::new(1, 2, 3);
        assert_eq!(format!("{:?}", p), "(1, 2, 3)");
    }
}
