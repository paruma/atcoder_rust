use cargo_snippet::snippet;

use crate::{
    data_structure::dsu::dsu_core::dsu_core::DsuCore,
    math::geometry::pos::pos::Pos,
};

#[allow(clippy::module_inception)]
#[snippet(prefix = "use grid_dsu::*;")]
pub mod grid_dsu {
    use itertools::Itertools;
    use super::{DsuCore, Pos};

    pub struct GridDsu {
        dsu: DsuCore,
        w: usize,
    }

    impl GridDsu {
        pub fn new(h: usize, w: usize) -> GridDsu {
            GridDsu {
                dsu: DsuCore::new(h * w),
                w,
            }
        }

        pub fn encode(&self, pos: Pos) -> usize {
            (pos.y * self.w as i64 + pos.x) as usize
        }

        pub fn decode(&self, i: usize) -> Pos {
            let y = (i / self.w) as i64;
            let x = (i % self.w) as i64;
            Pos::new(x, y)
        }

        pub fn size(&mut self, pos: Pos) -> usize {
            self.dsu.size(self.encode(pos))
        }

        pub fn same(&mut self, pos1: Pos, pos2: Pos) -> bool {
            self.dsu.same(self.encode(pos1), self.encode(pos2))
        }

        pub fn count_group(&self) -> usize {
            self.dsu.count_group()
        }

        pub fn merge(&mut self, pos1: Pos, pos2: Pos) -> Option<(usize, usize)> {
            self.dsu.merge(self.encode(pos1), self.encode(pos2))
        }

        pub fn groups(&mut self) -> Vec<Vec<Pos>> {
            self.dsu
                .groups()
                .into_iter()
                .map(|group| group.iter().copied().map(|i| self.decode(i)).collect_vec())
                .collect_vec()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::grid_dsu::GridDsu;
    use crate::math::geometry::pos::pos::Pos;

    #[test]
    fn test_new() {
        let _gdsu = GridDsu::new(3, 4);
    }

    #[test]
    fn test_encode_decode() {
        let gdsu = GridDsu::new(3, 4);
        let pos = Pos::new(1, 2);
        let encoded = gdsu.encode(pos);
        assert_eq!(encoded, 2 * 4 + 1);
        let decoded = gdsu.decode(encoded);
        assert_eq!(decoded, pos);
    }

    #[test]
    fn test_merge_and_same() {
        let mut gdsu = GridDsu::new(3, 3);
        let p00 = Pos::new(0, 0);
        let p01 = Pos::new(1, 0);
        let p11 = Pos::new(1, 1);

        assert!(!gdsu.same(p00, p01));
        gdsu.merge(p00, p01);
        assert!(gdsu.same(p00, p01));

        gdsu.merge(p01, p11);
        assert!(gdsu.same(p00, p11));
    }

    #[test]
    fn test_size() {
        let mut gdsu = GridDsu::new(2, 2);
        let p00 = Pos::new(0, 0);
        let p01 = Pos::new(1, 0);

        assert_eq!(gdsu.size(p00), 1);
        gdsu.merge(p00, p01);
        assert_eq!(gdsu.size(p00), 2);
    }

    #[test]
    fn test_count_group() {
        let mut gdsu = GridDsu::new(2, 2);
        assert_eq!(gdsu.count_group(), 4);
        gdsu.merge(Pos::new(0, 0), Pos::new(1, 0));
        assert_eq!(gdsu.count_group(), 3);
    }

    #[test]
    fn test_groups() {
        let mut gdsu = GridDsu::new(2, 2);
        let p00 = Pos::new(0, 0);
        let p01 = Pos::new(1, 0);
        let p10 = Pos::new(0, 1);
        let p11 = Pos::new(1, 1);

        gdsu.merge(p00, p01);
        gdsu.merge(p10, p11);

        let groups = gdsu.groups();
        assert_eq!(groups.len(), 2);
    }
}