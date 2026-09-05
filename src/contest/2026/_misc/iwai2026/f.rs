// 問題文と制約は読みましたか？
// #[fastout]

// (sx, sy) → (dx, dy) のコスト
fn solve(sx: i64, sy: i64, dx: i64, dy: i64) -> i64 {
    if sy >= 62 && dy >= 62 {
        return (sy - dy).abs();
    }
    if sy >= 62 {
        return solve(sx, 61, dx, dy) + (sy - 61); // 61 まで降りる
    }
    if dy >= 62 {
        return solve(sx, sy, dx, 61) + (dy - 61); // 61 まで降りる
    }

    assert!(sy <= 61);
    assert!(dy <= 61);
    (max(sy, dy)..62)
        .map(|cy| {
            let sx2 = sx >> cy;
            let dx2 = dx >> cy;

            (sx2- dx2).abs() // 横移動
            + cy - sy // 上移動
            + cy - dy // 上移動
        })
        .min()
        .unwrap()
}
fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        input! {
            sx: i64,
            sy: i64,
            dx: i64,
            dy: i64,
        }
        let ans = solve(sx, sy, dx, dy);
        println!("{}", ans);
    }
}

// ====== import ======
use std::cmp::{max, min};
#[allow(unused_imports)]
use {
    itertools::{Itertools, chain, iproduct, izip},
    proconio::{
        derive_readable, fastout, input,
        marker::{Bytes, Chars, Usize1},
    },
    std::{
        cmp::Reverse,
        collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet},
    },
};

// ====== output func ======
#[allow(unused_imports)]
use print_util::*;
pub mod print_util {
    use itertools::Itertools;
    use proconio::fastout;

    #[fastout]
    pub fn print_vec<T: std::fmt::Display>(arr: &[T]) {
        for a in arr {
            println!("{}", a);
        }
    }

    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
        println!("{}", arr.iter().join(" "));
    }

    #[fastout]
    pub fn print_vec2<T: std::fmt::Display, R: AsRef<[T]>>(arr: &[R]) {
        for row in arr {
            println!("{}", row.as_ref().iter().join(" "));
        }
    }

    pub fn print_bytes(bytes: &[u8]) {
        println!("{}", std::str::from_utf8(bytes).unwrap());
    }

    pub fn print_chars(chars: &[char]) {
        println!("{}", chars.iter().collect::<String>());
    }

    #[fastout]
    pub fn print_vec_bytes<R: AsRef<[u8]>>(vec_bytes: &[R]) {
        for row in vec_bytes {
            println!("{}", std::str::from_utf8(row.as_ref()).unwrap());
        }
    }

    #[fastout]
    pub fn print_vec_chars<R: AsRef<[char]>>(vec_chars: &[R]) {
        for row in vec_chars {
            println!("{}", row.as_ref().iter().collect::<String>());
        }
    }

    pub fn print_yesno(ans: bool) {
        println!("{}", if ans { "Yes" } else { "No" });
    }
}

// ====== snippet ======
