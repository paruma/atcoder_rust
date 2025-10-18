#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Query {
    x1: Usize1,
    x2: Usize1,
    y1: Usize1,
    y2: Usize1,
    z1: Usize1,
    z2: Usize1,
}

#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    xsss: Vec<Vec<Vec<i64>>>,
    nq: usize,
    qs: Vec<Query>,
}

pub mod cumsum_3d {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct CumSum3D {
        pub cumsum: Vec<Vec<Vec<i64>>>,
    }

    impl CumSum3D {
        pub fn new(xsss: &[Vec<Vec<i64>>]) -> CumSum3D {
            let x_len = xsss.len();
            let y_len = xsss[0].len();
            let z_len = xsss[0][0].len();

            let mut cumsum = vec![vec![vec![0; z_len + 1]; y_len + 1]; x_len + 1];
            for x in 1..x_len + 1 {
                for y in 1..y_len + 1 {
                    for z in 1..z_len + 1 {
                        cumsum[x][y][z] = xsss[x - 1][y - 1][z - 1]
                            + cumsum[x - 1][y][z]
                            + cumsum[x][y - 1][z]
                            + cumsum[x][y][z - 1]
                            - cumsum[x - 1][y - 1][z]
                            - cumsum[x - 1][y][z - 1]
                            - cumsum[x][y - 1][z - 1]
                            + cumsum[x - 1][y - 1][z - 1];
                    }
                }
            }
            CumSum3D { cumsum }
        }

        pub fn sum(&self, x1: usize, x2: usize, y1: usize, y2: usize, z1: usize, z2: usize) -> i64 {
            // [x1, x2) × [y1, y2) × [z1, z2) の範囲で総和を求める
            self.cumsum[x2][y2][z2]
                - self.cumsum[x1][y2][z2]
                - self.cumsum[x2][y1][z2]
                - self.cumsum[x2][y2][z1]
                + self.cumsum[x1][y1][z2]
                + self.cumsum[x1][y2][z1]
                + self.cumsum[x2][y1][z1]
                - self.cumsum[x1][y1][z1]
        }
    }
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            xsss: [[[i64; n]; n]; n],
            nq: usize,
            qs: [Query; nq],
        }
        Problem { n, xsss, nq, qs }
    }
    fn solve(&self) -> Answer {
        let c = CumSum3D::new(&self.xsss);
        let ans = self
            .qs
            .iter()
            .copied()
            .map(
                |Query {
                     x1,
                     x2,
                     y1,
                     y2,
                     z1,
                     z2,
                 }| {
                    let x2 = x2 + 1;
                    let y2 = y2 + 1;
                    let z2 = z2 + 1;
                    c.sum(x1, x2, y1, y2, z1, z2)
                },
            )
            .collect_vec();
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
    ans: Vec<i64>,
}

impl Answer {
    fn print(&self) {
        print_vec(&self.ans);
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

use cumsum_3d::CumSum3D;
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
