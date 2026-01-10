use crate::data_structure::ix::{Bounds, Ix, IxVec};
use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use acl_scc_ix::*;")]
pub mod acl_scc_ix {
    use super::{Bounds, Ix, IxVec};

    /// 強連結成分分解 (SCC) を行い、縮約グラフ (DAG) を構築するための構造体 (Ix版)
    #[derive(Clone, Debug)]
    pub struct SccGraphIx<I: Ix> {
        bounds: Bounds<I>,
        edges: Vec<(usize, usize)>,
    }

    impl<I: Ix> SccGraphIx<I> {
        pub fn new(bounds: Bounds<I>) -> Self {
            Self {
                bounds,
                edges: vec![],
            }
        }

        pub fn add_edge(&mut self, from: I, to: I) {
            let from_idx = self.bounds.to_index(from);
            let to_idx = self.bounds.to_index(to);
            self.edges.push((from_idx, to_idx));
        }

        /// SCC を実行し、結果と縮約グラフ (DAG) を返す
        ///
        /// 計算量: O(V + E)
        pub fn scc(self) -> CondensationGraphIx<I> {
            let n = self.bounds.range_size();
            let mut g = ac_library::SccGraph::new(n);
            for &(u, v) in &self.edges {
                g.add_edge(u, v);
            }
            let groups_indices = g.scc();

            // マッピング作成
            let mut mapping_vec = vec![0; n];
            for (i, group) in groups_indices.iter().enumerate() {
                for &v in group {
                    mapping_vec[v] = i;
                }
            }
            let mapping = IxVec::from_vec(self.bounds, mapping_vec.clone());

            // DAG作成
            let mut adj = vec![vec![]; groups_indices.len()];
            for &(u, v) in &self.edges {
                let u_group = mapping_vec[u];
                let v_group = mapping_vec[v];
                if u_group != v_group {
                    adj[u_group].push(v_group);
                }
            }
            for list in &mut adj {
                list.sort_unstable();
                list.dedup();
            }

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
                mapping,
                adj,
            }
        }
    }

    /// SCC の結果と縮約グラフ (DAG) を保持する構造体 (Ix版)
    #[derive(Debug, Clone)]
    pub struct CondensationGraphIx<I: Ix> {
        /// 強連結成分のリスト (トポロジカル順)
        pub groups: Vec<Vec<I>>,
        /// 各頂点が属するグループ番号 (0-indexed)
        pub mapping: IxVec<I, usize>,
        /// 縮約グラフの隣接リスト (DAG)
        /// `adj[i]` はグループ `i` から遷移可能なグループのリスト
        pub adj: Vec<Vec<usize>>,
    }

    impl<I: Ix> CondensationGraphIx<I> {
        pub fn group_count(&self) -> usize {
            self.groups.len()
        }
    }
}

pub use acl_scc_ix::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_structure::ix::Bounds;

    #[test]
    fn test_scc_ix() {
        // 0 -> 1 -> 0
        let bounds = Bounds::new(0, 1);
        let mut scc_graph = SccGraphIx::new(bounds);
        scc_graph.add_edge(0, 1);
        scc_graph.add_edge(1, 0);

        let cg = scc_graph.scc();

        assert_eq!(cg.group_count(), 1);
        assert_eq!(cg.mapping[0], 0);
        assert_eq!(cg.mapping[1], 0);
        assert!(cg.adj[0].is_empty());
    }

    #[test]
    fn test_scc_ix_grid() {
        // (0,0) -> (0,1) -> (0,0)
        // (0,0) -> (1,0)
        let bounds = Bounds::new((0, 0), (1, 1));
        let mut scc_graph = SccGraphIx::new(bounds);
        scc_graph.add_edge((0, 0), (0, 1));
        scc_graph.add_edge((0, 1), (0, 0));
        scc_graph.add_edge((0, 0), (1, 0));

        let cg = scc_graph.scc();

        // 期待: {(0,0), (0,1)} -> {(1,0)}, {(1,1)} (孤立)
        assert_eq!(cg.group_count(), 3);

        let g_u = cg.mapping[(0, 0)];
        let g_v = cg.mapping[(1, 0)];
        let g_isolated = cg.mapping[(1, 1)];

        assert_eq!(cg.mapping[(0, 1)], g_u);
        assert_ne!(g_u, g_v);
        assert_ne!(g_u, g_isolated);
        assert_ne!(g_v, g_isolated);

        // トポロジカル順なら g_u < g_v (依存関係があるため)
        // g_isolated は依存関係がないのでどこに来るか不定だが、ACLの実装による
        assert!(g_u < g_v);

        assert!(cg.adj[g_u].contains(&g_v));
        assert!(cg.adj[g_isolated].is_empty());
    }
}
