// use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
// #[snippet(prefix = "use reroot2::*;")]
pub mod reroot2 {
    use ac_library::Monoid;
    use mod_queue::*;

    /// 全方位木DPを行うためのトレイトです。
    pub trait Reroot {
        type M: Monoid; // 可換モノイド

        /// 頂点 `v` の値を集約結果 `x` に加えます（頂点自身の重みなどを処理します）。
        fn add_vertex(&self, x: &<Self::M as Monoid>::S, v: usize) -> <Self::M as Monoid>::S;

        /// 頂点 `v` から `ei` 番目のエッジ（隣接頂点 `adj[v][ei]`）方向から来る部分木の集約結果 `x` を加工します。
        fn add_edge(
            &self,
            x: &<Self::M as Monoid>::S,
            v: usize,
            ei: usize,
        ) -> <Self::M as Monoid>::S;

        /// 全方位木DPを実行し、各頂点を根としたときの値を求めます。
        ///
        /// 具体的には、頂点 `u` を根とした根付き木において、
        /// 各頂点 `v` の値 `f_u(v)` を以下で再帰的に定義します：
        /// ```text
        /// f_u(v) = add_vertex(⊕_{c ∈ ch_u(v)} add_edge(f_u(c), v, index(v, c)), v)
        /// ```
        /// ここで `ch_u(v)` は `u` を根とした時の `v` の子頂点の集合、
        /// `index(v, c)` は `adj[v]` における `c` のインデックス、
        /// ⊕ はモノイドの二項演算です。
        ///
        /// 返り値を `result` とすると、`result[u] = f_u(u)` です。
        ///
        /// # Arguments
        /// * `adj` - 木の隣接リスト
        ///
        /// # 計算量
        /// O(V) (V は頂点数)
        fn reroot(&self, adj: &[Vec<usize>]) -> Vec<<Self::M as Monoid>::S> {
            let n = adj.len();
            if n == 0 {
                return vec![];
            }
            if n == 1 {
                return vec![self.add_vertex(&Self::M::identity(), 0)];
            }

            // 1. 木の構造を整理（根を 0 とする）
            // children[u]: vec![(子の頂点番号 v, uからvへのインデックス, vからuへのインデックス)]
            // parent_info[u]: Some((親の頂点番号 p, uからpへのインデックス, pからuへのインデックス))
            let (children, parent_info, bfs_order) = {
                let mut children = vec![vec![]; n];
                let mut parent_info = vec![None; n];
                let mut bfs_order = Vec::with_capacity(n);
                let mut queue = Queue::new();

                let mut visited = vec![false; n];
                visited[0] = true;
                queue.push(0);

                while let Some(u) = queue.pop() {
                    bfs_order.push(u);
                    for (u_to_v, &v) in adj[u].iter().enumerate() {
                        if !visited[v] {
                            visited[v] = true;
                            let v_to_u = adj[v]
                                .iter()
                                .position(|&back| back == u)
                                .expect("Edge must be bidirectional");
                            children[u].push((v, u_to_v, v_to_u));
                            parent_info[v] = Some((u, v_to_u, u_to_v));
                            queue.push(v);
                        }
                    }
                }
                (children, parent_info, bfs_order)
            };

            // dp[u][i]: u から見て i 番目の隣接頂点方向にある部分木の集約値
            let mut dp: Vec<Vec<<Self::M as Monoid>::S>> = adj
                .iter()
                .map(|next_list| vec![Self::M::identity(); next_list.len()])
                .collect();

            // 2. ボトムアップパス（葉から根へ）
            // 各頂点 u について、その子方向からの値を集約して親への値を確定させます。
            for &u in bfs_order.iter().rev() {
                if let Some((p, _u_to_p, p_to_u)) = parent_info[u] {
                    let res = children[u]
                        .iter()
                        .map(|&(_v, u_to_v, _v_to_u)| self.add_edge(&dp[u][u_to_v], u, u_to_v))
                        .fold(Self::M::identity(), |acc, val| {
                            Self::M::binary_operation(&acc, &val)
                        });
                    // u から親 p 方向への値を dp[p] に書き込みます。
                    dp[p][p_to_u] = self.add_vertex(&res, u);
                }
            }

            // 3. トップダウンパス（根から葉へ）
            // 各頂点 u について、親と兄弟からの値を集約して各子への値を確定させます。
            for &u in &bfs_order {
                if children[u].is_empty() {
                    continue;
                }

                let edge_values: Vec<_> = dp[u]
                    .iter()
                    .enumerate()
                    .map(|(i, x)| self.add_edge(x, u, i))
                    .collect();

                let cum = CumMonoid::<Self::M>::new(&edge_values);

                for &(v, u_to_v, v_to_u) in &children[u] {
                    // v 以外のすべての兄弟と親からの値を合成して子 v へ送ります。
                    let res_without_v = cum.prod_without1(u_to_v);
                    dp[v][v_to_u] = self.add_vertex(&res_without_v, u);
                }
            }

            // 4. 各頂点を根とした最終集計
            (0..n)
                .map(|u| {
                    let res = dp[u]
                        .iter()
                        .enumerate()
                        .map(|(i, x)| self.add_edge(x, u, i))
                        .fold(Self::M::identity(), |acc, val| {
                            Self::M::binary_operation(&acc, &val)
                        });
                    self.add_vertex(&res, u)
                })
                .collect()
        }
    }

