use std::{collections::VecDeque, io::stdin};

use itertools::Itertools;
#[allow(unused_imports)]
use myio::*;
pub mod myio {
    use std::io::BufRead;

    pub trait Reader {
        fn read_line(&mut self) -> String;

        fn read_vec_i64(&mut self) -> Vec<i64> {
            let buf = self.read_line();
            buf.trim()
                .split(' ')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        }

        fn read_vec_str(&mut self) -> Vec<String> {
            let buf = self.read_line();
            buf.trim()
                .split(' ')
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        }

        fn read_i64_1(&mut self) -> i64 {
            let buf = self.read_line();
            buf.parse::<i64>().unwrap()
        }

        fn read_i64_2(&mut self) -> (i64, i64) {
            let ns = self.read_vec_i64();
            (ns[0], ns[1])
        }

        fn read_i64_3(&mut self) -> (i64, i64, i64) {
            let ns = self.read_vec_i64();
            (ns[0], ns[1], ns[2])
        }

        fn read_i64_4(&mut self) -> (i64, i64, i64, i64) {
            let ns = self.read_vec_i64();
            (ns[0], ns[1], ns[2], ns[3])
        }

        fn read_any1<T>(&mut self) -> T
        where
            T: std::str::FromStr,
            T::Err: std::fmt::Debug,
        {
            let buf = self.read_line();
            buf.parse::<T>().unwrap()
        }

        fn read_any2<T0, T1>(&mut self) -> (T0, T1)
        where
            T0: std::str::FromStr,
            T0::Err: std::fmt::Debug,
            T1: std::str::FromStr,
            T1::Err: std::fmt::Debug,
        {
            let buf = self.read_line();
            let splitted = buf.trim().split(' ').collect::<Vec<_>>();
            let a0 = splitted[0].parse::<T0>().unwrap();
            let a1 = splitted[0].parse::<T1>().unwrap();
            (a0, a1)
        }
    }

    impl<R: BufRead> Reader for R {
        fn read_line(&mut self) -> String {
            let mut buffer = String::new();
            self.read_line(&mut buffer).unwrap();
            buffer.trim().to_string()
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    a: usize,
    b: usize,
}
struct Graph {
    n_vertex: usize,
    edges: Vec<Edge>,
    adj: Vec<Vec<usize>>,
}

impl Graph {
    fn new(n_vertex: usize, edges: &[Edge]) -> Graph {
        let edges = edges.to_vec();
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n_vertex];
        for edge in &edges {
            adj[edge.a].push(edge.b);
            adj[edge.b].push(edge.a);
        }
        Graph {
            n_vertex,
            edges,
            adj,
        }
    }
}

fn calc_dist(from: usize, graph: &Graph) -> Vec<i64> {
    let mut visited = vec![false; graph.adj.len()];
    let mut open: VecDeque<usize> = VecDeque::new();
    let mut dist = vec![-1_i64; graph.adj.len()];

    open.push_front(from);
    visited[from] = true;
    dist[from] = 0;
    while !open.is_empty() {
        let current_idx = open.pop_back().unwrap();
        for &next_idx in &graph.adj[current_idx] {
            if !visited[next_idx] {
                visited[next_idx] = true;
                dist[next_idx] = dist[current_idx] + 1;
                open.push_front(next_idx);
            }
        }
    }
    dist
}

struct Problem {
    nv1: usize,
    nv2: usize,
    ne: usize,
    edges: Vec<Edge>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: i64,
}

impl Problem {
    fn read<R: Reader>(mut r: R) -> Problem {
        let (nv1, nv2, ne) = r.read_i64_3();
        let nv1 = nv1 as usize;
        let nv2 = nv2 as usize;
        let ne = ne as usize;
        let edges = (0..ne)
            .map(|_| {
                let (a, b) = r.read_i64_2();
                let a = a as usize - 1;
                let b = b as usize - 1;
                Edge { a, b }
            })
            .collect_vec();
        Problem {
            nv1,
            nv2,
            ne,
            edges,
        }
    }
    // 必要に応じてダミーデータでassertを書くのをする。
    fn solve(self) -> Answer {
        let is_group1 = |i: usize| i < self.nv1;
        let is_group2 = |i: usize| self.nv1 <= i && i < self.nv1 + self.nv2;

        // すべての辺の端点はどちらもgroup1 に属しているかgroup2 に属しているか
        assert!(self
            .edges
            .iter()
            .all(|e| { (is_group1(e.a) && is_group1(e.b)) || (is_group2(e.a) && is_group2(e.b)) }));

        // edges は2つの連結グラフ（この2つは非連結）で構成されている
        // edges を2つの連結グラフに分ける。
        let edges1 = self
            .edges
            .clone()
            .into_iter()
            .filter(|e| is_group1(e.a))
            .collect_vec();
        let edges2 = self
            .edges
            .clone()
            .into_iter()
            .filter(|e| is_group2(e.a))
            .map(|e| Edge {
                a: e.a - self.nv1,
                b: e.b - self.nv1,
            })
            .collect_vec();

        assert!(edges1.len() + edges2.len() == self.edges.len());
        let graph1 = Graph::new(self.nv1, &edges1);
        let graph2 = Graph::new(self.nv2, &edges2);

        let dist1 = calc_dist(0, &graph1);
        let dist2 = calc_dist(self.nv2 - 1, &graph2);


        Answer {
            ans: dist1.iter().max().unwrap() + dist2.iter().max().unwrap() + 1,
        }
    }
}

impl Answer {
    fn print(self) {
        println!("{}", self.ans)
    }
}

fn main() {
    Problem::read(stdin().lock()).solve().print();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, expected: Answer) {
        let actual = Problem::read(input.as_bytes()).solve();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test() {
        let input = "
3 4 6
1 2
2 3
4 5
4 6
1 3
6 7
        "
        .trim();
        check(input, Answer { ans: 5 });
    }
}
