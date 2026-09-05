// 問題文と制約は読みましたか？
// #[fastout]

fn l1_dist((a, b): (usize, usize), (c, d): (usize, usize)) -> usize {
    a.abs_diff(c) + b.abs_diff(d)
}
fn main() {
    input! {
        h :usize,
        w :usize,
        a: Usize1, // スタート/ゴール
        b: Usize1,
        r1: Usize1,
        c1: Usize1,
        r2: Usize1,
        c2: Usize1,
        p: Usize1, // ゴミ箱
        q: Usize1,
    }

    let ans = iproduct!(r1..=r2, c1..=c2)
        .map(|(x, y)| {
            // (a,b) → (x,y) → (p,q) → (a,b)
            l1_dist((a, b), (x, y)) + l1_dist((x, y), (p, q)) + l1_dist((p, q), (a, b))
        })
        .min()
        .unwrap();

    println!("{}", ans);
}

// ====== import ======
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
