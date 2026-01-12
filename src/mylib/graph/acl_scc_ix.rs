use crate::data_structure::ix::{Bounds, Ix, IxVec};
use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use acl_scc_ix::*;")]
pub mod acl_scc_ix {
    use super::{Bounds, Ix, IxVec};

    /// 強連結成分分解 (SCC) を行い、縮約グラフ (DAG) を構築するための構造体 (Ix版)
    #[derive(Clone, Debug)]
    pub struct SccGraphIxWrapper<I: Ix> {
        bounds: Bounds<I>,
        edges: Vec<(usize, usize)>,
    }

    impl<I: Ix> SccGraphIxWrapper<I> {
        /// 指定された範囲の頂点を持つグラフを作成する
        ///
        /// # Arguments
        /// * `bounds` - 頂点のインデックス範囲
        pub fn new(bounds: Bounds<I>) -> Self {
            Self {
                bounds,
                edges: vec![],
            }
        }

        /// 辺 `from -> to` を追加する
        ///
        /// # Arguments
        /// * `from` - 始点
        /// * `to` - 終点
        pub fn add_edge(&mut self, from: I, to: I) {
            let from_idx = self.bounds.to_index(from);
            let to_idx = self.bounds.to_index(to);
            self.edges.push((from_idx, to_idx));
        }

        /// SCC を実行し、結果を保持する `CondensationGraphIx` を返す
        ///
        /// # Returns
        /// SCC の結果を含む `CondensationGraphIx`
        ///
        /// # 計算量
        /// O(V + E)
        pub fn scc(self) -> CondensationGraphIx<I> {
            let n = self.bounds.range_size();
            let mut scc_graph = ac_library::SccGraph::new(n);
            for &(u, v) in &self.edges {
                scc_graph.add_edge(u, v);
            }
            let groups_indices = scc_graph.scc();

            // groups を I のベクタに変換
            let groups: Vec<Vec<I>> = groups_indices
                .into_iter()
                .map(|group| {
                    group
                        .into_iter()
                        .map(|idx| self.bounds.from_index(idx))
                        .collect()
                })
                .collect();

            CondensationGraphIx {
                groups,
                bounds: self.bounds,
                original_edges: self.edges,
            }
        }
    }

    /// SCC の結果を保持し、必要に応じて縮約グラフなどを提供する構造体 (Ix版)
    #[derive(Debug, Clone)]
    pub struct CondensationGraphIx<I: Ix> {
        /// 強連結成分のリスト (トポロジカル順)
        groups: Vec<Vec<I>>,
        bounds: Bounds<I>,
        original_edges: Vec<(usize, usize)>,
    }

    impl<I: Ix> CondensationGraphIx<I> {
        /// 強連結成分のリストを返す (トポロジカル順)
        pub fn groups(&self) -> &[Vec<I>] {
            &self.groups
        }

        /// グループ数 (縮約グラフの頂点数) を返す
        ///
        /// # Returns
        /// 強連結成分の個数
        pub fn group_count(&self) -> usize {
            self.groups.len()
        }

        /// 各頂点が属するグループ番号 (0-indexed) へのマッピングを返す
        ///
        /// # Returns
        /// `mapping[v]` は頂点 `v` が属するグループのインデックス
        ///
        /// # 計算量
        /// O(V)
        pub fn mapping(&self) -> IxVec<I, usize> {
            let mut mapping_vec = vec![0; self.bounds.range_size()];
            for (i, group) in self.groups.iter().enumerate() {
                for &v in group {
                    mapping_vec[self.bounds.to_index(v)] = i;
                }
            }
            IxVec::from_vec(self.bounds, mapping_vec)
        }

        /// 縮約グラフの辺のリストを返す (重複辺と自己ループは除去される)
        ///
        /// # Returns
        /// 縮約後の頂点 (グループID) 間の辺のリスト
        ///
        /// # 計算量
        /// O(V + E log E)
        pub fn condensation_edges(&self) -> Vec<(usize, usize)> {
            let mut mapping_vec = vec![0; self.bounds.range_size()];
            for (i, group) in self.groups.iter().enumerate() {
                for &v in group {
                    mapping_vec[self.bounds.to_index(v)] = i;
                }
            }

            let mut edges = vec![];
            for &(u_idx, v_idx) in &self.original_edges {
                let u_group = mapping_vec[u_idx];
                let v_group = mapping_vec[v_idx];
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
    use super::acl_scc_ix::*;
    use crate::data_structure::ix::Bounds;

    #[test]
    fn test_scc_ix() {
        // 0 -> 1 -> 0
        let bounds = Bounds::new(0, 1);
        let mut scc_graph = SccGraphIxWrapper::new(bounds);
        scc_graph.add_edge(0, 1);
        scc_graph.add_edge(1, 0);

        let cg = scc_graph.scc();

        assert_eq!(cg.group_count(), 1);
        assert_eq!(cg.groups().len(), 1);
        let mapping = cg.mapping();
        assert_eq!(mapping[0], 0);
        assert_eq!(mapping[1], 0);
        assert!(cg.condensation_adj()[0].is_empty());
    }

    #[test]
    fn test_scc_ix_grid() {
        // (0,0) -> (0,1) -> (0,0)
        // (0,0) -> (1,0)
        let bounds = Bounds::new((0, 0), (1, 1));
        let mut scc_graph = SccGraphIxWrapper::new(bounds);
        scc_graph.add_edge((0, 0), (0, 1));
        scc_graph.add_edge((0, 1), (0, 0));
        scc_graph.add_edge((0, 0), (1, 0));

        let cg = scc_graph.scc();

        // 期待: {(0,0), (0,1)} -> {(1,0)}, {(1,1)} (孤立)
        assert_eq!(cg.group_count(), 3);

        let mapping = cg.mapping();
        let g_u = mapping[(0, 0)];
        let g_v = mapping[(1, 0)];
        let g_isolated = mapping[(1, 1)];

        assert_eq!(mapping[(0, 1)], g_u);
        assert_ne!(g_u, g_v);
        assert_ne!(g_u, g_isolated);
        assert_ne!(g_v, g_isolated);

        // トポロジカル順なら g_u < g_v (依存関係があるため)
        assert!(g_u < g_v);

        let adj = cg.condensation_adj();
        assert!(adj[g_u].contains(&g_v));
        assert!(adj[g_isolated].is_empty());
    }
}
