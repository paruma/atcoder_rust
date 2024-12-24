//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    h: usize,
    w: usize,
    d: usize,
    grid: Vec<Vec<char>>,
}
use std::ops::{Index, IndexMut};
pub struct Grid {
    pub grid: Vec<Vec<char>>,
    pub h: usize,
    pub w: usize,
}
impl Index<Pos> for Grid {
    type Output = char;
    fn index(&self, index: Pos) -> &Self::Output {
        if self.is_within(index) {
            self.grid.index(index)
        } else {
            &'#'
        }
    }
}
impl IndexMut<Pos> for Grid {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        self.grid.index_mut(index)
    }
}
impl Grid {
    pub fn new(grid: Vec<Vec<char>>) -> Grid {
        let h = grid.len();
        let w = grid[0].len();
        Grid { grid, h, w }
    }
    pub fn is_within(&self, pos: Pos) -> bool {
        let h = self.h as i64;
        let w = self.w as i64;
        0 <= pos.y && pos.y < h && 0 <= pos.x && pos.x < w
    }
    pub fn can_move(&self, pos: Pos) -> bool {
        ['.', 'H'].contains(&self[pos])
    }
    pub fn all_pos_iter(&self) -> impl Iterator<Item = Pos> {
        iproduct!(0..self.h, 0..self.w).map(|(y, x)| Pos::new(x as i64, y as i64))
    }
    pub fn find_pos_of(&self, ch: char) -> Option<Pos> {
        self.all_pos_iter().find(|pos| self[*pos] == ch)
    }
    pub fn encode(&self, pos: Pos) -> usize {
        (pos.y * self.w as i64 + pos.x) as usize
    }
    pub fn decode(&self, i: usize) -> Pos {
        let y = (i / self.w) as i64;
        let x = (i % self.w) as i64;
        Pos::new(x, y)
    }
    pub fn debug(&self) {
        for row in &self.grid {
            eprintln!("{}", row.iter().collect::<String>());
        }
        eprintln!();
    }
    /// pos の部分は背景を灰色にして出力する
    pub fn debug_with_pos(&self, pos: Pos) {
        const GRAY: &str = "\x1b[48;2;127;127;127;37m";
        const RESET: &str = "\x1b[0m";
        for y in 0..self.h {
            let row = (0..self.w)
                .map(|x| {
                    if pos == Pos::new(x as i64, y as i64) {
                        format!("{}{}{}", GRAY, self.grid[y][x], RESET)
                    } else {
                        self.grid[y][x].to_string()
                    }
                })
                .join("");
            eprintln!("{}", row);
        }
        eprintln!();
    }
}

impl Problem {
    fn read() -> Problem {
        input! {
            h: usize,
            w: usize,
            d: usize,
            grid: [Chars; h],
        }
        Problem { h, w, d, grid }
    }

