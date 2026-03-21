// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize, // 金額の上限
        pvs: [(usize, i64); n],
    }
    // dp1[i][p] = [0, i) での商品をp円以下で選ぶときの価値最大値
    let mut dp1 = vec![vec![NEG_INF; m + 1]; n + 1];
    for p in 0..=m {
        dp1[0][p] = fin(0);
    }

    for (i, (item_p, item_v)) in pvs.iter().copied().enumerate() {
        for p in 0..=m {
            // 現在のアイテムを選択する/しない
            let choose = try_opt!(dp1[i][p.checked_sub(item_p)?]).unwrap_or(NEG_INF) + item_v;
            let no_choose = dp1[i][p];

            dp1[i + 1][p] = choose.max(no_choose);
        }
    }

    // dp2[i][p] = [n-i, n) での商品をp円以下で選ぶときの価値最大値
    let mut dp2 = vec![vec![NEG_INF; m + 1]; n + 1];
    for p in 0..=m {
        dp2[0][p] = fin(0);
    }

    for (i, (item_p, item_v)) in pvs.iter().copied().rev().enumerate() {
        for p in 0..=m {
            // 現在のアイテムを選択する/しない
            let choose = try_opt!(dp2[i][p.checked_sub(item_p)?]).unwrap_or(NEG_INF) + item_v;
            let no_choose = dp2[i][p];

            dp2[i + 1][p] = no_choose.max(choose);
        }
    }
    let max_dp = dp1[n][m];

    let ans = (0..n)
        .map(|i| {
            let (item_p_i, item_v_i) = pvs[i];
            let dir1 = &dp1[i]; //[0, i)
            let dir2 = &dp2[n - i - 1]; // [i + 1, n)

            // i を含まない
            let val_exclude_i = (0..=m)
                .map(|p| {
                    // dir1 から p
                    // dir2 から m - p
                    dir1[p] + dir2[m - p]
                })
                .max()
                .unwrap();
            // i を含む
            let val_include_i = (0..=m)
                .map(|p| {
                    // dir1 から p
                    // i から item_p_i
                    // dir2 から m - p - item_p_i

                    let prefix = dir1[p];
                    let suffix = try_opt!(dir2[m.checked_sub(p + item_p_i)?]).unwrap_or(NEG_INF);
                    prefix + item_v_i + suffix
                })
                .max()
                .unwrap();

            if val_exclude_i != max_dp {
                // i は選ばないといけない
                'A'
            } else if val_include_i == max_dp {
                // i は選んでも選ばなくてもよい
                'B'
            } else {
                // i は選んではいけない
                'C'
            }
        })
        .collect_vec();

    print_chars(&ans);
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
use {mod_neg_ext_int::*, rand::distr::slice::Choose};
pub mod mod_neg_ext_int {
    use ac_library::Monoid;
    use std::{
        cmp::Ordering,
        convert::Infallible,
        fmt,
        ops::{Add, AddAssign, Sub, SubAssign},
    };
    pub const NEG_INF: NegExtInt = NegExtInt::NEG_INF;
    pub fn fin(x: i64) -> NegExtInt {
        NegExtInt::fin(x)
    }
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct NegExtInt(i64);
    impl NegExtInt {
        pub const NEG_INF: Self = Self(i64::MIN);
        pub fn fin(x: i64) -> Self {
            Self(x)
        }
        pub fn get_fin(self) -> i64 {
            if self.is_fin() {
                self.0
            } else {
                panic!("called `NegExtInt::get_fin()` on a negative infinity")
            }
        }
        pub fn get_fin_or(self, default: i64) -> i64 {
            if self.is_fin() { self.0 } else { default }
        }
        #[inline]
        pub fn is_fin(self) -> bool {
            self.0 != i64::MIN
        }
        pub fn is_neg_inf(self) -> bool {
            self.0 == i64::MIN
        }
        pub fn to_option(self) -> Option<i64> {
            if self.is_fin() { Some(self.0) } else { None }
        }
        pub fn from_option(opt: Option<i64>) -> NegExtInt {
            match opt {
                Some(a) => Self(a),
                None => Self::NEG_INF,
            }
        }
        pub fn times(self, t: i64) -> Self {
            match t.cmp(&0) {
                Ordering::Less => panic!("t must be non-negative."),
                Ordering::Equal => Self(0),
                Ordering::Greater => {
                    if self.is_fin() {
                        Self(self.0 * t)
                    } else {
                        Self::NEG_INF
                    }
                }
            }
        }
    }
    impl Add for NegExtInt {
        type Output = NegExtInt;
        fn add(self, rhs: Self) -> Self::Output {
            if self.is_neg_inf() || rhs.is_neg_inf() {
                Self::NEG_INF
            } else {
                Self::fin(self.0 + rhs.0)
            }
        }
    }
    impl AddAssign for NegExtInt {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }
    impl Add<i64> for NegExtInt {
        type Output = NegExtInt;
        fn add(self, rhs: i64) -> Self::Output {
            if self.is_neg_inf() {
                Self::NEG_INF
            } else {
                Self::fin(self.0 + rhs)
            }
        }
    }
    impl AddAssign<i64> for NegExtInt {
        fn add_assign(&mut self, rhs: i64) {
            *self = *self + rhs;
        }
    }
    impl Sub<i64> for NegExtInt {
        type Output = NegExtInt;
        fn sub(self, rhs: i64) -> Self::Output {
            if self.is_neg_inf() {
                Self::NEG_INF
            } else {
                Self::fin(self.0 - rhs)
            }
        }
    }
    impl SubAssign<i64> for NegExtInt {
        fn sub_assign(&mut self, rhs: i64) {
            *self = *self - rhs;
        }
    }
    impl std::iter::Sum for NegExtInt {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            let mut s = 0;
            for x in iter {
                if x.is_neg_inf() {
                    return Self::NEG_INF;
                }
                s += x.0;
            }
            Self::fin(s)
        }
    }
    impl fmt::Display for NegExtInt {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.is_neg_inf() {
                write!(f, "-∞")
            } else {
                write!(f, "{}", self.0)
            }
        }
    }
    impl fmt::Debug for NegExtInt {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.is_neg_inf() {
                write!(f, "-∞")
            } else {
                write!(f, "{}", self.0)
            }
        }
    }
    pub struct NegExtIntAdditive(Infallible);
    impl Monoid for NegExtIntAdditive {
        type S = NegExtInt;
        fn identity() -> Self::S {
            NegExtInt::fin(0)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
    }
    pub struct NegExtIntMax(Infallible);
    impl Monoid for NegExtIntMax {
        type S = NegExtInt;
        fn identity() -> Self::S {
            NegExtInt::NEG_INF
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a.max(b)
        }
    }
}

