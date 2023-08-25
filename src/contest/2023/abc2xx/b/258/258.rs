use std::io::stdin;

struct Problem {
    n: usize,
    grid: Vec<Vec<i64>>,
}

impl Problem {
    fn read<R: IProconReader>(mut r: R) -> Problem {
        let n = r.read_usize_1();
        let grid = (0..n)
            .map(|_| r.read_bytes().iter().map(|ch| (ch - b'0') as i64).collect_vec())
            .collect_vec();
        Problem { n, grid }
    }
    fn solve(&self) -> Answer {
        let Problem { n, grid } = self;
        let n = *n;
        let dir_list: [Pos<i64>; 8] = [
            Pos::new(1, 0),
            Pos::new(1, 1),
            Pos::new(0, 1),
            Pos::new(-1, 1),
            Pos::new(-1, 0),
            Pos::new(-1, -1),
            Pos::new(0, -1),
            Pos::new(1, -1),
        ];
        let ans = dir_list
            .iter()
            .flat_map(|dir| {
                // dir で進んだ場合の最大値を求める。
                iproduct!(0..n, 0..n).map(move |(x, y)| {
                    let init_pos: Pos<i64> = Pos::new(x as i64, y as i64);
                    (0..n)
                        .map(move |i| {
                            let current_pos = init_pos + dir.scala_mul(i as i64);
                            let current_x = i64::rem_euclid(current_pos.x, n as i64) as usize;
                            let current_y = i64::rem_euclid(current_pos.y, n as i64) as usize;
                            grid[current_y][current_x]
                        })
                        .fold(0, |acc, x| acc * 10 + x)
                })
            })
            .max()
            .unwrap();

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        let Problem { n, grid } = self;
        let n = *n;
        let dir_list: [Pos<i64>; 8] = [
            Pos::new(1, 0),
            Pos::new(1, 1),
            Pos::new(0, 1),
            Pos::new(-1, 1),
            Pos::new(-1, 0),
            Pos::new(-1, -1),
            Pos::new(0, -1),
            Pos::new(1, -1),
        ];

        let init_pos_list =
            iproduct!(0..n, 0..n).map(|(x, y)| Pos::new(x as i64, y as i64)).collect_vec();

        let ans = iproduct!(dir_list.iter(), init_pos_list.iter())
            .map(|(&dir, &init_pos)| {
                // init_pos から dirの方向にn個の数字を読んだときの値を計算する
                (0..n)
                    .map(move |i| {
                        let current_pos = init_pos + dir.scala_mul(i as i64);
                        let current_x = i64::rem_euclid(current_pos.x, n as i64) as usize;
                        let current_y = i64::rem_euclid(current_pos.y, n as i64) as usize;
                        grid[current_y][current_x]
                    })
                    .fold(0, |acc, x| acc * 10 + x)
            })
            .max()
            .unwrap();

        Answer { ans }
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
    Problem::read(ProconReader::new(stdin().lock())).solve().print();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn check(input: &str, expected: Answer) {
        let actual = Problem::read(ProconReader::new(input.as_bytes())).solve();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_hoge() {
        // rust-analyzer でうまく認識されない
        let x = iproduct!([1, 2], iproduct!([3, 4], [5, 6])).map(|(x, (y, z))| x + y + z).count();
        assert_eq!(x, 8);
    }

    #[test]
    fn test_problem() {
        let _input = "
3
4
        "
        .trim();
        // check(_input, Answer { ans: 7 });
    }
}

// ====== snippet ======

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

        fn read_any_1<T>(&mut self) -> T
        where
            T: std::str::FromStr,
            T::Err: std::fmt::Debug,
        {
            let buf = self.read_line();
            buf.parse::<T>().unwrap()
        }

        fn read_any_2<T0, T1>(&mut self) -> (T0, T1)
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

        fn read_any_3<T0, T1, T2>(&mut self) -> (T0, T1, T2)
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
            self.read_any_1::<i64>()
        }

        fn read_i64_2(&mut self) -> (i64, i64) {
            self.read_any_2::<i64, i64>()
        }

        fn read_i64_3(&mut self) -> (i64, i64, i64) {
            self.read_any_3::<i64, i64, i64>()
        }

        fn read_usize_1(&mut self) -> usize {
            self.read_any_1::<usize>()
        }

        fn read_usize_2(&mut self) -> (usize, usize) {
            self.read_any_2::<usize, usize>()
        }

        fn read_usize_3(&mut self) -> (usize, usize, usize) {
            self.read_any_3::<usize, usize, usize>()
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
