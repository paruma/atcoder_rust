#![allow(clippy::let_unit_value)]
use proconio::input;
use union_find::*;

pub mod union_find {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Root {
        count: i32,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Node {
        Root { root: Root },
        NonRoot { parent_index: usize },
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct RootAndIndex {
        root: Root,
        index: usize,
    }
    #[derive(Clone, Debug)]
    pub struct UnionFind {
        pub nodes: Vec<Node>,
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
                .filter(|&node| matches ! (node , Node :: Root {.. } ))
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

// 7:26~
fn read() -> (i64, Vec<i64>) {
    input! {n:i64, seq: [i64;n]}
    (n, seq)
}

fn solve(n: i64, seq: &[i64]) -> i64 {
    let n = n as usize;
    let max = *seq.iter().max().unwrap() as usize;
    let mut uf = UnionFind::new(max + 1);

    for i in 0..n / 2 {
        let small_i = i;
        let big_i = n - 1 - i;

        uf.unite(seq[small_i] as usize, seq[big_i] as usize);
    }

    /*
        uf.nodes
            .iter()
            .filter(|&node| matches!(node, Node::NonRoot{..}))
            .count() as i64;
    */
    ((max + 1) - uf.num_groups()) as i64
}

//fn output() {}

fn main() {
    let (n, seq) = read();
    let ans = solve(n, &seq);
    //output();
    println!("{}", ans);
}

#[test]
fn make_rand_input() {
    use rand::rngs::SmallRng;
    use rand::Rng;
    use rand::SeedableRng;

    let mut rng = SmallRng::from_entropy();
    for _ in 0..10 {
        println!("{}", rng.gen_range(1, 6));
    }
}
