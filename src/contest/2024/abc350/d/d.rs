#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]

pub struct Edge {
    u: Usize1,
    v: Usize1,
}
impl Edge {
    pub fn new(u: usize, v: usize) -> Self {
        Self { u, v }
    }
}
pub fn make_adj(n_vertex: usize, edges: &[Edge]) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n_vertex];
    for &e in edges {
        adj[e.u].push(e.v);
    }
    adj
}

#[derive(Debug)]
struct Problem {
    nv: usize,
    ne: usize,
    edges: Vec<Edge>,
}

fn nc2(n: usize) -> usize {
    n * (n - 1) / 2
}

impl Problem {
    fn read() -> Problem {
        input! {
            nv: usize,
            ne: usize,
            edges: [Edge; ne]
        }
        Problem { nv, ne, edges }
    }
    fn solve(&self) -> Answer {
        let nv = self.nv;
        let mut uf: UnionFind = UnionFind::new(self.nv);

        for &e in &self.edges {
            uf.unite(e.u, e.v);
        }
        // 各グループの個数を求める

        let mut sum = 0;
        for v in 0..nv {
            if uf.root(v) == v {
                let cnt = uf.same_count(v);
                sum += nc2(cnt as usize);
            }
        }

        let ans = sum - self.ne;
        let ans = ans as i64;
        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        todo!();
        // let ans = 0;
        // Answer { ans }
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
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }

    fn check(p: &Problem) {
        assert_eq!(p.solve(), p.solve_naive());
    }

    fn make_random_problem() -> Problem {
        todo!()
        // let mut rng = SmallRng::from_entropy();
        // let n = rng.gen_range(1..=10);
        // let p = Problem { _a: n };
        // println!("{:?}", &p);
        // p
    }

    #[test]
    fn test_with_naive() {
        // 手動でテストを作るのもOK
        for _ in 0..100 {
            // let p = make_random_problem();
            // check(&p);
        }
    }
}

// ====== import ======
#[allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet};

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
