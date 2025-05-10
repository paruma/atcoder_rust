use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use lca::*;")]
pub mod lca {
    use std::convert::Infallible;

    use ac_library::{Monoid, Segtree};

    pub struct MinI64Usize(Infallible);
    impl Monoid for MinI64Usize {
        type S = (i64, usize);
        fn identity() -> Self::S {
            (i64::MAX, usize::MAX)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            std::cmp::min(*a, *b)
        }
    }

    pub struct Lca {
        dist: Vec<i64>,                        // dist[v]: ルートから v までの距離
        euler_tour_dist: Segtree<MinI64Usize>, // 根からの距離を euler tour の順に並べたもの
        euler_tour_in_time: Vec<usize>,
        #[allow(dead_code)]
        euler_tour_out_time: Vec<usize>,
    }

    impl Lca {
        /// tree_parent[i]: i の 親 を表す。根の場合は tree_parent[i] == i
        /// 計算量: O(nv log(nv)) (nv は頂点の数とする)
        pub fn new(tree_parent: &[Option<usize>]) -> Self {
            let nv = tree_parent.len();

            let tree_children = tree_parent.iter().copied().enumerate().fold(
                vec![vec![]; nv],
                |mut acc, (child, parent)| {
                    if let Some(parent) = parent {
                        acc[parent].push(child);
                    }
                    acc
                },
            );

            let root = (0..nv).find(|&v| tree_parent[v].is_none()).unwrap();

            let dist = {
                fn dfs(dist: &mut [i64], current: usize, tree_children: &[Vec<usize>]) {
                    for &child in &tree_children[current] {
                        dist[child] = dist[current] + 1;
                        dfs(dist, child, tree_children);
                    }
                }
                let mut dist = vec![0; nv];
                dfs(&mut dist, root, &tree_children);
                dist
            };

            let (euler_tour, euler_tour_in_time, euler_tour_out_time) = {
                fn dfs(
                    tour: &mut Vec<usize>,
                    in_time: &mut [usize],
                    out_time: &mut [usize],
                    current: usize,
                    tree_children: &[Vec<usize>],
                ) {
                    // 行きがけ
                    in_time[current] = in_time[current].min(tour.len());
                    out_time[current] = out_time[current].max(tour.len());
                    tour.push(current);

                    for &child in &tree_children[current] {
                        dfs(tour, in_time, out_time, child, tree_children);
                        in_time[current] = in_time[current].min(tour.len());
                        out_time[current] = out_time[current].max(tour.len());
                        tour.push(current);
                    }
                }
                let mut tour = vec![];
                let mut in_time = vec![usize::MAX; nv];
                let mut out_time = vec![usize::MIN; nv];
                dfs(&mut tour, &mut in_time, &mut out_time, root, &tree_children);
                (tour, in_time, out_time)
            };

            let euler_tour_dist = Segtree::<MinI64Usize>::from(
                euler_tour
                    .iter()
                    .copied()
                    .map(|v| (dist[v], v))
                    .collect::<Vec<(i64, usize)>>(),
            );

            Lca {
                dist,
                euler_tour_dist,
                euler_tour_in_time,
                euler_tour_out_time,
            }
        }

        /// u と v の LCA を求める
        /// 計算量 O(log(頂点の数))
        pub fn lca(&self, u: usize, v: usize) -> usize {
            let (time_min, time_max) = {
                use std::cmp::{max, min};
                let t1 = self.euler_tour_in_time[u];
                let t2 = self.euler_tour_in_time[v];
                (min(t1, t2), max(t1, t2))
            };
            // 区間 time_min..=time_max での根からの距離の最小値が LCA になる。
            self.euler_tour_dist.prod(time_min..=time_max).1
        }

        /// 計算量: O(log(頂点の数))
        pub fn dist(&self, u: usize, v: usize) -> i64 {
            self.dist[u] + self.dist[v] - 2 * self.dist[self.lca(u, v)]
        }

