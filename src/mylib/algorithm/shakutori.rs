use cargo_snippet::snippet;

#[snippet(prefix = "use shakutori::*;")]
#[allow(clippy::module_inception)]
pub mod shakutori {
    /// 各開始位置 `left` に対し、区間 `[left, right)` が条件を満たすような最大の `right` を求める。
    ///
    /// # 引数
    /// * `xs` - 対象の配列
    /// * `init` - 空の区間に対応する初期状態
    /// * `add` - 状態に要素を追加する関数。追加可能なら追加して `true`、不可なら状態を変更せずに `false` を返す。
    /// * `remove` - 状態から要素を削除する関数。
    ///
    /// # 戻り値
    /// `Vec<usize>`: 長さ `n + 1` の Vec。
    /// 各 `left` (0 <= left <= n) に対し、`result[left]` には区間 `[left, right)` が条件を満たす最大の `right` が格納される。
    pub fn shakutori_max_right<T, S, Add, Rem>(
        xs: &[T],
        init: S,
        mut add: Add,
        mut remove: Rem,
    ) -> Vec<usize>
    where
        Add: FnMut(&mut S, &T) -> bool,
        Rem: FnMut(&mut S, &T),
    {
        let n = xs.len();
        let mut state = init;
        let mut left = 0;
        let mut right = 0;
        let mut result = vec![0; n + 1];

        while left < n {
            if right < n && add(&mut state, &xs[right]) {
                // 右端を伸ばせる場合
                right += 1;
            } else if left < right {
                // これ以上右を伸ばせない場合、現在の left に対する答えを確定させ、左を縮める
                result[left] = right;
                remove(&mut state, &xs[left]);
                left += 1;
            } else {
                // left == right かつ add に失敗：要素 xs[left] 単体で NG の場合
                // この要素をスキップするため、left と right を共に進める
                result[left] = right;
                right += 1;
                left += 1;
                // state は既に初期状態 (空) なのでそのまま
            }
        }
        result[n] = n;
        result
    }

