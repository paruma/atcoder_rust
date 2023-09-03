use std::{collections::HashMap, io::stdin};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    a: usize,
    b: usize,
    len: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DirectedEdge {
    from: usize,
    to: usize,
    len: i64,
}
struct Problem {
    n_vertex: usize,
    n_edge: usize,
    edges: Vec<Edge>,
}

impl Problem {
    fn read<R: IProconReader>(mut r: R) -> Problem {
        let (n_vertex, n_edge) = r.read_usize_2();
        let edges = (0..n_edge)
            .map(|_| {
                let (a, b, c) = r.read_i64_3();
                Edge { a: (a - 1) as usize, b: (b - 1) as usize, len: c }
            })
            .collect_vec();
        Problem { n_vertex, n_edge, edges }
    }
    fn solve(&self) -> Answer {
        // 最初に書いたコード(permutation)
        let Problem { n_vertex, n_edge, edges } = self;

        let mut edge_to_len: HashMap<(usize, usize), i64> = HashMap::new();
        for &edge in edges {
            // ↓こうかけばよかった
            // edge_to_len.insert((edge.a, edge.b), edge.len);
            // edge_to_len.insert((edge.b, edge.a), edge.len);
            edge_to_len.entry((edge.a, edge.b)).or_insert(edge.len);
            edge_to_len.entry((edge.b, edge.a)).or_insert(edge.len);
        }

        let ans = (0..*n_vertex)
            .permutations(*n_vertex)
            .map(|path| {
                (0..*n_vertex - 1)
                    .map(|i| {
                        //i→i+1
                        edge_to_len.get(&(path[i], path[i + 1]))
                    })
                    .take_while(|x| x.is_some())
                    .map(|x| x.unwrap()) // flatten もありかも。ただし、エラーがでるmapとunwrap の方が安全かも。
                    .sum()
            })
            .max()
            .unwrap();

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // solveのリファクタリング(permuation)
        let Problem { n_vertex, n_edge, edges } = self;

        let edge_to_len = edges
            .iter()
            .flat_map(|e| [((e.a, e.b), e.len), ((e.b, e.a), e.len)])
            .collect::<HashMap<(usize, usize), i64>>();

        let ans = (0..*n_vertex)
            .permutations(*n_vertex)
            .map(|path| {
                (0..*n_vertex - 1)
                    .map(|i| {
                        //i→i+1
                        edge_to_len.get(&(path[i], path[i + 1]))
                    })
                    .take_while(|x| x.is_some())
                    .map(|x| x.unwrap())
                    .sum()
            })
            .max()
            .unwrap();

        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        // DFS(再帰)
        struct Dfs<'a> {
            visited: &'a mut Vec<bool>,
            adj: &'a Vec<Vec<DirectedEdge>>,
        }

