use std::io::stdin;

use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use myio::*;
pub mod myio {
    use std::io::BufRead;

    pub trait IProconReader {
        fn read_line(&mut self) -> String;

        fn read_bytes(&mut self) -> Vec<u8> {
            self.read_line().as_bytes().to_vec()
        }

        fn read_any1<T>(&mut self) -> T
        where
            T: std::str::FromStr,
            T::Err: std::fmt::Debug,
        {
            let buf = self.read_line();
            buf.parse::<T>().unwrap()
        }

        fn read_any2<T0, T1>(&mut self) -> (T0, T1)
        where
            T0: std::str::FromStr,
            T0::Err: std::fmt::Debug,
            T1: std::str::FromStr,
            T1::Err: std::fmt::Debug,
        {
            let buf = self.read_line();
            let splitted = buf.trim().split(' ').collect::<Vec<_>>();
            let a0 = splitted[0].parse::<T0>().unwrap();
            let a1 = splitted[1].parse::<T1>().unwrap();
            (a0, a1)
        }

        fn read_any3<T0, T1, T2>(&mut self) -> (T0, T1, T2)
        where
            T0: std::str::FromStr,
            T0::Err: std::fmt::Debug,
            T1: std::str::FromStr,
            T1::Err: std::fmt::Debug,
            T2: std::str::FromStr,
            T2::Err: std::fmt::Debug,
        {
            let buf = self.read_line();
            let splitted = buf.trim().split(' ').collect::<Vec<_>>();
            let a0 = splitted[0].parse::<T0>().unwrap();
            let a1 = splitted[1].parse::<T1>().unwrap();
            let a2 = splitted[2].parse::<T2>().unwrap();
            (a0, a1, a2)
        }
        fn read_vec_any<T>(&mut self) -> Vec<T>
        where
            T: std::str::FromStr,
            T::Err: std::fmt::Debug,
        {
            let buf = self.read_line();
            buf.trim().split(' ').map(|s| s.parse::<T>().unwrap()).collect::<Vec<T>>()
        }

        fn read_vec_i64(&mut self) -> Vec<i64> {
            self.read_vec_any::<i64>()
        }

        fn read_vec_usize(&mut self) -> Vec<usize> {
            self.read_vec_any::<usize>()
        }

        fn read_vec_str(&mut self) -> Vec<String> {
            self.read_vec_any::<String>()
        }

        fn read_i64_1(&mut self) -> i64 {
            self.read_any1::<i64>()
        }

        fn read_i64_2(&mut self) -> (i64, i64) {
            self.read_any2::<i64, i64>()
        }

        fn read_i64_3(&mut self) -> (i64, i64, i64) {
            self.read_any3::<i64, i64, i64>()
        }

        fn read_usize_1(&mut self) -> usize {
            self.read_any1::<usize>()
        }

        fn read_usize_2(&mut self) -> (usize, usize) {
            self.read_any2::<usize, usize>()
        }

        fn read_usize_3(&mut self) -> (usize, usize, usize) {
            self.read_any3::<usize, usize, usize>()
        }
    }

    pub struct ProconReader<R: BufRead> {
        buf_read: R,
    }

    impl<R: BufRead> ProconReader<R> {
        pub fn new(buf_read: R) -> ProconReader<R> {
            ProconReader { buf_read }
        }
    }

    impl<R: BufRead> IProconReader for ProconReader<R> {
        fn read_line(&mut self) -> String {
            let mut buffer = String::new();
            self.buf_read.read_line(&mut buffer).unwrap();
            buffer.trim().to_string()
        }
    }
}
use pos::*;
pub mod pos {
    use std::ops::{Add, Mul, Sub};
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    impl<T: num_traits::Zero + Copy> num_traits::Zero for Pos<T> {
        fn zero() -> Self {
            Pos::new(T::zero(), T::zero())
        }
        fn is_zero(&self) -> bool {
            self.x.is_zero() && self.y.is_zero()
        }
    }
}

