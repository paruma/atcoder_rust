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
    pub fn is_black(&self, pos: Pos) -> bool {
        ['#'].contains(&self[pos])
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
//#[derive_readable]
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Problem {
    n: usize,
    xs: Vec<i64>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            xs: [i64; n],
        }
        Problem { n, xs }
    }
    fn solve_sub(&self) -> bool {
        let n = self.n;
        let xs = &self.xs;
        if n == 1 {
            return true;
        }
        if n == 2 {
            return false;
        }
        // A_i が偶数である i の数

        let cnt_even = xs.iter().filter(|&&x| x % 2 == 0).count();
        if n == 3 {
            return cnt_even != 3;
        }
        // n = 4: cnt_even % 2 == 1
        // n = 5: cnt_even % 2 == 0
        // n = 6: cnt_even % 2 == 1
        if n % 2 == 0 {
            cnt_even % 2 == 1
        } else {
            cnt_even % 2 == 0
        }
    }
    #[allow(dead_code)]
    fn solve(&self) -> Answer {
        let ans = self.solve_sub();
        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        let xs = &self.xs;

        // 先手勝ち？
        fn rec(xs: &[i64], set: &HashSet<usize>) -> bool {
            if set.len() == xs.len() {
                return false;
            }
            let n = xs.len();

            (0..n).filter(|&i| xs[i] > 0).any(|i| {
                // 相手負け
                let mut next_xs = xs.to_vec();
                next_xs[i] -= 1;
                let mut next_set = set.clone();
                next_set.insert(i);
                !rec(&next_xs, &next_set)
            })
        }

        let ans = rec(xs, &HashSet::new());
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: bool, // 先手勝ち？
}

impl Answer {
    fn print(&self) {
        if self.ans {
            // 先手勝ち？
            println!("Fennec");
        } else {
            println!("Snuke");
        }
    }
}

fn main() {
    //Problem::read().solve().print();
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
        let n = rng.random_range(1..=8);
        let xs = (0..n).map(|_| rng.random_range(1..=2)).collect_vec();
        let p = Problem { n, xs };
        println!("{:?}", &p);
        p
    }

    #[allow(unreachable_code)]
    #[test]
    fn test_with_naive() {
        let num_tests = 109;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut rng = SmallRng::seed_from_u64(42);
        // let mut rng = SmallRng::from_os_rng();
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
    pub fn print_vec<T: std::fmt::Display>(arr: &[T]) {
        for a in arr {
            println!("{}", a);
        }
    }
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Display>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{}", x)).join(" ");
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
