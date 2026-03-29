// a と b を10進数の文字列として連結して得られる値
fn concat(a: i64, b: i64) -> i64 {
    a * 10_i64.pow(count_digits(b, 10) as u32) + b
}
// 問題文と制約は読みましたか？
// #[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut pq: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
    let pow2 = (0..)
        .map_while(|i| 2_i64.checked_pow(i as u32))
        .take_while(|&k| k <= 1_000_000_000)
        .collect_vec();
    let mut visited: HashSet<i64> = HashSet::new();

    for &p in &pow2 {
        pq.push(Reverse(p));
        visited.insert(p);
    }

    let mut cnt = 0;
    let mut ans = i64::MAX;

    while let Some(Reverse(cur)) = pq.pop() {
        if cnt == n - 1 {
            ans = cur;
            break;
        }

        // 10^9 より大きい値は入れない

        for &p in &pow2 {
            let next = concat(cur, p);
            if next <= 1_000_000_000 && !visited.contains(&next) {
                pq.push(Reverse(next));
                visited.insert(next);
            }
            //
        }

        cnt += 1;
    }

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
        dbg!(concat(13, 54));
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
use digit::*;
#[allow(clippy::module_inception)]
pub mod digit {
    /// n の base 進数を Little Endian で表す
    /// 例: `to_digits_le_vec(123, 10) == vec![3, 2, 1]`
    pub fn to_digits_le_vec(mut n: i64, base: i64) -> Vec<i64> {
        assert!(n >= 0);
        assert!(base >= 2);
        if n == 0 {
            return vec![];
        }
        let mut res = vec![];
        while n > 0 {
            res.push(n % base);
            n /= base;
        }
        res
    }
    /// n の base 進数を Little Endian で生成するイテレータ
    /// 例: `to_digits_le_iter(123, 10).collect::<Vec<_>>() == vec![3, 2, 1]`
    pub fn to_digits_le_iter(n: i64, base: i64) -> impl Iterator<Item = i64> {
        assert!(n >= 0);
        assert!(base >= 2);
        DigitsLeIterator { n, base }
    }
    struct DigitsLeIterator {
        n: i64,
        base: i64,
    }
    impl Iterator for DigitsLeIterator {
        type Item = i64;
        fn next(&mut self) -> Option<Self::Item> {
            if self.n == 0 {
                return None;
            }
            let digit = self.n % self.base;
            self.n /= self.base;
            Some(digit)
        }
    }
    /// Little Endian で表された各桁から、数値を評価する
    /// 例: `from_digits_le(&[3, 2, 1], 10) == 123`
    pub fn from_digits_le(digits: &[i64], base: i64) -> i64 {
        assert!(base >= 2);
        digits.iter().rfold(0, |acc, &d| acc * base + d)
    }
    /// x を base 進数で表した際の桁数を返す
    /// 例: `count_digits(123, 10) == 3`
    pub fn count_digits(mut x: i64, base: i64) -> usize {
        assert!(x >= 0);
        assert!(base >= 2);
        if x == 0 {
            return 0;
        }
        let mut count = 0;
        while x > 0 {
            x /= base;
            count += 1;
        }
        count
    }
}
