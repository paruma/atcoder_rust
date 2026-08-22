use cargo_snippet::snippet;

use crate::data_structure::ix::Ix;
use crate::math::algebra::ab_group::ab_group::AbGroup;

#[snippet(prefix = "use pos_i128::*;")]
#[allow(clippy::module_inception)]
pub mod pos_i128 {
    use std::io::BufRead;
    use std::iter::Sum;
    use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct PosI128 {
        pub x: i128,
        pub y: i128,
    }

    impl PosI128 {
        pub fn new(x: i128, y: i128) -> PosI128 {
            PosI128 { x, y }
        }

        pub fn new_from_usize(x: usize, y: usize) -> PosI128 {
            PosI128::new(x as i128, y as i128)
        }

        pub fn scalar_mul(self, rhs: i128) -> PosI128 {
            PosI128::new(self.x * rhs, self.y * rhs)
        }

        pub fn inner_product(self, rhs: Self) -> i128 {
            self.x * rhs.x + self.y * rhs.y
        }

        pub fn outer_product(self, rhs: Self) -> i128 {
            self.x * rhs.y - self.y * rhs.x
        }

        pub fn norm_square(self) -> i128 {
            self.inner_product(self)
        }

        pub fn l1_norm(self) -> i128 {
            self.x.abs() + self.y.abs()
        }

        pub fn linf_norm(self) -> i128 {
            self.x.abs().max(self.y.abs())
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

        /// 向きが同じであれば同一視する正規化 (方向ベクトル)
        /// 最大公約数で割り、符号はそのまま残す。
        /// (0,0) の場合は (0,0) を返す。
        /// 計算量: O(log(min(|x|, |y|)))
        pub fn normalize_direction(self) -> PosI128 {
            if self.x == 0 && self.y == 0 {
                return self;
            }
            let g = num::integer::gcd(self.x.abs(), self.y.abs());
            PosI128::new(self.x / g, self.y / g)
        }

        /// 平行であれば同一視する正規化（直線の傾きを表す）
        /// 最大公約数で割り、最初の非零成分 (x が優先) が正になるように符号を統一する。
        /// (0,0) の場合は (0,0) を返す。
        /// 計算量: O(log(min(|x|, |y|)))
        pub fn normalize_slope(self) -> PosI128 {
            if self.x == 0 && self.y == 0 {
                return self;
            }
            // 最初の非零成分 (x 優先) が正になるように符号を反転
            let p = if self.x < 0 || (self.x == 0 && self.y < 0) {
                -self
            } else {
                self
            };
            let g = num::integer::gcd(p.x, p.y);
            PosI128::new(p.x / g, p.y / g)
        }

        // 原点を中心に反時計回りに90度回転
        pub fn rotate90(self) -> PosI128 {
            PosI128::new(-self.y, self.x)
        }

        // 原点を中心に時計回りに90度回転
        pub fn rotate270(self) -> PosI128 {
            PosI128::new(self.y, -self.x)
        }

        /// グリッドの幅 `width` を指定して、座標 `(x, y)` を 1次元インデックス `y * width + x` に変換する。
        pub fn to_index_1d(self, width: usize) -> usize {
            assert!(
                self.x >= 0 && self.y >= 0,
                "PosI128::to_index_1d: x と y は 0 以上である必要があります。pos: ({}, {})",
                self.x,
                self.y
            );
            assert!(
                (self.x as usize) < width,
                "PosI128::to_index_1d: x は width 未満である必要があります。x: {}, width: {}",
                self.x,
                width
            );
            (self.y as usize) * width + (self.x as usize)
        }

        /// 1次元インデックスとグリッドの幅 `width` から、座標 `(x, y)` を復元する。
        pub fn from_index_1d(index: usize, width: usize) -> PosI128 {
            PosI128::new((index % width) as i128, (index / width) as i128)
        }

        pub fn around4_pos_iter(self) -> impl Iterator<Item = PosI128> {
            DIR4_LIST.iter().copied().map(move |d| self + d)
        }

        pub fn around8_pos_iter(self) -> impl Iterator<Item = PosI128> {
            DIR8_LIST.iter().copied().map(move |d| self + d)
        }
    }

