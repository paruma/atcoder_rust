#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    from: Usize1,
    to: Usize1,
    w: usize,
}

const W: usize = 1024;

fn solve1(nv: usize, ne: usize, es: &[Edge]) -> Option<usize> {
    // 頂点倍化 + BFS を使う
    // min{ w 上の辺重み xor sum | w: 頂点 0 から頂点 n-1 までの walk} を値域の空間で全探索する

    fn id(v: usize, x: usize) -> usize {
        v * W + (x as usize)
    }

    let snv = nv * W;
    let ses = (0..W)
        .flat_map(|x| {
            es.iter().map(move |e| {
                let from_v = e.from;
                let from_x = x;
                let to_v = e.to;
                let to_x = x ^ e.w;
                (id(from_v, from_x), id(to_v, to_x))
            })
        })
        .collect_vec();
    let s_adj = ses.iter().copied().fold(vec![vec![]; snv], |mut acc, e| {
        acc[e.0].push(e.1);
        acc
    });

    let mut open: Queue<usize> = Queue::new();
    let mut visited = vec![false; snv];

    open.push(id(0, 0));
    visited[id(0, 0)] = true;

    while let Some(current) = open.pop() {
        for &next in &s_adj[current] {
            if !visited[next] {
                visited[next] = true;
                open.push(next);
            }
        }
    }

    (0..W).find(|&x| visited[id(nv - 1, x)])
}
fn main() {
    input! {
        nv: usize,
        ne: usize,
        es: [Edge; ne],
    }

    let ans: Option<usize> = solve1(nv, ne, &es);
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
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

use ac_library::SccGraph;
// ====== import ======
#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, izip};
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet};

// ====== output func ======
#[allow(unused_imports)]
use print_util::*;
pub mod print_util {
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
    #[fastout]
    pub fn print_vec_chars(vec_chars: &[Vec<char>]) {
        for row in vec_chars {
            let msg = row.iter().collect::<String>();
            println!("{}", msg);
        }
    }
    pub fn print_yesno(ans: bool) {
        let msg = if ans { "Yes" } else { "No" };
        println!("{}", msg);
    }
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
            self.raw.push_back(value)
        }
        pub fn pop(&mut self) -> Option<T> {
            self.raw.pop_front()
        }
        pub fn peek(&self) -> Option<&T> {
            self.raw.front()
        }
        pub fn is_empty(&self) -> bool {
            self.raw.is_empty()
        }
        pub fn len(&self) -> usize {
            self.raw.len()
        }
    }
    impl<T> Default for Queue<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}
