#![allow(clippy::let_unit_value)]
use proconio::input;

//------snippet------

//-------------------

fn main() {
    input! {
        s:i64,
        t:i64,
        x:i64
        //
    }

    let ans = if s < t {
        s <= x && x < t
    } else {
        (s <= x && x < 24) || (0 <= x && x < t)
    };

    let ans_str = if ans { "Yes" } else { "No" };
    println!("{}", ans_str);
}
