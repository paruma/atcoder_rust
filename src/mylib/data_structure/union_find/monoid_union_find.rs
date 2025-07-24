use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use monoid_union_find::*;")]
/// 可換モノイドをのっけた Union Find
pub mod monoid_union_find {
    use ac_library::Monoid;
    use itertools::Itertools;

    #[derive(Clone, Debug)]
    struct RootInfo<S: Clone> {
        count: usize,
        prod: S,
    }

    #[derive(Clone, Debug)]
    struct NonRootInfo {
        parent: usize,
    }

    #[derive(Clone, Debug)]
    enum Node<S: Clone> {
        Root(RootInfo<S>),
        NonRoot(NonRootInfo),
    }

    impl<S: Clone> Node<S> {
        fn root(count: usize, prod: S) -> Node<S> {
            Node::Root(RootInfo { count, prod })
        }

        fn non_root(parent: usize) -> Node<S> {
            Node::NonRoot(NonRootInfo { parent })
        }

        fn as_root(&self) -> &RootInfo<S> {
            match self {
                Node::Root(info) => info,
                Node::NonRoot(_) => panic!(),
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct MonoidUnionFind<M: Monoid> {
        nodes: Vec<Node<M::S>>,
        cnt_groups: usize,
    }

    impl<M: Monoid> MonoidUnionFind<M> {
        pub fn new(data: &[M::S]) -> MonoidUnionFind<M> {
            let nodes = data.iter().map(|d| Node::root(1, d.clone())).collect_vec();
            MonoidUnionFind {
                nodes,
                cnt_groups: data.len(),
            }
        }

        pub fn root(&mut self, index: usize) -> usize {
            match &self.nodes[index] {
                Node::Root(_) => index,
                Node::NonRoot(info) => {
                    let root = self.root(info.parent);
                    // 経路圧縮
                    self.nodes[index] = Node::non_root(root);
                    root
                }
            }
        }

        //経路圧縮しないバージョン
        // fn root_index2(&self, index: usize) -> usize {
        //     match &self.nodes[index] {
        //         Node::Root(_) => index,
        //         Node::NonRoot(info) => self.root_index2(info.parent_index),
        //     }
        // }

        pub fn same_count(&mut self, index: usize) -> usize {
            let root_index = self.root(index);
            self.nodes[root_index].as_root().count
        }

        pub fn same_prod(&mut self, index: usize) -> M::S {
            let root_index = self.root(index);
            self.nodes[root_index].as_root().prod.clone()
        }

        pub fn same_prod_ref(&mut self, index: usize) -> &M::S {
            let root_index = self.root(index);
            &self.nodes[root_index].as_root().prod
        }

        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
        }

        pub fn num_groups(&self) -> usize {
            self.cnt_groups
        }

        pub fn unite(&mut self, x: usize, y: usize) -> bool {
            if self.same(x, y) {
                return false;
            }

            self.cnt_groups -= 1;

            let (smaller_root, larger_root) = {
                let x_root = self.root(x);
                let y_root = self.root(y);
                let x_count = self.nodes[x_root].as_root().count;
                let y_count = self.nodes[y_root].as_root().count;
                if x_count < y_count {
                    (x_root, y_root)
                } else {
                    (y_root, x_root)
                }
            };

            let smaller_root_info = self.nodes[smaller_root].as_root();
            let larger_root_info = self.nodes[larger_root].as_root();

            let count = smaller_root_info.count + larger_root_info.count;
            let prod = M::binary_operation(&smaller_root_info.prod, &larger_root_info.prod);

            // larger_root に smaller_root をくっつける
            self.nodes[smaller_root] = Node::non_root(larger_root);
            self.nodes[larger_root] = Node::root(count, prod);

            true
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
mod tests_monoid_union_find {
    use ac_library::Additive;
    use itertools::Itertools;

    fn sorted(xss: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        xss.iter()
            .map(|xs| xs.iter().copied().collect_vec())
            .sorted()
            .collect_vec()
    }
    #[test]
    fn test_uf() {
        use super::monoid_union_find::*;
        let n = 8;
        let data = (0..n).map(|i| i as i64).collect_vec();
        let mut uf = MonoidUnionFind::<Additive<i64>>::new(&data);
        assert!(uf.unite(0, 1));
        assert!(uf.unite(3, 4));
        assert!(uf.unite(4, 5));
        assert!(uf.unite(4, 6));
        assert!(uf.unite(1, 4));
        assert!(!uf.unite(1, 5)); // すでにつながっている

        /*
        |           [6]
        |            |
        |   [0]-[1]-[4]-[5]
        |            |
        |           [3]
        |   [2] [7]
         */
        assert_eq!(uf.num_groups(), 3);
        assert!(uf.same(0, 4));
        assert!(!uf.same(2, 4));

        assert_eq!(uf.same_prod(0), 19);
        assert_eq!(uf.same_prod(1), 19);
        assert_eq!(uf.same_prod(3), 19);
        assert_eq!(uf.same_prod(4), 19);
        assert_eq!(uf.same_prod(5), 19);
        assert_eq!(uf.same_prod(6), 19);

        assert_eq!(uf.same_prod(2), 2);
        assert_eq!(uf.same_prod(7), 7);

        assert_eq!(
            sorted(uf.groups()),
            sorted(vec![vec![0, 1, 3, 4, 5, 6], vec![2], vec![7]])
        );
    }
}
