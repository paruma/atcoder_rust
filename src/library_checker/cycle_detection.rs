// https://judge.yosupo.jp/problem/cycle_detection

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
            history: &mut Vec<EdgeIndex>,
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

        // 履歴からサイクルのみを抽出する
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
                    return Some(self.construct_cycle(vertex_on_cycle, &history));
                }
            }
            None
        }
    }

    /// 与えられた有向グラフにサイクルが存在するか判定して、存在したらサイクル上の点を返す
    ///
    /// 有向グラフは以下の２つの情報で与えられる
    /// * `nv`: 頂点の数 `nv`
    /// * `edges`: 辺のリスト。辺は始点と終点のペアで与えられる
    pub fn cycle_detection(nv: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
        CycleDetectionSolver::new(nv, edges).solve()
    }
}

fn main() {
    use cycle_detection::cycle_detection;
    input! {
        nv: usize,
        ne: usize,
        edges: [(usize, usize); ne],
    }

    let ans = cycle_detection(nv, &edges);
    if let Some(ans) = ans {
        println!("{}", ans.len());
        for x in ans {
            println!("{}", x);
        }
    } else {
        println!("-1");
    }
}

// ------------------入力------------------
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8 からコピー
#[macro_export]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

#[macro_export]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[macro_export]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}
