//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    height: usize,
    width: usize,
    grid: Vec<Vec<u8>>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            height: usize,
            width: usize,
            grid: [Bytes; height]
        }
        Problem {
            height,
            width,
            grid,
        }
    }

    fn is_ice(&self, pos: Pos<i64>) -> bool {
        *self.grid.at(pos) == b'.'
    }
    fn is_rock(&self, pos: Pos<i64>) -> bool {
        *self.grid.at(pos) == b'#'
    }

    fn solve(&self) -> Answer {
        // 初期値(1, 1)
        let init_pos: Pos<i64> = Pos::new(1, 1);

        let mut visited_all = vec![vec![false; self.width]; self.height];
        let mut visited_stop = vec![vec![false; self.width]; self.height];

        let mut open: Queue<Pos<i64>> = Queue::new();
        open.push(init_pos);
        *visited_all.at_mut(init_pos) = true;
        *visited_stop.at_mut(init_pos) = true;

        while let Some(current_pos) = open.pop() {
            for &d in &DIR4_LIST {
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
                // loop を抜けた段階で岩にぶつかるまで移動したことになる

                if !visited_stop.at(moving_pos) {
                    *visited_stop.at_mut(moving_pos) = true;
                    open.push(moving_pos);
                }
            }
        }
        // flatten 使っても良かったかも
        // visited_all.iter().flatten().filter(|x| **x).count();
        let ans = visited_all
            .iter()
            .map(|row| row.iter().filter(|x| **x).count())
            .sum::<usize>() as i64;
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // 解法2: 頂点倍化
        // 初期値(1, 1)
        let init_pos: Pos<i64> = Pos::new(1, 1);
        // 0: 止まっている
        // 1,2,3,4: 動いている。向きはそれぞれ↓→↑←
        let init_state = 0;

        let mut visited = vec![vec![vec![false; 5]; self.width]; self.height];

        let mut open: Queue<(Pos<i64>, usize)> = Queue::new();
        open.push((init_pos, init_state));
        visited.at_mut(init_pos)[init_state] = true;

        while let Some((current_pos, current_state)) = open.pop() {
            if current_state == 0 {
                // 止まっている
                for dir_idx in 0..4 {
                    let next_pos = current_pos + DIR4_LIST[dir_idx];
                    if !self.is_ice(next_pos) {
                        continue;
                    }
                    let next_state = dir_idx + 1;
                    if !visited.at(next_pos)[next_state] {
                        visited.at_mut(next_pos)[next_state] = true;
                        open.push((next_pos, next_state));
                    }
                }
            } else {
                let dir_idx = current_state - 1;
                let next_pos_candidate = current_pos + DIR4_LIST[dir_idx];
                let (next_pos, next_state) = if self.is_ice(next_pos_candidate) {
                    (next_pos_candidate, current_state)
                } else {
                    // 0 は「止まっている」を表す
                    (current_pos, 0)
                };
                if !visited.at(next_pos)[next_state] {
                    visited.at_mut(next_pos)[next_state] = true;
                    open.push((next_pos, next_state));
                }
            }
        }
        let ans = visited
            .iter()
            .flatten() // 各マスを全探索
            .filter(|visited_pos| {
                // visited_pos はマスごとの情報
                // どれかの状態で訪問しているかを判定
                visited_pos.iter().any(|x| *x)
            })
            .count() as i64;
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
    ans: i64,
}

impl Answer {
    fn print(&self) {
        println!("{}", self.ans);
    }
}

fn main() {
    Problem::read().solve2().print();
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
    marker::{Bytes, Usize1},
};
use std::collections::VecDeque;
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    pub const DIR8_LIST: [Pos<i64>; 8] = [
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 1 },
        Pos { x: 1, y: 0 },
        Pos { x: 1, y: -1 },
        Pos { x: 0, y: -1 },
        Pos { x: -1, y: -1 },
        Pos { x: -1, y: 0 },
        Pos { x: -1, y: 1 },
    ];
    pub const DIR4_LIST: [Pos<i64>; 4] = [
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 0 },
        Pos { x: 0, y: -1 },
        Pos { x: -1, y: 0 },
    ];
    impl Pos<i64> {
        pub fn around4_pos_iter(self) -> impl Iterator<Item = Pos<i64>> {
            DIR4_LIST.iter().copied().map(move |d| d + self)
        }
        pub fn around8_pos_iter(self) -> impl Iterator<Item = Pos<i64>> {
            DIR8_LIST.iter().copied().map(move |d| d + self)
        }
    }
}
use vec_vec_at::*;
pub mod vec_vec_at {
    use super::pos::*;
    use easy_ext::ext;
    #[ext(VecVecAt)]
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
    }
    impl<T> Default for Queue<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}
