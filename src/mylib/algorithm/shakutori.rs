// とりあえず作ってみたけど、あまり使いこなせる気がしない。
// もう少し尺取法必須の問題に触れてから考える。
// 今回は述語が単調減少のパターンしか扱ってないが、単調増加のパターンも欲しくなる？

use cargo_snippet::snippet;

#[snippet]
/// 尺取法を用いて、各開始位置 `left` に対して条件を満たす最大の `right` を計算する。
///
/// 連続する部分列 `target[left..right]` の集計状態を管理しながら、条件を満たす最大の範囲を探索する。
///
/// # 引数
/// * `target` - 走査対象の要素スライス
/// * `identity` - 空の区間における集計状態の初期値
/// * `p` - 現在の区間の集計状態に、右端の要素を新しく含めても条件を満たし続けられるか判定する述語
/// * `op` - 区間の右端を伸ばし、新しい要素を含めた集計状態を返す演算
/// * `inv_op` - 区間の左端を縮め、要素を除外した集計状態を返す演算
///
/// # 戻り値
/// 各開始位置 `left` (0..=target.len()) に対して、条件を満たす最大の `right` を格納した配列。
/// 各 `left` に対して、`target[left..right]` が条件を満たす最大の連続部分列となる。
pub fn shakutori_max_right<T, S, P, Op, InvOp>(
    target: &[T],
    identity: S,
    mut p: P,
    mut op: Op,
    mut inv_op: InvOp,
) -> Vec<usize>
where
    S: Clone,
    P: FnMut(&S, &T) -> bool,
    Op: FnMut(S, &T) -> S,
    InvOp: FnMut(S, &T) -> S,
{
    let n = target.len();
    let mut state = identity;
    let mut right = 0;
    let mut max_rights = Vec::with_capacity(n + 1);

    for left in 0..n {
        while right < n && p(&state, &target[right]) {
            state = op(state, &target[right]);
            right += 1;
        }

        max_rights.push(right);

        if left == right {
            right += 1;
        } else {
            state = inv_op(state, &target[left]);
        }
    }
    // left = n のケース
    max_rights.push(n);
    max_rights
}

#[snippet]
/// 尺取法を用いて、各終了位置 `right` に対して条件を満たす最小の `left` を計算する。
///
/// 連続する部分列 `target[left..right]` の集計状態を管理しながら、`right` を固定した際に条件を満たす最小の `left` を探索する。
///
/// # 引数
/// * `target` - 走査対象の要素スライス
/// * `identity` - 空の区間における集計状態の初期値
/// * `p` - 区間の左端の要素を除外しても、条件を満たし続けられるか判定する述語
/// * `op` - 区間の右端を伸ばし、新しい要素を含めた集計状態を返す演算
/// * `inv_op` - 区間の左端を縮め、要素を除外した集計状態を返す演算
///
/// # 戻り値
/// 各終了位置 `right` (0..=target.len()) に対して、条件を満たす最小の `left` を格納した配列。
/// 各 `right` に対して、`target[left..right]` が条件を満たす範囲で限界まで左端を削った連続部分列となる。
pub fn shakutori_min_left<T, S, P, Op, InvOp>(
    target: &[T],
    identity: S,
    mut p: P,
    mut op: Op,
    mut inv_op: InvOp,
) -> Vec<usize>
where
    S: Clone,
    P: FnMut(&S, &T) -> bool,
    Op: FnMut(S, &T) -> S,
    InvOp: FnMut(S, &T) -> S,
{
    let n = target.len();
    let mut state = identity;
    let mut left = 0;
    let mut min_lefts = Vec::with_capacity(n + 1);

    // right = 0 のケース
    min_lefts.push(0);

    for right in 0..n {
        state = op(state, &target[right]);
        while left <= right && p(&state, &target[left]) {
            state = inv_op(state, &target[left]);
            left += 1;
        }
        min_lefts.push(left);
    }
    min_lefts
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use rand::{Rng, SeedableRng, rngs::SmallRng};
    use std::collections::HashMap;

    #[test]
    fn test_max_len_with_sum_le() {
        let target = vec![2, 2, 1, 1, 3];
        let k = 5;

        let max_len = shakutori_max_right(
            &target,
            0,
            |&sum, &x| sum + x <= k,
            |sum, &x| sum + x,
            |sum, &x| sum - x,
        )
        .into_iter()
        .enumerate()
        .map(|(l, r)| r - l)
        .max()
        .unwrap_or(0);

        assert_eq!(max_len, 3);
    }

    #[test]
    fn test_min_len_with_sum_ge() {
        let target = vec![1, 2, 1, 3, 1, 2];
        let k = 4;

        // 合計が k 以上になる最小の長さを求める
        let res = shakutori_min_left(
            &target,
            0,
            |&sum, &x| sum - x >= k,
            |sum, &x| sum + x,
            |sum, &x| sum - x,
        );

        // res[r] は終了位置 r (0..=n) における最小の left。
        let min_len = res
            .into_iter()
            .enumerate()
            .filter(|&(r, l)| target[l..r].iter().sum::<i32>() >= k)
            .map(|(r, l)| r - l)
            .min();

        assert_eq!(min_len, Some(2)); // [1, 3] や [3, 1] が長さ 2
    }

    #[test]
    fn test_max_len_with_types_le() {
        let target = vec![1, 2, 3, 3, 4, 4, 5];
        let k = 3;

        let max_len = shakutori_max_right(
            &target,
            HashMap::new(),
            |bag, x| bag.len() < k || bag.contains_key(x),
            |mut bag, &x| {
                *bag.entry(x).or_insert(0) += 1;
                bag
            },
            |mut bag, &x| {
                let count = bag.get_mut(&x).unwrap();
                *count -= 1;
                if *count == 0 {
                    bag.remove(&x);
                }
                bag
            },
        )
        .into_iter()
        .enumerate()
        .map(|(l, r)| r - l)
        .max()
        .unwrap_or(0);

        assert_eq!(max_len, 5);
    }

    #[test]
    fn test_empty_target() {
        let target: Vec<i32> = vec![];
        let actual = shakutori_max_right(&target, 0, |_, _| true, |s, _| s, |s, _| s);
        assert_eq!(actual, vec![0]);

        let actual_min = shakutori_min_left(&target, 0, |_, _| true, |s, _| s, |s, _| s);
        assert_eq!(actual_min, vec![0]);
    }

    #[test]
    #[ignore]
    fn test_random() {
        let mut rng = SmallRng::seed_from_u64(42);
        for _ in 0..1000 {
            let n = rng.random_range(1..50);
            let target: Vec<i64> = (0..n).map(|_| rng.random_range(1..20)).collect();
            let k = rng.random_range(1..100);

            let actual = shakutori_max_right(
                &target,
                0,
                |&sum, &x| sum + x <= k,
                |sum, &x| sum + x,
                |sum, &x| sum - x,
            );

            let expected = (0..=n)
                .map(|left| {
                    let mut sum = 0;
                    let mut right = left;
                    while right < n && sum + target[right] <= k {
                        sum += target[right];
                        right += 1;
                    }
                    right
                })
                .collect_vec();

            assert_eq!(actual, expected, "Failed on target={:?}, k={}", target, k);
        }
    }
}
