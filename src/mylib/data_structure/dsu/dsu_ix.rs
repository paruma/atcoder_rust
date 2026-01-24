use super::dsu_core::DsuCore;
use crate::data_structure::ix::{Bounds, Ix};
use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use dsu_ix::*;")]
pub mod dsu_ix {
    use super::DsuCore;
    use super::{Bounds, Ix};

    #[derive(Clone, Debug)]
    pub struct DsuIx<I: Ix> {
        dsu: DsuCore,
        bounds: Bounds<I>,
    }

    impl<I: Ix> DsuIx<I> {
        /// 指定された範囲の要素を管理する DSU を作成する
        ///
        /// # Arguments
        /// * `bounds` - 要素のインデックス範囲
        pub fn new(bounds: Bounds<I>) -> Self {
            let n = bounds.range_size();
            Self {
                dsu: DsuCore::new(n),
                bounds,
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
        pub fn merge(&mut self, a: I, b: I) -> Option<(I, I)> {
            let a_idx = self.bounds.to_index(a);
            let b_idx = self.bounds.to_index(b);
            let res = self.dsu.merge(a_idx, b_idx);
            res.map(|(l, m)| (self.bounds.from_index(l), self.bounds.from_index(m)))
        }

        /// 要素 `a` と `b` が同じ集合に属しているか判定する
        pub fn same(&mut self, a: I, b: I) -> bool {
            let a_idx = self.bounds.to_index(a);
            let b_idx = self.bounds.to_index(b);
            self.dsu.same(a_idx, b_idx)
        }

        /// 要素 `a` が属する集合の代表元（リーダー）を返す
        pub fn leader(&mut self, a: I) -> I {
            let a_idx = self.bounds.to_index(a);
            let l_idx = self.dsu.leader(a_idx);
            self.bounds.from_index(l_idx)
        }

        /// 要素 `a` が属する集合の要素数を返す
        pub fn size(&mut self, a: I) -> usize {
            let a_idx = self.bounds.to_index(a);
            self.dsu.size(a_idx)
        }

        /// 集合の総数を返す
        pub fn count_group(&self) -> usize {
            self.dsu.count_group()
        }

        /// すべての集合を、それぞれの要素のリストとして返す
        pub fn groups(&mut self) -> Vec<Vec<I>> {
            self.dsu
                .groups()
                .into_iter()
                .map(|group| {
                    group
                        .into_iter()
                        .map(|idx| self.bounds.from_index(idx))
                        .collect()
                })
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::dsu_ix::*;
    use crate::data_structure::ix::Bounds;

    #[test]
    fn test_dsu_ix() {
        // 2x2 grid
        let bounds = Bounds::new((0, 0), (1, 1));
        let mut dsu = DsuIx::new(bounds);

        // merge (0,0) and (0,1)
        assert!(dsu.merge((0, 0), (0, 1)).is_some());
        assert!(dsu.same((0, 0), (0, 1)));
        assert!(!dsu.same((0, 0), (1, 0)));

        assert_eq!(dsu.leader((0, 0)), dsu.leader((0, 1)));
        assert_ne!(dsu.leader((0, 0)), dsu.leader((1, 0)));

        assert_eq!(dsu.size((0, 0)), 2);
        assert_eq!(dsu.count_group(), 3); // {(0,0),(0,1)}, {(1,0)}, {(1,1)}

        // groups
        let mut groups = dsu.groups();
        for group in &mut groups {
            group.sort_by(|a, b| a.partial_cmp(b).unwrap());
        }
        groups.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            groups,
            vec![vec![(0, 0), (0, 1)], vec![(1, 0)], vec![(1, 1)]]
        );
    }
}
