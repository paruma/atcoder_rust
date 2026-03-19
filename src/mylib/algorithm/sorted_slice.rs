use cargo_snippet::snippet;

#[snippet(prefix = "use sorted_slice::*;")]
#[allow(clippy::module_inception)]
pub mod sorted_slice {
    use std::ops::{Bound::*, Range, RangeBounds};

    /// ソート済みスライスに対する区間クエリを提供するトレイト。
    pub trait SortedSliceExt<T: Ord> {
        fn range_indices<R: RangeBounds<T>>(&self, range: R) -> Range<usize>;
        fn range_count<R: RangeBounds<T>>(&self, range: R) -> usize;
        fn range_min_index<R: RangeBounds<T>>(&self, range: R) -> Option<usize>;
        fn range_max_index<R: RangeBounds<T>>(&self, range: R) -> Option<usize>;
    }

    impl<T: Ord> SortedSliceExt<T> for [T] {
        /// `range` に含まれる要素のインデックス範囲 `[begin, end)` を返す。
        ///
        /// # 前提条件
        /// * `self`: 広義単調増加（ソート済み）であること
        ///
        /// # 計算量
        /// $O(\log N)$（$N$ は `self` の長さ）
        fn range_indices<R: RangeBounds<T>>(&self, range: R) -> Range<usize> {
            let begin = match range.start_bound() {
                Included(lo) => self.partition_point(|x| x < lo),
                Excluded(lo) => self.partition_point(|x| x <= lo),
                Unbounded => 0,
            };
            let end = match range.end_bound() {
                Included(hi) => self.partition_point(|x| x <= hi),
                Excluded(hi) => self.partition_point(|x| x < hi),
                Unbounded => self.len(),
            };
            // range の始点と終点が逆転している（例: 4..=2）とき、end < begin になりうるため begin..begin (空 Range) に丸める
            begin..end.max(begin)
        }

        /// `range` に含まれる要素の個数を返す。
        ///
        /// # 前提条件
        /// * `self`: 広義単調増加（ソート済み）であること
        ///
        /// # 計算量
        /// $O(\log N)$（$N$ は `self` の長さ）
        fn range_count<R: RangeBounds<T>>(&self, range: R) -> usize {
            self.range_indices(range).len()
        }

        /// `self[i] ∈ range` を満たす最小の `i` を返す。存在しない場合は `None`。
        ///
        /// # 前提条件
        /// * `self`: 広義単調増加（ソート済み）であること
        ///
        /// # 計算量
        /// $O(\log N)$（$N$ は `self` の長さ）
        fn range_min_index<R: RangeBounds<T>>(&self, range: R) -> Option<usize> {
            let r = self.range_indices(range);
            if r.is_empty() { None } else { Some(r.start) }
        }

