//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    h: usize,
    w: usize,
    k: usize,
    grid: Vec<Vec<char>>,
}

struct DfsPermutationsWithReplacement {
    h: usize,
    w: usize,
    k: usize,
    grid: Vec<Vec<char>>,
    // n個のものから重複を許してr個取る順列 n^r
    //n: usize,
    //r: usize,
}

// これは非再帰（Stack）だと書きにくい？
impl DfsPermutationsWithReplacement {
    fn new(h: usize, w: usize, k: usize, grid: Vec<Vec<char>>) -> Self {
        Self { h, w, k, grid }
    }

    /// 計算量: O(n^r)
    fn exec(&self) -> usize {
        let mut cnt = 0;
        for y in 0..self.h {
            for x in 0..self.w {
                let init = Pos::new(x as i64, y as i64);
                if !self.can_move(init) {
                    continue;
                }
                let mut visited: Vec<Vec<bool>> = vec![vec![false; self.w]; self.h];
                let cnt_current = self.exec_rec(init, &mut visited, 0);
                //dbg!(cnt_current);
                cnt += cnt_current;
            }
        }
        cnt
    }

    fn is_within(&self, pos: Pos<i64>) -> bool {
        (0..self.h as i64).contains(&pos.y) && (0..self.w as i64).contains(&pos.x)
    }

    fn can_move(&self, pos: Pos<i64>) -> bool {
        self.is_within(pos) && *self.grid.at(pos) == '.'
    }

    // seq が現在の状態、seq_list が結果の蓄積物
    fn exec_rec(
        &self,
        pos: Pos<i64>,
        visited: &mut Vec<Vec<bool>>,
        mut cnt_visited: usize,
    ) -> usize {
        *visited.at_mut(pos) = true;// ここいる？
        cnt_visited += 1;
        if cnt_visited == self.k + 1 {
            return 1;
        }

        let mut cnt = 0;

        for next in Pos::around4_pos_iter(pos).filter(|next| self.can_move(*next)) {
            if *visited.at(next) {
                continue;
            }
            *visited.at_mut(next) = true;
            cnt += self.exec_rec(next, visited, cnt_visited);
            *visited.at_mut(next) = false;
        }
        cnt
    }
}

impl Problem {
    fn read() -> Problem {
        input! {
            h: usize,
            w: usize,
            k: usize,
            grid: [Chars; h],
        }
        Problem { h, w, k, grid }
    }

    fn solve(&self) -> Answer {
        let h = self.h;
        let w = self.w;
        let k = self.k;
        let grid = &self.grid;

        let solver = DfsPermutationsWithReplacement::new(h, w, k, grid.clone());

        let ans = solver.exec() as i64;
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
        pub fn inner_product(self, rhs: Self) -> T {
            self.x * rhs.x + self.y * rhs.y
        }
        pub fn norm_square(self) -> T {
            self.inner_product(self)
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
