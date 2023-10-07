struct Problem {
    n: usize,
    cnt_table: HashMap<i64, i64>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            infos: [(i64, i64);n],
        }

        let cnt_table: HashMap<i64, i64> = infos.into_iter().collect();
        Problem { n, cnt_table }
    }
    fn solve(&self) -> Answer {
        let Problem { n, cnt_table } = self;
        let mut cnt_table = cnt_table.clone();
        let mut p_queue: BinaryHeap<Reverse<i64>> = BinaryHeap::new();

        for size in cnt_table.keys() {
            p_queue.push(Reverse(*size));
        }

        while let Some(Reverse(size)) = p_queue.pop() {
            // sizeに対応する数
            let current_cnt = cnt_table[&size];
            if current_cnt >= 2 {
                *cnt_table.entry(size * 2).or_insert(0) += cnt_table[&size] / 2;
                *cnt_table.get_mut(&size).unwrap() = cnt_table[&size] % 2;
                p_queue.push(Reverse(size * 2));
            }
        }
        let ans = cnt_table.values().sum();
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

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

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
