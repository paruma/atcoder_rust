#![allow(clippy::let_unit_value)]
use proconio::input;

//------snippet------

//-------------------

fn read() -> i64 {
    input! {
        //from OnceSource::from(""),
        x:i64
    }
    x
}

fn solve(x: i64) {
    if 0 <= x && x < 40 {
        println!("{}", 40 - x);
    } else if 40 <= x && x < 70 {
        println!("{}", 70 - x);
    } else if 70 <= x && x < 90 {
        println!("{}", 90 - x);
    } else {
        println!("expert");
    }
}

//fn output() {}

fn main() {
    let x = read();
    solve(x);
    //output();
    //println!("{}", ans);
}
