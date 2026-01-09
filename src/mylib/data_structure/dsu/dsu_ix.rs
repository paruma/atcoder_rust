use cargo_snippet::snippet;
use super::dsu_core::DsuCore;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use dsu_ix::*;")]
pub mod dsu_ix {
    use super::DsuCore;
    use crate::data_structure::ix::{Bounds, Ix};

    #[derive(Clone, Debug)]
    pub struct DsuIx<I: Ix> {
        dsu: DsuCore,
        bounds: Bounds<I>,
    }

    impl<I: Ix> DsuIx<I> {
        pub fn new(bounds: Bounds<I>) -> Self {
            let n = bounds.range_size();
            Self {
                dsu: DsuCore::new(n),
                bounds,
            }
        }

        pub fn merge(&mut self, a: I, b: I) -> Option<(I, I)> {
            let a_idx = self.bounds.to_index(a);
            let b_idx = self.bounds.to_index(b);
            let res = self.dsu.merge(a_idx, b_idx);
            res.map(|(l, m)| (self.bounds.from_index(l), self.bounds.from_index(m)))
        }

        pub fn same(&mut self, a: I, b: I) -> bool {
            let a_idx = self.bounds.to_index(a);
            let b_idx = self.bounds.to_index(b);
            self.dsu.same(a_idx, b_idx)
        }

        pub fn leader(&mut self, a: I) -> I {
            let a_idx = self.bounds.to_index(a);
            let l_idx = self.dsu.leader(a_idx);
            self.bounds.from_index(l_idx)
        }

        pub fn size(&mut self, a: I) -> usize {
            let a_idx = self.bounds.to_index(a);
            self.dsu.size(a_idx)
        }

        pub fn count_group(&self) -> usize {
            self.dsu.count_group()
        }

        pub fn groups(&mut self) -> Vec<Vec<I>> {
            self.dsu
                .groups()
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

pub use dsu_ix::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_structure::ix::Bounds;

    #[test]
    fn test_dsu_ix() {
        // 2x2 grid
        let bounds = Bounds::new((0, 0), (1, 1));
        let mut dsu = DsuIx::new(bounds);

        // merge (0,0) and (0,1)
        assert!(dsu.merge((0, 0), (0, 1)).is_some());
        assert!(dsu.same((0, 0), (0, 1)));
        assert!(!dsu.same((0, 0), (1, 0)));

        assert_eq!(dsu.leader((0, 0)), dsu.leader((0, 1)));
        assert_ne!(dsu.leader((0, 0)), dsu.leader((1, 0)));

        assert_eq!(dsu.size((0, 0)), 2);
        assert_eq!(dsu.count_group(), 3); // {(0,0),(0,1)}, {(1,0)}, {(1,1)}

        // groups
        let groups = dsu.groups();
        assert_eq!(groups.len(), 3);
    }
}
