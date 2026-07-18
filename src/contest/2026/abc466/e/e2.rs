/*
    [解法]

    選ぶ選ばないのDP(ひっくり返すかどうか)

*/
fn main() {
    input! {
        n: usize,
        k: usize,
        abs: [(i64, i64); n],
    }

    let ds = abs.iter().copied().map(|(a, b)| b - a).collect_vec();

    // dp[i][k][p] = [0, i) まで見て、k個の区間をひっくり返して、i-1番目をひっくり返したか? = p となる場合の
    // ひっくり返した添字の ds の値の総和の最大値
    let mut dp = vec![vec![[NEG_INF; 2]; k + 1]; n + 1];

    // 初期値決定
    {
        //最初ひっくり返す (制約外だが、K=0でエラーになるので注意)
        dp[1][1][true as usize] = fin(ds[0]);

        // 最初ひっくり返さない
        dp[1][0][false as usize] = fin(0);
    }

    for i in 2..=n {
        for ki in 0..=k {
            chmax!(
                dp[i][ki][true as usize],
                dp[i - 1][ki][true as usize] + fin(ds[i - 1])
            );
            if ki >= 1 {
                chmax!(
                    dp[i][ki][true as usize],
                    dp[i - 1][ki - 1][false as usize] + fin(ds[i - 1])
                );
            }

            chmax!(dp[i][ki][false as usize], dp[i - 1][ki][true as usize]);
            chmax!(dp[i][ki][false as usize], dp[i - 1][ki][false as usize]);
        }
    }

    let before_sum = abs.iter().copied().map(|(a, _)| a).sum::<i64>();
    let max_diff = dp[n].iter().flatten().max().unwrap().get_fin();
    let ans: i64 = before_sum + max_diff;
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
use mod_neg_ext_int::*;
pub mod mod_neg_ext_int {
    use ac_library::Monoid;
    use std::{
        cmp::Ordering,
        convert::Infallible,
        fmt,
        ops::{Add, AddAssign, Mul, Sub, SubAssign},
    };
    pub const NEG_INF: NegExtInt = NegExtInt::NEG_INF;
    pub fn fin(x: i64) -> NegExtInt {
        NegExtInt::fin(x)
    }
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
            self * t
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
    impl Mul<i64> for NegExtInt {
        type Output = NegExtInt;
        fn mul(self, rhs: i64) -> Self::Output {
            match rhs.cmp(&0) {
                Ordering::Less => panic!("multiplier must be non-negative."),
                Ordering::Equal => Self::fin(0),
                Ordering::Greater => {
                    if self.is_fin() {
                        Self::fin(self.0 * rhs)
                    } else {
                        Self::NEG_INF
                    }
                }
            }
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
pub mod chminmax {
    #[allow(unused_macros)]
    #[macro_export]
    macro_rules! chmin {
        ($ a : expr_2021 , $ b : expr_2021 ) => {
            if $a > $b {
                $a = $b;
                true
            } else {
                false
            }
        };
    }
    #[allow(unused_macros)]
    #[macro_export]
    macro_rules! chmax {
        ($ a : expr_2021 , $ b : expr_2021 ) => {
            if $a < $b {
                $a = $b;
                true
            } else {
                false
            }
        };
    }
}