        /// パス u-v 上に点 a があるかどうか
        /// 計算量: O(log(頂点の数))
        pub fn is_path_on(&self, u: usize, v: usize, a: usize) -> bool {
            self.dist(u, a) + self.dist(a, v) == self.dist(u, v)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use itertools::Itertools;

    use super::lca::Lca;

    fn lca_naive(tree_parent: &[Option<usize>], u: usize, v: usize) -> usize {
        let ancestor = |x| std::iter::successors(Some(x), |&acc| tree_parent[acc]).collect_vec();
        let u_ancestor = ancestor(u);
        let v_ancestor = ancestor(v).into_iter().collect::<HashSet<_>>();
        u_ancestor
            .iter()
            .copied()
            .find(|x| v_ancestor.contains(x))
            .unwrap()
    }

    #[test]
    fn test_lca() {
        // 0
        // ├ 1
        // │ ├ 3
        // │ │ └ 6
        // │ └ 4
        // │   ├ 7
        // │   ├ 8
        // │   └ 9
        // └ 2
        //   └ 5
        //     ├ 10
        //     └ 11
        let tree_parent = [0, 0, 0, 1, 1, 2, 3, 4, 4, 4, 5, 5]
            .iter()
            .copied()
            .enumerate()
            .map(|(cur, parent)| if cur == parent { None } else { Some(parent) })
            .collect_vec();
        let n = tree_parent.len();
        let lca = Lca::new(&tree_parent);

        // 使用例
        assert_eq!(lca.lca(6, 9), 1);
        assert_eq!(lca.lca(9, 10), 0);
        assert_eq!(lca.lca(1, 6), 1);
        assert_eq!(lca.lca(3, 3), 3);

        // 網羅テスト
        for u in 0..n {
            for v in 0..n {
                assert_eq!(lca.lca(u, v), lca_naive(&tree_parent, u, v));
            }
        }
    }

    #[test]
    fn test_lca2() {
        // ルートが0以外の木でテスト
        // 1
        // ├ 2
        // └ 0
        //   └ 3
        let tree_parent = vec![1, 1, 1, 0]
            .iter()
            .copied()
            .enumerate()
            .map(|(cur, parent)| if cur == parent { None } else { Some(parent) })
            .collect_vec();
        let n = tree_parent.len();
        let lca = Lca::new(&tree_parent);

        for u in 0..n {
            for v in 0..n {
                assert_eq!(lca.lca(u, v), lca_naive(&tree_parent, u, v));
            }
        }
    }

    #[test]
    fn test_lca3() {
        // 複数のサイズの木でテスト
        for n in 1..=8 {
            // 直線的な木 0 - 1 - ... - (n-1)
            let tree_parent = (0..n)
                .map(|i| if i == 0 { None } else { Some(i - 1) })
                .collect_vec();
            let lca = Lca::new(&tree_parent);
            for u in 0..n {
                for v in 0..n {
                    assert_eq!(lca.lca(u, v), lca_naive(&tree_parent, u, v));
                }
            }
        }
    }

    #[test]
    fn test_dist() {
        // 0
        // ├ 1
        // │ ├ 3
        // │ │ └ 6
        // │ └ 4
        // │   ├ 7
        // │   ├ 8
        // │   └ 9
        // └ 2
        //   └ 5
        //     ├ 10
        //     └ 11
        let tree_parent = vec![0, 0, 0, 1, 1, 2, 3, 4, 4, 4, 5, 5]
            .iter()
            .copied()
            .enumerate()
            .map(|(cur, parent)| if cur == parent { None } else { Some(parent) })
            .collect_vec();
        let lca = Lca::new(&tree_parent);

        assert_eq!(lca.dist(6, 9), 4);
        assert_eq!(lca.dist(9, 10), 6);
        assert_eq!(lca.dist(1, 6), 2);
        assert_eq!(lca.dist(3, 3), 0);
    }

    #[test]
    fn test_id_path_on() {
        // 0
        // ├ 1
        // │ ├ 3
        // │ │ └ 6
        // │ └ 4
        // │   ├ 7
        // │   ├ 8
        // │   └ 9
        // └ 2
        //   └ 5
        //     ├ 10
        //     └ 11
        let tree_parent = vec![0, 0, 0, 1, 1, 2, 3, 4, 4, 4, 5, 5]
            .iter()
            .copied()
            .enumerate()
            .map(|(cur, parent)| if cur == parent { None } else { Some(parent) })
            .collect_vec();
        let lca = Lca::new(&tree_parent);

        assert!(lca.is_path_on(6, 9, 4));
        assert!(!lca.is_path_on(6, 9, 8));
        assert!(lca.is_path_on(9, 10, 0));
        assert!(lca.is_path_on(9, 10, 4));
        assert!(lca.is_path_on(1, 6, 3));
        assert!(!lca.is_path_on(1, 6, 4));
        assert!(lca.is_path_on(3, 3, 3));
        assert!(!lca.is_path_on(3, 3, 6));
        assert!(!lca.is_path_on(3, 3, 1));
    }
}
