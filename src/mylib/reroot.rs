use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use reroot::*;")]
pub mod reroot {

    // Reroot の実装サンプル
    use ac_library::Max;

    #[allow(dead_code)]
    pub struct DistMaxReroot();
    impl Reroot for DistMaxReroot {
        type M = Max<u64>;

        fn add_vertex(&self, x: &<Self::M as Monoid>::S, _v: usize) -> <Self::M as Monoid>::S {
            *x
        }

        fn add_edge(
            &self,
            x: &<Self::M as Monoid>::S,
            _v: usize,
            _ei: usize,
        ) -> <Self::M as Monoid>::S {
            x + 1
        }
    }

    /// 全方位木DP
    pub trait Reroot {
        type M: Monoid; // 可換モノイド

        fn add_vertex(&self, x: &<Self::M as Monoid>::S, v: usize) -> <Self::M as Monoid>::S;
        fn add_edge(
            &self,
            x: &<Self::M as Monoid>::S,
            v: usize,
            ei: usize,
        ) -> <Self::M as Monoid>::S;

        fn prod(xs: &[<Self::M as Monoid>::S]) -> <Self::M as Monoid>::S {
            xs.iter().fold(Self::M::identity(), |acc, x| {
                Self::M::binary_operation(&acc, x)
            })
        }

        fn reroot(&self, adj: &[Vec<usize>]) -> Vec<<Self::M as Monoid>::S> {
            let nv = adj.len();
            // dp[v][i]: 頂点v から生える i番目の有向辺の先にある部分木に関する値
            let mut dp: Vec<Vec<<Self::M as Monoid>::S>> = adj
                .iter()
                .map(|next_list| {
                    let degree = next_list.len();
                    vec![Self::M::identity(); degree]
                })
                .collect_vec();

            {
                // 頂点を DFS 帰りかけ順に並べたもの
                let dfs_post_order = dfs_post_order(adj, 0);
                let mut visited = vec![false; nv];

                for &current_v in &dfs_post_order {
                    visited[current_v] = true;

                    for (current_e, next_v) in adj[current_v].iter().copied().enumerate() {
                        if !visited[next_v] {
                            continue;
                        }

                        dp[current_v][current_e] = {
                            let edge_dp_next = dp[next_v]
                                .iter()
                                .enumerate()
                                .filter(|(next_e, _)| adj[next_v][*next_e] != current_v)
                                .map(|(next_e, x)| self.add_edge(x, next_v, next_e))
                                .collect_vec();
                            let prod = Self::prod(&edge_dp_next);
                            self.add_vertex(&prod, next_v)
                        };
                    }
                }
            }
            {
                // 頂点を BFS の訪問順に並べたもの
                let bfs_order = bfs_order(adj, 0);
                let mut visited = vec![false; nv];
                for &current_v in &bfs_order {
                    visited[current_v] = true;
                    let edge_dp_current = dp[current_v]
                        .iter()
                        .enumerate()
                        .map(|(current_e, x)| self.add_edge(x, current_v, current_e))
                        .collect_vec();
                    let cum_monoid = CumMonoid::<Self::M>::new(&edge_dp_current);
                    for (current_e, next_v) in adj[current_v].iter().copied().enumerate() {
                        if visited[next_v] {
                            continue;
                        }
                        // 償却 O(1) で計算可能
                        let rev_current_e =
                            adj[next_v].iter().position(|&v| v == current_v).unwrap();

                        dp[next_v][rev_current_e] = {
                            let prod = cum_monoid.prod_without1(current_e);
                            self.add_vertex(&prod, current_v)
                        };
                    }
                }
            }
            dp.iter()
                .enumerate()
                .map(|(current_v, dp_current)| {
                    let edge_dp_current = dp_current
                        .iter()
                        .enumerate()
                        .map(|(current_e, x)| self.add_edge(x, current_v, current_e))
                        .collect_vec();
                    self.add_vertex(&Self::prod(&edge_dp_current), current_v)
                })
                .collect_vec()
        }
    }

    fn bfs_order(adj: &[Vec<usize>], init: usize) -> Vec<usize> {
        let nv = adj.len();
        let mut order = vec![];
        let mut visited = vec![false; nv];
        let mut open = Queue::new();
        open.push(init);
        order.push(init);
        visited[init] = true;
        while let Some(current) = open.pop() {
            for &next in &adj[current] {
                if !visited[next] {
                    order.push(next);
                    visited[next] = true;
                    open.push(next);
                }
            }
        }
        order
    }

