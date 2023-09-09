use std::{collections::VecDeque, io::stdin};

struct Problem {
    h: usize,
    w: usize,
    grid: Vec<Vec<u8>>,
}

impl Problem {
    fn read<R: IProconReader>(mut r: R) -> Problem {
        let (h, w) = r.read_usize_2();
        let grid = (0..h).map(|_| r.read_bytes()).collect_vec();
        Problem { h, w, grid }
    }

    fn is_within1(&self, y: i64, x: i64) -> bool {
        let h = self.h as i64;
        let w = self.w as i64;
        0 <= y && y < h && 0 <= x && x < w
    }

    fn solve(&self) -> Answer {
        let Problem { h, w, grid } = self;
        let h = *h;
        let w = *w;
        let dir = [Pos::new(1, 0), Pos::new(-1, 0), Pos::new(0, 1), Pos::new(0, -1)];
        let mut grid = grid.clone();
        // 視線の処理
        for (y, x) in iproduct!(0..h, 0..w) {
            let y = y as i64;
            let x = x as i64;
            let d_opt: Option<Pos<i64>> = match grid[y as usize][x as usize] {
                b'>' => Some(Pos::new(1, 0)),
                b'<' => Some(Pos::new(-1, 0)),
                b'v' => Some(Pos::new(0, 1)),
                b'^' => Some(Pos::new(0, -1)),
                _ => None,
            };
            if let Some(d) = d_opt {
                // (x,y) から dの方向に進む
                let mut current_x = x;
                let mut current_y = y;
                loop {
                    current_x += d.x;
                    current_y += d.y;
                    if self.is_within1(current_y, current_x)
                        && [b'.', b'!'].contains(&grid[current_y as usize][current_x as usize])
                    {
                        grid[current_y as usize][current_x as usize] = b'!';
                    } else {
                        break;
                    }
                }
            }
        }

        // BFS
        let mut open: VecDeque<Pos<i64>> = VecDeque::new();

        let init_pos = iproduct!(0..h, 0..w).find(|(y, x)| grid[*y][*x] == b'S').unwrap();
        let init_pos = Pos::new(init_pos.1 as i64, init_pos.0 as i64);
        open.push_front(init_pos);

        let mut visited = vec![vec![false; w]; h];
        let mut cnt = vec![vec![12_000_000_000_i64; w]; h];
        visited[init_pos.y as usize][init_pos.x as usize] = true;
        cnt[init_pos.y as usize][init_pos.x as usize] = 0;

        while !open.is_empty() {
            let current_pos = open.pop_back().unwrap();
            for &d in &dir {
                let next_pos = current_pos + d;
                if self.is_within1(next_pos.y, next_pos.x)
                    && [b'.', b'G'].contains(&grid[next_pos.y as usize][next_pos.x as usize])
                    && !visited[next_pos.y as usize][next_pos.x as usize]
                {
                    visited[next_pos.y as usize][next_pos.x as usize] = true;
                    cnt[next_pos.y as usize][next_pos.x as usize] =
                        cnt[current_pos.y as usize][current_pos.x as usize] + 1;
                    open.push_front(next_pos);
                }
            }
        }

        let goal_pos = iproduct!(0..h, 0..w).find(|(y, x)| grid[*y][*x] == b'G').unwrap();
        let goal_pos = Pos::new(goal_pos.1, goal_pos.0);
        let ans = cnt[goal_pos.y][goal_pos.x];
        let ans = if ans >= 12_000_000_000_i64 { -1 } else { ans };

        Answer { ans }
    }

