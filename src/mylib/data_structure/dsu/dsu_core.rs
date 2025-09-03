use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use dsu_core::*;")]
/// ac_library::Dsu の merge のみ実装を変えたもの
pub mod dsu_core {
    pub struct DsuCore {
        n: usize,
        // root node: -1 * component size
        // otherwise: parent
        parent_or_size: Vec<i32>,
    }

    impl DsuCore {
        pub fn new(size: usize) -> Self {
            Self {
                n: size,
                parent_or_size: vec![-1; size],
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
        ///
        pub fn merge(&mut self, a: usize, b: usize) -> Option<(usize, usize)> {
            assert!(a < self.n);
            assert!(b < self.n);
            let (mut x, mut y) = (self.leader(a), self.leader(b));
            if x == y {
                return None;
            }
            if -self.parent_or_size[x] < -self.parent_or_size[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.parent_or_size[x] += self.parent_or_size[y];
            self.parent_or_size[y] = x as i32;
            Some((x, y))
        }

        pub fn same(&mut self, a: usize, b: usize) -> bool {
            assert!(a < self.n);
            assert!(b < self.n);
            self.leader(a) == self.leader(b)
        }

        pub fn leader(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            if self.parent_or_size[a] < 0 {
                return a;
            }
            self.parent_or_size[a] = self.leader(self.parent_or_size[a] as usize) as i32;
            self.parent_or_size[a] as usize
        }

        pub fn size(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            let x = self.leader(a);
            -self.parent_or_size[x] as usize
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
}
