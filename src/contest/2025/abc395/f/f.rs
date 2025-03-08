#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    x: i64,
    us: Vec<i64>,
    ds: Vec<i64>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            x: i64,
            uds: [(i64, i64); n]
        }

        let us = uds.iter().copied().map(|x| x.0).collect_vec();
        let ds = uds.iter().copied().map(|x| x.1).collect_vec();
        Problem { n, x, us, ds }
    }

    fn solve(&self) -> Answer {
        // 二分探索
        let n = self.n;
        let x = self.x;
        let us = &self.us;
        let ds = &self.ds;

        // 下の歯の範囲を求める

        let h = bin_search(-1, 2_000_000_001, |h| {
            // [lower, upper]
            let mut lower = (0..n).map(|i| i64::max(0, h - us[i])).collect_vec();
            let mut upper = (0..n).map(|i| i64::min(ds[i], h)).collect_vec();

            for i in 0..n - 1 {
                lower[i + 1] = i64::max(lower[i + 1], lower[i] - x);
                upper[i + 1] = i64::min(upper[i + 1], upper[i] + x);
            }
            (0..n).all(|i| lower[i] <= upper[i])
        });

        let ans = us.iter().sum::<i64>() + ds.iter().sum::<i64>() - (n as i64 * h);
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // 差分制約系の最適化問題を立ててダイクストラ法をする

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        struct Edge {
            from: usize,
            to: usize,
            cost: i64,
        }

        impl Edge {
            fn new(from: usize, to: usize, cost: i64) -> Edge {
                Edge { from, to, cost }
            }
        }

        fn dijkstra(adj: &[Vec<Edge>], start: usize) -> Vec<ExtInt> {
            let n_vertex = adj.len();
            let mut pq: BinaryHeap<(Reverse<ExtInt>, usize)> = BinaryHeap::new();
            let mut dist = vec![INF; n_vertex];
            dist[start] = fin(0);
            pq.push((Reverse(fin(0)), start));

            while let Some((Reverse(d), current)) = pq.pop() {
                if dist[current] < d {
                    continue;
                }
                for e in &adj[current] {
                    if chmin!(dist[e.to], dist[e.from] + fin(e.cost)) {
                        pq.push((Reverse(dist[e.to]), e.to));
                    }
                }
            }
            dist
        }
        let nv = self.n + 2;

        // x[i] (0 <= i < n) はi番目の歯の上下境界の高さ
        // x[n] は下顎の高さ
        // x[n+1]は上顎の高さ
        // とすると、以下の差分制約系を解けば良いとなる
        // maximize x[n+1] - x[n]
        // s.t.
        // - 0 <= x[i] - x[n] <= D[i]
        // - 0 <= x[n+1] - x[i] <= U[i]
        // - |x[i] - x[i+1]| <= X
        let edges = {
            let mut edges = vec![];
            for i in 0..self.n {
                edges.push(Edge::new(self.n, i, self.ds[i]));
                edges.push(Edge::new(i, self.n, 0));

                edges.push(Edge::new(i, self.n + 1, self.us[i]));
                edges.push(Edge::new(self.n + 1, i, 0));
            }

            for i in 0..(self.n - 1) {
                edges.push(Edge::new(i, i + 1, self.x));
                edges.push(Edge::new(i + 1, i, self.x));
            }
            edges
        };

        let adj = edges.iter().fold(vec![vec![]; nv], |mut acc, e| {
            acc[e.from].push(*e);
            acc
        });

        let dist = dijkstra(&adj, self.n);

        let h = dist[self.n + 1].get_fin();
        let ans = self.us.iter().sum::<i64>() + self.ds.iter().sum::<i64>() - (self.n as i64 * h);
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
/// 二分探索をする
/// ```text
/// ng ng ng ok ok ok
///          ↑ここの引数の値を返す
/// ```
/// 計算量: O(log(|ok - ng|))
/// ## Arguments
/// * ok != ng
/// * |ok - ng| <= 2^63 - 1, |ok + ng| <= 2^63 - 1
/// * p の定義域について
///     * ng < ok の場合、p は区間 ng..ok で定義されている。
///     * ok < ng の場合、p は区間 ok..ng で定義されている。
/// * p の単調性について
///     * ng < ok の場合、p は単調増加
///     * ok < ng の場合、p は単調減少
/// ## Return
/// * ng < ok の場合: I = { i in ng..ok | p(i) == true } としたとき
///     * I が空でなければ、min I を返す。
///     * I が空ならば、ok を返す。
/// * ok < ng の場合: I = { i in ok..ng | p(i) == true } としたとき
///     * I が空でなければ、max I を返す。
///     * I が空ならば、ok を返す。
pub fn bin_search<F>(mut ok: i64, mut ng: i64, mut p: F) -> i64
where
    F: FnMut(i64) -> bool,
{
    debug_assert!(ok != ng);
    debug_assert!(ok.checked_sub(ng).is_some());
    debug_assert!(ok.checked_add(ng).is_some());
    while num::abs(ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        debug_assert!(mid != ok);
        debug_assert!(mid != ng);
        if p(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
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
use mod_ext_int::*;
pub mod mod_ext_int {
    use ac_library::Monoid;
    use std::{
        cmp::Ordering,
        convert::Infallible,
        fmt,
        ops::{Add, AddAssign, Sub, SubAssign},
    };
    pub const INF: ExtInt = ExtInt::INF;
    pub fn fin(x: i64) -> ExtInt {
        ExtInt::fin(x)
    }
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ExtInt(i64);
    impl ExtInt {
        pub const INF: Self = Self(i64::MAX);
        pub fn fin(x: i64) -> Self {
            Self(x)
        }
        pub fn get_fin(self) -> i64 {
            if self.is_fin() {
                self.0
            } else {
                panic!("called `ExtInt::get_fin()` on a infinity")
            }
        }
        pub fn get_fin_or(self, default: i64) -> i64 {
            if self.is_fin() {
                self.0
            } else {
                default
            }
        }
        #[inline]
        pub fn is_fin(self) -> bool {
            self.0 != i64::MAX
        }
        pub fn is_inf(self) -> bool {
            self.0 == i64::MAX
        }
        pub fn to_option(self) -> Option<i64> {
            if self.is_fin() {
                Some(self.0)
            } else {
                None
            }
        }
        pub fn from_option(opt: Option<i64>) -> ExtInt {
            match opt {
                Some(a) => Self(a),
                None => Self::INF,
            }
        }
        pub fn times(self, t: i64) -> Self {
            match t.cmp(&0) {
                Ordering::Less => panic!("t must be non-negative."),
                Ordering::Equal => Self(0),
                Ordering::Greater => {
                    if self.is_fin() {
                        Self(self.0 * t)
                    } else {
                        Self::INF
                    }
                }
            }
        }
    }
    impl Add for ExtInt {
        type Output = ExtInt;
        fn add(self, rhs: Self) -> Self::Output {
            if self.is_inf() || rhs.is_inf() {
                Self::INF
            } else {
                Self::fin(self.0 + rhs.0)
            }
        }
    }
    impl AddAssign for ExtInt {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }
    impl Add<i64> for ExtInt {
        type Output = ExtInt;
        fn add(self, rhs: i64) -> Self::Output {
            if self.is_inf() {
                Self::INF
            } else {
                Self::fin(self.0 + rhs)
            }
        }
    }
    impl AddAssign<i64> for ExtInt {
        fn add_assign(&mut self, rhs: i64) {
            *self = *self + rhs;
        }
    }
    impl Sub<i64> for ExtInt {
        type Output = ExtInt;
        fn sub(self, rhs: i64) -> Self::Output {
            if self.is_inf() {
                Self::INF
            } else {
                Self::fin(self.0 - rhs)
            }
        }
    }
    impl SubAssign<i64> for ExtInt {
        fn sub_assign(&mut self, rhs: i64) {
            *self = *self - rhs;
        }
    }
    impl std::iter::Sum for ExtInt {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            let mut s = 0;
            for x in iter {
                if x.is_inf() {
                    return Self::INF;
                }
                s += x.0;
            }
            Self::fin(s)
        }
    }
    impl fmt::Display for ExtInt {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.is_inf() {
                write!(f, "+∞")
            } else {
                write!(f, "{}", self.0)
            }
        }
    }
    impl fmt::Debug for ExtInt {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.is_inf() {
                write!(f, "+∞")
            } else {
                write!(f, "{}", self.0)
            }
        }
    }
    pub struct ExtIntAdditive(Infallible);
    impl Monoid for ExtIntAdditive {
        type S = ExtInt;
        fn identity() -> Self::S {
            ExtInt::fin(0)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
    }
    pub struct ExtIntMin(Infallible);
    impl Monoid for ExtIntMin {
        type S = ExtInt;
        fn identity() -> Self::S {
            ExtInt::INF
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a.min(b)
        }
    }
}
