fn solve(c: i64, d: i64) -> i64 {
    (1..=10)
        .map(|keta: u32| {
            // keta: c + x の桁
            // x の範囲
            // dbg!(keta);
            let min_x = (10_i64.pow(keta - 1) - c).max(1);
            let max_x = (10_i64.pow(keta) - c - 1).min(d);
            if min_x > max_x {
                return 0;
            }

            let fval_min = c * 10_i64.pow(keta) + c + min_x;
            let fval_max = c * 10_i64.pow(keta) + c + max_x;
            // dbg!(min_x);
            // dbg!(max_x);
            // dbg!(fval_min);
            // dbg!(fval_max);
            // dbg!((fval_max).sqrt() - (fval_min - 1).sqrt())

            (fval_max).sqrt() - (fval_min - 1).sqrt()
        })
        .sum::<i64>()
}
fn main() {
    input! {
        t: usize,
    }

    // dbg!("i128にする");

    let ans: Vec<i64> = (0..t)
        .map(|_| {
            input! {
                c: i64,
                d: i64,
            }
            solve(c, d)
        })
        .collect_vec();
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
        // let mut rng = SmallRng::from_entropy();
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
use num_integer::Roots;
#[allow(unused_imports)]
use {
    itertools::{chain, iproduct, izip, Itertools},
    proconio::{
        derive_readable, fastout, input,
        marker::{Bytes, Chars, Usize1},
    },
    rand::{rngs::SmallRng, seq::SliceRandom, Rng, SeedableRng},
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
/// 二分探索をする
/// ```text
/// ng ng ng ok ok ok
///          ↑ここの引数の値を返す
/// ```
/// 計算量: O(log(|ok - ng|))
/// ## Arguments
/// * ok != ng
/// * |ok - ng| <= 2^63 - 1, |ok + ng| <= 2^63 - 1
/// * p の定義域について
///     * ng < ok の場合、p は区間 ng..ok で定義されている。
///     * ok < ng の場合、p は区間 ok..ng で定義されている。
/// * p の単調性について
///     * ng < ok の場合、p は単調増加
///     * ok < ng の場合、p は単調減少
/// ## Return
/// * ng < ok の場合: I = { i in ng..ok | p(i) == true } としたとき
///     * I が空でなければ、min I を返す。
///     * I が空ならば、ok を返す。
/// * ok < ng の場合: I = { i in ok..ng | p(i) == true } としたとき
///     * I が空でなければ、max I を返す。
///     * I が空ならば、ok を返す。
pub fn bin_search<F>(mut ok: i64, mut ng: i64, mut p: F) -> i64
where
    F: FnMut(i64) -> bool,
{
    debug_assert!(ok != ng);
    debug_assert!(ok.checked_sub(ng).is_some());
    debug_assert!(ok.checked_add(ng).is_some());
    while num::abs(ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        debug_assert!(mid != ok);
        debug_assert!(mid != ng);
        if p(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}
