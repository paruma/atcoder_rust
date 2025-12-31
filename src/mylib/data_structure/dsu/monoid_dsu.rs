use crate::data_structure::dsu::dsu_core::dsu_core::DsuCore;
use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use monoid_dsu::*;", include = "dsu_core")]
pub mod monoid_dsu {
    use ac_library::Monoid;

    use super::DsuCore;

    #[derive(Clone, Debug)]
    pub struct MonoidDsu<M: Monoid> {
        dsu: DsuCore,
        prods: Vec<M::S>,
    }

    impl<M: Monoid> MonoidDsu<M> {
        pub fn new(data: &[M::S]) -> MonoidDsu<M> {
            let dsu = DsuCore::new(data.len());
            MonoidDsu {
                dsu,
                prods: data.to_vec(),
            }
        }

        pub fn merge(&mut self, a: usize, b: usize) -> Option<(usize, usize)> {
            let merge_result = self.dsu.merge(a, b);
            if let Some((leader, merged)) = merge_result {
                self.prods[leader] = M::binary_operation(&self.prods[leader], &self.prods[merged]);
                self.prods[merged] = M::identity();
            }
            merge_result
        }

        pub fn same(&mut self, a: usize, b: usize) -> bool {
            self.dsu.same(a, b)
        }

        pub fn leader(&mut self, a: usize) -> usize {
            self.dsu.leader(a)
        }

        pub fn size(&mut self, a: usize) -> usize {
            self.dsu.size(a)
        }

        pub fn groups(&mut self) -> Vec<Vec<usize>> {
            self.dsu.groups()
        }

        pub fn count_group(&self) -> usize {
            self.dsu.count_group()
        }

        pub fn prod(&mut self, a: usize) -> M::S {
            let leader = self.leader(a);
            self.prods[leader].clone()
        }

        /// 指定された要素 `a` が属するグループの現在の積に対して、新しい値 `d` を二項演算で適用する。
        ///
        /// この操作は、グループの積 `p` を `M::binary_operation(p, d)` で更新する。
        ///
        /// # 例
        /// `Additive` Monoid を使用している場合、グループの合計値に `d` を加算する。
        /// `Max` Monoid を使用している場合、グループの最大値を `max(current_max, d)` で更新する。
        pub fn apply(&mut self, a: usize, d: &M::S) {
            let leader = self.leader(a);
            self.prods[leader] = M::binary_operation(&self.prods[leader], d);
        }

        pub fn groups_with_prod(&mut self) -> Vec<(Vec<usize>, M::S)> {
            self.groups()
                .into_iter()
                .map(|x| {
                    let prod = self.prod(x[0]);
                    (x, prod)
                })
                .collect()
        }
    }
}

#[cfg(test)]
mod tests_monoid_dsu {
    use ac_library::{Additive, Max};
    use itertools::Itertools;

    fn sorted(xss: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        xss.iter()
            .map(|xs| xs.iter().copied().sorted().collect_vec())
            .sorted()
            .collect_vec()
    }
    #[test]
    fn test_dsu_functionality() {
        use super::monoid_dsu::*;
        type Add = Additive<i32>;
        let mut uf = MonoidDsu::<Add>::new(&[0; 8]);
        assert!(uf.merge(0, 1).is_some());
        assert!(uf.merge(3, 4).is_some());
        assert!(uf.merge(4, 5).is_some());
        assert!(uf.merge(4, 6).is_some());
        assert!(uf.merge(1, 4).is_some());
        assert!(uf.merge(1, 5).is_none()); // すでにつながっている

        assert!(uf.same(0, 4));
        assert!(!uf.same(2, 4));
        assert_eq!(
            sorted(uf.groups()),
            sorted(vec![vec![0, 1, 3, 4, 5, 6], vec![2], vec![7]])
        );
        assert_eq!(uf.count_group(), 3);
    }

