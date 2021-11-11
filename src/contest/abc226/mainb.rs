#![allow(clippy::let_unit_value)]
use itertools::Itertools;
use whiteread::Reader;

//------snippet------

//-------------------

fn read() -> Vec<Vec<i64>> {
    let mut rdr = Reader::from_stdin_naive();
    let n = rdr.p::<usize>();

    (0..n)
        .map(|_| {
            let len = rdr.p::<usize>();
            (0..len).map(|_| rdr.p::<i64>()).collect_vec()
        })
        .collect_vec()
}

fn solve(a: &[Vec<i64>]) -> usize {
    a.iter().unique().count()
}

//fn output() {}

fn main() {
    let a = read();
    let ans = solve(&a);
    //output();
    println!("{}", ans);
}
