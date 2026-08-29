/*
解法
N角形をN-2個の三角形に分割する。
N角形の重心はN-2個の三角形の重心を面積で重み付けして平均を取ったもの

頂点a,b,c の三角形の面積は
(b-a)×(c-a) = b×c - (b-c)×a  ← 外積は可換ではなく b×c - (b+c)×a ではない点に注意

頂点a,b,c の三角形の重心は (a+b+c)/3

面積×重心は

(b×c - (b-c)×a) (a+b+c)/3
= ...
と展開し、項ごとに総和を取れば、N-2個の面積×重心の和が求まる。

 */
fn main() {
    input! {
        n: usize,
        nq: usize,
        ps: [PosXY; n],
        qs: [(Usize1, Usize1); nq]
    }

    let ps = [ps.as_slice(), ps.as_slice()].concat();

    let sub_list = ps
        .iter()
        .copied()
        .tuple_windows()
        .map(|(p, q)| p - q)
        .collect_vec();

    let mult_list = ps
        .iter()
        .copied()
        .tuple_windows()
        .map(|(p, q)| p.outer_product(q))
        .collect_vec();
    let term1_list = ps
        .iter()
        .copied()
        .tuple_windows()
        .map(|(p, q)| (p + q) * (p.outer_product(q)))
        .collect_vec();

    let term2_list = ps
        .iter()
        .copied()
        .tuple_windows()
        .map(|(p, q)| {
            let a = p + q;
            let b = p - q;

            [-a.x * b.y, a.x * b.x, -a.y * b.y, a.y * b.x]
        })
        .collect_vec();

    let cum_sub_list = sub_list
        .iter()
        .copied()
        .scanl(Pos::zero(), |acc, x| *acc + x)
        .collect_vec();

    let cum_mult_list = mult_list
        .iter()
        .copied()
        .scanl(0, |acc, x| *acc + x)
        .collect_vec();

    let cum_term1_list = term1_list
        .iter()
        .copied()
        .scanl(Pos::zero(), |acc, x| *acc + x)
        .collect_vec();

    let cum_term2_list = term2_list
        .iter()
        .copied()
        .scanl([0, 0, 0, 0], |[a, b, c, d], [x, y, z, w]| {
            [*a + x, *b + y, *c + z, *d + w]
        })
        .collect_vec();

    for (u, v) in qs {
        let v = if v < u { v + n } else { v };

        // 面積の総和
        let area_sum2 = {
            let s1 = cum_mult_list[v] - cum_mult_list[u + 1];
            let s2 = cum_sub_list[v] - cum_sub_list[u + 1];

            s1 - s2.outer_product(ps[u])
        };

        // (三角形の重心 * 面積)の総和
        let vec_sum6 = {
            let term1 = cum_term1_list[v] - cum_term1_list[u + 1];
            let term2 = {
                let begin = cum_term2_list[u + 1];
                let end = cum_term2_list[v];

                let diff = [
                    end[0] - begin[0],
                    end[1] - begin[1],
                    end[2] - begin[2],
                    end[3] - begin[3],
                ];

                -Pos::new(
                    diff[0] * ps[u].x + diff[1] * ps[u].y,
                    diff[2] * ps[u].x + diff[3] * ps[u].y,
                )
            };

            let term3 = ps[u] * (cum_mult_list[v] - cum_mult_list[u + 1]);
            let term4 = -ps[u] * (cum_sub_list[v] - cum_sub_list[u + 1]).outer_product(ps[u]);
            term1 + term2 + term3 + term4
        };

        let ans_x = vec_sum6.x as f64 / (3.0 * area_sum2 as f64);
        let ans_y = vec_sum6.y as f64 / (3.0 * area_sum2 as f64);
        println!("{} {}", ans_x, ans_y);
        //
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
use {num::Zero, pos::*};
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
        pub fn new_from_usize(x: usize, y: usize) -> Pos {
            Pos::new(x as i64, y as i64)
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
        /// 向きが同じであれば同一視する正規化 (方向ベクトル)
        /// 最大公約数で割り、符号はそのまま残す。
        /// (0,0) の場合は (0,0) を返す。
        /// 計算量: O(log(min(|x|, |y|)))
        pub fn normalize_direction(self) -> Pos {
            if self.x == 0 && self.y == 0 {
                return self;
            }
            let g = num::integer::gcd(self.x.abs(), self.y.abs());
            Pos::new(self.x / g, self.y / g)
        }
        /// 平行であれば同一視する正規化（直線の傾きを表す）
        /// 最大公約数で割り、最初の非零成分 (x が優先) が正になるように符号を統一する。
        /// (0,0) の場合は (0,0) を返す。
        /// 計算量: O(log(min(|x|, |y|)))
        pub fn normalize_slope(self) -> Pos {
            if self.x == 0 && self.y == 0 {
                return self;
            }
            let p = if self.x < 0 || (self.x == 0 && self.y < 0) {
                -self
            } else {
                self
            };
            let g = num::integer::gcd(p.x, p.y);
            Pos::new(p.x / g, p.y / g)
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
use scan_iter::*;
#[allow(clippy::module_inception)]
pub mod scan_iter {
    #[derive(Clone)]
    pub struct Scanl<I, B, F> {
        iter: I,
        state: Option<B>,
        f: F,
    }
    impl<I, B, F> Scanl<I, B, F> {
        fn new(iter: I, init: B, f: F) -> Scanl<I, B, F> {
            Scanl {
                iter,
                state: Some(init),
                f,
            }
        }
    }
    impl<I, B, F> Iterator for Scanl<I, B, F>
    where
        B: Copy,
        I: Iterator,
        F: FnMut(&mut B, I::Item) -> B,
    {
        type Item = B;
        #[inline]
        fn next(&mut self) -> Option<B> {
            let retval = self.state?;
            let a_opt = self.iter.next();
            self.state = self
                .state
                .and_then(|mut s| a_opt.map(|a| (self.f)(&mut s, a)));
            Some(retval)
        }
    }
    pub trait IteratorExtScanLeft: Iterator + Sized {
        fn scanl<B, F>(self, init: B, f: F) -> Scanl<Self, B, F>
        where
            Self: Sized,
            F: FnMut(&mut B, Self::Item) -> B,
        {
            Scanl::new(self, init, f)
        }
    }
    impl<T: Iterator> IteratorExtScanLeft for T {}
}
