use crate::mylib::data_structure::dsu::dsu_core::dsu_core::DsuCore;
use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use enumerable_dsu::*;", include = "dsu_core")]
pub mod enumerable_dsu {
    use super::DsuCore;

    #[derive(Clone, Debug)]

    /// 指定の元を含むグループを償却 O(α(n)) で取得できる DSU。
    ///
    /// merge の償却計算量が O(log n) な点に注意
    pub struct EnumerableDsu {
        dsu: DsuCore,
        groups: Vec<Vec<usize>>,
    }

    impl EnumerableDsu {
        pub fn new(size: usize) -> EnumerableDsu {
            let dsu = DsuCore::new(size);
            let groups = (0..size).map(|i| vec![i]).collect();
            EnumerableDsu { dsu, groups }
        }

        pub fn merge(&mut self, a: usize, b: usize) -> Option<(usize, usize)> {
            let merge_result = self.dsu.merge(a, b);
            if let Some((leader, merged)) = merge_result {
                for x in std::mem::take(&mut self.groups[merged]) {
                    self.groups[leader].push(x);
                }
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

        pub fn group(&mut self, a: usize) -> &Vec<usize> {
            let leader = self.leader(a);
            &self.groups[leader]
        }
    }
}

#[cfg(test)]
mod tests_enumerable_dsu {
    use itertools::Itertools;

    fn sorted(mut xs: Vec<usize>) -> Vec<usize> {
        xs.sort();
        xs
    }
    fn sorted2(xss: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        xss.iter()
            .map(|xs| xs.iter().copied().sorted().collect_vec())
            .sorted()
            .collect_vec()
    }
    #[test]
    fn test_uf() {
        use super::enumerable_dsu::*;
        let mut uf = EnumerableDsu::new(8);
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
        assert_eq!(sorted(uf.group(0).clone()), vec![0, 1, 3, 4, 5, 6]);
        assert_eq!(sorted(uf.group(1).clone()), vec![0, 1, 3, 4, 5, 6]);
        assert_eq!(sorted(uf.group(2).clone()), vec![2]);
        assert_eq!(
            sorted2(uf.groups().clone()),
            sorted2(vec![vec![0, 1, 3, 4, 5, 6], vec![2], vec![7]])
        );
    }

    #[test]
    fn test_size() {
        use super::enumerable_dsu::*;
        let mut uf = EnumerableDsu::new(5);

        for i in 0..5 {
            assert_eq!(uf.size(i), 1);
        }

        uf.merge(0, 1);
        assert_eq!(uf.size(0), 2);
        assert_eq!(uf.size(1), 2);

        uf.merge(2, 3);
        uf.merge(0, 2);
        assert_eq!(uf.size(0), 4);
        assert_eq!(uf.size(3), 4);
        assert_eq!(uf.size(4), 1);
    }

    #[test]
    fn test_count_group() {
        use super::enumerable_dsu::*;
        let mut uf = EnumerableDsu::new(5);

        assert_eq!(uf.count_group(), 5);

        uf.merge(0, 1);
        assert_eq!(uf.count_group(), 4);

        uf.merge(2, 3);
        assert_eq!(uf.count_group(), 3);

        uf.merge(0, 2);
        assert_eq!(uf.count_group(), 2);

        // すでに同じグループの場合は減らない
        uf.merge(1, 3);
        assert_eq!(uf.count_group(), 2);
    }
}
