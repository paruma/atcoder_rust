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
            xs: [i64; n],
        }
        Problem { n, xs }
    }
    fn solve(&self) -> Answer {
        use ac_library::ModInt998244353 as Mint;
        let n = self.n;
        let xs = &self.xs;
        // TODO: n = 1がコーナーケース

        // dp[i][k][p][q] = xs[0..i] で 長さk の部分文字列を取って、初項がxs[p]で2項目がxs[q] であるようなものの数
        let mut dp = vec![vec![vec![vec![Mint::new(0); n]; n]; n + 1]; n + 1];

        // k=0 の初期化
        for i in 0..=n {
            for p in 0..n {
                for q in 0..n {
                    dp[i][0][p][q] = Mint::new(1);
                }
            }
        }

        // k=1 の初期化
        for i in 0..=n {
            for p in 0..i {
                for q in 0..n {
                    dp[i][1][p][q] = Mint::new(1);
                }
            }
        }

        for i in 1..n {
            for k in 1..n {
                for p in 0..n {
                    for q in 0..n {
                        let term1 = if xs[i] == xs[p] + (k as i64) * (xs[q] - xs[p]) {
                            dp[i][k][p][q]
                        } else {
                            Mint::new(0)
                        };
                        let term2 = dp[i][k + 1][p][q];
                        dp[i + 1][k + 1][p][q] = term1 + term2;
                    }
                }
            }
        }

        let uniq_idx = xs
            .iter()
            .copied()
            .enumerate()
            .unique_by(|(_, x)| *x)
            .map(|(i, _)| i)
            .collect_vec();

        //ans[l] = k = l + 1  のときの答え
        let mut ans = vec![0; n];
        ans[0] = n as i64; //k = 1の答え
        for l in 1..n {
            let k = l + 1;
            ans[l] = iproduct!(0..n, uniq_idx.iter().copied())
                .map(|(p, q)| dp[n][k][p][q])
                .sum::<Mint>()
                .val() as i64
        }

        // for i in 0..=n {
        //     for k in 0..=n {
        //         eprintln!("i={}, k={}", i, k);
        //         for p in 0..n {
        //             eprintln!("{}", dp[i][k][p].iter().join(" "));
        //         }
        //     }
        // }

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
        print_vec_1line(&self.ans);
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
        let n = rng.gen_range(1..=10);
        let xs = (0..n).map(|_| rng.gen_range(1..6)).collect_vec();
        let p = Problem { n, xs };
        println!("{:?}", &p);
        p
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
