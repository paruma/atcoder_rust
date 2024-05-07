#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Rect {
    width: usize,
    height: usize,
}

impl Rect {
    fn rev(&self) -> Rect {
        Rect {
            width: self.height,
            height: self.width,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Board {
    width: usize,
    height: usize,
    board: Vec<Vec<bool>>,
}

impl Board {
    fn new(width: usize, height: usize, board: &[Vec<bool>]) -> Board {
        Board {
            width,
            height,
            board: board.to_vec(),
        }
    }
    fn make_empty(width: usize, height: usize) -> Board {
        Board {
            width,
            height,
            board: vec![vec![false; width]; height],
        }
    }

    fn is_within(&self, pos: Pos<i64>) -> bool {
        (pos.y as usize) < self.height && (pos.x as usize) < self.width
    }

    fn is_blank(&self, pos: Pos<i64>) -> bool {
        self.is_within(pos) && !self.board[pos.y as usize][pos.x as usize]
    }

    fn count_blank(&self) -> usize {
        self.board.iter().flatten().filter(|&&x| !x).count()
    }

    fn is_corner(&self, pos: Pos<i64>) -> bool {
        let pos_left = pos + Pos::new(-1, 0);
        let pos_up = pos + Pos::new(0, -1);

        self.is_blank(pos) && !self.is_blank(pos_left) && !self.is_blank(pos_up)
    }

    // 左上が pos となるように tile をおけるかどうか
    fn can_put(&self, pos: Pos<i64>, tile: &Rect) -> bool {
        iproduct!(
            pos.y..pos.y + tile.height as i64,
            pos.x..pos.x + tile.width as i64
        )
        .map(|(y, x)| Pos::new(x, y))
        .all(|pos| self.is_blank(pos))
    }

    fn put(&self, pos: Pos<i64>, tile: &Rect, index: usize) -> Self {
        let mut next_board = self.board.clone();
        for (y, x) in iproduct!(
            pos.y..pos.y + tile.height as i64,
            pos.x..pos.x + tile.width as i64
        ) {
            next_board[y as usize][x as usize] = true;
        }

        Board::new(self.width, self.height, &next_board)
    }

    fn is_fill_all(&self) -> bool {
        self.board.iter().flatten().filter(|idx| **idx).count() == self.width * self.height
    }

    // fn pretty_string(&self) -> String {
    //     self.board
    //         .iter()
    //         .map(|row| {
    //             row.iter()
    //                 .map(|x| {
    //                     if let Some(i) = x {
    //                         (b'0' + *i as u8) as char
    //                     } else {
    //                         '.'
    //                     }
    //                 })
    //                 .collect::<String>()
    //         })
    //         .join("\n")
    // }
}
struct Problem {
    n_tiles: usize,
    field_size: Rect,
    tiles: Vec<Rect>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n_tiles: usize,
            field: Rect,
            tiles: [Rect; n_tiles],
        }
        Problem {
            n_tiles,
            field_size: field,
            tiles,
        }
    }
    fn solve(&self) -> Answer {
        // 計算量 4^N N! WH くらい (TLE)
        let field_size = self.field_size;
        let tiles = &self.tiles;

        // actioned_tiles[0]: 0番目のタイルとその回転を入れた配列
        let actioned_tiles = tiles
            .iter()
            .map(|tile| vec![*tile, tile.rev()])
            .collect_vec();

        struct Dfs {
            actioned_tiles: Vec<Vec<Rect>>,
            board_size: Rect,
        }

        impl Dfs {
            fn new(actioned_tiles: Vec<Vec<Rect>>, board_size: Rect) -> Self {
                Self {
                    actioned_tiles,
                    board_size,
                }
            }

            fn exec(&self) -> bool {
                let used_list = vec![false; self.actioned_tiles.len()];
                let open = vec![Pos::new(0, 0)];
                let board = Board::make_empty(self.board_size.width, self.board_size.height);

                self.exec_rec(used_list, open, board)
            }

            fn exec_rec(
                &self,
                used_list: Vec<bool>,
                mut open: Vec<Pos<i64>>,
                board: Board,
            ) -> bool {
                // println!("{}", board.pretty_string());
                // println!();

                if board.is_fill_all() {
                    return true;
                }

                let remaining_tile_area = izip!(&used_list, &self.actioned_tiles)
                    .filter(|(used, _)| !**used)
                    .map(|(_, tile)| tile[0].width * tile[0].height)
                    .sum::<usize>();
                if remaining_tile_area < board.count_blank() {
                    return false;
                }
                while let Some(current) = open.pop() {
                    let n_tiles = self.actioned_tiles.len();
                    for i in 0..n_tiles {
                        if used_list[i] {
                            continue;
                        }
                        for tile in &self.actioned_tiles[i] {
                            if board.can_put(current, tile) {
                                let next_used_list = {
                                    let mut next_used_list = used_list.clone();
                                    next_used_list[i] = true;
                                    next_used_list
                                };
                                let next_open = {
                                    let mut next_open = open.clone();
                                    let next1 = current + Pos::new(0, tile.height as i64);
                                    let next2 = current + Pos::new(tile.width as i64, 0);
                                    if board.is_within(next1) {
                                        next_open.push(next1);
                                    }
                                    if board.is_within(next2) {
                                        next_open.push(next2);
                                    }
                                    next_open
                                };
                                let next_board = board.put(current, tile, i);
                                let next_is_ok = self.exec_rec(
                                    next_used_list.clone(),
                                    next_open.clone(),
                                    next_board.clone(),
                                );
                                if next_is_ok {
                                    return true;
                                }
                            }
                        }
                    }
                }

                false
            }
        }

        let dfs = Dfs::new(actioned_tiles, field_size);
        let ans = dfs.exec();
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: bool,
}

impl Answer {
    fn print(&self) {
        print_yesno(self.ans);
    }
}

fn main() {
    Problem::read().solve().print();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }
}

use itertools::{iproduct, izip};
// ====== import ======
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};

