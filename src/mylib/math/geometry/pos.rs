use cargo_snippet::snippet;

use crate::math::algebra::ab_group::ab_group::AbGroup;
use crate::data_structure::ix::Ix;

#[snippet(prefix = "use pos::*;")]
#[allow(clippy::module_inception)]
pub mod pos {
    use std::io::BufRead;
    use std::iter::Sum;
    use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Pos {
        pub x: i64,
        pub y: i64,
    }

    impl Pos {
        pub fn new(x: i64, y: i64) -> Pos {
            Pos { x, y }
        }

        pub fn scala_mul(self, rhs: i64) -> Pos {
            Pos::new(self.x * rhs, self.y * rhs)
        }

        pub fn inner_product(self, rhs: Self) -> i64 {
            self.x * rhs.x + self.y * rhs.y
        }

        pub fn outer_product(self, rhs: Self) -> i64 {
            self.x * rhs.y - self.y * rhs.x
        }

        pub fn norm_square(self) -> i64 {
            self.inner_product(self)
        }

        pub fn l1_norm(self) -> i64 {
            self.x.abs() + self.y.abs()
        }

        pub fn linf_norm(self) -> i64 {
            self.x.abs().max(self.y.abs())
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
        // (0,0)の場合は(0,0)を返す。
        //
        // 計算量: O(log(min(|x|, |y|)))
        pub fn normalize(self) -> Pos {
            if self.x == 0 && self.y == 0 {
                return self;
            }
            let g = num::integer::gcd(self.x.abs(), self.y.abs());
            Pos::new(self.x / g, self.y / g)
        }

        // 原点を中心に反時計回りに90度回転
        pub fn rotate90(self) -> Pos {
            Pos::new(-self.y, self.x)
        }

        // 原点を中心に時計回りに90度回転
        pub fn rotate270(self) -> Pos {
            Pos::new(self.y, -self.x)
        }

        /// グリッドの幅 `width` を指定して、座標 `(x, y)` を 1次元インデックス `y * width + x` に変換する。
        pub fn to_index_1d(self, width: usize) -> usize {
            assert!(
                self.x >= 0 && self.y >= 0,
                "Pos::to_index_1d: x と y は 0 以上である必要があります。pos: ({}, {})",
                self.x,
                self.y
            );
            assert!(
                (self.x as usize) < width,
                "Pos::to_index_1d: x は width 未満である必要があります。x: {}, width: {}",
                self.x,
                width
            );
            (self.y as usize) * width + (self.x as usize)
        }

        /// 1次元インデックスとグリッドの幅 `width` から、座標 `(x, y)` を復元する。
        pub fn from_index_1d(index: usize, width: usize) -> Pos {
            Pos::new((index % width) as i64, (index / width) as i64)
        }

        pub fn around4_pos_iter(self) -> impl Iterator<Item = Pos> {
            DIR4_LIST.iter().copied().map(move |d| self + d)
        }

        pub fn around8_pos_iter(self) -> impl Iterator<Item = Pos> {
            DIR8_LIST.iter().copied().map(move |d| self + d)
        }
    }

    impl Add for Pos {
        type Output = Pos;

        fn add(self, rhs: Self) -> Self::Output {
            Pos::new(self.x + rhs.x, self.y + rhs.y)
        }
    }

    impl Sub for Pos {
        type Output = Pos;

        fn sub(self, rhs: Self) -> Self::Output {
            Pos::new(self.x - rhs.x, self.y - rhs.y)
        }
    }

    impl Neg for Pos {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Pos::new(-self.x, -self.y)
        }
    }

