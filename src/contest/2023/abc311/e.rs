use std::io::stdin;

use itertools::Itertools;
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

struct Problem {
    height: usize,
    width: usize,
    n_holes: usize,
    hole_pos_list: Vec<Pos<usize>>,
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
        let mut dp = vec![vec![0; self.width]; self.height];
        let mut is_hole = vec![vec![false; self.width]; self.height];

        for &hole_pos in &self.hole_pos_list {
            is_hole[hole_pos.y][hole_pos.x] = true;
        }

        for x in 0..self.width {
            if !is_hole[0][x] {
                // これつけ忘れていた
                dp[0][x] = 1;
            }
        }

        for y in 0..self.height {
            if !is_hole[y][0] {
                dp[y][0] = 1;
            }
        }

        for y in 1..self.height {
            for x in 1..self.width {
                if !is_hole[y][x] {
                    dp[y][x] =
                        [dp[y - 1][x - 1], dp[y - 1][x], dp[y][x - 1]].iter().min().unwrap() + 1;
                }
            }
        }

        Answer { ans: dp.iter().flatten().sum() }
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
    fn test_problem() {
        let input = "
3
4
        "
        .trim();
        check(input, Answer { ans: 7 });
    }
}
