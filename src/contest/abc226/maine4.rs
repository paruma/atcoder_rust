#![allow(clippy::let_unit_value)]

use proconio::{input, marker::Usize1};

//------snippet------

use num::{pow, Zero};
use rf::*;
pub mod rf {
    pub const MOD: i64 = 998244353;
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

//-------------------

struct Edge {
    v1: usize,
    v2: usize,
}

fn read() -> (usize, Vec<Edge>) {
    input! {
        n_v:usize,
        n_e:usize,
        edge_info: [(Usize1, Usize1); n_e],
    }

    // ここらへんテンプレ化できると嬉しいなぁ。
    let edges = edge_info
        .iter()
        .map(|(a, b)| Edge { v1: *a, v2: *b })
        .collect::<Vec<_>>();

    (n_v, edges)
}

// loop_cntが2重にカウントされてしまうのでだめ。
#[allow(dead_code)]
fn dfs_loop_cnt_wrong(
    before_idx: usize,
    current_idx: usize,
    next_list: &[Vec<usize>],
    visited: &mut [bool],
) -> usize {
    dbg!(current_idx);
    let mut loop_cnt = 0;

    for &next_idx in &next_list[current_idx] {
        // before → current → beforeみたいな訪問を防ぎたい
        if next_idx != before_idx {
            if visited[next_idx] {
                loop_cnt += 1;
            } else {
                visited[next_idx] = true;
                loop_cnt += dfs_loop_cnt_wrong(current_idx, next_idx, next_list, visited);
            }
        }
    }
    loop_cnt
}

fn dfs_loop_cnt(
    before_idx: usize,
    current_idx: usize,
    next_list: &[Vec<usize>],
    visited: &mut [bool],
) -> usize {
    dbg!(current_idx);
    let mut loop_cnt = if visited[current_idx] { 1 } else { 0 };
    visited[current_idx] = true;

    for &next_idx in &next_list[current_idx] {
        // before → current → beforeみたいな訪問を防ぎたい
        if next_idx != before_idx && !visited[next_idx] {
            loop_cnt += dfs_loop_cnt(current_idx, next_idx, next_list, visited);
        }
    }
    loop_cnt
}

// DFS解(再帰あり)
#[allow(clippy::needless_range_loop)]
fn solve(n_v: usize, edges: &[Edge]) -> RF {
    let mut next_list = vec![Vec::<usize>::new(); n_v];

    // Union Findの場合、ここが
    // uf.unite(edge.v1, edge.v2)
    // になっていた。
    for edge in edges {
        next_list[edge.v1].push(edge.v2);
        next_list[edge.v2].push(edge.v1);
    }

    let mut visited = vec![false; n_v];

    let mut connected_cnt = 0; // 閉路が1つの連結成分の数

    for init_idx in 0..n_v {
        if !visited[init_idx] {
            let loop_cnt = dfs_loop_cnt(init_idx, init_idx, &next_list, &mut visited);

            dbg!(loop_cnt);
            if loop_cnt == 1 {
                connected_cnt += 1;
            } else {
                return RF::zero();
            }
        }
    }

    pow(RF::new(2), connected_cnt)
}

//fn output() {}

fn main() {
    let (n_v, edges) = read();
    let ans = solve(n_v, &edges);

    println!("{}", ans.rep());
}
