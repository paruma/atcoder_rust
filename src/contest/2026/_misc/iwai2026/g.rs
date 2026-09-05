// 問題文と制約は読みましたか？
// #[fastout]
// #[fastout]

fn solve(xs: &[char], s: bool, k: usize) -> Vec<char> {
    let n = xs.len();
    if s {
        // Warong
        let mut ans = xs.to_vec();
        for i in 0..k {
            ans[i] = 'A';
        }

        let count_w = xs[k..].iter().copied().filter(|ch| *ch == 'W').count();
        let count_hatena = xs[k..].iter().copied().filter(|ch| *ch == '?').count();
        if count_w == 0 && count_hatena == 1 {
            for i in k..n {
                ans[i] = 'A';
            }
        }
        ans
    } else {
        // NotWarong
        let count_w1 = xs[..k].iter().copied().filter(|ch| *ch == 'W').count();
        let count_hatena1 = xs[..k].iter().copied().filter(|ch| *ch == '?').count();

        let count_w2 = xs[k..].iter().copied().filter(|ch| *ch == 'W').count();
        // let count_hatena2 = xs[k..].iter().copied().filter(|ch| *ch == '?').count();
        let mut ans = xs.to_vec();

        // 前半Aのみ
        if count_w1 == 0 && count_hatena1 == 0 {
            for i in k..n {
                ans[i] = 'A';
            }
        }

        // 前半にWを含まない?を1つ含む
        // 後半はWを含む
        if count_w1 == 0 && count_hatena1 == 1 && count_w2 > 0 {
            for i in 0..k {
                if ans[i] == '?' {
                    ans[i] = 'W';
                }
            }
        }

        ans
    }
}

fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        input! {
            r: Chars,
            s: String,
            k: usize
        }

        let s = s == "Warong";

        let ans = solve(&r, s, k);
        println!("{}", ans.iter().collect::<String>());
    }
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
