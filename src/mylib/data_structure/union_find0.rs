use cargo_snippet::snippet;

#[snippet(prefix = "use simple_union_find::*;")]
pub mod simple_union_find {
    use itertools::Itertools;

    #[derive(Clone, Debug)]
    struct RootInfo {
        count: usize,
    }

    #[derive(Clone, Debug)]
    struct NonRootInfo {
        parent: usize,
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

        fn non_root(parent: usize) -> Node {
            Node::NonRoot(NonRootInfo { parent })
        }

        fn as_root(&self) -> &RootInfo {
            match self {
                Node::Root(info) => info,
                Node::NonRoot(_) => panic!(),
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct UnionFind {
        nodes: Vec<Node>,
        cnt_groups: usize,
    }

    impl UnionFind {
        pub fn new(n: usize) -> UnionFind {
            let nodes = (0..n).map(|_| Node::root(1)).collect_vec();
            UnionFind {
                nodes,
                cnt_groups: n,
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

            let count_sum =
                self.nodes[smaller_root].as_root().count + self.nodes[larger_root].as_root().count;

            // larger_root に smaller_root をくっつける
            self.nodes[smaller_root] = Node::non_root(larger_root);
            self.nodes[larger_root] = Node::root(count_sum);

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

pub mod grid_union_find {
    use cargo_snippet::snippet;
    use itertools::Itertools;

    use super::super::super::pos0::pos::Pos;

    use super::super::union_find0::simple_union_find::UnionFind;
    #[snippet(name = "GridUnionFind")]
    pub struct GridUnionFind {
        pub uf: UnionFind,
        pub h: usize,
        pub w: usize,
    }

    #[snippet(name = "GridUnionFind")]
    impl GridUnionFind {
        pub fn new(h: usize, w: usize) -> GridUnionFind {
            GridUnionFind {
                uf: UnionFind::new(h * w),
                h,
                w,
            }
        }

        pub fn encode(&self, pos: Pos) -> usize {
            (pos.y * self.w as i64 + pos.x) as usize
        }

        pub fn decode(&self, i: usize) -> Pos {
            let y = (i / self.w) as i64;
            let x = (i % self.w) as i64;
            Pos::new(x, y)
        }

        pub fn same_count(&mut self, pos: Pos) -> usize {
            self.uf.same_count(self.encode(pos))
        }

        pub fn same(&mut self, pos1: Pos, pos2: Pos) -> bool {
            self.uf.same(self.encode(pos1), self.encode(pos2))
        }

        pub fn num_groups(&self) -> usize {
            self.uf.num_groups()
        }

        pub fn unite(&mut self, pos1: Pos, pos2: Pos) {
            self.uf.unite(self.encode(pos1), self.encode(pos2));
        }

        pub fn groups(&mut self) -> Vec<Vec<Pos>> {
            self.uf
                .groups()
                .into_iter()
                .map(|group| group.iter().copied().map(|i| self.decode(i)).collect_vec())
                .collect_vec()
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
            sorted(uf.groups()),
            sorted(vec![vec![0, 1, 3, 4, 5, 6], vec![2], vec![7]])
        );
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
