#![allow(clippy::let_unit_value)]
use proconio::{input, marker::Usize1};

//------snippet------

//-------------------

struct Edge {
    v1: usize,
    v2: usize,
}
fn read() -> (usize, Vec<Edge>) {
    input! {
        n_v:usize,
        edge_info: [(Usize1, Usize1);n_v-1],
    }

    let edges = edge_info
        .iter()
        .map(|(a, b)| Edge { v1: *a, v2: *b })
        .collect::<Vec<_>>();

    (n_v, edges)
}

fn solve(n_v: usize, edges: &[Edge]) -> bool {
    let mut next_list = vec![Vec::<usize>::new(); n_v];

    for edge in edges {
        next_list[edge.v1].push(edge.v2);
        next_list[edge.v2].push(edge.v1);
    }
    next_list.iter().any(|v| v.len() == n_v - 1)
}

//fn output() {}

fn main() {
    let (n_v, edges) = read();
    let ans = solve(n_v, &edges);
    let ans_str = if ans { "Yes" } else { "No" };
    //output();
    println!("{}", ans_str);
}
