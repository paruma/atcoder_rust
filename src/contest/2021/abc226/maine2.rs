#![allow(clippy::let_unit_value)]
use std::ops::Mul;

use proconio::{input, marker::Usize1};

//------snippet------
use union_find::*;
pub mod union_find {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Root {
        pub count: i32,
        pub n_loops: i32,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Node {
        Root { root: Root },
        NonRoot { parent_index: usize },
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct RootAndIndex {
        pub root: Root,
        pub index: usize,
    }
    #[derive(Clone, Debug)]
    pub struct UnionFind {
        pub nodes: Vec<Node>,
    }
    impl UnionFind {
        pub fn new(n: usize) -> UnionFind {
            let init_node = Node::Root {
                root: Root {
                    count: 1,
                    n_loops: 0,
                },
            };
            UnionFind {
                nodes: vec![init_node; n],
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
                let root_info = self.root_node(x);

                self.nodes[root_info.index] = Node::Root {
                    root: Root {
                        count: root_info.root.count,
                        n_loops: root_info.root.n_loops + 1,
                    },
                };
                // これが抜けてた...
                // UnionFindのテンプレコードにはreturnがあるんだけど、コピーするときにreturnを何故か消してしまった。
                // 早期returnやめるか？？？
                return; //消さないように
            }

            let x_root_info = self.root_node(x);
            let y_root_info = self.root_node(y);

            let (parent_root_info, child_root_info) =
                if x_root_info.root.count < y_root_info.root.count {
                    (y_root_info, x_root_info)
                } else {
                    (x_root_info, y_root_info)
                };

            self.nodes[parent_root_info.index] = Node::Root {
                root: Root {
                    count: parent_root_info.root.count + child_root_info.root.count,
                    n_loops: parent_root_info.root.n_loops + child_root_info.root.n_loops,
                },
            };

            self.nodes[child_root_info.index] = Node::NonRoot {
                parent_index: parent_root_info.index,
            };
        }
    }
}

//---
use num::One;
use rf::*;
pub mod rf {
    pub const MOD: i64 = 998244353;
    #[allow()]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct RF {
        rep: i64,
    }
    impl RF {
        pub fn new(x: i64) -> RF {
            RF {
                rep: x.rem_euclid(MOD),
            }
        }
        pub fn rep(self) -> i64 {
            self.rep
        }
    }
    impl RF {
        pub fn inv(self) -> Self {
            num::pow(self, (MOD - 2) as usize)
        }
    }
    impl num_traits::Zero for RF {
        fn zero() -> Self {
            RF::new(0)
        }
        fn is_zero(&self) -> bool {
            self.rep == 0
        }
    }
    impl num_traits::One for RF {
        fn one() -> Self {
            RF::new(1)
        }
    }
    macro_rules ! bi_ops_impl {($ std_ops : ident , $ fn : ident , $ op : tt ) => {impl std :: ops ::$ std_ops for RF {type Output = Self ; fn $ fn (self , rhs : Self ) -> Self :: Output {RF :: new (self . rep $ op rhs . rep ) } } } ; }
    bi_ops_impl ! (Add , add , + );
    bi_ops_impl ! (Sub , sub , - );
    bi_ops_impl ! (Mul , mul , * );
    impl std::ops::Div for RF {
        type Output = Self;
        fn div(self, rhs: Self) -> Self::Output {
            std::ops::Mul::mul(self, rhs.inv())
        }
    }
    macro_rules ! bi_ops_assign_impl {($ std_ops_assign : ident , $ fn_assign : ident , $ op : tt ) => {impl std :: ops ::$ std_ops_assign for RF {fn $ fn_assign (& mut self , rhs : Self ) {* self = * self $ op rhs } } } ; }
    bi_ops_assign_impl ! (AddAssign , add_assign , + );
    bi_ops_assign_impl ! (SubAssign , sub_assign , - );
    bi_ops_assign_impl ! (MulAssign , mul_assign , * );
    bi_ops_assign_impl ! (DivAssign , div_assign , / );
    impl std::ops::Neg for RF {
        type Output = Self;
        fn neg(self) -> Self::Output {
            RF::new(-self.rep)
        }
    }
}

//-------------------

struct Edge {
    v1: usize,
    v2: usize,
}

fn read() -> (usize, Vec<Edge>) {
    input! {
        n_v:usize,
        n_e:usize,
        edge_info: [(Usize1, Usize1); n_e],
    }

    // ここらへんテンプレ化できると嬉しいなぁ。
    let edges = edge_info
        .iter()
        .map(|(a, b)| Edge { v1: *a, v2: *b })
        .collect::<Vec<_>>();

    (n_v, edges)
}

fn solve(n_v: usize, edges: &[Edge]) -> RF {
    let mut uf = UnionFind::new(n_v);

    for edge in edges {
        uf.unite(edge.v1, edge.v2)
    }

    uf.nodes
        .iter()
        .filter_map(|node| match node {
            Node::Root { root } => Some(root),
            Node::NonRoot { .. } => None,
        }) // 連結成分の抽出 (ここまでをuf.roots()で呼び出せてもいいかも)
        .map(|root| {
            if root.n_loops == 1 {
                RF::new(2)
            } else {
                RF::new(0)
            }
        })
        .fold(RF::one(), Mul::mul) // product
}

//fn output() {}

fn main() {
    let (n_v, edges) = read();
    let ans = solve(n_v, &edges);

    println!("{}", ans.rep());
}
