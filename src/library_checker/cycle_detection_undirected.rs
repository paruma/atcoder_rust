// 問題: https://judge.yosupo.jp/problem/cycle_detection_undirected
// 解法: グラフのサイクル検出 (閉路検出) by DFS - けんちょんの競プロ精進記録 https://drken1215.hatenablog.com/entry/2023/05/20/200517

pub mod cycle_detection_undirected {

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct EdgeIndex {
        src: usize,
        dst: usize,
        idx: usize,
    }

    impl EdgeIndex {
        fn rev(self) -> Self {
            Self {
                src: self.dst,
                dst: self.src,
                idx: self.idx,
            }
        }
    }

    struct CycleDetectionUndirectedSolver {
        nv: usize,
        adj: Vec<Vec<EdgeIndex>>,
    }

    impl CycleDetectionUndirectedSolver {
        fn new(nv: usize, edges: &[(usize, usize)]) -> CycleDetectionUndirectedSolver {
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
                acc[e.dst].push(e.rev());
                acc
            });
            CycleDetectionUndirectedSolver { nv, adj }
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
                if let Some(prev_e) = prev_e {
                    if prev_e.idx == e.idx {
                        //逆流を禁止する
                        continue;
                    }
                }
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
        // サイクルは(頂点の添字列, 辺の添字列) というペアで返す。
        fn construct_cycle(
            &self,
            vertex_on_cycle: usize,
            history: &[EdgeIndex],
        ) -> (Vec<usize>, Vec<usize>) {
            let mut rev_cycle = vec![];
            for e in history.iter().rev() {
                rev_cycle.push(*e);
                if e.src == vertex_on_cycle {
                    break;
                }
            }
            let cycle = {
                rev_cycle.reverse();
                rev_cycle
            };

            let cycle_vertex: Vec<usize> = cycle.iter().copied().map(|e| e.src).collect();
            let cycle_edge: Vec<usize> = cycle.iter().copied().map(|e| e.idx).collect();

            (cycle_vertex, cycle_edge)
        }

        fn solve(&self) -> Option<(Vec<usize>, Vec<usize>)> {
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

    /// 与えられた有向グラフにサイクルが存在するか判定して、存在したらサイクルの情報を返す。
    ///
    /// 有向グラフは以下の２つの情報で与えられる
    /// * `nv`: 頂点の数 `nv`
    /// * `edges`: 辺のリスト。辺は始点と終点のペアで与えられる
    ///
    /// サイクルは (頂点の添字列, 辺の添字列) というペアで返す。
    /// 具体的には以下の条件を満たす頂点の添字列 (v_0, ..., v_{L-1}) と辺の添字列 (e_0, ..., e_{L-1})で返される。
    /// e_i は v_i と v_{i+1} を接続している(i = L のときは i + 1 = 0 として扱う)
    ///
    /// 計算量: O(E+V) (V は頂点の数, E は辺の数)
    pub fn cycle_detection_undirected(
        nv: usize,
        edges: &[(usize, usize)],
    ) -> Option<(Vec<usize>, Vec<usize>)> {
        CycleDetectionUndirectedSolver::new(nv, edges).solve()
    }
}

fn main() {
    use cycle_detection_undirected::cycle_detection_undirected;
    input! {
        nv: usize,
        ne: usize,
        edges: [(usize, usize); ne],
    }

    let ans = cycle_detection_undirected(nv, &edges);
    if let Some((vertices, edges)) = ans {
        println!("{}", vertices.len());

        print_vec_1line(&vertices);
        print_vec_1line(&edges);
    } else {
        println!("-1");
    }
}

// ------------------入力------------------
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8 からコピー
#[macro_export]
macro_rules! input {
    (source = $s:expr_2021, $($r:tt)*) => {
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
    ($next:expr_2021) => {};
    ($next:expr_2021, ) => {};

    ($next:expr_2021, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[macro_export]
macro_rules! read_value {
    ($next:expr_2021, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr_2021, [ $t:tt ; $len:expr_2021 ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr_2021, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr_2021, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr_2021, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

// ------------------出力------------------
pub fn print_vec_1line<T: std::fmt::Debug>(arr: &[T]) {
    let msg = arr
        .iter()
        .map(|x| format!("{:?}", x))
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", msg);
}
