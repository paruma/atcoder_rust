use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use acl_scc::*;")]
pub mod acl_scc {
    /// 強連結成分分解 (SCC) を行い、縮約グラフ (DAG) を構築するための構造体
    pub struct SccGraph {
        n: usize,
        pub edges: Vec<(usize, usize)>,
    }

    impl SccGraph {
        /// 頂点数 `n` でグラフを作成する
        pub fn new(n: usize) -> Self {
            Self { n, edges: vec![] }
        }

        /// 辺 `from -> to` を追加する
        pub fn add_edge(&mut self, from: usize, to: usize) {
            self.edges.push((from, to));
        }

        /// SCC を実行し、結果と縮約グラフ (DAG) を返す
        ///
        /// 計算量: O(V + E)
        pub fn scc(self) -> CondensationGraph {
            let mut g = ac_library::SccGraph::new(self.n);
            for &(u, v) in &self.edges {
                g.add_edge(u, v);
            }
            let groups = g.scc();

            // マッピング作成
            let mut mapping = vec![0; self.n];
            for (i, group) in groups.iter().enumerate() {
                for &v in group {
                    mapping[v] = i;
                }
            }

            // DAG作成 (重複辺は除去)
            let mut adj = vec![vec![]; groups.len()];
            for &(u, v) in &self.edges {
                let u_group = mapping[u];
                let v_group = mapping[v];
                if u_group != v_group {
                    adj[u_group].push(v_group);
                }
            }
            for list in &mut adj {
                list.sort_unstable();
                list.dedup();
            }

            CondensationGraph {
                groups,
                mapping,
                adj,
            }
        }
    }

    /// SCC の結果と縮約グラフ (DAG) を保持する構造体
    #[derive(Debug, Clone)]
    pub struct CondensationGraph {
        /// 強連結成分のリスト (トポロジカル順)
        pub groups: Vec<Vec<usize>>,
        /// 各頂点が属するグループ番号 (0-indexed)
        pub mapping: Vec<usize>,
        /// 縮約グラフの隣接リスト (DAG)
        /// `adj[i]` はグループ `i` から遷移可能なグループのリスト
        pub adj: Vec<Vec<usize>>,
    }

    impl CondensationGraph {
        /// グループ数 (縮約グラフの頂点数) を返す
        pub fn group_count(&self) -> usize {
            self.groups.len()
        }
    }
}

pub use acl_scc::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scc_usize() {
        // 0 -> 1 -> 2
        //      ^----|
        let mut graph = SccGraph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 1);

        let cg = graph.scc();

        // SCCの結果確認
        // トポロジカル順なので、0 -> {1, 2} の順になるはず
        assert_eq!(cg.group_count(), 2);

        let g0 = cg.mapping[0];
        let g1 = cg.mapping[1];
        let g2 = cg.mapping[2];

        assert_eq!(g1, g2); // 1と2は同じグループ
        assert_ne!(g0, g1); // 0は別のグループ

        // 0のグループが先に来るはず
        assert!(g0 < g1);

        // DAGの確認: g0 -> g1
        assert!(cg.adj[g0].contains(&g1));
        assert!(cg.adj[g1].is_empty());
    }
}
