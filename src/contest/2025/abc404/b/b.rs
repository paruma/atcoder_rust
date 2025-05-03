fn main() {
    input! {
        n: usize,
        xss: [Chars; n],
        yss: [Chars; n],
    }

    let xss0 = xss.clone();
    let xss1 = rotate_right(&xss0);
    let xss2 = rotate_right(&xss1);
    let xss3 = rotate_right(&xss2);

    let xss_rot = [xss0, xss1, xss2, xss3];

    let ans: usize = (0..4)
        .map(|i| {
            let cur_xss = &xss_rot[i];
            let diff = iproduct!(0..n, 0..n)
                .filter(|&(y, x)| cur_xss[y][x] != yss[y][x])
                .count();

            i + diff
        })
        .min()
        .unwrap();
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }
}

// ====== import ======
#[allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet};

// ====== output func ======
#[allow(unused_imports)]
use print_vec::*;
pub mod print_vec {

    use itertools::Itertools;
    use proconio::fastout;
    #[fastout]
    pub fn print_vec<T: std::fmt::Display>(arr: &[T]) {
        for a in arr {
            println!("{}", a);
        }
    }
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Display>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{}", x)).join(" ");
            println!("{}", msg);
        }
    }
    pub fn print_bytes(bytes: &[u8]) {
        let msg = String::from_utf8(bytes.to_vec()).unwrap();
        println!("{}", msg);
    }
    pub fn print_chars(chars: &[char]) {
        let msg = chars.iter().collect::<String>();
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
use array_2d_transformation::*;
#[allow(clippy::module_inception)]
pub mod array_2d_transformation {
    pub fn rotate_right<T>(table: &[Vec<T>]) -> Vec<Vec<T>>
    where
        T: Default + Clone + Copy,
    {
        let h = table.len();
        let w = table[0].len();
        let mut table_after = vec![vec![T::default(); h]; w];
        for (y, row) in table.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                table_after[x][h - 1 - y] = *v;
            }
        }
        table_after
    }
    pub fn rotate_left<T>(table: &[Vec<T>]) -> Vec<Vec<T>>
    where
        T: Default + Clone + Copy,
    {
        let h = table.len();
        let w = table[0].len();
        let mut table_after = vec![vec![T::default(); h]; w];
        for (y, row) in table.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                table_after[w - 1 - x][y] = *v;
            }
        }
        table_after
    }
    pub fn rotate_180_deg<T>(table: &[Vec<T>]) -> Vec<Vec<T>>
    where
        T: Default + Clone + Copy,
    {
        let h = table.len();
        let w = table[0].len();
        let mut table_after = vec![vec![T::default(); w]; h];
        for (y, row) in table.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                table_after[h - 1 - y][w - 1 - x] = *v;
            }
        }
        table_after
    }
    pub fn transpose<T>(table: &[Vec<T>]) -> Vec<Vec<T>>
    where
        T: Default + Clone + Copy,
    {
        let h = table.len();
        let w = table[0].len();
        let mut table_after = vec![vec![T::default(); h]; w];
        for (y, row) in table.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                table_after[x][y] = *v;
            }
        }
        table_after
    }
    pub fn reflect_x_axis<T>(table: &[Vec<T>]) -> Vec<Vec<T>>
    where
        T: Default + Clone + Copy,
    {
        let h = table.len();
        let w = table[0].len();
        let mut table_after = vec![vec![T::default(); w]; h];
        for (y, row) in table.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                table_after[h - 1 - y][x] = *v;
            }
        }
        table_after
    }
    pub fn reflect_y_axis<T>(table: &[Vec<T>]) -> Vec<Vec<T>>
    where
        T: Default + Clone + Copy,
    {
        let h = table.len();
        let w = table[0].len();
        let mut table_after = vec![vec![T::default(); w]; h];
        for (y, row) in table.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                table_after[y][w - 1 - x] = *v;
            }
        }
        table_after
    }
}
