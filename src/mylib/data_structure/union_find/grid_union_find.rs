use crate::{
    data_structure::union_find::simple_union_find::simple_union_find::UnionFind,
    math::geometry::pos::pos::Pos,
};

#[allow(clippy::module_inception)]
pub mod grid_union_find {
    use cargo_snippet::snippet;
    use itertools::Itertools;

    use super::{Pos, UnionFind};

    #[snippet(name = "GridUnionFind")]
    #[derive(Clone, Debug)]
    pub struct GridUnionFind {
        pub uf: UnionFind,
        pub h: usize,
        pub w: usize,
    }

    #[snippet(name = "GridUnionFind")]
    impl GridUnionFind {
        pub fn new(h: usize, w: usize) -> GridUnionFind {
            GridUnionFind {
                uf: UnionFind::new(h * w),
                h,
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

        pub fn same_count(&mut self, pos: Pos) -> usize {
            self.uf.same_count(self.encode(pos))
        }

        pub fn same(&mut self, pos1: Pos, pos2: Pos) -> bool {
            self.uf.same(self.encode(pos1), self.encode(pos2))
        }

        pub fn num_groups(&self) -> usize {
            self.uf.num_groups()
        }

        pub fn unite(&mut self, pos1: Pos, pos2: Pos) {
            self.uf.unite(self.encode(pos1), self.encode(pos2));
        }

        pub fn groups(&mut self) -> Vec<Vec<Pos>> {
            self.uf
                .groups()
                .into_iter()
                .map(|group| group.iter().copied().map(|i| self.decode(i)).collect_vec())
                .collect_vec()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::grid_union_find::GridUnionFind;
    use crate::math::geometry::pos::pos::Pos;

    #[test]
    fn test_new() {
        let guf = GridUnionFind::new(3, 4);
        assert_eq!(guf.h, 3);
        assert_eq!(guf.w, 4);
    }

    #[test]
    fn test_encode_decode() {
        let guf = GridUnionFind::new(3, 4);
        let pos = Pos::new(1, 2);
        let encoded = guf.encode(pos);
        assert_eq!(encoded, 2 * 4 + 1); // y * w + x = 2 * 4 + 1 = 9
        let decoded = guf.decode(encoded);
        assert_eq!(decoded, pos);

        let pos2 = Pos::new(0, 0);
        let encoded2 = guf.encode(pos2);
        assert_eq!(encoded2, 0);
        let decoded2 = guf.decode(encoded2);
        assert_eq!(decoded2, pos2);

        let pos3 = Pos::new(3, 2);
        let encoded3 = guf.encode(pos3);
        assert_eq!(encoded3, 2 * 4 + 3); // y * w + x = 2 * 4 + 3 = 11
        let decoded3 = guf.decode(encoded3);
        assert_eq!(decoded3, pos3);
    }

    #[test]
    fn test_unite_and_same() {
        let mut guf = GridUnionFind::new(3, 3); // 3x3 grid
        let p00 = Pos::new(0, 0);
        let p01 = Pos::new(1, 0);
        let p10 = Pos::new(0, 1);
        let p11 = Pos::new(1, 1);
        let p22 = Pos::new(2, 2);

        assert!(!guf.same(p00, p01));
        guf.unite(p00, p01);
        assert!(guf.same(p00, p01));
        assert!(!guf.same(p00, p10));

        guf.unite(p01, p11);
        assert!(guf.same(p00, p11));
        assert!(!guf.same(p00, p22));
    }

    #[test]
    fn test_same_count() {
        let mut guf = GridUnionFind::new(2, 2); // 2x2 grid
        let p00 = Pos::new(0, 0);
        let p01 = Pos::new(1, 0);
        let p10 = Pos::new(0, 1);
        let p11 = Pos::new(1, 1);

        assert_eq!(guf.same_count(p00), 1);
        guf.unite(p00, p01);
        assert_eq!(guf.same_count(p00), 2);
        guf.unite(p10, p11);
        guf.unite(p00, p10);
        assert_eq!(guf.same_count(p00), 4);
    }

    #[test]
    fn test_num_groups() {
        let mut guf = GridUnionFind::new(2, 2); // 2x2 grid
        let p00 = Pos::new(0, 0);
        let p01 = Pos::new(1, 0);
        let p10 = Pos::new(0, 1);
        let p11 = Pos::new(1, 1);

        assert_eq!(guf.num_groups(), 4);
        guf.unite(p00, p01);
        assert_eq!(guf.num_groups(), 3);
        guf.unite(p10, p11);
        assert_eq!(guf.num_groups(), 2);
        guf.unite(p00, p10);
        assert_eq!(guf.num_groups(), 1);
    }

    #[test]
    fn test_groups() {
        let mut guf = GridUnionFind::new(2, 2); // 2x2 grid
        let p00 = Pos::new(0, 0);
        let p01 = Pos::new(1, 0);
        let p10 = Pos::new(0, 1);
        let p11 = Pos::new(1, 1);

        guf.unite(p00, p01);
        guf.unite(p10, p11);

        let groups = guf.groups();
        assert_eq!(groups.len(), 2);

        let mut expected_group1 = vec![p00, p01];
        expected_group1.sort_unstable();
        let mut actual_group1 = groups[0].clone();
        actual_group1.sort_unstable();
        assert_eq!(actual_group1, expected_group1);

        let mut expected_group2 = vec![p10, p11];
        expected_group2.sort_unstable();
        let mut actual_group2 = groups[1].clone();
        actual_group2.sort_unstable();
        assert_eq!(actual_group2, expected_group2);

        guf.unite(p00, p10);
        let groups_all_united = guf.groups();
        assert_eq!(groups_all_united.len(), 1);
        let mut expected_all = vec![p00, p01, p10, p11];
        expected_all.sort_unstable();
        let mut actual_all = groups_all_united[0].clone();
        actual_all.sort_unstable();
        assert_eq!(actual_all, expected_all);
    }
}
