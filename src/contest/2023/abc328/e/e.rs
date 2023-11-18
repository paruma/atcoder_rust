#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Edge {
    u: Usize1,
    v: Usize1,
    w: i64,
}
struct Problem {
    n_vertex: usize,
    n_edge: usize,
    k: i64,
    edges: Vec<Edge>,
}

pub fn has_cycle(n_vertex: usize, edges: &[Edge]) -> bool {
    let mut uf = UnionFind::new(n_vertex);
    for &e in edges {
        if uf.equiv(e.u, e.v) {
            return true;
        }
        uf.union(e.u, e.v);
    }
    false
}
impl Problem {
    fn read() -> Problem {
        input! {
            n_vertex: usize,
            n_edge: usize,
            k: i64,
            edges: [Edge; n_edge],
        }
        Problem { n_vertex, n_edge, k, edges }
    }
    fn solve(&self) -> Answer {
        let Problem { n_vertex, n_edge, k, edges } = self;
        let ans = edges
            .iter()
            .combinations(*n_vertex - 1)
            .filter(|sub_edges| {
                // combinations のところでcopied すればよかった
                let sub_edges = sub_edges.iter().copied().copied().collect_vec();
                !has_cycle(*n_vertex, &sub_edges)
            })
            .map(|sub_edges| sub_edges.iter().map(|e| e.w).sum::<i64>() % k)
            .min()
            .unwrap();
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

// ====== import ======
#[allow(unused_imports)]
use itertools::Itertools;
use petgraph::unionfind::UnionFind;
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
