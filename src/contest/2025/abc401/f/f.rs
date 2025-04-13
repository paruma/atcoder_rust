//#[derive_readable]
#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    u: Usize1,
    v: Usize1,
}
#[derive(Debug, Clone)]
struct Problem {
    nv1: usize,
    nv2: usize,
    es1: Vec<Edge>,
    es2: Vec<Edge>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            nv1: usize,
            es1: [Edge; nv1-1],
            nv2: usize,
            es2: [Edge; nv2-1],

        }
        Problem { nv1, nv2, es1, es2 }
    }

    fn solve(&self) -> Answer {
        let nv1 = self.nv1;
        let nv2 = self.nv2;

        let adj1 = self
            .es1
            .iter()
            .copied()
            .fold(vec![vec![]; nv1], |mut acc, e| {
                acc[e.u].push(e.v);
                acc[e.v].push(e.u);
                acc
            });
        let adj2 = self
            .es2
            .iter()
            .copied()
            .fold(vec![vec![]; nv2], |mut acc, e| {
                acc[e.u].push(e.v);
                acc[e.v].push(e.u);
                acc
            });

        let max_dist1 = DistMaxReroot {}
            .reroot(&adj1)
            .iter()
            .map(|i| (i - 1) as i64)
            .collect_vec();
        let max_dist2 = DistMaxReroot {}
            .reroot(&adj2)
            .iter()
            .map(|i| (i - 1) as i64)
            .sorted()
            .collect_vec();

        let max_dist2_cumsum = CumSum::new(&max_dist2);

        let diam1 = max_dist1.iter().copied().max().unwrap();
        let diam2 = max_dist2.iter().copied().max().unwrap();

        let max_diam = i64::max(diam1, diam2);

        let ans = (0..nv1)
            .map(|i| {
                // max_dist1[i] + 1 + max_dist2[j] < max_diam
                // max_dist2[j] < max_diam - max_dist1[i] - 1
                // だったら max_diam を使う

                let bound = max_dist2.lower_bound(&(max_diam - max_dist1[i] - 1));
                let sub_ans = bound as i64 * max_diam
                    + (nv2 - bound) as i64 * (max_dist1[i] + 1)
                    + max_dist2_cumsum.range_sum(bound..nv2);
                dbg!(bound);
                dbg!(sub_ans);
                sub_ans
            })
            .sum::<i64>();
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

use ac_library::{Max, Monoid};
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
use std::convert::Infallible;
use superslice::Ext;

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

fn make_adj(nv: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; nv];
    for &(v1, v2) in edges {
        adj[v1].push(v2);
        adj[v2].push(v1);
    }
    adj
}

fn bfs_order(adj: &[Vec<usize>], init: usize) -> Vec<usize> {
    let nv = adj.len();
    let mut order = vec![];
    let mut visited = vec![false; nv];
    let mut open = Queue::new();
    open.push(init);
    order.push(init);
    visited[init] = true;
    while let Some(current) = open.pop() {
        for &next in &adj[current] {
            if !visited[next] {
                order.push(next);
                visited[next] = true;
                open.push(next);
            }
        }
    }
    order
}

fn dfs_post_order(adj: &[Vec<usize>], init: usize) -> Vec<usize> {
    enum State {
        Pre(usize),
        Post(usize),
    }

    let nv = adj.len();
    let mut order = vec![];
    let mut visited = vec![false; nv];
    let mut open = Stack::new();
    open.push(State::Post(init));
    open.push(State::Pre(init));
    while let Some(current) = open.pop() {
        match current {
            State::Pre(v) => {
                visited[v] = true;
                for &edge in &adj[v] {
                    if !visited[edge] {
                        open.push(State::Post(edge));
                        open.push(State::Pre(edge));
                    }
                }
            }
            State::Post(v) => {
                // 帰りがけ
                order.push(v);
            }
        }
    }
    order
}

/// 全方位木DP
trait Reroot {
    type M: Monoid; // 可換モノイド

    fn add_root(&self, x: &<Self::M as Monoid>::S, root: usize) -> <Self::M as Monoid>::S;

    fn prod(xs: &[<Self::M as Monoid>::S]) -> <Self::M as Monoid>::S {
        xs.iter().fold(Self::M::identity(), |acc, x| {
            Self::M::binary_operation(&acc, x)
        })
    }