pub fn print_vecvec<T: std::fmt::Debug>(arr: &Vec<Vec<T>>, height: usize, width: usize) {
    for i in 0..height {
        for j in 0..width {
            print!("{:?} ", arr[i][j]);
        }
        println!();
    }
}
use scan_iter::*;
pub mod scan_iter {
    #[derive(Clone)]
    pub struct Scanl<I, B, F> {
        iter: I,
        state: Option<B>,
        f: F,
    }
    impl<I, B, F> Scanl<I, B, F> {
        fn new(iter: I, init: B, f: F) -> Scanl<I, B, F> {
            Scanl { iter, state: Some(init), f }
        }
    }
    impl<I, B, F> Iterator for Scanl<I, B, F>
    where
        B: Clone,
        I: Iterator,
        F: FnMut(&mut B, I::Item) -> B,
    {
        type Item = B;
        #[inline]
        fn next(&mut self) -> Option<B> {
            let retval = self.state.clone()?;
            let a_opt = self.iter.next();
            self.state = self.state.clone().and_then(|mut s| a_opt.map(|a| (self.f)(&mut s, a)));
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

struct Problem {
    height: usize,
    width: usize,
    n_holes: usize,
    hole_pos_list: Vec<Pos<usize>>,
}

fn cumsum_2d(xss: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    if xss.is_empty() {
        return vec![vec![0]];
    }

    let height = xss.len();
    let width = xss[0].len();
    let mut cumsum = vec![vec![0; width + 1]; height + 1];
    for y in 1..height + 1 {
        for x in 1..width + 1 {
            cumsum[y][x] =
                cumsum[y - 1][x] + cumsum[y][x - 1] - cumsum[y - 1][x - 1] + xss[y - 1][x - 1];
        }
    }
    cumsum
}

fn sum_from_cumsum_2d(
    cumsum: &Vec<Vec<i64>>,
    (x1, y1): (usize, usize),
    (x2, y2): (usize, usize),
) -> i64 {
    // [x1, x2) × [y1, y2) の範囲で総和を求める
    cumsum[y2][x2] - cumsum[y2][x1] - cumsum[y1][x2] + cumsum[y1][x1]
}

/// 二分探索をする
/// ```text
/// ng ng ng ok ok ok
///          ↑ここの引数の値を返す
/// ```
/// ## Arguments
/// * ok != ng
/// * |ok - ng| <= 2^63 - 1, |ok + ng| <= 2^63 - 1
/// * p の定義域について
///     * ng < ok の場合、p は区間 ng..ok で定義されている。
///     * ok < ng の場合、p は区間 ok..ng で定義されている。
/// * p の単調性について
///     * ng < ok の場合、p は単調増加
///     * ok < ng の場合、p は単調減少
/// ## Return
/// * ng < ok の場合: I = { i in ng..ok | p(i) == true } としたとき
///     * I が空でなければ、min I を返す。
///     * I が空ならば、ok を返す。
/// * ok < ng の場合: I = { i in ok..ng | p(i) == true } としたとき
///     * I が空でなければ、max I を返す。
///     * I が空ならば、ok を返す。
pub fn bin_search<F>(mut ok: i64, mut ng: i64, p: F) -> i64
where
    F: Fn(i64) -> bool,
{
    assert!(ok != ng);
    assert!(ok.checked_sub(ng).is_some());
    assert!(ok.checked_add(ng).is_some());
    while num::abs(ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        assert!(mid != ok);
        assert!(mid != ng);
        if p(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}

impl Problem {
    fn read<R: IProconReader>(mut r: R) -> Problem {
        let (height, width, n_holes) = r.read_usize_3();
        let hole_pos_list = (0..n_holes)
            .map(|_| {
                let (y, x) = r.read_usize_2();
                Pos::new(x - 1, y - 1)
            })
            .collect_vec();
        Problem { height, width, n_holes, hole_pos_list }
    }
    fn solve(&self) -> Answer {
        let mut hole_indicator = vec![vec![0; self.width]; self.height];

        for &hole_pos in &self.hole_pos_list {
            hole_indicator[hole_pos.y][hole_pos.x] = 1;
        }
        let hole_indicator_cumsum = cumsum_2d(&hole_indicator);

        let has_hole = |(x, y): (usize, usize), size: usize| {
            // 左上が(x,y)、サイズが size の正方形に穴があるかどうか
            if x + size >= self.width + 1 || y + size >= self.height + 1 {
                return true;
            }
            sum_from_cumsum_2d(&hole_indicator_cumsum, (x, y), (x + size, y + size)) > 0
        };

        let ng_size = usize::min(self.width, self.height) + 1; // ここ+1し忘れた
        let ans = iproduct!(0..self.height, 0..self.width)
            .map(|(y, x)| {
                // 左上が (y, x) の正方形で穴がない最大の一辺の長さを返す
                // ok: 0
                // ng: usize::min(self.width, self.height) + 1
                let no_hole = |size: i64| !has_hole((x, y), size as usize);
                bin_search(0, ng_size as i64, no_hole)
            })
            .sum::<i64>();

        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: i64,
}

impl Answer {
    fn print(&self) {
        println!("{}", self.ans)
    }
}

fn main() {
    Problem::read(ProconReader::new(stdin().lock())).solve().print();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, expected: Answer) {
        let actual = Problem::read(ProconReader::new(input.as_bytes())).solve();
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_cumsum2d() {
        let xss = vec![vec![1, 2], vec![4, 5]];
        let cumsum_actual = cumsum_2d(&xss);
        let cumsum_expected = vec![vec![0, 0, 0], vec![0, 1, 3], vec![0, 5, 12]];
        assert_eq!(cumsum_expected, cumsum_actual);
    }

    #[test]
    fn test_problem() {
        let input = "
3
4
        "
        .trim();
        check(input, Answer { ans: 7 });
    }
}
