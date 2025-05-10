#[allow(clippy::module_inception)]
pub mod lca_doubling {
    use std::mem::swap;

    pub struct Lca {
        dist: Vec<i64>,            // dist[v]: ルートから v までの距離
        ancestor: Vec<Vec<usize>>, // ancestor[i][v]: v の 2^i 先の祖先
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

            // root の親を root として扱う
            let tree_parent = tree_parent
                .iter()
                .copied()
                .enumerate()
                .map(|(cur, parent)| parent.unwrap_or(cur))
                .collect::<Vec<usize>>();

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
                // nv の２進展開の桁数
                let k = (usize::BITS - nv.leading_zeros()) as usize;
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

    use super::lca_doubling::Lca;

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
