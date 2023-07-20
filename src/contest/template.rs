#[allow(unused_imports)]
use myio::*;
pub mod myio {
    use std::io;

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

    #[derive(Default)]
    pub struct StdReader {}
    impl StdReader {
        pub fn new() -> StdReader {
            StdReader {}
        }
    }
    impl Reader for StdReader {
        fn read_line(&mut self) -> String {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            buffer.trim().to_string()
        }
    }

    pub struct TextReader {
        lines: Vec<String>,
        current_line: usize,
    }

    impl TextReader {
        pub fn new(s: &str) -> Self {
            TextReader {
                lines: s.lines().map(|s| s.to_string()).collect(),
                current_line: 0,
            }
        }
    }

    impl Reader for TextReader {
        fn read_line(&mut self) -> String {
            if self.current_line < self.lines.len() {
                let line = self.lines[self.current_line].clone();
                self.current_line += 1;
                line
            } else {
                panic!("cannot read line");
            }
        }
    }
}

struct Problem {
    a: i64,
    b: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: i64,
}

impl Problem {
    fn read<R: Reader>(mut r: R) -> Problem {
        let a = r.read_i64_1();
        let b = r.read_i64_1();
        Problem { a, b }
    }
    // 必要に応じてダミーデータでassertを書くのをする。
    fn solve(self) -> Answer {
        Answer {
            ans: self.a + self.b,
        }
    }
}

impl Answer {
    fn print(self) {
        println!("{}", self.ans)
    }
}

fn main() {
    Problem::read(StdReader::new()).solve().print();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, expected: Answer) {
        let actual = Problem::read(TextReader::new(input)).solve();
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
