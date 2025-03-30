//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    k: usize,
    xs: Vec<i64>,
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
        use ac_library::ModInt998244353 as Mint;
        let n = self.n;
        let k = self.k;

        let xs = self.xs.iter().copied().map(Mint::new).collect_vec();

        let mut dp = vec![vec![Mint::new(0); n]; k + 1];

        let comb = Comb::new(k);

        // k = 1
        dp[1][0] = xs[0];
        for j in 1..n {
            dp[1][j] = dp[1][j - 1] + xs[j] * (j + 1);
        }

        for l in 2..=k {
            dp[l][0] = xs[0].pow(l as u64);
            for j in 1..n {
                dp[l][j] = dp[l][j - 1]
                    + xs[j].pow(l as u64) * (j + 1)
                    + (1..=l - 1)
                        .map(|p| {
                            //
                            comb.comb(l, p) * dp[l - p][j - 1] * xs[j].pow(p as u64)
                        })
                        .sum::<Mint>()
            }
        }

        let ans = dp[k].iter().sum::<Mint>().val() as i64;
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
use mod_combinatorics::*;
pub mod mod_combinatorics {
    use ac_library::ModInt998244353 as Mint;
    pub struct Comb {
        fac: Vec<Mint>,
        invfac: Vec<Mint>,
    }
    impl Comb {
        pub fn new(max_val: usize) -> Self {
            let mut inv = vec![Mint::new(0); max_val + 1];
            let mut fac = vec![Mint::new(0); max_val + 1];
            let mut invfac = vec![Mint::new(0); max_val + 1];
            fac[0] = 1.into();
            fac[1] = 1.into();
            invfac[0] = 1.into();
            invfac[1] = 1.into();
            inv[1] = 1.into();
            let modulus = Mint::modulus() as usize;
            for i in 2..=max_val {
                inv[i] = -inv[modulus % i] * Mint::new(modulus / i);
                fac[i] = fac[i - 1] * Mint::new(i);
                invfac[i] = invfac[i - 1] * inv[i];
            }
            Self { fac, invfac }
        }
        pub fn comb(&self, n: usize, k: usize) -> Mint {
            if n < k {
                0.into()
            } else {
                self.fac[n] * self.invfac[k] * self.invfac[n - k]
            }
        }
        pub fn perm(&self, n: usize, k: usize) -> Mint {
            if n < k {
                0.into()
            } else {
                self.fac[n] * self.invfac[n - k]
            }
        }
        pub fn factorial(&self, n: usize) -> Mint {
            self.fac[n]
        }
        pub fn inv_factorial(&self, n: usize) -> Mint {
            self.invfac[n]
        }
    }
}
