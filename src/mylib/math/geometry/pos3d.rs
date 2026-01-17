use cargo_snippet::snippet;

use crate::data_structure::ix::Ix;

#[snippet(prefix = "use pos3d::*;")]
#[allow(clippy::module_inception)]
pub mod pos3d {
    use std::io::BufRead;
    use std::iter::Sum;
    use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Pos3d {
        pub x: i64,
        pub y: i64,
        pub z: i64,
    }

    impl Pos3d {
        pub fn new(x: i64, y: i64, z: i64) -> Pos3d {
            Pos3d { x, y, z }
        }

        pub fn scala_mul(self, rhs: i64) -> Pos3d {
            Pos3d::new(self.x * rhs, self.y * rhs, self.z * rhs)
        }

        pub fn inner_product(self, rhs: Self) -> i64 {
            self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
        }

        pub fn outer_product(self, rhs: Self) -> Pos3d {
            Pos3d::new(
                self.y * rhs.z - self.z * rhs.y,
                self.z * rhs.x - self.x * rhs.z,
                self.x * rhs.y - self.y * rhs.x,
            )
        }

        pub fn norm_square(self) -> i64 {
            self.inner_product(self)
        }

        pub fn l1_norm(self) -> i64 {
            self.x.abs() + self.y.abs() + self.z.abs()
        }

        pub fn linf_norm(self) -> i64 {
            self.x.abs().max(self.y.abs()).max(self.z.abs())
        }

        pub fn dist_square(self, rhs: Self) -> i64 {
            (self - rhs).norm_square()
        }

        pub fn l1_dist(self, rhs: Self) -> i64 {
            (self - rhs).l1_norm()
        }

        pub fn linf_dist(self, rhs: Self) -> i64 {
            (self - rhs).linf_norm()
        }

        // ベクトルを正規化する（最大公約数で割る）。
        // (0,0,0)の場合は(0,0,0)を返す。
        //
        // 計算量: O(log(min(|x|, |y|, |z|)))
        pub fn normalize(self) -> Pos3d {
            if self.x == 0 && self.y == 0 && self.z == 0 {
                return self;
            }
            let g = num::integer::gcd(self.x.abs(), num::integer::gcd(self.y.abs(), self.z.abs()));
            Pos3d::new(self.x / g, self.y / g, self.z / g)
        }

        /// グリッドの幅 `width` と高さ `height` を指定して、座標 `(x, y, z)` を 1次元インデックス `z * (width * height) + y * width + x` に変換する。
        pub fn to_index_1d(self, width: usize, height: usize) -> usize {
            assert!(
                self.x >= 0 && self.y >= 0 && self.z >= 0,
                "Pos3d::to_index_1d: x, y, z は 0 以上である必要があります。pos: ({}, {}, {})",
                self.x,
                self.y,
                self.z
            );
            assert!(
                (self.x as usize) < width,
                "Pos3d::to_index_1d: x は width 未満である必要があります。x: {}, width: {}",
                self.x,
                width
            );
            assert!(
                (self.y as usize) < height,
                "Pos3d::to_index_1d: y は height 未満である必要があります。y: {}, height: {}",
                self.y,
                height
            );
            (self.z as usize) * width * height + (self.y as usize) * width + (self.x as usize)
        }

        /// 1次元インデックスとグリッドの幅 `width`, 高さ `height` から、座標 `(x, y, z)` を復元する。
        pub fn from_index_1d(index: usize, width: usize, height: usize) -> Pos3d {
            let z = index / (width * height);
            let rem = index % (width * height);
            let y = rem / width;
            let x = rem % width;
            Pos3d::new(x as i64, y as i64, z as i64)
        }

        pub fn around6_pos_iter(self) -> impl Iterator<Item = Pos3d> {
            DIR6_LIST.iter().copied().map(move |d| self + d)
        }

        pub fn around26_pos_iter(self) -> impl Iterator<Item = Pos3d> {
            DIR26_LIST.iter().copied().map(move |d| self + d)
        }
    }

