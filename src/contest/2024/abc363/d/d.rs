//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: i64,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: i64,
        }
        Problem { n }
    }
    fn solve(&self) -> Answer {
        // 回文数を以下のようにパターン分けする。
        // 0
        // a
        // aa
        // aba
        // abba
        // abcba
        // abccba
        // ...
        // a in [1, 9]
        // b, c in [0, 9]

        let n = self.n - 1; // 0オリジンにする

        if n == 0 {
            return Answer { ans: vec![0] };
        };

        let mut cnt = 1;

        for i in 1.. {
            // 例えば i=3のとき、abcba, abccba のパターンを考える

            // 例えば
            // a in [1, 9]
            // b, c in [0, 9]
            // となる (a,b,c) は 9 * 10 * 10 = 900 通り
            let cnt_in_pattern = 9 * i64::pow(10, i - 1);

            // abcba パターン
            if (cnt..cnt + cnt_in_pattern).contains(&n) {
                let abc = n - cnt + i64::pow(10, i - 1);
                let abc_digits = to_base_n_value(abc, 10);
                let ans = chain!(abc_digits.iter(), abc_digits.iter().rev().skip(1))
                    .copied()
                    .collect_vec();
                return Answer { ans };
            }
            cnt += cnt_in_pattern;

            // abccba パターン
            if n < cnt + cnt_in_pattern {
                let abc = n - cnt + i64::pow(10, i - 1);
                let abc_digits = to_base_n_value(abc, 10);
                let ans = chain!(abc_digits.iter(), abc_digits.iter().rev())
                    .copied()
                    .collect_vec();
                return Answer { ans };
            }
            cnt += cnt_in_pattern
        }
        panic!()
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
        println!("{}", self.ans.iter().join(""));
    }
}

fn main() {
    Problem::read().solve().print();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use bstr::join;
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};

    #[test]
    fn test_problem() {
        for n in 1..300 {
            dbg!(n);
            let p = Problem { n };
            let msg = p.solve().ans.iter().join("");
            dbg!(msg);
        }
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
use itertools::{Itertools, chain, iproduct, izip};
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
use positional_notation::*;
#[allow(clippy::module_inception)]
pub mod positional_notation {
    pub fn eval_base_n_value(xs: &[i64], base: i64) -> i64 {
        xs.iter().fold(0, |acc, &x| acc * base + x)
    }
    pub fn to_base_n_value(x: i64, base: i64) -> Vec<i64> {
        let mut ret = vec![];
        let mut x = x;
        while x > 0 {
            ret.push(x % base);
            x /= base;
        }
        ret.reverse();
        ret
    }
}
