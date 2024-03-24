#[derive_readable]
struct Problem {
    w: usize,
    b: usize,
}

impl Problem {
    fn read() -> Problem {
        input! {
            p: Problem
        }
        p
    }
    fn solve(&self) -> Answer {
        let w = self.w;
        let b = self.b;
        let piano = b"wbwbwwbwbwbw";
        let piano_loop = std::iter::repeat(piano)
            .take(100)
            .flatten()
            .copied()
            .collect_vec();

        let ans = (0..piano_loop.len() - (w + b)).any(|start| {
            // let xs = piano_loop[start..start + w + b].to_vec();
            let xs = &piano_loop[start..start + w + b];
            let count_w = xs.iter().copied().filter(|x| *x == b'w').count();
            let count_b = xs.iter().copied().filter(|x| *x == b'b').count();
            count_w == w && count_b == b
        });
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        let w = self.w;
        let b = self.b;
        let piano_loop = b"wbwbwwbwbwbw".repeat(100);
        let ans = piano_loop.windows(w + b).any(|xs| {
            let count_w = xs.iter().copied().filter(|x| *x == b'w').count();
            let count_b = xs.iter().copied().filter(|x| *x == b'b').count();
            count_w == w && count_b == b
        });

        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: bool,
}

impl Answer {
    fn print(&self) {
        print_yesno(self.ans);
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
