use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use enumerable_union_find::*;")]
pub mod enumerable_union_find {
    use itertools::Itertools;

    #[derive(Clone, Debug)]
    struct Node {
        root: usize,
        group: Vec<usize>, // 子ノードの場合は無視する
    }

    #[derive(Clone, Debug)]
    pub struct UnionFind {
        nodes: Vec<Node>,
        cnt_groups: usize,
    }

    impl UnionFind {
        pub fn new(n: usize) -> UnionFind {
            let nodes = (0..n)
                .map(|i| Node {
                    root: i,
                    group: vec![i],
                })
                .collect_vec();
            UnionFind {
                nodes,
                cnt_groups: n,
            }
        }

        fn root(&self, index: usize) -> usize {
            self.nodes[index].root
        }

        pub fn same_count(&mut self, index: usize) -> usize {
            let root_index = self.root(index);
            self.nodes[root_index].group.len()
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
                let x_count = self.nodes[x_root].group.len();
                let y_count = self.nodes[y_root].group.len();
                if x_count < y_count {
                    (x_root, y_root)
                } else {
                    (y_root, x_root)
                }
            };

            for x in std::mem::take(&mut self.nodes[smaller_root].group) {
                self.nodes[larger_root].group.push(x);
                self.nodes[x].root = larger_root;
            }
            self.nodes[smaller_root].root = larger_root;

            true
        }

        pub fn group(&mut self, x: usize) -> &Vec<usize> {
            &self.nodes[self.root(x)].group
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
mod tests_enumerable_union_find {
    use itertools::Itertools;

    fn sorted_2d(xss: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        xss.iter()
            .map(|xs| xs.iter().copied().collect_vec())
            .sorted()
            .collect_vec()
    }

    fn sorted(xs: Vec<usize>) -> Vec<usize> {
        xs.iter().copied().sorted().collect_vec()
    }
    #[test]
    fn test_uf() {
        use super::enumerable_union_find::*;
        let mut uf = UnionFind::new(8);
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
        assert_eq!(
            sorted_2d(uf.groups()),
            sorted_2d(vec![vec![0, 1, 3, 4, 5, 6], vec![2], vec![7]])
        );
        assert_eq!(sorted(uf.group(0).clone()), vec![0, 1, 3, 4, 5, 6]);
        assert_eq!(sorted(uf.group(1).clone()), vec![0, 1, 3, 4, 5, 6]);
        assert_eq!(sorted(uf.group(2).clone()), vec![2]);
        assert_eq!(sorted(uf.group(3).clone()), vec![0, 1, 3, 4, 5, 6]);
        assert_eq!(sorted(uf.group(4).clone()), vec![0, 1, 3, 4, 5, 6]);
        assert_eq!(sorted(uf.group(5).clone()), vec![0, 1, 3, 4, 5, 6]);
        assert_eq!(sorted(uf.group(6).clone()), vec![0, 1, 3, 4, 5, 6]);
        assert_eq!(sorted(uf.group(7).clone()), vec![7]);
    }
}