    impl Sum for Pos {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Pos::new(0, 0), |acc, x| acc + x)
        }
    }

    impl<'a> Sum<&'a Pos> for Pos {
        fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
            iter.fold(Pos::new(0, 0), |a, b| a + *b)
        }
    }

    impl num_traits::Zero for Pos {
        fn zero() -> Self {
            Pos::new(0, 0)
        }

        fn is_zero(&self) -> bool {
            self.x.is_zero() && self.y.is_zero()
        }
    }

    impl AddAssign for Pos {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }

    impl SubAssign for Pos {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }

    impl Mul<i64> for Pos {
        type Output = Pos;

        fn mul(self, rhs: i64) -> Self::Output {
            Pos::new(self.x * rhs, self.y * rhs)
        }
    }

    impl MulAssign<i64> for Pos {
        fn mul_assign(&mut self, rhs: i64) {
            *self = *self * rhs
        }
    }

    use std::fmt::{Debug, Error, Formatter};

    impl Debug for Pos {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            f.write_fmt(format_args!("({}, {})", self.x, self.y))?;
            Ok(())
        }
    }

    use proconio::source::{Readable, Source};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum PosXY {}
    impl Readable for PosXY {
        type Output = Pos;
        fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Pos {
            let x = i64::read(source);
            let y = i64::read(source);
            Pos::new(x, y)
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum PosYX {}
    impl Readable for PosYX {
        type Output = Pos;
        fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Pos {
            let y = i64::read(source);
            let x = i64::read(source);
            Pos::new(x, y)
        }
    }

    /// 1-indexed で与えられた座標(YX)
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum PosYX1 {}
    impl Readable for PosYX1 {
        type Output = Pos;
        fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Pos {
            let y = i64::read(source) - 1;
            let x = i64::read(source) - 1;
            Pos::new(x, y)
        }
    }

    pub const DIR8_LIST: [Pos; 8] = [
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 1 },
        Pos { x: 1, y: 0 },
        Pos { x: 1, y: -1 },
        Pos { x: 0, y: -1 },
        Pos { x: -1, y: -1 },
        Pos { x: -1, y: 0 },
        Pos { x: -1, y: 1 },
    ];

    pub const DIR4_LIST: [Pos; 4] = [
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 0 },
        Pos { x: 0, y: -1 },
        Pos { x: -1, y: 0 },
    ];
}

#[snippet(prefix = "use vec_vec_at::*;")]
pub mod vec_vec_at {
    use std::ops::{Index, IndexMut};

    use super::pos::*;
    use easy_ext::ext;

    #[ext(ExtVecVec)]
    impl<T> Vec<Vec<T>> {
        pub fn width(&self) -> usize {
            if self.is_empty() {
                // 0 扱いにしておく
                0
            } else {
                self[0].len()
            }
        }

        pub fn height(&self) -> usize {
            self.len()
        }

        pub fn is_within(&self, pos: Pos) -> bool {
            (0..self.width() as i64).contains(&pos.x) && (0..self.height() as i64).contains(&pos.y)
        }
    }
    impl<T> Index<Pos> for Vec<Vec<T>> {
        type Output = T;

        fn index(&self, index: Pos) -> &Self::Output {
            if cfg!(debug_assertions) && !self.is_within(index) {
                panic!(
                    "index out of bounds: the size (w, h) is ({}, {}) but the index (x, y) is ({}, {})",
                    self.width(),
                    self.height(),
                    index.x,
                    index.y
                );
            }

            &self[index.y as usize][index.x as usize]
        }
    }

    impl<T> IndexMut<Pos> for Vec<Vec<T>> {
        fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
            if cfg!(debug_assertions) && !self.is_within(index) {
                panic!(
                    "index out of bounds: the size (w, h) is ({}, {}) but the index (x, y) is ({}, {})",
                    self.width(),
                    self.height(),
                    index.x,
                    index.y
                );
            }

            &mut self[index.y as usize][index.x as usize]
        }
    }
}

#[snippet(prefix = "use pos_ix::*;")]
pub mod pos_ix {
    use super::Ix;
    use super::pos::Pos;

    impl Ix for Pos {
        fn range((min, max): (Self, Self)) -> impl Iterator<Item = Self> {
            (min.y..=max.y).flat_map(move |y| (min.x..=max.x).map(move |x| Pos::new(x, y)))
        }

