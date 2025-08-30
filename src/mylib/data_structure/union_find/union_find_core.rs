use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use union_find_core::*;")]
/// ac_library::Dsu のラッパー
pub mod union_find_core {
    use ac_library::Dsu;
    pub struct UnionFindCore {
        uf: Dsu,
    }

    impl UnionFindCore {
        pub fn new(n: usize) -> UnionFindCore {
            UnionFindCore { uf: Dsu::new(n) }
        }

        pub fn root(&mut self, v: usize) -> usize {
            self.uf.leader(v)
        }

        pub fn same_count(&mut self, v: usize) -> usize {
            self.uf.size(v)
        }

        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.uf.same(x, y)
        }

        /// 2 つの要素 `x` と `y` が属する集合を統合します。
        ///
        /// # 戻り値
        /// - `Some((root, merged))`:
        ///   - `root` は統合後の集合の代表元（リーダー）
        ///   - `merged` は統合されて消える側の旧代表元
        /// - `None`:
        ///   - `x` と `y` がすでに同じ集合に属していた場合
        ///
        pub fn unite(&mut self, x: usize, y: usize) -> Option<(usize, usize)> {
            let rx = self.uf.leader(x);
            let ry = self.uf.leader(y);
            if rx == ry {
                return None;
            }

            let root = self.uf.merge(rx, ry);
            let merged = root ^ rx ^ ry; // rx と ry のうち root でない方

            Some((root, merged))
        }

        pub fn groups(&mut self) -> Vec<Vec<usize>> {
            self.uf.groups()
        }
    }
}

#[cfg(test)]
mod tests_union_find_core {
    use itertools::Itertools;

    fn sorted(xss: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        xss.iter()
            .map(|xs| xs.iter().copied().collect_vec())
            .sorted()
            .collect_vec()
    }
    #[test]
    fn test_uf() {
        use super::union_find_core::*;
        let mut uf = UnionFindCore::new(8);
        assert!(uf.unite(0, 1).is_some());
        assert!(uf.unite(3, 4).is_some());
        assert!(uf.unite(4, 5).is_some());
        assert!(uf.unite(4, 6).is_some());
        assert!(uf.unite(1, 4).is_some());
        assert!(uf.unite(1, 5).is_none()); // すでにつながっている

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
}
