#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct TestCase {
    a1: i64,
    a2: i64,
    a3: i64,
}

impl TestCase {
    fn solve(&self) -> i64 {
        // 実装途中
        let ad = self.a1;
        let bd = self.a2;
        let sd = self.a3;

        let mint10 = Mint::new(10);
        let amin = mint10.pow((ad - 1) as u64);
        let amax = mint10.pow((ad) as u64) - Mint::new(1);
        let bmin = mint10.pow((bd - 1) as u64);
        let bmax = mint10.pow((bd) as u64) - Mint::new(1);

        let smin = mint10.pow((sd - 1) as u64);
        let smax = mint10.pow((sd) as u64) - Mint::new(1);

        let ans13: Mint = {
            // max(amax + bmin, smin)
            // max(10^ad + 10^(bd-1) -1, 10^(sd -1))
            let s_sub_min = if sd - 1 <= ad || sd - 1 <= bd - 1 {
                amax + bmin
            } else {
                smin
            };

            // min(amin + bmax, smax)
            // max(10^(ad-1) + 10^(bd) -1, 10^sd -1)
            let s_sub_max = if sd <= ad - 1 || sd <= bd {
                amin + bmax
            } else {
                smax
            };

            let p = amin;
            let q = amax;

            // P - Q + 1 =
            (p - q + Mint::new(1)) * (s_sub_max - s_sub_min + Mint::new(1))
        };
        let ans14 = {};
        let ans23 = Mint::new(0);
        let ans24 = Mint::new(0);

        0
    }
}
#[derive(Debug, Clone)]
struct Problem {
    ts: Vec<TestCase>,
}
use ac_library::ModInt998244353 as Mint;

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            ts: [TestCase; n],
        }

        Problem { ts }
    }
    fn solve(&self) -> Answer {
        let ans = self.ts.iter().map(|t| t.solve()).collect_vec();
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
        print_vec(&self.ans)
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
