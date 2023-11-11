//#[derive_readable]
struct Problem {
    n: i64,
    m: usize,
    xs: Vec<usize>,
    ys: Vec<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Edge {
    from: usize,
    to: usize,
}

impl Edge {
    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }
}

pub fn make_adj(n_vertex: usize, edges: &[Edge]) -> Vec<Vec<Edge>> {
    let mut adj = vec![vec![]; n_vertex];
    for &e in edges {
        adj[e.from].push(e);
    }
    adj
}

// これもスニペット化したい。

#[allow(clippy::collapsible_else_if)]
pub fn is_bipartite_graph(adj: &Vec<Vec<Edge>>) -> bool {
    let n_vertex = adj.len();
    let mut visited = vec![false; n_vertex];
    let mut odd_even_list = vec![-1; n_vertex];
    for init in 0..n_vertex {
        if visited[init] {
            continue;
        }
        let mut open: Queue<usize> = Queue::new();
        open.push(init);
        visited[init] = true;
        odd_even_list[init] = 0;
        while let Some(current) = open.pop() {
            for &e in &adj[current] {
                if !visited[e.to] {
                    visited[e.to] = true;
                    open.push(e.to);
                    odd_even_list[e.to] = (odd_even_list[e.from] + 1) % 2;
                } else {
                    if odd_even_list[e.from] == odd_even_list[e.to] {
                        return false;
                    }
                }
            }
        }
    }
    true
}

// これもスニペット化したい。
fn is_bipartite_graph_by_uf(n_vertex: usize, edges: &[Edge]) -> bool {
    let mut uf = UnionFind::new(2 * n_vertex);
    for &e in edges {
        // (x, y + n_vertex) の辺がある: xのラベルが0なら、yのラベルは1
        // (x + n_vertex, y) の辺がある: xのラベルが1なら、yのラベルは0
        uf.union(e.from, e.to + n_vertex);
        uf.union(e.from + n_vertex, e.to);
    }
    (0..n_vertex).all(|i| !uf.equiv(i, i + n_vertex))
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: i64,
            m: usize,
            xs: [Usize1; m],
            ys: [Usize1; m],
        }
        Problem { n, m, xs, ys }
    }
    fn solve(&self) -> Answer {
        let Problem { n, m, xs, ys } = self;
        let edges =
            izip!(xs, ys).flat_map(|(&x, &y)| [Edge::new(x, y), Edge::new(y, x)]).collect_vec();
        let n_vertex = *n as usize;
        let adj = make_adj(n_vertex, &edges);

        let ans = is_bipartite_graph(&adj);
        let ans2 = is_bipartite_graph_by_uf(n_vertex, &edges);
        assert_eq!(ans, ans2);
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // UnionFind を使った二部グラフの判定 (2SATのノリ)
        let Problem { n, m, xs, ys } = self;
        let n_vertex = *n as usize;

        let mut uf = UnionFind::new(n_vertex);
        for (&x, &y) in izip!(xs, ys) {
            uf.union(x, y + n_vertex);
            uf.union(x + n_vertex, y);
        }

        let ans = false;
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: bool,
}

impl Answer {
    fn print(&self) {
        //println!("{}", self.ans);
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
use itertools::izip;
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
