use cargo_snippet::snippet;

#[snippet(prefix = "use functional_graph::*;")]
#[allow(clippy::module_inception)]
pub mod functional_graph {
    /// Functional Graph (各頂点の出次数が 1 の有向グラフ) の解析構造体
    ///
    /// 各連結成分はちょうど1つのサイクルを持ち、そのサイクルにいくつかの木が流れ込む構造をしています。
    #[derive(Clone, Debug)]
    pub struct FunctionalGraph {
        /// 頂点 u が属する連結成分の ID
        comp_id: Vec<usize>,
        /// 頂点 u からサイクルに到達するまでの距離。サイクル上の頂点は 0。
        depth: Vec<usize>,
        /// 頂点 u から進んで最初に到達するサイクル上の頂点
        root_on_cycle: Vec<usize>,
        /// サイクル内での位置 (0-indexed)
        pos_in_cycle: Vec<usize>,
        /// 各連結成分ごとのサイクル長
        cycle_lens: Vec<usize>,
        // 逆向きの木（サイクルを根とする森）での DFS 順
        in_time: Vec<usize>,
        out_time: Vec<usize>,
    }

    impl FunctionalGraph {
        /// Functional Graph を構築する
        ///
        /// # Arguments
        /// * `next` - 各頂点の遷移先 (0..nv-1)
        ///
        /// # 計算量
        /// O(V)
        pub fn new(next: &[usize]) -> Self {
            let nv = next.len();
            let mut in_degree = vec![0; nv];
            for &v in next {
                in_degree[v] += 1;
            }

            // 1. 入次数 0 の頂点から削っていき、サイクル外の頂点を特定する
            let mut queue = std::collections::VecDeque::new();
            for i in 0..nv {
                if in_degree[i] == 0 {
                    queue.push_back(i);
                }
            }

            let mut is_on_cycle = vec![true; nv];
            while let Some(u) = queue.pop_front() {
                is_on_cycle[u] = false;
                let v = next[u];
                in_degree[v] -= 1;
                if in_degree[v] == 0 {
                    queue.push_back(v);
                }
            }

            // 2. サイクルを特定し、連結成分に分ける
            let mut comp_id = vec![usize::MAX; nv];
            let mut pos_in_cycle = vec![0; nv];
            let mut cycles = Vec::new();
            let mut cycle_lens = Vec::new();

            for i in 0..nv {
                if is_on_cycle[i] && comp_id[i] == usize::MAX {
                    let cid = cycles.len();
                    let mut curr_cycle = Vec::new();
                    let mut curr = i;
                    while comp_id[curr] == usize::MAX {
                        comp_id[curr] = cid;
                        pos_in_cycle[curr] = curr_cycle.len();
                        curr_cycle.push(curr);
                        curr = next[curr];
                    }
                    cycle_lens.push(curr_cycle.len());
                    cycles.push(curr_cycle);
                }
            }

            // 3. 逆向きの辺（木構造）を構築し、サイクル上の頂点を根として DFS
            let mut rev_adj = vec![vec![]; nv];
            for i in 0..nv {
                if !is_on_cycle[i] {
                    rev_adj[next[i]].push(i);
                }
            }

            let mut depth = vec![0; nv];
            let mut root_on_cycle = vec![0; nv];
            let mut in_time = vec![0; nv];
            let mut out_time = vec![0; nv];
            let mut timer = 0;

            for cid in 0..cycles.len() {
                for &root in &cycles[cid] {
                    Self::dfs_forest(
                        root,
                        root,
                        cid,
                        0,
                        &rev_adj,
                        &mut depth,
                        &mut root_on_cycle,
                        &mut comp_id,
                        &mut in_time,
                        &mut out_time,
                        &mut timer,
                    );
                }
            }

            Self {
                comp_id,
                depth,
                root_on_cycle,
                pos_in_cycle,
                cycle_lens,
                in_time,
                out_time,
            }
        }

