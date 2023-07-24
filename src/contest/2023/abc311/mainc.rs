use std::{collections::VecDeque, io::stdin};

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
    n_v: usize,
    vs: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    n_v: usize,
    vs: Vec<usize>,
}

impl Problem {
    fn read<R: Reader>(mut r: R) -> Problem {
        let n_v = r.read_any1::<usize>();
        let vs = r
            .read_vec_any::<usize>()
            .iter()
            .map(|v| v - 1)
            .collect_vec();

        Problem { n_v, vs }
    }
    fn solve0(&self) -> usize {
        // 閉路の始点を返す
        let mut visited = vec![false; self.n_v];

        for from in 0..self.n_v {
            // from から探索
            if visited[from] {
                continue;
            }
            let mut open: VecDeque<usize> = VecDeque::new();
            // 訪問する点を追加していく。サイクルになったらこれを返す
            visited[from] = true;
            open.push_front(from);
            // BFS
            while !open.is_empty() {
                let current_idx = open.pop_back().unwrap();
                let next_idx = self.vs[current_idx];
                if !visited[next_idx] {
                    visited[next_idx] = true;
                    open.push_front(next_idx);
                } else {
                    // visited[next_idx] ということは閉路がある。
                    return next_idx;
                }
            }
        }
        panic!(); // 閉路の存在性は保証されている。
    }
    // 必要に応じてダミーデータでassertを書くのをする。
    fn solve(&self) -> Answer {
        let from = self.solve0(); // 閉路の始点
        dbg!(from);
        let mut cycle: Vec<usize> = vec![];
        let mut open: VecDeque<usize> = VecDeque::new();
        // 訪問する点を追加していく。サイクルになったらこれを返す
        open.push_front(from);
        cycle.push(from);
        // BFS
        while !open.is_empty() {
            let current_idx = open.pop_back().unwrap();
            let next_idx = self.vs[current_idx];
            if next_idx == from {
                break;
            } else {
                cycle.push(next_idx);
                open.push_front(next_idx);
            }
        }
        Answer {
            n_v: cycle.len(),
            vs: cycle.clone(),
        }
    }
}

impl Answer {
    fn print(self) {
        println!("{}", self.n_v);
        // 1オリジンにする
        let vs = self.vs.iter().map(|v| v + 1).collect_vec();
        println!("{}", vs.iter().join(" "));
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
        // check(input, Answer { ans: 7 });
    }
}
