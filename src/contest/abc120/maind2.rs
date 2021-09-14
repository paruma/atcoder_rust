use itertools::Itertools;
use proconio::input;
mod union_find {

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

            // 自分と同じグループのノードの数
            let x_count = x_root_node.root.count;
            let y_count = y_root_node.root.count;

            let x_root_index = x_root_node.index;
            let y_root_index = y_root_node.index;

            if x_count < y_count {
                // yのrootにxのrootをくっつける
                self.nodes[x_root_index] = Node::NonRoot {
                    parent_index: y_root_index,
                };
                self.nodes[y_root_index] = Node::Root {
                    root: Root {
                        count: x_count + y_count,
                    },
                }
            } else {
                // xのrootにyのrootをくっつける

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

mod scanl {
    #[derive(Clone)]
    pub struct Scanl<I, B, F> {
        iter: I,
        state: Option<B>,
        f: F,
    }

    impl<I, B, F> Scanl<I, B, F> {
        fn new(iter: I, init: B, f: F) -> Scanl<I, B, F> {
            Scanl {
                iter,
                state: Some(init),
                f,
            }
        }
    }

    impl<I, B, F> Iterator for Scanl<I, B, F>
    where
        B: Clone + Copy,
        I: Iterator,
        F: FnMut(&mut B, I::Item) -> B,
    {
        type Item = B;

        #[inline]
        fn next(&mut self) -> Option<B> {
            let retval = self.state?;
            let a_opt = self.iter.next();

            self.state = self
                .state
                .and_then(|mut s| a_opt.map(|a| (self.f)(&mut s, a)));

            Some(retval)
        }
    }

    pub trait IteratorExtScanLeft: Iterator + Sized {
        fn scanl<B, F>(self, init: B, f: F) -> Scanl<Self, B, F>
        where
            Self: Sized,
            F: FnMut(&mut B, Self::Item) -> B,
        {
            Scanl::new(self, init, f)
        }
    }

    impl<T: Iterator> IteratorExtScanLeft for T {}
}
//0:50~1:35

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Bridge {
    a: usize,
    b: usize,
}

fn read() -> (usize, usize, Vec<Bridge>) {
    input! {
        n_islands: usize,
        n_bridges: usize,
        bridge_info: [(usize, usize); n_bridges],
    }
    let bridges = bridge_info
        .iter()
        .map(|(a, b)| Bridge {
            a: *a - 1,
            b: *b - 1,
        })
        .collect::<Vec<_>>();

    (n_islands, n_bridges, bridges)
}

fn test<T: std::iter::DoubleEndedIterator>(a: T) {}

fn solve(n_islands: usize, n_bridges: usize, bridges: &[Bridge]) -> Vec<i64> {
    use scanl::*;
    use union_find::*;
    let mut uf = UnionFind::new(n_islands);

    let comb2 = |n: i64| n * (n - 1) / 2;

    let diffs = bridges.iter().rev().map(|&bridge| {
        if !uf.same(bridge.a, bridge.b) {
            let same_count_a: i64 = uf.same_count(bridge.a) as i64;
            let same_count_b: i64 = uf.same_count(bridge.b) as i64;
            let diff =
                comb2(same_count_a + same_count_b) - comb2(same_count_a) - comb2(same_count_b);
            uf.unite(bridge.a, bridge.b);

            diff
        } else {
            0
        }
    });

    let all_score = comb2(n_islands as i64);

    diffs
        .scanl(0_i64, |acc, x| *acc + x)
        .map(|score| all_score - score)
        //.take(n_bridges)
        .collect::<Vec<i64>>() // revが取れない。
        .into_iter()
        .dropping_back(1)
        .rev()
        .collect::<Vec<i64>>()
}

fn output(ans: &[i64]) {
    for &x in ans {
        println!("{}", x);
    }
}
fn main() {
    let (n_islands, n_bridges, bridges) = read();
    let ans = solve(n_islands, n_bridges, &bridges);
    output(&ans);
}
