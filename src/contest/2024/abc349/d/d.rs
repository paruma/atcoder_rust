#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Range {
    l: i64,
    r: i64,
}

#[derive_readable]
#[derive(Debug)]
struct Problem {
    range: Range,
}

impl Problem {
    fn read() -> Problem {
        input! {
            p: Problem
        }
        p
    }
    fn solve(&self) -> Answer {
        let range = self.range;
        let mut ans = vec![];
        let mut current = range.l;
        loop {
            let mut changes = false;
            for i in (0..u32::min(current.trailing_zeros(), 61) + 1).rev() {
                // 2^i を足す
                let next = current + 2_i64.pow(i);
                if next <= range.r {
                    changes = true;
                    ans.push(Range {
                        l: current,
                        r: next,
                    });
                    current = next;
                    break;
                }
            }
            if !changes {
                break;
            }
        }
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // セグ木を意識した実装
        let mut left = self.range.l;
        let mut right = self.range.r;
        let mut pow2 = 1;
        let mut ans: Vec<Range> = vec![];

        while left < right {
            if left & 1 == 1 {
                ans.push(Range {
                    l: left * pow2,
                    r: (left + 1) * pow2,
                });

                left += 1;
            }

            if right & 1 == 1 {
                right -= 1;
                ans.push(Range {
                    l: right * pow2,
                    r: (right + 1) * pow2,
                });
            }

            left >>= 1;
            right >>= 1;
            pow2 <<= 1;
        }

        ans.sort_by_key(|r| r.l);
        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        todo!();
        // let ans = 0;
        // Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<Range>,
}

impl Answer {
    fn print(&self) {
        println!("{}", self.ans.len());
        for range in &self.ans {
            println!("{} {}", range.l, range.r);
        }
    }
}

fn main() {
    Problem::read().solve2().print();
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

    fn check(p: &Problem) {
        assert_eq!(p.solve(), p.solve_naive());
    }

    fn make_random_problem() -> Problem {
        todo!()
        // let mut rng = SmallRng::from_entropy();
        // let n = rng.gen_range(1..=10);
        // let p = Problem { _a: n };
        // println!("{:?}", &p);
        // p
    }

    #[test]
    fn test_with_naive() {
        // 手動でテストを作るのもOK
        for _ in 0..100 {
            // let p = make_random_problem();
            // check(&p);
        }
    }
}

// ====== import ======
#[allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
use num::PrimInt;
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet};

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
