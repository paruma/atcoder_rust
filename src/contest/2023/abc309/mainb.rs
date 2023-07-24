use std::io::stdin;

use itertools::Itertools;
#[allow(unused_imports)]
use myio::*;
pub mod myio {
    use std::io::BufRead;

    pub trait Reader {
        fn read_line(&mut self) -> String;

        fn read_vec_i64(&mut self) -> Vec<i64> {
            let buf = self.read_line();
            buf.trim()
                .split(' ')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        }

        fn read_vec_str(&mut self) -> Vec<String> {
            let buf = self.read_line();
            buf.trim()
                .split(' ')
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        }

        fn read_i64_1(&mut self) -> i64 {
            let buf = self.read_line();
            buf.parse::<i64>().unwrap()
        }

        fn read_i64_2(&mut self) -> (i64, i64) {
            let ns = self.read_vec_i64();
            (ns[0], ns[1])
        }

        fn read_i64_3(&mut self) -> (i64, i64, i64) {
            let ns = self.read_vec_i64();
            (ns[0], ns[1], ns[2])
        }

        fn read_i64_4(&mut self) -> (i64, i64, i64, i64) {
            let ns = self.read_vec_i64();
            (ns[0], ns[1], ns[2], ns[3])
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
            let a1 = splitted[0].parse::<T1>().unwrap();
            (a0, a1)
        }
    }

    impl<R: BufRead> Reader for R {
        fn read_line(&mut self) -> String {
            let mut buffer = String::new();
            self.read_line(&mut buffer).unwrap();
            buffer.trim().to_string()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Problem {
    n: i64,
    mat: Vec<Vec<u8>>,
}

impl Problem {
    fn read<R: Reader>(mut r: R) -> Problem {
        let n = r.read_i64_1();
        let mat = (0..n)
            .map(|_| r.read_line().bytes().collect_vec())
            .collect_vec();
        Problem { n, mat }
    }
    // 必要に応じてダミーデータでassertを書くのをする。
    fn solve(self) -> Answer {
        let mut ans_mat = self.mat.clone();
        let n = self.n as usize;

        // 配るバージョン
        // 上の辺
        for x in 0..=n - 2 {
            ans_mat[0][x + 1] = self.mat[0][x];
        }

        // 右の辺
        for y in 0..=n - 2 {
            ans_mat[y + 1][n - 1] = self.mat[y][n - 1];
        }
        // 下の辺
        for x in 1..=n - 1 {
            ans_mat[n - 1][x - 1] = self.mat[n - 1][x];
        }

        // 左の辺
        for y in 1..=n - 1 {
            ans_mat[y - 1][0] = self.mat[y][0];
        }
        Answer { mat: ans_mat }
    }
}

struct Answer {
    mat: Vec<Vec<u8>>,
}

impl Answer {
    fn print(self) {
        self.mat
            .iter()
            .map(|row| String::from_utf8(row.clone()).unwrap())
            .for_each(|row| println!("{}", row));
    }
}

fn main() {
    Problem::read(stdin().lock()).solve().print();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, expected: Answer) {
        let actual = Problem::read(input.as_bytes()).solve();
        // assert_eq!(expected, actual);
    }

    #[test]
    fn test() {}
}
