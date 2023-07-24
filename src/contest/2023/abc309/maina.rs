use std::io::stdin;

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

struct Problem {
    a: i64,
    b: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: bool,
}

impl Problem {
    fn read<R: Reader>(mut r: R) -> Problem {
        let (a, b) = r.read_i64_2();
        Problem { a, b }
    }
    // 必要に応じてダミーデータでassertを書くのをする。
    fn solve(self) -> Answer {
        let a = self.a - 1;
        let b = self.b - 1;

        assert!(a < b);

        let ax = a % 3;
        let bx = b % 3;

        let ay = a / 3;
        let by = b / 3;

        Answer {
            ans: ay == by && bx - ax == 1,
        }
    }
}

impl Answer {
    fn print(self) {
        let msg = if self.ans { "Yes" } else { "No" };
        println!("{}", msg);
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
5 6
        "
        .trim();
        check(input, Answer { ans: true });
    }

    #[test]
    fn test2() {
        let input = "
3 4
        "
        .trim();
        check(input, Answer { ans: false });
    }

    #[test]
    fn test3() {
        let input = "
4 6
        "
        .trim();
        check(input, Answer { ans: false });
    }
}