        impl Dfs<'_> {
            fn exec(&mut self, v: usize) -> i64 {
                // v から訪問した場合の最大の長さを返す
                self.visited[v] = true;
                let mut max = 0;
                // 行きがけ
                for &edge in &self.adj[v] {
                    if !self.visited[edge.to] {
                        let max_cand = self.exec(edge.to) + edge.len;
                        max = i64::max(max, max_cand);
                    }
                }
                // 帰りがけ
                self.visited[v] = false;
                max
            }
        }
        let Problem { n_vertex, n_edge, edges } = self;

        // 隣接リスト
        let mut adj: Vec<Vec<DirectedEdge>> = vec![vec![]; *n_vertex];
        for &edge in edges {
            adj[edge.a].push(DirectedEdge { from: edge.a, to: edge.b, len: edge.len });
            adj[edge.b].push(DirectedEdge { from: edge.b, to: edge.a, len: edge.len });
        }

        let mut visited = vec![false; *n_vertex];
        let mut dfs = Dfs { adj: &adj, visited: &mut visited };
        let mut max = 0;
        for init in 0..*n_vertex {
            let cand_max = dfs.exec(init);
            max = i64::max(max, cand_max);
        }

        let ans = max;

        Answer { ans }
    }
    /*
    Stack では解けなかった
        fn solve4(&self) -> Answer {
            // DFS(Stackを使う)
            let Problem { n_vertex, n_edge, edges } = self;

            // 隣接リスト
            let mut adj: Vec<Vec<DirectedEdge>> = vec![vec![]; *n_vertex];
            for &edge in edges {
                adj[edge.a].push(DirectedEdge { from: edge.a, to: edge.b, len: edge.len });
                adj[edge.b].push(DirectedEdge { from: edge.b, to: edge.a, len: edge.len });
            }

            struct State {
                v: usize,       //頂点
                total_len: i64, // 今までの経路の長さ
            }

            // 経路だけ求めたい
            let ans = (0..*n_vertex)
                .map(|init| {
                    // init から始めた場合の長さの最大値を求める。
                    let mut visited = vec![false; *n_vertex];
                    let mut open = Vec::<State>::new();
                    open.push(State { v: init, total_len: 0 });

                    while let Some(current) = open.pop() {
                        // 行きがけ
                        visited[current.v] = true;
                        for e in &adj[current.v] {
                            if !visited[e.to] {
                                open.push(State { v: e.to, total_len: current.total_len + e.len })
                            }
                        }

                        // visited[current.v] をfalse にするタイミングがない？
                    }
                    0
                })
                .max()
                .unwrap();

            Answer { ans }
        }
        */

    fn solve5(&self) -> Answer {
        // DFS(再帰2)
        struct Dfs<'a> {
            adj: &'a Vec<Vec<DirectedEdge>>,
            visited: Vec<bool>,
            max: i64,
        }

        impl Dfs<'_> {
            fn new(adj: &Vec<Vec<DirectedEdge>>) -> Dfs<'_> {
                Dfs { adj, visited: vec![false; adj.len()], max: 0 }
            }
            fn exec(&mut self, v: usize, sum: i64) {
                // 頂点 v から先の探索をする。v までの経路長は sum である。

                // 行きがけ
                self.visited[v] = true;
                self.max = i64::max(self.max, sum);

                for &edge in &self.adj[v] {
                    if !self.visited[edge.to] {
                        self.exec(edge.to, sum + edge.len);
                    }
                }
                // 帰りがけ
                self.visited[v] = false;
            }
        }
        let Problem { n_vertex, n_edge, edges } = self;

        // 隣接リスト
        let mut adj: Vec<Vec<DirectedEdge>> = vec![vec![]; *n_vertex];
        for &edge in edges {
            adj[edge.a].push(DirectedEdge { from: edge.a, to: edge.b, len: edge.len });
            adj[edge.b].push(DirectedEdge { from: edge.b, to: edge.a, len: edge.len });
        }

        let mut dfs = Dfs::new(&adj);
        for init in 0..*n_vertex {
            dfs.exec(init, 0);
        }

        let ans = dfs.max;

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
    Problem::read(ProconReader::new(stdin().lock())).solve5().print();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn check(input: &str, expected: Answer) {
        let actual = Problem::read(ProconReader::new(input.as_bytes())).solve();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_problem() {
        let _input = "
3
4
        "
        .trim();
        // check(_input, Answer { ans: 7 });
    }
}

// ====== snippet ======

use itertools::Itertools;
#[allow(unused_imports)]
use myio::*;
pub mod myio {
    use std::io::BufRead;

    pub trait IProconReader {
        fn read_line(&mut self) -> String;

        fn read_bytes(&mut self) -> Vec<u8> {
            self.read_line().as_bytes().to_vec()
        }

        fn read_any_1<T>(&mut self) -> T
        where
            T: std::str::FromStr,
            T::Err: std::fmt::Debug,
        {
            let buf = self.read_line();
            buf.parse::<T>().unwrap()
        }

