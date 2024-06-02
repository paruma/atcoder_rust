#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct TestCase {
    a: i64,
    b: i64,
    c: i64,
}

fn pow10(x: i64) -> Mint {
    Mint::pow(Mint::new(10), x as u64)
}

// 0 + 1 + ... + (x-1) を計算する
fn seq_sum(x: Mint) -> Mint {
    x * (x - 1) / 2
}

// begin + (begin + 1) + ... + (end  - 1) を計算する
fn seq_range_sum(begin: Mint, end: Mint) -> Mint {
    seq_sum(end) - seq_sum(begin)
}

impl TestCase {
    fn solve(&self) -> i64 {
        // [方針]
        // a>=b として一般性を失わない
        // c が a, a+1 以外の場合は0通りである。
        // a桁+b桁 が a桁の領域とa+1桁の領域に分ける。a桁の領域の数を計算する。
        // a+1桁の領域は全体からa桁の領域を引く
        // a桁の領域の計算の際には、a==b の場合と a>b の場合で場合分けする。

        let a = i64::max(self.a, self.b);
        let b = i64::min(self.a, self.b);
        let c = self.c;

        if ![a, a + 1].contains(&c) {
            return 0;
        }

        let ans = if a == b {
            // [(x, y) in (pow10(a-1)..pow10(a))^2 | x + y < pow10(c)]

            // (pow10(a-1)..pow10(a))^2 という正方形を考える
            let side_len = pow10(a) - pow10(a - 1);

            let cnt_sq = side_len * side_len;

            // a桁 + a桁 が a桁になる場合の数
            // // 1 + 2 + ... + 8 * pow10(a-1)
            let cnt_triangle = seq_sum(Mint::new(8) * pow10(a - 1) + 1);

            if c == a {
                cnt_triangle
            } else if c == a + 1 {
                cnt_sq - cnt_triangle
            } else {
                unreachable!();
            }
        } else {
            // a > b
            let side_len_long = pow10(a) - pow10(a - 1);
            let side_len_short = pow10(b) - pow10(b - 1);

            let cnt_rect = side_len_long * side_len_short;

            // y in [pow10(b-1), pow10(b)) に対して、x in [pow10(a-1), pow10(a) - y) だと、x + y < pow10(a) となる
            let cnt_left = seq_range_sum(
                (pow10(a) - (pow10(b) - 1)) - pow10(a - 1),
                (pow10(a) - pow10(b - 1)) - pow10(a - 1) + 1,
            ); // c == a の場合の数

            if c == a {
                cnt_left
            } else if c == a + 1 {
                cnt_rect - cnt_left
            } else {
                unreachable!()
            }
        };
        ans.val() as i64
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
use num::pow;
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
