//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    xs: Vec<i64>,
}

// x = 56 = 2^3 * 7 の場合は(3, 7) を返す
fn g(x: i64) -> (u32, i64) {
    let cnt = x.trailing_zeros();
    let remain = x >> cnt;
    (cnt, remain)
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            xs: [i64; n],
        }
        Problem { n, xs }
    }

    fn solve(&self) -> Answer {
        // オーバーフローに気をつける
        let n = self.n;

        let xs = &self.xs;
        // x = 2^a * b (b は奇数) と表すときの a で分類する

        let groups = xs.iter().copied().fold(vec![vec![]; 30], |mut acc, x| {
            let (a, _b) = g(x);
            acc[a as usize].push(x);
            acc
        });

        // a が異なる者同士で計算
        let term1 = {
            let sum_list = groups
                .iter()
                .map(|g| g.iter().copied().sum::<i64>())
                .collect_vec();

            let mut suffix_sum = 0;
            let mut suffix_cnt = 0;
            let mut sub_ans = 0;

            for a in (0..30).rev() {
                let sub_sub_ans =
                    (suffix_sum * groups[a].len() as i64 + sum_list[a] * suffix_cnt as i64) >> a;
                suffix_sum += sum_list[a];
                suffix_cnt += groups[a].len();

                sub_ans += sub_sub_ans;
            }
            sub_ans
        };

        let term2 = (0..30)
            .map(|a| {
                // a が同じ値同士で計算
                let mut table_sum = vec![HashMap::<i64, i64>::new(); 30];
                let mut table_cnt = vec![HashMap::<i64, i64>::new(); 30];
                let group = &groups[a];
                let mut sub_ans = 0;

                for &x in group {
                    let (_, y) = g(x);
                    for i in 1..30 {
                        let rem = y % (1 << i);
                        *table_sum[i].entry(rem).or_insert(0) += y;
                        *table_cnt[i].entry(rem).or_insert(0) += 1;
                    }

                    for i in 1..30 {
                        // (y + z) % 2^i = 2^{i-1} となるような z に対して、
                        // f(2^a(y + z)) = (y + z)/2^{i-1} である
                        let m = 1 << i;
                        let opposite_rem = (3 * m / 2 - y % m) % m;
                        let opposite_sum = table_sum[i].get(&opposite_rem).copied().unwrap_or(0);
                        let opposite_cnt = table_cnt[i].get(&opposite_rem).copied().unwrap_or(0);

                        let sub_sub_ans = (opposite_sum + y * opposite_cnt) >> (i - 1);
                        sub_ans += sub_sub_ans;
                    }
                }

                sub_ans
            })
            .sum::<i64>();

        let ans = term1 + term2;
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