#[allow(clippy::module_inception)]
#[macro_use]
pub mod try_opt {
    /// `?` 演算子を式・ブロックの中で使うためのマクロ。
    /// IIFE `(|| Some(...))()` の代替として用いる。
    /// 複文ブロックは `try` ブロックと同様に書け、最後の式が自動的に `Some` で包まれる。
    /// # Example
    /// ```
    /// # use mylib::try_opt;
    /// let dp = [[10_i64, 20, 30]];
    /// let neg_inf = i64::MIN / 2;
    /// // 単一式
    /// let item_p = 1_usize;
    /// let p = 2_usize;
    /// let val = try_opt!(dp[0][p.checked_sub(item_p)?]).unwrap_or(neg_inf);
    /// assert_eq!(val, 20);
    /// let p = 0_usize;
    /// let val = try_opt!(dp[0][p.checked_sub(item_p)?]).unwrap_or(neg_inf);
    /// assert_eq!(val, neg_inf);
    /// // 複文ブロック（try ブロックと同様に書ける）
    /// let f = |n: i32| if n > 0 { Some(n * 10) } else { None };
    /// let result = try_opt! {
    ///     let a = f(1)?;
    ///     let b = f(2)?;
    ///     a + b
    /// };
    /// assert_eq!(result, Some(30));
    /// let result = try_opt! {
    ///     let a = f(1)?;
    ///     let b = f(-1)?;
    ///     a + b
    /// };
    /// assert_eq!(result, None);
    /// ```
    #[allow(unused_macros)]
    #[macro_export]
    macro_rules ! try_opt {($ e : expr ) => {(|| Some ($ e ) ) () } ; ($ ($ t : tt ) * ) => {(|| Some ({$ ($ t ) * } ) ) () } ; }
}
