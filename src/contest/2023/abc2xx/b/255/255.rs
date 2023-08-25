use std::io::stdin;

struct Problem {
    n_people: usize,
    n_light_people: usize,
    light_people: Vec<usize>,
    people_pos: Vec<Pos<i64>>,
}

fn dist(p1: Pos<i64>, p2: Pos<i64>) -> f64 {
    let diff = p2 - p1;
    let dx = diff.x as f64;
    let dy = diff.y as f64;

    f64::sqrt(dx * dx + dy * dy)
}

fn dist_square(p1: Pos<i64>, p2: Pos<i64>) -> i64 {
    let diff = p2 - p1;
    diff.x * diff.x + diff.y * diff.y
}

impl Problem {
    fn read<R: IProconReader>(mut r: R) -> Problem {
        let (n_people, n_light_people) = r.read_usize_2();
        let light_people = r.read_vec_usize().iter().map(|i| *i - 1).collect_vec();
        let people_pos = (0..n_people)
            .map(|_| {
                let (x, y) = r.read_i64_2();
                Pos::new(x, y)
            })
            .collect_vec();
        Problem { n_people, n_light_people, light_people, people_pos }
    }
    fn solve(&self) -> Answer {
        let Problem { n_people, n_light_people, light_people, people_pos } = self;
        let light_pos_list = light_people.iter().map(|i| people_pos[*i]).collect_vec();
        let ans = people_pos
            .iter()
            .map(|pos| {
                // pos から最も近いあかりまでの距離を求める
                light_pos_list
                    .iter()
                    .map(|light_pos| dist(*pos, *light_pos))
                    .min_by(|x, y| x.partial_cmp(y).unwrap())
                    .unwrap()
            })
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap();

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        let Problem { n_people, n_light_people, light_people, people_pos } = self;
        let light_pos_list = light_people.iter().map(|i| people_pos[*i]).collect_vec();
        let ans_square = people_pos
            .iter()
            .map(|pos| {
                // pos から最も近いあかりまでの距離を求める
                light_pos_list.iter().map(|light_pos| dist_square(*pos, *light_pos)).min().unwrap()
            })
            .max()
            .unwrap();
        let ans = f64::sqrt(ans_square as f64);

        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Answer {
    ans: f64,
}

impl Answer {
    fn print(&self) {
        println!("{}", self.ans);
    }
}

fn main() {
    Problem::read(ProconReader::new(stdin().lock())).solve2().print();
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

use itertools::Itertools;
// ====== snippet ======
use pos::*;
pub mod pos {
    use std::ops::{Add, Mul, Sub};
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Pos<T> {
        pub x: T,
        pub y: T,
    }
    impl<T> Pos<T> {
        pub fn new(x: T, y: T) -> Pos<T> {
            Pos { x, y }
        }
    }
    impl<T: Mul<Output = T> + Copy> Pos<T> {
        pub fn scala_mul(self, rhs: T) -> Pos<T> {
            Pos::new(self.x * rhs, self.y * rhs)
        }
    }
    impl<T: Add<Output = T> + Mul<Output = T> + Copy> Pos<T> {
        pub fn norm_square(self) -> T {
            self.x * self.x + self.y * self.y
        }
    }
    impl<T: Add<Output = T> + Copy> Add for Pos<T> {
        type Output = Pos<T>;
        fn add(self, rhs: Self) -> Self::Output {
            Pos::new(self.x + rhs.x, self.y + rhs.y)
        }
    }
    impl<T: Sub<Output = T> + Copy> Sub for Pos<T> {
        type Output = Pos<T>;
        fn sub(self, rhs: Self) -> Self::Output {
            Pos::new(self.x - rhs.x, self.y - rhs.y)
        }
    }
    impl<T: num_traits::Zero + Copy> num_traits::Zero for Pos<T> {
        fn zero() -> Self {
            Pos::new(T::zero(), T::zero())
        }
        fn is_zero(&self) -> bool {
            self.x.is_zero() && self.y.is_zero()
        }
    }
}
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
