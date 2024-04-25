use cargo_snippet::snippet;

#[snippet(prefix = "use simple_union_find::*;")]
pub mod simple_union_find {
    use itertools::Itertools;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct RootInfo {
        count: usize,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct NonRootInfo {
        parent_index: usize,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Node {
        Root(RootInfo),
        NonRoot(NonRootInfo),
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct RootAndIndex {
        info: RootInfo,
        index: usize,
    }

    #[derive(Clone, Debug)]
    pub struct UnionFind {
        nodes: Vec<Node>,
        cnt_groups: usize,
    }

    impl UnionFind {
        pub fn new(n: usize) -> UnionFind {
            UnionFind {
                nodes: vec![Node::Root(RootInfo { count: 1 }); n],
                cnt_groups: n,
            }
        }

        fn root_node(&mut self, index: usize) -> RootAndIndex {
            match self.nodes[index] {
                Node::Root(info) => RootAndIndex { info, index },
                Node::NonRoot(info) => {
                    let root_and_index = self.root_node(info.parent_index);
                    // 経路圧縮
                    self.nodes[index] = Node::NonRoot(NonRootInfo {
                        parent_index: root_and_index.index,
                    });
                    root_and_index
                }
            }
        }
        // 経路圧縮しないバージョン
        // fn root_node(&self, index: usize) -> RootAndIndex {
        //     match self.nodes[index] {
        //         Node::Root(info) => RootAndIndex { info, index },
        //         Node::NonRoot(info) => self.root_node(info.parent_index),
        //     }
        // }

        pub fn root(&mut self, index: usize) -> usize {
            self.root_node(index).index
        }

        pub fn same_count(&mut self, index: usize) -> usize {
            self.root_node(index).info.count
        }

        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
        }

        pub fn num_groups(&self) -> usize {
            self.cnt_groups
        }

        pub fn unite(&mut self, x: usize, y: usize) {
            if self.same(x, y) {
                return;
            }

            self.cnt_groups -= 1;

            let x_root_node = self.root_node(x);
            let y_root_node = self.root_node(y);

            let (smaller_root, larger_root) = if x_root_node.info.count < y_root_node.info.count {
                (x_root_node, y_root_node)
            } else {
                (y_root_node, x_root_node)
            };

            // larger_root に smaller_root をくっつける
            self.nodes[smaller_root.index] = Node::NonRoot(NonRootInfo {
                parent_index: larger_root.index,
            });
            self.nodes[larger_root.index] = Node::Root(RootInfo {
                count: smaller_root.info.count + larger_root.info.count,
            });
        }

        pub fn groups(&mut self) -> Vec<Vec<usize>> {
            let n = self.nodes.len();

            let roots = (0..n).map(|i| self.root(i)).collect_vec();

            let group_size = (0..n).map(|i| roots[i]).fold(vec![0; n], |mut acc, x| {
                acc[x] += 1;
                acc
            });

            let result = {
                let mut result = vec![Vec::new(); n];
                for i in 0..n {
                    result[i].reserve(group_size[i]);
                }
                for i in 0..n {
                    result[roots[i]].push(i);
                }
                result
            };
            result.into_iter().filter(|x| !x.is_empty()).collect_vec()
        }
    }
}

#[cfg(test)]
mod tests_simple_union_find {
    use itertools::Itertools;

    fn sorted(xss: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        xss.iter()
            .map(|xs| xs.iter().copied().collect_vec())
            .sorted()
            .collect_vec()
    }
    #[test]
    fn test_uf() {
        use super::simple_union_find::*;
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
        assert_eq!(
            sorted(uf.groups()),
            sorted(vec![vec![0, 1, 3, 4, 5, 6], vec![2], vec![7]])
        );
    }
}
