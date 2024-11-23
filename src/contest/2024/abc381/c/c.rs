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
        let xs = &self.xs;
        let ans = (0..self.n)
            .filter(|&i| xs[i] == '/')
            .map(|i| {
                if i == 0 || i == n - 1 {
                    return 1;
                }
                let cnt1 = {
                    let mut j = i;
                    loop {
                        if j == 0 || xs[j - 1] != '1' {
                            break;
                        }
                        j -= 1;
                    }
                    i - j
                };

                let cnt2 = {
                    let mut j = i;
                    loop {
                        if j == n - 1 || xs[j + 1] != '2' {
                            break;
                        }
                        j += 1;
                    }
                    j - i
                };

                cnt1.min(cnt2) * 2 + 1
            })
            .max()
            .unwrap() as i64;

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // ランレングスを使う
        let n = self.n;
        let xs = &self.xs;
        let run_length = xs.iter().copied().dedup_with_count().collect_vec();
        let ans = (0..run_length.len())
            .filter(|&i| run_length[i].1 == '/')
            .map(|i| {
                if i == 0 || i == run_length.len() - 1 || run_length[i].0 > 1 {
                    return 1;
                }

                let cnt1 = if run_length[i - 1].1 == '1' {
                    run_length[i - 1].0
                } else {
                    0
                };

                let cnt2 = if run_length[i + 1].1 == '2' {
                    run_length[i + 1].0
                } else {
                    0
                };

                let cnt = cnt1.min(cnt2);
                cnt * 2 + 1
            })
            .max()
            .unwrap_or(0) as i64;

        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        // loop を find で書き換え
        let n = self.n;
        let xs = &self.xs;
        let ans = (0..self.n)
            .filter(|&i| xs[i] == '/')
            .map(|i| {
                if i == 0 || i == n - 1 {
                    return 1;
                }
                let cnt1 = {
                    let pos = (0..i).rev().find(|&i| xs[i] != '1');
                    pos.map(|pos| i - pos - 1).unwrap_or(i)
                };

                let cnt2 = {
                    let pos = (i + 1..n).find(|&i| xs[i] != '2');
                    pos.map(|pos| pos - i - 1).unwrap_or(n - i - 1)
                };

                cnt1.min(cnt2) * 2 + 1
            })
            .max()
            .unwrap() as i64;

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
    Problem::read().solve3().print();
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
