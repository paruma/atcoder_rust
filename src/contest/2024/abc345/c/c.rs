//#[derive_readable]
struct Problem {
    s: Vec<u8>,
}

fn nc2(n: i64) -> i64 {
    n * (n - 1) / 2
}

impl Problem {
    fn read() -> Problem {
        input! {
            s: Bytes
        }
        Problem { s }
    }
    fn solve(&self) -> Answer {
        let n = self.s.len();

        if self.s.iter().all_unique() {
            let ans = nc2(n as i64);
            return Answer { ans };
        }
        // counts でよい

        let tmp = self.s.iter().copied().into_group_map_by(|x| *x);
        let cnt_list = tmp
            .iter()
            .map(|(ch, xs)| (*ch, xs.len()))
            .collect::<HashSet<_>>();
        let ans = nc2(n as i64)
            - cnt_list
                .iter()
                .map(|(_, cnt)| if *cnt == 1 { 0 } else { nc2(*cnt as i64) })
                .sum::<i64>()
            + 1;
        Answer { ans }
    }

    fn solve0(&self) -> Answer {
        let n = self.s.len();
        let ans = (0..n)
            .tuple_combinations()
            .map(|(i, j)| {
                let mut tmp = self.s.clone();
                tmp.swap(i, j);
                tmp
            })
            .unique()
            .inspect(|x| {
                dbg!(String::from_utf8(x.clone()).unwrap());
            })
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
    // Problem::read().solve0().print();
    Problem::read().solve().print();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    fn sub(s: &[u8]) {
        let p = Problem { s: s.to_vec() };
        dbg!(p.solve().ans);
        assert_eq!(p.solve().ans, p.solve0().ans);
    }

    #[test]
    fn test_problem() {
        sub(b"a");
        sub(b"ab");
        sub(b"abdefg");
        sub(b"aaabbb");
        sub(b"aaabbbffff");
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
