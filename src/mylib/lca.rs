use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use lca::*;")]
pub mod lca {
    use std::mem::swap;

    pub struct Lca {
        dist: Vec<i64>,            // dist[v]: ルートから v までの距離
        ancestor: Vec<Vec<usize>>, // ancestor[i][v]: v の 2^i 先の祖先
    }

    impl Lca {
        /// tree_parent[i]: i の 親 を表す。根の場合は tree_parent[i] == i
        /// 計算量: O(nv log(nv)) (nv は頂点の数とする)
        pub fn new(tree_parent: &[usize]) -> Self {
            let nv = tree_parent.len();

            let tree_children = tree_parent.iter().copied().enumerate().fold(
                vec![vec![]; nv],
                |mut acc, (child, parent)| {
                    if child != parent {
                        acc[parent].push(child);
                    }
                    acc
                },
            );

            let root = (0..nv).find(|&v| tree_parent[v] == v).unwrap();

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

            let ancestor = {
                // 2^k >= nv となる最小のk
                let k = (0..).find(|&i| (1 << i) >= nv).unwrap();
                let mut ancestor = vec![vec![0; nv]; k];
                ancestor[0] = tree_parent.to_vec();
                for i in 1..k {
                    for v in 0..nv {
                        let f = &ancestor[i - 1];
                        ancestor[i][v] = f[f[v]]
                    }
                }

                ancestor
            };
            Lca { dist, ancestor }
        }

        /// u と v の LCA を求める
        /// 計算量 O(log(頂点の数))
        pub fn lca(&self, u: usize, v: usize) -> usize {
            let mut u = u;
            let mut v = v;
            // u のほうが深いとする (dist[u] >= dist[v] となるようにする)
            if self.dist[u] < self.dist[v] {
                swap(&mut u, &mut v);
            }

            // 深さを揃える (u を dist[u] - dist[v] だけ根の方向に動かす)
            let dist_diff = self.dist[u] - self.dist[v];
            u = self
                .ancestor
                .iter()
                .enumerate()
                .filter(|(k, _)| (dist_diff >> k) & 1 == 1)
                .map(|(_, f)| f)
                .fold(u, |acc, f| f[acc]);

            if u == v {
                return u;
            }

            // u を LCA の子まで進める
            for f in self.ancestor.iter().rev() {
                if f[u] != f[v] {
                    u = f[u];
                    v = f[v];
                }
            }

            self.ancestor[0][u]
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

    fn lca_naive(tree_parent: &[usize], u: usize, v: usize) -> usize {
        let ancestor = |x| {
            std::iter::successors(Some(x), |&acc| {
                let next = tree_parent[acc];
                (next != acc).then_some(next)
            })
            .collect_vec()
        };
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
        let tree_parent = vec![0, 0, 0, 1, 1, 2, 3, 4, 4, 4, 5, 5];
        let n = tree_parent.len();
        let lca = Lca::new(&tree_parent);

        assert_eq!(lca.lca(6, 9), 1);
        assert_eq!(lca.lca(9, 10), 0);
        assert_eq!(lca.lca(1, 6), 1);
        assert_eq!(lca.lca(3, 3), 3);

        for u in 0..n {
            for v in 0..n {
                assert_eq!(lca.lca(u, v), lca_naive(&tree_parent, u, v));
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
        let tree_parent = vec![0, 0, 0, 1, 1, 2, 3, 4, 4, 4, 5, 5];
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
        let tree_parent = vec![0, 0, 0, 1, 1, 2, 3, 4, 4, 4, 5, 5];
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
