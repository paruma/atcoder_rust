#![allow(clippy::let_unit_value)]
use proconio::input;

//------snippet------

//-------------------

fn read() -> (usize, usize, Vec<(i64, i64)>) {
    input! {
        //from OnceSource::from(""),
        n: usize,
        m:usize,
        abs: [(i64,i64); m],
    }

    (n, m, abs)
}

#[allow(unused_variables)]
fn solve(n: usize, m: usize, abs: &[(i64, i64)]) -> i64 {
    // 左向きの辺をすべて右向きにする

    /*
    let mut abs = abs.iter().map(|(a, b)| {

        if *b <= *a{

        }

    }).collect_vec();
    //abs.sort_by_key(|(a, b)| *a);
    abs.sort_by_key(|(a, b)| *b - *a);
    let mut is_used = vec![false; n];
    let mut cnt = 0;
    let mut pos = -1;

    for (a, b) in abs {
        if pos < a {
            pos = b;
            cnt += 1;
        }
    }

    cnt
    */
    0
}

//fn output() {}

fn main() {
    let (n, m, abs) = read();
    let ans = solve(n, m, &abs);
    //output();
    println!("{}", ans);
}
