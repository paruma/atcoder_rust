use crate::math::algebra::ab_group::ab_group::{AbGroup, AdditiveAbGroup};
use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use potentialized_dsu::*;", include = "ab_group")]
pub mod potentialized_dsu {
    use super::{AbGroup, AdditiveAbGroup};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum MergeResult {
        /// 新しくマージされた場合
        Merged { leader: usize, merged: usize },
        /// すでに同じ集合だった場合（変化なし）
        Unchanged,
        /// 矛盾があった場合
        Contradiction,
    }

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
    pub struct PotentializedDsuArbitrary<G: AbGroup>
    where
        G::S: PartialEq,
    {
        n: usize,
        nodes: Vec<Node>,
        // 親 のポテンシャル - 自分のポテンシャル
        p_diff: Vec<G::S>,
        cnt_groups: usize,
    }

    pub type PotentializedDsu = PotentializedDsuArbitrary<AdditiveAbGroup<i64>>;

    impl<G: AbGroup> PotentializedDsuArbitrary<G>
    where
        G::S: PartialEq,
    {
        pub fn new(size: usize) -> Self {
            Self {
                n: size,
                nodes: vec![Node::root(1); size],
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
            if self.nodes[ldst].size() < self.nodes[lsrc].size() {
                std::mem::swap(&mut lsrc, &mut ldst);
                std::mem::swap(&mut psrc, &mut pdst);
                diff = G::neg(&diff);
            }
            let size_lsrc = self.nodes[lsrc].size();
            let size_ldst = self.nodes[ldst].size();

            self.nodes[ldst] = Node::root(size_lsrc + size_ldst);
            self.nodes[lsrc] = Node::child(ldst);
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
            if self.nodes[a].is_root() {
                return (a, G::zero());
            }
            let parent = self.nodes[a].parent();
            let (leader, parent_potential) = self.leader_potential(parent);
            self.nodes[a] = Node::child(leader);

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

#[cfg(test)]
mod tests_potentialized_dsu {
    use super::potentialized_dsu::*;
    use crate::math::algebra::ab_group::ab_group::{AbGroup, AdditiveAbGroup};
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
        let mut pdsu = PotentializedDsuArbitrary::<Add>::new(8);

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
        let mut pdsu = PotentializedDsuArbitrary::<Add>::new(5);

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
        let mut uf = PotentializedDsuArbitrary::<Add>::new(5);

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

    #[test]
    fn test_size() {
        type Add = AdditiveAbGroup<i32>;
        let mut uf = PotentializedDsuArbitrary::<Add>::new(5);

        for i in 0..5 {
            assert_eq!(uf.size(i), 1);
        }

        uf.merge(0, 1, 10);
        assert_eq!(uf.size(0), 2);
        assert_eq!(uf.size(1), 2);
        assert_eq!(uf.size(2), 1);

        uf.merge(2, 3, 20);
        assert_eq!(uf.size(2), 2);
        assert_eq!(uf.size(3), 2);

        uf.merge(0, 2, 30);
        assert_eq!(uf.size(0), 4);
        assert_eq!(uf.size(1), 4);
        assert_eq!(uf.size(2), 4);
        assert_eq!(uf.size(3), 4);
        assert_eq!(uf.size(4), 1);

        // 既に同じグループの場合はサイズは変わらない
        uf.merge(1, 3, 40);
        assert_eq!(uf.size(0), 4);

        uf.merge(4, 0, 50);
        assert_eq!(uf.size(4), 5);
    }

    struct NaivePotentializedDsu<G: AbGroup> {
        groups: Vec<std::collections::BTreeMap<usize, G::S>>,
    }

    impl<G: AbGroup> NaivePotentializedDsu<G>
    where
        G::S: Clone + PartialEq + std::fmt::Debug,
    {
        fn new(n: usize) -> Self {
            let mut groups = Vec::new();
            for i in 0..n {
                let mut map = std::collections::BTreeMap::new();
                map.insert(i, G::zero());
                groups.push(map);
            }
            Self { groups }
        }

        fn find_group(&self, a: usize) -> usize {
            self.groups.iter().position(|g| g.contains_key(&a)).unwrap()
        }

        fn merge(&mut self, src: usize, dst: usize, diff: G::S) -> MergeResult {
            let i = self.find_group(src);
            let j = self.find_group(dst);
            if i == j {
                let g = &self.groups[i];
                let ps = g.get(&src).unwrap();
                let pd = g.get(&dst).unwrap();
                if G::sub(pd, ps) == diff {
                    MergeResult::Unchanged
                } else {
                    MergeResult::Contradiction
                }
            } else {
                let ps_old = self.groups[i].get(&src).unwrap().clone();
                let pd_old = self.groups[j].get(&dst).unwrap().clone();
                let offset = G::sub(&G::add(&ps_old, &diff), &pd_old);

                let mut g_j = self.groups.remove(j);
                let i = self.find_group(src);
                for v in g_j.values_mut() {
                    *v = G::add(v, &offset);
                }

                self.groups[i].extend(g_j);
                MergeResult::Merged {
                    leader: 0,
                    merged: 0,
                }
            }
        }

        fn diff(&self, src: usize, dst: usize) -> Option<G::S> {
            let i = self.find_group(src);
            let j = self.find_group(dst);
            if i == j {
                let g = &self.groups[i];
                Some(G::sub(g.get(&dst).unwrap(), g.get(&src).unwrap()))
            } else {
                None
            }
        }
    }

    #[test]
    #[ignore]
    fn test_random_potentialized() {
        use rand::prelude::*;
        let mut rng = StdRng::seed_from_u64(42);
        type Add = AdditiveAbGroup<i64>;

        for _ in 0..200 {
            let n = rng.random_range(1..=30);
            let mut dsu = PotentializedDsuArbitrary::<Add>::new(n);
            let mut naive = NaivePotentializedDsu::<Add>::new(n);

            for _ in 0..200 {
                let op = rng.random_range(0..6);
                match op {
                    0 => {
                        // merge
                        let src = rng.random_range(0..n);
                        let dst = rng.random_range(0..n);
                        let diff = rng.random_range(-100..100);
                        let res = dsu.merge(src, dst, diff);
                        let naive_res = naive.merge(src, dst, diff);
                        match (res, naive_res) {
                            (MergeResult::Merged { .. }, MergeResult::Merged { .. }) => {}
                            (MergeResult::Unchanged, MergeResult::Unchanged) => {}
                            (MergeResult::Contradiction, MergeResult::Contradiction) => {}
                            _ => panic!("merge result mismatch: {:?} vs {:?}", res, naive_res),
                        }
                    }
                    1 => {
                        // same
                        let a = rng.random_range(0..n);
                        let b = rng.random_range(0..n);
                        assert_eq!(dsu.same(a, b), naive.diff(a, b).is_some());
                    }
                    2 => {
                        // diff
                        let a = rng.random_range(0..n);
                        let b = rng.random_range(0..n);
                        assert_eq!(dsu.diff(a, b), naive.diff(a, b));
                    }
                    3 => {
                        // size
                        let a = rng.random_range(0..n);
                        assert_eq!(dsu.size(a), naive.groups[naive.find_group(a)].len());
                    }
                    4 => {
                        // count_group
                        assert_eq!(dsu.count_group(), naive.groups.len());
                    }
                    5 => {
                        // groups
                        let dg = sorted(dsu.groups());
                        let mut ng = naive
                            .groups
                            .iter()
                            .map(|g| g.keys().copied().collect_vec())
                            .map(|mut v| {
                                v.sort();
                                v
                            })
                            .collect_vec();
                        ng.sort();
                        assert_eq!(dg, ng);
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
