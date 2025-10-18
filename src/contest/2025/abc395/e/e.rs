#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    from: Usize1,
    to: Usize1,
}
#[derive(Debug, Clone)]
struct Problem {
    nv: usize,
    ne: usize,
    x: i64,
    es: Vec<Edge>,
}

macro_rules! chmin {
    ($a: expr, $b: expr) => {
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    };
}
impl Problem {
    fn read() -> Problem {
        input! {
            nv: usize,
            ne: usize,
            x: i64,
            es: [Edge; ne],
        }
        Problem { nv, ne, x, es }
    }

    fn solve(&self) -> Answer {
        // ダイクストラ法を問題に合わせて実装
        let nv = self.nv;
        let ne = self.ne;
        let x = self.x;

        let normal_adj: Vec<Vec<usize>> = self.es.iter().fold(vec![vec![]; nv], |mut acc, e| {
            acc[e.from].push(e.to);
            acc
        });

        let rev_adj: Vec<Vec<usize>> = self.es.iter().fold(vec![vec![]; nv], |mut acc, e| {
            acc[e.to].push(e.from);
            acc
        });

        let adj = [normal_adj, rev_adj];

        let mut pq: BinaryHeap<(Reverse<ExtInt>, (usize, usize))> = BinaryHeap::new();
        let mut dist = vec![[INF; 2]; nv];
        dist[0][0] = fin(0);
        pq.push((Reverse(fin(0)), (0, 0)));

        while let Some((Reverse(d), (current_pos, current_is_rev))) = pq.pop() {
            if dist[current_pos][current_is_rev] < d {
                continue;
            }
            for &next in &adj[current_is_rev][current_pos] {
                if chmin!(
                    dist[next][current_is_rev],
                    dist[current_pos][current_is_rev] + fin(1)
                ) {
                    pq.push((Reverse(dist[next][current_is_rev]), (next, current_is_rev)));
                }
            }
            // 反転世界に行く
            if chmin!(
                dist[current_pos][1 - current_is_rev],
                dist[current_pos][current_is_rev] + fin(x)
            ) {
                pq.push((
                    Reverse(dist[current_pos][1 - current_is_rev]),
                    (current_pos, 1 - current_is_rev),
                ));
            }
        }
        // dbg!(&dist);

        let ans = std::cmp::min(dist[nv - 1][0], dist[nv - 1][1]).get_fin();
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // グラフを再構築して、ダイクストラ法ライブラリを使用
        // solve と比べると、多少メモリを消費し、実行時間が長くなる。

        let nv = self.nv;
        let ext_es = {
            let mut ext_es = vec![];
            let origin_es = &self.es;
            for e in origin_es {
                ext_es.push(EdgeCost::new(e.from, e.to, 1));
                ext_es.push(EdgeCost::new(e.to + nv, e.from + nv, 1));
            }

            for v in 0..nv {
                ext_es.push(EdgeCost::new(v, v + nv, self.x));
                ext_es.push(EdgeCost::new(v + nv, v, self.x));
            }

            ext_es
        };

        let ext_nv = 2 * nv;

        let adj = ext_es
            .iter()
            .copied()
            .fold(vec![vec![]; ext_nv], |mut acc, e| {
                acc[e.from].push(e);
                acc
            });

        let dist = dijkstra(&adj, 0);
        let ans = std::cmp::min(dist[nv - 1], dist[nv - 1 + nv]).get_fin();
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
fn dijkstra(adj: &[Vec<EdgeCost>], start: usize) -> Vec<ExtInt> {
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct EdgeCost {
    from: usize,
    to: usize,
    cost: i64,
}
impl EdgeCost {
    fn new(from: usize, to: usize, cost: i64) -> Self {
        Self { from, to, cost }
    }
}
