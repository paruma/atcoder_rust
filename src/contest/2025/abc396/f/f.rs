#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    m: usize,
    xs: Vec<usize>,
}

/// 転倒数 #{(i, j) | i < j and xs[i] > xs[j]} を求める
/// 計算量: O(n log n)
pub fn inversion_number(xs: &[usize]) -> i64 {
    use ac_library::FenwickTree;
    if xs.is_empty() {
        return 0;
    }
    let max_val = xs.iter().copied().max().unwrap();
    let mut fenwick = FenwickTree::new(max_val + 1, 0_i64);
    let mut cnt = 0;
    for &x in xs {
        cnt += fenwick.sum(x + 1..);
        fenwick.add(x, 1);
    }
    cnt
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            m: usize,
            xs: [usize; n],
        }
        Problem { n, m, xs }
    }

    fn solve(&self) -> Answer {
        let n = self.n;
        let m = self.m;
        let xs = &self.xs;

        let mut inv_num = inversion_number(xs);

        let poss_map = xs
            .iter()
            .copied()
            .enumerate()
            .fold(vec![vec![]; m], |mut acc, (i, x)| {
                acc[x].push(i);
                acc
            });

        let mut ans = vec![inv_num];

        for x in (1..m).rev() {
            // let diff_sum = poss_map[x]
            //     .iter()
            //     .copied()
            //     .map(|pos| pos as i64 - (n - pos - 1) as i64)
            //     .sum::<i64>();

            let poss = &poss_map[x];

            let diff_sum = poss
                .iter()
                .copied()
                .enumerate()
                .map(|(i, pos)| {
                    // i番目の x は pos にある
                    let left_all = pos; // #[0, pos)
                    let left_same = i; // #[0, i)
                    let right_all = n - pos - 1; // #(pos, n)
                    let right_same = poss.len() - i - 1; // #(i, poss.len())

                    let left_cnt = left_all - left_same;
                    let right_cnt = right_all - right_same;
                    left_cnt as i64 - right_cnt as i64
                })
                .sum::<i64>();
            inv_num += diff_sum;
            ans.push(inv_num);
        }
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
        // dbg!(inversion_number(&[2, 0, 1, 3]));
        // dbg!(inversion_number(&[3, 1, 2, 0]));
        // dbg!(inversion_number(&[0, 2, 3, 1]));
        // dbg!(inversion_number(&[1, 3, 0, 2]));

        // dbg!(inversion_number(&[1, 1, 2, 3]));
        // dbg!(inversion_number(&[2, 2, 3, 0]));
        // dbg!(inversion_number(&[3, 3, 0, 1]));
        // dbg!(inversion_number(&[0, 0, 1, 2]));
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
