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

#[cfg(test)]
mod tests {
    #[test]
    fn test_uf() {
        use super::union_find::*;
        let mut uf = UnionFind::new(8);
        uf.unite(0, 1);
        uf.unite(3, 4);
        uf.unite(4, 5);
        uf.unite(4, 6);
        uf.unite(1, 4);

        // {0,1,3,4,5,6}, {2}, {7}

        assert_eq!(uf.num_groups(), 3);
        assert!(uf.same(0, 4));
        assert!(!uf.same(2, 4));
    }
}
