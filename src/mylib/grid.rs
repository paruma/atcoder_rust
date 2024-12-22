pub mod grid_template {
    use std::ops::{Index, IndexMut};

    use cargo_snippet::snippet;
    use itertools::iproduct;

    use crate::mylib::pos0::pos::Pos;

    #[snippet(name = "Grid", prefix = "use std::ops::{Index, IndexMut};")]
    pub struct Grid {
        pub grid: Vec<Vec<char>>,
        pub h: usize,
        pub w: usize,
    }

    #[snippet(name = "Grid")]
    impl Index<Pos> for Grid {
        type Output = char;

        fn index(&self, index: Pos) -> &Self::Output {
            if self.is_within(index) {
                self.grid.index(index)
            } else {
                &'#'
            }
        }
    }

    #[snippet(name = "Grid")]
    impl IndexMut<Pos> for Grid {
        fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
            self.grid.index_mut(index)
        }
    }

    #[snippet(name = "Grid")]
    impl Grid {
        pub fn new(grid: Vec<Vec<char>>) -> Grid {
            let h = grid.len();
            let w = grid[0].len();
            Grid { grid, h, w }
        }

        pub fn is_within(&self, pos: Pos) -> bool {
            let h = self.h as i64;
            let w = self.w as i64;
            0 <= pos.y && pos.y < h && 0 <= pos.x && pos.x < w
        }

        pub fn can_move(&self, pos: Pos) -> bool {
            ['.'].contains(&self[pos])
        }

        pub fn all_pos_iter(&self) -> impl Iterator<Item = Pos> {
            iproduct!(0..self.h, 0..self.w).map(|(y, x)| Pos::new(x as i64, y as i64))
        }

        pub fn find_pos_of(&self, ch: char) -> Option<Pos> {
            self.all_pos_iter().find(|pos| self[*pos] == ch)
        }

        pub fn encode(&self, pos: Pos) -> usize {
            (pos.y * self.w as i64 + pos.x) as usize
        }

        pub fn decode(&self, i: usize) -> Pos {
            let y = (i / self.w) as i64;
            let x = (i % self.w) as i64;
            Pos::new(x, y)
        }
    }
}