    impl Add for PosI128 {
        type Output = PosI128;

        fn add(self, rhs: Self) -> Self::Output {
            PosI128::new(self.x + rhs.x, self.y + rhs.y)
        }
    }

    impl Sub for PosI128 {
        type Output = PosI128;

        fn sub(self, rhs: Self) -> Self::Output {
            PosI128::new(self.x - rhs.x, self.y - rhs.y)
        }
    }

    impl Neg for PosI128 {
        type Output = Self;

        fn neg(self) -> Self::Output {
            PosI128::new(-self.x, -self.y)
        }
    }

    impl Sum for PosI128 {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(PosI128::new(0, 0), |acc, x| acc + x)
        }
    }

    impl<'a> Sum<&'a PosI128> for PosI128 {
        fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
            iter.fold(PosI128::new(0, 0), |a, b| a + *b)
        }
    }

    impl num_traits::Zero for PosI128 {
        fn zero() -> Self {
            PosI128::new(0, 0)
        }

        fn is_zero(&self) -> bool {
            self.x.is_zero() && self.y.is_zero()
        }
    }

    impl AddAssign for PosI128 {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }

    impl SubAssign for PosI128 {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }

    impl Mul<i128> for PosI128 {
        type Output = PosI128;

        fn mul(self, rhs: i128) -> Self::Output {
            PosI128::new(self.x * rhs, self.y * rhs)
        }
    }

    impl MulAssign<i128> for PosI128 {
        fn mul_assign(&mut self, rhs: i128) {
            *self = *self * rhs
        }
    }

    use std::fmt::{Debug, Error, Formatter};

    impl Debug for PosI128 {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            f.write_fmt(format_args!("({}, {})", self.x, self.y))?;
            Ok(())
        }
    }

    use proconio::source::{Readable, Source};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum PosI128XY {}
    impl Readable for PosI128XY {
        type Output = PosI128;
        fn read<R: BufRead, S: Source<R>>(source: &mut S) -> PosI128 {
            let x = i128::read(source);
            let y = i128::read(source);
            PosI128::new(x, y)
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum PosI128YX {}
    impl Readable for PosI128YX {
        type Output = PosI128;
        fn read<R: BufRead, S: Source<R>>(source: &mut S) -> PosI128 {
            let y = i128::read(source);
            let x = i128::read(source);
            PosI128::new(x, y)
        }
    }

    /// 1-indexed で与えられた座標(YX)
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum PosI128YX1 {}
    impl Readable for PosI128YX1 {
        type Output = PosI128;
        fn read<R: BufRead, S: Source<R>>(source: &mut S) -> PosI128 {
            let y = i128::read(source) - 1;
            let x = i128::read(source) - 1;
            PosI128::new(x, y)
        }
    }

    pub const DIR8_LIST: [PosI128; 8] = [
        PosI128 { x: 0, y: 1 },
        PosI128 { x: 1, y: 1 },
        PosI128 { x: 1, y: 0 },
        PosI128 { x: 1, y: -1 },
        PosI128 { x: 0, y: -1 },
        PosI128 { x: -1, y: -1 },
        PosI128 { x: -1, y: 0 },
        PosI128 { x: -1, y: 1 },
    ];

    pub const DIR4_LIST: [PosI128; 4] = [
        PosI128 { x: 0, y: 1 },
        PosI128 { x: 1, y: 0 },
        PosI128 { x: 0, y: -1 },
        PosI128 { x: -1, y: 0 },
    ];
}

#[snippet(prefix = "use vec_vec_at_i128::*;")]
pub mod vec_vec_at_i128 {
    use std::ops::{Index, IndexMut};

    use super::pos_i128::*;
    use easy_ext::ext;

    #[ext(ExtVecVecI128)]
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

        pub fn is_within(&self, pos: PosI128) -> bool {
            (0..self.width() as i128).contains(&pos.x)
                && (0..self.height() as i128).contains(&pos.y)
        }
    }
    impl<T> Index<PosI128> for Vec<Vec<T>> {
        type Output = T;

        fn index(&self, index: PosI128) -> &Self::Output {
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

    impl<T> IndexMut<PosI128> for Vec<Vec<T>> {
        fn index_mut(&mut self, index: PosI128) -> &mut Self::Output {
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

#[snippet(prefix = "use pos_i128_ix::*;")]
pub mod pos_i128_ix {
    use super::Ix;
    use super::pos_i128::PosI128;

    impl Ix for PosI128 {
        fn range((min, max): (Self, Self)) -> impl Iterator<Item = Self> {
            (min.y..=max.y).flat_map(move |y| (min.x..=max.x).map(move |x| PosI128::new(x, y)))
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
            let dy = (index / width) as i128;
            let dx = (index % width) as i128;
            PosI128::new(min.x + dx, min.y + dy)
        }

        fn in_range((min, max): (Self, Self), i: Self) -> bool {
            min.x <= i.x && i.x <= max.x && min.y <= i.y && i.y <= max.y
        }
    }
}

#[snippet(prefix = "use pos_i128_ab_group::*;")]
pub mod pos_i128_ab_group {
    use super::AbGroup;
    use super::pos_i128::PosI128;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct PosI128AbGroup;

    impl AbGroup for PosI128AbGroup {
        type S = PosI128;
        fn zero() -> Self::S {
            PosI128::new(0, 0)
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

    use super::pos_i128::*;
    use super::pos_i128_ab_group::*;
    use crate::math::algebra::ab_group::ab_group::AbGroup;

    #[test]
    fn test_pos_i128_ab_group() {
        let p1 = PosI128::new(1, 2);
        let p2 = PosI128::new(3, 4);
        assert_eq!(PosI128AbGroup::zero(), PosI128::new(0, 0));
        assert_eq!(PosI128AbGroup::add(&p1, &p2), PosI128::new(4, 6));
        assert_eq!(PosI128AbGroup::neg(&p1), PosI128::new(-1, -2));
        assert_eq!(PosI128AbGroup::sub(&p1, &p2), PosI128::new(-2, -2));
    }

    #[test]
    fn test_read() {
        let mut source = OnceSource::from("1 2");
        let p = PosI128XY::read(&mut source);
        assert_eq!(p, PosI128::new(1, 2));

        let mut source = OnceSource::from("3 4");
        let p = PosI128YX::read(&mut source);
        assert_eq!(p, PosI128::new(4, 3));

        let mut source = OnceSource::from("5 6");
        let p = PosI128YX1::read(&mut source);
        assert_eq!(p, PosI128::new(5, 4));
    }

    #[test]
    fn test_pos_add() {
        let p1: PosI128 = PosI128::new(2, 3);
        let p2: PosI128 = PosI128::new(4, 7);
        assert_eq!(p1 + p2, PosI128::new(6, 10));
    }

    #[test]
    fn test_new_from_usize() {
        assert_eq!(PosI128::new_from_usize(2, 3), PosI128::new(2, 3));
    }

    #[test]
    fn test_pos_sub() {
        let p1: PosI128 = PosI128::new(2, 3);
        let p2: PosI128 = PosI128::new(4, 7);
        assert_eq!(p2 - p1, PosI128::new(2, 4));
    }

    #[test]
    fn test_pos_neg() {
        let p1: PosI128 = PosI128::new(2, -3);
        assert_eq!(-p1, PosI128::new(-2, 3));
    }

    #[test]
    fn test_pos_zero() {
        let zero: PosI128 = PosI128::new(0, 0);
        assert_eq!(PosI128::zero(), zero);
        assert!(zero.is_zero());
    }

    #[test]
    fn test_pos_add_assign() {
        let p1: PosI128 = PosI128::new(2, 3);
        let mut p2: PosI128 = PosI128::new(4, 7);
        p2 += p1;
        assert_eq!(p2.x, 6);
        assert_eq!(p2.y, 10);
    }

    #[test]
    fn test_pos_sub_assign() {
        let p1: PosI128 = PosI128::new(2, 3);
        let mut p2: PosI128 = PosI128::new(4, 7);
        p2 -= p1;
        assert_eq!(p2.x, 2);
        assert_eq!(p2.y, 4);
    }

    #[test]
    fn test_sum() {
        let ps = [PosI128::new(1, 2), PosI128::new(3, 4), PosI128::new(5, 6)];
        assert_eq!(ps.iter().copied().sum::<PosI128>(), PosI128::new(9, 12));
        assert_eq!(ps.iter().sum::<PosI128>(), PosI128::new(9, 12));

        let empty: [PosI128; 0] = [];
        assert_eq!(empty.iter().copied().sum::<PosI128>(), PosI128::new(0, 0));
        assert_eq!(empty.iter().sum::<PosI128>(), PosI128::new(0, 0));
    }

    #[test]
    fn test_pos_scalar_mul() {
        let p: PosI128 = PosI128::new(2, 3);
        assert_eq!(p.scalar_mul(4), PosI128::new(8, 12));
        assert_eq!(p * 4, PosI128::new(8, 12));

        let mut p2 = PosI128::new(2, 3);
        p2 *= 4;
        assert_eq!(p2, PosI128::new(8, 12));
    }

    #[test]
    fn test_pos_inner_product() {
        let p1: PosI128 = PosI128::new(2, 3);
        let p2: PosI128 = PosI128::new(4, 5);
        assert_eq!(p1.inner_product(p2), 23);
    }

    #[test]
    fn test_pos_outer_product() {
        let p1: PosI128 = PosI128::new(2, 3);
        let p2: PosI128 = PosI128::new(4, 5);
        // 2*5 - 3*4 = 10 - 12 = -2
        assert_eq!(p1.outer_product(p2), -2);
    }

    #[test]
    fn test_normalize_direction() {
        assert_eq!(PosI128::new(6, 9).normalize_direction(), PosI128::new(2, 3));
        assert_eq!(
            PosI128::new(-6, 9).normalize_direction(),
            PosI128::new(-2, 3)
        );
        assert_eq!(PosI128::new(0, 5).normalize_direction(), PosI128::new(0, 1));
        assert_eq!(PosI128::new(0, 0).normalize_direction(), PosI128::new(0, 0));
    }

    #[test]
    fn test_normalize_slope() {
        assert_eq!(PosI128::new(6, 9).normalize_slope(), PosI128::new(2, 3));
        assert_eq!(PosI128::new(-6, -9).normalize_slope(), PosI128::new(2, 3));
        assert_eq!(PosI128::new(2, -1).normalize_slope(), PosI128::new(2, -1));
        assert_eq!(PosI128::new(-2, 1).normalize_slope(), PosI128::new(2, -1));
        assert_eq!(PosI128::new(0, -5).normalize_slope(), PosI128::new(0, 1));
        assert_eq!(PosI128::new(0, 0).normalize_slope(), PosI128::new(0, 0));
    }

    #[test]
    fn test_pos_rotate() {
        let p = PosI128::new(2, 3);
        assert_eq!(p.rotate90(), PosI128::new(-3, 2));
        assert_eq!(p.rotate270(), PosI128::new(3, -2));

        let p2 = PosI128::new(1, 0);
        assert_eq!(p2.rotate90(), PosI128::new(0, 1));
        assert_eq!(p2.rotate270(), PosI128::new(0, -1));
    }

    #[test]
    fn test_pos_index_1d() {
        let width = 10;
        let p = PosI128::new(2, 3); // y=3, x=2 -> 3*10 + 2 = 32
        assert_eq!(p.to_index_1d(width), 32);
        assert_eq!(PosI128::from_index_1d(32, width), p);

        let p_zero = PosI128::new(0, 0);
        assert_eq!(p_zero.to_index_1d(width), 0);
        assert_eq!(PosI128::from_index_1d(0, width), p_zero);
    }

    #[test]
    #[should_panic(expected = "x と y は 0 以上である必要があります")]
    fn test_pos_index_1d_panic_negative() {
        PosI128::new(-1, 0).to_index_1d(10);
    }

    #[test]
    #[should_panic(expected = "x は width 未満である必要があります")]
    fn test_pos_index_1d_panic_width() {
        PosI128::new(10, 0).to_index_1d(10);
    }

    #[test]
    fn test_pos_norm_square() {
        let p: PosI128 = PosI128::new(2, 3);
        assert_eq!(p.norm_square(), 13);
    }

    #[test]
    fn test_pos_norms() {
        let p = PosI128::new(2, -3);
        assert_eq!(p.l1_norm(), 5);
        assert_eq!(p.linf_norm(), 3);
    }

    #[test]
    fn test_pos_dists() {
        let p1 = PosI128::new(1, 2);
        let p2 = PosI128::new(4, -2);
        assert_eq!(p1.l1_dist(p2), 7);
        assert_eq!(p1.linf_dist(p2), 4);
        assert_eq!(p1.dist_square(p2), 25);
    }

    #[test]
    fn test_around4_pos_iter() {
        let p: PosI128 = PosI128::new(2, 3);
        let actual = p.around4_pos_iter().collect::<HashSet<PosI128>>();
        let expected = HashSet::from([
            PosI128::new(2, 2),
            PosI128::new(3, 3),
            PosI128::new(2, 4),
            PosI128::new(1, 3),
        ]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_around8_pos_iter() {
        let p: PosI128 = PosI128::new(2, 3);
        let actual = p.around8_pos_iter().collect::<HashSet<PosI128>>();
        let expected = HashSet::from([
            PosI128::new(2, 2),
            PosI128::new(3, 2),
            PosI128::new(3, 3),
            PosI128::new(3, 4),
            PosI128::new(2, 4),
            PosI128::new(1, 4),
            PosI128::new(1, 4),
            PosI128::new(1, 3),
            PosI128::new(1, 2),
        ]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_pos_debug() {
        let p = PosI128::new(2, 3);
        assert_eq!(format!("{:?}", p), "(2, 3)");
    }
}

#[cfg(test)]
mod tests_vec_vec_at_i128 {
    use super::pos_i128::*;
    use super::vec_vec_at_i128::ExtVecVecI128;

    #[test]
    fn test_vec_vec_at_i128() {
        let mut xss = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(xss[PosI128::new(2, 1)], 6);
        xss[PosI128::new(2, 1)] = 60;

        assert_eq!(xss, vec![vec![1, 2, 3], vec![4, 5, 60]])
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    #[allow(clippy::useless_vec)]
    fn test_vec_vec_at_i128_panic_index() {
        let xss = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let _ = xss[PosI128::new(3, 1)];
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    #[allow(clippy::useless_vec)]
    fn test_vec_vec_at_i128_panic_index_mut() {
        let mut xss = vec![vec![1, 2, 3], vec![4, 5, 6]];
        xss[PosI128::new(2, 2)] = 100;
    }

    #[test]
    fn test_vec_vec_at_i128_empty() {
        let xss: Vec<Vec<i32>> = vec![];
        assert_eq!(xss.width(), 0);
        assert_eq!(xss.height(), 0);
    }
}

#[cfg(test)]
mod tests_pos_i128_ix {
    use super::Ix;
    use super::pos_i128::*;

    #[test]
    fn test_pos_i128_ix() {
        let min = PosI128::new(1, 1);
        let max = PosI128::new(3, 2);
        // x: 1..=3, y: 1..=2
        // y=1: (1,1), (2,1), (3,1)
        // y=2: (1,2), (2,2), (3,2)

        let bounds = (min, max);
        assert_eq!(PosI128::range_size(bounds), 6);

        let vec: Vec<PosI128> = PosI128::range(bounds).collect();
        assert_eq!(
            vec,
            vec![
                PosI128::new(1, 1),
                PosI128::new(2, 1),
                PosI128::new(3, 1),
                PosI128::new(1, 2),
                PosI128::new(2, 2),
                PosI128::new(3, 2)
            ]
        );

        assert_eq!(PosI128::to_index(bounds, PosI128::new(1, 1)), 0);
        assert_eq!(PosI128::to_index(bounds, PosI128::new(2, 1)), 1);
        assert_eq!(PosI128::to_index(bounds, PosI128::new(3, 1)), 2);
        assert_eq!(PosI128::to_index(bounds, PosI128::new(1, 2)), 3);
        assert_eq!(PosI128::to_index(bounds, PosI128::new(3, 2)), 5);

        assert!(PosI128::in_range(bounds, PosI128::new(2, 1)));
        assert!(!PosI128::in_range(bounds, PosI128::new(0, 1)));
        assert!(!PosI128::in_range(bounds, PosI128::new(1, 3)));
    }
}
