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
    fn area(&self) -> usize {
        self.width * self.height
    }
}

struct Board {
    width: usize,
    height: usize,
    board: Vec<Vec<bool>>,
    tile_area: usize, // おいたタイルの面積
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        Board {
            width,
            height,
            board: vec![vec![false; width]; height],
            tile_area: 0,
        }
    }

    fn is_within(&self, x: usize, y: usize) -> bool {
        y < self.height && x < self.width
    }

    fn is_tile_pos(&self, x: usize, y: usize) -> bool {
        !self.is_within(x, y) || self.board[y][x]
    }

    fn can_put_tile(&self, pos: Pos<usize>, tile: &Rect) -> bool {
        // pos が左上になるようにタイルを置けるか
        iproduct!(pos.y..pos.y + tile.height, pos.x..pos.x + tile.width)
            .all(|(y, x)| !self.is_tile_pos(x, y))
    }

    fn put_tile(&mut self, pos: Pos<usize>, tile: &Rect) -> bool {
        if self.can_put_tile(pos, tile) {
            for (y, x) in iproduct!(pos.y..pos.y + tile.height, pos.x..pos.x + tile.width) {
                self.board[y][x] = true;
            }
            self.tile_area += tile.area();
            true
        } else {
            false
        }
    }

    fn is_covered(&self) -> bool {
        self.tile_area == self.width * self.height
    }
}

struct Field {
    field: Rect,
}

impl Field {
    fn new(field: Rect) -> Field {
        Field { field }
    }

    // tile_list の順番に左上から敷き詰められるか
    fn can_cover(&self, tile_list: &[Rect]) -> bool {
        let width = self.field.width;
        let height = self.field.height;
        let mut board = Board::new(width, height);

        let mut tile_list_iter = tile_list.iter().copied();

        for i in 0..width * height {
            let y = i / width;
            let x = i % width;
            if board.is_tile_pos(x, y) {
                continue;
            }

            match tile_list_iter.next() {
                None => return false,
                Some(tile) => {
                    if !board.put_tile(Pos::new(x, y), &tile) {
                        return false;
                    }
                }
            }
            if board.is_covered() {
                return true;
            }
        }

        false
    }
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
        let n = self.n_tiles;
        let field_size = self.field_size;
        let tiles = &self.tiles;

        // actioned_tiles[0]: 0番目のタイルとその回転を入れた配列
        let actioned_tiles = tiles
            .iter()
            .map(|tile| vec![*tile, tile.rev()])
            .collect_vec();

        let field = Field { field: field_size };

        let ans = actioned_tiles
            .iter()
            .permutations(n)
            .flat_map(|actioned_tiles_permu| {
                actioned_tiles_permu
                    .iter()
                    .copied()
                    .multi_cartesian_product()
            })
            .any(|tile_list| {
                let tile_list = tile_list.into_iter().copied().collect_vec();
                field.can_cover(&tile_list)
            });

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
