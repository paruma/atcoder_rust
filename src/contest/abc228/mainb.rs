#![allow(clippy::let_unit_value)]
use std::collections::VecDeque;

use itertools::Itertools;

use whiteread::Reader;

//------snippet------

//-------------------

fn read() -> (usize, Vec<usize>) {
    let mut rdr = Reader::from_stdin_naive();
    let n = rdr.p::<usize>();
    let x = rdr.p::<usize>() - 1;
    let a = (0..n).map(|_| rdr.p::<usize>() - 1).collect_vec();
    /*
    input! {
        n:usize,
        x:Usize1,
        a: [Usize1; n],
    }
    */
    (x, a)
}

fn solve(x: usize, a: &[usize]) -> usize {
    let mut visited = vec![false; a.len()];
    visited[x] = true;

    let mut open: VecDeque<usize> = VecDeque::new();

    open.push_front(x);

    while !open.is_empty() {
        let current_idx = open.pop_back().unwrap();
        let next_idx = a[current_idx];
        if !visited[next_idx] {
            visited[next_idx] = true;
            open.push_front(next_idx);
        }
    }
    visited.iter().filter(|&&cond| cond).count()
}

fn main() {
    let (x, a) = read();
    let ans = solve(x, &a);
    //output();
    println!("{}", ans);
}
