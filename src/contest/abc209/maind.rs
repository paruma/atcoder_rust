#![allow(clippy::let_unit_value)]
use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Query {
    c: usize,
    d: usize,
}
struct Edge {
    v1: usize,
    v2: usize,
}
fn read() -> (usize, usize, Vec<Edge>, Vec<Query>) {
    input! {
        n_v:usize,n_q:usize,
        edge_info: [(Usize1, Usize1);n_v-1],
        query_info: [(Usize1, Usize1);n_q],
    }

    let edges = edge_info
        .iter()
        .map(|(a, b)| Edge { v1: *a, v2: *b })
        .collect::<Vec<_>>();
    let queries = query_info
        .iter()
        .map(|(a, b)| Query { c: *a, d: *b })
        .collect::<Vec<_>>();

    (n_v, n_q, edges, queries)
}

enum Pos {
    Road,
    Town,
}

fn solve(n_v: usize, _n_q: usize, edges: &[Edge], queries: &[Query]) -> Vec<Pos> {
    let mut next_list = vec![Vec::<usize>::new(); n_v];

    for edge in edges {
        next_list[edge.v1].push(edge.v2);
        next_list[edge.v2].push(edge.v1);
    }

    let mut labels = vec![-1; n_v];
    let mut visited = vec![false; n_v];
    let mut open: VecDeque<usize> = VecDeque::new();

    open.push_front(0);
    visited[0] = true;
    labels[0] = 0;

    while !open.is_empty() {
        let current_idx = open.pop_back().unwrap();

        for &next_idx in &next_list[current_idx] {
            if !visited[next_idx] {
                visited[next_idx] = true;
                open.push_front(next_idx);
                labels[next_idx] = (labels[current_idx] + 1) % 2;
            }
        }
    }

    //dbg!(labels.clone());

    queries
        .iter()
        .map(|&q| {
            if labels[q.c] == labels[q.d] {
                Pos::Town
            } else {
                Pos::Road
            }
        })
        .collect_vec()
}

fn output(ans: &[Pos]) {
    for pos in ans {
        let msg = match pos {
            Pos::Road => "Road",
            Pos::Town => "Town",
        };
        println!("{}", msg);
    }
}

fn main() {
    let (n_v, n_q, edges, queries) = read();
    let ans = solve(n_v, n_q, &edges, &queries);
    output(&ans);
    //println!("{}", ans);
}
