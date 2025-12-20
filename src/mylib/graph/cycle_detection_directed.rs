// 問題: https://judge.yosupo.jp/problem/cycle_detection
// 解法: グラフのサイクル検出 (閉路検出) by DFS - けんちょんの競プロ精進記録 https://drken1215.hatenablog.com/entry/2023/05/20/200517

use cargo_snippet::snippet;

#[snippet]
pub mod cycle_detection {

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct EdgeIndex {
        src: usize,
        dst: usize,
        idx: usize,
    }

    struct CycleDetectionSolver {
        nv: usize,
        adj: Vec<Vec<EdgeIndex>>,
    }

    impl CycleDetectionSolver {
        fn new(nv: usize, edges: &[(usize, usize)]) -> CycleDetectionSolver {
            let edges = edges
                .iter()
                .copied()
                .enumerate()
                .map(|(i, e)| EdgeIndex {
                    src: e.0,
                    dst: e.1,
                    idx: i,
                })
                .collect::<Vec<_>>();
            let adj = edges.iter().copied().fold(vec![vec![]; nv], |mut acc, e| {
                acc[e.src].push(e);
                acc
            });
            CycleDetectionSolver { nv, adj }
        }

        // サイクルがあったらそのサイクル上の点を返す。なければ None を返す
        fn dfs(
            &self,
            current_v: usize,
            prev_e: Option<EdgeIndex>,
            visited_pre: &mut Vec<bool>,
            visited_post: &mut Vec<bool>,
            history: &mut Vec<EdgeIndex>, // 行きがけで追加して帰りがけで削除される
        ) -> Option<usize> {
            visited_pre[current_v] = true;
            if let Some(prev_e) = prev_e {
                assert_eq!(current_v, prev_e.dst);
                history.push(prev_e);
            }

            for e in &self.adj[current_v] {
                if visited_pre[e.dst] && !visited_post[e.dst] {
                    // e.dst が行きがけで訪問済だが帰りがけで未訪問: e.dst から current_v に到達可能(閉路がある)
                    // 逆に e.dst が帰りがけで訪問済の場合は、e.dst から current_v に到達不可能
                    history.push(*e);
                    return Some(e.dst);
                }
                if visited_pre[e.dst] {
                    continue;
                }

                let vertex_on_cycle = self.dfs(e.dst, Some(*e), visited_pre, visited_post, history);

                if vertex_on_cycle.is_some() {
                    return vertex_on_cycle;
                }
            }

            history.pop();
            visited_post[current_v] = true;
            None
        }

        // 履歴 history からからサイクルのみを抽出する
        // v: サイクル検出をした頂点
        fn construct_cycle(&self, vertex_on_cycle: usize, history: &[EdgeIndex]) -> Vec<usize> {
            let mut rev_cycle = vec![];
            for e in history.iter().rev() {
                rev_cycle.push(*e);
                if e.src == vertex_on_cycle {
                    break;
                }
            }
            rev_cycle.iter().copied().rev().map(|e| e.idx).collect()
        }

        fn solve(&self) -> Option<Vec<usize>> {
            let mut visited_pre = vec![false; self.nv];
            let mut visited_post = vec![false; self.nv];
            for start in 0..self.nv {
                if visited_pre[start] {
                    continue;
                }
                let mut history = vec![];
                let vertex_on_cycle = self.dfs(
                    start,
                    None,
                    &mut visited_pre,
                    &mut visited_post,
                    &mut history,
                );
                if let Some(vertex_on_cycle) = vertex_on_cycle {
                    // 閉路が存在する場合、history は以下のようになっている
                    // start → 閉路始まり-(閉路1周)→閉路始まり
                    // 「start → 閉路始まり」の部分を construct_cycle で消去する
                    return Some(self.construct_cycle(vertex_on_cycle, &history));
                }
            }
            None
        }
    }

    /// 与えられた有向グラフにサイクルが存在するか判定して、存在したらサイクル上の点を返す
    ///
    /// 有向グラフは以下の２つの情報で与えられる。
    ///
    /// * `nv`: 頂点の数 `nv`
    /// * `edges`: 辺のリスト。辺は始点と終点のペアで与えられる
    ///
    /// # 計算量
    /// O(E+V) (V は頂点の数, E は辺の数)
    pub fn cycle_detection(nv: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
        CycleDetectionSolver::new(nv, edges).solve()
    }
}

#[cfg(test)]
mod tests {
    // https://judge.yosupo.jp/problem/cycle_detection のサンプルを使ったテスト
    use itertools::Itertools;

    struct TestCase {
        nv: usize,
        edges: Vec<(usize, usize)>,
    }

    fn is_cycle(test_case: &TestCase, cycle: &[usize]) -> bool {
        cycle
            .iter()
            .copied()
            .map(|i| test_case.edges[i])
            .circular_tuple_windows()
            .all(|(e1, e2)| e1.1 == e2.0)
    }

    #[test]
    fn test_cycle_detection1() {
        let t = TestCase {
            nv: 5,
            edges: vec![(0, 3), (0, 4), (4, 2), (4, 3), (4, 0), (2, 1), (1, 0)],
        };

        let actual = super::cycle_detection::cycle_detection(t.nv, &t.edges);
        assert!(actual.is_some());
        assert!(is_cycle(&t, &actual.unwrap()));
    }
    #[test]
    fn test_cycle_detection2() {
        let t = TestCase {
            nv: 2,
            edges: vec![(1, 0)],
        };

        let actual = super::cycle_detection::cycle_detection(t.nv, &t.edges);
        assert!(actual.is_none());
    }

    #[test]
    fn test_cycle_detection3() {
        let t = TestCase {
            nv: 4,
            edges: vec![(0, 1), (1, 2), (2, 0), (0, 1), (1, 3), (3, 0)],
        };

        let actual = super::cycle_detection::cycle_detection(t.nv, &t.edges);
        assert!(actual.is_some());
        assert!(is_cycle(&t, &actual.unwrap()));
    }

    #[test]
    fn test_cycle_detection_no_cycle_linear() {
        // 0 -> 1 -> 2
        // start=0 で 0, 1, 2 が訪問済みになる。
        // start=1, 2 のループで continue が呼ばれる。
        let t = TestCase {
            nv: 3,
            edges: vec![(0, 1), (1, 2)],
        };

        let actual = super::cycle_detection::cycle_detection(t.nv, &t.edges);
        assert!(actual.is_none());
    }
}
