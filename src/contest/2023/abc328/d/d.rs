#[derive_readable]
struct Problem {
    s: Bytes,
}

impl Problem {
    fn read() -> Problem {
        input! {
            p: Problem
        }
        p
    }
    fn solve(&self) -> Answer {
        let s = &self.s;
        let mut stack: Vec<u8> = Vec::new();
        for ch in s {
            stack.push(*ch);

            // 上3つがABC の場合
            let stack_size = stack.len();
            // stack.get(stack_size - 3..stack_size) == Some(b"ABC") こう書ける？
            // stack_size >= 3 && &stack[stack_size..] == b"ABC".as_slice(); こうもかけそう。
            // stack.ends_with(b"ABC"); こうもかけそう。
            if stack_size >= 3
                && stack[stack_size - 3] == b'A'
                && stack[stack_size - 2] == b'B'
                && stack[stack_size - 1] == b'C'
            {
                stack.pop();
                stack.pop();
                stack.pop();
            }
        }

        let ans = stack;
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<u8>,
}

impl Answer {
    fn print(&self) {
        // TODO: print_bytes で書ける
        println!("{}", String::from_utf8(self.ans.clone()).unwrap());
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
