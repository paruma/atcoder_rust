#![allow(clippy::let_unit_value)]
use std::collections::{binary_heap, BinaryHeap, HashMap};

use itertools::Itertools;
use proconio::input;

//------snippet------

//-------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct ElemIdx {
    i: usize,
    x: i64,
}

impl PartialOrd for ElemIdx {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        PartialOrd::partial_cmp(&self.i, &other.i).map(|c| c.reverse())
    }
}

impl Ord for ElemIdx {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]

enum Query {
    Process { x: i64 },
    Output { x: i64 },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]

enum QueryIdx {
    Process { ei: ElemIdx },
    Output { ei: ElemIdx },
}

// Query+Indexを作る

fn read() -> Vec<Query> {
    input! {
        qn:usize,
        query_list: [(i64, i64); qn]
        //from OnceSource::from(""),
    }
    let query_list = query_list
        .iter()
        .map(|(t, x)| -> Query {
            if *t == 1 {
                Query::Process { x: *x }
            } else {
                Query::Output { x: *x }
            }
        })
        .collect_vec();

    query_list
}

fn solve(query_list: &[Query]) {
    let n = 1048576_i64;
    let query_list = (0..query_list.len()).map(|i| match query_list[i] {
        Query::Process { x } => QueryIdx::Process {
            ei: ElemIdx { i, x },
        },
        Query::Output { x } => QueryIdx::Output {
            ei: ElemIdx { i, x },
        },
    });

    let mut hist: Vec<Vec<ElemIdx>> = vec![vec![]; n as usize];

    for query in query_list {
        match query {
            QueryIdx::Process { ei } => {
                hist[(ei.x as i64 % n) as usize].push(ei);
            }
            QueryIdx::Output { .. } => {}
        }
    }

    let mut hist2 = hist.iter().filter(|&x| !x.is_empty()).collect_vec();

    let start_idx = (0..hist2.len())
        .find(|&current_i| {
            let prev_i = if current_i == 0 {
                n as usize - 1
            } else {
                current_i - 1
            };
            let current_e = hist2[current_i][0].x % n;
            let prev_e = hist2[prev_i][0].x % n;

            let diff = (current_e - prev_e + n) % n;
            hist[prev_i].len() <= (diff as usize)
        })
        .unwrap();

    let mut decision: Vec<Option<ElemIdx>> = vec![None; n as usize];
    let mut hist3 = hist2
        .iter()
        .map(|&x| {
            let y: BinaryHeap<ElemIdx> = BinaryHeap::from(x.clone());
            y
        })
        .collect_vec();

    let mut open: BinaryHeap<ElemIdx> = BinaryHeap::new();

    // decisionを作る
    let first_elem = hist3[start_idx].pop().unwrap();
    let mut pos = first_elem.i;
    decision[pos] = Some(first_elem);
    pos = (pos + 1) % (n as usize);

    //decision[]

    for i in 0..hist3.len() {
        let current_idx = (start_idx + i) % (n as usize);
        open.append(&mut hist3[current_idx]);
        let next_pos = (start_idx + i + 1) % (n as usize);
    }
}

fn main() {
    let query_list = read();
    solve(&query_list);

    //output();
    //println!("{}", ans);
}
