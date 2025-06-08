#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    u: Usize1,
    v: Usize1,
    w: i64,
}
impl Edge {
    fn rev(&self) -> Self {
        Self {
            u: self.v,
            v: self.u,
            w: self.w,
        }
    }
}

fn post_order(adj: &Vec<Vec<Edge>>, init: usize) -> Vec<usize> {
    struct DfsGraph<'a> {
        adj: &'a Vec<Vec<Edge>>,
        visited: Vec<bool>,
        post_order: Vec<usize>,
    }

    impl DfsGraph<'_> {
        fn new(adj: &Vec<Vec<Edge>>) -> DfsGraph<'_> {
            // adj.len() は グラフの頂点の数
            DfsGraph {
                adj,
                visited: vec![false; adj.len()],
                post_order: vec![],
            }
        }
        /// 計算量: O(頂点の数 + 辺の数)
        fn exec(&mut self, v: usize) {
            // 行きがけ
            self.visited[v] = true;

            for &edge in &self.adj[v] {
                if !self.visited[edge.v] {
                    self.exec(edge.v);
                }
            }
            // 帰りがけ
            self.post_order.push(v);
        }
    }
    let mut dfs = DfsGraph::new(adj);
    dfs.exec(init);
    dfs.post_order
}

fn make_adj(n_vertex: usize, edges: &[Edge]) -> Vec<Vec<Edge>> {
    let mut adj = vec![vec![]; n_vertex];
    for &e in edges {
        adj[e.u].push(e);
        adj[e.v].push(e.rev());
    }
    adj
}
fn main() {
    input! {
        n: usize,
        xs: [i64; n],
        edges: [Edge; n - 1]
    }

    let adj = make_adj(n, &edges);

    let post_order = post_order(&adj, 0);

    let mut dp_電子 = vec![0; n];
    let mut dp_エネルギー = vec![0; n];

    for v in post_order {
        dp_電子[v] = adj[v].iter().copied().map(|e| dp_電子[e.v]).sum::<i64>() + xs[v];
        dp_エネルギー[v] = adj[v]
            .iter()
            .copied()
            .map(|e| dp_電子[e.v].abs() * e.w + dp_エネルギー[e.v])
            .sum::<i64>();
    }

    let ans: i64 = dp_エネルギー[0];
    println!("{}", ans);
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
