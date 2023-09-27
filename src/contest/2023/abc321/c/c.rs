use std::io::stdin;

struct Problem {
    k: usize,
}

impl Problem {
    fn read<R: IProconReader>(mut r: R) -> Problem {
        let k = r.read_usize_1();
        Problem { k }
    }
    fn solve(&self) -> Answer {
        let k = self.k;
        let base: [i64; 10] = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let ans = (0_usize..10)
            .powerset()
            .map(|s| s.iter().map(|i| base[*i]).fold(0, |acc, x| acc * 10 + x))
            .filter(|x| *x != 0)
            .sorted()
            .nth(k - 1)
            .unwrap();

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // 初期実装の軽微なリファクタリング
        let k = self.k;
        let base: [i64; 10] = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let ans = base
            .into_iter()
            .powerset()
            .map(|s| s.into_iter().fold(0, |acc, x| acc * 10 + x))
            .filter(|x| *x != 0)
            .sorted()
            .nth(k - 1)
            .unwrap();

        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        // multi_cartesian_product を使った解法
        let k = self.k;
        let base: [i64; 10] = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        // 2^10
        let ans = std::iter::repeat([true, false])
            .take(10)
            .multi_cartesian_product()
            .map(|s| {
                izip!(base, s)
                    .filter_map(|(digit, p)| p.then_some(digit))
                    .fold(0, |acc, x| acc * 10 + x)
            })
            .filter(|&x| x != 0)
            .sorted()
            .nth(k - 1)
            .unwrap();
        Answer { ans }
    }

    fn solve4(&self) -> Answer {
        // bit全探索 を使った解法
        let k = self.k;
        let base: [i64; 10] = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        // 2^10
        let ans = (0..(1 << 10))
            .map(|bit_pattern| {
                let bit_pattern_vec = (0..10).map(|i| (bit_pattern >> i) & 1 == 1).collect_vec();
                izip!(base, bit_pattern_vec)
                    .filter_map(|(digit, p)| p.then_some(digit))
                    .fold(0, |acc, x| acc * 10 + x)
            })
            .filter(|&x| x != 0) //
            .sorted()
            .nth(k - 1)
            .unwrap();
        Answer { ans }
    }

    fn solve5(&self) -> Answer {
        // DFS を使った解法

        struct Dfs {
            size: usize,
            seq_list: Vec<Vec<bool>>,
        }
        impl Dfs {
            fn new(size: usize) -> Dfs {
                Dfs { size, seq_list: vec![] }
            }
            fn exec(&mut self, seq: &mut Vec<bool>) {
                if seq.len() == self.size {
                    // ここがforループの中のようなもの
                    self.seq_list.push(seq.clone());
                    return;
                }

                for p in [true, false] {
                    seq.push(p);
                    self.exec(seq);
                    seq.pop();
                }
            }
        }
        let k = self.k;
        let base: [i64; 10] = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let mut dfs = Dfs::new(10);
        dfs.exec(&mut vec![]);
        let seq_list = dfs.seq_list;
        let ans = seq_list
            .iter()
            .map(|v| {
                izip!(base, v)
                    .filter_map(|(digit, p)| p.then_some(digit))
                    .fold(0, |acc, x| acc * 10 + x)
            })
            .filter(|&x| x != 0)
            .sorted()
            .nth(k - 1)
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
    Problem::read(ProconReader::new(stdin().lock())).solve5().print();
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

use itertools::{izip, Itertools};
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