// ====== output func ======
#[allow(unused_imports)]
use print_vec::*;
pub mod print_vec {
    use itertools::Itertools;
    use proconio::fastout;
    #[fastout]
    pub fn print_vec<T: std::fmt::Debug>(arr: &[T]) {
        for a in arr {
            println!("{:?}", a);
        }
    }
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Debug>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{:?}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Debug>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{:?}", x)).join(" ");
            println!("{}", msg);
        }
    }
    pub fn print_bytes(bytes: &[u8]) {
        let msg = String::from_utf8(bytes.to_vec()).unwrap();
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec_bytes(vec_bytes: &[Vec<u8>]) {
        for row in vec_bytes {
            let msg = String::from_utf8(row.to_vec()).unwrap();
            println!("{}", msg);
        }
    }
}

#[allow(unused)]
fn print_yesno(ans: bool) {
    let msg = if ans { "Yes" } else { "No" };
    println!("{}", msg);
}

// ====== snippet ======

use pos::*;
pub mod pos {
    use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Pos<T> {
        pub x: T,
        pub y: T,
    }
    impl<T> Pos<T> {
        pub fn new(x: T, y: T) -> Pos<T> {
            Pos { x, y }
        }
    }
    impl<T: Mul<Output = T> + Copy> Pos<T> {
        pub fn scala_mul(self, rhs: T) -> Pos<T> {
            Pos::new(self.x * rhs, self.y * rhs)
        }
    }
    impl<T: Add<Output = T> + Mul<Output = T> + Copy> Pos<T> {
        pub fn norm_square(self) -> T {
            self.x * self.x + self.y * self.y
        }
    }
    impl<T: Add<Output = T> + Copy> Add for Pos<T> {
        type Output = Pos<T>;
        fn add(self, rhs: Self) -> Self::Output {
            Pos::new(self.x + rhs.x, self.y + rhs.y)
        }
    }
    impl<T: Sub<Output = T> + Copy> Sub for Pos<T> {
        type Output = Pos<T>;
        fn sub(self, rhs: Self) -> Self::Output {
            Pos::new(self.x - rhs.x, self.y - rhs.y)
        }
    }
    impl<T: Neg<Output = T>> Neg for Pos<T> {
        type Output = Self;
        fn neg(self) -> Self::Output {
            Pos::new(-self.x, -self.y)
        }
    }
    impl<T: num_traits::Zero + Copy> num_traits::Zero for Pos<T> {
        fn zero() -> Self {
            Pos::new(T::zero(), T::zero())
        }
        fn is_zero(&self) -> bool {
            self.x.is_zero() && self.y.is_zero()
        }
    }
    impl<T: Add<Output = T> + Copy> AddAssign for Pos<T> {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }
    impl<T: Sub<Output = T> + Copy> SubAssign for Pos<T> {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }
    pub const DIR8_LIST: [Pos<i64>; 8] = [
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 1 },
        Pos { x: 1, y: 0 },
        Pos { x: 1, y: -1 },
        Pos { x: 0, y: -1 },
        Pos { x: -1, y: -1 },
        Pos { x: -1, y: 0 },
        Pos { x: -1, y: 1 },
    ];
    pub const DIR4_LIST: [Pos<i64>; 4] = [
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 0 },
        Pos { x: 0, y: -1 },
        Pos { x: -1, y: 0 },
    ];
    impl Pos<i64> {
        pub fn around4_pos_iter(self) -> impl Iterator<Item = Pos<i64>> {
            DIR4_LIST.iter().copied().map(move |d| d + self)
        }
        pub fn around8_pos_iter(self) -> impl Iterator<Item = Pos<i64>> {
            DIR8_LIST.iter().copied().map(move |d| d + self)
        }
    }
}

use mod_queue::*;
pub mod mod_queue {
    use std::collections::VecDeque;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Queue<T> {
        raw: VecDeque<T>,
    }
    impl<T> Queue<T> {
        pub fn new() -> Self {
            Queue {
                raw: VecDeque::new(),
            }
        }
        pub fn push(&mut self, value: T) {
            self.raw.push_front(value)
        }
        pub fn pop(&mut self) -> Option<T> {
            self.raw.pop_back()
        }
        pub fn peek(&self) -> Option<&T> {
            self.raw.back()
        }
        pub fn is_empty(&self) -> bool {
            self.raw.is_empty()
        }
    }
    impl<T> Default for Queue<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}
