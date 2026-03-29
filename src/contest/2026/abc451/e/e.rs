// 問題文と制約は読みましたか？
// #[fastout]
fn main() {
    input! {
        n: usize,
    }

    let dists = {
        let mut dists = vec![vec![i64::MAX; n]; n];

        for i in 0..n - 1 {
            for j in (i + 1)..n {
                // 最初
                input! {
                    x: i64,
                }
                dists[i][j] = x;
            }
        }
        dists
    };

    let mut es: Vec<(usize, usize, i64)> = vec![];
    let mut dsu = DsuCore::new(n);

    for (i, j, d) in (0..n - 1)
        .flat_map(|i| ((i + 1)..n).map(move |j| (i, j)))
        .map(|(i, j)| (i, j, dists[i][j]))
        .sorted_by_key(|(_, _, d)| *d)
    {
        if !dsu.same(i, j) {
            dsu.merge(i, j);
            es.push((i, j, d));
        }
    }

    let adj1 = es
        .iter()
        .copied()
        .fold(vec![vec![]; n], |mut acc, (u, v, _)| {
            acc[u].push(v);
            acc[v].push(u);
            acc
        });

    let adj2 = es
        .iter()
        .copied()
        .fold(vec![vec![]; n], |mut acc, (u, v, c)| {
            acc[u].push((v, c));
            acc[v].push((u, c));
            acc
        });

    let lca = Lca::new(&adj1, 0);

    let dist_from0 = {
        let children = make_tree_children(&adj2, 0);
        let ord = dfs_pre_order(&adj1, 0);
        let mut dp = vec![i64::MAX; n];
        dp[0] = 0;

        for cur in ord {
            for &(child, cost) in &children[cur] {
                dp[child] = dp[cur] + cost;
            }
        }
        dp
    };

    let is_ok = (|| {
        for i in 0..n - 1 {
            for j in (i + 1)..n {
                let lca = lca.lca(i, j);

                let expected = dists[i][j];
                let actual = dist_from0[i] + dist_from0[j] - 2 * dist_from0[lca];
                if expected != actual {
                    return false;
                }
            }
        }
        true
    })();
    print_yesno(is_ok);
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

use lca::*;
#[allow(clippy::module_inception)]
pub mod lca {
    use ac_library::{Monoid, Segtree};
    use std::convert::Infallible;
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct MinI64Usize(Infallible);
    impl Monoid for MinI64Usize {
        type S = (i64, usize);
        fn identity() -> Self::S {
            (i64::MAX, usize::MAX)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            std::cmp::min(*a, *b)
        }
    }
    #[derive(Clone)]
    pub struct Lca {
        dist: Vec<i64>,
        euler_tour_dist: Segtree<MinI64Usize>,
        euler_tour_in_time: Vec<usize>,
        #[allow(dead_code)]
        euler_tour_out_time: Vec<usize>,
    }
    impl Lca {
        /// LCA (Lowest Common Ancestor) を構築する。
        /// # Arguments
        /// * `adj` - 木の隣接リスト (無向グラフまたは、親から子への有向グラフ)
        /// * `root` - 根の頂点番号
        /// # 計算量
        /// O(V) (V は頂点数)
        pub fn new(adj: &[Vec<usize>], root: usize) -> Self {
            let nv = adj.len();
            let dist = {
                fn dfs(dist: &mut [i64], current: usize, adj: &[Vec<usize>], parent: usize) {
                    for &child in &adj[current] {
                        if child == parent {
                            continue;
                        }
                        dist[child] = dist[current] + 1;
                        dfs(dist, child, adj, current);
                    }
                }
                let mut dist = vec![0; nv];
                dfs(&mut dist, root, adj, root);
                dist
            };
            let (euler_tour, euler_tour_in_time, euler_tour_out_time) = {
                fn dfs(
                    tour: &mut Vec<usize>,
                    in_time: &mut [usize],
                    out_time: &mut [usize],
                    current: usize,
                    adj: &[Vec<usize>],
                    parent: usize,
                ) {
                    in_time[current] = in_time[current].min(tour.len());
                    out_time[current] = out_time[current].max(tour.len());
                    tour.push(current);
                    for &child in &adj[current] {
                        if child == parent {
                            continue;
                        }
                        dfs(tour, in_time, out_time, child, adj, current);
                        in_time[current] = in_time[current].min(tour.len());
                        out_time[current] = out_time[current].max(tour.len());
                        tour.push(current);
                    }
                }
                let mut tour = vec![];
                let mut in_time = vec![usize::MAX; nv];
                let mut out_time = vec![usize::MIN; nv];
                dfs(&mut tour, &mut in_time, &mut out_time, root, adj, root);
                (tour, in_time, out_time)
            };
            let euler_tour_dist = Segtree::<MinI64Usize>::from(
                euler_tour
                    .iter()
                    .copied()
                    .map(|v| (dist[v], v))
                    .collect::<Vec<(i64, usize)>>(),
            );
            Lca {
                dist,
                euler_tour_dist,
                euler_tour_in_time,
                euler_tour_out_time,
            }
        }
        /// u と v の LCA を求める
        /// # 計算量
        /// O(log V)
        pub fn lca(&self, u: usize, v: usize) -> usize {
            let (time_min, time_max) = {
                use std::cmp::{max, min};
                let t1 = self.euler_tour_in_time[u];
                let t2 = self.euler_tour_in_time[v];
                (min(t1, t2), max(t1, t2))
            };
            self.euler_tour_dist.prod(time_min..=time_max).1
        }
        /// 頂点 u と v の距離を求めます。
        /// # 計算量
        /// O(log V)
        pub fn dist(&self, u: usize, v: usize) -> i64 {
            self.dist[u] + self.dist[v] - 2 * self.dist[self.lca(u, v)]
        }
        /// パス u-v 上に点 a があるかどうかを判定します。
        /// # 計算量
        /// O(log V)
        pub fn is_path_on(&self, u: usize, v: usize, a: usize) -> bool {
            self.dist(u, a) + self.dist(a, v) == self.dist(u, v)
        }
    }
}
/// 根付き木の隣接リスト `adj` と根 `root` から、各頂点の子頂点リストを求めます。
/// # 計算量
/// O(V + E)
pub fn make_tree_children(adj: &[Vec<(usize, i64)>], root: usize) -> Vec<Vec<(usize, i64)>> {
    let n = adj.len();
    let mut children = vec![vec![]; n];
    let mut visited = vec![false; n];
    let mut queue = std::collections::VecDeque::new();
    visited[root] = true;
    queue.push_back(root);
    while let Some(v) = queue.pop_front() {
        for &(u, c) in &adj[v] {
            if !visited[u] {
                visited[u] = true;
                children[v].push((u, c));
                queue.push_back(u);
            }
        }
    }
    children
}
/// 深さ優先探索 (DFS) を行い、行きがけ順 (pre-order) での頂点順序を返します。
/// # 計算量
/// O(V + E)
pub fn dfs_pre_order(adj: &[Vec<usize>], init: usize) -> Vec<usize> {
    fn dfs(
        adj: &[Vec<usize>],
        current: usize,
        visited: &mut Vec<bool>,
        pre_order: &mut Vec<usize>,
    ) {
        visited[current] = true;
        pre_order.push(current);
        for &next in &adj[current] {
            if !visited[next] {
                dfs(adj, next, visited, pre_order);
            }
        }
    }
    let nv = adj.len();
    let mut visited = vec![false; nv];
    let mut pre_order = vec![];
    dfs(adj, init, &mut visited, &mut pre_order);
    pre_order
}
