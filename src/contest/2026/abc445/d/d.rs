// 問題文と制約は読みましたか？
#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Piece {
    h: i64,
    w: i64,
}
#[fastout]
fn main() {
    input! {
        h: i64,
        w: i64,
        n: usize,
        ps: [Piece; n],
    }

    let mut h_map = HashMap::<i64, BTreeSet<usize>>::new();
    let mut w_map = HashMap::<i64, BTreeSet<usize>>::new();

    for (i, p) in ps.iter().copied().enumerate() {
        h_map.entry(p.h).or_default().insert(i);
        w_map.entry(p.w).or_default().insert(i);
    }

    let mut cur_h = h;
    let mut cur_w = w;
    let mut cur_r = 1;
    let mut cur_c = 1;

    let mut ans = vec![(i64::MAX, i64::MAX); n];

    while cur_h > 0 && cur_w > 0 {
        // dbg!(cur_h);
        // dbg!(cur_w);
        if let Some(is) = h_map.get_mut(&cur_h) {
            if let Some(&i) = is.iter().min() {
                // dbg!(i);
                let p = ps[i];
                // dbg!(p);
                is.remove(&i);
                w_map.get_mut(&p.w).unwrap().remove(&i);

                ans[i] = (cur_r, cur_c);

                cur_c += p.w;
                cur_w -= p.w;
            }
        }
        // dbg!(cur_h);
        // dbg!(cur_w);

        if let Some(is) = w_map.get_mut(&cur_w) {
            if let Some(&i) = is.iter().min() {
                // dbg!(i);
                let p = ps[i];
                // dbg!(p);
                is.remove(&i);
                h_map.get_mut(&p.h).unwrap().remove(&i);

                ans[i] = (cur_r, cur_c);

                cur_r += p.h;
                cur_h -= p.h;
            }
        }
    }

    for (r, c) in ans {
        println!("{} {}", r, c);
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