        fn range_size((min, max): (Self, Self)) -> usize {
            if min.x > max.x || min.y > max.y {
                0
            } else {
                ((max.x - min.x + 1) * (max.y - min.y + 1)) as usize
            }
        }

        fn to_index((min, max): (Self, Self), i: Self) -> usize {
            if !Self::in_range((min, max), i) {
                panic!("index out of bounds: {:?} is not in {:?}", i, (min, max));
            }
            let width = (max.x - min.x + 1) as usize;
            let dy = (i.y - min.y) as usize;
            let dx = (i.x - min.x) as usize;
            dy * width + dx
        }

        fn from_index((min, max): (Self, Self), index: usize) -> Self {
            if index >= Self::range_size((min, max)) {
                panic!("index out of range: {} for bounds {:?}", index, (min, max));
            }
            let width = (max.x - min.x + 1) as usize;
            let dy = (index / width) as i64;
            let dx = (index % width) as i64;
            Pos::new(min.x + dx, min.y + dy)
        }

        fn in_range((min, max): (Self, Self), i: Self) -> bool {
            min.x <= i.x && i.x <= max.x && min.y <= i.y && i.y <= max.y
        }
    }
}

#[snippet(prefix = "use pos_ab_group::*;")]
pub mod pos_ab_group {
    use super::pos::Pos;
    use super::AbGroup;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct PosAbGroup;

    impl AbGroup for PosAbGroup {
        type S = Pos;
        fn zero() -> Self::S {
            Pos::new(0, 0)
        }
        fn add(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
        fn neg(a: &Self::S) -> Self::S {
            -(*a)
        }
        fn sub(a: &Self::S, b: &Self::S) -> Self::S {
            *a - *b
        }
    }
}

#[cfg(test)]
mod tests_pos {

    use std::collections::HashSet;

    use num::Zero;
    use proconio::source::Readable;
    use proconio::source::once::OnceSource;

    use super::pos::*;
    use super::pos_ab_group::*;
    use crate::math::algebra::ab_group::ab_group::AbGroup;

    #[test]
    fn test_pos_ab_group() {
        let p1 = Pos::new(1, 2);
        let p2 = Pos::new(3, 4);
        assert_eq!(PosAbGroup::zero(), Pos::new(0, 0));
        assert_eq!(PosAbGroup::add(&p1, &p2), Pos::new(4, 6));
        assert_eq!(PosAbGroup::neg(&p1), Pos::new(-1, -2));
        assert_eq!(PosAbGroup::sub(&p1, &p2), Pos::new(-2, -2));
    }

    #[test]
    fn test_read() {
        let mut source = OnceSource::from("1 2");
        let p = PosXY::read(&mut source);
        assert_eq!(p, Pos::new(1, 2));

        let mut source = OnceSource::from("3 4");
        let p = PosYX::read(&mut source);
        assert_eq!(p, Pos::new(4, 3));

        let mut source = OnceSource::from("5 6");
        let p = PosYX1::read(&mut source);
        assert_eq!(p, Pos::new(5, 4));
    }

    #[test]
    fn test_pos_add() {
        let p1: Pos = Pos::new(2, 3);
        let p2: Pos = Pos::new(4, 7);
        assert_eq!(p1 + p2, Pos::new(6, 10));
    }

    #[test]
    fn test_pos_sub() {
        let p1: Pos = Pos::new(2, 3);
        let p2: Pos = Pos::new(4, 7);
        assert_eq!(p2 - p1, Pos::new(2, 4));
    }

    #[test]
    fn test_pos_neg() {
        let p1: Pos = Pos::new(2, -3);
        assert_eq!(-p1, Pos::new(-2, 3));
    }

    #[test]
    fn test_pos_zero() {
        let zero: Pos = Pos::new(0, 0);
        assert_eq!(Pos::zero(), zero);
        assert!(zero.is_zero());
    }

