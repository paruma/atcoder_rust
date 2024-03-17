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
struct Problem {
    n_tiles: usize,
    field: Rect,
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
            field,
            tiles,
        }
    }
    fn solve(&self) -> Answer {
        let n_tiles = self.n_tiles;
        let field = self.field;
        let tiles = &self.tiles;

        // タイル90度回転
        // let tiles = tiles
        //     .iter()
        //     .copied()
        //     .flat_map(|r| [r, r.rev()])
        //     .collect_vec();

        let ans = 0;
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: i64,
}

impl Answer {
    fn print(&self) {
        println!("{}", self.ans);
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
