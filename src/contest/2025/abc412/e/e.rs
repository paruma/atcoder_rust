fn main() {
    input! {
        l: usize,
        r: usize,
    }

    let sqrt_r = num_integer::sqrt(r) as usize;

    // sqrt(r) 以下の素数をふるう
    let mut is_prime = vec![true; sqrt_r + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for p in 2..=sqrt_r {
        if !is_prime[p] {
            continue;
        }

        for q in (p * 2..=sqrt_r).step_by(p) {
            is_prime[q] = false;
        }
    }

    // is_prime2[v - l] = v が素数か？
    // ↓1が素数扱いになってるけど気にしない
    // let mut is_prime2 = vec![true; r - l + 1];
    // is_prime_pow[v - l] = v が 1 or 素数 or 素数のべき乗か
    let mut is_prime_pow = vec![true; r - l + 1];

    for p in 2..=sqrt_r {
        if !is_prime[p] {
            continue;
        }

        let start = num_integer::div_ceil(l, p) * p;
        let start = if start == p { p * 2 } else { start };

        for q in (start..=r).step_by(p) {
            // is_prime2[q - l] = false;
            is_prime_pow[q - l] = false;
        }

        for q in std::iter::successors(Some(p), |acc| acc.checked_mul(p)) {
            if (l..=r).contains(&q) {
                is_prime_pow[q - l] = true;
            }
        }
    }

    // dbg!(is_prime);
    // dbg!(is_prime2);
    // dbg!(&is_prime_pow);

    let ans: usize = if !is_prime_pow[0] {
        is_prime_pow.iter().filter(|&&p| p).count() + 1
    } else {
        is_prime_pow.iter().filter(|&&p| p).count()
    };
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
use itertools::{chain, iproduct, izip, Itertools};
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