    /// 累積積を効率的に計算するための構造体です。
    pub struct CumMonoid<M: Monoid> {
        prefix: Vec<M::S>,
        suffix: Vec<M::S>,
    }

    impl<M: Monoid> CumMonoid<M> {
        pub fn new(xs: &[M::S]) -> Self {
            let n = xs.len();
            let mut prefix = vec![M::identity(); n + 1];
            let mut suffix = vec![M::identity(); n + 1];
            for i in 0..n {
                prefix[i + 1] = M::binary_operation(&prefix[i], &xs[i]);
            }
            for i in (0..n).rev() {
                suffix[i] = M::binary_operation(&xs[i], &suffix[i + 1]);
            }
            Self { prefix, suffix }
        }

        /// インデックス `i` の要素を除いた全体の積を求めます。
        pub fn prod_without1(&self, i: usize) -> M::S {
            M::binary_operation(&self.prefix[i], &self.suffix[i + 1])
        }
    }

    // --- Reroot の実装サンプル ---
    use ac_library::Max;

    #[derive(Clone, Copy, Debug)]
    pub struct DistMaxReroot;
    impl Reroot for DistMaxReroot {
        type M = Max<u64>;

        fn add_vertex(&self, x: &u64, _v: usize) -> u64 {
            *x
        }
        fn add_edge(&self, x: &u64, _v: usize, _ei: usize) -> u64 {
            x + 1
        }
    }

    pub mod mod_queue {
        use std::collections::VecDeque;
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct Queue<T> {
            raw: VecDeque<T>,
        }
        impl<T> Queue<T> {
            pub fn new() -> Self {
                Queue {
                    raw: VecDeque::new(),
                }
            }
            pub fn push(&mut self, value: T) {
                self.raw.push_back(value)
            }
            pub fn pop(&mut self) -> Option<T> {
                self.raw.pop_front()
            }
            pub fn is_empty(&self) -> bool {
                self.raw.is_empty()
            }
        }
        impl<T> Default for Queue<T> {
            fn default() -> Self {
                Self::new()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::reroot2::*;

    #[test]
    fn test_reroot_path_graph() {
        // 0 -- 1 -- 2 -- 3
        let adj = vec![vec![1], vec![0, 2], vec![1, 3], vec![2]];
        let reroot_solver = DistMaxReroot;
        let result = reroot_solver.reroot(&adj);

        // 各頂点からの最大距離
        // 0: 3 (to 3)
        // 1: 2 (to 3 or 0)
        // 2: 2 (to 0 or 3)
        // 3: 3 (to 0)
        assert_eq!(result, vec![3, 2, 2, 3]);
    }

    #[test]
    fn test_reroot_star_graph() {
        //   1
        //   |
        // 0-+-2
        //   |
        //   3
        let adj = vec![vec![1, 2, 3], vec![0], vec![0], vec![0]];
        let reroot_solver = DistMaxReroot;
        let result = reroot_solver.reroot(&adj);

        // 各頂点からの最大距離
        // 0: 1 (to 1, 2, or 3)
        // 1: 2 (to 2 or 3 via 0)
        // 2: 2 (to 1 or 3 via 0)
        // 3: 2 (to 1 or 2 via 0)
        assert_eq!(result, vec![1, 2, 2, 2]);
    }

    #[test]
    fn test_reroot_single_node() {
        // 0
        let adj = vec![vec![]];
        let reroot_solver = DistMaxReroot;
        let result = reroot_solver.reroot(&adj);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_reroot_two_nodes() {
        // 0 -- 1
        let adj = vec![vec![1], vec![0]];
        let reroot_solver = DistMaxReroot;
        let result = reroot_solver.reroot(&adj);
        assert_eq!(result, vec![1, 1]);
    }
}