    fn dfs_post_order(adj: &[Vec<usize>], init: usize) -> Vec<usize> {
        enum State {
            Pre(usize),
            Post(usize),
        }

        let nv = adj.len();
        let mut order = vec![];
        let mut visited = vec![false; nv];
        let mut open = Stack::new();
        open.push(State::Post(init));
        open.push(State::Pre(init));
        while let Some(current) = open.pop() {
            match current {
                State::Pre(v) => {
                    visited[v] = true;
                    for &edge in &adj[v] {
                        if !visited[edge] {
                            open.push(State::Post(edge));
                            open.push(State::Pre(edge));
                        }
                    }
                }
                State::Post(v) => {
                    // 帰りがけ
                    order.push(v);
                }
            }
        }
        order
    }

    use ac_library::Monoid;
    use cum_monoid::*;
    pub mod cum_monoid {
        use ac_library::Monoid;
        pub struct CumMonoid<M>
        where
            M: Monoid,
        {
            prefix_prod: Vec<M::S>,
            suffix_prod: Vec<M::S>,
        }
        impl<M> CumMonoid<M>
        where
            M: Monoid,
        {
            pub fn new(xs: &[M::S]) -> CumMonoid<M> {
                let mut prefix_prod = vec![M::identity(); xs.len() + 1];
                let mut suffix_prod = vec![M::identity(); xs.len() + 1];
                for i in 0..xs.len() {
                    prefix_prod[i + 1] = M::binary_operation(&prefix_prod[i], &xs[i]);
                }
                for i in (0..xs.len()).rev() {
                    suffix_prod[i] = M::binary_operation(&xs[i], &suffix_prod[i + 1]);
                }
                CumMonoid {
                    prefix_prod,
                    suffix_prod,
                }
            }

            /// [0, i), [i + 1, n) の区間で総積を取る
            pub fn prod_without1(&self, i: usize) -> M::S {
                M::binary_operation(&self.prefix_prod[i], &self.suffix_prod[i + 1])
            }
        }
    }
    use itertools::Itertools;
    use mod_stack::*;
    pub mod mod_stack {
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct Stack<T> {
            raw: Vec<T>,
        }
        impl<T> Stack<T> {
            pub fn new() -> Self {
                Stack { raw: Vec::new() }
            }
            pub fn push(&mut self, value: T) {
                self.raw.push(value)
            }
            pub fn pop(&mut self) -> Option<T> {
                self.raw.pop()
            }
            pub fn peek(&self) -> Option<&T> {
                self.raw.last()
            }
            pub fn is_empty(&self) -> bool {
                self.raw.is_empty()
            }
            pub fn len(&self) -> usize {
                self.raw.len()
            }
        }
        impl<T> Default for Stack<T> {
            fn default() -> Self {
                Self::new()
            }
        }
    }

    use mod_queue::*;
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
            pub fn peek(&self) -> Option<&T> {
                self.raw.front()
            }
            pub fn is_empty(&self) -> bool {
                self.raw.is_empty()
            }
            pub fn len(&self) -> usize {
                self.raw.len()
            }
        }
        impl<T> Default for Queue<T> {
            fn default() -> Self {
                Self::new()
            }
        }
    }
}

// テスト: 以下の問題が通っている
// EDPC V - Subtree https://atcoder.jp/contests/dp/tasks/dp_v
// ABC348 E - Minimize Sum of Distances https://atcoder.jp/contests/abc348/tasks/abc348_e
// ABC401 F - Add One Edge3 https://atcoder.jp/contests/abc401/tasks/abc401_f

#[cfg(test)]
mod tests {
    use super::reroot::*;

    #[test]
    fn test_reroot_path_graph() {
        // 0 -- 1 -- 2 -- 3
        let adj = vec![vec![1], vec![0, 2], vec![1, 3], vec![2]];
        let reroot_solver = DistMaxReroot();
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
        let reroot_solver = DistMaxReroot();
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
        let reroot_solver = DistMaxReroot();
        let result = reroot_solver.reroot(&adj);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_reroot_two_nodes() {
        // 0 -- 1
        let adj = vec![vec![1], vec![0]];
        let reroot_solver = DistMaxReroot();
        let result = reroot_solver.reroot(&adj);
        assert_eq!(result, vec![1, 1]);
    }
}
