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

use pos::*;
pub mod pos {
    use std::ops::{Add, Sub};
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
struct Problem {
    height: usize,
    width: usize,
    grid: Vec<Vec<u8>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: usize,
}

trait VecVecAt<T> {
    fn at(&self, pos: Pos<i64>) -> &T;
    fn at_mut(&mut self, pos: Pos<i64>) -> &mut T;
}

impl<T> VecVecAt<T> for Vec<Vec<T>> {
    fn at(&self, pos: Pos<i64>) -> &T {
        &self[pos.y as usize][pos.x as usize]
    }

    fn at_mut(&mut self, pos: Pos<i64>) -> &mut T {
        &mut self[pos.y as usize][pos.x as usize]
    }
}

impl Problem {
    fn read<R: Reader>(mut r: R) -> Problem {
        // TODO: read_usize2 がほしい
        let (height, width) = r.read_any2::<usize, usize>();
        let grid = (0..height).map(|_| r.read_line().as_bytes().to_vec()).collect_vec();
        Problem { height, width, grid }
    }
    fn is_ice(&self, pos: Pos<i64>) -> bool {
        *self.grid.at(pos) == b'.'
    }
    fn is_rock(&self, pos: Pos<i64>) -> bool {
        *self.grid.at(pos) == b'#'
    }

    // 必要に応じてダミーデータでassertを書くのをする。
    fn solve(&self) -> Answer {
        // 初期値(1, 1)
        let init_pos: Pos<i64> = Pos::new(1, 1);
        let dir = [Pos::new(1, 0), Pos::new(-1, 0), Pos::new(0, 1), Pos::new(0, -1)];
        let mut visited_all = vec![vec![false; self.width]; self.height];
        let mut visited_stop = vec![vec![false; self.width]; self.height];

        let mut open: VecDeque<Pos<i64>> = VecDeque::new();
        open.push_front(init_pos);
        *visited_all.at_mut(init_pos) = true;
        *visited_stop.at_mut(init_pos) = true;

        // while let でリファクタリングできそう
        while !open.is_empty() {
            let current_pos = open.pop_back().unwrap();
            for &d in &dir {
                let mut moving_pos = current_pos;

                // current_pos から dの方向に進む。
                // 岩にぶつかるまで進む
                loop {
                    // TODO += 実装していないので後で実装する
                    let next_pos = moving_pos + d;
                    if self.is_ice(next_pos) {
                        moving_pos = next_pos;
                        *visited_all.at_mut(moving_pos) = true;
                    } else {
                        //岩
                        break;
                    }
                }
                // 岩にぶつかるまで移動した
                if !visited_stop.at(moving_pos) {
                    *visited_stop.at_mut(moving_pos) = true; // 不要
                    open.push_front(moving_pos);
                }
            }
        }
        // flatten 使っても良かったかも
        // visited_all.iter().flatten().filter(|x| **x).count();
        let ans = visited_all.iter().map(|row| row.iter().filter(|x| **x).count()).sum::<usize>();
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
3
4
        "
        .trim();
        check(input, Answer { ans: 7 });
    }
}
