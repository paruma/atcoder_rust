use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use lca_doubling::*;")]
pub mod lca_doubling {
    use std::mem::swap;

    pub struct Lca {
        dist: Vec<i64>,            // dist[v]: ルートから v までの距離
        ancestor: Vec<Vec<usize>>, // ancestor[i][v]: v の 2^i 先の祖先
    }

    impl Lca {
        pub fn new(adj: &[Vec<usize>], root: usize) -> Self {
            let nv = adj.len();
            let mut dist = vec![-1; nv];
            let mut parent = vec![root; nv];

            let mut q = std::collections::VecDeque::new();
            q.push_back(root);
            dist[root] = 0;

            while let Some(u) = q.pop_front() {
                for &v in &adj[u] {
                    if v != parent[u] {
                        dist[v] = dist[u] + 1;
                        parent[v] = u;
                        q.push_back(v);
                    }
                }
            }

            let k = if nv == 0 {
                0
            } else {
                (usize::BITS - nv.leading_zeros()) as usize
            };

            let mut ancestor = vec![vec![0; nv]; k];
            if nv > 0 {
                ancestor[0] = parent;
            }

            for i in 1..k {
                for v in 0..nv {
                    let p = ancestor[i - 1][v];
                    ancestor[i][v] = ancestor[i - 1][p];
                }
            }

            Lca { dist, ancestor }
        }

        /// u と v の LCA を求める
        ///
        /// # 計算量
        /// O(log(頂点の数))
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

        ///
        /// # 計算量
        /// O(log(頂点の数))
        pub fn dist(&self, u: usize, v: usize) -> i64 {
            self.dist[u] + self.dist[v] - 2 * self.dist[self.lca(u, v)]
        }

        /// パス u-v 上に点 a があるかどうか
        ///
        /// # 計算量
        /// O(log(頂点の数))
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
        let tree_parent_vec = [0, 0, 0, 1, 1, 2, 3, 4, 4, 4, 5, 5]
            .iter()
            .copied()
            .enumerate()
            .map(|(cur, parent)| if cur == parent { None } else { Some(parent) })
            .collect_vec();
        let n = tree_parent_vec.len();

        let (adj, root) = {
            let mut adj = vec![vec![]; n];
            let mut root = 0;
            for (i, &p_opt) in tree_parent_vec.iter().enumerate() {
                if let Some(p) = p_opt {
                    adj[i].push(p);
                    adj[p].push(i);
                } else {
                    root = i;
                }
            }
            (adj, root)
        };
        let lca = Lca::new(&adj, root);

        // 使用例
        assert_eq!(lca.lca(6, 9), 1);
        assert_eq!(lca.lca(9, 10), 0);
        assert_eq!(lca.lca(1, 6), 1);
        assert_eq!(lca.lca(3, 3), 3);

        // 網羅テスト
        for u in 0..n {
            for v in 0..n {
                assert_eq!(lca.lca(u, v), lca_naive(&tree_parent_vec, u, v));
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
        let tree_parent_vec = [1, 1, 1, 0]
            .iter()
            .copied()
            .enumerate()
            .map(|(cur, parent)| if cur == parent { None } else { Some(parent) })
            .collect_vec();
        let n = tree_parent_vec.len();

        let (adj, root) = {
            let mut adj = vec![vec![]; n];
            let mut root = 0;
            for (i, &p_opt) in tree_parent_vec.iter().enumerate() {
                if let Some(p) = p_opt {
                    adj[i].push(p);
                    adj[p].push(i);
                } else {
                    root = i;
                }
            }
            (adj, root)
        };
        let lca = Lca::new(&adj, root);

        for u in 0..n {
            for v in 0..n {
                assert_eq!(lca.lca(u, v), lca_naive(&tree_parent_vec, u, v));
            }
        }
    }

    #[test]
    fn test_lca3() {
        // 複数のサイズの木でテスト
        for n in 1..=8 {
            // 直線的な木 0 - 1 - ... - (n-1)
            let tree_parent_vec = (0..n)
                .map(|i| if i == 0 { None } else { Some(i - 1) })
                .collect_vec();

            let (adj, root) = {
                let mut adj = vec![vec![]; n];
                let mut root = 0;
                for (i, &p_opt) in tree_parent_vec.iter().enumerate() {
                    if let Some(p) = p_opt {
                        adj[i].push(p);
                        adj[p].push(i);
                    } else {
                        root = i;
                    }
                }
                (adj, root)
            };
            let lca = Lca::new(&adj, root);
            for u in 0..n {
                for v in 0..n {
                    assert_eq!(lca.lca(u, v), lca_naive(&tree_parent_vec, u, v));
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
        let tree_parent_vec = [0, 0, 0, 1, 1, 2, 3, 4, 4, 4, 5, 5]
            .iter()
            .copied()
            .enumerate()
            .map(|(cur, parent)| if cur == parent { None } else { Some(parent) })
            .collect_vec();
        let n = tree_parent_vec.len();

        let (adj, root) = {
            let mut adj = vec![vec![]; n];
            let mut root = 0;
            for (i, &p_opt) in tree_parent_vec.iter().enumerate() {
                if let Some(p) = p_opt {
                    adj[i].push(p);
                    adj[p].push(i);
                } else {
                    root = i;
                }
            }
            (adj, root)
        };
        let lca = Lca::new(&adj, root);

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
        let tree_parent_vec = [0, 0, 0, 1, 1, 2, 3, 4, 4, 4, 5, 5]
            .iter()
            .copied()
            .enumerate()
            .map(|(cur, parent)| if cur == parent { None } else { Some(parent) })
            .collect_vec();
        let n = tree_parent_vec.len();

        let (adj, root) = {
            let mut adj = vec![vec![]; n];
            let mut root = 0;
            for (i, &p_opt) in tree_parent_vec.iter().enumerate() {
                if let Some(p) = p_opt {
                    adj[i].push(p);
                    adj[p].push(i);
                } else {
                    root = i;
                }
            }
            (adj, root)
        };
        let lca = Lca::new(&adj, root);

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
