use std::io::stdin;

struct Problem {
    h: usize,
    w: usize,
    grid: Vec<Vec<u8>>,
}

fn l1_dist(p0: (usize, usize), p1: (usize, usize)) -> i64 {
    let p0y = p0.0 as i64;
    let p0x = p0.1 as i64;
    let p1y = p1.0 as i64;
    let p1x = p1.1 as i64;
    let dy = i64::abs(p1y - p0y);
    let dx = i64::abs(p1x - p0x);
    dx + dy
}

impl Problem {
    fn read<R: IProconReader>(mut r: R) -> Problem {
        let (h, w) = r.read_usize_2();
        let grid = (0..h).map(|_| r.read_bytes()).collect_vec();
        Problem { h, w, grid }
    }
    fn solve(&self) -> Answer {
        let Problem { h, w, grid } = self;
        // コマのある点を求める。2つある。
        let pos_list = iproduct!(0..*h, 0..*w).filter(|(y, x)| grid[*y][*x] == b'o').collect_vec();
        assert!(pos_list.len() == 2);
        let ans = l1_dist(pos_list[0], pos_list[1]);
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
