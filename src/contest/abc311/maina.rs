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
            let a1 = splitted[1].parse::<T1>().unwrap();
            (a0, a1)
        }
        fn read_vec_any<T>(&mut self) -> Vec<T>
        where
            T: std::str::FromStr,
            T::Err: std::fmt::Debug,
        {
            let buf = self.read_line();
            buf.trim()
                .split(' ')
                .map(|s| s.parse::<T>().unwrap())
                .collect::<Vec<T>>()
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

struct Problem {
    n: usize,
    s: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: usize,
}

impl Problem {
    fn read<R: Reader>(mut r: R) -> Problem {
        let n = r.read_i64_1() as usize;
        let s = r.read_line().bytes().collect_vec();
        Problem { n, s }
    }
    // 必要に応じてダミーデータでassertを書くのをする。
    fn solve(self) -> Answer {
        let ia = self
            .s
            .iter()
            .enumerate()
            .find(|(_, c)| **c == b'A')
            .unwrap()
            .0;
        let ib = self
            .s
            .iter()
            .enumerate()
            .find(|(_, c)| **c == b'B')
            .unwrap()
            .0;
        let ic = self
            .s
            .iter()
            .enumerate()
            .find(|(_, c)| **c == b'C')
            .unwrap()
            .0;

        // こう書くとよかった
        // [b'a', b'b', b'c'].map(|ch| self.s.iter().enumerate().find(|(_, ch2)| ch1 == **ch2).unwrap().0).iter().max()

        let ans = usize::max(ia, usize::max(ib, ic)) + 1;
        Answer { ans }
    }
}

impl Answer {
    fn print(self) {
        println!("{}", self.ans)
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
        assert_eq!(expected, actual);
    }

    #[test]
    fn test() {
        let input = "
3
4
        "
        .trim();
        check(input, Answer { ans: 7 });
    }
}
