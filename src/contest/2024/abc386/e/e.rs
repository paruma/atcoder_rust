//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    k: usize,
    xs: Vec<i64>,
}

fn combinations_naive(n: usize, k: usize, xs: &[i64]) -> i64 {
    struct DfsCombinations {
        // n個のものからk個取る組合せ nCk
        n: usize,
        k: usize,
        xs: Vec<i64>,
    }

    impl DfsCombinations {
        fn new(n: usize, k: usize, xs: Vec<i64>) -> Self {
            Self { n, k, xs }
        }

        fn exec(&self) -> Vec<i64> {
            let mut seq_list = vec![];
            self.exec_rec(&mut vec![], &mut seq_list);
            seq_list
        }

        // seq が現在の状態、seq_list が結果の蓄積物
        fn exec_rec(&self, seq: &mut Vec<usize>, xor_sum_list: &mut Vec<i64>) {
            if seq.len() == self.k {
                // ここがforループの中のようなもの
                let xor_sum = seq
                    .iter()
                    .copied()
                    .map(|i| self.xs[i])
                    .fold(0, |acc, x| acc ^ x);
                xor_sum_list.push(xor_sum);
                return;
            }

            let begin = seq.last().copied().map(|x| x + 1).unwrap_or(0);

            // ループ範囲は具体例 (r=2 くらい) を考えるとわかる
            for i in begin..self.n - self.k + 1 + seq.len() {
                seq.push(i);
                self.exec_rec(seq, xor_sum_list);
                seq.pop();
            }
        }
    }
    let dfs = DfsCombinations::new(n, k, xs.to_vec());
    dfs.exec().iter().copied().max().unwrap()
}

fn combinations(n: usize, k: usize, xs: &[i64]) -> i64 {
    struct DfsCombinations {
        // n個のものからk個取る組合せ nCk
        n: usize,
        k: usize,
        xs: Vec<i64>,
    }

    impl DfsCombinations {
        fn new(n: usize, k: usize, xs: Vec<i64>) -> Self {
            Self { n, k, xs }
        }

        fn exec(&self) -> Vec<i64> {
            let mut seq_list = vec![];
            self.exec_rec(&mut vec![], 0, &mut seq_list);
            seq_list
        }

        // seq が現在の状態、seq_list が結果の蓄積物
        fn exec_rec(&self, seq: &mut Vec<usize>, xor_sum: i64, xor_sum_list: &mut Vec<i64>) {
            if seq.len() == self.k {
                // ここがforループの中のようなもの
                xor_sum_list.push(xor_sum);
                return;
            }

            let begin = seq.last().copied().map(|x| x + 1).unwrap_or(0);

            // ループ範囲は具体例 (r=2 くらい) を考えるとわかる
            for i in begin..self.n - self.k + 1 + seq.len() {
                seq.push(i);
                self.exec_rec(seq, xor_sum ^ self.xs[i], xor_sum_list);
                seq.pop();
            }
        }
    }
    if k * 2 <= n {
        let dfs = DfsCombinations::new(n, k, xs.to_vec());
        dfs.exec().iter().copied().max().unwrap()
    } else {
        let dfs = DfsCombinations::new(n, n - k, xs.to_vec());
        let all = xs.iter().copied().fold(0, |acc, x| acc ^ x);
        dfs.exec().iter().copied().map(|x| all ^ x).max().unwrap()
    }
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            k: usize,
            xs: [i64; n],
        }
        Problem { n, k, xs }
    }

    fn solve(&self) -> Answer {
        //let ans = combinations_naive(self.n, self.k, &self.xs);
        let ans = combinations(self.n, self.k, &self.xs);
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
    // let n = 200_000;
    // println!("{n} {n}");
    // println!("{}", std::iter::repeat(2).take(n).join(" "));
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
