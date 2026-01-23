use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use dsu_core::*;")]
/// ac_library::Dsu の merge のみ実装を変えたもの
pub mod dsu_core {
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
    pub struct DsuCore {
        n: usize,
        nodes: Vec<Node>,
        cnt_groups: usize,
    }

    impl DsuCore {
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

        pub fn same(&mut self, a: usize, b: usize) -> bool {
            assert!(a < self.n);
            assert!(b < self.n);
            self.leader(a) == self.leader(b)
        }

        pub fn leader(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            if self.nodes[a].is_root() {
                return a;
            }
            let parent = self.nodes[a].parent();
            let new_parent = self.leader(parent);
            self.nodes[a] = Node::child(new_parent);
            new_parent
        }

        pub fn size(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            let x = self.leader(a);
            self.nodes[x].size()
        }

        pub fn count_group(&self) -> usize {
            self.cnt_groups
        }

        pub fn groups(&mut self) -> Vec<Vec<usize>> {
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

pub use dsu_core::*;

#[cfg(test)]
mod tests_dsu_core {
    use itertools::Itertools;

    fn sorted(xss: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        xss.iter()
            .map(|xs| xs.iter().copied().sorted().collect_vec())
            .sorted()
            .collect_vec()
    }
    #[test]
    fn test_uf() {
        use super::dsu_core::*;
        let mut uf = DsuCore::new(8);
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
        use super::dsu_core::*;
        let mut uf = DsuCore::new(5);

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
        let mut uf2 = DsuCore::new(10);
        assert_eq!(uf2.count_group(), 10);
        for i in 0..9 {
            uf2.merge(i, i + 1);
        }
        assert_eq!(uf2.count_group(), 1);
    }

    #[test]
    fn test_merge() {
        use super::dsu_core::*;
        let mut uf = DsuCore::new(5);

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
        // // {0, 1, 2, 3, 4}
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
        use super::dsu_core::*;
        let mut uf = DsuCore::new(5);

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
}
