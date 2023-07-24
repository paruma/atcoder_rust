#![allow(clippy::let_unit_value)]
use proconio::{input, marker::Usize1};

//------snippet------
use union_find::*;
pub mod union_find {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Root {
        count: i32,
        pub n_loops: i64,
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
            UnionFind {
                nodes: vec![
                    Node::Root {
                        root: Root {
                            count: 1,
                            n_loops: 0
                        }
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
                // RootAndIndexの値はパターンマッチで
                // let {root, index} = self.root_node(x);
                // と書いたほうがいいのかもしれないなぁ。
                let root_node = self.root_node(x);
                let root_index = root_node.index;

                self.nodes[root_index] = Node::Root {
                    root: Root {
                        count: root_node.root.count,
                        n_loops: root_node.root.n_loops + 1,
                    },
                };
                // これが抜けてた...
                // UnionFindのテンプレコードにはreturnがあるんだけど、コピーするときにreturnを何故か消してしまった。
                // 早期returnやめるか？？？
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
                        n_loops: x_root_node.root.n_loops + y_root_node.root.n_loops,
                    },
                }
            } else {
                self.nodes[y_root_index] = Node::NonRoot {
                    parent_index: x_root_index,
                };
                self.nodes[x_root_index] = Node::Root {
                    root: Root {
                        count: x_count + y_count,
                        n_loops: x_root_node.root.n_loops + y_root_node.root.n_loops,
                    },
                }
            }
        }
    }
}

//---
use num::{One, Zero};
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
        edge_info: [(Usize1, Usize1);n_e],
    }

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

    let mut ans = RF::one();
    for node in uf.nodes {
        match node {
            Node::Root { root } => {
                //dbg!(root.n_loops); //dbgあると間に合わない
                if root.n_loops == 1 {
                    ans *= RF::new(2);
                } else {
                    ans = RF::zero();
                }
            }
            Node::NonRoot { parent_index: _ } => {}
        }
    }
    // これ不要(何も考えずにコード修正してたときの名残)
    if ans == RF::one() {
        RF::zero()
    } else {
        ans
    }
}

//fn output() {}

fn main() {
    let (n_v, edges) = read();
    let ans = solve(n_v, &edges);

    println!("{}", ans.rep());
}