    #[test]
    fn test_size() {
        use super::monoid_dsu::*;
        type Add = Additive<i32>;
        let mut uf = MonoidDsu::<Add>::new(&[0; 5]);

        for i in 0..5 {
            assert_eq!(uf.size(i), 1);
        }

        uf.merge(0, 1);
        assert_eq!(uf.size(0), 2);
        assert_eq!(uf.size(1), 2);

        uf.merge(2, 3);
        uf.merge(0, 2);
        assert_eq!(uf.size(0), 4);
        assert_eq!(uf.size(4), 1);
    }

    #[test]
    fn test_monoid_sum() {
        use super::monoid_dsu::*;
        type Add = Additive<i32>;
        let mut uf = MonoidDsu::<Add>::new(&[1, 2, 3, 4, 5, 6, 7, 8]);

        uf.merge(0, 1);
        assert_eq!(uf.prod(0), 3);
        assert_eq!(uf.prod(1), 3);

        uf.merge(2, 3);
        assert_eq!(uf.prod(2), 7);
        assert_eq!(uf.prod(3), 7);

        uf.merge(0, 2);
        assert_eq!(uf.prod(0), 10);
        assert_eq!(uf.prod(1), 10);
        assert_eq!(uf.prod(2), 10);
        assert_eq!(uf.prod(3), 10);

        assert_eq!(uf.prod(4), 5);
    }

    #[test]
    fn test_groups_with_prod() {
        use super::monoid_dsu::*;
        type Add = Additive<i32>;
        let mut uf = MonoidDsu::<Add>::new(&[1, 2, 3, 4, 5]);

        uf.merge(0, 1);
        uf.merge(2, 3);
        uf.merge(0, 2);

        let mut result = uf.groups_with_prod();
        // Sort by the first element of the group vector for stable testing
        result.sort_by_key(|(group, _)| group[0]);

        #[allow(clippy::useless_vec)]
        let expected = vec![(vec![0, 1, 2, 3], 10), (vec![4], 5)];

        assert_eq!(result.len(), expected.len());
        for (i, (group, prod)) in result.iter().enumerate() {
            let (expected_group, expected_prod) = &expected[i];
            assert_eq!(
                sorted(vec![group.clone()])[0],
                sorted(vec![expected_group.clone()])[0]
            );
            assert_eq!(*prod, *expected_prod);
        }
    }

    #[test]
    fn test_apply() {
        use super::monoid_dsu::*;
        type Add = Additive<i32>;
        let mut uf = MonoidDsu::<Add>::new(&[1, 2, 3, 4, 5]);

        uf.merge(0, 1); // group {0, 1}, prod = 3
        uf.merge(2, 3); // group {2, 3}, prod = 7

        // Apply to a group
        uf.apply(0, &10);
        assert_eq!(uf.prod(0), 13); // 3 + 10
        assert_eq!(uf.prod(1), 13);

        // Apply to another group
        uf.apply(3, &20);
        assert_eq!(uf.prod(2), 27); // 7 + 20
        assert_eq!(uf.prod(3), 27);

        // Apply to a single element group
        uf.apply(4, &30);
        assert_eq!(uf.prod(4), 35); // 5 + 30

        // Merge groups after apply
        uf.merge(0, 4);
        assert_eq!(uf.prod(0), 48); // 13 + 35
    }

    #[test]
    fn test_monoid_max() {
        use super::monoid_dsu::*;
        type M = Max<i32>;
        let mut uf = MonoidDsu::<M>::new(&[1, 2, 3, 4, 5, 6, 7, 8]);

        uf.merge(0, 1);
        assert_eq!(uf.prod(0), 2);

        uf.merge(2, 3);
        assert_eq!(uf.prod(2), 4);

        uf.merge(0, 2);
        assert_eq!(uf.prod(0), 4);

        uf.merge(5, 6);
        assert_eq!(uf.prod(5), 7);

        uf.merge(5, 7);
        assert_eq!(uf.prod(5), 8);

        uf.merge(0, 5);
        assert_eq!(uf.prod(0), 8);
    }
}
