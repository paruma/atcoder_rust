fn main() {
    input! {
        n: usize,
        xs: [Usize1; n],
    }

    let ys = xs
        .iter()
        .copied()
        .tuple_windows()
        .map(|(x0, x1)| if x0 < x1 { 1 } else { -1 })
        .collect_vec();

    let zs = ys
        .iter()
        .copied()
        .tuple_windows()
        .map(|(y0, y1)| y1 - y0)
        .collect_vec();

    let mut poss0 = vec![];
    let mut poss2 = vec![];
    let mut possneg2 = vec![];

    for (i, z) in zs.iter().copied().enumerate() {
        if z == 0 {
            poss0.push(i);
        } else if z == 2 {
            poss2.push(i);
        } else {
            // z == -2
            possneg2.push(i);
        }
    }
    // 番兵
    poss0.push(zs.len());
    poss2.push(zs.len());
    possneg2.push(zs.len());

    // dbg!(&ys);
    // dbg!(&zs);
    // dbg!(&poss0);
    // dbg!(&poss2);
    // dbg!(&possneg2);

    // [n-4, n)

    let ans: usize = (0..=n - 4)
        .map(|begin| {
            if ys[begin] == 1 {
                let next_neg2 = {
                    let key = possneg2.lower_bound(&begin);
                    possneg2[key]
                };
                let next2 = {
                    let key = poss2.lower_bound(&begin);
                    poss2[key]
                };

                // dbg!(next_neg2);
                // dbg!(next2);

                if next_neg2 >= next2 || next2 == zs.len() {
                    0
                } else {
                    let next_next_neg2 = {
                        let key = possneg2.upper_bound(&next2);
                        possneg2[key]
                    };
                    next_next_neg2 - next2
                }
            } else {
                0
            }
        })
        .sum::<usize>();
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
use superslice::Ext;

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
