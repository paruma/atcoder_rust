#![allow(clippy::let_unit_value)]
use std::collections::VecDeque;

use itertools::Itertools;
use num::{One, Zero};
use proconio::{input, marker::Usize1, source::once::OnceSource};

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
    bi_ops_impl ! (Add , add , + ) ;
    bi_ops_impl ! (Sub , sub , - ) ;
    bi_ops_impl ! (Mul , mul , * ) ;
    impl std::ops::Div for RF {
        type Output = Self;
        fn div(self, rhs: Self) -> Self::Output {
            std::ops::Mul::mul(self, rhs.inv())
        }
    }
    macro_rules ! bi_ops_assign_impl {($ std_ops_assign : ident , $ fn_assign : ident , $ op : tt ) => {impl std :: ops ::$ std_ops_assign for RF {fn $ fn_assign (& mut self , rhs : Self ) {* self = * self $ op rhs } } } ; }
    bi_ops_assign_impl ! (AddAssign , add_assign , + ) ;
    bi_ops_assign_impl ! (SubAssign , sub_assign , - ) ;
    bi_ops_assign_impl ! (MulAssign , mul_assign , * ) ;
    bi_ops_assign_impl ! (DivAssign , div_assign , / ) ;
    impl std::ops::Neg for RF {
        type Output = Self;
        fn neg(self) -> Self::Output {
            RF::new(-self.rep)
        }
    }
}
use rf::*;

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
    let dist: Vec<Option<i64>> = {
        let mut dist: Vec<Option<i64>> = vec![None; n_v];
        let mut visited = vec![false; n_v];
        let mut open: VecDeque<usize> = VecDeque::new();

        open.push_front(0);
        dist[0] = Some(0);
        visited[0] = true;

        while !open.is_empty() {
            let current_idx = open.pop_back().unwrap();

            assert!(dist[current_idx].is_some());
            for &next_idx in &next_list[current_idx] {
                if !visited[next_idx] {
                    visited[next_idx] = true;
                    if dist[next_idx].is_none()
                        || dist[next_idx].unwrap() > dist[current_idx].unwrap() + 1
                    {
                        open.push_front(next_idx);
                        dist[next_idx] = Some(dist[current_idx].unwrap() + 1);
                    }
                }
            }
        }
        dist
    };
    // 距離付をもとに場合の数の計算

    let mut visited = vec![false; n_v];
    let mut open: VecDeque<usize> = VecDeque::new();
    let mut dp = vec![RF::zero(); n_v];

    open.push_front(0);
    dp[0] = RF::one();
    visited[0] = true;

    while !open.is_empty() {
        let current_idx = open.pop_back().unwrap();

        for &next_idx in &next_list[current_idx] {
            if dist[next_idx].unwrap() == dist[current_idx].unwrap() + 1 {
                if !visited[next_idx] {
                    open.push_front(next_idx);
                    visited[next_idx] = true;
                }
                //dp[next_idx] += dp[current_idx];
                let dp_current = dp[current_idx];

                dp[next_idx] += dp_current;
                //dbg!((next_idx, dp[next_idx]));
            }
        }
    }

    //dbg!(dp.clone());
    dp[n_v - 1]
}

//fn output() {}

fn main() {
    let (n_v, n_e, edges) = read();
    let ans = solve(n_v, n_e, &edges);
    //output();
    println!("{}", ans.rep());
}
