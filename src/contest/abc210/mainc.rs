#![allow(clippy::let_unit_value)]
use std::{cmp::max, collections::HashMap};

use proconio::input;

fn read() -> (usize, usize, Vec<i64>) {
    input! {n_candies:usize, k:usize, colors: [i64;n_candies]}
    (n_candies, k, colors)
}

fn solve(n_candies: usize, k: usize, colors: &[i64]) -> i64 {
    // c-> 連続したk個のキャンディのうち色がcのものの数
    let mut hmap: HashMap<i64, i64> = HashMap::new();

    #[allow(clippy::needless_range_loop)]
    for i in 0..k {
        let counter = hmap.entry(colors[i]).or_insert(0);
        *counter += 1;
    }

    let mut ans = hmap.len();

    for i in 0..(n_candies - k) {
        let removed_color = colors[i];
        let added_color = colors[i + k];

        let removed_cnt = hmap.entry(removed_color).or_default();
        *removed_cnt -= 1;
        if *removed_cnt <= 0 {
            hmap.remove(&removed_color);
        }
        let added_cnt = hmap.entry(added_color).or_insert(0);
        *added_cnt += 1;

        ans = max(ans, hmap.len());
    }
    //dbg!(hmap);

    ans as i64
}

//fn output() {}

fn main() {
    let (n_candies, k, colors) = read();
    let ans = solve(n_candies, k, &colors);
    println!("{}", ans);
}
