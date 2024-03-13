//#[derive_readable]
struct Problem {
    n: usize,
    xs: Vec<i64>,
    ps: Vec<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    from: usize,
    to: usize,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            xs: [i64; n ],
            ps: [Usize1; n-1 ],
        }
        Problem { n, xs, ps }
    }
    fn solve(&self) -> Answer {
        let Problem { n, xs, ps } = self;
        let n = *n;

        let mut adj = vec![vec![]; n];
        for i in 0..(n - 1) {
            // i+1 ← ps[i] って辺がある
            adj[ps[i]].push(i + 1);
        }

        let mut open: Queue<(usize, i64)> = Queue::new();
        let mut visited = vec![false; n];
        open.push((0, 0));
        let mut height_list = vec![-3; n];
        visited[0] = true;
        height_list[0] = 0;

        while let Some((current, height)) = open.pop() {
            for &next in &adj[current] {
                if !visited[next] {
                    visited[next] = true;
                    height_list[next] = height + 1;
                    open.push((next, height + 1));
                }
            }
        }
        // lg!(&height_list);

        let max_height = height_list.iter().copied().max().unwrap();
        let mut coef = vec![0; max_height as usize + 1];

        for i in 0..n {
            let h = height_list[i];
            coef[h as usize] += xs[i];
        }

        //lg!(&coef);

        // 後ろから見て非ゼロの

        let ans = coef
            .iter()
            .rev()
            .copied()
            .find(|x| *x != 0)
            .map(|x| if x > 0 { "+" } else { "-" })
            .unwrap_or("0");

        let ans = ans.to_string();
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: String,
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

use mod_queue::*;
pub mod mod_queue {
    use std::collections::VecDeque;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Queue<T> {
        raw: VecDeque<T>,
    }
    impl<T> Queue<T> {
        pub fn new() -> Self {
            Queue {
                raw: VecDeque::new(),
            }
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
