#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Query {
    xl: Usize1,
    xr: Usize1,
    yl: Usize1,
    yr: Usize1,
}

#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    nq: usize,
    xs: Vec<usize>,
    ys: Vec<usize>,
    qs: Vec<Query>,
}

fn pow(base: i64, n: i64, m: i64) -> i64 {
    let mut base = base as i128;
    let m = m as i128;
    let mut ans = 1;
    let mut n = n;

    while n > 0 {
        if n & 1 == 1 {
            ans *= base;
            ans %= m;
        }
        base = base * base;
        ans %= m;
        n >>= 1;
    }
    ans as i64
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            nq: usize,
            xs: [Usize1; n],
            ys: [Usize1; n],
            qs: [Query; nq],
        }
        Problem { n, nq, xs, ys, qs }
    }
    fn solve(&self) -> Answer {
        // 謎ハッシュ x ↦ (998244853^x + 998244353) mod 10^9 + 7 を使った
        let n: usize = self.n;
        let nq: usize = self.nq;
        let xs: &Vec<usize> = &self.xs;
        let ys: &Vec<usize> = &self.ys;
        let qs: &Vec<Query> = &self.qs;
        let m = 1_000_000_007;
        //let m = 3;

        let offset = 998244353_i64;

        let xsh = xs
            .iter()
            .copied()
            .map(|x| (pow(998244853_i64, x as i64, m) + offset) % m)
            .collect_vec();

        let ysh = ys
            .iter()
            .copied()
            .map(|x| (pow(998244853_i64, x as i64, m) + offset) % m)
            .collect_vec();

        let xshc = xsh
            .iter()
            .copied()
            .scanl(0_i64, |acc, x| (*acc + x) % m)
            .collect_vec();

        let yshc = ysh
            .iter()
            .copied()
            .scanl(0_i64, |acc, x| (*acc + x) % m)
            .collect_vec();

        let ans = qs
            .iter()
            .copied()
            .map(|q| {
                let xsum = (xshc[q.xr + 1] - xshc[q.xl] + m) % m;
                let ysum = (yshc[q.yr + 1] - yshc[q.yl] + m) % m;
                xsum == ysum && q.xr - q.xl == q.yr - q.yl
            })
            .collect_vec();

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // Zobrist Hash を使う
        let n: usize = self.n;
        let nq: usize = self.nq;
        let xs: &Vec<usize> = &self.xs;
        let ys: &Vec<usize> = &self.ys;
        let qs: &Vec<Query> = &self.qs;

        use ac_library::ModInt998244353 as Mint;

        use rand::{rngs::SmallRng, *};
        // let mut rng = SmallRng::from_entropy();
        let mut rng = SmallRng::seed_from_u64(42);

        let rands = (0..n)
            .map(|_| Mint::new(rng.gen_range(0..998244353)))
            .collect_vec();

        let xsh = xs.iter().copied().map(|x| rands[x]).collect_vec();
        let ysh = ys.iter().copied().map(|x| rands[x]).collect_vec();

        let xshc = xsh
            .iter()
            .copied()
            .scanl(Mint::new(0), |acc, x| *acc + x)
            .collect_vec();

        let yshc = ysh
            .iter()
            .copied()
            .scanl(Mint::new(0), |acc, x| *acc + x)
            .collect_vec();

        let ans = qs
            .iter()
            .copied()
            .map(|q| {
                let xsum = xshc[q.xr + 1] - xshc[q.xl];
                let ysum = yshc[q.yr + 1] - yshc[q.yl];
                xsum == ysum //&& q.xr - q.xl == q.yr - q.yl
            })
            .collect_vec();

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
    ans: Vec<bool>,
}

impl Answer {
    fn print(&self) {
        for x in &self.ans {
            print_yesno(*x);
        }
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

use ac_library::pow_mod;
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

use scan_iter::*;
pub mod scan_iter {
    #[derive(Clone)]
    pub struct Scanl<I, B, F> {
        iter: I,
        state: Option<B>,
        f: F,
    }
    impl<I, B, F> Scanl<I, B, F> {
        fn new(iter: I, init: B, f: F) -> Scanl<I, B, F> {
            Scanl {
                iter,
                state: Some(init),
                f,
            }
        }
    }
    impl<I, B, F> Iterator for Scanl<I, B, F>
    where
        B: Copy,
        I: Iterator,
        F: FnMut(&mut B, I::Item) -> B,
    {
        type Item = B;
        #[inline]
        fn next(&mut self) -> Option<B> {
            let retval = self.state?;
            let a_opt = self.iter.next();
            self.state = self
                .state
                .and_then(|mut s| a_opt.map(|a| (self.f)(&mut s, a)));
            Some(retval)
        }
    }
    pub trait IteratorExtScanLeft: Iterator + Sized {
        fn scanl<B, F>(self, init: B, f: F) -> Scanl<Self, B, F>
        where
            Self: Sized,
            F: FnMut(&mut B, Self::Item) -> B,
        {
            Scanl::new(self, init, f)
        }
    }
    impl<T: Iterator> IteratorExtScanLeft for T {}
}