    impl Add for Pos3d {
        type Output = Pos3d;

        fn add(self, rhs: Self) -> Self::Output {
            Pos3d::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
        }
    }

    impl Sub for Pos3d {
        type Output = Pos3d;

        fn sub(self, rhs: Self) -> Self::Output {
            Pos3d::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
        }
    }

    impl Neg for Pos3d {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Pos3d::new(-self.x, -self.y, -self.z)
        }
    }

    impl Sum for Pos3d {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Pos3d::new(0, 0, 0), |acc, x| acc + x)
        }
    }

    impl<'a> Sum<&'a Pos3d> for Pos3d {
        fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
            iter.fold(Pos3d::new(0, 0, 0), |a, b| a + *b)
        }
    }

    impl num_traits::Zero for Pos3d {
        fn zero() -> Self {
            Pos3d::new(0, 0, 0)
        }

        fn is_zero(&self) -> bool {
            self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
        }
    }

    impl AddAssign for Pos3d {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }

    impl SubAssign for Pos3d {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }

    impl Mul<i64> for Pos3d {
        type Output = Pos3d;

        fn mul(self, rhs: i64) -> Self::Output {
            self.scala_mul(rhs)
        }
    }

    impl MulAssign<i64> for Pos3d {
        fn mul_assign(&mut self, rhs: i64) {
            *self = *self * rhs
        }
    }

    use std::fmt::{Debug, Error, Formatter};

    impl Debug for Pos3d {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            f.write_fmt(format_args!("({}, {}, {})", self.x, self.y, self.z))?;
            Ok(())
        }
    }

    use proconio::source::{Readable, Source};

    pub enum PosXYZ {}
    impl Readable for PosXYZ {
        type Output = Pos3d;
        fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Pos3d {
            let x = i64::read(source);
            let y = i64::read(source);
            let z = i64::read(source);
            Pos3d::new(x, y, z)
        }
    }

    pub const DIR6_LIST: [Pos3d; 6] = [
        Pos3d { x: 1, y: 0, z: 0 },
        Pos3d { x: -1, y: 0, z: 0 },
        Pos3d { x: 0, y: 1, z: 0 },
        Pos3d { x: 0, y: -1, z: 0 },
        Pos3d { x: 0, y: 0, z: 1 },
        Pos3d { x: 0, y: 0, z: -1 },
    ];

    #[rustfmt::skip]
    pub const DIR26_LIST: [Pos3d; 26] = [
        Pos3d { x: 1, y: 0, z: 0 },
        Pos3d { x: -1, y: 0, z: 0 },
        Pos3d { x: 0, y: 1, z: 0 },
        Pos3d { x: 0, y: -1, z: 0 },
        Pos3d { x: 0, y: 0, z: 1 },
        Pos3d { x: 0, y: 0, z: -1 },
        Pos3d { x: 1, y: 1, z: 0 },
        Pos3d { x: 1, y: -1, z: 0 },
        Pos3d { x: -1, y: 1, z: 0 },
        Pos3d { x: -1, y: -1, z: 0 },
        Pos3d { x: 1, y: 0, z: 1 },
        Pos3d { x: 1, y: 0, z: -1 },
        Pos3d { x: -1, y: 0, z: 1 },
        Pos3d { x: -1, y: 0, z: -1 },
        Pos3d { x: 0, y: 1, z: 1 },
        Pos3d { x: 0, y: 1, z: -1 },
        Pos3d { x: 0, y: -1, z: 1 },
        Pos3d { x: 0, y: -1, z: -1 },
        Pos3d { x: 1, y: 1, z: 1 },
        Pos3d { x: 1, y: 1, z: -1 },
        Pos3d { x: 1, y: -1, z: 1 },
        Pos3d { x: 1, y: -1, z: -1 },
        Pos3d { x: -1, y: 1, z: 1 },
        Pos3d { x: -1, y: 1, z: -1 },
        Pos3d { x: -1, y: -1, z: 1 },
        Pos3d { x: -1, y: -1, z: -1 },
    ];
}

