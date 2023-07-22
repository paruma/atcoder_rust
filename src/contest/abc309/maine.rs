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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Insurance {
    person: usize,
    generation: i64,
}

struct Problem {
    n_people: usize,
    n_insurances: usize,
    family_info: Vec<usize>,
    insurances: Vec<Insurance>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: i64,
}

impl Problem {
    fn read<R: Reader>(mut r: R) -> Problem {
        let (n_people, n_insurances) = r.read_any2::<usize, usize>();
        let family_info = r
            .read_vec_any::<usize>()
            .iter()
            .map(|x| x - 1) // 0-origin にする
            .collect_vec();
        let insurances = (0..n_insurances)
            .map(|_| {
                let (person, generation) = r.read_any2::<usize, i64>();
                let person = person - 1; // 0-origin にする
                Insurance { person, generation }
            })
            .collect_vec();
        Problem {
            n_people,
            n_insurances,
            family_info,
            insurances,
        }
    }
    // 必要に応じてダミーデータでassertを書くのをする。
    fn solve(self) -> Answer {
        let mut adj: Vec<Vec<usize>> = vec![vec![]; self.n_people];
        for i in 0..self.n_people - 1 {
            let child = i + 1;
            let parent = self.family_info[i];
            adj[parent].push(child);
        }

        // gen[i]: 人i かその祖先が持っている保険で何世代下まで保険補償できるか
        // -1 は保険がないことを表す
        let mut gen: Vec<i64> = vec![-1; self.n_people];
        for &insurance in &self.insurances {
            gen[insurance.person] = i64::max(insurance.generation, gen[insurance.person]);
        }

        let mut visited = vec![false; self.n_people];
        let mut open: VecDeque<usize> = VecDeque::new();
        open.push_front(0);
        while !open.is_empty() {
            let current_idx = open.pop_back().unwrap();
            for &next_idx in &adj[current_idx] {
                gen[next_idx] = i64::max(gen[current_idx] - 1, gen[next_idx]);
                open.push_front(next_idx);
            }
        }
        let ans = gen.iter().filter(|g| **g >= 0).count() as i64;
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
        7 3
        1 2 1 3 3 3
        1 1
        1 2
        4 3"
        .trim();
        check(input, Answer { ans: 4 });
    }

    #[test]
    fn test2() {
        let input = "
        10 10
        1 1 3 1 2 3 3 5 7
        2 1
        5 1
        4 3
        6 3
        2 1
        7 3
        9 2
        1 2
        6 2
        8 1"
        .trim();
        check(input, Answer { ans: 10 });
    }
}
