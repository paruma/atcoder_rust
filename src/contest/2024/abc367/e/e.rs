//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    k: i64,
    xs: Vec<usize>,
    bs: Vec<i64>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            k: i64,
            xs: [Usize1; n],
            bs: [i64; n]
        }
        Problem { n, k, xs, bs }
    }
    fn solve(&self) -> Answer {
        let n = self.n;
        let k = self.k as usize;
        let xs = &self.xs;
        let bs = &self.bs;
        let ans = if k == 0 {
            bs.clone()
        } else {
            let d = Doubling::new(xs, k);
            (0..n).map(|i| bs[d.eval(k, i)]).collect_vec()
        };
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        let n = self.n;
        let k = self.k as usize;
        let xs = &self.xs;
        let bs = &self.bs;

        let ans = if k == 0 {
            bs.clone()
        } else {
            let transform = Transform::new(n);
            let xs_k = transform.pow(xs, k);

            (0..n).map(|i| bs[xs_k[i]]).collect_vec()
        };
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
use doubling::*;
#[allow(clippy::module_inception)]
pub mod doubling {
    pub struct Doubling {
        n: usize,
        log: usize,
        dp: Vec<Vec<usize>>,
    }
    impl Doubling {
        /// doubling 前処理の構築をする
        /// k は 合成回数の最大値 (k>=1)
        /// [計算量]
        /// n = f.len() としたとき、O(n log k)
        pub fn new(f: &[usize], k: usize) -> Doubling {
            let n = f.len();
            let log = (usize::BITS - k.leading_zeros()) as usize;
            let mut dp = vec![vec![0; n]; log];
            dp[0] = f.to_vec();
            for i in 1..log {
                for x in 0..n {
                    let f = &dp[i - 1];
                    dp[i][x] = f[f[x]];
                }
            }
            Doubling { n, log, dp }
        }
        /// (f の k回合成)(x) を求める。
        /// 計算量: O(log k)
        pub fn eval(&self, k: usize, x: usize) -> usize {
            assert!((0..self.n).contains(&x));
            assert!(k < (1 << self.log));
            if k == 0 {
                return x;
            }
            self.dp
                .iter()
                .enumerate()
                .filter(|(i, _)| (k >> i) & 1 == 1)
                .map(|(_, f)| f)
                .fold(x, |acc, f| f[acc])
        }
    }
}

use dynamic_monoid::*;
use monoid_transform::*;
pub mod dynamic_monoid {
    pub trait DynamicMonoid {
        type S: Clone;
        fn identity(&self) -> Self::S;
        fn binary_operation(&self, a: &Self::S, b: &Self::S) -> Self::S;
        /// base^n を求める
        fn pow(&self, base: &Self::S, n: usize) -> Self::S {
            let mut base = base.clone();
            let mut ans = self.identity();
            let mut n = n;
            while n > 0 {
                if n & 1 == 1 {
                    ans = self.binary_operation(&ans, &base);
                }
                base = self.binary_operation(&base, &base);
                n >>= 1;
            }
            ans
        }
    }
}
pub mod monoid_transform {
    use super::dynamic_monoid::DynamicMonoid;
    use itertools::Itertools;
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Transform {
        n: usize,
    }
    impl Transform {
        pub fn new(n: usize) -> Self {
            Self { n }
        }
    }
    impl DynamicMonoid for Transform {
        type S = Vec<usize>;
        fn identity(&self) -> Self::S {
            (0..self.n).collect_vec()
        }
        fn binary_operation(&self, a: &Self::S, b: &Self::S) -> Self::S {
            (0..self.n).map(|i| a[b[i]]).collect_vec()
        }
    }
}
