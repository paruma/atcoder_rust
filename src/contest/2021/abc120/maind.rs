use proconio::input;
#[allow(dead_code)]
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

fn solve(n_islands: usize, n_bridges: usize, bridges: &[Bridge]) -> Vec<i64> {
    use union_find::*;
    let mut uf = UnionFind::new(n_islands);

    let mut ans_rev: Vec<i64> = Vec::new();
    let comb2 = |n: i64| n * (n - 1) / 2;
    let mut score = comb2(n_islands as i64);
    ans_rev.push(score);

    // scanチャンス
    for &bridge in bridges.iter().rev() {
        if !uf.same(bridge.a, bridge.b) {
            let same_count_a: i64 = uf.same_count(bridge.a) as i64;
            let same_count_b: i64 = uf.same_count(bridge.b) as i64;
            let score_sub =
                comb2(same_count_a + same_count_b) - comb2(same_count_a) - comb2(same_count_b);
            uf.unite(bridge.a, bridge.b);
            score -= score_sub;
        }
        ans_rev.push(score);
    }

    ans_rev
        .into_iter()
        .take(n_bridges) // 1つ除く
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
