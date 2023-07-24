#![allow(clippy::let_unit_value)]
use itertools::Itertools;
use proconio::input;

//------snippet------

//-------------------

fn read() -> (i64, i64, i64) {
    input! {
        //from OnceSource::from(""),
        k:i64,a:i64, b:i64
    }
    (a, b, k)
}

#[allow(dead_code)]
fn to_k(x: i64, k: i64) -> i64 {
    // 100_000

    let mut pow10 = 1;
    let mut powk = 1;
    let mut ans = 0;
    for _ in 0..=6 {
        let digit = (x / pow10) % 10;
        // x %10
        // x /10 %10
        // x/100 %10
        ans += powk * digit;
        pow10 *= 10;
        powk *= k;
    }
    ans
}

fn to_k2(x: i64, k: i64) -> i64 {
    // 100_000
    // 10進数をk進数に変換
    let str = x.to_string().bytes().collect_vec();

    let mut ans: i64 = 0;
    //let mut pow10: i64 = 1;
    let mut powk: i64 = 1;
    for i in (0..str.len()).rev() {
        let digit = str[i] - b'0';
        ans += (digit as i64) * powk;
        //pow10 *= 10;
        powk *= k;
    }
    //dbg!(ans)
    ans
}

fn solve(a: i64, b: i64, k: i64) -> i64 {
    to_k2(a, k) * to_k2(b, k)
}

//fn output() {}

fn main() {
    let (a, b, k) = read();
    let ans = solve(a, b, k);
    //output();
    println!("{}", ans);
}
