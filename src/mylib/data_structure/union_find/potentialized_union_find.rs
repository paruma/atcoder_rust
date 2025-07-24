use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use potentialized_union_find::*;")]
pub mod potentialized_union_find {
    use itertools::Itertools;

    #[derive(Clone, Debug)]
    struct RootInfo {
        count: usize,
    }

    #[derive(Clone, Debug)]
    struct NonRootInfo {
        parent: usize,
        /// 親のポテンシャル - 自分のポテンシャル
        potential_diff: i64,
    }

    #[derive(Clone, Debug)]
    enum Node {
        Root(RootInfo),
        NonRoot(NonRootInfo),
    }

    impl Node {
        fn root(count: usize) -> Node {
            Node::Root(RootInfo { count })
        }

        fn non_root(parent: usize, potential_diff: i64) -> Node {
            Node::NonRoot(NonRootInfo {
                parent,
                potential_diff,
            })
        }

        fn as_root(&self) -> &RootInfo {
            match self {
                Node::Root(info) => info,
                Node::NonRoot(_) => panic!(),
            }
        }
    }

    #[derive(Clone, Debug)]
    struct ToRoot {
        root_index: usize,
        /// root のポテンシャル - 自分のポテンシャル
        potential_diff: i64,
    }

    #[derive(Clone, Debug)]
    pub struct PotentializedUnionFind {
        nodes: Vec<Node>,
        cnt_groups: usize,
    }
    pub enum UnionResult {
        Consistent { updated: bool },
        Inconsistent,
    }

    impl UnionResult {
        pub fn updated(&self) -> bool {
            match self {
                UnionResult::Consistent { updated } => *updated,
                UnionResult::Inconsistent => false,
            }
        }
        pub fn is_consistent(&self) -> bool {
            matches!(self, UnionResult::Consistent { .. })
        }
        pub fn is_inconsistent(&self) -> bool {
            matches!(self, UnionResult::Inconsistent { .. })
        }
    }

    impl PotentializedUnionFind {
        pub fn new(n: usize) -> PotentializedUnionFind {
            PotentializedUnionFind {
                nodes: vec![Node::Root(RootInfo { count: 1 }); n],
                cnt_groups: n,
            }
        }

        fn root_node(&mut self, index: usize) -> ToRoot {
            match &self.nodes[index] {
                Node::Root(_) => ToRoot {
                    root_index: index,
                    potential_diff: 0,
                },
                Node::NonRoot(my_info) => {
                    let to_parent_potential_diff = my_info.potential_diff;
                    let parent_to_root = self.root_node(my_info.parent);

                    //       to_parent_potential_diff     parent_to_root.potential_diff
                    // 自分 -------------------------> 親 ------------------------------> root
                    let new_potential_diff =
                        to_parent_potential_diff + parent_to_root.potential_diff;

                    // 経路圧縮
                    self.nodes[index] =
                        Node::non_root(parent_to_root.root_index, new_potential_diff);
                    ToRoot {
                        root_index: parent_to_root.root_index,
                        potential_diff: new_potential_diff,
                    }
                }
            }
        }

        pub fn root(&mut self, index: usize) -> usize {
            self.root_node(index).root_index
        }

        pub fn same_count(&mut self, index: usize) -> usize {
            let root_index = self.root(index);
            self.nodes[root_index].as_root().count
        }

        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
        }

        pub fn num_groups(&self) -> usize {
            self.cnt_groups
        }