    fn reroot(&self, adj: &[Vec<usize>]) -> Vec<<Self::M as Monoid>::S> {
        let nv = adj.len();
        // dp[v][i]: 頂点v から生える i番目の有向辺の先にある部分木に関する値
        let mut dp: Vec<Vec<<Self::M as Monoid>::S>> = adj
            .iter()
            .map(|edges| {
                let degree = edges.len();
                vec![Self::M::identity(); degree]
            })
            .collect_vec();

        {
            // 頂点を DFS 帰りかけ順に並べたもの
            let dfs_post_order = dfs_post_order(adj, 0);
            let mut visited = vec![false; nv];

            for &current in &dfs_post_order {
                visited[current] = true;

                for (edge_i, next) in adj[current].iter().copied().enumerate() {
                    if !visited[next] {
                        continue;
                    }

                    dp[current][edge_i] = {
                        let val = Self::prod(&dp[next]);
                        self.add_root(&val, next)
                    };
                }
            }
        }
        {
            // 頂点を BFS の訪問順に並べたもの
            let bfs_order = bfs_order(adj, 0);
            let mut visited = vec![false; nv];
            for &current in &bfs_order {
                visited[current] = true;
                let cum_monoid = CumMonoid::<Self::M>::new(&dp[current]);
                for (edge_i, next) in adj[current].iter().copied().enumerate() {
                    if visited[next] {
                        continue;
                    }
                    // 償却 O(1) で計算可能
                    let rev_edge_i = adj[next].iter().position(|&v| v == current).unwrap();

                    dp[next][rev_edge_i] = {
                        let val = cum_monoid.prod_without1(edge_i);
                        self.add_root(&val, current)
                    };
                }
            }
        }
        dp.iter()
            .enumerate()
            .map(|(v, dp_v)| self.add_root(&Self::prod(dp_v), v))
            .collect_vec()
    }
}

struct DistMaxReroot();
impl Reroot for DistMaxReroot {
    type M = Max<u64>;

    fn add_root(&self, x: &<Self::M as Monoid>::S, _root: usize) -> <Self::M as Monoid>::S {
        x + 1
    }
}
use cum_monoid::*;
pub mod cum_monoid {
    use ac_library::{Max, Min, Monoid};
    pub struct CumMonoid<M>
    where
        M: Monoid,
    {
        prefix_prod: Vec<M::S>,
        suffix_prod: Vec<M::S>,
    }
    impl<M> CumMonoid<M>
    where
        M: Monoid,
    {
        pub fn new(xs: &[M::S]) -> CumMonoid<M> {
            let mut prefix_prod = vec![M::identity(); xs.len() + 1];
            let mut suffix_prod = vec![M::identity(); xs.len() + 1];
            for i in 0..xs.len() {
                prefix_prod[i + 1] = M::binary_operation(&prefix_prod[i], &xs[i]);
            }
            for i in (0..xs.len()).rev() {
                suffix_prod[i] = M::binary_operation(&xs[i], &suffix_prod[i + 1]);
            }
            CumMonoid {
                prefix_prod,
                suffix_prod,
            }
        }
        /// [0, i) の総積 (前から累積)
        pub fn prefix_prod(&self, i: usize) -> M::S {
            self.prefix_prod[i].clone()
        }
        /// [i, n) の総積 (後ろから累積)
        pub fn suffix_prod(&self, i: usize) -> M::S {
            self.suffix_prod[i].clone()
        }
        /// [0, i), [i + 1, n) の区間で総積を取る
        pub fn prod_without1(&self, i: usize) -> M::S {
            M::binary_operation(&self.prefix_prod[i], &self.suffix_prod[i + 1])
        }
        pub fn prod_without_range(&self, l: usize, r: usize) -> M::S {
            M::binary_operation(&self.prefix_prod[l], &self.suffix_prod[r])
        }
    }
    pub struct CumMin {
        cum: CumMonoid<Min<i64>>,
    }
    impl CumMin {
        pub fn new(xs: &[i64]) -> CumMin {
            CumMin {
                cum: CumMonoid::new(xs),
            }
        }
        /// [0, i) の総積 (前から累積)
        pub fn prefix_min(&self, i: usize) -> i64 {
            self.cum.prefix_prod(i)
        }
        /// [i, n) の総積 (後ろから累積)
        pub fn suffix_min(&self, i: usize) -> i64 {
            self.cum.suffix_prod(i)
        }
        /// [0, i), [i + 1, n) の区間で総積を取る
        pub fn min_without1(&self, i: usize) -> i64 {
            self.cum.prod_without1(i)
        }
        pub fn min_without_range(&self, l: usize, r: usize) -> i64 {
            self.cum.prod_without_range(l, r)
        }
    }
    pub struct CumMax {
        cum: CumMonoid<Max<i64>>,
    }
    impl CumMax {
        pub fn new(xs: &[i64]) -> CumMax {
            CumMax {
                cum: CumMonoid::new(xs),
            }
        }
        /// [0, i) の総積 (前から累積)
        pub fn prefix_max(&self, i: usize) -> i64 {
            self.cum.prefix_prod(i)
        }
        /// [i, n) の総積 (後ろから累積)
        pub fn suffix_max(&self, i: usize) -> i64 {
            self.cum.suffix_prod(i)
        }
        /// [0, i), [i + 1, n) の区間で総積を取る
        pub fn max_without1(&self, i: usize) -> i64 {
            self.cum.prod_without1(i)
        }
        pub fn max_without_range(&self, l: usize, r: usize) -> i64 {
            self.cum.prod_without_range(l, r)
        }
    }
}
use mod_stack::*;
pub mod mod_stack {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Stack<T> {
        raw: Vec<T>,
    }
    impl<T> Stack<T> {
        pub fn new() -> Self {
            Stack { raw: Vec::new() }
        }
        pub fn push(&mut self, value: T) {
            self.raw.push(value)
        }
        pub fn pop(&mut self) -> Option<T> {
            self.raw.pop()
        }
        pub fn peek(&self) -> Option<&T> {
            self.raw.last()
        }
        pub fn is_empty(&self) -> bool {
            self.raw.is_empty()
        }
        pub fn len(&self) -> usize {
            self.raw.len()
        }
    }
    impl<T> Default for Stack<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}

