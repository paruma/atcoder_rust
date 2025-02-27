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
            buf.trim().split(' ').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>()
        }

        fn read_vec_str(&mut self) -> Vec<String> {
            let buf = self.read_line();
            buf.trim().split(' ').map(|s| s.to_string()).collect::<Vec<String>>()
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
            buf.trim().split(' ').map(|s| s.parse::<T>().unwrap()).collect::<Vec<T>>()
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
    n_people: usize,
    day_len: usize,
    schedule: Vec<Vec<bool>>, // schedule[i][j]: 人i がj日目に暇なら true
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: i64,
}

impl Problem {
    fn read<R: Reader>(mut r: R) -> Problem {
        let (n_people, day_len) = r.read_any2::<usize, usize>();
        let schedule = (0..n_people)
            .map(|_| r.read_line().as_bytes().iter().map(|c| *c == b'o').collect_vec())
            .collect_vec();
        Problem { n_people, day_len, schedule }
    }
    // 必要に応じてダミーデータでassertを書くのをする。
    fn solve(self) -> Answer {
        // free[i]: i日目がみんな暇
        let free = (0..self.day_len)
            .map(|day| (0..self.n_people).all(|person| self.schedule[person][day]))
            .collect_vec();

        let mut max_cnt = 0;
        let mut cnt = 0;
        for is_free_day in free {
            if is_free_day {
                cnt += 1;
            } else {
                max_cnt = i64::max(max_cnt, cnt);
                cnt = 0;
            }
            // ここでmax_cnt の更新をしても良かった
        }
        max_cnt = i64::max(max_cnt, cnt);
        // group_by を使うと良かった？

        Answer { ans: max_cnt }
    }

    fn solve2(self) -> Answer {
        // dedup_with_count を使った解法
        // free[i]: i日目がみんな暇
        let free = (0..self.day_len)
            .map(|day| (0..self.n_people).all(|person| self.schedule[person][day]))
            .collect_vec();

        // trueが連続している区間の最大の長さ
        let ans = free
            .iter()
            .dedup_with_count()
            .filter(|(_cnt, is_free)| **is_free)
            .map(|(cnt, _is_free)| cnt)
            .max()
            .unwrap_or(0) as i64;

        Answer { ans }
    }
}

impl Answer {
    fn print(self) {
        println!("{}", self.ans)
    }
}

fn main() {
    Problem::read(stdin().lock()).solve2().print();
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
