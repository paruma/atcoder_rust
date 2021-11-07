#![allow(clippy::let_unit_value)]
use proconio::input;

//------snippet------

//-------------------

fn main() {
    input! {
        a: f32
    }
    let ans = (a + 0.5f32) as i32;
    println!("{}", ans);
}
