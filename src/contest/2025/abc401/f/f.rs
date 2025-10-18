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
            .map(|i| *i as i64)
            .collect_vec();
        let max_dist2 = DistMaxReroot {}
            .reroot(&adj2)
            .iter()
            .map(|i| *i as i64)
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
                bound as i64 * max_diam
                    + (nv2 - bound) as i64 * (max_dist1[i] + 1)
                    + max_dist2_cumsum.range_sum(bound..nv2)
            })
            .sum::<i64>();
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // 頻度分布の畳み込みを使う
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
            .map(|i| *i as i64)
            .collect_vec();
        let max_dist2 = DistMaxReroot {}
            .reroot(&adj2)
            .iter()
            .map(|i| *i as i64)
            .sorted()
            .collect_vec();

        let diam1 = max_dist1.iter().copied().max().unwrap();
        let diam2 = max_dist2.iter().copied().max().unwrap();
        let max_diam = i64::max(diam1, diam2);

        let max_dist1_cnts = max_dist1
            .iter()
            .copied()
            .fold(vec![0; nv1 + 1], |mut acc, d| {
                acc[d as usize] += 1;
                acc
            });

        let max_dist2_cnts = max_dist2
            .iter()
            .copied()
            .fold(vec![0; nv2 + 1], |mut acc, d| {
                acc[d as usize] += 1;
                acc
            });

        let dist_sum_cnts = ac_library::convolution_i64(&max_dist1_cnts, &max_dist2_cnts);

        let term1 = (0..max_diam - 1)
            .map(|d| dist_sum_cnts[d as usize])
            .sum::<i64>()
            * max_diam;

        let term2 = ((max_diam - 1) as usize..dist_sum_cnts.len())
            .map(|d| dist_sum_cnts[d] * (d + 1) as i64)
            .sum::<i64>();

        let ans = term1 + term2;
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

struct DistMaxReroot();
impl Reroot for DistMaxReroot {
    type M = Max<u64>;

    fn add_vertex(&self, x: &<Self::M as Monoid>::S, _v: usize) -> <Self::M as Monoid>::S {
        *x
    }

    fn add_edge(
        &self,
        x: &<Self::M as Monoid>::S,
        _v: usize,
        _ei: usize,
    ) -> <Self::M as Monoid>::S {
        x + 1
    }
}

use reroot::*;
#[allow(clippy::module_inception)]
pub mod reroot {
    use ac_library::Max;
    struct DistMaxReroot();
    impl Reroot for DistMaxReroot {
        type M = Max<u64>;
        fn add_vertex(&self, x: &<Self::M as Monoid>::S, _v: usize) -> <Self::M as Monoid>::S {
            *x
        }
        fn add_edge(
            &self,
            x: &<Self::M as Monoid>::S,
            _v: usize,
            _ei: usize,
        ) -> <Self::M as Monoid>::S {
            x + 1
        }
    }
    /// 全方位木DP
    pub trait Reroot {
        type M: Monoid;
        fn add_vertex(&self, x: &<Self::M as Monoid>::S, v: usize) -> <Self::M as Monoid>::S;
        fn add_edge(
            &self,
            x: &<Self::M as Monoid>::S,
            v: usize,
            ei: usize,
        ) -> <Self::M as Monoid>::S;
        fn prod(xs: &[<Self::M as Monoid>::S]) -> <Self::M as Monoid>::S {
            xs.iter().fold(Self::M::identity(), |acc, x| {
                Self::M::binary_operation(&acc, x)
            })
        }
        fn reroot(&self, adj: &[Vec<usize>]) -> Vec<<Self::M as Monoid>::S> {
            let nv = adj.len();
            let mut dp: Vec<Vec<<Self::M as Monoid>::S>> = adj
                .iter()
                .map(|next_list| {
                    let degree = next_list.len();
                    vec![Self::M::identity(); degree]
                })
                .collect_vec();
            {
                let dfs_post_order = dfs_post_order(adj, 0);
                let mut visited = vec![false; nv];
                for &current_v in &dfs_post_order {
                    visited[current_v] = true;
                    for (current_e, next_v) in adj[current_v].iter().copied().enumerate() {
                        if !visited[next_v] {
                            continue;
                        }
                        dp[current_v][current_e] = {
                            let edge_dp_next = dp[next_v]
                                .iter()
                                .enumerate()
                                .filter(|(next_e, _)| adj[next_v][*next_e] != current_v)
                                .map(|(next_e, x)| self.add_edge(x, next_v, next_e))
                                .collect_vec();
                            let prod = Self::prod(&edge_dp_next);
                            self.add_vertex(&prod, next_v)
                        };
                    }
                }
            }
            {
                let bfs_order = bfs_order(adj, 0);
                let mut visited = vec![false; nv];
                for &current_v in &bfs_order {
                    visited[current_v] = true;
                    let edge_dp_current = dp[current_v]
                        .iter()
                        .enumerate()
                        .map(|(current_e, x)| self.add_edge(x, current_v, current_e))
                        .collect_vec();
                    let cum_monoid = CumMonoid::<Self::M>::new(&edge_dp_current);
                    for (current_e, next_v) in adj[current_v].iter().copied().enumerate() {
                        if visited[next_v] {
                            continue;
                        }
                        let rev_current_e =
                            adj[next_v].iter().position(|&v| v == current_v).unwrap();
                        dp[next_v][rev_current_e] = {
                            let prod = cum_monoid.prod_without1(current_e);
                            self.add_vertex(&prod, current_v)
                        };
                    }
                }
            }
            dp.iter()
                .enumerate()
                .map(|(current_v, dp_current)| {
                    let edge_dp_current = dp_current
                        .iter()
                        .enumerate()
                        .map(|(current_e, x)| self.add_edge(x, current_v, current_e))
                        .collect_vec();
                    self.add_vertex(&Self::prod(&edge_dp_current), current_v)
                })
                .collect_vec()
        }
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
                    order.push(v);
                }
            }
        }
        order
    }
    use ac_library::Monoid;
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
            /// [0, i), [i + 1, n) の区間で総積を取る
            pub fn prod_without1(&self, i: usize) -> M::S {
                M::binary_operation(&self.prefix_prod[i], &self.suffix_prod[i + 1])
            }
        }
    }
    use itertools::Itertools;
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
