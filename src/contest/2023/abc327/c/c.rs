//#[derive_readable]
struct Problem {
    grid: Vec<Vec<i64>>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            grid:[[i64; 9]; 9],
        }
        Problem { grid }
    }
    #[allow(clippy::needless_range_loop)]
    fn is_ok(&self) -> bool {
        let grid = &self.grid;
        // 行
        for y in 0..9 {
            if !(0..9).map(|x| grid[y][x]).all_unique() {
                return false;
            }
        }
        // 列
        for x in 0..9 {
            if !(0..9).map(|y| grid[y][x]).all_unique() {
                return false;
            }
        }
        // 3*3
        for bx in 0..3 {
            for by in 0..3 {
                if !iproduct!(0..3, 0..3)
                    .map(|(ix, iy)| {
                        let x = 3 * bx + ix;
                        let y = 3 * by + iy;
                        grid[y][x]
                    })
                    .all_unique()
                {
                    return false;
                }
            }
        }

        true
    }
    fn solve(&self) -> Answer {
        let ans = self.is_ok();
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: bool,
}

impl Answer {
    fn print(&self) {
        //println!("{}", self.ans);
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

// ====== import ======
use itertools::iproduct;
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