    fn is_within2(&self, pos: Pos<i64>) -> bool {
        let h = self.h as i64;
        let w = self.w as i64;
        0 <= pos.y && pos.y < h && 0 <= pos.x && pos.x < w
    }
    fn solve2(&self) -> Answer {
        let Problem { h, w, grid } = self;
        let h = *h;
        let w = *w;
        let dir = [Pos::new(1, 0), Pos::new(-1, 0), Pos::new(0, 1), Pos::new(0, -1)];
        let mut grid = grid.clone();
        // 視線の処理
        for (y, x) in iproduct!(0..h, 0..w) {
            let pos = Pos::new(x as i64, y as i64);
            let d_opt: Option<Pos<i64>> = match *grid.at(pos) {
                b'>' => Some(Pos::new(1, 0)),
                b'<' => Some(Pos::new(-1, 0)),
                b'v' => Some(Pos::new(0, 1)),
                b'^' => Some(Pos::new(0, -1)),
                _ => None,
            };
            if let Some(d) = d_opt {
                // current_pos から d の方向に進む
                let mut current_pos = pos;
                loop {
                    current_pos += d;
                    if self.is_within2(current_pos) && [b'.', b'!'].contains(grid.at(current_pos)) {
                        *grid.at_mut(current_pos) = b'!';
                    } else {
                        break;
                    }
                }
            }
        }

        let search_pos = |ch: u8| {
            iproduct!(0..h, 0..w)
                .find(|(y, x)| grid[*y][*x] == ch)
                .map(|(y, x)| Pos::new(x as i64, y as i64))
        };

        // BFS
        let mut open: VecDeque<Pos<i64>> = VecDeque::new();

        let init_pos = search_pos(b'S').unwrap();
        open.push_front(init_pos);

        let mut visited = vec![vec![false; w]; h];
        let mut cnt = vec![vec![Inf; w]; h];
        *visited.at_mut(init_pos) = true;
        *cnt.at_mut(init_pos) = Fin(0);

        while !open.is_empty() {
            let current_pos = open.pop_back().unwrap();
            for &d in &dir {
                let next_pos = current_pos + d;
                if self.is_within2(next_pos)
                    && [b'.', b'G'].contains(grid.at(next_pos))
                    && !visited.at(next_pos)
                {
                    *visited.at_mut(next_pos) = true;
                    *cnt.at_mut(next_pos) = *cnt.at(current_pos) + Fin(1);
                    open.push_front(next_pos);
                }
            }
        }

        let goal_pos = search_pos(b'G').unwrap();
        let ans = cnt.at(goal_pos).get_fin_or(-1);

        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        let Problem { h, w, grid } = self;
        let h = *h;
        let w = *w;
        let dir = [Pos::new(1, 0), Pos::new(-1, 0), Pos::new(0, 1), Pos::new(0, -1)];

        struct Grid {
            grid: Vec<Vec<u8>>,
            h: usize,
            w: usize,
        }

        impl Grid {
            fn new(grid: Vec<Vec<u8>>) -> Grid {
                let h = grid.len();
                let w = grid[0].len();
                Grid { grid, h, w }
            }

            fn is_within(&self, pos: Pos<i64>) -> bool {
                let h = self.h as i64;
                let w = self.w as i64;
                0 <= pos.y && pos.y < h && 0 <= pos.x && pos.x < w
            }

            fn at(&self, pos: Pos<i64>) -> &u8 {
                if self.is_within(pos) {
                    self.grid.at(pos)
                } else {
                    &b'#'
                }
            }

            fn at_mut(&mut self, pos: Pos<i64>) -> &mut u8 {
                self.grid.at_mut(pos)
            }

            fn sight_can_move(&self, pos: Pos<i64>) -> bool {
                b".!SG".contains(self.at(pos))
            }

            fn player_can_move(&self, pos: Pos<i64>) -> bool {
                b".SG".contains(self.at(pos))
            }
        }

        let mut grid = Grid::new(grid.clone());
        // 視線の処理
        for (y, x) in iproduct!(0..h, 0..w) {
            let trainer_pos = Pos::new(x as i64, y as i64);
            let d_opt: Option<Pos<i64>> = match *grid.at(trainer_pos) {
                b'>' => Some(Pos::new(1, 0)),
                b'<' => Some(Pos::new(-1, 0)),
                b'v' => Some(Pos::new(0, 1)),
                b'^' => Some(Pos::new(0, -1)),
                _ => None,
            };
            if let Some(d) = d_opt {
                // trainer_pos から d の方向に進む
                let mut current_sight_pos = trainer_pos;
                loop {
                    current_sight_pos += d;
                    if grid.sight_can_move(current_sight_pos) {
                        *grid.at_mut(current_sight_pos) = b'!';
                    } else {
                        break;
                    }
                }
            }
        }
        let grid = grid;

        let search_pos = |ch: u8| -> Option<Pos<i64>> {
            iproduct!(0..h, 0..w)
                .map(|(y, x)| Pos::new(x as i64, y as i64))
                .find(|pos| *grid.at(*pos) == ch)
        };

        // BFS
        let mut open: VecDeque<Pos<i64>> = VecDeque::new();

        let init_pos = search_pos(b'S').unwrap();
        open.push_front(init_pos);

        let mut visited = vec![vec![false; w]; h];
        let mut cnt = vec![vec![Inf; w]; h];
        *visited.at_mut(init_pos) = true;
        *cnt.at_mut(init_pos) = Fin(0);

        while !open.is_empty() {
            let current_pos = open.pop_back().unwrap();
            for &d in &dir {
                let next_pos = current_pos + d;
                if grid.player_can_move(next_pos) && !visited.at(next_pos) {
                    *visited.at_mut(next_pos) = true;
                    *cnt.at_mut(next_pos) = *cnt.at(current_pos) + Fin(1);
                    open.push_front(next_pos);
                }
            }
        }

        let goal_pos = search_pos(b'G').unwrap();
        let ans = cnt.at(goal_pos).get_fin_or(-1);

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
use mod_ext_int::ExtInt::{self, *};
pub mod mod_ext_int {
    use std::{cmp::Ordering, ops::Add};
    use ExtInt::*;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ExtInt {
        Inf,
        Fin(i64),
    }
    impl ExtInt {
        pub fn get_fin(self) -> i64 {
            match self {
                Fin(val) => val,
                Inf => panic!("called `ExtInt::get_fin()` on a `Fin` value"),
            }
        }
        pub fn get_fin_or(self, default: i64) -> i64 {
            match self {
                Fin(val) => val,
                Inf => default,
            }
        }
        pub fn is_fin(self) -> bool {
            matches!(self, Fin(_))
        }
        pub fn is_inf(self) -> bool {
            matches!(self, Inf)
        }
        pub fn to_option(self) -> Option<i64> {
            match self {
                Inf => None,
                Fin(a) => Some(a),
            }
        }
        pub fn from_option(opt: Option<i64>) -> ExtInt {
            match opt {
                Some(a) => Fin(a),
                None => Inf,
            }
        }
    }
    impl Add for ExtInt {
        type Output = ExtInt;
        fn add(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (Inf, Inf) => Inf,
                (Inf, Fin(_)) => Inf,
                (Fin(_), Inf) => Inf,
                (Fin(a), Fin(b)) => Fin(a + b),
            }
        }
    }
    impl PartialOrd for ExtInt {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match (self, other) {
                (Inf, Inf) => Some(Ordering::Equal),
                (Inf, Fin(_)) => Some(Ordering::Greater),
                (Fin(_), Inf) => Some(Ordering::Less),
                (Fin(a), Fin(b)) => PartialOrd::partial_cmp(a, b),
            }
        }
    }
    impl Ord for ExtInt {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }
}

use pos::*;
pub mod pos {
    use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};
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
    pub trait VecVecAt<T> {
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
}

use itertools::{iproduct, Itertools};
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
