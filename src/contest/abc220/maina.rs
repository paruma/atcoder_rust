#![allow(clippy::let_unit_value)]
use proconio::input;

//------snippet------

//-------------------

fn main() {
    input! {
        a:i64, b:i64,c:i64
    }

    let ans = (b / c) * c;
    if a <= ans && ans <= b {
        println!("{}", ans);
    } else {
        println!("{}", -1)
    }

    //println!("{}", ans);
}
