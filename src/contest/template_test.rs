use std::io::stdin;

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
            buf.trim()
                .split(' ')
                .map(|s| s.parse::<T>().unwrap())
                .collect::<Vec<T>>()
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

struct Problem {
    a: i64,
    b: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: i64,
}

impl Problem {
    fn read<R: IProconReader>(mut r: R) -> Problem {
        let a = r.read_i64_1();
        let b = r.read_i64_1();
        Problem { a, b }
    }
    fn solve(&self) -> Answer {
        let ans = self.a + self.b;
        Answer { ans }
    }
}

impl Answer {
    fn print(&self) {
        println!("{}", self.ans)
    }
}

fn main() {
    Problem::read(ProconReader::new(stdin().lock()))
        .solve()
        .print();
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

    #[test]
    fn test_reader() {
        let input = "
        hoge fuga piyo
        hoge2
        1
        2 3
        4 5 6
        7 8 9 10
        11 12 13 14
        15 16 17 18
        hoge3 hoge4
        19
        20 21
        22 23 24
        25
        26 27
        28 29 30"
            .trim();

        let mut r = ProconReader::new(input.as_bytes());
        assert_eq!(r.read_line(), "hoge fuga piyo".to_string());
        assert_eq!(r.read_bytes(), vec![b'h', b'o', b'g', b'e', b'2']);
        assert_eq!(r.read_any1::<i64>(), 1_i64);
        assert_eq!(r.read_any2::<i64, i64>(), (2, 3));
        assert_eq!(r.read_any3::<i64, i64, i64>(), (4, 5, 6));
        assert_eq!(r.read_vec_any::<i64>(), vec![7, 8, 9, 10]);
        assert_eq!(r.read_vec_i64(), vec![11, 12, 13, 14]);
        assert_eq!(r.read_vec_usize(), vec![15, 16, 17, 18]);
        assert_eq!(
            r.read_vec_str(),
            vec!["hoge3".to_string(), "hoge4".to_string()]
        );
        assert_eq!(r.read_i64_1(), 19);
        assert_eq!(r.read_i64_2(), (20, 21));
        assert_eq!(r.read_i64_3(), (22, 23, 24));
        assert_eq!(r.read_usize_1(), 25);
        assert_eq!(r.read_usize_2(), (26, 27));
        assert_eq!(r.read_usize_3(), (28, 29, 30));
    }
}