#[snippet(prefix = "use vec_vec_vec_at::*;")]
pub mod vec_vec_vec_at {
    use std::ops::{Index, IndexMut};

    use super::pos3d::*;
    use easy_ext::ext;

    #[ext(ExtVecVecVec)]
    impl<T> Vec<Vec<Vec<T>>> {
        pub fn width(&self) -> usize {
            if self.is_empty() || self[0].is_empty() {
                0
            } else {
                self[0][0].len()
            }
        }

        pub fn height(&self) -> usize {
            if self.is_empty() { 0 } else { self[0].len() }
        }

        pub fn depth(&self) -> usize {
            self.len()
        }

        pub fn is_within(&self, pos: Pos3d) -> bool {
            (0..self.width() as i64).contains(&pos.x)
                && (0..self.height() as i64).contains(&pos.y)
                && (0..self.depth() as i64).contains(&pos.z)
        }
    }

    impl<T> Index<Pos3d> for Vec<Vec<Vec<T>>> {
        type Output = T;

        fn index(&self, index: Pos3d) -> &Self::Output {
            if cfg!(debug_assertions) && !self.is_within(index) {
                panic!(
                    "index out of bounds: the size (w, h, d) is ({}, {}, {}) but the index (x, y, z) is ({}, {}, {})",
                    self.width(),
                    self.height(),
                    self.depth(),
                    index.x,
                    index.y,
                    index.z
                );
            }

            &self[index.z as usize][index.y as usize][index.x as usize]
        }
    }

    impl<T> IndexMut<Pos3d> for Vec<Vec<Vec<T>>> {
        fn index_mut(&mut self, index: Pos3d) -> &mut Self::Output {
            if cfg!(debug_assertions) && !self.is_within(index) {
                panic!(
                    "index out of bounds: the size (w, h, d) is ({}, {}, {}) but the index (x, y, z) is ({}, {}, {})",
                    self.width(),
                    self.height(),
                    self.depth(),
                    index.x,
                    index.y,
                    index.z
                );
            }

            &mut self[index.z as usize][index.y as usize][index.x as usize]
        }
    }
}

#[snippet(prefix = "use pos3d_ix::*;")]
pub mod pos3d_ix {
    use super::Ix;
    use super::pos3d::Pos3d;

    impl Ix for Pos3d {
        fn range((min, max): (Self, Self)) -> impl Iterator<Item = Self> {
            (min.z..=max.z).flat_map(move |z| {
                (min.y..=max.y).flat_map(move |y| (min.x..=max.x).map(move |x| Pos3d::new(x, y, z)))
            })
        }

        fn range_size((min, max): (Self, Self)) -> usize {
            if min.x > max.x || min.y > max.y || min.z > max.z {
                0
            } else {
                ((max.x - min.x + 1) * (max.y - min.y + 1) * (max.z - min.z + 1)) as usize
            }
        }

        fn to_index((min, max): (Self, Self), i: Self) -> usize {
            if !Self::in_range((min, max), i) {
                panic!("index out of bounds: {:?} is not in {:?}", i, (min, max));
            }
            let width = (max.x - min.x + 1) as usize;
            let height = (max.y - min.y + 1) as usize;
            let dz = (i.z - min.z) as usize;
            let dy = (i.y - min.y) as usize;
            let dx = (i.x - min.x) as usize;
            dz * (width * height) + dy * width + dx
        }

        fn from_index((min, max): (Self, Self), index: usize) -> Self {
            if index >= Self::range_size((min, max)) {
                panic!("index out of range: {} for bounds {:?}", index, (min, max));
            }
            let width = (max.x - min.x + 1) as usize;
            let height = (max.y - min.y + 1) as usize;
            let dz = (index / (width * height)) as i64;
            let rem = index % (width * height);
            let dy = (rem / width) as i64;
            let dx = (rem % width) as i64;
            Pos3d::new(min.x + dx, min.y + dy, min.z + dz)
        }

