// 問題文と制約は読みましたか？

// s は t の（連続とは限らない）部分列か？
fn pred(s: &[char], t: &[char]) -> bool {
    let mut s_iter = s.iter().copied().peekable();
    let mut t_iter = t.iter().copied().peekable();
    // s を全部食べれたら OK

    loop {
        if s_iter.peek() == t_iter.peek() {
            s_iter.next();
            t_iter.next();
        } else {
            t_iter.next();
        }
        if s_iter.peek().is_none() {
            return true;
        }

        if t_iter.peek().is_none() {
            return false;
        }
    }
    panic!()
}
// #[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let n = s.len();

    let mut begin = 0;
    let mut end = 0;
    let mut len_sum = 0;

    while begin < n {
        // begin..end が条件を満たす範囲で end を繰り返し進める
        while end < n {
            // end を1進めたときに条件を満たさなくなる場合は break
            if pred(&t, &s[begin..end + 1]) {
                break;
            }

            // end を進める
            end += 1;
        }
        // dbg!(begin);
        // dbg!(end);

        len_sum += end - begin;

        if begin == end {
            end += 1; // begin が end を追い抜かなさいように end も進める。
            begin += 1;
        } else {
            // begin を進める
            begin += 1;
        }
    }

    // dbg!(len_sum);
    // dbg!(n * (n + 1) / 2);

    let ans: usize = len_sum;
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
        dbg!(pred(
            &"ac".chars().collect_vec(),
            &"abc".chars().collect_vec()
        ));

        dbg!(pred(
            &"".chars().collect_vec(),
            &"abc".chars().collect_vec()
        ));
        dbg!(pred(
            &"c".chars().collect_vec(),
            &"abc".chars().collect_vec()
        ));

        dbg!(pred(
            &"ba".chars().collect_vec(),
            &"abc".chars().collect_vec()
        ));

        dbg!(pred(
            &"acb".chars().collect_vec(),
            &"abc".chars().collect_vec()
        ));

        dbg!(pred(
            &"abr".chars().collect_vec(),
            &"abc".chars().collect_vec()
        ));
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
