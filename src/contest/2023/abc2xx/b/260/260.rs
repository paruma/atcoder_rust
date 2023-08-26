use std::{cmp::Reverse, collections::HashSet, io::stdin};

struct Problem {
    n: usize,
    x: usize, // 数学の点が高い方から x 人を合格とする。
    y: usize, // まだ合格となっていない受験者のうち、英語の点が高い方から y 人を合格とする。
    z: usize, // まだ合格となっていない受験者のうち、数学と英語の合計点が高い方から z 人を合格とする。
    math_points: Vec<i64>,
    english_points: Vec<i64>,
}

impl Problem {
    fn read<R: IProconReader>(mut r: R) -> Problem {
        let (n, x, y, z) = r.read_usize_4();
        let math_points = r.read_vec_i64();
        let english_points = r.read_vec_i64();
        Problem { n, x, y, z, math_points, english_points }
    }
    fn solve(&self) -> Answer {
        let Problem { n, x, y, z, math_points, english_points } = self;
        let math_plus_english_point = Iterator::zip(math_points.iter(), english_points.iter())
            .map(|(math, english)| math + english)
            .collect_vec();
        let mut examinee_set = (0..*n).collect::<HashSet<_>>(); // まだ合格が確定していない人の集まり
        let mut admission_set = HashSet::<usize>::new(); // 合格者の集まり

        // 数学の点が高い方から x 人を合格とする。
        let admission_list1 = examinee_set
            .iter()
            .map(|&i| (i, math_points[i]))
            .sorted_by_key(|p| (Reverse(p.1), p.0))
            .map(|p| p.0)
            .take(*x)
            .collect_vec();
        for i in admission_list1 {
            examinee_set.remove(&i);
            admission_set.insert(i);
        }

        // まだ合格となっていない受験者のうち、英語の点が高い方から y 人を合格とする。
        let admission_list2 = examinee_set
            .iter()
            .map(|&i| (i, english_points[i]))
            .sorted_by_key(|p| (Reverse(p.1), p.0))
            .map(|p| p.0)
            .take(*y)
            .collect_vec();

        for i in admission_list2 {
            examinee_set.remove(&i);
            admission_set.insert(i);
        }

        // まだ合格となっていない受験者のうち、数学と英語の合計点が高い方から z 人を合格とする。
        let admission_list3 = examinee_set
            .iter()
            .map(|&i| (i, math_plus_english_point[i]))
            .sorted_by_key(|p| (Reverse(p.1), p.0))
            .map(|p| p.0)
            .take(*z)
            .collect_vec();

        for i in admission_list3 {
            examinee_set.remove(&i);
            admission_set.insert(i);
        }

        let ans = admission_set.iter().map(|i| *i + 1).sorted().collect_vec();
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        let Problem { n, x, y, z, math_points, english_points } = self;
        let mut passed = vec![false; *n];

        // 数学の点が高い方から x 人を合格とする。
        let passed_list1 = (0..*n)
            .filter(|&i| !passed[i])
            .map(|i| (i, math_points[i]))
            .sorted_by_key(|p| (Reverse(p.1), p.0))
            .map(|p| p.0)
            .take(*x)
            .collect_vec();
        for i in passed_list1 {
            passed[i] = true;
        }

        // まだ合格となっていない受験者のうち、英語の点が高い方から y 人を合格とする。
        let passed_list2 = (0..*n)
            .filter(|&i| !passed[i])
            .map(|i| (i, english_points[i]))
            .sorted_by_key(|p| (Reverse(p.1), p.0))
            .map(|p| p.0)
            .take(*y)
            .collect_vec();
        for i in passed_list2 {
            passed[i] = true;
        }

        // まだ合格となっていない受験者のうち、数学と英語の合計点が高い方から z 人を合格とする。
        let passed_list3 = (0..*n)
            .filter(|&i| !passed[i])
            .map(|i| (i, math_points[i] + english_points[i]))
            .sorted_by_key(|p| (Reverse(p.1), p.0))
            .map(|p| p.0)
            .take(*z)
            .collect_vec();
        for i in passed_list3 {
            passed[i] = true;
        }

        let ans = (0..*n).filter(|&i| passed[i]).map(|i| i + 1).collect_vec();
        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        let Problem { n, x, y, z, math_points, english_points } = self;
        let mut passed = vec![false; *n];

        let mut decide_passed = |points: &[i64], count: usize| {
            // points で定まる点数の上位count人は合格にする
            let passed_list = (0..*n)
                .filter(|&i| !passed[i])
                .map(|i| (i, points[i]))
                .sorted_by_key(|p| (Reverse(p.1), p.0))
                .map(|p| p.0)
                .take(count)
                .collect_vec();
            for i in passed_list {
                passed[i] = true;
            }
        };
        let math_english_points = izip!(math_points, english_points)
            .map(|(math, english)| *math + *english)
            .collect_vec();

        decide_passed(math_points, *x);
        decide_passed(english_points, *y);
        decide_passed(&math_english_points, *z);

        let ans = (0..*n).filter(|&i| passed[i]).map(|i| i + 1).collect_vec();
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<usize>,
}

impl Answer {
    fn print(&self) {
        for &x in &self.ans {
            println!("{}", x);
        }
    }
}

fn main() {
    let xs = vec![1];
    let ys = vec![2];
    Iterator::zip(xs.iter(), ys.iter()).count();
    izip!(xs, ys).count();
    Problem::read(ProconReader::new(stdin().lock())).solve3().print();
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