        /// diff = dst のポテンシャル - src のポテンシャル となるように統合する
        pub fn unite(&mut self, src: usize, dst: usize, diff: i64) -> UnionResult {
            if self.same(src, dst) {
                if self.diff(src, dst) == Some(diff) {
                    return UnionResult::Consistent { updated: false };
                } else {
                    return UnionResult::Inconsistent;
                }
            }

            self.cnt_groups -= 1;

            let src_root_node = self.root_node(src);
            let dst_root_node = self.root_node(dst);

            //                  root_diff
            //            src_root -→ dst_root
            //                ↑           ↑
            // src_root_node. |           | dst_root_node.
            // potential_diff |           | potential_diff
            //               src ------→ dst
            //                    diff

            let root_diff = -src_root_node.potential_diff + diff + dst_root_node.potential_diff;

            // src_root_node が小さくなるように、必要に応じて swap する
            let (src_root_node, dst_root_node, root_diff) = {
                let src_cnt = self.nodes[src_root_node.root_index].as_root().count;
                let dst_cnt = self.nodes[dst_root_node.root_index].as_root().count;

                if src_cnt <= dst_cnt {
                    (src_root_node, dst_root_node, root_diff)
                } else {
                    (dst_root_node, src_root_node, -root_diff)
                }
            };

            let count_sum = self.nodes[src_root_node.root_index].as_root().count
                + self.nodes[dst_root_node.root_index].as_root().count;

            // dst に src をくっつける
            self.nodes[src_root_node.root_index] =
                Node::non_root(dst_root_node.root_index, root_diff);
            self.nodes[dst_root_node.root_index] = Node::root(count_sum);

            UnionResult::Consistent { updated: true }
        }

        /// dst のポテンシャル - src のポテンシャル を求める
        pub fn diff(&mut self, src: usize, dst: usize) -> Option<i64> {
            //  root
            //   ↑     ↖
            //  src --> dst
            if self.same(src, dst) {
                Some(self.root_node(src).potential_diff - self.root_node(dst).potential_diff)
            } else {
                None
            }
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
mod tests_potentialized_union_find {
    use itertools::Itertools;

    fn sorted(xss: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        xss.iter()
            .map(|xs| xs.iter().copied().collect_vec())
            .sorted()
            .collect_vec()
    }
    #[test]
    fn test_uf() {
        use super::potentialized_union_find::*;
        let mut uf = PotentializedUnionFind::new(8);
        uf.unite(0, 1, 1);
        uf.unite(3, 4, 2);
        uf.unite(4, 5, 3);
        uf.unite(4, 6, 4);
        uf.unite(1, 4, 5);
        uf.unite(0, 5, 9);
        uf.unite(0, 5, 1000); // すでにつながっているので何も起こらない

        /*
        |                 [6]
        |                  ↑
        |                  |4
        |      1       5   |   3
        | [0] --→ [1] --→ [4] --→ [5]
        |                  ↑
        |                  |2
        |                  |
        |                 [3]
        | [2]    [7]
         */

        assert_eq!(uf.num_groups(), 3);
        assert!(uf.same(0, 4));
        assert!(!uf.same(2, 4));
        assert_eq!(
            sorted(uf.groups()),
            sorted(vec![vec![0, 1, 3, 4, 5, 6], vec![2], vec![7]])
        );
        assert_eq!(uf.diff(0, 4), Some(6));
        assert_eq!(uf.diff(4, 0), Some(-6));
        assert_eq!(uf.diff(6, 5), Some(-1));
        assert_eq!(uf.diff(0, 2), None);
    }

    #[test]
    fn test_uf_consistency() {
        use super::potentialized_union_find::*;
        let mut uf = PotentializedUnionFind::new(4);

        //    1   5
        // [0]→[1]→[2]    [3]

        let union_result1 = uf.unite(0, 1, 1);
        assert!(union_result1.is_consistent());
        assert!(!union_result1.is_inconsistent());
        assert!(union_result1.updated());

        uf.unite(1, 2, 5);

        let union_result2 = uf.unite(0, 2, 6);
        assert!(union_result2.is_consistent());
        assert!(!union_result2.is_inconsistent());
        assert!(!union_result2.updated());

        let union_result3 = uf.unite(0, 2, 1000);
        assert!(!union_result3.is_consistent());
        assert!(union_result3.is_inconsistent());
        assert!(!union_result3.updated());
    }
}
