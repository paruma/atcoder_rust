use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use acl_scc_ix::*;")]
pub mod acl_scc_ix {
    use crate::data_structure::ix::{Bounds, Ix};
    use ac_library::SccGraph;

    #[derive(Clone, Debug)]
    pub struct SccGraphIx<I: Ix> {
        graph: SccGraph,
        bounds: Bounds<I>,
    }

    impl<I: Ix> SccGraphIx<I> {
        pub fn new(bounds: Bounds<I>) -> Self {
            let n = bounds.range_size();
            Self {
                graph: SccGraph::new(n),
                bounds,
            }
        }

        pub fn add_edge(&mut self, from: I, to: I) {
            let from_idx = self.bounds.to_index(from);
            let to_idx = self.bounds.to_index(to);
            self.graph.add_edge(from_idx, to_idx);
        }

        pub fn scc(&self) -> Vec<Vec<I>> {
            self.graph
                .scc()
                .into_iter()
                .map(|group| {
                    group
                        .into_iter()
                        .map(|idx| self.bounds.from_index(idx))
                        .collect()
                })
                .collect()
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
        let mut scc = SccGraphIx::new(bounds);
        scc.add_edge(0, 1);
        scc.add_edge(1, 0);
        let groups = scc.scc();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].len(), 2);
    }

    #[test]
    fn test_scc_ix_grid() {
        // (0,0) -> (0,1) -> (0,0)
        // (1,0)
        let bounds = Bounds::new((0, 0), (1, 1));
        let mut scc = SccGraphIx::new(bounds);
        scc.add_edge((0, 0), (0, 1));
        scc.add_edge((0, 1), (0, 0));
        
        let groups = scc.scc();
        // {(0,0), (0,1)}, {(1,0)}, {(1,1)} -> 3 groups
        assert_eq!(groups.len(), 3);
    }
}
