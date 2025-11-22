fn solve(h: usize, w: usize, xs: &[i64], ys: &[i64]) -> Option<Vec<Vec<i64>>> {
    // x と y が逆になってる。。。
    let mut grid = vec![vec![i64::MAX; w]; h];
    let mut puttable_row: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); h];
    let mut puttable_col: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); w];

    let mut row_map = HashMap::<i64, usize>::new();
    for (i, x) in xs.iter().copied().enumerate() {
        row_map.insert(x, i);
    }

    let mut col_map = HashMap::<i64, usize>::new();
    for (i, y) in ys.iter().copied().enumerate() {
        col_map.insert(y, i);
    }

    for cur in (1..=w * h).rev() {
        let cur = cur as i64;
        if row_map.contains_key(&cur) {
            let x = row_map[&cur];
            for y in 0..w {
                puttable_col[y].insert(x);
            }
        }

        if col_map.contains_key(&cur) {
            let y = col_map[&cur];
            for x in 0..h {
                puttable_col[x].insert(y);
            }
        }

        if row_map.contains_key(&cur) && col_map.contains_key(&cur) {
            let y = col_map[&cur];
            let x = row_map[&cur];
            grid[x][y] = cur;
        } else if row_map.contains_key(&cur) && !col_map.contains_key(&cur) {
            let x = row_map[&cur];
            if puttable_col[x].is_empty() {
                return None;
            } else {
                let y = *puttable_col[x].iter().min().unwrap();
                puttable_col[x].remove(&y);
                grid[x][y] = cur;
            }
        } else if !row_map.contains_key(&cur) && col_map.contains_key(&cur) {
            let y = col_map[&cur];
            if puttable_row[y].is_empty() {
                return None;
            } else {
                let x = *puttable_row[y].iter().min().unwrap();
                puttable_row[x].remove(&x);
                grid[x][y] = cur;
            }
        } else {
            return None;
        }
    }

    Some(grid)
}
#[fastout]
fn main() {
    input! {
        t: usize,
    }
    // cross explosion する

    for _ in 0..t {
        input! {
            h: usize,
            w: usize,
            xs: [i64; h],
            ys: [i64; w],
        }

        let ans = solve(h, w, &xs, &ys);

        if let Some(ans) = ans {
            for row in ans {
                let msg = row.iter().map(|x| format!("{}", x)).join(" ");
                println!("Yes");
                println!("{}", msg);
            }
        } else {
            println!("No");
        }
    }
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
use std::collections::BTreeSet;
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
        collections::{BinaryHeap, HashMap, HashSet},
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
    #[fastout]
    pub fn print_vec_chars(vec_chars: &[Vec<char>]) {
        for row in vec_chars {
            let msg = row.iter().collect::<String>();
            println!("{}", msg);
        }
    }
    pub fn print_yesno(ans: bool) {
        let msg = if ans { "Yes" } else { "No" };
        println!("{}", msg);
    }
}

// ====== snippet ======
