use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use acl_scc::*;")]
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

#[cfg(test)]
mod tests {
    use super::acl_scc::*;

    #[test]
    fn test_scc_usize() {
        // 0 -> 1 -> 2
        //      ^----|
        let mut graph = SccGraphWrapper::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 1);

        let cg = graph.scc();

        // SCCの結果確認
        // トポロジカル順なので、0 -> {1, 2} の順になるはず
        assert_eq!(cg.group_count(), 2);
        assert_eq!(cg.groups().len(), 2);

        let v_to_g = cg.vertex_to_group();
        let g0 = v_to_g[0];
        let g1 = v_to_g[1];
        let g2 = v_to_g[2];

        assert_eq!(g1, g2); // 1と2は同じグループ
        assert_ne!(g0, g1); // 0は別のグループ

        // 0のグループが先に来るはず
        assert!(g0 < g1);

        // DAGの確認: g0 -> g1
        let adj = cg.condensation_adj();
        assert!(adj[g0].contains(&g1));
        assert!(adj[g1].is_empty());
    }
}
