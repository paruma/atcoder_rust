// 問題文と制約は読みましたか？

fn a_cnts(xs: &[char]) -> Vec<usize> {
    let mut acc = vec![];
    let mut cnt = 0;

    for &x in xs {
        if x == 'A' {
            cnt += 1;
        } else {
            acc.push(cnt);
            cnt = 0;
        }
    }
    acc.push(cnt);
    acc
}

fn solve(xs: &[char], ys: &[char]) -> Option<i64> {
    let xs_removed_a = xs.iter().copied().filter(|&ch| ch != 'A').collect_vec();
    let ys_removed_a = ys.iter().copied().filter(|&ch| ch != 'A').collect_vec();

    if xs_removed_a != ys_removed_a {
        return None;
    }

    // let xs_rle = xs.iter().copied().dedup_with_count().collect_vec();
    // let ys_rle = ys.iter().copied().dedup_with_count().collect_vec();

    // let xs_cnts = xs_rle
    //     .iter()
    //     .copied()
    //     .filter(|(_, ch)| *ch == 'A')
    //     .map(|(cnt, _)| cnt)
    //     .collect_vec();

    // let ys_cnts = ys_rle
    //     .iter()
    //     .copied()
    //     .filter(|(_, ch)| *ch == 'A')
    //     .map(|(cnt, _)| cnt)
    //     .collect_vec();

    let xs_cnts = a_cnts(xs);
    let ys_cnts = a_cnts(ys);
    assert_eq!(xs_cnts.len(), ys_cnts.len());
    let ans = izip!(xs_cnts, ys_cnts)
        .map(|(xs_cnt, ys_cnt)| xs_cnt.abs_diff(ys_cnt))
        .sum::<usize>() as i64;
    Some(ans)
}
// #[fastout]
fn main() {
    input! {
        xs: Chars,
        ys: Chars,
    }
    let ans: Option<i64> = solve(&xs, &ys);
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
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
