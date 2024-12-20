#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CollectionTiming {
    q: i64, // q で割ったあまりが
    r: i64, // r の日に回収する
}

impl CollectionTiming {
    fn next_collect(self, day: i64) -> i64 {
        if day % self.q == self.r {
            day
        } else {
            (day + self.q - self.r) / self.q * self.q + self.r
        }
    }

    fn next_collect2(self, day: i64) -> i64 {
        // 回収日はある整数nを用いて n*q + r と書ける。
        // (n-1)*q + r < day <= n*q + r を満たすので、
        // そういうn を求めて、n*q + rを計算すれば良い。
        // (n-1)*q + r < day <= n*q + r は
        // n-1 < (day -r)/q <= n と同値。 (割り算は実数の意味での割り算で切り捨てしない)
        // これは天井関数の特徴づけそのもの

        let n = num_integer::div_ceil(day - self.r, self.q);
        n * self.q + self.r
    }
}

#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Query {
    t: Usize1, // t番目のゴミを
    d: i64,    // d日目に出す
}

#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    timing: Vec<CollectionTiming>,
    nq: usize,
    qs: Vec<Query>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            timing: [CollectionTiming; n],
            nq: usize,
            qs: [Query; nq],
        }
        Problem { n, timing, nq, qs }
    }

    fn solve(&self) -> Answer {
        let ans = self
            .qs
            .iter()
            .copied()
            .map(|q| self.timing[q.t].next_collect2(q.d))
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
