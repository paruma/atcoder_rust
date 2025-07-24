#[allow(clippy::module_inception)]
pub mod grid_union_find {
    use cargo_snippet::snippet;
    use itertools::Itertools;

    use crate::mylib::{
        data_structure::union_find::simple_union_find::simple_union_find::UnionFind, pos0::pos::Pos,
    };

    #[snippet(name = "GridUnionFind")]
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
