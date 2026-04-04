use mylib::math::neg_ext_int::mod_neg_ext_int::*;
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
    weight: usize,
    value: i64,
}

/// 計算量: O(|items| * max_weight)
#[allow(dead_code)]
fn knapsack(n: usize, items: &[Item], max_weight: usize) -> i64 {
    // dp[i][w] := [0,i) の items を使用したときの重さ w 以下での価値の最大値
    // dp[0][0] = 0
    // dp[0][w] = 0 (w!=0)
    // dp[i+1][w] = max(dp[i][w], dp[i][w-items[i].weight] + items[i].value);
    // 答えは必ず存在する。max_weight>=0なら、重さmax_weight以下となるような選び方は必ず存在するので（何も選ばないという選び方）

    let mut dp = vec![vec![NEG_INF; max_weight + 1]; n + 1];

    for w in 0..=max_weight {
        dp[0][w] = fin(0);
    }

    for (i, item) in items.iter().enumerate() {
        for w in 0..=max_weight {
            // 現在のアイテムを選択する/しない
            let choose = if w < item.weight {
                NEG_INF
            } else {
                dp[i][w - item.weight] + fin(item.value)
            };
            let no_choose = dp[i][w];
            dp[i + 1][w] = max(choose, no_choose);
        }
    }
    dp[n][max_weight].get_fin()
}

/// 計算量: O(|items| * max_weight)
#[allow(dead_code)]
fn knapsack_kubaru(n: usize, items: &[Item], max_weight: usize) -> i64 {
    // dp[i][w] := [0,i) の items を使用したときの重さ w 以下での価値の最大値
    // dp[0][0] = 0
    // dp[0][w] = 0 (w!=0)
    // dp[i+1][w] = max(dp[i][w], dp[i][w-items[i].weight] + items[i].value);
    // 答えは必ず存在する。max_weight>=0なら、重さmax_weight以下となるような選び方は必ず存在するので（何も選ばないという選び方）

    let mut dp = vec![vec![NEG_INF; max_weight + 1]; n + 1];

    for w in 0..=max_weight {
        dp[0][w] = fin(0);
    }

    for (i, item) in items.iter().enumerate() {
        for w in 0..=max_weight {
            // 現在のアイテムを選択する/しない
            // (i, w) の状態から次の状態に配っていく。
            let choose = dp[i][w] + item.value;
            let no_choose = dp[i][w];
            if w + item.weight <= max_weight {
                chmax!(dp[i + 1][w + item.weight], choose);
            }
            chmax!(dp[i + 1][w], no_choose);
        }
    }
    dp[n][max_weight].get_fin()
}

#[allow(dead_code)]
fn knapsack_with_restore(n: usize, items: &[Item], max_weight: usize) -> (i64, Vec<bool>) {
    // dp[i][w] := [0,i) の items を使用したときの重さ w 以下での価値の最大値
    // dp[0][0] = 0
    // dp[0][w] = 0 (w!=0)
    // dp[i+1][w] = max(dp[i][w], dp[i][w-items[i].weight] + items[i].value);
    // 答えは必ず存在する。max_weight>=0なら、重さmax_weight以下となるような選び方は必ず存在するので（何も選ばないという選び方）

    // dp[i][w] := [0,i) の items を使用したときの重さ w 以下での価値の最大値
    let mut dp = vec![vec![NEG_INF; max_weight + 1]; n + 1];

    // prev[i][w] := [0, i + 1) の items を使用したときの重さ w 以下での価値を最大化
    // したとき、i番目の item を使用したかどうか。
    let mut prev = vec![vec![false; max_weight + 1]; n];

    for w in 0..=max_weight {
        dp[0][w] = fin(0);
    }

    for (i, item) in items.iter().enumerate() {
        for w in 0..=max_weight {
            // 現在のアイテムを選択する/しない
            let choose = if w < item.weight {
                NEG_INF
            } else {
                dp[i][w - item.weight] + item.value
            };
            let no_choose = dp[i][w];

            if choose > no_choose {
                dp[i + 1][w] = choose;
                prev[i][w] = true;
            } else {
                dp[i + 1][w] = no_choose;
                prev[i][w] = false;
            }
        }
    }
    // 各アイテムを選んだかどうかを復元
    let path = {
        let mut path = vec![false; n];
        let mut current_weight = max_weight;
        for i in (0..n).rev() {
            path[i] = prev[i][current_weight];
            if path[i] {
                current_weight -= items[i].weight;
            }
        }
        path
    };

    (dp[n][max_weight].get_fin(), path)
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
}

