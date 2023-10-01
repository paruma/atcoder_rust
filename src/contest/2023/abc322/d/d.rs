use std::io::stdin;

struct Problem {
    mino_list: Vec<Grid>,
}

struct MinoInfo {
    mino: Grid,
    x_min: usize,
    x_max: usize,
    y_min: usize,
    y_max: usize,
}

impl MinoInfo {
    fn new(mino: Grid) -> MinoInfo {
        if (0_usize..4).filter(|&x| (0_usize..4).any(|y| mino[y][x] == b'#')).count() == 0 {
            dbg!("何かがおかしい");
        }
        let x_min =
            (0_usize..4).filter(|&x| (0_usize..4).any(|y| mino[y][x] == b'#')).min().unwrap();
        let x_max =
            (0_usize..4).filter(|&x| (0_usize..4).any(|y| mino[y][x] == b'#')).max().unwrap();

        let y_min =
            (0_usize..4).filter(|&y| (0_usize..4).any(|x| mino[y][x] == b'#')).min().unwrap();
        let y_max =
            (0_usize..4).filter(|&y| (0_usize..4).any(|x| mino[y][x] == b'#')).max().unwrap();

        MinoInfo { mino, x_min, x_max, y_min, y_max }
    }
}

type Grid = Vec<Vec<u8>>;

fn rotate(mino: &Grid) -> Grid {
    let mut next = vec![vec![b'-'; 4]; 4];
    for y in 0..4 {
        for x in 0..4 {
            next[y][x] = mino[x][3 - y];
        }
    }
    next
}

impl Problem {
    fn read<R: IProconReader>(mut r: R) -> Problem {
        let mino_list =
            (0..3).map(|_| ((0..4).map(|_| r.read_bytes()).collect_vec())).collect_vec();
        Problem { mino_list }
    }
    fn solve(&self) -> Answer {
        let mino_list = &self.mino_list;
        // 回転を入れる
        let mino_list = mino_list
            .iter()
            .map(|mino| {
                let mino0 = mino.clone();
                let mino1 = rotate(&mino0);
                let mino2 = rotate(&mino1);
                let mino3 = rotate(&mino2);
                [mino0, mino1, mino2, mino3].map(|mino| MinoInfo::new(mino))
            })
            .collect_vec();

        let ans = mino_list.iter().multi_cartesian_product().any(|minos| {
            assert!(minos.len() == 3);
            let mut is_ok = false;
            'loop0: for y0 in 0..4 - minos[0].y_max + minos[0].y_min {
                for x0 in 0..4 - minos[0].x_max + minos[0].x_min {
                    'loop1: for y1 in 0..4 - minos[1].y_max + minos[1].y_min {
                        for x1 in 0..4 - minos[1].x_max + minos[1].x_min {
                            //
                            'loop2: for y2 in 0..4 - minos[2].y_max + minos[2].y_min {
                                for x2 in 0..4 - minos[2].x_max + minos[2].x_min {
                                    // ここから塗り始める
                                    let mut grid = vec![vec![b'.'; 4]; 4];
                                    for ry0 in 0..=minos[0].y_max - minos[0].y_min {
                                        for rx0 in 0..=minos[0].x_max - minos[0].x_min {
                                            grid[y0 + ry0][x0 + rx0] = minos[0].mino
                                                [minos[0].y_min + ry0][minos[0].x_min + rx0];
                                        }
                                    }

                                    for ry1 in 0..=minos[1].y_max - minos[1].y_min {
                                        for rx1 in 0..=minos[1].x_max - minos[1].x_min {
                                            if grid[y1 + ry1][x1 + rx1] == b'#'
                                                && minos[1].mino[minos[1].y_min + ry1]
                                                    [minos[1].x_min + rx1]
                                                    == b'#'
                                            {
                                                continue 'loop1;
                                            }
                                            if grid[y1 + ry1][x1 + rx1] == b'.' {
                                                grid[y1 + ry1][x1 + rx1] = minos[1].mino
                                                    [minos[1].y_min + ry1][minos[1].x_min + rx1];
                                            }
                                        }
                                    }

                                    for ry2 in 0..=minos[2].y_max - minos[2].y_min {
                                        for rx2 in 0..=minos[2].x_max - minos[2].x_min {
                                            if grid[y2 + ry2][x2 + rx2] == b'#'
                                                && minos[2].mino[minos[2].y_min + ry2]
                                                    [minos[2].x_min + rx2]
                                                    == b'#'
                                            {
                                                continue 'loop2;
                                            }
                                            if grid[y2 + ry2][x2 + rx2] == b'.' {
                                                grid[y2 + ry2][x2 + rx2] = minos[2].mino
                                                    [minos[2].y_min + ry2][minos[2].x_min + rx2];
                                            }
                                        }
                                    }
                                    //dbg!(&grid);
                                    if grid.iter().flatten().filter(|&&ch| ch != b'.').count() == 16
                                    {
                                        is_ok = true;
                                    }
                                    // ここまでたどり着いたらOK
                                }
                            }
                        }
                    }
                }
            }

            is_ok
        });

        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: bool,
}

impl Answer {
    fn print(&self) {
        let msg = if self.ans { "Yes" } else { "No" };
        println!("{}", msg);
    }
}

fn main() {
    Problem::read(ProconReader::new(stdin().lock())).solve().print();
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
use pos::*;
pub mod pos {
    use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};
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
    impl<T: Neg<Output = T>> Neg for Pos<T> {
        type Output = Self;
        fn neg(self) -> Self::Output {
            Pos::new(-self.x, -self.y)
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
    impl<T: Add<Output = T> + Copy> AddAssign for Pos<T> {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }
    impl<T: Sub<Output = T> + Copy> SubAssign for Pos<T> {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }
}

use vec_vec_at::*;
pub mod vec_vec_at {
    use super::pos::*;
    use easy_ext::ext;
    #[ext]
    impl<T> Vec<Vec<T>> {
        pub fn at(&self, pos: Pos<i64>) -> &T {
            &self[pos.y as usize][pos.x as usize]
        }
        pub fn at_mut(&mut self, pos: Pos<i64>) -> &mut T {
            &mut self[pos.y as usize][pos.x as usize]
        }
    }
}

use itertools::Itertools;
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