    #[test]
    fn test_pos_add_assign() {
        let p1: Pos = Pos::new(2, 3);
        let mut p2: Pos = Pos::new(4, 7);
        p2 += p1;
        assert_eq!(p2.x, 6);
        assert_eq!(p2.y, 10);
    }

    #[test]
    fn test_pos_sub_assign() {
        let p1: Pos = Pos::new(2, 3);
        let mut p2: Pos = Pos::new(4, 7);
        p2 -= p1;
        assert_eq!(p2.x, 2);
        assert_eq!(p2.y, 4);
    }

    #[test]
    fn test_sum() {
        let ps = [Pos::new(1, 2), Pos::new(3, 4), Pos::new(5, 6)];
        assert_eq!(ps.iter().copied().sum::<Pos>(), Pos::new(9, 12));
        assert_eq!(ps.iter().sum::<Pos>(), Pos::new(9, 12));

        let empty: [Pos; 0] = [];
        assert_eq!(empty.iter().copied().sum::<Pos>(), Pos::new(0, 0));
        assert_eq!(empty.iter().sum::<Pos>(), Pos::new(0, 0));
    }

    #[test]
    fn test_pos_scala_mul() {
        let p: Pos = Pos::new(2, 3);
        assert_eq!(p.scala_mul(4), Pos::new(8, 12));
        assert_eq!(p * 4, Pos::new(8, 12));

        let mut p2 = Pos::new(2, 3);
        p2 *= 4;
        assert_eq!(p2, Pos::new(8, 12));
    }

    #[test]
    fn test_pos_inner_product() {
        let p1: Pos = Pos::new(2, 3);
        let p2: Pos = Pos::new(4, 5);
        assert_eq!(p1.inner_product(p2), 23);
    }

    #[test]
    fn test_pos_outer_product() {
        let p1: Pos = Pos::new(2, 3);
        let p2: Pos = Pos::new(4, 5);
        // 2*5 - 3*4 = 10 - 12 = -2
        assert_eq!(p1.outer_product(p2), -2);
    }

    #[test]
    fn test_pos_normalize() {
        assert_eq!(Pos::new(6, 9).normalize(), Pos::new(2, 3));
        assert_eq!(Pos::new(-6, 9).normalize(), Pos::new(-2, 3));
        assert_eq!(Pos::new(0, 5).normalize(), Pos::new(0, 1));
        assert_eq!(Pos::new(0, 0).normalize(), Pos::new(0, 0));
    }

    #[test]
    fn test_pos_rotate() {
        let p = Pos::new(2, 3);
        assert_eq!(p.rotate90(), Pos::new(-3, 2));
        assert_eq!(p.rotate270(), Pos::new(3, -2));

        let p2 = Pos::new(1, 0);
        assert_eq!(p2.rotate90(), Pos::new(0, 1));
        assert_eq!(p2.rotate270(), Pos::new(0, -1));
    }

    #[test]
    fn test_pos_index_1d() {
        let width = 10;
        let p = Pos::new(2, 3); // y=3, x=2 -> 3*10 + 2 = 32
        assert_eq!(p.to_index_1d(width), 32);
        assert_eq!(Pos::from_index_1d(32, width), p);

        let p_zero = Pos::new(0, 0);
        assert_eq!(p_zero.to_index_1d(width), 0);
        assert_eq!(Pos::from_index_1d(0, width), p_zero);
    }

    #[test]
    #[should_panic(expected = "x と y は 0 以上である必要があります")]
    fn test_pos_index_1d_panic_negative() {
        Pos::new(-1, 0).to_index_1d(10);
    }

    #[test]
    #[should_panic(expected = "x は width 未満である必要があります")]
    fn test_pos_index_1d_panic_width() {
        Pos::new(10, 0).to_index_1d(10);
    }

    #[test]
    fn test_pos_norm_square() {
        let p: Pos = Pos::new(2, 3);
        assert_eq!(p.norm_square(), 13);
    }

    #[test]
    fn test_pos_norms() {
        let p = Pos::new(2, -3);
        assert_eq!(p.l1_norm(), 5);
        assert_eq!(p.linf_norm(), 3);
    }