        /// `self[i] ∈ range` を満たす最大の `i` を返す。存在しない場合は `None`。
        ///
        /// # 前提条件
        /// * `self`: 広義単調増加（ソート済み）であること
        ///
        /// # 計算量
        /// $O(\log N)$（$N$ は `self` の長さ）
        fn range_max_index<R: RangeBounds<T>>(&self, range: R) -> Option<usize> {
            let r = self.range_indices(range);
            if r.is_empty() { None } else { Some(r.end - 1) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sorted_slice::*;
    use rand::{Rng, SeedableRng};

    #[test]
    fn test_basic() {
        let xs: &[i64] = &[1, 2, 2, 3, 5, 7];

        // 通常ケース: 範囲内に複数の要素がある
        assert_eq!(xs.range_count(2..=4), 3);
        assert_eq!(xs.range_min_index(2..=4), Some(1));
        assert_eq!(xs.range_max_index(2..=4), Some(3));
        assert_eq!(xs.range_indices(2..=4), 1..4);

        // 半開区間
        assert_eq!(xs.range_count(2..5), 3);
        assert_eq!(xs.range_min_index(2..5), Some(1));
        assert_eq!(xs.range_max_index(2..5), Some(3));
        assert_eq!(xs.range_indices(2..5), 1..4);

        // 上界のみ
        assert_eq!(xs.range_count(..3), 3); // 1, 2, 2
        assert_eq!(xs.range_min_index(..3), Some(0));
        assert_eq!(xs.range_max_index(..3), Some(2));

        // 下界のみ
        assert_eq!(xs.range_count(3..), 3); // 3, 5, 7
        assert_eq!(xs.range_min_index(3..), Some(3));
        assert_eq!(xs.range_max_index(3..), Some(5));

        // 全体
        assert_eq!(xs.range_count(..), 6);
        assert_eq!(xs.range_min_index(..), Some(0));
        assert_eq!(xs.range_max_index(..), Some(5));
        assert_eq!(xs.range_indices(..), 0..6);

        // 配列中に存在しない値を含む区間
        assert_eq!(xs.range_count(4..6), 1); // 5のみ
        assert_eq!(xs.range_min_index(4..6), Some(4));
        assert_eq!(xs.range_max_index(4..6), Some(4));

        // 配列中の要素でちょうど境界が一致するケース
        assert_eq!(xs.range_count(1..=7), 6); // 全要素
        assert_eq!(xs.range_min_index(1..=7), Some(0));
        assert_eq!(xs.range_max_index(1..=7), Some(5));

        // 要素が1個のみマッチ
        assert_eq!(xs.range_count(3..=3), 1);
        assert_eq!(xs.range_min_index(3..=3), Some(3));
        assert_eq!(xs.range_max_index(3..=3), Some(3));

        // マッチなし（配列範囲外）
        assert_eq!(xs.range_count(8..10), 0);
        assert_eq!(xs.range_min_index(8..10), None);
        assert_eq!(xs.range_max_index(8..10), None);
        assert!(xs.range_indices(8..10).is_empty());

        // マッチなし（配列内のギャップ）
        assert_eq!(xs.range_count(4..5), 0); // 4は存在しない、5は5..に入らない
        assert_eq!(xs.range_min_index(4..5), None);
        assert_eq!(xs.range_max_index(4..5), None);
        assert!(xs.range_indices(4..5).is_empty());

        // 空配列
        let empty: &[i64] = &[];
        assert_eq!(empty.range_count(..), 0);
        assert_eq!(empty.range_min_index(..), None);
        assert_eq!(empty.range_max_index(..), None);
        assert!(empty.range_indices(..).is_empty());

        assert_eq!(empty.range_count(1..=5), 0);
        assert_eq!(empty.range_min_index(1..=5), None);
        assert_eq!(empty.range_max_index(1..=5), None);

        // 全要素を含まない区間（上界が全要素より小さい）
        assert_eq!(xs.range_count(..0), 0);
        assert_eq!(xs.range_min_index(..0), None);
        assert_eq!(xs.range_max_index(..0), None);

        // 除外上界が最小要素と一致
        assert_eq!(xs.range_count(..1), 0); // 1 は excluded
        assert_eq!(xs.range_min_index(..1), None);

        // 含む上界が最小要素と一致
        assert_eq!(xs.range_count(..=1), 1);
        assert_eq!(xs.range_min_index(..=1), Some(0));
        assert_eq!(xs.range_max_index(..=1), Some(0));

        // 除外下界
        assert_eq!(
            xs.range_count((
                std::ops::Bound::Excluded(2i64),
                std::ops::Bound::Included(5i64)
            )),
            2
        ); // 3, 5
        assert_eq!(
            xs.range_min_index((
                std::ops::Bound::Excluded(2i64),
                std::ops::Bound::Included(5i64)
            )),
            Some(3)
        );
        assert_eq!(
            xs.range_max_index((
                std::ops::Bound::Excluded(2i64),
                std::ops::Bound::Included(5i64)
            )),
            Some(4)
        );
    }

    #[ignore]
    #[test]
    fn test_random_sorted_slice() {
        use std::ops::Bound::{self, Excluded, Included, Unbounded};
        use std::ops::RangeBounds;

        // Bound のバリアント（Included/Excluded/Unbounded）と値をランダムに生成する
        fn random_bound(rng: &mut rand::rngs::SmallRng) -> Bound<i64> {
            match rng.random_range(0..3u32) {
                0 => Included(rng.random_range(0..10)),
                1 => Excluded(rng.random_range(0..10)),
                _ => Unbounded,
            }
        }

        let mut rng = rand::rngs::SmallRng::seed_from_u64(42);

        for _ in 0..5000 {
            let n = rng.random_range(0..=20);
            let mut xs: Vec<i64> = (0..n).map(|_| rng.random_range(0..10)).collect();
            xs.sort();

            let range = (random_bound(&mut rng), random_bound(&mut rng));

            // ナイーブ実装
            let naive_indices: Vec<usize> = (0..n).filter(|&i| range.contains(&xs[i])).collect();
            let naive_count = naive_indices.len();
            let naive_min = naive_indices.first().copied();
            let naive_max = naive_indices.last().copied();

            assert_eq!(
                xs.range_count(range),
                naive_count,
                "range_count mismatch: xs={xs:?}, range={range:?}"
            );
            assert_eq!(
                xs.range_min_index(range),
                naive_min,
                "range_min_index mismatch: xs={xs:?}, range={range:?}"
            );
            assert_eq!(
                xs.range_max_index(range),
                naive_max,
                "range_max_index mismatch: xs={xs:?}, range={range:?}"
            );

            let indices = xs.range_indices(range);
            let expected_indices = if naive_indices.is_empty() {
                // 空の Range（is_empty == true）
                let b = indices.start;
                b..b
            } else {
                *naive_indices.first().unwrap()..*naive_indices.last().unwrap() + 1
            };
            assert_eq!(
                indices, expected_indices,
                "range_indices mismatch: xs={xs:?}, range={range:?}"
            );
        }
    }
}
