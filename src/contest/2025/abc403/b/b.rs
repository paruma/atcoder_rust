fn solve1(xs: &[char], ys: &[char]) -> bool {
    std::iter::repeat('a'..='z')
        .take(4)
        .multi_cartesian_product()
        .any(|zs| {
            //
            let next_xs = {
                let mut next_xs = xs.to_vec();
                let mut cnt = 0;
                for i in 0..xs.len() {
                    if xs[i] == '?' {
                        next_xs[i] = zs[cnt];
                        cnt += 1;
                    }
                }
                next_xs
            };
            // windows を使うと良かった
            // (0..next_xs.len() - ys.len() + 1).any(|i| next_xs[i..i + ys.len()] == ys)
            next_xs.windows(ys.len()).any(|sub_xs| sub_xs == ys)
        })
}

fn solve2(xs: &[char], ys: &[char]) -> bool {
    // 全部 multi_cartesian_product
    let alphabet = ('a'..='z').collect_vec();

    xs.iter()
        .copied()
        .map(|x| if x == '?' { alphabet.clone() } else { vec![x] })
        .multi_cartesian_product()
        .any(|next_xs| next_xs.windows(ys.len()).contains(&ys))
}

fn solve3(xs: &[char], ys: &[char]) -> bool {
    // 工夫する
    xs.windows(ys.len())
        .any(|sub_xs| izip!(sub_xs, ys).all(|(&x, &y)| x == '?' || x == y))
}
fn main() {
    input! {
        xs: Chars,
        ys: Chars,
    }
    let ans: bool = solve3(&xs, &ys);
    print_yesno(ans);
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
use print_vec::*;
pub mod print_vec {

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
}

#[allow(unused)]
fn print_yesno(ans: bool) {
    let msg = if ans { "Yes" } else { "No" };
    println!("{}", msg);
}

// ====== snippet ======
