use crate::mylib::data_structure::dsu::dsu_core::dsu_core::DsuCore;
use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use leader_tracking_dsu::*;", include = "dsu_core")]
pub mod leader_tracking_dsu {
    use std::collections::BTreeSet;

    use super::DsuCore;

    #[derive(Clone, Debug)]

    /// リーダーの集合を O(1) で取得できる DSU
    /// merge の償却計算量が O(log n) な点に注意
    pub struct LeaderTrackingDsu {
        dsu: DsuCore,
        leaders: BTreeSet<usize>,
    }

    impl LeaderTrackingDsu {
        pub fn new(size: usize) -> LeaderTrackingDsu {
            let dsu = DsuCore::new(size);
            let leaders = (0..size).collect();
            LeaderTrackingDsu { dsu, leaders }
        }

        /// 計算量: O(log N)
        pub fn merge(&mut self, a: usize, b: usize) -> Option<(usize, usize)> {
            let merge_result = self.dsu.merge(a, b);
            if let Some((_, merged)) = merge_result {
                self.leaders.remove(&merged);
            }

            merge_result
        }

        pub fn same(&mut self, a: usize, b: usize) -> bool {
            self.dsu.same(a, b)
        }

        pub fn leader(&mut self, a: usize) -> usize {
            self.dsu.leader(a)
        }

        pub fn leaders(&self) -> &BTreeSet<usize> {
            &self.leaders
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
    }
}

#[cfg(test)]
mod tests_leader_tracking_dsu {
    use itertools::Itertools;

    fn sorted2(xss: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        xss.iter()
            .map(|xs| xs.iter().copied().sorted().collect_vec())
            .sorted()
            .collect_vec()
    }
    #[test]
    fn test_uf() {
        use super::leader_tracking_dsu::*;
        let mut uf = LeaderTrackingDsu::new(8);
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

        assert_eq!(uf.leaders().len(), 3);
        assert!(uf.leaders().contains(&2));
        assert!(uf.leaders().contains(&7));
        {
            let l0 = uf.leader(0);
            let l1 = uf.leader(1);
            assert!(uf.leaders().contains(&l0));
            assert!(uf.leaders().contains(&l1));
        }

        assert_eq!(
            sorted2(uf.groups().clone()),
            sorted2(vec![vec![0, 1, 3, 4, 5, 6], vec![2], vec![7]])
        );
    }
}
