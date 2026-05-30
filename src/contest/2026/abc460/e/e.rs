use ac_library::ModInt998244353 as Mint;

fn solve(n: i128, m: i128) -> Mint {
    (1..=19)
        .map(|k| {
            // k: 桁
            if n < 10_i128.pow(k - 1) {
                // たとえば k=3, n=99の場合
                return Mint::new(0);
            }
            let y_begin = 10_i128.pow(k - 1);
            let y_end = 10_i128.pow(k).min(n + 1);
            let y_cand = Mint::new(y_end - y_begin);

            // 10^k * x + y ≡ x + y (mod m)
            // 10^k * x ≡ x (mod m)
            // (10^k - 1) x ≡ 0 (mod m)
            // A = 10^k - 1 とおいて
            // ax = my
            // n = 13, m = 4, a=99 のとき
            // x = 4, 8, 12 (floor(13/4)個)
            // 一般には floor(n/(m/ gcd(a,m))個

            let a = 10_i128.pow(k) - 1;
            let x_cand = Mint::new(n / (m / a.gcd(&m)));
            x_cand * y_cand
        })
        .sum::<Mint>()
}

#[fastout]
fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        input! {
            n: i128,
            m: i128,
        }

        let ans = solve(n, m);
        println!("{}", ans);
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
        let x = 4;
        let y = 2;
        // dbg!(solve_naive(10, 11));
    }

    fn solve_naive(n: i128, m: i128) -> Mint {
        let ans = iproduct!(1..=n, 1..=n)
            .filter(|&(x, y)| {
                let concat_xy = format!("{}{}", x, y).parse::<i128>().unwrap();
                (concat_xy % m) == ((x + y) % m)
            })
            .count();
        Mint::new(ans)
    }

    /// 間違っていたら false を返す
    fn process_one_test(rng: &mut SmallRng) -> bool {
        // ==== 問題を作る ====
        let n = rng.random_range(1..=1000); // n は大きいほうがよさそう
        let m = rng.random_range(2..=1000);

        // ==== 解く ====
        let main_ans = solve(n, m);
        let naive_ans = solve_naive(n, m);

        // ==== 間違っていたら報告をする ====
        if main_ans != naive_ans {
            // 問題を出力
            println!("{:?}", (n, m));
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
        let num_tests = 1000;
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
use num::Integer;
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
