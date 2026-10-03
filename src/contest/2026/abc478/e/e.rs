// 問題文と制約は読みましたか？
// #[fastout]
fn solve(n: usize, nq: usize, tuvs: &[(usize, usize, usize)]) -> Option<Vec<i64>> {
    let mut scc = SccGraphWrapper::new(n);
    for &(_t, u, v) in tuvs {
        scc.add_edge(v, u);
    }

    let group_graph = scc.scc();

    let v_to_g = group_graph.vertex_to_group();
    // 同じグループの中に t=1 の辺があったらだめ

    for &(t, u, v) in tuvs {
        if v_to_g[u] == v_to_g[v] && t == 1 {
            return None;
        }
    }

    let group_cnt = group_graph.group_count();
    let mut edges = HashMap::<(usize, usize), usize>::new();

    for &(t, u, v) in tuvs {
        let ug = v_to_g[u];
        let vg = v_to_g[v];
        if ug != vg {
            let e = edges.entry((ug, vg)).or_insert(0);
            if t == 1 {
                *e = 1;
            }
        }
    }

    let mut adj = vec![vec![]; group_cnt];

    for ((ug, vg), t) in edges {
        adj[vg].push((ug, t as i64));
    }

    let mut ans_group = vec![n as i64; group_cnt];

    for from in 0..group_cnt {
        for &(to, t) in &adj[from] {
            ans_group[to] = ans_group[to].min(ans_group[from] - t);
        }
    }

    let mut ans = vec![0; n];
    for i in 0..n {
        ans[i] = ans_group[v_to_g[i]];
    }

    Some(ans)
}

fn main() {
    input! {
        n: usize,
        nq: usize,
        tuvs: [(usize, Usize1, Usize1); nq],
    }

    let ans = solve(n, nq, &tuvs);

    if let Some(ans) = ans {
        println!("Yes");
        println!("{}", ans.iter().join(" "));
    } else {
        println!("No");
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

use ac_library::SccGraph;
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
use acl_scc::*;
#[allow(clippy::module_inception)]
pub mod acl_scc {
    /// 強連結成分分解 (SCC) を行い、縮約グラフ (DAG) を構築するための構造体
    #[derive(Clone, Debug)]
    pub struct SccGraphWrapper {
        nv: usize,
        edges: Vec<(usize, usize)>,
    }
    impl SccGraphWrapper {
        /// 頂点数 `nv` でグラフを作成する
        ///
        /// # Arguments
        /// * `nv` - 頂点数
        pub fn new(nv: usize) -> Self {
            Self { nv, edges: vec![] }
        }
        /// 辺 `from -> to` を追加する
        ///
        /// # Arguments
        /// * `from` - 始点
        /// * `to` - 終点
        pub fn add_edge(&mut self, from: usize, to: usize) {
            self.edges.push((from, to));
        }
        /// SCC を実行し、結果を保持する `CondensationGraph` を返す
        ///
        /// # Returns
        /// SCC の結果を含む `CondensationGraph`
        ///
        /// # 計算量
        /// O(V + E)
        pub fn scc(self) -> CondensationGraph {
            let mut scc_graph = ac_library::SccGraph::new(self.nv);
            for &(u, v) in &self.edges {
                scc_graph.add_edge(u, v);
            }
            let groups = scc_graph.scc();
            CondensationGraph {
                groups,
                original_edges: self.edges,
                original_nv: self.nv,
            }
        }
    }
    /// SCC の結果を保持し、必要に応じて縮約グラフなどを提供する構造体
    #[derive(Debug, Clone)]
    pub struct CondensationGraph {
        /// 強連結成分のリスト (トポロジカル順)
        groups: Vec<Vec<usize>>,
        original_edges: Vec<(usize, usize)>,
        original_nv: usize,
    }
    impl CondensationGraph {
        /// 強連結成分のリストを返す (トポロジカル順)
        pub fn groups(&self) -> &[Vec<usize>] {
            &self.groups
        }
        /// 強連結成分の個数 (縮約グラフの頂点数) を返す
        pub fn group_count(&self) -> usize {
            self.groups.len()
        }
        /// 各頂点が属するグループ番号 (0-indexed) への対応表を返す
        ///
        /// # Returns
        /// 戻り値を `v_to_g` としたとき、`v_to_g[v]` は頂点 `v` が属するグループのインデックス
        ///
        /// # 計算量
        /// O(V)
        pub fn vertex_to_group(&self) -> Vec<usize> {
            let mut v_to_g = vec![0; self.original_nv];
            for (i, group) in self.groups.iter().enumerate() {
                for &v in group {
                    v_to_g[v] = i;
                }
            }
            v_to_g
        }
        /// 縮約グラフの辺のリストを返す (重複辺と自己ループは除去される)
        ///
        /// # Returns
        /// 縮約後の頂点 (グループID) 間の辺のリスト
        ///
        /// # 計算量
        /// O(V + E log E)
        pub fn condensation_edges(&self) -> Vec<(usize, usize)> {
            let v_to_g = self.vertex_to_group();
            let mut edges = vec![];
            for &(u, v) in &self.original_edges {
                let u_group = v_to_g[u];
                let v_group = v_to_g[v];
                if u_group != v_group {
                    edges.push((u_group, v_group));
                }
            }
            edges.sort_unstable();
            edges.dedup();
            edges
        }
        /// 縮約グラフの隣接リストを返す (重複辺は除去される)
        ///
        /// # Returns
        /// 縮約後の頂点 (グループID) をインデックスとする隣接リスト
        ///
        /// # 計算量
        /// O(V + E log E)
        pub fn condensation_adj(&self) -> Vec<Vec<usize>> {
            let mut adj = vec![vec![]; self.group_count()];
            for (u_g, v_group) in self.condensation_edges() {
                adj[u_g].push(v_group);
            }
            adj
        }
    }
}
