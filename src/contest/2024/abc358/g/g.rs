//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    h: usize,
    w: usize,
    k: usize,
    start: Pos<i64>,
    grid: Vec<Vec<i64>>,
}

struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Vec<i64>>,
}

impl Grid {
    fn new(width: usize, height: usize, grid: &[Vec<i64>]) -> Grid {
        Grid {
            width,
            height,
            grid: grid.to_vec(),
        }
    }

    pub fn at(&self, pos: Pos<i64>) -> i64 {
        self.grid[pos.y as usize][pos.x as usize]
    }

    pub fn encode(&self, pos: Pos<i64>) -> usize {
        (pos.y * self.width as i64 + pos.x) as usize
    }

    pub fn decode(&self, i: usize) -> Pos<i64> {
        let y = (i / self.width) as i64;
        let x = (i % self.width) as i64;
        Pos::new(x, y)
    }

    pub fn is_within(&self, pos: Pos<i64>) -> bool {
        let h = self.height as i64;
        let w = self.width as i64;
        0 <= pos.y && pos.y < h && 0 <= pos.x && pos.x < w
    }

    pub fn all_pos_iter(&self) -> impl Iterator<Item = Pos<i64>> {
        iproduct!(0..self.height, 0..self.width).map(|(y, x)| Pos::new(x as i64, y as i64))
    }
}

