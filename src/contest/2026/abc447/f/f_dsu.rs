// 解法: ムカデグラフの脊椎パスの内部頂点は次数≥4、両端は次数≥3 が必要。
// 次数≥4 の頂点を DSU でまとめ、隣接する次数3の頂点も加えた部分木の直径+1 を主要候補とする。
// 次数≥3 同士が隣接する辺（長さ2ムカデ）や次数≥2 頂点の存在（長さ1）も候補に加えた最大値が答え。

fn tree_diam(adj: &[Vec<usize>]) -> i64 {
    fn bfs(adj: &[Vec<usize>], init: usize) -> (i64, Vec<usize>) {
        let n = adj.len();
        let mut dist = vec![0; n];
        let mut prev = vec![None; n];
        let mut visited = vec![false; n];
        let mut open = Queue::new();
        open.push(init);
        visited[init] = true;
        while let Some(current) = open.pop() {
            for &next in &adj[current] {
                if !visited[next] {
                    dist[next] = dist[current] + 1;
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
        let path: Vec<usize> = {
            let mut path: Vec<usize> =
                std::iter::successors(Some(furthest), |&i| prev[i]).collect();
            path.reverse();
            path
        };
        (max_dist, path)
    }
    let x = *bfs(&adj, 0).1.last().unwrap();
    bfs(&adj, x).0
}

#[fastout]
fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        input! {
            nv: usize,
            es: [(Usize1, Usize1); nv-1],
        }

        let adj = es
            .iter()
            .copied()
            .fold(vec![vec![]; nv], |mut acc, (u, v)| {
                acc[u].push(v);
                acc[v].push(u);
                acc
            });

        let degs = adj.iter().map(|nexts| nexts.len()).collect_vec();
        let mut dsu = DsuCore::new(nv);

        for &(u, v) in &es {
            if degs[u] >= 4 && degs[v] >= 4 {
                dsu.merge(u, v);
            }
        }
        let cand1: i64 = dsu
            .groups()
            .iter()
            .filter(|g| degs[g[0]] >= 4)
            .map(|g| {
                let mut g2 = g.clone();
                // gの隣にある次数3を追加する
                for &v in g {
                    for &next in &adj[v] {
                        if degs[next] == 3 {
                            g2.push(next);
                        }
                    }
                }
                g2.sort();
                // g2 圧縮する
                // adj_g2

                let mut adj_g2 = vec![vec![]; g2.len()];

                for &v in &g2 {
                    let v_cc = g2.binary_search(&v).unwrap();

                    for &next in &adj[v] {
                        if let Ok(next_cc) = g2.binary_search(&next) {
                            adj_g2[v_cc].push(next_cc);
                            // 逆方向はいらないはず。別のループで生える
                        }
                    }
                }

                tree_diam(&adj_g2) + 1
            })
            .max()
            .unwrap_or(0);

        // 長さ2のムカデグラフ
        let cand2 = {
            let mut dsu = DsuCore::new(nv);
            for &(u, v) in &es {
                if degs[u] >= 3 && degs[v] >= 3 {
                    dsu.merge(u, v);
                }
            }
            if dsu.groups().iter().any(|g| g.len() >= 2) {
                2
            } else {
                0
            }
        };

        // 長さ1のムカデグラフ
        let cand3 = {
            //
            if nv >= 3 { 1 } else { 0 }
        };
        let ans = [cand1, cand2, cand3].iter().copied().max().unwrap();
        println!("{}", ans);
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }

    /// 間違っていたら false を返す
    fn process_one_test(rng: &mut SmallRng) -> bool {
        // ==== 問題を作る ====
        let n = rng.random_range(1..=10);
        let xs = (0..n).map(|_| rng.random_range(0..10)).collect_vec();

        // ==== 解く ====
        let main_ans = xs.len();
        let naive_ans = 1;

        // ==== 間違っていたら報告をする ====
        if main_ans != naive_ans {
            // 問題を出力
            println!("{:?}", (n, xs));
            println!("main ans : {:?}", main_ans);
            println!("naive ans: {:?}", naive_ans);
            return false;
        }
        true
    }

    #[allow(unreachable_code)]
    #[test]
    #[ignore]
    fn test_with_naive() {
        let num_tests = 100;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut cnt_wrong = 0;
        let mut rng = SmallRng::seed_from_u64(42);
        // let mut rng = SmallRng::from_os_rng();
        for _ in 0..num_tests {
            let is_ok = process_one_test(&mut rng);
            if !is_ok {
                cnt_wrong += 1;
            }
            if cnt_wrong >= max_wrong_case {
                break;
            }
        }
        if cnt_wrong > 0 {
            println!("{} cases are wrong.", cnt_wrong);
            panic!();
        }
    }
}

// ====== import ======
#[allow(unused_imports)]
use {
    itertools::{Itertools, chain, iproduct, izip},
    proconio::{
        derive_readable, fastout, input,
        marker::{Bytes, Chars, Usize1},
    },
    rand::{Rng, SeedableRng, rngs::SmallRng, seq::SliceRandom},
    std::{
        cmp::Reverse,
        collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet},
    },
};

