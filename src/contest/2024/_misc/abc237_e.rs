//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    nv: usize,
    ne: usize,
    hs: Vec<i64>,
    edges: Vec<(usize, usize)>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            nv: usize,
            ne: usize,
            hs: [i64; nv],
            edges: [(Usize1, Usize1); ne],
        }
        Problem { nv, ne, hs, edges }
    }
    fn score_diff(&self, from: usize, to: usize) -> i64 {
        if self.hs[from] >= self.hs[to] {
            self.hs[from] - self.hs[to]
        } else {
            -2 * (self.hs[to] - self.hs[from])
        }
    }
    fn solve(&self) -> Answer {
        // 負辺を含むため、ダイクストラ法だとTLEする。
        let adj = self
            .edges
            .iter()
            .copied()
            .fold(vec![vec![]; self.nv], |mut acc, (u, v)| {
                acc[u].push(v);
                acc[v].push(u);
                acc
            });

        let mut pq: BinaryHeap<(NegExtInt, usize)> = BinaryHeap::new();
        let mut scores = vec![NegInf; self.nv];

        scores[0] = Fin(0);
        pq.push((Fin(0), 0));

        while let Some((score, current)) = pq.pop() {
            if scores[current] > score {
                continue;
            }
            for &next in &adj[current] {
                if chmax!(
                    scores[next],
                    scores[current] + Fin(self.score_diff(current, next))
                ) {
                    pq.push((scores[next], next));
                }
            }
        }

        let ans = scores.iter().copied().max().unwrap().get_fin();
        Answer { ans }
    }
}
impl Problem {
    fn potential_diff(&self, from: usize, to: usize) -> i64 {
        if self.hs[from] >= self.hs[to] {
            0
        } else {
            -(self.hs[to] - self.hs[from])
        }
    }
    fn solve2(&self) -> Answer {
        // ポテンシャルを使ってダイクストラ法を解く
        let adj = self
            .edges
            .iter()
            .copied()
            .fold(vec![vec![]; self.nv], |mut acc, (u, v)| {
                acc[u].push(v);
                acc[v].push(u);
                acc
            });
        let mut potential_list = vec![NegInf; self.nv];
        let mut pq = BinaryHeap::<(NegExtInt, usize)>::new();
        potential_list[0] = Fin(self.hs[0]);
        pq.push((potential_list[0], 0));

        while let Some((potential, current)) = pq.pop() {
            if potential_list[current] > potential {
                continue;
            }
            for &next in &adj[current] {
                if chmax!(
                    potential_list[next],
                    potential_list[current] + self.potential_diff(current, next)
                ) {
                    pq.push((potential_list[next], next));
                }
            }
        }

        let ans = izip!(potential_list, &self.hs)
            .map(|(potential, height)| potential + Fin(-height))
            .max()
            .unwrap()
            .get_fin();

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
use mod_neg_ext_int::NegExtInt::{self, *};
pub mod mod_neg_ext_int {
    use ac_library::Monoid;
    use std::{
        cmp::Ordering,
        convert::Infallible,
        ops::{Add, AddAssign},
    };
    use NegExtInt::*;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NegExtInt {
        NegInf,
        Fin(i64),
    }
    impl NegExtInt {
        pub fn get_fin(self) -> i64 {
            match self {
                Fin(val) => val,
                NegInf => panic!("called `NegExtInt::get_fin()` on a `Fin` value"),
            }
        }
        pub fn get_fin_or(self, default: i64) -> i64 {
            match self {
                Fin(val) => val,
                NegInf => default,
            }
        }
        pub fn is_fin(self) -> bool {
            matches!(self, Fin(_))
        }
        pub fn is_neginf(self) -> bool {
            matches!(self, NegInf)
        }
        pub fn to_option(self) -> Option<i64> {
            match self {
                NegInf => None,
                Fin(a) => Some(a),
            }
        }
        pub fn from_option(opt: Option<i64>) -> NegExtInt {
            match opt {
                Some(a) => Fin(a),
                None => NegInf,
            }
        }
        pub fn times(self, t: i64) -> Self {
            match t.cmp(&0) {
                Ordering::Less => panic!("t must be non-negative."),
                Ordering::Equal => Fin(0),
                Ordering::Greater => match self {
                    NegInf => NegInf,
                    Fin(a) => Fin(a * t),
                },
            }
        }
    }
    impl Add for NegExtInt {
        type Output = NegExtInt;
        fn add(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (NegInf, NegInf) => NegInf,
                (NegInf, Fin(_)) => NegInf,
                (Fin(_), NegInf) => NegInf,
                (Fin(a), Fin(b)) => Fin(a + b),
            }
        }
    }
    impl AddAssign for NegExtInt {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }
    impl Add<i64> for NegExtInt {
        type Output = NegExtInt;
        fn add(self, rhs: i64) -> Self::Output {
            match self {
                NegInf => NegInf,
                Fin(a) => Fin(a + rhs),
            }
        }
    }
    impl AddAssign<i64> for NegExtInt {
        fn add_assign(&mut self, rhs: i64) {
            *self = *self + rhs;
        }
    }
    impl std::iter::Sum for NegExtInt {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            let mut s = 0;
            for x in iter {
                match x {
                    NegInf => return NegInf,
                    Fin(x) => s += x,
                }
            }
            Fin(s)
        }
    }
    impl PartialOrd for NegExtInt {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match (self, other) {
                (NegInf, NegInf) => Some(Ordering::Equal),
                (NegInf, Fin(_)) => Some(Ordering::Less),
                (Fin(_), NegInf) => Some(Ordering::Greater),
                (Fin(a), Fin(b)) => PartialOrd::partial_cmp(a, b),
            }
        }
    }
    impl Ord for NegExtInt {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }
    pub struct NegExtIntAdditive(Infallible);
    impl Monoid for NegExtIntAdditive {
        type S = NegExtInt;
        fn identity() -> Self::S {
            Fin(0)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
    }
    pub struct NegExtIntMax(Infallible);
    impl Monoid for NegExtIntMax {
        type S = NegExtInt;
        fn identity() -> Self::S {
            NegInf
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a.max(b)
        }
    }
}
#[allow(clippy::module_inception)]
#[macro_use]
pub mod chminmax {
    #[allow(unused_macros)]
    #[macro_export]
    macro_rules! chmin {
        ($ a : expr , $ b : expr ) => {
            if $a > $b {
                $a = $b;
                true
            } else {
                false
            }
        };
    }
    #[allow(unused_macros)]
    #[macro_export]
    macro_rules! chmax {
        ($ a : expr , $ b : expr ) => {
            if $a < $b {
                $a = $b;
                true
            } else {
                false
            }
        };
    }
}
