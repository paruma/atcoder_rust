#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Query {
    begin: Usize1,
    end: usize,
    sum: i64,
}

fn solve(n: usize, m: usize, qs: &[Query]) -> Option<i64> {
    let nv = n + 1;

    let mut es = vec![];
    for i in 0..n {
        es.push(bellman_ford::Edge::new(i + 1, i, -1));
    }

    for q in qs {
        es.push(bellman_ford::Edge::new(q.begin, q.end, q.sum));
        es.push(bellman_ford::Edge::new(q.end, q.begin, -q.sum));
    }
    let primal_solutions = bellman_ford::bellman_ford(&es, nv, nv - 1);
    let dual_solution = primal_solutions[0].dual();
    match dual_solution {
        bellman_ford::Solution::Unbounded => panic!(), // 今回の問題ではあり得ない。
        bellman_ford::Solution::Bounded(x) => Some(-x),
        bellman_ford::Solution::Infeasible => None,
    }
}
fn main() {
    input! {
        n: usize,
        m: usize,
        qs: [Query; m],
    }

    let ans: Option<i64> = solve(n, m, &qs);
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("{}", -1);
    }
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
#[allow(clippy::module_inception)]
pub mod bellman_ford {
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    /// 最短路問題の解。双対問題を考える場合は非有界と非許容が逆になるので注意
    pub enum Solution {
        /// 非有界 (負閉路を経由することでいくらでもパスを短くできる。-∞ 扱い)
        Unbounded,
        /// 有界
        Bounded(i64),
        /// 非許容 (連結でなくて到達不可能。 +∞ 扱い)
        Infeasible,
    }
    impl Solution {
        /// 双対問題の解に変換する (非有界と非許容が入れ替わる)
        pub fn dual(&self) -> Self {
            match self {
                Solution::Unbounded => Solution::Infeasible,
                Solution::Bounded(x) => Solution::Bounded(*x),
                Solution::Infeasible => Solution::Unbounded,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Edge {
        from: usize,
        to: usize,
        cost: i64,
    }
    impl Edge {
        pub fn new(from: usize, to: usize, cost: i64) -> Self {
            Self { from, to, cost }
        }
    }
    pub fn bellman_ford(edges: &[Edge], n_vertex: usize, start: usize) -> Vec<Solution> {
        let mut dist = vec![INF; n_vertex];
        dist[start] = fin(0);
        for _ in 0..n_vertex {
            for edge in edges {
                chmin!(dist[edge.to], dist[edge.from] + fin(edge.cost));
            }
        }
        let mut adj = vec![vec![]; n_vertex];
        for edge in edges {
            adj[edge.from].push(edge.to);
        }
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        let mut is_unbounded = vec![false; n_vertex];
        let mut visited = vec![false; n_vertex];
        for edge in edges {
            if dist[edge.to] > dist[edge.from] + fin(edge.cost) {
                if !visited[edge.to] {
                    visited[edge.to] = true;
                    queue.push_back(edge.to);
                }
            }
        }
        while let Some(u) = queue.pop_front() {
            is_unbounded[u] = true;
            for &v in &adj[u] {
                if !visited[v] {
                    visited[v] = true;
                    queue.push_back(v);
                }
            }
        }
        dist.into_iter()
            .zip(is_unbounded)
            .map(|(d, unbounded)| {
                if unbounded {
                    Solution::Unbounded
                } else if d.is_inf() {
                    Solution::Infeasible
                } else {
                    Solution::Bounded(d.get_fin())
                }
            })
            .collect()
    }
    use mod_ext_int::*;
    mod mod_ext_int {
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
}
