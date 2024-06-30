//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    k: usize,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            k: usize,
        }
        Problem { n, k }
    }
    fn solve(&self) -> Answer {
        use ac_library::ModInt998244353 as Mint;
        let n = self.n;
        let k = self.k;

        if n == 1 {
            return Answer { ans: 1 };
        }

        // dp[i]: i回後の操作で黒いボールが一番左にある確率
        let mut dp = vec![Mint::new(0); k + 1];
        dp[0] = Mint::new(1);

        let n_inv = Mint::new(n).inv();
        let n_ = Mint::new(n);

        for i in 1..=k {
            let value1 = dp[i - 1] * (n_ * n_ - n_ * 2 + 2) * (n_inv * n_inv);
            let value2 = (-dp[i - 1] + 1) * (n_ - 1) * 2 * (n_inv * n_inv) / (n - 1);
            dp[i] = value1 + value2;
            //dp[i] = (Mint::new(1) - dp[i - 1]) / (n - 1);
        }

        // dbg!(&dp
        //     .iter()
        //     .copied()
        //     .map(|p| p.to_rational_str())
        //     .collect_vec());

        let ans = dp[k] + Mint::new(n * (n + 1) / 2 - 1) * (-dp[k] + 1) / Mint::new(n - 1);
        //dbg!(ans.to_rational_str());
        let ans = ans.val() as i64;
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

use modint_to_rational::*;
pub mod modint_to_rational {
    use num_rational::Rational64;
    pub trait ToRational {
        fn to_rational(&self) -> Option<Rational64>;
        fn to_rational_str(&self) -> String {
            self.to_rational()
                .map(|x| x.to_string())
                .unwrap_or("cannot reconstruct".to_string())
        }
    }
    impl ToRational for ac_library::ModInt998244353 {
        /// 注意: 1000 * 2000 = 2*10^6 の計算をしている
        fn to_rational(&self) -> Option<Rational64> {
            if self.val() == 0 {
                return Some(Rational64::new(0, 1));
            }
            for denom in 1..1000 {
                let denom_inv = Self::new(denom).inv();
                for numer in -1000..1000 {
                    if *self == denom_inv * Self::new(numer) {
                        return Some(Rational64::new(numer, denom));
                    }
                }
            }
            None
        }
    }
}
