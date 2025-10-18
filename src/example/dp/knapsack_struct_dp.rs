use std::cmp::max;

#[allow(unused_macros)]
macro_rules! chmax {
    ($a: expr_2021, $b: expr_2021) => {
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Item {
    weight: i64,
    value: i64,
}

/// 計算量: O(|items| * max_weight)
#[allow(dead_code)]
fn knapsack(n: usize, items: &[Item], max_weight: i64) -> i64 {
    // dp[i][w] := [0,i) の items を使用したときの重さ w 以下での価値の最大値
    // dp[0][0] = 0
    // dp[0][w] = 0 (w!=0)
    // dp[i+1][w] = max(dp[i][w], dp[i][w-items[i].weight] + items[i].value);
    // 答えは必ず存在する。max_weight>=0なら、重さmax_weight以下となるような選び方は必ず存在するので（何も選ばないという選び方）

    struct Dp {
        dp: Vec<Vec<NegExtInt>>,
    }
    impl Dp {
        fn new(n: usize, max_weight: i64) -> Dp {
            let max_weight = max_weight as usize;
            Dp {
                dp: vec![vec![NEG_INF; max_weight + 1]; n + 1],
            }
        }

        fn at(&self, i: usize, w: i64) -> &NegExtInt {
            // 添字の小さい方だけ考慮すればよい（大きい方はスルー）
            if w < 0 {
                &NEG_INF
            } else {
                &self.dp[i][w as usize]
            }
        }

        fn at_mut(&mut self, i: usize, w: i64) -> &mut NegExtInt {
            &mut self.dp[i][w as usize]
        }
    }

    let mut dp = Dp::new(n, max_weight);

    for w in 0..=max_weight {
        *dp.at_mut(0, w) = fin(0);
    }

    for (i, item) in items.iter().enumerate() {
        for w in 0..=max_weight {
            // 現在のアイテムを選択する/しない
            let choose = *dp.at(i, w - item.weight) + fin(item.value);
            let no_choose = *dp.at(i, w);
            *dp.at_mut(i + 1, w) = max(choose, no_choose);
        }
    }
    dp.at(n, max_weight).get_fin()
}

/// 計算量: O(|items| * max_weight)
#[allow(dead_code)]
fn knapsack_kubaru(n: usize, items: &[Item], max_weight: i64) -> i64 {
    // dp[i][w] := [0,i) の items を使用したときの重さ w 以下での価値の最大値
    // dp[0][0] = 0
    // dp[0][w] = 0 (w!=0)
    // dp[i+1][w] = max(dp[i][w], dp[i][w-items[i].weight] + items[i].value);
    // 答えは必ず存在する。max_weight>=0なら、重さmax_weight以下となるような選び方は必ず存在するので（何も選ばないという選び方）

    struct Dp {
        dp: Vec<Vec<NegExtInt>>,
    }
    impl Dp {
        fn new(n: usize, max_weight: i64) -> Dp {
            let max_weight = max_weight as usize;
            Dp {
                dp: vec![vec![NEG_INF; max_weight + 1]; n + 1],
            }
        }

        fn at(&self, i: usize, w: i64) -> &NegExtInt {
            // 添字の小さい方だけ考慮すればよい（大きい方はスルー）
            if w < 0 {
                // 配る場合、この if 文は不要
                &NEG_INF
            } else {
                &self.dp[i][w as usize]
            }
        }

        fn at_mut(&mut self, i: usize, w: i64) -> &mut NegExtInt {
            &mut self.dp[i][w as usize]
        }
    }

    let mut dp = Dp::new(n, max_weight);

    for w in 0..=max_weight {
        *dp.at_mut(0, w) = fin(0);
    }

    for (i, item) in items.iter().enumerate() {
        for w in 0..=max_weight {
            // 現在のアイテムを選択する/しない
            // (i, w) の状態から配っていく。
            let choose = *dp.at(i, w) + fin(item.value);
            let no_choose = *dp.at(i, w);
            if w + item.weight <= max_weight {
                chmax!(*dp.at_mut(i + 1, w + item.weight), choose);
            }
            chmax!(*dp.at_mut(i + 1, w), no_choose);
        }
    }
    dp.at(n, max_weight).get_fin()
}

#[allow(dead_code)]
fn knapsack_with_restore(n: usize, items: &[Item], max_weight: i64) -> (i64, Vec<bool>) {
    // dp[i][w] := [0,i) の items を使用したときの重さ w 以下での価値の最大値
    // dp[0][0] = 0
    // dp[0][w] = 0 (w!=0)
    // dp[i+1][w] = max(dp[i][w], dp[i][w-items[i].weight] + items[i].value);
    // 答えは必ず存在する。max_weight>=0なら、重さmax_weight以下となるような選び方は必ず存在するので（何も選ばないという選び方）

    struct Dp {
        dp: Vec<Vec<NegExtInt>>,
    }
    impl Dp {
        fn new(n: usize, max_weight: i64) -> Dp {
            let max_weight = max_weight as usize;
            Dp {
                dp: vec![vec![NEG_INF; max_weight + 1]; n + 1],
            }
        }

        fn at(&self, i: usize, w: i64) -> &NegExtInt {
            // 添字の小さい方だけ考慮すればよい（大きい方はスルー）
            if w < 0 {
                &NEG_INF
            } else {
                &self.dp[i][w as usize]
            }
        }

        fn at_mut(&mut self, i: usize, w: i64) -> &mut NegExtInt {
            &mut self.dp[i][w as usize]
        }
    }

    // dp[i][w] := [0,i) の items を使用したときの重さ w 以下での価値の最大値
    let mut dp = Dp::new(n, max_weight);

    // prev[i][w] := [0, i + 1) の items を使用したときの重さ w 以下での価値を最大化
    // したとき、i番目の item を使用したかどうか。
    let mut prev = vec![vec![false; max_weight as usize + 1]; n];

    for w in 0..=max_weight {
        *dp.at_mut(0, w) = fin(0);
    }

    for (i, item) in items.iter().enumerate() {
        for w in 0..=max_weight {
            // 現在のアイテムを選択する/しない
            let choose = *dp.at(i, w - item.weight) + fin(item.value);
            let no_choose = *dp.at(i, w);
            if choose > no_choose {
                *dp.at_mut(i + 1, w) = choose;
                prev[i][w as usize] = true;
            } else {
                *dp.at_mut(i + 1, w) = no_choose;
                prev[i][w as usize] = false;
            }
        }
    }
    // 各アイテムを選んだかどうかを復元
    let path = {
        let mut path = vec![false; n];
        let mut current_weight = max_weight;
        for i in (0..n).rev() {
            path[i] = prev[i][current_weight as usize];
            if path[i] {
                current_weight -= items[i].weight;
            }
        }
        path
    };

    (dp.at(n, max_weight).get_fin(), path)
}
#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_knapsack() {
        let n: usize = 6;
        let items = [(2, 3), (1, 2), (3, 6), (2, 1), (1, 3), (5, 85)];
        let items = items
            .iter()
            .map(|(weight, value)| Item {
                weight: *weight,
                value: *value,
            })
            .collect_vec();

        let max_weight = 9;

        let ans = knapsack(n, &items, max_weight);
        assert_eq!(ans, 94);

        let ans = knapsack_kubaru(n, &items, max_weight);
        assert_eq!(ans, 94);
    }

    #[test]
    fn test_knapsack_restore() {
        let n: usize = 6;
        let items = [(2, 3), (1, 2), (3, 6), (2, 1), (1, 3), (5, 85)];
        let items = items
            .iter()
            .map(|(weight, value)| Item {
                weight: *weight,
                value: *value,
            })
            .collect_vec();

        let max_weight = 9;

        let (ans, path) = knapsack_with_restore(n, &items, max_weight);
        assert_eq!(ans, 94);
        assert_eq!(
            (0..n)
                .filter(|i| path[*i])
                .map(|i| items[i].value)
                .sum::<i64>(),
            94
        )
    }
}

//---------snippet---------
use mod_neg_ext_int::*;
#[allow(dead_code)]
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
            if self.is_fin() {
                self.0
            } else {
                default
            }
        }
        #[inline]
        pub fn is_fin(self) -> bool {
            self.0 != i64::MIN
        }
        pub fn is_neg_inf(self) -> bool {
            self.0 == i64::MIN
        }
        pub fn to_option(self) -> Option<i64> {
            if self.is_fin() {
                Some(self.0)
            } else {
                None
            }
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

//---------snippet---------
