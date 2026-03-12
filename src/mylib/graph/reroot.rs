use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use reroot::*;")]
pub mod reroot {
    use ac_library::Monoid;
    use mod_queue::*;

    /// 全方位木DPを行うためのトレイトです。
    pub trait Reroot {
        type M: Monoid; // 可換モノイド

        /// 子部分木+辺の集約結果に頂点の値を加えます。
        ///
        /// # Arguments
        /// * `x` - 頂点 `v` の各子の「部分木+辺」の集約値
        /// * `v` - 対象の頂点
        ///
        /// # Returns
        /// `x` に頂点 `v` 自身の値を加えた結果
        fn add_vertex(&self, x: &<Self::M as Monoid>::S, v: usize) -> <Self::M as Monoid>::S;

        /// 部分木の集約結果にエッジの値を加えます。
        ///
        /// # Arguments
        /// * `x` - 隣接頂点 `adj[v][ei]` を根とする部分木の集約値
        /// * `v` - エッジの始点
        /// * `ei` - 頂点 `v` の `ei` 番目のエッジ（隣接頂点 `adj[v][ei]`）
        ///
        /// # Returns
        /// `x` にエッジ `v--adj[v][ei]` に関する値を加えた結果
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
            // parent[u]: Some((親の頂点番号 p, uからpへのインデックス, pからuへのインデックス))
            let (children, parent, bfs_order) = {
                let mut children = vec![vec![]; n];
                let mut parent = vec![None; n];
                let mut bfs_order = Vec::with_capacity(n);
                let mut queue = Queue::new();

                let mut visited = vec![false; n];
                visited[0] = true;
                queue.push(0);

                while let Some(cur) = queue.pop() {
                    bfs_order.push(cur);
                    for (cur_to_next, &next) in adj[cur].iter().enumerate() {
                        if !visited[next] {
                            visited[next] = true;
                            let next_to_cur = adj[next]
                                .iter()
                                .position(|&back| back == cur)
                                .expect("Edge must be bidirectional");
                            children[cur].push((next, cur_to_next, next_to_cur));
                            parent[next] = Some((cur, next_to_cur, cur_to_next));
                            queue.push(next);
                        }
                    }
                }
                (children, parent, bfs_order)
            };

            // dp[u][i]: u から見て i 番目の隣接頂点方向にある部分木の集約値
            let mut dp: Vec<Vec<<Self::M as Monoid>::S>> = adj
                .iter()
                .map(|next_list| vec![Self::M::identity(); next_list.len()])
                .collect();

            // 2. 下向きの部分木の集約値を決定
            // 各頂点 u とその親 p に対して、p → u の方向の u を根とする部分木の集約値を計算する
            for &u in bfs_order.iter().rev() {
                if let Some((p, _u_to_p, p_to_u)) = parent[u] {
                    let res = children[u]
                        .iter()
                        .map(|&(_c, u_to_c, _c_to_u)| self.add_edge(&dp[u][u_to_c], u, u_to_c))
                        .fold(Self::M::identity(), |acc, val| {
                            Self::M::binary_operation(&acc, &val)
                        });
                    // p → u の方向の u を根とする部分木の集約値
                    dp[p][p_to_u] = self.add_vertex(&res, u);
                }
            }

            // 3. 上向きの部分木の集約値を決定
            // 各頂点 u とその子 c に対して、c → p 方向の u を根とする部分木の集約値を計算する
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

                for &(c, u_to_c, c_to_u) in &children[u] {
                    let res_without_c = cum.prod_without1(u_to_c);
                    // c → p 方向の u を根とする部分木の集約値
                    dp[c][c_to_u] = self.add_vertex(&res_without_c, u);
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
    struct CumMonoid<M: Monoid> {
        prefix: Vec<M::S>,
        suffix: Vec<M::S>,
    }

    impl<M: Monoid> CumMonoid<M> {
        fn new(xs: &[M::S]) -> Self {
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
        fn prod_without1(&self, i: usize) -> M::S {
            M::binary_operation(&self.prefix[i], &self.suffix[i + 1])
        }
    }

    mod mod_queue {
        use std::collections::VecDeque;
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct Queue<T> {
            raw: VecDeque<T>,
        }
        impl<T> Queue<T> {
            #[allow(clippy::new_without_default)]
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
}

#[cfg(test)]
mod tests {
    use super::reroot::*;

    #[test]
    fn test_reroot_empty_graph() {
        let adj: Vec<Vec<usize>> = vec![];
        let reroot_solver = DistMaxReroot;
        let result = reroot_solver.reroot(&adj);
        assert_eq!(result, vec![]);
    }

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
