use std::io::stdin;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Color(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    u: usize,
    v: usize,
}
struct Problem {
    nv: usize,
    ne: usize,
    vertex_to_color: Vec<Color>,
    edges: Vec<Edge>,
}

fn connected_component(nv: usize, edges: &[Edge]) -> i64 {
    let mut uf = UnionFind::new(nv);
    for &Edge { u, v } in edges {
        uf.unite(u, v);
    }
    uf.num_groups() as i64
}

impl Problem {
    fn read<R: IProconReader>(mut r: R) -> Problem {
        let (nv, ne) = r.read_usize_2();
        let vertex_to_color =
            r.read_vec_i64().iter().copied().map(|c| Color(c as usize - 1)).collect();
        let edges = (0..ne)
            .map(|_| {
                let (u, v) = r.read_usize_2();
                let u = u - 1;
                let v = v - 1;
                Edge { u, v }
            })
            .collect();
        Problem { nv, ne, vertex_to_color, edges }
    }
    fn solve(&self) -> Answer {
        // itertools がほしい
        // 頂点の番号の振り直しをする（色ごとに0から番号をふる）
        let mut color_to_cnt = vec![0; self.nv];

        // 古い番号→新しい番号
        let mut old_vertex_to_new = vec![0; self.nv];
        for (v, color) in self.vertex_to_color.iter().copied().enumerate() {
            old_vertex_to_new[v] = color_to_cnt[color.0];
            color_to_cnt[color.0] += 1;
        }

        // 辺を色で分ける。
        // let mut color_to_vertex_list = vec![vec![]; self.nv];
        // for (v, color) in self.vertex_to_color.iter().copied().enumerate() {
        //     color_to_vertex_list[color.0].push(v);
        // }
        //色ごとにグラフを作る
        let mut color_to_edge_list = vec![vec![]; self.nv];

        for &e in &self.edges {
            if self.vertex_to_color[e.u] == self.vertex_to_color[e.v] {
                let new_edge = Edge {
                    u: old_vertex_to_new[e.u],
                    v: old_vertex_to_new[e.v],
                };

                color_to_edge_list[self.vertex_to_color[e.u].0].push(new_edge);
            }
        }

        let ans = (0..self.nv)
            .map(|color| {
                // 連結成分数を求める
                let edge_list = &color_to_edge_list[color];
                let nv = color_to_cnt[color];

                let cnt = connected_component(nv, edge_list);
                if cnt == 0 || cnt == 1 {
                    0
                } else {
                    cnt - 1
                }
            })
            .sum::<i64>();

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
    Problem::read(ProconReader::new(stdin().lock())).solve().print();
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

use union_find::*;
pub mod union_find {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Root {
        count: i32,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Node {
        Root { root: Root },
        NonRoot { parent_index: usize },
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct RootAndIndex {
        root: Root,
        index: usize,
    }
    #[derive(Clone, Debug)]
    pub struct UnionFind {
        nodes: Vec<Node>,
    }
    impl UnionFind {
        pub fn new(n: usize) -> UnionFind {
            UnionFind { nodes: vec![Node::Root { root: Root { count: 1 } }; n] }
        }
        fn root_node(&mut self, index: usize) -> RootAndIndex {
            match self.nodes[index] {
                Node::Root { root } => RootAndIndex { root, index },
                Node::NonRoot { parent_index } => {
                    let root_and_index = self.root_node(parent_index);
                    self.nodes[index] = Node::NonRoot { parent_index: root_and_index.index };
                    root_and_index
                }
            }
        }
        pub fn root(&mut self, index: usize) -> usize {
            self.root_node(index).index
        }
        pub fn same_count(&mut self, index: usize) -> i32 {
            self.root_node(index).root.count
        }
        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
        }
        pub fn num_groups(&self) -> usize {
            self.nodes.iter().filter(|&node| matches!(node, Node::Root { .. })).count()
        }
        pub fn unite(&mut self, x: usize, y: usize) {
            if self.same(x, y) {
                return;
            }
            let x_root_node = self.root_node(x);
            let y_root_node = self.root_node(y);
            let x_count = x_root_node.root.count;
            let y_count = y_root_node.root.count;
            let x_root_index = x_root_node.index;
            let y_root_index = y_root_node.index;
            if x_count < y_count {
                self.nodes[x_root_index] = Node::NonRoot { parent_index: y_root_index };
                self.nodes[y_root_index] = Node::Root { root: Root { count: x_count + y_count } }
            } else {
                self.nodes[y_root_index] = Node::NonRoot { parent_index: x_root_index };
                self.nodes[x_root_index] = Node::Root { root: Root { count: x_count + y_count } }
            }
        }
    }
}
