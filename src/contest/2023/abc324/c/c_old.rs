//#[derive_readable]
struct Problem {
    recieved: Vec<u8>,
    cand_list: Vec<Vec<u8>>,
}

fn is_dist_leq1(str1: &[u8], str2: &[u8]) -> bool {
    if str1 == str2 {
        return true;
    }
    if usize::abs_diff(str1.len(), str2.len()) >= 2 {
        return false;
    }

    if str1.len() == str2.len() {
        return izip!(str1, str2).filter(|(c1, c2)| c1 != c2).count() <= 1;
    }

    if str1.len() == str2.len() + 1 {
        // str1 のほうが1長い
        let mut q1 = Queue::new();
        let mut q2 = Queue::new();

        for c1 in str1 {
            q1.push(*c1);
        }
        for c2 in str2 {
            q2.push(*c2);
        }
        let mut diff_cnt = 0;

        loop {
            if !q1.is_empty() && q1.peek() == q2.peek() {
                q1.pop();
                q2.pop();
            }
            if q1.peek() != q2.peek() {
                q1.pop();
                diff_cnt += 1;
            }
            if q1.is_empty() {
                break;
            }
        }

        return diff_cnt <= 1;
        // abacbc
        // aba bc

        // abc
        //  bc

        // abc
        // ab
    }
    if str1.len() + 1 == str2.len() {
        return is_dist_leq1(str2, str1);
    }

    panic!()
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: i64,
            recieved: Bytes,
            cand_list: [Bytes; n],
        }
        Problem { recieved, cand_list }
    }
    fn solve(&self) -> Answer {
        let ans = self
            .cand_list
            .iter()
            .enumerate()
            .filter(|(_i, cand)| is_dist_leq1(cand, &self.recieved))
            .map(|(i, _cand)| i + 1) //1オリジンにする
            .collect_vec();
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<usize>,
}

impl Answer {
    fn print(&self) {
        println!("{}", self.ans.len());
        print_vec_1line(&self.ans);
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
        // abacbc
        // aba bc

        // abc
        //  bc

        // abc
        // ab
        assert!(is_dist_leq1(b"abacbc", b"ababc"));
        assert!(is_dist_leq1(b"abc", b"bc"));
        assert!(is_dist_leq1(b"abc", b"ab"));
        assert!(is_dist_leq1(b"ab", b"abc"));
        assert!(is_dist_leq1(b"", b""));
        assert!(!is_dist_leq1(b"abc", b"c"));
        assert!(is_dist_leq1(b"abc", b"ac"));
        assert!(!is_dist_leq1(b"abc", b"ad"));

        assert_eq!(1 + 1, 2);
    }
}

// ====== import ======
use itertools::izip;
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
use mod_queue::*;
pub mod mod_queue {
    use std::collections::VecDeque;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Queue<T> {
        raw: VecDeque<T>,
    }
    impl<T> Queue<T> {
        pub fn new() -> Self {
            Queue { raw: VecDeque::new() }
        }
        pub fn push(&mut self, value: T) {
            self.raw.push_front(value)
        }
        pub fn pop(&mut self) -> Option<T> {
            self.raw.pop_back()
        }
        pub fn peek(&self) -> Option<&T> {
            self.raw.back()
        }
        pub fn is_empty(&self) -> bool {
            self.raw.is_empty()
        }
    }
    impl<T> Default for Queue<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}
