fn solve_sub(cnts: &[i64], idxes: &[usize]) -> i64 {
    let n = idxes.len();
    let mut dp = vec![[0; 2]; n + 1];
    for i in 0..n {
        dp[i + 1][0] = dp[i][1];
        dp[i + 1][1] = i64::min(dp[i][0], dp[i][1]) + cnts[idxes[i]];
    }
    i64::min(dp[n][0], dp[n][1])
}
fn main() {
    input! {
        n: usize,
        d: usize,
        xs: [usize; n],
    }

    let max = xs.iter().copied().max().unwrap();
    let cnts = xs.iter().copied().fold(vec![0; max + 1], |mut acc, x| {
        acc[x] += 1;
        acc
    });

    let exists = cnts.iter().copied().map(|cnt| cnt > 0).collect_vec();

    let ans: i64 = if d == 0 {
        cnts.iter()
            .copied()
            .map(|cnt| if cnt == 0 { 0 } else { cnt - 1 })
            .sum::<i64>()
    } else {
        (0..d)
            .map(|r| {
                // d で割った余りが r
                let idxes = (0..)
                    .map(|i| i * d + r)
                    .take_while(|i| *i <= max)
                    .collect_vec();

                let rle = idxes
                    .iter()
                    .copied()
                    .map(|i| exists[i])
                    .dedup_with_count()
                    .collect_vec();

                let mut cnt_all = 0;
                let mut ans_sum = 0;
                for (cnt, pred) in rle {
                    if pred {
                        let sub_ans = solve_sub(&cnts, &idxes[cnt_all..cnt_all + cnt]);
                        ans_sum += sub_ans;
                    }
                    cnt_all += cnt;
                }
                ans_sum
            })
            .sum::<i64>()
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
