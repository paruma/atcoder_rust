//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    xs: Vec<char>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            xs: Chars,
        }
        Problem { n, xs }
    }

    fn solve(&self) -> Answer {
        let n = self.n;
        let xs = self
            .xs
            .iter()
            .copied()
            .map(|ch| ((ch as u8) - b'0') as i64)
            .collect_vec();

        let cnt1 = xs.iter().copied().filter(|x| *x == 1).count();
        // i番目の1の場所
        let mut pos1 = vec![];
        for (i, x) in xs.iter().copied().enumerate() {
            if x == 1 {
                pos1.push(i)
            }
        }

        // dbg!(&pos1);

        let mut dist = pos1
            .iter()
            .copied()
            .enumerate()
            .map(|(i, p)| i.abs_diff(p) as i64)
            .sum::<i64>();

        let mut dist_list = vec![];

        let mut diffs = vec![0; n + 1]; // imos法で管理
        diffs[0] = -(cnt1 as i64);
        for (i, p) in pos1.iter().copied().enumerate() {
            diffs[p - i] += 2;
        }

        // dbg!(&diffs);

        for begin in 0..n - cnt1 + 1 {
            dist_list.push(dist);
            dist += diffs[begin];
            diffs[begin + 1] += diffs[begin];
        }

        // dbg!(&diffs);
        // dbg!(&dist_list);

        let ans = dist_list.iter().copied().min().unwrap();

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // 解法: abs_diff の和の最小化は中央値

        let n = self.n;
        let xs = self
            .xs
            .iter()
            .copied()
            .map(|ch| ((ch as u8) - b'0') as i64)
            .collect_vec();

        let pos1 = xs.iter().copied().positions(|x| x == 1).collect_vec();
        // sum |pos1[i] - i - x| を最小化する

        let ys = (0..pos1.len()).map(|i| pos1[i] - i).collect_vec();

        let argmin = ys[ys.len() / 2];

        let ans = (0..pos1.len())
            .map(|i| usize::abs_diff(pos1[i] - i, argmin))
            .sum::<usize>() as i64;

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
    Problem::read().solve2().print();
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
