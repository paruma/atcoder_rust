// 問題文と制約は読みましたか？
// #[fastout]
// #[fastout]
fn main() {
    input! {
        t: usize
    }
    for _ in 0..t {
        input! {
            x1: i128,
            y1: i128,
            r1: i128,
            x2: i128,
            y2: i128,
            r2: i128,
        }

        let d2 = {
            let delx = x1 - x2;
            let dely = y1 - y2;

            delx * delx + dely * dely
        };

        // 勘違いして、r1, r2 を二乗しようとしてしまった（√を外す気持ちだけど元から√はついていない）
        let r12 = r1 * r1;
        let r22 = r2 * r2;

        let cond1 = { r12 + r22 - d2 < 0 || (r12 + r22 - d2) * (r12 + r22 - d2) <= 4 * r12 * r22 };

        let cond2 = { d2 - r12 - r22 < 0 || (r12 + r22 - d2) * (r12 + r22 - d2) <= 4 * r12 * r22 };
        let ans = cond1 && cond2;
        print_yesno(ans);
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }

    /// 間違っていたら false を返す
    fn process_one_test(rng: &mut SmallRng) -> bool {
        // ==== 問題を作る ====
        let n = rng.random_range(1..=10);
        let xs = (0..n).map(|_| rng.random_range(0..10)).collect_vec();

        // ==== 解く ====
        let main_ans = xs.len();
        let naive_ans = 1;

        // ==== 間違っていたら報告をする ====
        if main_ans != naive_ans {
            // 問題を出力
            println!("{:?}", (n, xs));
            println!("main ans : {:?}", main_ans);
            println!("naive ans: {:?}", naive_ans);
            return false;
        }
        true
    }

    #[allow(unreachable_code)]
    #[test]
    #[ignore]
    fn test_with_naive() {
        let num_tests = 100;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut cnt_wrong = 0;
        let mut rng = SmallRng::seed_from_u64(42);
        // let mut rng = SmallRng::from_os_rng();
        for _ in 0..num_tests {
            let is_ok = process_one_test(&mut rng);
            if !is_ok {
                cnt_wrong += 1;
            }
            if cnt_wrong >= max_wrong_case {
                break;
            }
        }
        if cnt_wrong > 0 {
            println!("{} cases are wrong.", cnt_wrong);
            panic!();
        }
    }
}

// ====== import ======
#[allow(unused_imports)]
use {
    itertools::{Itertools, chain, iproduct, izip},
    proconio::{
        derive_readable, fastout, input,
        marker::{Bytes, Chars, Usize1},
    },
    rand::{Rng, SeedableRng, rngs::SmallRng, seq::SliceRandom},
    std::{
        cmp::Reverse,
        collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet},
    },
};

// ====== output func ======
#[allow(unused_imports)]
use print_util::*;
pub mod print_util {
    use itertools::Itertools;
    use proconio::fastout;

    #[fastout]
    pub fn print_vec<T: std::fmt::Display>(arr: &[T]) {
        for a in arr {
            println!("{}", a);
        }
    }

    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
        println!("{}", arr.iter().join(" "));
    }

    #[fastout]
    pub fn print_vec2<T: std::fmt::Display, R: AsRef<[T]>>(arr: &[R]) {
        for row in arr {
            println!("{}", row.as_ref().iter().join(" "));
        }
    }

    pub fn print_bytes(bytes: &[u8]) {
        println!("{}", std::str::from_utf8(bytes).unwrap());
    }

    pub fn print_chars(chars: &[char]) {
        println!("{}", chars.iter().collect::<String>());
    }

    #[fastout]
    pub fn print_vec_bytes<R: AsRef<[u8]>>(vec_bytes: &[R]) {
        for row in vec_bytes {
            println!("{}", std::str::from_utf8(row.as_ref()).unwrap());
        }
    }

    #[fastout]
    pub fn print_vec_chars<R: AsRef<[char]>>(vec_chars: &[R]) {
        for row in vec_chars {
            println!("{}", row.as_ref().iter().collect::<String>());
        }
    }

    pub fn print_yesno(ans: bool) {
        println!("{}", if ans { "Yes" } else { "No" });
    }
}

// ====== snippet ======
use pos::*;
#[allow(clippy::module_inception)]
pub mod pos {
    use std::io::BufRead;
    use std::iter::Sum;
    use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct Pos {
        pub x: i64,
        pub y: i64,
    }
    impl Pos {
        pub fn new(x: i64, y: i64) -> Pos {
            Pos { x, y }
        }
        pub fn scalar_mul(self, rhs: i64) -> Pos {
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
        pub fn normalize(self) -> Pos {
            if self.x == 0 && self.y == 0 {
                return self;
            }
            let g = num::integer::gcd(self.x.abs(), self.y.abs());
            Pos::new(self.x / g, self.y / g)
        }
        pub fn rotate90(self) -> Pos {
            Pos::new(-self.y, self.x)
        }
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