    fn solve(&self) -> Answer {
        let h = self.h;
        let w = self.w;
        let grid = Grid::new(self.grid.clone());

        let mut open: Queue<Pos> = Queue::new();
        let mut visited = vec![vec![false; w]; h];
        let mut dist = vec![vec![i64::MAX; w]; h];

        for p in grid.all_pos_iter().filter(|p| grid[*p] == 'H') {
            if self.d >= 1 {
                open.push(p);
            }
            visited[p] = true;
            dist[p] = 0;
        }

        while let Some(current) = open.pop() {
            for next in Pos::around4_pos_iter(current).filter(|p| grid.can_move(*p)) {
                if !visited[next] {
                    visited[next] = true;
                    dist[next] = dist[current] + 1;
                    if dist[next] < self.d as i64 {
                        open.push(next);
                    }
                }
            }
        }
        let ans = visited.iter().flatten().filter(|p| **p).count();
        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        todo!();
        // let ans = 0;
        // Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: usize,
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
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    struct WrongTestCase {
        problem: Problem,
        main_ans: Answer,
        naive_ans: Answer,
    }

    #[allow(dead_code)]
    fn check(p: &Problem) -> Option<WrongTestCase> {
        let main_ans = p.solve();
        let naive_ans = p.solve_naive();
        if main_ans != naive_ans {
            Some(WrongTestCase {
                problem: p.clone(),
                main_ans,
                naive_ans,
            })
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn make_random_problem(rng: &mut SmallRng) -> Problem {
        todo!()
        // let n = rng.gen_range(1..=10);
        // let p = Problem { _a: n };
        // println!("{:?}", &p);
        // p
    }

    #[allow(unreachable_code)]
    #[test]
    fn test_with_naive() {
        let num_tests = 0;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut rng = SmallRng::seed_from_u64(42);
        // let mut rng = SmallRng::from_entropy();
        let mut wrong_cases: Vec<WrongTestCase> = vec![];
        for _ in 0..num_tests {
            let p = make_random_problem(&mut rng);
            let result = check(&p);
            if let Some(wrong_test_case) = result {
                wrong_cases.push(wrong_test_case);
            }
            if wrong_cases.len() >= max_wrong_case {
                break;
            }
        }

        if !wrong_cases.is_empty() {
            for t in &wrong_cases {
                println!("{:?}", t.problem);
                println!("main ans : {:?}", t.main_ans);
                println!("naive ans: {:?}", t.naive_ans);
                println!();
            }
            println!("{} cases are wrong.", wrong_cases.len());
            panic!();
        }
    }
}

// ====== import ======
#[allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet};

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
    pub fn print_chars(chars: &[char]) {
        let msg = chars.iter().collect::<String>();
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
use mod_queue::*;
pub mod mod_queue {
    use std::collections::VecDeque;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Queue<T> {
        raw: VecDeque<T>,
    }
    impl<T> Queue<T> {
        pub fn new() -> Self {
            Queue {
                raw: VecDeque::new(),
            }
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
        pub fn len(&self) -> usize {
            self.raw.len()
        }
    }
    impl<T> Default for Queue<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}
use pos::*;
pub mod pos {
    use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Pos {
        pub x: i64,
        pub y: i64,
    }
    impl Pos {
        pub fn new(x: i64, y: i64) -> Pos {
            Pos { x, y }
        }
    }
    impl Pos {
        pub fn scala_mul(self, rhs: i64) -> Pos {
            Pos::new(self.x * rhs, self.y * rhs)
        }
    }
    impl Pos {
        pub fn inner_product(self, rhs: Self) -> i64 {
            self.x * rhs.x + self.y * rhs.y
        }
        pub fn norm_square(self) -> i64 {
            self.inner_product(self)
        }
    }
    impl Add for Pos {
        type Output = Pos;
        fn add(self, rhs: Self) -> Self::Output {
            Pos::new(self.x + rhs.x, self.y + rhs.y)
        }
    }
    impl Sub for Pos {
        type Output = Pos;
        fn sub(self, rhs: Self) -> Self::Output {
            Pos::new(self.x - rhs.x, self.y - rhs.y)
        }
    }
    impl Neg for Pos {
        type Output = Self;
        fn neg(self) -> Self::Output {
            Pos::new(-self.x, -self.y)
        }
    }
    impl num_traits::Zero for Pos {
        fn zero() -> Self {
            Pos::new(0, 0)
        }
        fn is_zero(&self) -> bool {
            self.x.is_zero() && self.y.is_zero()
        }
    }
    impl AddAssign for Pos {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }
    impl SubAssign for Pos {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }
    pub const DIR8_LIST: [Pos; 8] = [
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 1 },
        Pos { x: 1, y: 0 },
        Pos { x: 1, y: -1 },
        Pos { x: 0, y: -1 },
        Pos { x: -1, y: -1 },
        Pos { x: -1, y: 0 },
        Pos { x: -1, y: 1 },
    ];
    pub const DIR4_LIST: [Pos; 4] = [
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 0 },
        Pos { x: 0, y: -1 },
        Pos { x: -1, y: 0 },
    ];
    impl Pos {
        pub fn around4_pos_iter(self) -> impl Iterator<Item = Pos> {
            DIR4_LIST.iter().copied().map(move |d| self + d)
        }
        pub fn around8_pos_iter(self) -> impl Iterator<Item = Pos> {
            DIR8_LIST.iter().copied().map(move |d| self + d)
        }
    }
}

use vec_vec_at::*;
pub mod vec_vec_at {
    use super::pos::*;
    use easy_ext::ext;
    use std::ops::{Index, IndexMut};
    #[ext(ExtVecVec)]
    impl<T> Vec<Vec<T>> {
        pub fn width(&self) -> usize {
            if self.is_empty() {
                0
            } else {
                self[0].len()
            }
        }
        pub fn height(&self) -> usize {
            self.len()
        }
        pub fn is_within(&self, pos: Pos) -> bool {
            (0..self.width() as i64).contains(&pos.x) && (0..self.height() as i64).contains(&pos.y)
        }
    }
    impl<T> Index<Pos> for Vec<Vec<T>> {
        type Output = T;
        fn index(&self, index: Pos) -> &Self::Output {
            if cfg!(debug_assertions) && !self.is_within(index) {
                panic ! ("index out of bounds: the size (w, h) is ({}, {}) but the index (x, y) is ({}, {})" , self . width () , self . height () , index . x , index . y );
            }
            &self[index.y as usize][index.x as usize]
        }
    }
    impl<T> IndexMut<Pos> for Vec<Vec<T>> {
        fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
            if cfg!(debug_assertions) && !self.is_within(index) {
                panic ! ("index out of bounds: the size (w, h) is ({}, {}) but the index (x, y) is ({}, {})" , self . width () , self . height () , index . x , index . y );
            }
            &mut self[index.y as usize][index.x as usize]
        }
    }
}
