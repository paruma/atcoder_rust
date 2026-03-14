// 問題文と制約は読みましたか？
// #[fastout]

// [1, x] * [1, y] にいくつ黒点ある？
fn sub1(mut x: i64, mut y: i64) -> i64 {
    // x <= y を仮定しておく
    if x > y {
        std::mem::swap(&mut x, &mut y);
    }

    assert!(x <= y);

    std::iter::successors(Some(3_i64), |acc| Some(acc + 2))
        .take_while(|&r| r <= y + 1)
        .map(|r| {
            //
            if r <= x + 1 {
                // r = 3, x=2 OK, x=1 NG
                2 * r - 3
            } else {
                x
            }
        })
        .sum::<i64>()
}

// [x1, x2] * [y1, y2] にいくつ黒点ある？, x1, y1 >=1
fn sub2(x1: i64, x2: i64, y1: i64, y2: i64) -> i64 {
    // dbg!(x1, x2, y1, y2);
    assert!(x1 >= 1);
    assert!(x2 >= 1);
    assert!(y1 >= 1);
    assert!(y2 >= 1);
    sub1(x2, y2) - sub1(x2, y1 - 1) - sub1(x1 - 1, y2) + sub1(x1 - 1, y1 - 1)
}

fn sub3(mut x1: i64, mut x2: i64, mut y1: i64, mut y2: i64) -> i64 {
    if x1 < 0 && x2 < 0 {
        x1 *= -1;
        x2 *= -1;
        swap(&mut x1, &mut x2);
    }

    if y1 < 0 && y2 < 0 {
        y1 *= -1;
        y2 *= -1;
        swap(&mut y1, &mut y2);
    }
    sub2(x1, x2, y1, y2)
}

fn main() {
    input! {
        l: i64,
        r: i64,
        d: i64,
        u: i64,
    }
    // l = r = 0 のケースがいや

    let mut x_ranges = vec![];
    if l == 0 && r == 0 {
    } else if l < 0 && r == 0 {
        x_ranges.push((l, -1));
    } else if l < 0 && r < 0 {
        x_ranges.push((l, r));
    } else if l < 0 && r > 0 {
        x_ranges.push((l, -1));
        x_ranges.push((1, r));
    } else if l == 0 && r > 0 {
        x_ranges.push((1, r));
    } else {
        x_ranges.push((l, r));
    }

    let mut y_ranges = vec![];
    if d == 0 && u == 0 {
    } else if d < 0 && u == 0 {
        y_ranges.push((d, -1));
    } else if d < 0 && u < 0 {
        y_ranges.push((d, u));
    } else if d < 0 && u > 0 {
        y_ranges.push((d, -1));
        y_ranges.push((1, u));
    } else if d == 0 && u > 0 {
        y_ranges.push((1, u));
    } else {
        y_ranges.push((d, u));
    }
    // dbg!(&x_ranges);
    // dbg!(&y_ranges);

    let term1 = iproduct!(x_ranges, y_ranges)
        .map(|((x1, x2), (y1, y2))| {
            //
            sub3(x1, x2, y1, y2)
        })
        .sum::<i64>();

    // dbg!(term1);
    //if

    // 軸上
    let term2 = {
        //
        let mut cnt = 0;
        if (l..=r).contains(&0) {
            cnt += (d..=u).filter(|y| y % 2 == 0).count() as i64;
        }

        if (d..=u).contains(&0) {
            cnt += (l..=r).filter(|x| x % 2 == 0).count() as i64;
        }

        if (l..=r).contains(&0) && (d..=u).contains(&0) {
            cnt -= 1;
        }
        cnt
    };
    let ans: i64 = term1 + term2;
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
        assert_eq!(sub1(4, 5), 10);
        assert_eq!(sub1(3, 5), 6);
        assert_eq!(sub1(3, 4), 6);
        assert_eq!(sub1(3, 3), 3);
        assert_eq!(sub1(2, 5), 5);
        assert_eq!(sub1(5, 2), 5);
        assert_eq!(sub1(5, 0), 0);
        assert_eq!(sub1(4, 0), 0);
        assert_eq!(sub1(3, 0), 0);
        assert_eq!(sub1(2, 0), 0);
        assert_eq!(sub1(1, 0), 0);
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
use std::mem::swap;
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
