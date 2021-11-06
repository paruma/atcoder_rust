#![allow(clippy::let_unit_value)]
use itertools::Itertools;
use proconio::{input, marker::Chars};

//------snippet------

//-------------------

fn main() {
    input! {
        s: Chars
    }

    let x = s.into_iter().unique().collect_vec().len();
    let ans = match x {
        1 => 1,
        2 => 3,
        3 => 6,
        _ => -1,
    };
    println!("{}", ans);
}