impl Problem {
    fn read() -> Problem {
        input! {
            h: usize,
            w: usize,
            k: usize,
            start: (i64, i64),
            grid: [[i64; w]; h],
        }
        let start = Pos::new(start.1 - 1, start.0 - 1);
        Problem {
            h,
            w,
            k,
            start,
            grid,
        }
    }
    fn solve(&self) -> Answer {
        // 解法: 最初にベルマンフォードぽいのをして、後はその場にとどまってもらう
        let k = self.k;
        let w = self.w;
        let h = self.h;
        let n_step_bf = usize::min(k, w * h);
        let n_step_stopping = k - n_step_bf;
        let mut dp = vec![NegInf; w * h];

        let grid = Grid::new(self.w, self.h, &self.grid);
        let dir5: [Pos<i64>; 5] = [
            Pos { x: 0, y: 0 },
            Pos { x: 0, y: 1 },
            Pos { x: 1, y: 0 },
            Pos { x: 0, y: -1 },
            Pos { x: -1, y: 0 },
        ];

        dp[grid.encode(self.start)] = Fin(0);

        // ベルマンフォードぽく start から n_step_bf 回動かす
        for _ in 0..n_step_bf {
            let mut next_dp = vec![NegInf; w * h];
            for pos_i in 0..w * h {
                let pos = grid.decode(pos_i);
                for next_pos in dir5
                    .iter()
                    .copied()
                    .map(|dir| pos + dir)
                    .filter(|next_pos| grid.is_within(*next_pos))
                {
                    let next_pos_i = grid.encode(next_pos);
                    next_dp[next_pos_i] =
                        NegExtInt::max(next_dp[next_pos_i], dp[pos_i] + grid.at(next_pos));
                }
            }
            dp = next_dp;
        }

        // その場に n_step_stopping 回とどまる
        for pos_i in 0..w * h {
            let pos = grid.decode(pos_i);
            dp[pos_i] += Fin(grid.at(pos) * n_step_stopping as i64);
        }

        let ans = dp.iter().copied().max().unwrap().get_fin();
        Answer { ans }
    }
    fn solve_tle(&self) -> Answer {
        // 行列累乗を使ったもの
        // 計算量 (WH)^3 log K ≒ 5 × 10^11 (TLE)

        let grid = Grid::new(self.w, self.h, &self.grid);

        let size = self.h * self.w;

        let matrix = {
            let mut matrix = vec![vec![NegInf; size]; size];
            let dir5: [Pos<i64>; 5] = [
                Pos { x: 0, y: 0 },
                Pos { x: 0, y: 1 },
                Pos { x: 1, y: 0 },
                Pos { x: 0, y: -1 },
                Pos { x: -1, y: 0 },
            ];
            for pos in grid.all_pos_iter() {
                for next_pos in dir5
                    .iter()
                    .copied()
                    .map(|dir| pos + dir)
                    .filter(|next_pos| grid.is_within(*next_pos))
                {
                    let pos_encode = grid.encode(pos);
                    let next_pos_encode = grid.encode(next_pos);
                    matrix[pos_encode][next_pos_encode] = Fin(grid.at(next_pos));
                }
            }
            MaxPlusMatrix::new(&matrix)
        };

        let monoid = MaxPlusMatrixMonoid { size };

        let powered = monoid.pow(&matrix, self.k);

        let start_encoded = grid.encode(self.start);
        let ans = powered.matrix[start_encoded]
            .iter()
            .copied()
            .max()
            .unwrap()
            .get_fin();
        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        todo!();
        // let ans = 0;
        // Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: i64,
}

impl Answer {
    fn print(&self) {
        println!("{}", self.ans);
    }
}

fn main() {
    Problem::read().solve().print();
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

    #[allow(dead_code)]
    #[derive(Debug)]
    struct WrongTestCase {
        problem: Problem,
        main_ans: Answer,
        naive_ans: Answer,
    }

    #[allow(dead_code)]
    fn check(p: &Problem) -> Option<WrongTestCase> {
        let main_ans = p.solve_tle();
        let naive_ans = p.solve_naive();
        if main_ans != naive_ans {
            Some(WrongTestCase {
                problem: p.clone(),
                main_ans,
                naive_ans,
            })
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn make_random_problem(rng: &mut SmallRng) -> Problem {
        todo!()
        // let n = rng.gen_range(1..=10);
        // let p = Problem { _a: n };
        // println!("{:?}", &p);
        // p
    }

    #[allow(unreachable_code)]
    #[test]
    fn test_with_naive() {
        let num_tests = 0;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut rng = SmallRng::seed_from_u64(42);
        // let mut rng = SmallRng::from_entropy();
        let mut wrong_cases: Vec<WrongTestCase> = vec![];
        for _ in 0..num_tests {
            let p = make_random_problem(&mut rng);
            let result = check(&p);
            if let Some(wrong_test_case) = result {
                wrong_cases.push(wrong_test_case);
            }
            if wrong_cases.len() >= max_wrong_case {
                break;
            }
        }

        if !wrong_cases.is_empty() {
            for t in &wrong_cases {
                println!("{:?}", t.problem);
                println!("main ans : {:?}", t.main_ans);
                println!("naive ans: {:?}", t.naive_ans);
                println!();
            }
            println!("{} cases are wrong.", wrong_cases.len());
            panic!();
        }
    }
}

// ====== import ======
#[allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::convert::Infallible;

// ====== output func ======
#[allow(unused_imports)]
use print_vec::*;
pub mod print_vec {

    use itertools::Itertools;
    use proconio::fastout;
    #[fastout]
    pub fn print_vec<T: std::fmt::Debug>(arr: &[T]) {
        for a in arr {
            println!("{:?}", a);
        }
    }
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Debug>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{:?}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Debug>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{:?}", x)).join(" ");
            println!("{}", msg);
        }
    }
    pub fn print_bytes(bytes: &[u8]) {
        let msg = String::from_utf8(bytes.to_vec()).unwrap();
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec_bytes(vec_bytes: &[Vec<u8>]) {
        for row in vec_bytes {
            let msg = String::from_utf8(row.to_vec()).unwrap();
            println!("{}", msg);
        }
    }
}

#[allow(unused)]
fn print_yesno(ans: bool) {
    let msg = if ans { "Yes" } else { "No" };
    println!("{}", msg);
}

// ====== snippet ======
use dynamic_monoid::*;
pub mod dynamic_monoid {
    pub trait DynamicMonoid {
        type S: Clone;
        fn identity(&self) -> Self::S;
        fn binary_operation(&self, a: &Self::S, b: &Self::S) -> Self::S;
        /// base^n を求める
        fn pow(&self, base: &Self::S, n: usize) -> Self::S {
            let mut base = base.clone();
            let mut ans = self.identity();
            let mut n = n;
            while n > 0 {
                if n & 1 == 1 {
                    ans = self.binary_operation(&ans, &base);
                }
                base = self.binary_operation(&base, &base);
                n >>= 1;
            }
            ans
        }
    }
}

use mod_neg_ext_int::NegExtInt::{self, *};
pub mod mod_neg_ext_int {
    use ac_library::Monoid;
    use std::{
        cmp::Ordering,
        convert::Infallible,
        ops::{Add, AddAssign},
    };
    use NegExtInt::*;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NegExtInt {
        NegInf,
        Fin(i64),
    }
    impl NegExtInt {
        pub fn get_fin(self) -> i64 {
            match self {
                Fin(val) => val,
                NegInf => panic!("called `NegExtInt::get_fin()` on a `Fin` value"),
            }
        }
        pub fn get_fin_or(self, default: i64) -> i64 {
            match self {
                Fin(val) => val,
                NegInf => default,
            }
        }
        pub fn is_fin(self) -> bool {
            matches!(self, Fin(_))
        }
        pub fn is_neginf(self) -> bool {
            matches!(self, NegInf)
        }
        pub fn to_option(self) -> Option<i64> {
            match self {
                NegInf => None,
                Fin(a) => Some(a),
            }
        }
        pub fn from_option(opt: Option<i64>) -> NegExtInt {
            match opt {
                Some(a) => Fin(a),
                None => NegInf,
            }
        }
        pub fn times(self, t: i64) -> Self {
            match t.cmp(&0) {
                Ordering::Less => panic!("t must be non-negative."),
                Ordering::Equal => Fin(0),
                Ordering::Greater => match self {
                    NegInf => NegInf,
                    Fin(a) => Fin(a * t),
                },
            }
        }
    }
    impl Add for NegExtInt {
        type Output = NegExtInt;
        fn add(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (NegInf, NegInf) => NegInf,
                (NegInf, Fin(_)) => NegInf,
                (Fin(_), NegInf) => NegInf,
                (Fin(a), Fin(b)) => Fin(a + b),
            }
        }
    }
    impl AddAssign for NegExtInt {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }
    impl Add<i64> for NegExtInt {
        type Output = NegExtInt;
        fn add(self, rhs: i64) -> Self::Output {
            match self {
                NegInf => NegInf,
                Fin(a) => Fin(a + rhs),
            }
        }
    }
    impl AddAssign<i64> for NegExtInt {
        fn add_assign(&mut self, rhs: i64) {
            *self = *self + rhs;
        }
    }
    impl std::iter::Sum for NegExtInt {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            let mut s = 0;
            for x in iter {
                match x {
                    NegInf => return NegInf,
                    Fin(x) => s += x,
                }
            }
            Fin(s)
        }
    }
    impl PartialOrd for NegExtInt {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match (self, other) {
                (NegInf, NegInf) => Some(Ordering::Equal),
                (NegInf, Fin(_)) => Some(Ordering::Less),
                (Fin(_), NegInf) => Some(Ordering::Greater),
                (Fin(a), Fin(b)) => PartialOrd::partial_cmp(a, b),
            }
        }
    }
    impl Ord for NegExtInt {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }
    pub struct NegExtIntAdditive(Infallible);
    impl Monoid for NegExtIntAdditive {
        type S = NegExtInt;
        fn identity() -> Self::S {
            Fin(0)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
    }
    pub struct NegExtIntMax(Infallible);
    impl Monoid for NegExtIntMax {
        type S = NegExtInt;
        fn identity() -> Self::S {
            NegInf
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a.max(b)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MaxPlusMatrix {
    matrix: Vec<Vec<NegExtInt>>,
}

impl MaxPlusMatrix {
    fn new(matrix: &Vec<Vec<NegExtInt>>) -> Self {
        MaxPlusMatrix {
            matrix: matrix.clone(),
        }
    }
}

struct MaxPlusMatrixMonoid {
    size: usize,
}
impl DynamicMonoid for MaxPlusMatrixMonoid {
    type S = MaxPlusMatrix;

    fn identity(&self) -> Self::S {
        let matrix = (0..self.size)
            .map(|y| {
                (0..self.size)
                    .map(move |x| if x == y { Fin(0) } else { NegInf })
                    .collect_vec()
            })
            .collect_vec();
        MaxPlusMatrix::new(&matrix)
    }

    fn binary_operation(&self, a: &Self::S, b: &Self::S) -> Self::S {
        let ans_matrix = (0..self.size)
            .map(|y| {
                (0..self.size)
                    .map(move |x| {
                        (0..self.size)
                            .map(|k| a.matrix[y][k] + b.matrix[k][x])
                            .max()
                            .unwrap()
                    })
                    .collect_vec()
            })
            .collect_vec();

        MaxPlusMatrix::new(&ans_matrix)
    }
}

use pos::*;
pub mod pos {
    use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Pos<T> {
        pub x: T,
        pub y: T,
    }
    impl<T> Pos<T> {
        pub fn new(x: T, y: T) -> Pos<T> {
            Pos { x, y }
        }
    }
    impl<T: Mul<Output = T> + Copy> Pos<T> {
        pub fn scala_mul(self, rhs: T) -> Pos<T> {
            Pos::new(self.x * rhs, self.y * rhs)
        }
    }
    impl<T: Add<Output = T> + Mul<Output = T> + Copy> Pos<T> {
        pub fn norm_square(self) -> T {
            self.x * self.x + self.y * self.y
        }
    }
    impl<T: Add<Output = T> + Copy> Add for Pos<T> {
        type Output = Pos<T>;
        fn add(self, rhs: Self) -> Self::Output {
            Pos::new(self.x + rhs.x, self.y + rhs.y)
        }
    }
    impl<T: Sub<Output = T> + Copy> Sub for Pos<T> {
        type Output = Pos<T>;
        fn sub(self, rhs: Self) -> Self::Output {
            Pos::new(self.x - rhs.x, self.y - rhs.y)
        }
    }
    impl<T: Neg<Output = T>> Neg for Pos<T> {
        type Output = Self;
        fn neg(self) -> Self::Output {
            Pos::new(-self.x, -self.y)
        }
    }
    impl<T: num_traits::Zero + Copy> num_traits::Zero for Pos<T> {
        fn zero() -> Self {
            Pos::new(T::zero(), T::zero())
        }
        fn is_zero(&self) -> bool {
            self.x.is_zero() && self.y.is_zero()
        }
    }
    impl<T: Add<Output = T> + Copy> AddAssign for Pos<T> {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }
    impl<T: Sub<Output = T> + Copy> SubAssign for Pos<T> {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }
    pub const DIR8_LIST: [Pos<i64>; 8] = [
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 1 },
        Pos { x: 1, y: 0 },
        Pos { x: 1, y: -1 },
        Pos { x: 0, y: -1 },
        Pos { x: -1, y: -1 },
        Pos { x: -1, y: 0 },
        Pos { x: -1, y: 1 },
    ];
    pub const DIR4_LIST: [Pos<i64>; 4] = [
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 0 },
        Pos { x: 0, y: -1 },
        Pos { x: -1, y: 0 },
    ];
    impl Pos<i64> {
        pub fn around4_pos_iter(self) -> impl Iterator<Item = Pos<i64>> {
            DIR4_LIST.iter().copied().map(move |d| d + self)
        }
        pub fn around8_pos_iter(self) -> impl Iterator<Item = Pos<i64>> {
            DIR8_LIST.iter().copied().map(move |d| d + self)
        }
    }
}
