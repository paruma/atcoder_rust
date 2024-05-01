pub mod grid_template {
    use cargo_snippet::snippet;
    use itertools::iproduct;

    use crate::mylib::pos0::{pos::Pos, vec_vec_at::*};

    #[snippet(name = "Grid")]
    pub struct Grid {
        pub grid: Vec<Vec<u8>>,
        pub h: usize,
        pub w: usize,
    }

    #[snippet(name = "Grid")]
    impl Grid {
        pub fn new(grid: Vec<Vec<u8>>) -> Grid {
            let h = grid.len();
            let w = grid[0].len();
            Grid { grid, h, w }
        }

        pub fn is_within(&self, pos: Pos<i64>) -> bool {
            let h = self.h as i64;
            let w = self.w as i64;
            0 <= pos.y && pos.y < h && 0 <= pos.x && pos.x < w
        }

        pub fn at(&self, pos: Pos<i64>) -> &u8 {
            if self.is_within(pos) {
                self.grid.at(pos)
            } else {
                &b'#'
            }
        }

        pub fn at_mut(&mut self, pos: Pos<i64>) -> &mut u8 {
            self.grid.at_mut(pos)
        }

        pub fn can_move(&self, pos: Pos<i64>) -> bool {
            b".".contains(self.at(pos))
        }

        pub fn all_pos_iter(&self) -> impl Iterator<Item = Pos<i64>> {
            iproduct!(0..self.h, 0..self.w).map(|(y, x)| Pos::new(x as i64, y as i64))
        }

        pub fn find_pos_of(&self, ch: u8) -> Option<Pos<i64>> {
            self.all_pos_iter().find(|pos| self.at(*pos) == &ch)
        }
    }
}
