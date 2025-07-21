fn main() {
    input! {
        n: i64,
        m: usize,
        xys: [(i64, i64); m],
    }
    // まずは愚直

    let ans = if n <= 1200 {
        let mut dp = vec![0; n as usize + 1];

        dp[0] = 0;

        for i in 1..=n {
            for &(x, y) in &xys {
                if i >= x {
                    dp[i as usize] = i64::max(dp[i as usize], dp[(i - (x - y)) as usize] + x);
                }
                dp[i as usize] = i64::max(dp[i as usize], dp[(i - 1) as usize] + 1);
            }
        }

        dp[n as usize]
    } else {
        let mut dp = vec![0; 1200 + 1];

        dp[0] = 0;

        for i in 1..=1200 {
            for &(x, y) in &xys {
                if i >= x {
                    dp[i as usize] = i64::max(dp[i as usize], dp[(i - (x - y)) as usize] + x);
                }
                dp[i as usize] = i64::max(dp[i as usize], dp[(i - 1) as usize] + 1);
            }
        }

        //let saikyo = xys.iter().max_by(|((x1, y1), (x2, y2))|)
        let (saikyo_x, saikyo_y) = *xys
            .iter()
            .max_by_key(|(x, y)| (rational::Rational64::new(*x, x - y), Reverse(x - y)))
            .unwrap();
        let saikyo_diff = saikyo_x - saikyo_y;
        let cnt = (n - saikyo_x) / saikyo_diff + 1;
        let remain = n - saikyo_diff * cnt;
        dp[remain as usize] + cnt * saikyo_x
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
use num::rational;
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
