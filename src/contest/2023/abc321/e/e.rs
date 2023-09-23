use std::io::stdin;

struct TestCase {
    n: i64,
    x: i64,
    k: i64,
}
struct Problem {
    n: usize,
    test_case_list: Vec<TestCase>,
}

fn pow2(i: i64) -> i64 {
    i64::pow(2, i as u32)
}

fn solve0(n: i64, height: i64, is_right: bool) -> i64 {
    // 10^18 = 2^60 くらい
    if height >= 62 {
        return 0;
    } else if height == 0 {
        return 1;
    }

    let (begin, end) = if is_right {
        (pow2(height), pow2(height) / 2 * 3)
    } else {
        (pow2(height) / 2 * 3, pow2(height) * 2)
    };

    if n < begin {
        0
    } else if n < end {
        n - begin + 1
    } else {
        end - begin
    }
}

impl TestCase {
    fn solve(&self) -> i64 {
        let n = self.n;
        let x = self.x;
        let k = self.k;

        if k == 0 {
            if x <= n {
                return 1;
            } else {
                return 0;
            }
        }

        let x_height = x.ilog2() as i64;
        let x_down_height = x_height + k;
        let x_up_height = i64::abs(x_height - k);
        // トップの場合分けに注意

        // x == 1という場合分けが必要。

        // is_down_right
        // x が右側にある
        let x_is_right = x / i64::pow(2, (x_height - 1) as u32) == 2;
        let x_down_is_right = x_is_right;
        let x_up_is_right = {
            if x_is_right {
                x_height - k > 0
            } else {
                x_height - k < 0
            }
        };

        let down_cnt = solve0(n, x_down_height, x_down_is_right);
        let up_cnt = solve0(n, x_up_height, x_up_is_right);

        up_cnt + down_cnt
    }
}

impl Problem {
    fn read<R: IProconReader>(mut r: R) -> Problem {
        let n = r.read_usize_1();
        let test_case_list = (0..n)
            .map(|_| {
                let (n, x, k) = r.read_i64_3();
                TestCase { n, x, k }
            })
            .collect_vec();
        Problem { n, test_case_list }
    }
    fn solve(&self) -> Answer {
        let Problem { n, test_case_list } = self;

        let ans = test_case_list.iter().map(|t| t.solve()).collect_vec();
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<i64>,
}

impl Answer {
    fn print(&self) {
        for &x in &self.ans {
            println!("{}", x);
        }
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

        fn read_i64_4(&mut self) -> (i64, i64, i64, i64) {
            self.read_any_4::<i64, i64, i64, i64>()
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

        fn read_usize_4(&mut self) -> (usize, usize, usize, usize) {
            self.read_any_4::<usize, usize, usize, usize>()
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
