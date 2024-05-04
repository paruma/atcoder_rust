// #[derive_readable]

#[derive(Debug)]
struct Problem {
    nv: usize,
    edges: Vec<(usize, usize)>,
    costs: Vec<i64>,
}

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

    fn reroot(&self, adj: &[Vec<usize>]) -> Vec<<Self::M as Monoid>::S> where {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CostDistSum {
    cost_sum: i64,
    cost_dist_sum: i64,
}

pub struct CostDistSumMonoid(Infallible);
impl Monoid for CostDistSumMonoid {
    type S = CostDistSum;
    fn identity() -> Self::S {
        CostDistSum {
            cost_sum: 0,
            cost_dist_sum: 0,
        }
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        CostDistSum {
            cost_sum: a.cost_sum + b.cost_sum,
            cost_dist_sum: a.cost_dist_sum + b.cost_dist_sum,
        }
    }
}
struct CostDistSumReroot {
    cost_list: Vec<i64>,
}
impl CostDistSumReroot {
    fn new(costs: &[i64]) -> Self {
        Self {
            cost_list: costs.to_vec(),
        }
    }
}
impl Reroot for CostDistSumReroot {
    type M = CostDistSumMonoid;

    fn add_root(&self, x: &<Self::M as Monoid>::S, root: usize) -> <Self::M as Monoid>::S {
        CostDistSum {
            cost_sum: x.cost_sum + self.cost_list[root],
            cost_dist_sum: x.cost_dist_sum + x.cost_sum,
        }
    }
}
struct DistMaxReroot();
impl Reroot for DistMaxReroot {
    type M = Max<u64>;

    fn add_root(&self, x: &<Self::M as Monoid>::S, _root: usize) -> <Self::M as Monoid>::S {
        x + 1
    }
}
impl Problem {
    fn read() -> Problem {
        input! {
            nv: usize,
            edges: [(Usize1, Usize1); nv - 1],
            costs: [i64; nv],
        }
        Problem { nv, edges, costs }
    }
    fn solve(&self) -> Answer {
        let nv = self.nv;
        let adj = make_adj(nv, &self.edges);
        let ans = CostDistSumReroot::new(&self.costs.clone()).reroot(&adj);
        let ans = ans.iter().copied().map(|x| x.cost_dist_sum).min().unwrap();
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

    fn check(p: &Problem) {
        assert_eq!(p.solve(), p.solve_naive());
    }

    fn make_random_problem() -> Problem {
        todo!()
        // let mut rng = SmallRng::from_entropy();
        // let n = rng.gen_range(1..=10);
        // let p = Problem { _a: n };
        // println!("{:?}", &p);
        // p
    }

    #[test]
    fn test_with_naive() {
        // 手動でテストを作るのもOK
        for _ in 0..100 {
            // let p = make_random_problem();
            // check(&p);
        }
    }
}

use std::convert::Infallible;

use ac_library::{Max, Monoid};
// ====== import ======
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};

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
            self.raw.push_front(value)
        }
        pub fn pop(&mut self) -> Option<T> {
            self.raw.pop_back()
        }
        pub fn peek(&self) -> Option<&T> {
            self.raw.back()
        }
        pub fn is_empty(&self) -> bool {
            self.raw.is_empty()
        }
    }
    impl<T> Default for Queue<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}

use cum_monoid::*;
pub mod cum_monoid {
    use ac_library::Monoid;
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

        pub fn all_prod(&self) -> M::S {
            self.prefix_prod[self.prefix_prod.len() - 1].clone()
        }
        /// [0, i), [i + 1, n) の区間で総積を取る
        pub fn prod_without1(&self, i: usize) -> M::S {
            M::binary_operation(&self.prefix_prod[i], &self.suffix_prod[i + 1])
        }
        pub fn prod_without_range(&self, l: usize, r: usize) -> M::S {
            M::binary_operation(&self.prefix_prod[l], &self.suffix_prod[r])
        }
    }
}
