use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use random_test::*;")]
pub mod reroot {

    // Reroot の実装サンプル
    use ac_library::Max;
    struct DistMaxReroot();
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
                .map(|edges| {
                    let degree = edges.len();
                    vec![Self::M::identity(); degree]
                })
                .collect_vec();

            {
                // 頂点を DFS 帰りかけ順に並べたもの
                let dfs_post_order = dfs_post_order(adj, 0);
                let mut visited = vec![false; nv];

                for &current in &dfs_post_order {
                    visited[current] = true;

                    for (edge_i, next) in adj[current].iter().copied().enumerate() {
                        if !visited[next] {
                            continue;
                        }

                        dp[current][edge_i] = {
                            let edge_dp_next = dp[next]
                                .iter()
                                .enumerate()
                                .filter(|(ei, _)| adj[next][*ei] != current)
                                .map(|(ei, x)| self.add_edge(x, next, ei))
                                .collect_vec();
                            let prod = Self::prod(&edge_dp_next);
                            self.add_vertex(&prod, next)
                        };
                    }
                }
            }
            {
                // 頂点を BFS の訪問順に並べたもの
                let bfs_order = bfs_order(adj, 0);
                let mut visited = vec![false; nv];
                for &current in &bfs_order {
                    visited[current] = true;
                    let edge_dp_current = dp[current]
                        .iter()
                        .enumerate()
                        .map(|(ei, x)| self.add_edge(x, current, ei))
                        .collect_vec();
                    let cum_monoid = CumMonoid::<Self::M>::new(&edge_dp_current);
                    for (edge_i, next) in adj[current].iter().copied().enumerate() {
                        if visited[next] {
                            continue;
                        }
                        // 償却 O(1) で計算可能
                        let rev_edge_i = adj[next].iter().position(|&v| v == current).unwrap();

                        dp[next][rev_edge_i] = {
                            let prod = cum_monoid.prod_without1(edge_i);
                            self.add_vertex(&prod, current)
                        };
                    }
                }
            }
            dp.iter()
                .enumerate()
                .map(|(v, dp_v)| {
                    let edge_dp_v = dp_v
                        .iter()
                        .enumerate()
                        .map(|(ei, x)| self.add_edge(x, v, ei))
                        .collect_vec();
                    self.add_vertex(&Self::prod(&edge_dp_v), v)
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
