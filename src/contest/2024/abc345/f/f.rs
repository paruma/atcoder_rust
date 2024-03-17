#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    u: Usize1,
    v: Usize1,
    idx: usize,
}
struct Problem {
    n_v: usize,
    n_e: usize,
    k: usize,
    edges: Vec<Edge>,
}
fn make_adj(n_vertex: usize, edges: &[Edge]) -> Vec<Vec<Edge>> {
    let mut adj = vec![vec![]; n_vertex];

    for &e in edges {
        adj[e.u].push(e);
        adj[e.v].push(e);
    }

    adj
}
impl Problem {
    fn read() -> Problem {
        input! {
            n_v: usize,
            n_e: usize,
            k: usize,
            edges: [(Usize1, Usize1); n_e],
        }
        let edges = edges
            .iter()
            .enumerate()
            .map(|(idx, (u, v))| Edge { u: *u, v: *v, idx })
            .collect_vec();

        Problem { n_v, n_e, k, edges }
    }
    fn solve(&self) -> Answer {
        if self.k % 2 == 1 {
            return Answer { ans: None };
        }
        if self.k == 0 {
            return Answer { ans: Some(vec![]) };
        }
        // 辺の繋がりをグラフにする
        let orig_adj = make_adj(self.n_v, &self.edges);
        let orig_degree = orig_adj.iter().map(|es| es.len()).collect_vec();

        // もとのグラフ

        // 辺のグラフ

        // 頂点0に触れている辺
        // 頂点1に触れている辺...

        let mut tmp = vec![HashSet::<usize>::new(); self.n_v];

        for &e in &self.edges {
            tmp[e.u].insert(e.idx);
            tmp[e.v].insert(e.idx);
        }

        let is_next = |ei1: usize, ei2: usize| {
            let e1 = self.edges[ei1];
            let e2 = self.edges[ei2];

            tmp[e1.u].contains(&e2.idx)
                || tmp[e1.v].contains(&e2.idx)
                || tmp[e2.u].contains(&e1.idx)
                || tmp[e2.v].contains(&e1.idx)
        };

        let mut edge_graph_adj = vec![vec![]; self.n_e];
        for ei in 0..self.n_e {
            let e = self.edges[ei];
            // orig_adj[e.u] と orig_adj[e.v] を合併（ただしei自身は除く）

            for &e in &orig_adj[e.u] {
                if e.idx != ei {
                    edge_graph_adj[ei].push(e.idx);
                }
            }
            for &e in &orig_adj[e.v] {
                if e.idx != ei {
                    edge_graph_adj[ei].push(e.idx);
                }
            }
        }

        let edge_graph_degree = |e: Edge| orig_degree[e.u] + orig_degree[e.v] - 1;

        let edge_graph_degree_list = (0..self.n_e)
            .map(|ei| {
                let e = self.edges[ei];
                edge_graph_degree(e)
            })
            .collect_vec();

        // let mut uf = UnionFind::new(self.n_e);

        let mut edge_list: Vec<usize> = vec![];
        let mut edge_set: HashSet<usize> = HashSet::new();

        for (ei, e) in self
            .edges
            .iter()
            .enumerate()
            .sorted_by_key(|(ei, e)| edge_graph_degree_list[*ei])
        {
            // 隣り合っているすべての辺がまだ入っていないか確認する
            if edge_graph_adj[ei]
                .iter()
                .all(|next_ei| !edge_set.contains(next_ei))
            {
                edge_list.push(ei);
                edge_set.insert(ei);
            }
            // もし k/2個以上になったら、終了

            if edge_list.len() == self.k / 2 {
                return Answer {
                    ans: Some(edge_list.clone()),
                };
            }
        }

        let ans = None;
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Option<Vec<usize>>,
}

impl Answer {
    fn print(&self) {
        match &self.ans {
            Some(ans) => {
                println!("Yes");
                println!("{}", ans.len());
                let ans = ans.iter().copied().map(|x| x + 1).collect_vec();
                print_vec_1line(&ans);
            }
            None => println!("No"),
        }
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

use std::collections::HashSet;

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
            UnionFind {
                nodes: vec![
                    Node::Root {
                        root: Root { count: 1 }
                    };
                    n
                ],
            }
        }
        fn root_node(&mut self, index: usize) -> RootAndIndex {
            match self.nodes[index] {
                Node::Root { root } => RootAndIndex { root, index },
                Node::NonRoot { parent_index } => {
                    let root_and_index = self.root_node(parent_index);
                    self.nodes[index] = Node::NonRoot {
                        parent_index: root_and_index.index,
                    };
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
            self.nodes
                .iter()
                .filter(|&node| matches!(node, Node::Root { .. }))
                .count()
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
                self.nodes[x_root_index] = Node::NonRoot {
                    parent_index: y_root_index,
                };
                self.nodes[y_root_index] = Node::Root {
                    root: Root {
                        count: x_count + y_count,
                    },
                }
            } else {
                self.nodes[y_root_index] = Node::NonRoot {
                    parent_index: x_root_index,
                };
                self.nodes[x_root_index] = Node::Root {
                    root: Root {
                        count: x_count + y_count,
                    },
                }
            }
        }
    }
}
