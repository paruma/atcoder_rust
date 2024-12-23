pub mod grid_template {
    use std::ops::{Index, IndexMut};

    use cargo_snippet::snippet;
    use itertools::{iproduct, Itertools};

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

        pub fn debug(&self) {
            for row in &self.grid {
                eprintln!("{}", row.iter().collect::<String>());
            }
            eprintln!();
        }

        /// pos の部分は背景を灰色にして出力する
        pub fn debug_with_pos(&self, pos: Pos) {
            const GRAY: &str = "\x1b[48;2;127;127;127;37m";
            const RESET: &str = "\x1b[0m";
            for y in 0..self.h {
                let row = (0..self.w)
                    .map(|x| {
                        if pos == Pos::new(x as i64, y as i64) {
                            format!("{}{}{}", GRAY, self.grid[y][x], RESET)
                        } else {
                            self.grid[y][x].to_string()
                        }
                    })
                    .join("");
                eprintln!("{}", row);
            }
            eprintln!();
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::mylib::pos0::pos::Pos;

    use super::grid_template::Grid;

    #[test]
    fn test_debug() {
        let g = Grid::new(vec![vec!['#', '.', '.'], vec!['.', '#', '.']]);
        g.debug();
        g.debug_with_pos(Pos::new(1, 1));
    }
}
