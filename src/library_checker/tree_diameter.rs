// 問題: https://judge.yosupo.jp/problem/tree_diameter
/// 木の直径を求める(直径の長さと直径を構成する頂点のリストを返す)
/// edges: 辺の情報 (頂点, 頂点, コスト) のリスト
/// 計算量: O(n) (n が頂点の数のとき)
pub fn tree_diameter(edges: &[(usize, usize, i64)]) -> (i64, Vec<usize>) {
    let nv = edges.len() + 1;
    let adj = edges
        .iter()
        .copied()
        .fold(vec![vec![]; nv], |mut acc, (u, v, cost)| {
            acc[u].push((v, cost));
            acc[v].push((u, cost));
            acc
        });
    // init から最も遠い点までの距離と、init から最も遠い点までいくのに訪問する頂点のリストを返す
    fn bfs(adj: &[Vec<(usize, i64)>], init: usize) -> (i64, Vec<usize>) {
        let n = adj.len(); // 頂点の数

        let mut dist = vec![0; n];
        let mut prev = vec![None; n];
        let mut visited = vec![false; n];
        let mut open = Queue::new();
        open.push(init);
        visited[init] = true;

        while let Some(current) = open.pop() {
            for &(next, cost) in &adj[current] {
                if !visited[next] {
                    dist[next] = dist[current] + cost;
                    prev[next] = Some(current);
                    visited[next] = true;
                    open.push(next);
                }
            }
        }
        let (furthest, max_dist) = dist
            .iter()
            .copied()
            .enumerate()
            .max_by_key(|(_, d)| *d)
            .unwrap();
        // 復元
        let path: Vec<usize> = {
            let mut path: Vec<usize> =
                std::iter::successors(Some(furthest), |&i| prev[i]).collect();
            path.reverse();
            path
        };

        (max_dist, path)
    }

    // 頂点 0 から最も遠い点 x を求める
    let x = *bfs(&adj, 0).1.last().unwrap();

    // 頂点 x から最も遠い点 y までの距離を求める
    bfs(&adj, x)
}
fn main() {
    input! {
        nv: usize,
        edges: [(usize, usize, i64); nv-1],
    }

    let (diam_len, diam_path) = tree_diameter(&edges);
    println!("{} {}", diam_len, diam_path.len());
    print_vec_1line(&diam_path);
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

// ---  library ---
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
            self.raw.push_front(value)
        }
        pub fn pop(&mut self) -> Option<T> {
            self.raw.pop_back()
        }
        pub fn peek(&self) -> Option<&T> {
            self.raw.back()
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

// -------- 出力 --------

pub fn print_vec_1line<T: std::fmt::Debug>(arr: &[T]) {
    let msg = arr
        .iter()
        .map(|x| format!("{:?}", x))
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", msg);
}
