use ac_library::{ModInt as Mint, Monoid};
pub struct MintProd(Infallible);
impl Monoid for MintProd {
    type S = Mint;
    fn identity() -> Self::S {
        Mint::new(1)
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        a * b
    }
}
struct DpvReroot();
impl Reroot for DpvReroot {
    type M = MintProd;

    fn add_vertex(&self, x: &<Self::M as Monoid>::S, _v: usize) -> <Self::M as Monoid>::S {
        *x
    }

    fn add_edge(
        &self,
        x: &<Self::M as Monoid>::S,
        _v: usize,
        _ei: usize,
    ) -> <Self::M as Monoid>::S {
        *x + 1
    }
}
fn main() {
    input! {
        nv: usize,
        m: u32,
        es: [(Usize1, Usize1); nv - 1],
    }
    use ac_library::ModInt as Mint;
    Mint::set_modulus(m);
    let adj = es
        .iter()
        .copied()
        .fold(vec![vec![]; nv], |mut acc, (x, y)| {
            acc[x].push(y);
            acc[y].push(x);
            acc
        });
    let ans: Vec<Mint> = DpvReroot {}.reroot(&adj);
    let ans = ans.iter().copied().map(|x| x.val()).collect_vec();
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
use std::convert::Infallible;

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

use reroot::*;
pub mod reroot {

    /// 全方位木DP
    pub trait Reroot {
        type M: Monoid; // 可換モノイド

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
                            let edge_dp_next = dp[next]
                                .iter()
                                .enumerate()
                                .filter(|(ei, _)| adj[next][*ei] != current)
                                .map(|(ei, x)| self.add_edge(x, next, ei))
                                .collect_vec();
                            let prod = Self::prod(&edge_dp_next);
                            self.add_vertex(&prod, next)
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
                    let edge_dp_current = dp[current]
                        .iter()
                        .enumerate()
                        .map(|(ei, x)| self.add_edge(x, current, ei))
                        .collect_vec();
                    let cum_monoid = CumMonoid::<Self::M>::new(&edge_dp_current);
                    for (edge_i, next) in adj[current].iter().copied().enumerate() {
                        if visited[next] {
                            continue;
                        }
                        // 償却 O(1) で計算可能
                        let rev_edge_i = adj[next].iter().position(|&v| v == current).unwrap();

                        dp[next][rev_edge_i] = {
                            let prod = cum_monoid.prod_without1(edge_i);
                            self.add_vertex(&prod, current)
                        };
                    }
                }
            }
            dp.iter()
                .enumerate()
                .map(|(v, dp_v)| {
                    let edge_dp_v = dp_v
                        .iter()
                        .enumerate()
                        .map(|(ei, x)| self.add_edge(x, v, ei))
                        .collect_vec();
                    self.add_vertex(&Self::prod(&edge_dp_v), v)
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
                    // 帰りがけ
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
