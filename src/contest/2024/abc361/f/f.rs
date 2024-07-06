//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: i64,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: i64
        }
        Problem { n }
    }
    fn solve(&self) -> Answer {
        let n = self.n;
        let sum1 = 1; // x = 1^b = 1

        let mut visited = vec![false; 1_000_001];
        let sum2 = (2..=(n + 1).sqrt().min(1_000_000))
            .map(|a| {
                // a^b <= x となる b=2,3... の数を求める
                // a が 何かのべき乗であるケースは除く。
                if visited[a as usize] {
                    return 0;
                }
                if a * a > n {
                    0
                } else {
                    let mut acc = a * a;
                    let mut cnt = 0;
                    loop {
                        if acc <= 1_000_000 {
                            visited[acc as usize] = true;
                        }

                        if acc > n {
                            break;
                        }
                        if acc.checked_mul(a).is_none() {
                            cnt += 1;
                            break;
                        }
                        acc *= a;
                        cnt += 1;
                    }
                    cnt
                }
            })
            .sum::<i64>();

        let sum3 = {
            // (n + 1).sqrt().min(1_000_000) から n_sqrt() まで
            if (n + 1).sqrt() <= 1_000_000 {
                0
            } else {
                let mut set: HashSet<i64> = HashSet::new();
                // 1_000_001 以上、n.sqrt() 以下で x^b の形で表せるもの
                for i in 2..=1_000_000 {
                    let mut acc = i;
                    loop {
                        acc *= i;
                        if acc > n.sqrt() {
                            break;
                        }
                        if acc >= 1_000_001 {
                            set.insert(acc);
                        }
                    }
                }
                n.sqrt() - 1_000_000 - set.len() as i64
            }
        };

        let ans = sum1 + sum2 + sum3;
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
use num_integer::Roots;
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Chars, Usize1},
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
