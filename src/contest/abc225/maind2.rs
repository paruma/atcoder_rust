#![allow(clippy::let_unit_value)]
use std::io::BufRead;

use itertools::Itertools;
use proconio::{input, marker::Chars};

//------snippet------

//-------------------

enum Query {
    Connect { x: usize, y: usize },
    Disconnect { x: usize, y: usize },
    Print { x: usize },
}

fn read() -> (usize, usize, Vec<Query>) {
    input! {
        //from OnceSource::from(""),
        n: usize, q:usize,
        qu: [String; q]
    }

    let qs = qu
        .iter()
        .map(|q| {
            let q: Vec<&str> = q.split(' ').collect();
            let qty = q[0];
            match qty {
                "1" => {
                    let x: usize = q[1].parse().unwrap();
                    let y: usize = q[2].parse().unwrap();
                    Query::Connect { x, y }
                }
                "2" => {
                    let x: usize = q[1].parse().unwrap();
                    let y: usize = q[2].parse().unwrap();
                    Query::Disconnect { x, y }
                }
                _ => {
                    let x: usize = q[1].parse().unwrap();
                    Query::Print { x }
                }
            }
        })
        .collect_vec();

    (n, q, qs)
}

fn solve(n: usize, _q: usize, qs: &[Query]) {
    println!("test");
}

fn main() {
    let (n, q, qs) = read();
    solve(n, q, &qs);
    //output();

    //println!("{}", ans);
}
