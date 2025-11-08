use monoid_template::*;
#[allow(unused_variables)]
pub mod monoid_template {
    use std::convert::Infallible;

    use ac_library::Monoid;

    pub struct MyMax(Infallible);
    impl Monoid for MyMax {
        type S = (u64, i64);
        fn identity() -> Self::S {
            (0, i64::MIN)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            (*a).max(*b)
        }
    }
}
use ac_library::{Max, Monoid};
struct DistMaxReroot();
impl Reroot for DistMaxReroot {
    type M = MyMax;
    fn add_vertex(&self, x: &<Self::M as Monoid>::S, v: usize) -> <Self::M as Monoid>::S {
        let idx = if x.1 < 0 { v as i64 } else { x.1 };
        (x.0, idx)
    }
    fn add_edge(&self, x: &<Self::M as Monoid>::S, v: usize, _ei: usize) -> <Self::M as Monoid>::S {
        // let idx = if x.1 < 0 { v as i64 } else { x.1 };
        (x.0 + 1, x.1)
    }
}
fn main() {
    input! {
        nv: usize,
        es: [(Usize1, Usize1); nv-1],
    }

    let adj = es
        .iter()
        .copied()
        .fold(vec![vec![]; nv], |mut acc, (u, v)| {
            acc[u].push(v);
            acc[v].push(u);
            acc
        });
    let dp = DistMaxReroot {}.reroot(&adj);
    let ans: Vec<i64> = dp.iter().copied().map(|(_, i)| (i + 1)).collect_vec();
    print_vec(&ans);
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

    /// 間違っていたら false を返す
    fn process_one_test(rng: &mut SmallRng) -> bool {
        // ==== 問題を作る ====
        let n = rng.random_range(1..=10);
        let xs = (0..n).map(|_| rng.random_range(0..10)).collect_vec();

        // ==== 解く ====
        let main_ans = xs.len();
        let naive_ans = 1;

        // ==== 間違っていたら報告をする ====
        if main_ans != naive_ans {
            // 問題を出力
            println!("{:?}", (n, xs));
            println!("main ans : {:?}", main_ans);
            println!("naive ans: {:?}", naive_ans);
            return false;
        }
        true
    }

    #[allow(unreachable_code)]
    #[test]
    #[ignore]
    fn test_with_naive() {
        let num_tests = 100;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut cnt_wrong = 0;
        let mut rng = SmallRng::seed_from_u64(42);
        // let mut rng = SmallRng::from_entropy();
        for _ in 0..num_tests {
            let is_ok = process_one_test(&mut rng);
            if !is_ok {
                cnt_wrong += 1;
            }
            if cnt_wrong >= max_wrong_case {
                break;
            }
        }
        if cnt_wrong > 0 {
            println!("{} cases are wrong.", cnt_wrong);
            panic!();
        }
    }
}

// ====== import ======
#[allow(unused_imports)]
use {
    itertools::{chain, iproduct, izip, Itertools},
    proconio::{
        derive_readable, fastout, input,
        marker::{Bytes, Chars, Usize1},
    },
    rand::{rngs::SmallRng, seq::SliceRandom, Rng, SeedableRng},
    std::{
        cmp::Reverse,
        collections::{BinaryHeap, HashMap, HashSet},
    },
};

// ====== output func ======
#[allow(unused_imports)]
use print_util::*;
pub mod print_util {
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
    #[fastout]
    pub fn print_vec_chars(vec_chars: &[Vec<char>]) {
        for row in vec_chars {
            let msg = row.iter().collect::<String>();
            println!("{}", msg);
        }
    }
    pub fn print_yesno(ans: bool) {
        let msg = if ans { "Yes" } else { "No" };
        println!("{}", msg);
    }
}

// ====== snippet ======
use reroot::*;
#[allow(clippy::module_inception)]
pub mod reroot {

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
