struct Ring {
    len: i64,
}

impl Ring {
    fn new(len: i64) -> Ring {
        Ring { len }
    }
    fn inc(&self, x: i64) -> i64 {
        (x + 1) % self.len
    }

    fn dec(&self, x: i64) -> i64 {
        (x + self.len - 1) % self.len
    }
    /// 時計回りに src から dst に移動したときの道のり
    fn dist_right(&self, src: i64, dst: i64) -> i64 {
        assert!((0..self.len).contains(&src));
        assert!((0..self.len).contains(&dst));
        if src > dst {
            dst + self.len - src
        } else {
            dst - src
        }
    }

    /// 反時計回りに src から dst に移動したときの道のり
    fn dist_left(&self, src: i64, dst: i64) -> i64 {
        assert!((0..self.len).contains(&src));
        assert!((0..self.len).contains(&dst));
        self.dist_right(dst, src)
    }

    /// begin から end に時計回りに回ったときに x に当たるかどうか
    fn contains(&self, begin: i64, end: i64, x: i64) -> bool {
        assert!((0..self.len).contains(&begin));
        assert!((0..self.len).contains(&end));
        if end < begin {
            (begin..end + self.len).contains(&x)
                || (begin..end + self.len).contains(&(x + self.len))
        } else {
            (begin..end).contains(&x)
        }
    }
}
fn solve_naive(n_point_div2: usize, ls: &[(i64, i64)], qs: &[(i64, i64)]) -> Vec<i64> {
    let n_point = (n_point_div2 as i64) * 2;
    let ring = Ring::new(n_point);
    qs.iter()
        .copied()
        .map(|(c, d)| {
            let q_slope = (c + d) % n_point;
            let q_intercept = (c - d).rem_euclid(n_point);
            let q_intercept = 0;
            dbg!(q_slope);
            dbg!(q_intercept);

            ls.iter()
                .copied()
                .filter(|&(a, b)| {
                    dbg!();
                    //
                    let a_rev = (q_slope - a).rem_euclid(n_point);
                    let b_rev = (q_slope - b).rem_euclid(n_point);

                    dbg!(a, a_rev, b, b_rev);

                    let intercept_a = if a <= a_rev {
                        a + n_point - a_rev
                    } else {
                        a - a_rev
                    };
                    let intercept_b = if b <= b_rev {
                        b + n_point - b_rev
                    } else {
                        b - b_rev
                    };
                    // let intercept_a = (a - a_rev).rem_euclid(n_point);
                    // let intercept_b = (b - b_rev).rem_euclid(n_point);
                    dbg!(intercept_a, intercept_b, q_intercept);
                    // q_intercept が intercept_a と intercept_b の間にあるか？
                    (min(intercept_a, intercept_b)..=max(intercept_a, intercept_b))
                        .contains(&q_intercept)

                    // if (intercept_a - intercept_b).rem_euclid(n_point) < n_point / 2 {
                    //     // b → a 時計
                    //     ring.contains(b, a, q_intercept)
                    // } else {
                    //     // a → b 時計
                    //     ring.contains(a, b, q_intercept)
                    // }
                })
                .count() as i64
        })
        .collect_vec()
}
fn main() {
    input! {
        n_point_div2: usize,
        n_line: usize,
        ls: [(i64, i64); n_line],
        q: usize,
        qs: [(i64, i64); q],
    }
    let n_point = (n_point_div2 * 2) as i64;
    let ls = ls
        .iter()
        .copied()
        .map(|(a, b)| (a % n_point, b % n_point))
        .collect_vec();
    let qs = qs
        .iter()
        .copied()
        .map(|(a, b)| (a % n_point, b % n_point))
        .collect_vec();
    let ans: Vec<i64> = solve_naive(n_point_div2, &ls, &qs);
    print_vec(&ans);
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
use std::cmp::{max, min};
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