// ====== output func ======
#[allow(unused_imports)]
use print_util::*;
pub mod print_util {
    use itertools::Itertools;
    use proconio::fastout;

    #[fastout]
    pub fn print_vec<T: std::fmt::Display>(arr: &[T]) {
        for a in arr {
            println!("{}", a);
        }
    }

    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
        println!("{}", arr.iter().join(" "));
    }

    #[fastout]
    pub fn print_vec2<T: std::fmt::Display, R: AsRef<[T]>>(arr: &[R]) {
        for row in arr {
            println!("{}", row.as_ref().iter().join(" "));
        }
    }

    pub fn print_bytes(bytes: &[u8]) {
        println!("{}", std::str::from_utf8(bytes).unwrap());
    }

    pub fn print_chars(chars: &[char]) {
        println!("{}", chars.iter().collect::<String>());
    }

    #[fastout]
    pub fn print_vec_bytes<R: AsRef<[u8]>>(vec_bytes: &[R]) {
        for row in vec_bytes {
            println!("{}", std::str::from_utf8(row.as_ref()).unwrap());
        }
    }

    #[fastout]
    pub fn print_vec_chars<R: AsRef<[char]>>(vec_chars: &[R]) {
        for row in vec_chars {
            println!("{}", row.as_ref().iter().collect::<String>());
        }
    }

    pub fn print_yesno(ans: bool) {
        println!("{}", if ans { "Yes" } else { "No" });
    }
}

