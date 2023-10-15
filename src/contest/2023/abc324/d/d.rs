//#[derive_readable]
struct Problem {
    n: i64,
    s: Vec<u8>,
}

fn is_ok(s_sorted: &[u8], sq: i64) -> bool {
    let n = s_sorted.len();
    let mut sq_bytes = sq.to_string().bytes().collect_vec();
    if sq_bytes.len() > n {
        return false;
    }
    for _ in (0..(n - sq_bytes.len())) {
        sq_bytes.push(b'0');
    }
    sq_bytes.sort();
    sq_bytes == s_sorted
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: i64,
            s: Bytes,
        }
        Problem { n, s }
    }
    fn solve(&self) -> Answer {
        let s_sorted = self.s.iter().copied().sorted().collect_vec();
        // 0のケアが必要
        let ans = (0_i64..3_170_000) //(0..4_000_000)
            .map(|i| i * i)
            .filter(|&sq| is_ok(&s_sorted, sq))
            //.filter(|sq| sq.to_string().bytes().sorted().collect_vec() == s_sorted)
            .count() as i64;

        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: i64,
}

impl Answer {
    fn print(&self) {
        println!("{}", self.ans);
    }
}

fn main() {
    Problem::read().solve().print();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }
}

use std::collections::HashSet;

// ====== import ======
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};

// ====== output func ======
#[allow(unused_imports)]
use print_vec::*;
pub mod print_vec {
    use itertools::Itertools;
    use proconio::fastout;
    #[fastout]
    pub fn print_vec<T: std::fmt::Debug>(arr: &[T]) {
        for a in arr {
            println!("{:?}", a);
        }
    }
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Debug>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{:?}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Debug>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{:?}", x)).join(" ");
            println!("{}", msg);
        }
    }
    pub fn print_bytes(bytes: &[u8]) {
        let msg = String::from_utf8(bytes.to_vec()).unwrap();
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
