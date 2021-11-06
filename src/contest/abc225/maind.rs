#![allow(clippy::let_unit_value)]
#![allow(clippy::many_single_char_names)]

use itertools::Itertools;
//use proconio::{input, marker::Chars};
use whiteread::Reader;

//------snippet------

//-------------------

#[allow(dead_code)]
enum Query {
    Connect { x: usize, y: usize },
    Disconnect { x: usize, y: usize },
    Print { x: usize },
}

fn read() -> (usize, usize, Vec<Query>) {
    let mut rdr = Reader::from_stdin_naive();
    let (n, q) = rdr.p::<(usize, usize)>();

    let qs = (0..q)
        .map(|_| {
            let t = rdr.p::<usize>();

            match t {
                1 => {
                    let x: usize = rdr.p::<usize>() - 1;
                    let y: usize = rdr.p::<usize>() - 1;
                    Query::Connect { x, y }
                }
                2 => {
                    let x: usize = rdr.p::<usize>() - 1;
                    let y: usize = rdr.p::<usize>() - 1;
                    Query::Disconnect { x, y }
                }
                _ => {
                    let x: usize = rdr.p::<usize>() - 1;
                    Query::Print { x }
                }
            }
        })
        .collect_vec();

    (n, q, qs)
}

fn solve(_n: usize, _q: usize, qs: &[Query]) {
    for _q in qs {
        /*
        match q {
            Query::Connect { x, y } => {}
            Query::Disconnect { x, y } => {}
            Query::Print { x } => {}
        }
        */
    }
    //println!("test");
}

fn main() {
    let (n, q, qs) = read();
    solve(n, q, &qs);
    //output();

    //println!("{}", ans);
}
