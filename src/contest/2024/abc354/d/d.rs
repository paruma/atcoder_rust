#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
}

struct QuerySolver {
    cumsum: CumSum2D,
    height: i64,
    width: i64,
}
impl QuerySolver {
    fn new(grid: &Vec<Vec<i64>>, height: usize, width: usize) -> Self {
        let cumsum = CumSum2D::new(grid);
        Self {
            cumsum,
            height: height as i64,
            width: width as i64,
        }
    }

    fn prefix_sum(&self, y: i64, x: i64) -> i64 {
        //  [0, x) × [0, y) で数える
        let cnt_sq_x = div_floor(x, self.width);
        let cnt_sq_y = div_floor(y, self.height);
        let remain_x = i64::rem_euclid(x, self.width) as usize;
        let remain_y = i64::rem_euclid(y, self.height) as usize;
        let height_usize = self.height as usize;
        let width_usize = self.width as usize;
        let cnt_in_sq = self
            .cumsum
            .get_rect_sum((0, 0), (width_usize, height_usize));

        let sum1 = cnt_sq_x * cnt_sq_y * cnt_in_sq;
        let sum2 = self.cumsum.get_rect_sum((0, 0), (width_usize, remain_y)) * cnt_sq_x;
        let sum3: i64 = self.cumsum.get_rect_sum((0, 0), (remain_x, height_usize)) * cnt_sq_y;
        let sum4 = self.cumsum.get_rect_sum((0, 0), (remain_x, remain_y));
        sum1 + sum2 + sum3 + sum4
    }

    fn solve(&self, a: i64, b: i64, c: i64, d: i64) -> i64 {
        let x1 = a;
        let y1 = b;
        let x2 = c;
        let y2 = d;
        // self.prefix_sum(y2 + 1, x2 + 1) - self.prefix_sum(y1, x2 + 1) - self.prefix_sum(y2 + 1, x1)
        //     + self.prefix_sum(y1, x1)

        self.prefix_sum(y2, x2) - self.prefix_sum(y1, x2) - self.prefix_sum(y2, x1)
            + self.prefix_sum(y1, x1)
    }
}

impl Problem {
    fn read() -> Problem {
        input! {
            a: i64,
            b: i64,
            c: i64,
            d: i64,
        }
        Problem { a, b, c, d }
    }
    fn solve(&self) -> Answer {
        let a = self.a;
        let b = self.b;
        let c = self.c;
        let d = self.d;
        let grid = vec![vec![2, 1, 0, 1], vec![1, 2, 1, 0]];
        let solver = QuerySolver::new(&grid, 2, 4);
        let ans = solver.solve(a, b, c, d);
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
        // let n = rng.random_range(1..=10);
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
use num_integer::div_floor;
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};
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

use cumsum_2d::*;
pub mod cumsum_2d {
    pub struct CumSum2D {
        pub cumsum: Vec<Vec<i64>>,
    }
    impl CumSum2D {
        pub fn new(xss: &Vec<Vec<i64>>) -> CumSum2D {
            if xss.is_empty() {
                return CumSum2D {
                    cumsum: vec![vec![0]],
                };
            }
            let height = xss.len();
            let width = xss[0].len();
            let mut cumsum = vec![vec![0; width + 1]; height + 1];
            for y in 1..height + 1 {
                for x in 1..width + 1 {
                    cumsum[y][x] = cumsum[y - 1][x] + cumsum[y][x - 1] - cumsum[y - 1][x - 1]
                        + xss[y - 1][x - 1];
                }
            }
            CumSum2D { cumsum }
        }
        pub fn get_rect_sum(&self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> i64 {
            self.cumsum[y2][x2] - self.cumsum[y2][x1] - self.cumsum[y1][x2] + self.cumsum[y1][x1]
        }
    }
}
