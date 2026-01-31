#[allow(clippy::module_inception)]
/// 経路圧縮なしの DsuCore。マージテクにより各操作 O(log N)。
pub mod dsu_core_no_compress {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    /// DSU 内の各要素の状態（親のインデックスまたは集合のサイズ）を保持する構造体。
    ///
    /// メモリ効率（32ビット整数 1 つ分）を維持したまま、以下の 2 つの状態を表現します。
    ///
    /// 1. **Root (根)**:
    ///    - 値が負の場合、その要素は集合の代表元（リーダー）です。
    ///    - 値の絶対値 `|v|` は、その集合に属する要素の数（サイズ）を表します。
    ///    - 例: `-1` はサイズ 1 の集合の根、`-5` はサイズ 5 の集合の根。
    ///
    /// 2. **Child (子)**:
    ///    - 値が 0 以上の場合、その要素は他の要素を親に持っています。
    ///    - 値 `v` は、親要素のインデックスを表します。
    struct Node(i32);

    impl Node {
        fn root(size: usize) -> Self {
            Self(-(size as i32))
        }

        fn child(parent: usize) -> Self {
            Self(parent as i32)
        }

        fn is_root(&self) -> bool {
            self.0 < 0
        }

        fn parent(&self) -> usize {
            self.0 as usize
        }

        fn size(&self) -> usize {
            (-self.0) as usize
        }
    }

    #[derive(Clone, Debug)]
    pub struct DsuCoreNoCompress {
        n: usize,
        nodes: Vec<Node>,
        cnt_groups: usize,
    }

    impl DsuCoreNoCompress {
        pub fn new(size: usize) -> Self {
            Self {
                n: size,
                nodes: vec![Node::root(1); size],
                cnt_groups: size,
            }
        }

        /// 2 つの要素 `a` と `b` が属する集合を統合する
        ///
        /// # 戻り値
        /// - `Some((leader, merged))`:
        ///   - `leader` は統合後の集合の代表元（リーダー）
        ///   - `merged` は統合されて消える側の旧代表元
        /// - `None`:
        ///   - `a` と `b` がすでに同じ集合に属していた場合
        pub fn merge(&mut self, a: usize, b: usize) -> Option<(usize, usize)> {
            assert!(a < self.n);
            assert!(b < self.n);
            let (mut x, mut y) = (self.leader(a), self.leader(b));
            if x == y {
                return None;
            }
            // x のサイズ >= y のサイズ になるように swap する
            if self.nodes[x].size() < self.nodes[y].size() {
                std::mem::swap(&mut x, &mut y);
            }
            let size_x = self.nodes[x].size();
            let size_y = self.nodes[y].size();

            self.nodes[x] = Node::root(size_x + size_y);
            self.nodes[y] = Node::child(x);
            self.cnt_groups -= 1;
            Some((x, y))
        }

        pub fn same(&self, a: usize, b: usize) -> bool {
            assert!(a < self.n);
            assert!(b < self.n);
            self.leader(a) == self.leader(b)
        }

        pub fn leader(&self, a: usize) -> usize {
            assert!(a < self.n);
            let mut curr = a;
            while !self.nodes[curr].is_root() {
                curr = self.nodes[curr].parent();
            }
            curr
        }

        pub fn size(&self, a: usize) -> usize {
            assert!(a < self.n);
            let x = self.leader(a);
            self.nodes[x].size()
        }

        pub fn count_group(&self) -> usize {
            self.cnt_groups
        }

        pub fn groups(&self) -> Vec<Vec<usize>> {
            let mut leader_buf = vec![0; self.n];
            let mut group_size = vec![0; self.n];
            for i in 0..self.n {
                leader_buf[i] = self.leader(i);
                group_size[leader_buf[i]] += 1;
            }
            let mut result = vec![Vec::new(); self.n];
            for i in 0..self.n {
                result[i].reserve(group_size[i]);
            }
            for i in 0..self.n {
                result[leader_buf[i]].push(i);
            }
            result
                .into_iter()
                .filter(|x| !x.is_empty())
                .collect::<Vec<Vec<usize>>>()
        }
    }
}

pub use dsu_core_no_compress::*;

#[cfg(test)]
mod tests_dsu_core_no_compress {
    use itertools::Itertools;