        fn in_range((min, max): (Self, Self), i: Self) -> bool {
            min.x <= i.x
                && i.x <= max.x
                && min.y <= i.y
                && i.y <= max.y
                && min.z <= i.z
                && i.z <= max.z
        }
    }
}

#[cfg(test)]
mod tests_pos3d {
    use super::pos3d::*;
    use num::Zero;
    use proconio::source::Readable;
    use proconio::source::once::OnceSource;
    use std::collections::HashSet;

    #[test]
    fn test_read() {
        let mut source = OnceSource::from("1 2 3");
        let p = PosXYZ::read(&mut source);
        assert_eq!(p, Pos3d::new(1, 2, 3));
    }

    #[test]
    fn test_pos3d_add() {
        let p1 = Pos3d::new(1, 2, 3);
        let p2 = Pos3d::new(4, 5, 6);
        assert_eq!(p1 + p2, Pos3d::new(5, 7, 9));
    }

    #[test]
    fn test_pos3d_sub() {
        let p1 = Pos3d::new(1, 2, 3);
        let p2 = Pos3d::new(4, 5, 6);
        assert_eq!(p2 - p1, Pos3d::new(3, 3, 3));
    }

    #[test]
    fn test_pos3d_neg() {
        let p1 = Pos3d::new(1, -2, 3);
        assert_eq!(-p1, Pos3d::new(-1, 2, -3));
    }

    #[test]
    fn test_pos3d_zero() {
        let zero = Pos3d::new(0, 0, 0);
        assert_eq!(Pos3d::zero(), zero);
        assert!(zero.is_zero());
    }

    #[test]
    fn test_pos3d_scala_mul() {
        let p = Pos3d::new(1, 2, 3);
        assert_eq!(p * 2, Pos3d::new(2, 4, 6));
    }

    #[test]
    fn test_pos3d_inner_product() {
        let p1 = Pos3d::new(1, 2, 3);
        let p2 = Pos3d::new(4, 5, 6);
        assert_eq!(p1.inner_product(p2), 4 + 10 + 18);
    }

    #[test]
    fn test_pos3d_outer_product() {
        let p1 = Pos3d::new(1, 0, 0);
        let p2 = Pos3d::new(0, 1, 0);
        assert_eq!(p1.outer_product(p2), Pos3d::new(0, 0, 1));
    }

    #[test]
    fn test_pos3d_normalize() {
        assert_eq!(Pos3d::new(2, 4, 6).normalize(), Pos3d::new(1, 2, 3));
        assert_eq!(Pos3d::new(0, 0, 0).normalize(), Pos3d::new(0, 0, 0));
    }

    #[test]
    fn test_pos3d_norms() {
        let p = Pos3d::new(2, -3, 6);
        assert_eq!(p.l1_norm(), 11);
        assert_eq!(p.linf_norm(), 6);
        assert_eq!(p.norm_square(), 49);
    }

    #[test]
    fn test_pos3d_dists() {
        let p1 = Pos3d::new(1, 2, 3);
        let p2 = Pos3d::new(4, -2, 3);
        // diff: (3, -4, 0)
        assert_eq!(p1.l1_dist(p2), 7);
        assert_eq!(p1.linf_dist(p2), 4);
        assert_eq!(p1.dist_square(p2), 25);
    }

    #[test]
    fn test_around6_pos_iter() {
        let p = Pos3d::new(0, 0, 0);
        let actual = p.around6_pos_iter().collect::<HashSet<_>>();
        assert_eq!(actual.len(), 6);
        assert!(actual.contains(&Pos3d::new(1, 0, 0)));
        assert!(actual.contains(&Pos3d::new(-1, 0, 0)));
        assert!(actual.contains(&Pos3d::new(0, 1, 0)));
        assert!(actual.contains(&Pos3d::new(0, -1, 0)));
        assert!(actual.contains(&Pos3d::new(0, 0, 1)));
        assert!(actual.contains(&Pos3d::new(0, 0, -1)));
    }

