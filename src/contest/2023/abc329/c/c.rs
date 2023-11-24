#[derive_readable]
struct Problem {
    n: usize,
    s: Bytes,
}

impl Problem {
    fn read() -> Problem {
        input! {
            p:Problem
        }
        p
    }
    fn solve(&self) -> Answer {
        let s = &self.s;
        let run_length = s.iter().copied().dedup_with_count().collect_vec();
        let mut cnt = vec![0; 26];
        for &(n, c) in run_length.iter() {
            cnt[(c - b'a') as usize] = max(n, cnt[(c - b'a') as usize]);
        }

        let ans = cnt.iter().copied().sum::<usize>() as i64;
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        let s = &self.s;
        let run_length = s.iter().copied().dedup_with_count().collect_vec();

        let ans = run_length
            .iter()
            .copied()
            .into_group_map_by(|(_cnt, ch)| *ch)
            .values()
            .map(|v| v.iter().map(|(cnt, _ch)| cnt).max().unwrap())
            .sum::<usize>() as i64;

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
    Problem::read().solve2().print();
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

use std::cmp::max;

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
