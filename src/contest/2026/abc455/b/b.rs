// 問題文と制約は読みましたか？
fn is_sprial(grid: &[Vec<char>]) -> bool {
    let rotated = rotate_180_deg(&grid);
    grid == rotated
}

// #[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        grid: [Chars; h],
    }
    let ans: usize = iproduct!((0..=h).tuple_combinations(), (0..=w).tuple_combinations(),)
        .filter(|&((y_begin, y_end), (x_begin, x_end))| {
            let mut sub_grid = vec![vec!['a'; x_end - x_begin]; y_end - y_begin];
            for y in y_begin..y_end {
                for x in x_begin..x_end {
                    sub_grid[y - y_begin][x - x_begin] = grid[y][x];
                }
            }
            is_sprial(&sub_grid)
        })
        .count();
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

    /// 間違っていたら false を返す
    fn process_one_test(rng: &mut SmallRng) -> bool {
        // ==== 問題を作る ====
        let n = rng.random_range(1..=10);
        let xs = (0..n).map(|_| rng.random_range(0..10)).collect_vec();

        // ==== 解く ====
        let main_ans = xs.len();
        let naive_ans = 1;

        // ==== 間違っていたら報告をする ====
        if main_ans != naive_ans {
            // 問題を出力
            println!("{:?}", (n, xs));
            println!("main ans : {:?}", main_ans);
            println!("naive ans: {:?}", naive_ans);
            return false;
        }
        true
    }

    #[allow(unreachable_code)]
    #[test]
    #[ignore]
    fn test_with_naive() {
        let num_tests = 100;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut cnt_wrong = 0;
        let mut rng = SmallRng::seed_from_u64(42);
        // let mut rng = SmallRng::from_os_rng();
        for _ in 0..num_tests {
            let is_ok = process_one_test(&mut rng);
            if !is_ok {
                cnt_wrong += 1;
            }
            if cnt_wrong >= max_wrong_case {
                break;
            }
        }
        if cnt_wrong > 0 {
            println!("{} cases are wrong.", cnt_wrong);
            panic!();
        }
    }
}

// ====== import ======
#[allow(unused_imports)]
use {
    itertools::{Itertools, chain, iproduct, izip},
    proconio::{
        derive_readable, fastout, input,
        marker::{Bytes, Chars, Usize1},
    },
    rand::{Rng, SeedableRng, rngs::SmallRng, seq::SliceRandom},
    std::{
        cmp::Reverse,
        collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet},
    },
};

// ====== output func ======
#[allow(unused_imports)]
use print_util::*;
pub mod print_util {
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
        println!("{}", arr.iter().join(" "));
    }

    #[fastout]
    pub fn print_vec2<T: std::fmt::Display, R: AsRef<[T]>>(arr: &[R]) {
        for row in arr {
            println!("{}", row.as_ref().iter().join(" "));
        }
    }

    pub fn print_bytes(bytes: &[u8]) {
        println!("{}", std::str::from_utf8(bytes).unwrap());
    }

    pub fn print_chars(chars: &[char]) {
        println!("{}", chars.iter().collect::<String>());
    }

    #[fastout]
    pub fn print_vec_bytes<R: AsRef<[u8]>>(vec_bytes: &[R]) {
        for row in vec_bytes {
            println!("{}", std::str::from_utf8(row.as_ref()).unwrap());
        }
    }

    #[fastout]
    pub fn print_vec_chars<R: AsRef<[char]>>(vec_chars: &[R]) {
        for row in vec_chars {
            println!("{}", row.as_ref().iter().collect::<String>());
        }
    }

    pub fn print_yesno(ans: bool) {
        println!("{}", if ans { "Yes" } else { "No" });
    }
}

// ====== snippet ======
use array_2d_transformation::*;
#[allow(clippy::module_inception)]
/// 2次元配列 (`Vec<Vec<T>>`) を変換するためのモジュールです。
/// このモジュールは、2次元配列に対する回転、転置、反転といった幾何学的変換を行う関数を提供します。
/// すべての関数は `&[Vec<T>]` を受け取り、新しい `Vec<Vec<T>>` を返します。
/// 要素型 `T` は `Default`、`Clone`、`Copy` を実装している必要があります。
/// 注: これらの関数は、空の行や列 (例: 0xH または Wx0 の行列) を持つ配列には対応していません。
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
