//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    m: usize,
    xs: Vec<usize>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            m: usize,
            xs: [Usize1; n],
        }
        Problem { n, m, xs }
    }

    fn solve(&self) -> Answer {
        use ac_library::ModInt998244353 as Mint;
        let n = self.n;
        let m = self.m;

        let mut scc_graph = SccGraph::new(n);
        for to in 0..n {
            let from = self.xs[to];
            scc_graph.add_edge(from, to);
        }

        let scc = scc_graph.scc();

        let to_scc_idx = {
            let mut to_scc_idx = vec![0; n];
            for (i, component) in scc.iter().enumerate() {
                for v in component {
                    to_scc_idx[*v] = i;
                }
            }
            to_scc_idx
        };

        let scc_adj = {
            let mut scc_adj = vec![HashSet::<usize>::new(); scc.len()];
            for to in 0..n {
                let from = self.xs[to];
                let from_scc_idx = to_scc_idx[from];
                let to_scc_idx = to_scc_idx[to];
                if from_scc_idx != to_scc_idx {
                    scc_adj[from_scc_idx].insert(to_scc_idx);
                }
            }
            scc_adj
                .iter()
                .map(|s| s.iter().copied().collect_vec())
                .collect_vec()
        };

        let scc_in_degree = {
            let mut scc_in_degree = vec![0; scc.len()];
            for to in scc_adj.iter().flatten().copied() {
                scc_in_degree[to] += 1;
            }
            scc_in_degree
        };

        let mut dp = (0..scc.len())
            .map(|_| Segtree::<MintAdditive<Mod998244353>>::from(vec![Mint::new(0); m]))
            .collect_vec();

        for i in (0..scc.len()).rev() {
            for k in 0..m {
                let next_val = scc_adj[i]
                    .iter()
                    .copied()
                    .map(|next| dp[next].prod(..=k))
                    .product();
                //
                dp[i].set(k, next_val);
            }
        }

        let ans = (0..scc.len())
            .filter(|i| scc_in_degree[*i] == 0)
            .map(|i| dp[i].all_prod())
            .product::<Mint>()
            .val() as i64;
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

use ac_library::{Mod998244353, SccGraph, Segtree};
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
use monoid_modint::*;
pub mod monoid_modint {
    use ac_library::{Modulus, Monoid, StaticModInt};
    use std::{convert::Infallible, marker::PhantomData};
    pub struct MintAdditive<Mod>(Infallible, PhantomData<fn() -> Mod>);
    impl<Mod> Monoid for MintAdditive<Mod>
    where
        Mod: Modulus,
    {
        type S = StaticModInt<Mod>;
        fn identity() -> Self::S {
            StaticModInt::raw(0)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a + b
        }
    }
    pub struct MintMultiplicative<Mod>(Infallible, PhantomData<fn() -> Mod>);
    impl<Mod> Monoid for MintMultiplicative<Mod>
    where
        Mod: Modulus,
    {
        type S = StaticModInt<Mod>;
        fn identity() -> Self::S {
            StaticModInt::raw(1)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a * b
        }
    }
}
