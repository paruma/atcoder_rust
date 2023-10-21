//#[derive_readable]
struct Problem {
    grid: Grid,
}
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
            &b'.'
        }
    }
}

struct Visited {
    visited: Vec<Vec<bool>>,
    h: usize,
    w: usize,
}

impl Visited {
    fn new(visited: Vec<Vec<bool>>) -> Visited {
        let h = visited.len();
        let w = visited[0].len();
        Visited { visited, h, w }
    }

    fn is_within(&self, pos: Pos<i64>) -> bool {
        let h = self.h as i64;
        let w = self.w as i64;
        0 <= pos.y && pos.y < h && 0 <= pos.x && pos.x < w
    }

    fn at(&self, pos: Pos<i64>) -> &bool {
        if self.is_within(pos) {
            self.visited.at(pos)
        } else {
            &false
        }
    }

    fn at_mut(&mut self, pos: Pos<i64>) -> &mut bool {
        self.visited.at_mut(pos)
    }
}
impl Problem {
    fn read() -> Problem {
        input! {
            h: usize,
            w: usize,
            grid: [Bytes; h],
        }
        Problem { grid: Grid { h, w, grid } }
    }
    fn solve(&self) -> Answer {
        let grid = &self.grid;
        // 8方向のあれ
        let dir_list = [
            Pos::new(0, 1),
            Pos::new(0, -1),
            Pos::new(1, 1),
            Pos::new(1, 0),
            Pos::new(1, -1),
            Pos::new(-1, 1),
            Pos::new(-1, 0),
            Pos::new(-1, -1),
        ];

        let mut visited = Visited::new(vec![vec![false; grid.w]; grid.h]);

        let mut cnt = 0; //連結成分の数

        for (init_y, init_x) in iproduct!(0..grid.h, 0..grid.w) {
            let init_pos = Pos::new(init_x as i64, init_y as i64);
            if *grid.at(init_pos) != b'#' || *visited.at(init_pos) {
                continue;
            }
            // 未訪問かつ init_pos にセンサがある
            cnt += 1;
            let mut open: Queue<Pos<i64>> = Queue::new();
            *visited.at_mut(init_pos) = true;
            open.push(init_pos);
            while let Some(current_pos) = open.pop() {
                // dbg!(current_pos);
                for &d in &dir_list {
                    let next_pos = current_pos + d;
                    if *grid.at(next_pos) == b'#' && !visited.at(next_pos) {
                        open.push(next_pos);
                        *visited.at_mut(next_pos) = true;
                    }
                }
            }
        }
        let ans = cnt;
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
    Problem::read().solve().print();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }
}

// ====== import ======
use itertools::iproduct;
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};

// ====== output func ======
#[allow(unused_imports)]
use print_vec::*;
pub mod print_vec {
    use itertools::Itertools;
    use proconio::fastout;
    #[fastout]
    pub fn print_vec<T: std::fmt::Debug>(arr: &[T]) {
        for a in arr {
            println!("{:?}", a);
        }
    }
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Debug>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{:?}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Debug>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{:?}", x)).join(" ");
            println!("{}", msg);
        }
    }
    pub fn print_bytes(bytes: &[u8]) {
        let msg = String::from_utf8(bytes.to_vec()).unwrap();
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec_bytes(vec_bytes: &[Vec<u8>]) {
        for row in vec_bytes {
            let msg = String::from_utf8(row.to_vec()).unwrap();
            println!("{}", msg);
        }
    }
}

#[allow(unused)]
fn print_yesno(ans: bool) {
    let msg = if ans { "Yes" } else { "No" };
    println!("{}", msg);
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

use mod_queue::*;
pub mod mod_queue {
    use std::collections::VecDeque;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Queue<T> {
        raw: VecDeque<T>,
    }
    impl<T> Queue<T> {
        pub fn new() -> Self {
            Queue { raw: VecDeque::new() }
        }
        pub fn push(&mut self, value: T) {
            self.raw.push_front(value)
        }
        pub fn pop(&mut self) -> Option<T> {
            self.raw.pop_back()
        }
        pub fn peek(&self) -> Option<&T> {
            self.raw.back()
        }
        pub fn is_empty(&self) -> bool {
            self.raw.is_empty()
        }
    }
    impl<T> Default for Queue<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}
