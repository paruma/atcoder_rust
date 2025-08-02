struct DfsGraph<'a> {
    adj: &'a [Vec<usize>],
    dst: usize,
}

impl DfsGraph<'_> {
    fn new(adj: &[Vec<usize>], dst: usize) -> DfsGraph<'_> {
        // adj.len() は グラフの頂点の数
        DfsGraph { adj, dst }
    }

    /// 計算量: O(頂点の数 + 辺の数)
    fn exec(&self, v: usize, visited: &mut Vec<bool>) -> Option<Vec<usize>> {
        if v == self.dst {
            if visited[v] {
                return None;
            } else {
                return Some(vec![v]);
            }
        }
        // 行きがけ
        visited[v] = true;

        for &next in &self.adj[v] {
            if !visited[next] {
                let mut ans_opt = self.exec(next, visited);
                if let Some(ans) = &mut ans_opt {
                    ans.push(v);
                    visited[v] = false;
                    return Some(ans_opt.unwrap());
                }
            }
        }
        // 帰りがけ
        visited[v] = false;
        None
    }
}
fn solve(nv: usize, ne: usize, src: usize, dst: usize, es: &[(usize, usize)]) -> Vec<usize> {
    let mut adj = es.iter().copied().fold(vec![vec![]; nv], |mut acc, e| {
        acc[e.0].push(e.1);
        acc[e.1].push(e.0);
        acc
    });

    adj.iter_mut().for_each(|row| row.sort());

    let d = DfsGraph::new(&adj, dst);

    let mut ans = d.exec(src, &mut vec![false; nv]).unwrap();
    ans.reverse();

    ans
}

#[fastout]
fn main() {
    input! {
        t: usize
    }

    let ans = (0..t)
        .map(|_| {
            input! {
                nv: usize,
                ne: usize,
                src: Usize1,
                dst: Usize1,
                es: [(Usize1, Usize1); ne]
            }

            solve(nv, ne, src, dst, &es)
        })
        .collect_vec();

    for row in ans {
        let row = row.iter().copied().map(|x| x + 1).collect_vec();
        let msg = row.iter().map(|x| format!("{}", x)).join(" ");
        println!("{}", msg);
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

// ====== import ======
#[allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
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