        fn dfs_forest(
            u: usize,
            root: usize,
            cid: usize,
            d: usize,
            rev_adj: &[Vec<usize>],
            depth: &mut [usize],
            root_on_cycle: &mut [usize],
            comp_id: &mut [usize],
            in_time: &mut [usize],
            out_time: &mut [usize],
            timer: &mut usize,
        ) {
            in_time[u] = *timer;
            *timer += 1;
            depth[u] = d;
            root_on_cycle[u] = root;
            comp_id[u] = cid;

            for &v in &rev_adj[u] {
                Self::dfs_forest(
                    v,
                    root,
                    cid,
                    d + 1,
                    rev_adj,
                    depth,
                    root_on_cycle,
                    comp_id,
                    in_time,
                    out_time,
                    timer,
                );
            }
            out_time[u] = *timer;
        }

        /// 頂点 u から v への最短距離を返す。到達不能なら None。
        ///
        /// # 計算量
        /// O(1)
        pub fn distance(&self, u: usize, v: usize) -> Option<usize> {
            if self.comp_id[u] != self.comp_id[v] {
                return None;
            }

            if self.depth[v] > 0 {
                // ケース A: v がサイクル外（木の部分）にある
                // u は v の子孫（逆向きの木において）である必要がある
                if self.root_on_cycle[u] == self.root_on_cycle[v]
                    && self.in_time[v] <= self.in_time[u]
                    && self.out_time[u] <= self.out_time[v]
                {
                    Some(self.depth[u] - self.depth[v])
                } else {
                    None
                }
            } else {
                // ケース B: v がサイクル上にある
                // u はいつかはサイクルに到達する
                let d_to_cycle = self.depth[u];
                let u_root = self.root_on_cycle[u];
                let cid = self.comp_id[u];
                let clen = self.cycle_lens[cid];
                let dist_on_cycle =
                    (self.pos_in_cycle[v] + clen - self.pos_in_cycle[u_root]) % clen;
                Some(d_to_cycle + dist_on_cycle)
            }
        }

        /// 頂点 u から v に到達可能か判定する
        ///
        /// # 計算量
        /// O(1)
        pub fn is_reachable(&self, u: usize, v: usize) -> bool {
            self.distance(u, v).is_some()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::functional_graph::*;

    fn find_distance_brute_force(u: usize, v: usize, next: &[usize]) -> Option<usize> {
        let mut curr = u;
        for d in 0..next.len() {
            if curr == v {
                return Some(d);
            }
            curr = next[curr];
        }
        None
    }

    #[test]
    fn test_functional_graph_basic() {
        // 0 -> 1 -> 2 -> 0 (cycle)
        // 3 -> 1
        // 4 -> 3
        // 5 -> 5 (self-loop cycle)
        let next = vec![1, 2, 0, 1, 3, 5];
        let fg = FunctionalGraph::new(&next);
        // 距離判定
        assert_eq!(fg.distance(4, 1), Some(2));
        assert_eq!(fg.distance(4, 2), Some(3));
        assert_eq!(fg.distance(4, 0), Some(4));
        assert_eq!(fg.distance(4, 3), Some(1));
        assert_eq!(fg.distance(1, 0), Some(2));
        assert_eq!(fg.distance(0, 3), None); // 逆向きは不可
        assert_eq!(fg.distance(4, 5), None); // 別コンポーネント

        // 到達可能性
        assert!(fg.is_reachable(4, 0));
        assert!(!fg.is_reachable(0, 4));
    }

    #[test]
    fn test_functional_graph_multi_cycle() {
        // 0 -> 1 -> 0
        // 2 -> 3 -> 2
        let next = vec![1, 0, 3, 2];
        let fg = FunctionalGraph::new(&next);
        assert_eq!(fg.distance(0, 1), Some(1));
        assert_eq!(fg.distance(0, 2), None);
    }

    #[test]
    #[ignore]
    fn test_functional_graph_random() {
        use rand::prelude::*;
        let mut rng = StdRng::seed_from_u64(42);

        for _ in 0..100 {
            let nv = rng.random_range(1..=50);
            let mut next = vec![0; nv];
            for i in 0..nv {
                next[i] = rng.random_range(0..nv);
            }

            let fg = FunctionalGraph::new(&next);

            for u in 0..nv {
                for v in 0..nv {
                    let expected = find_distance_brute_force(u, v, &next);
                    let actual = fg.distance(u, v);
                    assert_eq!(
                        actual, expected,
                        "distance({}, {}) mismatch. next: {:?}",
                        u, v, next
                    );
                }
            }
        }
    }
}
