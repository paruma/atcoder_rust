#![allow(clippy::let_unit_value)]
use std::collections::VecDeque;

use itertools::Itertools;
use num::{One, Zero};
use proconio::{input, marker::Usize1};

pub mod rf {
    pub const MOD: i64 = 1_000_000_007;
    #[allow()]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct RF {
        rep: i64,
    }
    impl RF {
        pub fn new(x: i64) -> RF {
            RF {
                rep: x.rem_euclid(MOD),
            }
        }
        pub fn rep(self) -> i64 {
            self.rep
        }
    }
    impl RF {
        pub fn inv(self) -> Self {
            num::pow(self, (MOD - 2) as usize)
        }
    }
    impl num_traits::Zero for RF {
        fn zero() -> Self {
            RF::new(0)
        }
        fn is_zero(&self) -> bool {
            self.rep == 0
        }
    }
    impl num_traits::One for RF {
        fn one() -> Self {
            RF::new(1)
        }
    }
    macro_rules ! bi_ops_impl {($ std_ops : ident , $ fn : ident , $ op : tt ) => {impl std :: ops ::$ std_ops for RF {type Output = Self ; fn $ fn (self , rhs : Self ) -> Self :: Output {RF :: new (self . rep $ op rhs . rep ) } } } ; }
    bi_ops_impl ! (Add , add , + );
    bi_ops_impl ! (Sub , sub , - );
    bi_ops_impl ! (Mul , mul , * );
    impl std::ops::Div for RF {
        type Output = Self;
        fn div(self, rhs: Self) -> Self::Output {
            std::ops::Mul::mul(self, rhs.inv())
        }
    }
    macro_rules ! bi_ops_assign_impl {($ std_ops_assign : ident , $ fn_assign : ident , $ op : tt ) => {impl std :: ops ::$ std_ops_assign for RF {fn $ fn_assign (& mut self , rhs : Self ) {* self = * self $ op rhs } } } ; }
    bi_ops_assign_impl ! (AddAssign , add_assign , + );
    bi_ops_assign_impl ! (SubAssign , sub_assign , - );
    bi_ops_assign_impl ! (MulAssign , mul_assign , * );
    bi_ops_assign_impl ! (DivAssign , div_assign , / );
    impl std::ops::Neg for RF {
        type Output = Self;
        fn neg(self) -> Self::Output {
            RF::new(-self.rep)
        }
    }
}
use rf::*;

use tropical::Trop::{self, *};
pub mod tropical {
    use std::{cmp::Ordering, ops::Add};
    use Trop::*;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Trop {
        Inf,
        Fin(i64),
    }
    impl Trop {
        pub fn get_fin(self) -> i64 {
            match self {
                Fin(val) => val,
                Inf => panic!("called `Trop::get_fin()` on a `Fin` value"),
            }
        }
        pub fn get_fin_or(self, default: i64) -> i64 {
            match self {
                Fin(val) => val,
                Inf => default,
            }
        }
        pub fn is_fin(self) -> bool {
            matches!(self, Fin(_))
        }
        pub fn is_inf(self) -> bool {
            matches!(self, Inf)
        }
    }
    impl Add for Trop {
        type Output = Trop;
        fn add(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (Inf, Inf) => Inf,
                (Inf, Fin(_)) => Inf,
                (Fin(_), Inf) => Inf,
                (Fin(a), Fin(b)) => Fin(a + b),
            }
        }
    }
    impl PartialOrd for Trop {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match (self, other) {
                (Inf, Inf) => Some(Ordering::Equal),
                (Inf, Fin(_)) => Some(Ordering::Greater),
                (Fin(_), Inf) => Some(Ordering::Less),
                (Fin(a), Fin(b)) => PartialOrd::partial_cmp(a, b),
            }
        }
    }
    impl Ord for Trop {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    a: usize,
    b: usize,
}
// 23:26
fn read() -> (usize, usize, Vec<Edge>) {
    input! {

        n_v: usize,n_e:usize,edge_info:[(Usize1,Usize1); n_e ]
    }
    let edges = edge_info
        .iter()
        .map(|(a, b)| Edge { a: *a, b: *b })
        .collect_vec();
    (n_v, n_e, edges)
}

fn solve(n_v: usize, _n_e: usize, edges: &[Edge]) -> RF {
    let mut next_list = vec![Vec::<usize>::new(); n_v];

    for edge in edges {
        next_list[edge.a].push(edge.b);
        next_list[edge.b].push(edge.a);
    }

    // BFSで距離付
    let dist: Vec<Trop> = {
        let mut dist: Vec<Trop> = vec![Inf; n_v];
        let mut visited = vec![false; n_v];
        let mut open: VecDeque<usize> = VecDeque::new();

        open.push_front(0);
        dist[0] = Fin(0);
        visited[0] = true;

        // while letで書き換えられる
        while !open.is_empty() {
            let current_idx = open.pop_back().unwrap();

            assert!(dist[current_idx].is_fin());
            for &next_idx in &next_list[current_idx] {
                if !visited[next_idx] {
                    visited[next_idx] = true;

                    if dist[next_idx] > dist[current_idx] + Fin(1) {
                        open.push_front(next_idx);
                        dist[next_idx] = dist[current_idx] + Fin(1);
                    }
                }
            }
        }
        dist
    };
    // 距離付をもとに場合の数の計算

    let mut visited = vec![false; n_v];
    let mut open: VecDeque<usize> = VecDeque::new();
    let mut dp = vec![RF::zero(); n_v]; // 配る

    open.push_front(0);
    dp[0] = RF::one();
    visited[0] = true;

    while !open.is_empty() {
        let current_idx = open.pop_back().unwrap();

        for &next_idx in &next_list[current_idx] {
            // いつものBFSみたいにここでvisited判定しない。visitedしてても、足し算をする必要がある。
            if dist[next_idx] == dist[current_idx] + Fin(1) {
                if !visited[next_idx] {
                    open.push_front(next_idx);
                    visited[next_idx] = true;
                }
                //dp[next_idx] += dp[current_idx];
                let dp_current = dp[current_idx];
                dp[next_idx] += dp_current;
            }
        }
    }
    dp[n_v - 1]
}

//fn output() {}

fn main() {
    let (n_v, n_e, edges) = read();
    let ans = solve(n_v, n_e, &edges);
    //output();
    println!("{}", ans.rep());
}