    /// 各終了位置 `right` に対し、区間 `[left, right)` が条件を満たすような最小の `left` を求める。
    ///
    /// # 引数
    /// * `xs` - 対象の配列
    /// * `init` - 空の区間に対応する初期状態
    /// * `add` - 状態に要素を追加する関数。追加可能なら追加して `true`、不可なら状態を変更せずに `false` を返す。
    /// * `remove` - 状態から要素を削除する関数。
    ///
    /// # 戻り値
    /// `Vec<usize>`: 長さ `n + 1` の Vec。
    /// 各 `right` (0 <= right <= n) に対し、`result[right]` には区間 `[left, right)` が条件を満たす最小の `left` が格納される。
    pub fn shakutori_min_left<T, S, Add, Rem>(
        xs: &[T],
        init: S,
        mut add: Add,
        mut remove: Rem,
    ) -> Vec<usize>
    where
        Add: FnMut(&mut S, &T) -> bool,
        Rem: FnMut(&mut S, &T),
    {
        let n = xs.len();
        let mut state = init;
        let mut left = 0;
        let mut right = 0;
        let mut result = vec![0; n + 1];

        while right < n {
            if add(&mut state, &xs[right]) {
                // 右端の要素 xs[right] を含めても条件を満たす場合
                right += 1;
                result[right] = left;
            } else if left < right {
                // xs[right] を含めると NG。条件を満たすまで左端を削る
                remove(&mut state, &xs[left]);
                left += 1;
            } else {
                // left == right かつ add に失敗：要素 xs[right] 単体で NG の場合
                // この要素を含めることは不可能。left と right を共に進めることでスキップする。
                right += 1;
                left += 1;
                result[right] = left;
                // state は既に初期状態 (空)
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::shakutori::*;
    use hashbag::HashBag;
    use rand::prelude::*;
    use std::collections::HashSet;

    // --- Basic Tests ---

    #[test]
    fn test_sum_limit_max_right() {
        // 各開始位置に対し、区間和が K 以下となる最大の終了位置をテストする。
        let xs = vec![1, 2, 3, 4, 5];
        let k = 6;

        let result = shakutori_max_right(
            &xs,
            0,
            |sum, &x| {
                if *sum + x <= k {
                    *sum += x;
                    true
                } else {
                    false
                }
            },
            |sum, &x| *sum -= x,
        );

        assert_eq!(result, vec![3, 3, 3, 4, 5, 5]);
    }

    #[test]
    fn test_min_left_sum_limit() {
        // 各終了位置に対し、区間和が K 以下となる最小の開始位置をテストする。
        let xs = vec![1, 2, 3, 4, 5];
        let k = 6;

        let result = shakutori_min_left(
            &xs,
            0,
            |sum, &x| {
                if *sum + x <= k {
                    *sum += x;
                    true
                } else {
                    false
                }
            },
            |sum, &x| *sum -= x,
        );

        assert_eq!(result, vec![0, 0, 0, 0, 3, 4]);
    }

    #[test]
    fn test_kind_count_limit_max_right() {
        // 各開始位置に対し、区間に含まれる要素の種類数が K 以下となる最大の終了位置をテストする。
        let xs = vec![1, 2, 1, 3, 3, 2, 1];
        let k = 2;

        let result = shakutori_max_right(
            &xs,
            HashBag::new(),
            |bag, &x| {
                if bag.contains(&x) > 0 || bag.set_len() < k {
                    bag.insert(x);
                    true
                } else {
                    false
                }
            },
            |bag, &x| {
                bag.remove(&x);
            },
        );

        assert_eq!(result, vec![3, 3, 5, 6, 6, 7, 7, 7]);
    }

    #[test]
    fn test_kind_count_limit_min_left() {
        // 各終了位置に対し、区間に含まれる要素の種類数が K 以下となる最小の開始位置をテストする。
        let xs = vec![1, 2, 1, 3, 3, 2, 1];
        let k = 2;

        let result = shakutori_min_left(
            &xs,
            HashBag::new(),
            |bag, &x| {
                if bag.contains(&x) > 0 || bag.set_len() < k {
                    bag.insert(x);
                    true
                } else {
                    false
                }
            },
            |bag, &x| {
                bag.remove(&x);
            },
        );

        assert_eq!(result, vec![0, 0, 0, 0, 2, 2, 3, 5]);
    }

    #[test]
    fn test_unique_limit() {
        // 各開始位置に対し、区間に重複する要素が含まれない最大の終了位置をテストする。
        let xs = vec![1, 2, 1, 3, 2];

        let result = shakutori_max_right(
            &xs,
            HashSet::new(),
            |set, &x| set.insert(x),
            |set, &x| {
                set.remove(&x);
            },
        );

        assert_eq!(result, vec![2, 4, 5, 5, 5, 5]);
    }

    #[test]
    fn test_min_left_unique() {
        // 各終了位置に対し、区間に重複する要素が含まれない最小の開始位置をテストする。
        let xs = vec![1, 2, 1, 3, 2];

        let result = shakutori_min_left(
            &xs,
            HashSet::new(),
            |set, &x| set.insert(x),
            |set, &x| {
                set.remove(&x);
            },
        );

        assert_eq!(result, vec![0, 0, 0, 1, 1, 2]);
    }

    // --- Impossible Element Tests (Edge Cases) ---

    #[test]
    fn test_sum_limit_impossible_max_right() {
        // 単体で条件を満たさない要素が含まれる場合に、
        // 各開始位置に対する最大の終了位置を正しく計算できるかテストする。
        let xs = vec![1, 10, 2, 3];
        let k = 5;

        let result = shakutori_max_right(
            &xs,
            0,
            |sum, &x| {
                if *sum + x <= k {
                    *sum += x;
                    true
                } else {
                    false
                }
            },
            |sum, &x| *sum -= x,
        );
        assert_eq!(result, vec![1, 1, 4, 4, 4]);
    }

    #[test]
    fn test_sum_limit_impossible_min_left() {
        // 単体で条件を満たさない要素が含まれる場合に、
        // 各終了位置に対する最小の開始位置を正しく計算できるかテストする。
        let xs = vec![1, 10, 2, 3];
        let k = 5;

        let result = shakutori_min_left(
            &xs,
            0,
            |sum, &x| {
                if *sum + x <= k {
                    *sum += x;
                    true
                } else {
                    false
                }
            },
            |sum, &x| *sum -= x,
        );
        assert_eq!(result, vec![0, 0, 2, 2, 2]);
    }

    // --- Random Tests ---

    #[test]
    #[ignore]
    fn test_random_sum_limit() {
        // ランダムな配列と区間和制限 K に対し、愚直解としゃくとり法の結果が一致するかテストする。
        let mut rng = rand::rng();

        for _ in 0..100 {
            let n = rng.random_range(1..=30);
            let k: i64 = rng.random_range(1..=50);
            let xs: Vec<i64> = (0..n)
                .map(|_| {
                    let x = rng.random_range(1..=20);
                    if rng.random_bool(0.1) { k + x } else { x }
                })
                .collect();

            // リファレンス実装: max_right
            // 区間 [l, r) が条件を満たす最大の r を求める
            // 条件: sum(sub) <= k
            let expected_max_right: Vec<usize> = (0..=n)
                .map(|l| {
                    (l..=n)
                        .take_while(|&r| {
                            let sub = &xs[l..r];
                            sub.iter().sum::<i64>() <= k
                        })
                        .last()
                        .unwrap()
                })
                .collect();

            // リファレンス実装: min_left
            // 区間 [l, r) が条件を満たす最小の l を求める
            let expected_min_left: Vec<usize> = (0..=n)
                .map(|r| {
                    (0..=r)
                        .find(|&l| {
                            let sub = &xs[l..r];
                            sub.iter().sum::<i64>() <= k
                        })
                        .unwrap_or(r)
                })
                .collect();

            let actual_max_right = shakutori_max_right(
                &xs,
                0i64,
                |sum, &x| {
                    if *sum + x <= k {
                        *sum += x;
                        true
                    } else {
                        false
                    }
                },
                |sum, &x| *sum -= x,
            );

            let actual_min_left = shakutori_min_left(
                &xs,
                0i64,
                |sum, &x| {
                    if *sum + x <= k {
                        *sum += x;
                        true
                    } else {
                        false
                    }
                },
                |sum, &x| *sum -= x,
            );

            assert_eq!(
                actual_max_right, expected_max_right,
                "max_right failed for xs={:?}, k={}",
                xs, k
            );
            assert_eq!(
                actual_min_left, expected_min_left,
                "min_left failed for xs={:?}, k={}",
                xs, k
            );
        }
    }
}