    fn sorted(xss: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        xss.iter()
            .map(|xs| xs.iter().copied().sorted().collect_vec())
            .sorted()
            .collect_vec()
    }
    #[test]
    fn test_uf() {
        use super::dsu_core_no_compress::*;
        let mut uf = DsuCoreNoCompress::new(8);
        assert!(uf.merge(0, 1).is_some());
        assert!(uf.merge(3, 4).is_some());
        assert!(uf.merge(4, 5).is_some());
        assert!(uf.merge(4, 6).is_some());
        assert!(uf.merge(1, 4).is_some());
        assert!(uf.merge(1, 5).is_none()); // すでにつながっている

        /*
        |           [6]
        |            |
        |   [0]-[1]-[4]-[5]
        |            |
        |           [3]
        |   [2] [7]
         */
        assert!(uf.same(0, 4));
        assert!(!uf.same(2, 4));
        assert_eq!(
            sorted(uf.groups()),
            sorted(vec![vec![0, 1, 3, 4, 5, 6], vec![2], vec![7]])
        );
    }

    #[test]
    fn test_count_group() {
        use super::dsu_core_no_compress::*;
        let mut uf = DsuCoreNoCompress::new(5);

        assert_eq!(uf.count_group(), 5);

        uf.merge(0, 1); // {0, 1}, {2}, {3}, {4}
        assert_eq!(uf.count_group(), 4);

        uf.merge(2, 3); // {0, 1}, {2, 3}, {4}
        assert_eq!(uf.count_group(), 3);

        uf.merge(0, 2); // {0, 1, 2, 3}, {4}
        assert_eq!(uf.count_group(), 2);

        // Try merging already connected components
        assert_eq!(uf.merge(0, 1), None);
        assert_eq!(uf.count_group(), 2); // count should not change

        uf.merge(4, 0); // {0, 1, 2, 3, 4}
        assert_eq!(uf.count_group(), 1);

        // Merge all into one group
        let mut uf2 = DsuCoreNoCompress::new(10);
        assert_eq!(uf2.count_group(), 10);
        for i in 0..9 {
            uf2.merge(i, i + 1);
        }
        assert_eq!(uf2.count_group(), 1);
    }

    #[test]
    fn test_merge() {
        use super::dsu_core_no_compress::*;
        let mut uf = DsuCoreNoCompress::new(5);

        // Merge 0 and 1
        // {0}, {1}, {2}, {3}, {4}
        // ↓
        // {0, 1}, {2}, {3}, {4}
        let res1 = uf.merge(0, 1);
        assert!(res1.is_some());
        let (leader1, merged1) = res1.unwrap();
        // The new leader is either 0 or 1, the merged is the other
        assert!((leader1 == 0 && merged1 == 1) || (leader1 == 1 && merged1 == 0));
        assert_eq!(uf.leader(0), leader1);
        assert_eq!(uf.leader(1), leader1);

        // Merge 2 and 3
        // {0, 1}, {2}, {3}, {4}
        // ↓
        // {0, 1}, {2, 3}, {4}
        let res2 = uf.merge(2, 3);
        assert!(res2.is_some());
        let (leader2, merged2) = res2.unwrap();
        assert!((leader2 == 2 && merged2 == 3) || (leader2 == 3 && merged2 == 2));
        assert_eq!(uf.leader(2), leader2);
        assert_eq!(uf.leader(3), leader2);

        // Merge the two sets {0, 1} and {2, 3}
        // {0, 1}, {2, 3}, {4}
        // ↓
        // {0, 1, 2, 3}, {4}
        let old_leader1 = uf.leader(0); // This is leader1
        let old_leader2 = uf.leader(2); // This is leader2
        let res3 = uf.merge(0, 2);
        assert!(res3.is_some());
        let (leader3, merged3) = res3.unwrap();
        assert!(
            (leader3 == old_leader1 && merged3 == old_leader2)
                || (leader3 == old_leader2 && merged3 == old_leader1)
        );
        assert_eq!(uf.leader(0), leader3);
        assert_eq!(uf.leader(3), leader3);

        // Try merging already connected components
        // {0, 1, 2, 3}, {4} (no change)
        assert_eq!(uf.merge(0, 1), None);
        assert_eq!(uf.merge(2, 3), None);
        assert_eq!(uf.merge(0, 3), None);

        // Merge with a single element component
        // {0, 1, 2, 3}, {4}
        // ↓
        // {0, 1, 2, 3, 4}
        let old_leader3 = uf.leader(0);
        let old_leader4 = uf.leader(4); // this is 4
        let res4 = uf.merge(4, 1);
        assert!(res4.is_some());
        let (leader4, merged4) = res4.unwrap();
        assert!(
            (leader4 == old_leader3 && merged4 == old_leader4)
                || (leader4 == old_leader4 && merged4 == old_leader3)
        );
        assert_eq!(uf.leader(4), leader4);
        assert_eq!(uf.leader(0), leader4);
    }

