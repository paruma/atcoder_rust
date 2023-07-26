use std::io::stdin;

#[allow(unused_imports)]
use myio::*;
use num_integer::Integer;
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

        fn read_any_4<T0, T1, T2, T3>(&mut self) -> (T0, T1, T2, T3)
        where
            T0: std::str::FromStr,
            T0::Err: std::fmt::Debug,
            T1: std::str::FromStr,
            T1::Err: std::fmt::Debug,
            T2: std::str::FromStr,
            T2::Err: std::fmt::Debug,
            T3: std::str::FromStr,
            T3::Err: std::fmt::Debug,
        {
            let buf = self.read_line();
            let splitted = buf.trim().split(' ').collect::<Vec<_>>();
            let a0 = splitted[0].parse::<T0>().unwrap();
            let a1 = splitted[1].parse::<T1>().unwrap();
            let a2 = splitted[2].parse::<T2>().unwrap();
            let a3 = splitted[3].parse::<T3>().unwrap();
            (a0, a1, a2, a3)
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

struct Problem {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
}

impl Problem {
    fn read<R: IProconReader>(mut r: R) -> Problem {
        let (a, b, c, d) = r.read_any_4::<i64, i64, i64, i64>();
        Problem { a, b, c, d }
    }
    fn solve(&self) -> Answer {
        let &Problem { a, b, c, d } = self;
        // a: 水色ボールの数
        // k回操作後
        // 水色: a + k*b
        // 赤色: k*c
        // こうなる最小の k>=0 を求めたい: a + k*b <= k*c*d
        // こう変形できる: a <= k*(c*d -b)
        // c*d -b <= 0 のとき: 解なし
        // c*d-b > 0 のとき: a/(c*d-b) <= k
        // ceil(a/(c*d-b))を求めれば良い。

        let ans = if c * d - b <= 0 {
            None
        } else {
            // num_integer::div_ceil
            Some(num_integer::Integer::div_ceil(&a, &(c * d - b)))
        };
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Option<i64>,
}

impl Answer {
    fn print(&self) {
        let msg = self.ans.unwrap_or(-1);
        println!("{}", msg)
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
        // c*d -b = 4
        // a = 5
        let input = "
5 2 3 2
        "
        .trim();
        check(input, Answer { ans: Some(2) });
    }

    #[test]
    fn test_problem2() {
        // c*d -b = 3
        // a = 6
        // 操作
        // 6, 0
        // 9 3
        // 12 6 ←ここで2倍以下
        let input = "
6 3 3 2
        "
        .trim();
        check(input, Answer { ans: Some(2) });
    }

    #[test]
    fn test_problem3() {
        // c*d -b = 0
        // a = 6
        // 操作
        // 6 0
        // 12 3
        // 18 6
        // 24 9
        let input = "
6 6 3 2
        "
        .trim();
        check(input, Answer { ans: None });
    }

    
    #[test]
    fn test_problem4() {
        // c*d -b = -1
        // a = 6
        // 操作
        // 6 0
        // 12 1
        // 24 2
        // ...
        let input = "
6 6 1 2
        "
        .trim();
        check(input, Answer { ans: None });
    }
}