    #[test]
    fn test_around26_pos_iter() {
        let p = Pos3d::new(0, 0, 0);
        let actual = p.around26_pos_iter().collect::<HashSet<_>>();
        assert_eq!(actual.len(), 26);
        assert!(actual.contains(&Pos3d::new(1, 1, 1)));
        assert!(!actual.contains(&Pos3d::new(0, 0, 0)));
    }

    #[test]
    fn test_sum() {
        let ps = [
            Pos3d::new(1, 2, 3),
            Pos3d::new(4, 5, 6),
            Pos3d::new(7, 8, 9),
        ];
        assert_eq!(ps.iter().copied().sum::<Pos3d>(), Pos3d::new(12, 15, 18));
        assert_eq!(ps.iter().sum::<Pos3d>(), Pos3d::new(12, 15, 18));

        let empty: [Pos3d; 0] = [];
        assert_eq!(empty.iter().copied().sum::<Pos3d>(), Pos3d::new(0, 0, 0));
        assert_eq!(empty.iter().sum::<Pos3d>(), Pos3d::new(0, 0, 0));
    }

    #[test]
    fn test_pos3d_assign_ops() {
        let mut p = Pos3d::new(1, 2, 3);
        p += Pos3d::new(1, 1, 1);
        assert_eq!(p, Pos3d::new(2, 3, 4));

        p -= Pos3d::new(2, 2, 2);
        assert_eq!(p, Pos3d::new(0, 1, 2));

        p *= 10;
        assert_eq!(p, Pos3d::new(0, 10, 20));
    }

    #[test]
    fn test_pos3d_debug() {
        let p = Pos3d::new(1, 2, 3);
        assert_eq!(format!("{:?}", p), "(1, 2, 3)");
    }

    #[test]
    fn test_pos3d_index_1d() {
        let width = 10;
        let height = 5;
        let p = Pos3d::new(2, 3, 1); // z=1, y=3, x=2 -> 1*(10*5) + 3*10 + 2 = 50 + 30 + 2 = 82
        assert_eq!(p.to_index_1d(width, height), 82);
        assert_eq!(Pos3d::from_index_1d(82, width, height), p);
    }

    #[test]
    #[should_panic(expected = "x, y, z は 0 以上である必要があります")]
    fn test_pos3d_index_1d_panic_negative() {
        Pos3d::new(-1, 0, 0).to_index_1d(10, 5);
    }

    #[test]
    #[should_panic(expected = "x は width 未満である必要があります")]
    fn test_pos3d_index_1d_panic_width() {
        Pos3d::new(10, 0, 0).to_index_1d(10, 5);
    }

    #[test]
    #[should_panic(expected = "y は height 未満である必要があります")]
    fn test_pos3d_index_1d_panic_height() {
        Pos3d::new(0, 5, 0).to_index_1d(10, 5);
    }
}

#[cfg(test)]
mod tests_vec_vec_vec_at {
    use super::pos3d::*;
    use super::vec_vec_vec_at::ExtVecVecVec;

    #[test]
    #[allow(clippy::useless_vec)]
    fn test_vec_vec_vec_at() {
        let mut xsss = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]];
        assert_eq!(xsss.width(), 2);
        assert_eq!(xsss.height(), 2);
        assert_eq!(xsss.depth(), 2);
        // z=0: [[1, 2], [3, 4]]
        // z=1: [[5, 6], [7, 8]]
        assert_eq!(xsss[Pos3d::new(1, 0, 1)], 6);
        xsss[Pos3d::new(1, 0, 1)] = 60;
        assert_eq!(xsss[1][0][1], 60);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    #[allow(clippy::useless_vec)]
    fn test_vec_vec_vec_at_panic_z() {
        let xsss = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]];
        let _ = xsss[Pos3d::new(0, 0, 2)];
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    #[allow(clippy::useless_vec)]
    fn test_vec_vec_vec_at_panic_y() {
        let xsss = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]];
        let _ = xsss[Pos3d::new(0, 2, 0)];
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    #[allow(clippy::useless_vec)]
    fn test_vec_vec_vec_at_panic_x() {
        let xsss = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]];
        let _ = xsss[Pos3d::new(2, 0, 0)];
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    #[allow(clippy::useless_vec)]
    fn test_vec_vec_vec_at_panic_neg() {
        let xsss = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]];
        let _ = xsss[Pos3d::new(-1, 0, 0)];
    }

    #[test]
    fn test_vec_vec_vec_at_empty() {
        let xsss: Vec<Vec<Vec<i32>>> = vec![];
        assert_eq!(xsss.width(), 0);
        assert_eq!(xsss.height(), 0);
        assert_eq!(xsss.depth(), 0);

        let xsss2: Vec<Vec<Vec<i32>>> = vec![vec![]];
        assert_eq!(xsss2.width(), 0);
        assert_eq!(xsss2.height(), 0);
    }
}

