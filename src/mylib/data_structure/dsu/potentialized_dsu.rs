use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use potentialized_dsu::*;")]
pub mod potentialized_dsu {
    use std::{
        convert::Infallible,
        iter::Sum,
        marker::PhantomData,
        ops::{Add, Neg},
    };

    /// 可換群 (Abelian Group)
    pub trait AbGroup {
        type S: Clone;
        fn zero() -> Self::S;
        fn add(a: &Self::S, b: &Self::S) -> Self::S;
        fn neg(a: &Self::S) -> Self::S;
        fn sub(a: &Self::S, b: &Self::S) -> Self::S {
            Self::add(a, &Self::neg(b))
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct AdditiveAbGroup<T>(Infallible, PhantomData<fn() -> T>);
    impl<T: Sum + Add<Output = T> + Neg<Output = T> + Copy> AbGroup for AdditiveAbGroup<T> {
        type S = T;
        fn zero() -> Self::S {
            std::iter::empty().sum()
        }
        fn add(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
        fn neg(a: &Self::S) -> Self::S {
            -(*a)
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct XorAbGroup(Infallible);

    impl AbGroup for XorAbGroup {
        type S = u64;
        fn zero() -> Self::S {
            0
        }
        fn add(a: &Self::S, b: &Self::S) -> Self::S {
            *a ^ *b
        }
        fn neg(a: &Self::S) -> Self::S {
            *a
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum MergeResult {
        /// 新しくマージされた場合
        Merged { leader: usize, merged: usize },
        /// すでに同じ集合だった場合（変化なし）
        Unchanged,
        /// 矛盾があった場合
        Contradiction,
    }

    #[derive(Clone, Debug)]
    pub struct PotentializedDsu<G: AbGroup>
    where
        G::S: PartialEq,
    {
        n: usize,
        // root node: -1 * component size
        // otherwise: parent
        parent_or_size: Vec<i32>,
        // 親 のポテンシャル - 自分のポテンシャル
        p_diff: Vec<G::S>,
        cnt_groups: usize,
    }

    impl<G: AbGroup> PotentializedDsu<G>
    where
        G::S: PartialEq,
    {
        pub fn new(size: usize) -> Self {
            Self {
                n: size,
                parent_or_size: vec![-1; size],
                p_diff: vec![G::zero(); size],
                cnt_groups: size,
            }
        }

        /// 2 つの要素 `src` と `dst` が属する集合を統合する。
        /// diff = dst のポテンシャル - src のポテンシャル となるように統合する
        pub fn merge(&mut self, src: usize, dst: usize, mut diff: G::S) -> MergeResult {
            assert!(src < self.n);
            assert!(dst < self.n);
            let (mut lsrc, mut psrc) = self.leader_potential(src);
            let (mut ldst, mut pdst) = self.leader_potential(dst);
            if lsrc == ldst {
                let result = if self.diff(src, dst).unwrap() == diff {
                    MergeResult::Unchanged
                } else {
                    MergeResult::Contradiction
                };
                return result;
            }
            // ldst のサイズが大きくなるように必要に応じて swap
            if -self.parent_or_size[ldst] < -self.parent_or_size[lsrc] {
                std::mem::swap(&mut lsrc, &mut ldst);
                std::mem::swap(&mut psrc, &mut pdst);
                diff = G::neg(&diff);
            }
            self.parent_or_size[ldst] += self.parent_or_size[lsrc];
            self.parent_or_size[lsrc] = ldst as i32;
            self.cnt_groups -= 1;

            //          ldiff
            //     lsrc -----→ ldst
            //       ↑           ↑
            //  psrc |           | pdst
            //       |           |
            //      src ------→ dst
            //           diff
            let ldiff = G::add(&G::neg(&psrc), &G::add(&diff, &pdst));

            self.p_diff[lsrc] = ldiff;

            MergeResult::Merged {
                leader: ldst,
                merged: lsrc,
            }
        }

        pub fn same(&mut self, a: usize, b: usize) -> bool {
            assert!(a < self.n);
            assert!(b < self.n);
            self.leader(a) == self.leader(b)
        }

        /// dst のポテンシャル - src のポテンシャル を求める
        pub fn diff(&mut self, src: usize, dst: usize) -> Option<G::S> {
            //  leader
            //   ↑     ↖
            //  src --> dst
            if self.same(src, dst) {
                let (_, psrc) = self.leader_potential(src);
                let (_, pdst) = self.leader_potential(dst);
                let diff = G::sub(&psrc, &pdst);
                Some(diff)
            } else {
                None
            }
        }

        // leader と (leader のポテンシャル - a のポテンシャル) を返す
        fn leader_potential(&mut self, a: usize) -> (usize, G::S) {
            assert!(a < self.n);
            // a 自身が leader
            if self.parent_or_size[a] < 0 {
                return (a, G::zero());
            }
            let parent = self.parent_or_size[a] as usize;
            let (leader, parent_potential) = self.leader_potential(parent);
            self.parent_or_size[a] = leader as i32;

            //           p_diff[a]          parent_potential
            // 自分(a) -----------> parent ----------------> leader
            let potential = G::add(&self.p_diff[a], &parent_potential);
            self.p_diff[a] = potential.clone();
            (leader, potential)
        }

        pub fn leader(&mut self, a: usize) -> usize {
            self.leader_potential(a).0
        }

        pub fn size(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            let x = self.leader(a);
            -self.parent_or_size[x] as usize
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

#[cfg(test)]
mod tests_potentialized_dsu {
    use super::potentialized_dsu::*;
    use itertools::Itertools;

    fn sorted(xss: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        xss.iter()
            .map(|xs| xs.iter().copied().sorted().collect_vec())
            .sorted()
            .collect_vec()
    }

    #[test]
    fn test_potentialized_dsu_functionality() {
        type Add = AdditiveAbGroup<i32>;
        let mut pdsu = PotentializedDsu::<Add>::new(8);

        // p[1] - p[0] = 1
        matches!(pdsu.merge(0, 1, 1), MergeResult::Merged { .. });
        // p[4] - p[3] = 2
        matches!(pdsu.merge(3, 4, 2), MergeResult::Merged { .. });
        // p[5] - p[4] = 3
        matches!(pdsu.merge(4, 5, 3), MergeResult::Merged { .. });
        // p[6] - p[4] = 4
        matches!(pdsu.merge(4, 6, 4), MergeResult::Merged { .. });
        // p[4] - p[1] = 5
        matches!(pdsu.merge(1, 4, 5), MergeResult::Merged { .. });

        // p[5] - p[1] = (p[5]-p[4]) + (p[4]-p[1]) = 3 + 5 = 8
        matches!(pdsu.merge(1, 5, 8), MergeResult::Unchanged);
        // p[5] - p[1] = 9 is a contradiction
        matches!(pdsu.merge(1, 5, 9), MergeResult::Contradiction);

        assert!(pdsu.same(0, 4));
        assert!(!pdsu.same(2, 4));
        assert_eq!(
            sorted(pdsu.groups()),
            sorted(vec![vec![0, 1, 3, 4, 5, 6], vec![2], vec![7]])
        );
        assert_eq!(pdsu.count_group(), 3);
    }

    #[test]
    fn test_diff() {
        type Add = AdditiveAbGroup<i32>;
        let mut pdsu = PotentializedDsu::<Add>::new(5);

        // p[1] - p[0] = 1
        pdsu.merge(0, 1, 1);
        assert_eq!(pdsu.diff(0, 1), Some(1));
        assert_eq!(pdsu.diff(1, 0), Some(-1));

        // p[3] - p[2] = 2
        pdsu.merge(2, 3, 2);
        assert_eq!(pdsu.diff(2, 3), Some(2));

        // p[2] - p[0] = 3
        // (p[2]-p[0]) = (p[2]-p[3]) + (p[3]-p[1]) + (p[1]-p[0])
        // p[3]-p[1] = (p[3]-p[2]) - (p[1]-p[2])
        // p[1] - p[2] = (p[1]-p[0]) - (p[2]-p[0]) = 1 - 3 = -2
        // p[3]-p[1] = 2 - (-2) = 4
        pdsu.merge(0, 2, 3);
        assert_eq!(pdsu.diff(0, 2), Some(3));
        assert_eq!(pdsu.diff(0, 3), Some(5)); // (p[3]-p[2]) + (p[2]-p[0]) = 2 + 3 = 5
        assert_eq!(pdsu.diff(1, 3), Some(4)); // (p[3]-p[0]) - (p[1]-p[0]) = 5 - 1 = 4

        // No path
        assert_eq!(pdsu.diff(0, 4), None);

        // Contradiction
        // p[1]-p[3] is 4, but try to merge with diff 5
        matches!(pdsu.merge(1, 3, 5), MergeResult::Contradiction);

        // Unchanged
        // p[1]-p[3] is 4, merge with diff 4
        matches!(pdsu.merge(1, 3, 4), MergeResult::Unchanged);
    }

    #[test]
    fn test_merge() {
        type Add = AdditiveAbGroup<i32>;
        let mut uf = PotentializedDsu::<Add>::new(5);

        // Merge 0 and 1
        // {0}, {1}, {2}, {3}, {4}
        // ↓
        // {0, 1}, {2}, {3}, {4}
        let res1 = uf.merge(0, 1, 0);
        assert!(matches!(res1, MergeResult::Merged { .. }));
        if let MergeResult::Merged {
            leader: leader1,
            merged: merged1,
        } = res1
        {
            // The new leader is either 0 or 1, the merged is the other
            assert!((leader1 == 0 && merged1 == 1) || (leader1 == 1 && merged1 == 0));
            assert_eq!(uf.leader(0), leader1);
            assert_eq!(uf.leader(1), leader1);
        }

        // Merge 2 and 3
        // {0, 1}, {2}, {3}, {4}
        // ↓
        // {0, 1}, {2, 3}, {4}
        let res2 = uf.merge(2, 3, 0);
        assert!(matches!(res2, MergeResult::Merged { .. }));
        if let MergeResult::Merged {
            leader: leader2,
            merged: merged2,
        } = res2
        {
            assert!((leader2 == 2 && merged2 == 3) || (leader2 == 3 && merged2 == 2));
            assert_eq!(uf.leader(2), leader2);
            assert_eq!(uf.leader(3), leader2);
        }

        // Merge the two sets {0, 1} and {2, 3}
        // {0, 1}, {2, 3}, {4}
        // ↓
        // {0, 1, 2, 3}, {4}
        let old_leader1 = uf.leader(0); // This is leader1
        let old_leader2 = uf.leader(2); // This is leader2
        let res3 = uf.merge(0, 2, 0);
        assert!(matches!(res3, MergeResult::Merged { .. }));
        if let MergeResult::Merged {
            leader: leader3,
            merged: merged3,
        } = res3
        {
            assert!(
                (leader3 == old_leader1 && merged3 == old_leader2)
                    || (leader3 == old_leader2 && merged3 == old_leader1)
            );
            assert_eq!(uf.leader(0), leader3);
            assert_eq!(uf.leader(3), leader3);
        }

        // Try merging already connected components
        // {0, 1, 2, 3}, {4} (no change)
        assert!(matches!(uf.merge(0, 1, 0), MergeResult::Unchanged));
        assert!(matches!(uf.merge(2, 3, 0), MergeResult::Unchanged));
        assert!(matches!(uf.merge(0, 3, 0), MergeResult::Unchanged));

        // Merge with a single element component
        // {0, 1, 2, 3}, {4}
        // ↓
        // // {0, 1, 2, 3, 4}
        let old_leader3 = uf.leader(0);
        let old_leader4 = uf.leader(4); // this is 4
        let res4 = uf.merge(4, 1, 0);
        assert!(matches!(res4, MergeResult::Merged { .. }));
        if let MergeResult::Merged {
            leader: leader4,
            merged: merged4,
        } = res4
        {
            assert!(
                (leader4 == old_leader3 && merged4 == old_leader4)
                    || (leader4 == old_leader4 && merged4 == old_leader3)
            );
            assert_eq!(uf.leader(4), leader4);
            assert_eq!(uf.leader(0), leader4);
        }
    }
}