    #[test]
    fn test_size() {
        use super::dsu_core_no_compress::*;
        let mut uf = DsuCoreNoCompress::new(5);

        // 各要素の初期サイズは 1
        for i in 0..5 {
            assert_eq!(uf.size(i), 1);
        }

        uf.merge(0, 1);
        assert_eq!(uf.size(0), 2);
        assert_eq!(uf.size(1), 2);
        assert_eq!(uf.size(2), 1);

        uf.merge(2, 3);
        assert_eq!(uf.size(2), 2);
        assert_eq!(uf.size(3), 2);

        uf.merge(0, 2);
        assert_eq!(uf.size(0), 4);
        assert_eq!(uf.size(1), 4);
        assert_eq!(uf.size(2), 4);
        assert_eq!(uf.size(3), 4);
        assert_eq!(uf.size(4), 1);

        // 既に同じグループの場合はサイズは変わらない
        uf.merge(1, 3);
        assert_eq!(uf.size(0), 4);

        uf.merge(4, 0);
        assert_eq!(uf.size(4), 5);
    }

    struct NaiveDsuCore {
        groups: Vec<std::collections::BTreeSet<usize>>,
    }

    impl NaiveDsuCore {
        fn new(n: usize) -> Self {
            Self {
                groups: (0..n)
                    .map(|i| {
                        let mut set = std::collections::BTreeSet::new();
                        set.insert(i);
                        set
                    })
                    .collect(),
            }
        }

        fn merge(&mut self, a: usize, b: usize) {
            let i = self.groups.iter().position(|g| g.contains(&a)).unwrap();
            let j = self.groups.iter().position(|g| g.contains(&b)).unwrap();
            if i != j {
                let g_j = self.groups.remove(j);
                let i = self.groups.iter().position(|g| g.contains(&a)).unwrap();
                self.groups[i].extend(g_j);
            }
        }

        fn same(&self, a: usize, b: usize) -> bool {
            self.groups.iter().any(|g| g.contains(&a) && g.contains(&b))
        }

        fn size(&self, a: usize) -> usize {
            self.groups.iter().find(|g| g.contains(&a)).unwrap().len()
        }

        fn count_group(&self) -> usize {
            self.groups.len()
        }

        fn groups(&self) -> Vec<Vec<usize>> {
            let mut res: Vec<Vec<usize>> = self
                .groups
                .iter()
                .map(|g| g.iter().copied().collect())
                .collect();
            res.sort();
            res
        }
    }

    #[test]
    #[ignore]
    fn test_random() {
        use rand::prelude::*;
        let mut rng = StdRng::seed_from_u64(42);

        for _ in 0..200 {
            let n = rng.random_range(1..=30);
            let mut dsu = super::dsu_core_no_compress::DsuCoreNoCompress::new(n);
            let mut naive = NaiveDsuCore::new(n);

            for i in 0..200 {
                let op = rng.random_range(0..5);
                match op {
                    0 => {
                        // merge
                        let a = rng.random_range(0..n);
                        let b = rng.random_range(0..n);
                        let old_leader_a = dsu.leader(a);
                        let old_leader_b = dsu.leader(b);
                        let res = dsu.merge(a, b);
                        naive.merge(a, b);
                        if let Some((leader, merged)) = res {
                            assert!(
                                (leader == old_leader_a && merged == old_leader_b)
                                    || (leader == old_leader_b && merged == old_leader_a),
                                "merge result mismatch at step {}: n={}, a={}, b={}",
                                i,
                                n,
                                a,
                                b
                            );
                            assert_eq!(dsu.leader(leader), leader);
                            assert_eq!(dsu.leader(merged), leader);
                        } else {
                            assert_eq!(old_leader_a, old_leader_b);
                        }
                    }
                    1 => {
                        // same
                        let a = rng.random_range(0..n);
                        let b = rng.random_range(0..n);
                        assert_eq!(
                            dsu.same(a, b),
                            naive.same(a, b),
                            "same mismatch at step {}: n={}, a={}, b={}",
                            i,
                            n,
                            a,
                            b
                        );
                    }
                    2 => {
                        // size
                        let a = rng.random_range(0..n);
                        assert_eq!(
                            dsu.size(a),
                            naive.size(a),
                            "size mismatch at step {}: n={}, a={}",
                            i,
                            n,
                            a
                        );
                    }
                    3 => {
                        // count_group
                        assert_eq!(
                            dsu.count_group(),
                            naive.count_group(),
                            "count_group mismatch at step {}: n={}",
                            i,
                            n
                        );
                    }
                    4 => {
                        // groups
                        assert_eq!(
                            sorted(dsu.groups()),
                            naive.groups(),
                            "groups mismatch at step {}: n={}",
                            i,
                            n
                        );
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
