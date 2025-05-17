fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        garbage_pos_list: [(Usize1, Usize1); n],
        nq: usize,
    }

    let mut rows = vec![HashSet::<usize>::new(); h];
    let mut cols = vec![HashSet::<usize>::new(); w];

    for (i, (gy, gx)) in garbage_pos_list.iter().copied().enumerate() {
        rows[gy].insert(i);
        cols[gx].insert(i);
    }

    let mut ans: Vec<usize> = vec![];
    for _ in 0..nq {
        input! {
            t: usize
        }

        if t == 1 {
            input! {
                y: Usize1
            }
            ans.push(rows[y].len());
            for &i in &rows[y] {
                let (_, x) = garbage_pos_list[i];
                cols[x].remove(&i);
            }

            // rows[y].clear();
            rows[y] = HashSet::new();
        } else {
            input! {
                x: Usize1
            }
            ans.push(cols[x].len());
            for &i in &cols[x] {
                let (y, _) = garbage_pos_list[i];
                rows[y].remove(&i);
            }

            // cols[x].clear();
            cols[x] = HashSet::new();
        }
    }

    print_vec(&ans);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};

    #[test]
    fn test_problem() {
        dbg!(std::mem::needs_drop::<usize>());
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
