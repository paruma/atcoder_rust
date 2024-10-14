use std::cmp::max;

#[allow(unused_macros)]
macro_rules! chmax {
    ($a: expr, $b: expr) => {
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
                dp: vec![vec![NegInf; max_weight + 1]; n + 1],
            }
        }

        fn at(&self, i: usize, w: i64) -> &NegExtInt {
            // 添字の小さい方だけ考慮すればよい（大きい方はスルー）
            if w < 0 {
                &NegInf
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
        *dp.at_mut(0, w) = Fin(0);
    }

    for (i, item) in items.iter().enumerate() {
        for w in 0..=max_weight {
            // 現在のアイテムを選択する/しない
            let choose = *dp.at(i, w - item.weight) + Fin(item.value);
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
                dp: vec![vec![NegInf; max_weight + 1]; n + 1],
            }
        }

        fn at(&self, i: usize, w: i64) -> &NegExtInt {
            // 添字の小さい方だけ考慮すればよい（大きい方はスルー）
            if w < 0 {
                // 配る場合、この if 文は不要
                &NegInf
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
        *dp.at_mut(0, w) = Fin(0);
    }

    for (i, item) in items.iter().enumerate() {
        for w in 0..=max_weight {
            // 現在のアイテムを選択する/しない
            // (i, w) の状態から配っていく。
            let choose = *dp.at(i, w) + Fin(item.value);
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
                dp: vec![vec![NegInf; max_weight + 1]; n + 1],
            }
        }

        fn at(&self, i: usize, w: i64) -> &NegExtInt {
            // 添字の小さい方だけ考慮すればよい（大きい方はスルー）
            if w < 0 {
                &NegInf
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
        *dp.at_mut(0, w) = Fin(0);
    }

    for (i, item) in items.iter().enumerate() {
        for w in 0..=max_weight {
            // 現在のアイテムを選択する/しない
            let choose = *dp.at(i, w - item.weight) + Fin(item.value);
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
use mod_neg_ext_int::NegExtInt::{self, *};
pub mod mod_neg_ext_int {
    use std::{cmp::Ordering, ops::Add};
    use NegExtInt::*;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NegExtInt {
        NegInf,
        Fin(i64),
    }
    #[allow(dead_code)]
    impl NegExtInt {
        pub fn get_fin(self) -> i64 {
            match self {
                Fin(val) => val,
                NegInf => panic!("called `NegExtInt::get_fin()` on a `Fin` value"),
            }
        }
        pub fn get_fin_or(self, default: i64) -> i64 {
            match self {
                Fin(val) => val,
                NegInf => default,
            }
        }
        pub fn is_fin(self) -> bool {
            matches!(self, Fin(_))
        }
        pub fn is_neginf(self) -> bool {
            matches!(self, NegInf)
        }
        pub fn to_option(self) -> Option<i64> {
            match self {
                NegInf => None,
                Fin(a) => Some(a),
            }
        }
        pub fn from_option(opt: Option<i64>) -> NegExtInt {
            match opt {
                Some(a) => Fin(a),
                None => NegInf,
            }
        }
    }
    impl Add for NegExtInt {
        type Output = NegExtInt;
        fn add(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (NegInf, NegInf) => NegInf,
                (NegInf, Fin(_)) => NegInf,
                (Fin(_), NegInf) => NegInf,
                (Fin(a), Fin(b)) => Fin(a + b),
            }
        }
    }
    impl PartialOrd for NegExtInt {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match (self, other) {
                (NegInf, NegInf) => Some(Ordering::Equal),
                (NegInf, Fin(_)) => Some(Ordering::Less),
                (Fin(_), NegInf) => Some(Ordering::Greater),
                (Fin(a), Fin(b)) => PartialOrd::partial_cmp(a, b),
            }
        }
    }
    impl Ord for NegExtInt {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }
}

//---------snippet---------
