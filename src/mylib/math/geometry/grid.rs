pub mod grid_template {
    use std::ops::{Index, IndexMut};

    use cargo_snippet::snippet;
    use itertools::{Itertools, iproduct};

    use crate::math::geometry::pos::pos::Pos;

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
    use crate::math::geometry::pos::pos::Pos;

    use super::grid_template::Grid;

    #[test]
    fn test_grid_new_and_index() {
        let grid_vec = vec![vec!['#', '.'], vec!['S', 'G']];
        let grid = Grid::new(grid_vec.clone());
        assert_eq!(grid.h, 2);
        assert_eq!(grid.w, 2);
        assert_eq!(grid[Pos::new(0, 0)], '#');
        assert_eq!(grid[Pos::new(1, 0)], '.');
        assert_eq!(grid[Pos::new(0, 1)], 'S');
        assert_eq!(grid[Pos::new(1, 1)], 'G');
    }

    #[test]
    fn test_is_within() {
        let grid = Grid::new(vec![vec!['.'; 3]; 2]);
        assert!(grid.is_within(Pos::new(0, 0)));
        assert!(grid.is_within(Pos::new(2, 1)));
        assert!(!grid.is_within(Pos::new(3, 1)));
        assert!(!grid.is_within(Pos::new(2, 2)));
        assert!(!grid.is_within(Pos::new(-1, 0)));
    }

    #[test]
    fn test_can_move() {
        let grid = Grid::new(vec![vec!['.', '#'], vec!['S', 'G']]);
        assert!(grid.can_move(Pos::new(0, 0)));
        assert!(!grid.can_move(Pos::new(1, 0)));
    }

    #[test]
    fn test_all_pos_iter() {
        let grid = Grid::new(vec![vec!['.'; 2]; 2]);
        let positions = grid.all_pos_iter().collect::<Vec<_>>();
        assert_eq!(
            positions,
            vec![
                Pos::new(0, 0),
                Pos::new(1, 0),
                Pos::new(0, 1),
                Pos::new(1, 1)
            ]
        );
    }

    #[test]
    fn test_find_pos_of() {
        let grid = Grid::new(vec![vec!['.', 'S'], vec!['#', 'G']]);
        assert_eq!(grid.find_pos_of('S'), Some(Pos::new(1, 0)));
        assert_eq!(grid.find_pos_of('G'), Some(Pos::new(1, 1)));
        assert_eq!(grid.find_pos_of('A'), None);
    }

    #[test]
    fn test_encode_decode() {
        let grid = Grid::new(vec![vec!['.'; 4]; 3]);
        let pos = Pos::new(2, 1);
        let encoded = grid.encode(pos);
        assert_eq!(encoded, 6);
        let decoded = grid.decode(encoded);
        assert_eq!(decoded, pos);

        let pos = Pos::new(0, 0);
        let encoded = grid.encode(pos);
        assert_eq!(encoded, 0);
        let decoded = grid.decode(encoded);
        assert_eq!(decoded, pos);

        let pos = Pos::new(3, 2);
        let encoded = grid.encode(pos);
        assert_eq!(encoded, 11);
        let decoded = grid.decode(encoded);
        assert_eq!(decoded, pos);
    }

    #[test]
    fn test_debug() {
        let g = Grid::new(vec![vec!['#', '.', '.'], vec!['.', '#', '.']]);
        g.debug();
        g.debug_with_pos(Pos::new(1, 1));
    }

    #[test]
    fn test_index_out_of_bounds() {
        let grid = Grid::new(vec![vec!['.']]);
        assert_eq!(grid[Pos::new(0, 0)], '.');
        assert_eq!(grid[Pos::new(-1, 0)], '#');
        assert_eq!(grid[Pos::new(0, -1)], '#');
        assert_eq!(grid[Pos::new(1, 0)], '#');
        assert_eq!(grid[Pos::new(0, 1)], '#');
    }

    #[test]
    fn test_index_mut() {
        let mut grid = Grid::new(vec![vec!['.']]);
        grid[Pos::new(0, 0)] = '#';
        assert_eq!(grid[Pos::new(0, 0)], '#');
    }
}