#[cfg(test)]
mod tests_pos3d_ix {
    use super::Ix;
    use super::pos3d::*;

    #[test]
    fn test_pos3d_ix() {
        let min = Pos3d::new(1, 1, 1);
        let max = Pos3d::new(2, 2, 2);
        let bounds = (min, max);
        assert_eq!(Pos3d::range_size(bounds), 8);

        let vec: Vec<_> = Pos3d::range(bounds).collect();
        assert_eq!(vec.len(), 8);
        assert_eq!(vec[0], Pos3d::new(1, 1, 1));
        assert_eq!(vec[1], Pos3d::new(2, 1, 1));
        assert_eq!(vec[2], Pos3d::new(1, 2, 1));
        assert_eq!(vec[3], Pos3d::new(2, 2, 1));
        assert_eq!(vec[4], Pos3d::new(1, 1, 2));
        assert_eq!(vec[7], Pos3d::new(2, 2, 2));

        assert_eq!(Pos3d::to_index(bounds, Pos3d::new(1, 1, 1)), 0);
        assert_eq!(Pos3d::to_index(bounds, Pos3d::new(2, 2, 2)), 7);
        assert_eq!(Pos3d::from_index(bounds, 0), Pos3d::new(1, 1, 1));
        assert_eq!(Pos3d::from_index(bounds, 7), Pos3d::new(2, 2, 2));

        assert!(Pos3d::in_range(bounds, Pos3d::new(1, 1, 1)));
        assert!(Pos3d::in_range(bounds, Pos3d::new(2, 2, 2)));
        assert!(!Pos3d::in_range(bounds, Pos3d::new(0, 1, 1)));
        assert!(!Pos3d::in_range(bounds, Pos3d::new(3, 1, 1)));
        assert!(!Pos3d::in_range(bounds, Pos3d::new(1, 0, 1)));
        assert!(!Pos3d::in_range(bounds, Pos3d::new(1, 3, 1)));
        assert!(!Pos3d::in_range(bounds, Pos3d::new(1, 1, 0)));
        assert!(!Pos3d::in_range(bounds, Pos3d::new(1, 1, 3)));
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_pos3d_ix_panic() {
        let min = Pos3d::new(1, 1, 1);
        let max = Pos3d::new(2, 2, 2);
        let bounds = (min, max);
        Pos3d::to_index(bounds, Pos3d::new(0, 0, 0));
    }

    #[test]
    #[should_panic(expected = "index out of range")]
    fn test_pos3d_ix_from_index_panic() {
        let min = Pos3d::new(1, 1, 1);
        let max = Pos3d::new(2, 2, 2);
        let bounds = (min, max);
        Pos3d::from_index(bounds, 100);
    }

    #[test]
    fn test_pos3d_ix_empty() {
        let min = Pos3d::new(2, 2, 2);
        let max = Pos3d::new(1, 1, 1);
        let bounds = (min, max);
        assert_eq!(Pos3d::range_size(bounds), 0);
        assert!(Pos3d::range(bounds).next().is_none());
    }
}
