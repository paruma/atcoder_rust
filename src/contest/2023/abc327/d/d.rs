//#[derive_readable]
struct Problem {
    n: i64,
    m: usize,
    xs: Vec<usize>,
    ys: Vec<usize>,
}

// TODO: スニペットにしたい
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    from: usize,
    to: usize,
}

impl Edge {
    fn new(from: usize, to: usize) -> Edge {
        Edge { from, to }
    }
}

fn make_adj(n_vertex: usize, edges: &[Edge]) -> Vec<Vec<Edge>> {
    let mut adj = vec![vec![]; n_vertex];

    for &e in edges {
        adj[e.from].push(e);
    }

    adj
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

        let mut visited = vec![false; n_vertex];
        let mut odd_even_list = vec![-1; n_vertex]; // 0 or 1 を入れる
        for init in 0..n_vertex {
            if visited[init] {
                continue;
            }
            // TODO: サンプルコードで Queue を使うようにする
            let mut open: VecDeque<usize> = VecDeque::new();
            open.push_front(init);
            visited[init] = true;
            odd_even_list[init] = 0;

            while let Some(current) = open.pop_back() {
                for &e in &adj[current] {
                    if !visited[e.to] {
                        visited[e.to] = true;
                        open.push_front(e.to);
                        odd_even_list[e.to] = (odd_even_list[e.from] + 1) % 2;
                    } else {
                        // 訪問済
                        // 偶奇チェックをする
                        if odd_even_list[e.from] == odd_even_list[e.to] {
                            return Answer { ans: false };
                        }
                    }
                }
            }
        }

        let ans = true;
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

use std::collections::VecDeque;

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
