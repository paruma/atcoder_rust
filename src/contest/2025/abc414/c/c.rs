fn main() {
    input! {
        a: i64,
        n: i64,
    }
    // let ans: i64 = (1..=6).map(|i|{
    //     let
    // });
    let ans = (1..1_000_000)
        .flat_map(|x| {
            let x10 = to_base_n_value(x, 10);
            let abba10 = chain!(x10.iter(), x10.iter().rev()).copied().collect_vec();
            let aba10 = chain!(x10.iter(), x10.iter().rev().skip(1))
                .copied()
                .collect_vec();
            let abba = eval_base_n_value(&abba10, 10);
            let aba = eval_base_n_value(&aba10, 10);
            [abba, aba]
        })
        .filter(|&x| {
            if x > n {
                return false;
            }
            let x_a = to_base_n_value(x, a);
            // x_a.iter().eq(x_a.iter().rev()) // こう書けばよかった
            izip!(x_a.iter().copied(), x_a.iter().rev().copied()).all(|(x, y)| x == y)
        })
        .sum::<i64>();
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }
}

// ====== import ======
#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, izip};
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet};

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
        let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Display>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{}", x)).join(" ");
            println!("{}", msg);
        }
    }
    pub fn print_bytes(bytes: &[u8]) {
        let msg = String::from_utf8(bytes.to_vec()).unwrap();
        println!("{}", msg);
    }
    pub fn print_chars(chars: &[char]) {
        let msg = chars.iter().collect::<String>();
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec_bytes(vec_bytes: &[Vec<u8>]) {
        for row in vec_bytes {
            let msg = String::from_utf8(row.to_vec()).unwrap();
            println!("{}", msg);
        }
    }
    #[fastout]
    pub fn print_vec_chars(vec_chars: &[Vec<char>]) {
        for row in vec_chars {
            let msg = row.iter().collect::<String>();
            println!("{}", msg);
        }
    }
    pub fn print_yesno(ans: bool) {
        let msg = if ans { "Yes" } else { "No" };
        println!("{}", msg);
    }
}

// ====== snippet ======

/// 配列 xs で表された base 進数の値を評価する
/// 例: `eval_base_n_value(&[1, 2, 3], 10) == 123`
pub fn eval_base_n_value(xs: &[i64], base: i64) -> i64 {
    xs.iter().fold(0, |acc, &x| acc * base + x)
}

/// n の base 進数での表記を Vec で表す
/// 例: `to_base_n_value(123, 10) == vec![1, 2, 3]`
pub fn to_base_n_value(x: i64, base: i64) -> Vec<i64> {
    let mut ret = vec![];
    let mut x = x;
    while x > 0 {
        ret.push(x % base);
        x /= base;
    }
    ret.reverse();
    ret
}
