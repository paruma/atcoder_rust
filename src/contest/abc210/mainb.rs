#![allow(clippy::let_unit_value)]
use proconio::{input, marker::Chars};

fn read() -> (i64, Vec<char>) {
    input! {n: i64, s: Chars}
    (n, s)
}

fn solve(_n: i64, s: &[char]) -> String {
    // 0: 良いカード
    // 1: 悪いカード

    //0: Takahashi
    //1: Aoki
    let mut tern = 0;

    for &c in s {
        if c == '1' {
            break;
        }
        tern = (tern + 1) % 2;
    }

    if tern == 0 {
        String::from("Takahashi")
    } else {
        String::from("Aoki")
    }
}

//fn output() {}

fn main() {
    let (n, s) = read();
    let ans = solve(n, &s);
    println!("{}", ans);
}
