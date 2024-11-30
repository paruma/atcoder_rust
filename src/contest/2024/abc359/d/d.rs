//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    k: usize,
    str: Vec<u8>,
}

fn is_palindrome(str: &[u8]) -> bool {
    // str.iter().eq(str.iter().rev()); とした方が良い。
    str == str.iter().copied().rev().collect_vec()
}

fn to_str(b: &[u8]) -> String {
    String::from_utf8(b.to_vec()).unwrap()
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            k: usize,
            str: Bytes,
        }
        Problem { n, k, str }
    }
    fn solve(&self) -> Answer {
        use ac_library::ModInt998244353 as Mint;
        let n = self.n;
        let k: usize = self.k;
        let str = self.str.clone();
        // k-1 文字を持っておく
        let mut dp = vec![HashMap::<Vec<u8>, Mint>::new(); n + 1];

        let dummy = b"abcdefghijklmn"[0..k - 1].to_vec();

        dp[0].insert(b"abcdefghijklmn"[0..k - 1].to_vec(), Mint::new(1));

        for (i, ch) in str.iter().copied().enumerate() {
            let next_list = match ch {
                b'A' => vec![b'A'],
                b'B' => vec![b'B'],
                b'?' => vec![b'A', b'B'],
                _ => panic!(),
            };

            let mut next_dp = HashMap::<Vec<u8>, Mint>::new();

            for next in next_list {
                for (key, value) in &dp[i] {
                    // 過去 k-1 文字が key にある
                    let mut next_key = key.clone();
                    next_key.push(next);
                    if !is_palindrome(&next_key) {
                        // next_key の先頭を取る。
                        next_key = next_key[1..].to_vec();
                        *next_dp.entry(next_key).or_insert(Mint::new(0)) += value;
                    }
                }
            }
            dp[i + 1] = next_dp;
        }
        let ans = dp[n].values().sum::<Mint>().val() as i64;
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
        // let k = rng.gen_range(2..=10);
        // let n = rng.gen_range(k + 1..=14);
        // let chars = b"AB?";
        // let str = (0..n)
        //     .map(|_| {
        //         let i = rng.gen_range(0..3);
        //         chars[i]
        //     })
        //     .collect_vec();
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
