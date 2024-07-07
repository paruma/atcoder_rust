//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: i64,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: i64
        }
        Problem { n }
    }
    fn solve(&self) -> Answer {
        // 問題: n以下の累乗数を求める
        // 解法: a の寄与を考える(a を固定したときに a^b <=n となる b=2,3,... が何通りあるかを考える)
        // a <= 10^6 までは愚直に計算する。
        // a > 10^6 の場合は、b=2 の1通り。
        // a は累乗数の場合は除外し、累乗数でないものに対して計算をする。

        // このコードは以下の提出をリファクタリングしたもの
        // https://atcoder.jp/contests/abc361/submissions/55318346

        let n = self.n;
        // a = 1 の寄与: 1通り
        let sum1 = 1;

        // 2 <= a <= 10^6 に対する寄与
        let sum2 = {
            // 10^6 までの累乗数かどうかの判定を前処理で計算
            let is_perfect_power = {
                let mut is_perfect_power = vec![false; 1_000_001];
                for a in (2..).take_while(|a| a * a <= 1_000_000) {
                    std::iter::successors(Some(a * a), |acc| acc.checked_mul(&a))
                        .take_while(|&acc| acc <= 1_000_000)
                        .for_each(|x| is_perfect_power[x] = true);
                }
                is_perfect_power
            };
            (2..)
                .take_while(|&a| a * a <= n && a <= 1_000_000)
                .filter(|a| !is_perfect_power[*a as usize])
                .map(|a| {
                    // a^b <= n となる b=2,3... の数を求める (a は累乗数でないとする)

                    std::iter::successors(Some(a * a), |acc| acc.checked_mul(&a))
                        .take_while(|acc| *acc <= n)
                        .count() as i64
                })
                .sum::<i64>()
        };

        // a > 10^6 に対する寄与
        let sum3 = {
            if n <= 1_000_000_000_000
            // 10^12
            {
                // a > 10^6 の寄与はない。
                0
            } else {
                // a > 10^6 かつ a * a <= n となる累乗数 a の列挙
                let perfect_power_set = (2..)
                    .take_while(|x| {
                        // a >= x^2 で、a^2 <= n なので、x^4 <= n
                        checked_pow(*x, 4)
                            .map(|x_pow4| x_pow4 <= n)
                            .unwrap_or(false)
                    })
                    .flat_map(|x| {
                        std::iter::successors(Some(x * x), move |acc| acc.checked_mul(&x))
                            .take_while(|a| a.checked_mul(a).map(|aa| aa <= n).unwrap_or(false))
                            .filter(|a| *a > 1_000_000)
                    })
                    .collect::<HashSet<_>>();
                // a > 10^6, a * a <= n を満たす非累乗数 a に対して、それぞれ b = 2 という1つ分の寄与がある。
                n.sqrt() - 1_000_000 - perfect_power_set.len() as i64
            }
        };

        let ans = sum1 + sum2 + sum3;
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
use num::{checked_pow, CheckedMul};
use num_integer::Roots;
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Chars, Usize1},
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