    #[test]
    fn test_pos_dists() {
        let p1 = Pos::new(1, 2);
        let p2 = Pos::new(4, -2);
        assert_eq!(p1.l1_dist(p2), 7);
        assert_eq!(p1.linf_dist(p2), 4);
        assert_eq!(p1.dist_square(p2), 25);
    }

    #[test]
    fn test_around4_pos_iter() {
        let p: Pos = Pos::new(2, 3);
        let actual = p.around4_pos_iter().collect::<HashSet<Pos>>();
        let expected = HashSet::from([
            Pos::new(2, 2),
            Pos::new(3, 3),
            Pos::new(2, 4),
            Pos::new(1, 3),
        ]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_around8_pos_iter() {
        let p: Pos = Pos::new(2, 3);
        let actual = p.around8_pos_iter().collect::<HashSet<Pos>>();
        let expected = HashSet::from([
            Pos::new(2, 2),
            Pos::new(3, 2),
            Pos::new(3, 3),
            Pos::new(3, 4),
            Pos::new(2, 4),
            Pos::new(1, 4),
            Pos::new(1, 4),
            Pos::new(1, 3),
            Pos::new(1, 2),
        ]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_pos_debug() {
        let p = Pos::new(2, 3);
        assert_eq!(format!("{:?}", p), "(2, 3)");
    }
}

#[cfg(test)]
mod tests_vec_vec_at {
    use super::pos::*;
    use super::vec_vec_at::ExtVecVec;

    #[test]
    fn test_vec_vec_at() {
        let mut xss = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(xss[Pos::new(2, 1)], 6);
        xss[Pos::new(2, 1)] = 60;

        assert_eq!(xss, vec![vec![1, 2, 3], vec![4, 5, 60]])
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    #[allow(clippy::useless_vec)]
    fn test_vec_vec_at_panic_index() {
        let xss = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let _ = xss[Pos::new(3, 1)];
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    #[allow(clippy::useless_vec)]
    fn test_vec_vec_at_panic_index_mut() {
        let mut xss = vec![vec![1, 2, 3], vec![4, 5, 6]];
        xss[Pos::new(2, 2)] = 100;
    }

    #[test]
    fn test_vec_vec_at_empty() {
        let xss: Vec<Vec<i32>> = vec![];
        assert_eq!(xss.width(), 0);
        assert_eq!(xss.height(), 0);
    }
}

#[cfg(test)]
mod tests_pos_ix {
    use super::Ix;
    use super::pos::*;

    #[test]
    fn test_pos_ix() {
        let min = Pos::new(1, 1);
        let max = Pos::new(3, 2);
        // x: 1..=3, y: 1..=2
        // y=1: (1,1), (2,1), (3,1)
        // y=2: (1,2), (2,2), (3,2)

        let bounds = (min, max);
        assert_eq!(Pos::range_size(bounds), 6);

        let vec: Vec<Pos> = Pos::range(bounds).collect();
        assert_eq!(
            vec,
            vec![
                Pos::new(1, 1),
                Pos::new(2, 1),
                Pos::new(3, 1),
                Pos::new(1, 2),
                Pos::new(2, 2),
                Pos::new(3, 2)
            ]
        );

        assert_eq!(Pos::to_index(bounds, Pos::new(1, 1)), 0);
        assert_eq!(Pos::to_index(bounds, Pos::new(2, 1)), 1);
        assert_eq!(Pos::to_index(bounds, Pos::new(3, 1)), 2);
        assert_eq!(Pos::to_index(bounds, Pos::new(1, 2)), 3);
        assert_eq!(Pos::to_index(bounds, Pos::new(3, 2)), 5);

        assert!(Pos::in_range(bounds, Pos::new(2, 1)));
        assert!(!Pos::in_range(bounds, Pos::new(0, 1)));
        assert!(!Pos::in_range(bounds, Pos::new(1, 3)));
    }
}