// ====== snippet ======
use dsu_core::*;
#[allow(clippy::module_inception)]
/// ac_library::Dsu の merge のみ実装を変えたもの
pub mod dsu_core {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    /// DSU 内の各要素の状態（親のインデックスまたは集合のサイズ）を保持する構造体。
    /// メモリ効率（32ビット整数 1 つ分）を維持したまま、以下の 2 つの状態を表現します。
    /// 1. **Root (根)**:
    ///    - 値が負の場合、その要素は集合の代表元（リーダー）です。
    ///    - 値の絶対値 `|v|` は、その集合に属する要素の数（サイズ）を表します。
    ///    - 例: `-1` はサイズ 1 の集合の根、`-5` はサイズ 5 の集合の根。
    /// 2. **Child (子)**:
    ///    - 値が 0 以上の場合、その要素は他の要素を親に持っています。
    ///    - 値 `v` は、親要素のインデックスを表します。
    struct Node(i32);
    impl Node {
        fn root(size: usize) -> Self {
            Self(-(size as i32))
        }
        fn child(parent: usize) -> Self {
            Self(parent as i32)
        }
        fn is_root(&self) -> bool {
            self.0 < 0
        }
        fn parent(&self) -> usize {
            self.0 as usize
        }
        fn size(&self) -> usize {
            (-self.0) as usize
        }
    }
    #[derive(Clone, Debug)]
    pub struct DsuCore {
        n: usize,
        nodes: Vec<Node>,
        cnt_groups: usize,
    }
    impl DsuCore {
        pub fn new(size: usize) -> Self {
            Self {
                n: size,
                nodes: vec![Node::root(1); size],
                cnt_groups: size,
            }
        }
        /// 2 つの要素 `a` と `b` が属する集合を統合する
        /// # 戻り値
        /// - `Some((leader, merged))`:
        ///   - `leader` は統合後の集合の代表元（リーダー）
        ///   - `merged` は統合されて消える側の旧代表元
        /// - `None`:
        ///   - `a` と `b` がすでに同じ集合に属していた場合
        pub fn merge(&mut self, a: usize, b: usize) -> Option<(usize, usize)> {
            assert!(a < self.n);
            assert!(b < self.n);
            let (mut x, mut y) = (self.leader(a), self.leader(b));
            if x == y {
                return None;
            }
            if self.nodes[x].size() < self.nodes[y].size() {
                std::mem::swap(&mut x, &mut y);
            }
            let size_x = self.nodes[x].size();
            let size_y = self.nodes[y].size();
            self.nodes[x] = Node::root(size_x + size_y);
            self.nodes[y] = Node::child(x);
            self.cnt_groups -= 1;
            Some((x, y))
        }
        pub fn same(&mut self, a: usize, b: usize) -> bool {
            assert!(a < self.n);
            assert!(b < self.n);
            self.leader(a) == self.leader(b)
        }
        pub fn leader(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            if self.nodes[a].is_root() {
                return a;
            }
            let parent = self.nodes[a].parent();
            let new_parent = self.leader(parent);
            self.nodes[a] = Node::child(new_parent);
            new_parent
        }
        pub fn size(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            let x = self.leader(a);
            self.nodes[x].size()
        }
        pub fn count_group(&self) -> usize {
            self.cnt_groups
        }
        pub fn groups(&mut self) -> Vec<Vec<usize>> {
            let mut leader_buf = vec![0; self.n];
            let mut group_size = vec![0; self.n];
            for i in 0..self.n {
                leader_buf[i] = self.leader(i);
                group_size[leader_buf[i]] += 1;
            }
            let mut result = vec![Vec::new(); self.n];
            for i in 0..self.n {
                result[i].reserve(group_size[i]);
            }
            for i in 0..self.n {
                result[leader_buf[i]].push(i);
            }
            result
                .into_iter()
                .filter(|x| !x.is_empty())
                .collect::<Vec<Vec<usize>>>()
        }
    }
}
use mod_queue::*;
/// 木の直径を求める(直径の長さと直径を構成する頂点のリストを返す)
/// # Arguments
/// * `edges` - 辺の情報 (頂点, 頂点, コスト) のリスト
/// # Returns
/// `(直径の長さ, 直径を構成する頂点のリスト)`
/// # 計算量
/// O(V) (V は頂点数)
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
    fn bfs(adj: &[Vec<(usize, i64)>], init: usize) -> (i64, Vec<usize>) {
        let n = adj.len();
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
        let path: Vec<usize> = {
            let mut path: Vec<usize> =
                std::iter::successors(Some(furthest), |&i| prev[i]).collect();
            path.reverse();
            path
        };
        (max_dist, path)
    }
    let x = *bfs(&adj, 0).1.last().unwrap();
    bfs(&adj, x)
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
/// 重みなし木の直径を求める(直径の長さと直径を構成する頂点のリストを返す)
/// # Arguments
/// * `edges` - 辺の情報 (頂点, 頂点) のリスト
/// # Returns
/// `(直径の長さ, 直径を構成する頂点のリスト)`
/// # 計算量
/// O(V) (V は頂点数)
pub fn tree_diameter_no_weight(edges: &[(usize, usize)]) -> (i64, Vec<usize>) {
    let edges: Vec<(usize, usize, i64)> = edges.iter().copied().map(|(u, v)| (u, v, 1)).collect();
    tree_diameter(&edges)
}
