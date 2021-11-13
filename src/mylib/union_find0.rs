use cargo_snippet::snippet;

#[snippet(prefix = "use union_find::*;")]
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
        pub fn root_node(&mut self, index: usize) -> RootAndIndex {
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

    #[test]
    fn test_uf_loop() {
        use super::union_find::*;
        let mut uf = UnionFind::new(8);

        assert_eq!(uf.root_node(0).root.n_loops, 0);
        uf.unite(0, 1);
        assert_eq!(uf.root_node(0).root.n_loops, 0);
        uf.unite(1, 2);
        assert_eq!(uf.root_node(0).root.n_loops, 0);
        uf.unite(2, 0);
        assert_eq!(uf.root_node(0).root.n_loops, 1);

        uf.unite(3, 4);
        uf.unite(4, 5);
        uf.unite(5, 3);

        uf.unite(1, 4);

        assert_eq!(uf.root_node(0).root.n_loops, 2);

        // ループの数ってちゃんと計算できてるのかな？
        /*
        -----
        | | |
        -----

         これってループの数そもそもいくつ？
        */
    }
}
