// 問題文と制約は読みましたか？
// #[fastout]
fn main() {
    input! {
        n: i64,
        k: i64,
    }
    let ans: i64 = (1..=n)
        .filter(|x| {
            let sum = to_base_n_value_iter(*x, 10).sum::<i64>();
            sum == k
        })
        .count() as i64;
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
use positional_notation::*;
#[allow(clippy::module_inception)]
pub mod positional_notation {
    /// 配列 xs で表された base 進数の値を評価する。
    /// 例: `eval_base_n_value(&[1, 2, 3], 10) == 123`
    pub fn eval_base_n_value(xs: &[i64], base: i64) -> i64 {
        xs.iter().fold(0, |acc, &x| acc * base + x)
    }
    /// x の base 進数での表記を Vec で表す。
    /// 例
    /// * `to_base_n_value(123, 10) == vec![1, 2, 3]`
    /// * `to_base_n_value(0, 10) == vec![]`
    pub fn to_base_n_value(x: i64, base: i64) -> Vec<i64> {
        assert!(x >= 0);
        assert!(base >= 2);
        let mut ret = vec![];
        let mut x = x;
        while x > 0 {
            ret.push(x % base);
            x /= base;
        }
        ret.reverse();
        ret
    }
    /// x を base 進数で表記した際の桁数を返す。
    /// 例
    /// * `count_digits(123, 10) == 3`
    /// * `count_digits(0, 10) == 0`
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
    /// `to_base_n_value_iter` 関数が返すイテレータ。
    /// 数値を指定された基数で表した際の各桁を順に生成する。
    pub struct BaseNIterator {
        n: i64,
        base: i64,
        current_power: i64,
    }
    impl Iterator for BaseNIterator {
        type Item = i64;
        fn next(&mut self) -> Option<Self::Item> {
            if self.current_power == 0 {
                return None;
            }
            let digit = self.n / self.current_power;
            self.n %= self.current_power;
            self.current_power /= self.base;
            Some(digit)
        }
    }
    /// x の base 進数での表記をイテレータで返す。
    /// 例
    /// - `to_base_n_value_iter(123, 10).collect() == vec![1, 2, 3]`
    /// - `to_base_n_value_iter(0, 10).collect() == vec![]`
    pub fn to_base_n_value_iter(x: i64, base: i64) -> BaseNIterator {
        assert!(x >= 0);
        assert!(base >= 2);
        if x == 0 {
            return BaseNIterator {
                n: 0,
                base,
                current_power: 0,
            };
        }
        let mut current_power = 1;
        while x / current_power >= base {
            current_power *= base;
        }
        BaseNIterator {
            n: x,
            base,
            current_power,
        }
    }
}