        fn read_any_2<T0, T1>(&mut self) -> (T0, T1)
        where
            T0: std::str::FromStr,
            T0::Err: std::fmt::Debug,
            T1: std::str::FromStr,
            T1::Err: std::fmt::Debug,
        {
            let buf = self.read_line();
            let splitted = buf.trim().split(' ').collect::<Vec<_>>();
            let a0 = splitted[0].parse::<T0>().unwrap();
            let a1 = splitted[1].parse::<T1>().unwrap();
            (a0, a1)
        }

        fn read_any_3<T0, T1, T2>(&mut self) -> (T0, T1, T2)
        where
            T0: std::str::FromStr,
            T0::Err: std::fmt::Debug,
            T1: std::str::FromStr,
            T1::Err: std::fmt::Debug,
            T2: std::str::FromStr,
            T2::Err: std::fmt::Debug,
        {
            let buf = self.read_line();
            let splitted = buf.trim().split(' ').collect::<Vec<_>>();
            let a0 = splitted[0].parse::<T0>().unwrap();
            let a1 = splitted[1].parse::<T1>().unwrap();
            let a2 = splitted[2].parse::<T2>().unwrap();
            (a0, a1, a2)
        }

        fn read_any_4<T0, T1, T2, T3>(&mut self) -> (T0, T1, T2, T3)
        where
            T0: std::str::FromStr,
            T0::Err: std::fmt::Debug,
            T1: std::str::FromStr,
            T1::Err: std::fmt::Debug,
            T2: std::str::FromStr,
            T2::Err: std::fmt::Debug,
            T3: std::str::FromStr,
            T3::Err: std::fmt::Debug,
        {
            let buf = self.read_line();
            let splitted = buf.trim().split(' ').collect::<Vec<_>>();
            let a0 = splitted[0].parse::<T0>().unwrap();
            let a1 = splitted[1].parse::<T1>().unwrap();
            let a2 = splitted[2].parse::<T2>().unwrap();
            let a3 = splitted[3].parse::<T3>().unwrap();
            (a0, a1, a2, a3)
        }
        fn read_vec_any<T>(&mut self) -> Vec<T>
        where
            T: std::str::FromStr,
            T::Err: std::fmt::Debug,
        {
            let buf = self.read_line();
            buf.trim().split(' ').map(|s| s.parse::<T>().unwrap()).collect::<Vec<T>>()
        }

        fn read_vec_i64(&mut self) -> Vec<i64> {
            self.read_vec_any::<i64>()
        }

        fn read_vec_usize(&mut self) -> Vec<usize> {
            self.read_vec_any::<usize>()
        }

        fn read_vec_str(&mut self) -> Vec<String> {
            self.read_vec_any::<String>()
        }

        fn read_i64_1(&mut self) -> i64 {
            self.read_any_1::<i64>()
        }

        fn read_i64_2(&mut self) -> (i64, i64) {
            self.read_any_2::<i64, i64>()
        }

        fn read_i64_3(&mut self) -> (i64, i64, i64) {
            self.read_any_3::<i64, i64, i64>()
        }

        fn read_i64_4(&mut self) -> (i64, i64, i64, i64) {
            self.read_any_4::<i64, i64, i64, i64>()
        }

        fn read_usize_1(&mut self) -> usize {
            self.read_any_1::<usize>()
        }

        fn read_usize_2(&mut self) -> (usize, usize) {
            self.read_any_2::<usize, usize>()
        }

        fn read_usize_3(&mut self) -> (usize, usize, usize) {
            self.read_any_3::<usize, usize, usize>()
        }

        fn read_usize_4(&mut self) -> (usize, usize, usize, usize) {
            self.read_any_4::<usize, usize, usize, usize>()
        }
    }

    pub struct ProconReader<R: BufRead> {
        buf_read: R,
    }

    impl<R: BufRead> ProconReader<R> {
        pub fn new(buf_read: R) -> ProconReader<R> {
            ProconReader { buf_read }
        }
    }

    impl<R: BufRead> IProconReader for ProconReader<R> {
        fn read_line(&mut self) -> String {
            let mut buffer = String::new();
            self.buf_read.read_line(&mut buffer).unwrap();
            buffer.trim().to_string()
        }
    }
}
