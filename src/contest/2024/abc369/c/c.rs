//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    xs: Vec<i64>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            xs: [i64; n]
        }
        Problem { n, xs }
    }
    fn solve(&self) -> Answer {
        // [3, 6, 9, 3] の差分をとる: [3, 3, -6]
        // 差分が同じ値でグループ分けする: [3, 3], [-6]
        // [3, 3] は [3, 6, 9] が等差になっていることを表している。
        // この部分のカウントは 3 + 2 + 1
        let diffs = self
            .xs
            .iter()
            .copied()
            .tuple_windows()
            .map(|(x, y)| y - x)
            .collect_vec();

        let cnts = diffs
            .iter()
            .dedup_with_count()
            .map(|(cnt, _)| cnt)
            .collect_vec();

        let ans1 = cnts
            .iter()
            .copied()
            .map(|cnt| (cnt + 1) * (cnt + 2) / 2)
            .sum::<usize>();
        let ans = (ans1 + 1 - cnts.len()) as i64;

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // [3, 6, 9, 3] を [3, 6, 9] と [9, 3] に分けて、それぞれのグループで数え上げの寄与を計算。
        // 9 の部分はカウントがかぶるので適当に引き算する。
        let n = self.n;
        let xs = &self.xs;

        let mut groups: Vec<Vec<i64>> = vec![];
        let mut current_group: Vec<i64> = vec![];

        for (i, x) in xs.iter().enumerate() {
            let x = *x;
            if current_group.len() <= 1 {
                current_group.push(x);
            } else {
                let first = current_group[0];
                let second = current_group[1];
                let diff = second - first; // 等差
                let last = current_group.last().unwrap();
                if last + diff == x {
                    current_group.push(x)
                } else {
                    groups.push(current_group.clone());
                    current_group.clear();
                    current_group.push(xs[i - 1]);
                    current_group.push(x);
                }
            }
        }

        if !current_group.is_empty() {
            groups.push(current_group);
        }

        let ans = groups
            .iter()
            .map(|g| {
                let k = g.len();
                k * (k + 1) / 2
            })
            .sum::<usize>()
            - (groups.len() - 1);
        let ans = ans as i64;
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