use mod_queue::*;
pub mod mod_queue {
    use std::collections::VecDeque;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Queue<T> {
        raw: VecDeque<T>,
    }
    impl<T> Queue<T> {
        pub fn new() -> Self {
            Queue {
                raw: VecDeque::new(),
            }
        }
        pub fn push(&mut self, value: T) {
            self.raw.push_back(value)
        }
        pub fn pop(&mut self) -> Option<T> {
            self.raw.pop_front()
        }
        pub fn peek(&self) -> Option<&T> {
            self.raw.front()
        }
        pub fn is_empty(&self) -> bool {
            self.raw.is_empty()
        }
        pub fn len(&self) -> usize {
            self.raw.len()
        }
    }
    impl<T> Default for Queue<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}
use cumsum::*;
pub mod cumsum {
    pub fn prefix_sum(xs: &[i64]) -> Vec<i64> {
        let mut prefix_sum = vec![0; xs.len() + 1];
        for i in 1..xs.len() + 1 {
            prefix_sum[i] = prefix_sum[i - 1] + xs[i - 1];
        }
        prefix_sum
    }
    use std::ops::{Bound, Range, RangeBounds};
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct CumSum {
        pub cumsum: Vec<i64>,
    }
    impl CumSum {
        /// 計算量: O(|xs|)
        pub fn new(xs: &[i64]) -> CumSum {
            let mut cumsum = vec![0; xs.len() + 1];
            for i in 1..xs.len() + 1 {
                cumsum[i] = cumsum[i - 1] + xs[i - 1];
            }
            CumSum { cumsum }
        }
        fn open(&self, range: impl RangeBounds<usize>) -> Range<usize> {
            use Bound::Excluded;
            use Bound::Included;
            use Bound::Unbounded;
            let begin = match range.start_bound() {
                Unbounded => 0,
                Included(&x) => x,
                Excluded(&x) => x + 1,
            };
            let end = match range.end_bound() {
                Excluded(&x) => x,
                Included(&x) => x + 1,
                Unbounded => self.cumsum.len() - 1,
            };
            begin..end
        }
        /// 計算量: O(1)
        pub fn range_sum(&self, range: impl RangeBounds<usize>) -> i64 {
            let range = self.open(range);
            self.cumsum[range.end] - self.cumsum[range.start]
        }
        pub fn prefix_sum(&self, end: usize) -> i64 {
            self.cumsum[end]
        }
        pub fn suffix_sum(&self, begin: usize) -> i64 {
            self.cumsum[self.cumsum.len() - 1] - self.cumsum[begin]
        }
    }
}
