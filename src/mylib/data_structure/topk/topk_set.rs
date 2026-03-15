use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use topk_set::*;")]
pub mod topk_set {
    use std::fmt;

    /// 値が大きい方から最大 K 個を保持するセット（同一値の重複なし）。
    ///
    /// ヒープを使用せず、スタック上の固定長配列で動作する。
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TopKSet<T, const K: usize> {
        // 不変条件:
        // 1. buf[0..len] は T の狭義降順でソートされている（buf[0] > buf[1] > ...）
        // 2. len <= K
        // 3. buf[len..K] はすべて T::default()（未使用スロットは常にデフォルト値）
        buf: [T; K],
        len: usize,
    }

    /// 値が大きい方から最大 2 個を保持するセット。
    pub type Top2Set<T> = TopKSet<T, 2>;
    /// 値が大きい方から最大 3 個を保持するセット。
    pub type Top3Set<T> = TopKSet<T, 3>;
    /// 値が大きい方から最大 4 個を保持するセット。
    pub type Top4Set<T> = TopKSet<T, 4>;
    /// 値が大きい方から最大 5 個を保持するセット。
    pub type Top5Set<T> = TopKSet<T, 5>;

    impl<T, const K: usize> TopKSet<T, K>
    where
        T: Ord + Copy + Default,
    {
        /// 空の TopKSet を作成する。
        ///
        /// 計算量は $O(K)$。
        pub fn new() -> Self {
            Self {
                buf: [T::default(); K],
                len: 0,
            }
        }

        /// 要素を 1 つだけ含む TopKSet を作成する。
        ///
        /// 計算量は $O(K)$。
        pub fn unit(value: T) -> Self {
            Self::new().inserted(value)
        }

        /// 要素を 1 つ追加する。
        ///
        /// 計算量は $O(K)$。
        pub fn insert(&mut self, value: T) {
            // 挿入位置を見つける（狭義降順なので、value 以上の最初の位置）
            let pos = self.buf[..self.len]
                .iter()
                .position(|&x| value >= x)
                .unwrap_or(self.len);

            // 重複排除: 既に同じ値が存在する場合は何もしない
            if pos < self.len && self.buf[pos] == value {
                return;
            }

            if self.len < K {
                // スロットに空きがある: pos に挿入して右シフト
                self.len += 1;
            } else if pos == K {
                // 全要素未満: 何もしない
                return;
            }
            // pos < K の場合（満杯だが挿入位置あり）: 末尾を捨てて右シフト & 挿入

            // 右シフト & 挿入
            // for ループより copy_within (内部で memmove) の方が速い（実測済み）
            self.buf.copy_within(pos..self.len - 1, pos + 1);
            self.buf[pos] = value;
        }

        /// 要素を 1 つ追加した新しい TopKSet を返す。
        ///
        /// 計算量は $O(K)$。
        #[must_use]
        pub fn inserted(self, value: T) -> Self {
            let mut result = self;
            result.insert(value);
            result
        }

        /// other の全要素を追加する。
        ///
        /// 計算量は $O(K^2)$。
        pub fn merge(&mut self, other: Self) {
            for x in other.iter() {
                self.insert(x);
            }
        }

        /// other の全要素を追加した新しい TopKSet を返す。
        ///
        /// 計算量は $O(K^2)$。
        #[must_use]
        pub fn merged(self, other: Self) -> Self {
            let mut result = self;
            result.merge(other);
            result
        }

        /// i 番目に大きい要素を返す（0-indexed）。
        ///
        /// i >= len の場合は None を返す。計算量は $O(1)$。
        pub fn nth(&self, i: usize) -> Option<T> {
            if i < self.len {
                Some(self.buf[i])
            } else {
                None
            }
        }

        /// 保持している最大の要素を返す。
        ///
        /// `nth(0)` と同じ。計算量は $O(1)$。
        pub fn max(&self) -> Option<T> {
            self.nth(0)
        }

        /// 保持している要素数を返す。
        ///
        /// 計算量は $O(1)$。
        pub fn len(&self) -> usize {
            self.len
        }

        /// 空かどうかを返す。
        ///
        /// 計算量は $O(1)$。
        pub fn is_empty(&self) -> bool {
            self.len == 0
        }

        /// 保持している要素のイテレータを返す（T 降順）。
        pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
            self.buf[..self.len].iter().copied()
        }
    }

    impl<T, const K: usize> Default for TopKSet<T, K>
    where
        T: Ord + Copy + Default,
    {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T, const K: usize> FromIterator<T> for TopKSet<T, K>
    where
        T: Ord + Copy + Default,
    {
        /// イテレータの各要素を順に insert した結果と等価。
        ///
        /// 計算量は $O(NK)$（N はイテレータの要素数）。
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut result = Self::new();
            for x in iter {
                result.insert(x);
            }
            result
        }
    }

    impl<T: Copy, const K: usize> IntoIterator for TopKSet<T, K> {
        type Item = T;
        type IntoIter = std::iter::Take<std::array::IntoIter<T, K>>;

        fn into_iter(self) -> Self::IntoIter {
            self.buf.into_iter().take(self.len)
        }
    }

    impl<T: fmt::Debug, const K: usize> fmt::Debug for TopKSet<T, K> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            for (i, x) in self.buf[..self.len].iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", x)?;
            }
            write!(f, "}}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::topk_set::*;
    use rand::{Rng, SeedableRng, rngs::SmallRng};
    use std::collections::BTreeSet;

    // ---- 基本操作 ----

    #[test]
    fn test_new() {
        let t = TopKSet::<i64, 3>::new();
        assert_eq!(t.len(), 0);
        assert!(t.is_empty());
        assert_eq!(t.nth(0), None);
        assert_eq!(t.max(), None);
    }

    #[test]
    fn test_unit() {
        let t = TopKSet::<i64, 3>::unit(5);
        assert_eq!(t.len(), 1);
        assert!(!t.is_empty());
        assert_eq!(t.nth(0), Some(5));
        assert_eq!(t.nth(1), None);
    }

    #[test]
    fn test_insert_basic() {
        let mut t = TopKSet::<i64, 3>::new();
        t.insert(3);
        t.insert(1);
        t.insert(2);
        assert_eq!(t.len(), 3);
        assert_eq!(t.nth(0), Some(3));
        assert_eq!(t.nth(1), Some(2));
        assert_eq!(t.nth(2), Some(1));
        assert_eq!(t.nth(3), None);
    }

    // ---- 重複排除（TopKSet 固有）----

    #[test]
    fn test_duplicate_dedup_len1() {
        // 同じ値を 2 回 insert しても len=1
        let mut t = TopKSet::<i64, 3>::new();
        t.insert(5);
        t.insert(5);
        assert_eq!(t.len(), 1);
        assert_eq!(t.nth(0), Some(5));
        assert_eq!(t.nth(1), None);
    }

    #[test]
    fn test_duplicate_dedup_k_times() {
        // 同じ値を K 回 insert しても len=1
        let mut t = TopKSet::<i64, 3>::new();
        t.insert(5);
        t.insert(5);
        t.insert(5);
        assert_eq!(t.len(), 1);
    }

    #[test]
    fn test_distinct_values_fill_k() {
        // 異なる値を K 個 insert すると len=K
        let mut t = TopKSet::<i64, 3>::new();
        t.insert(1);
        t.insert(2);
        t.insert(3);
        assert_eq!(t.len(), 3);
    }

    // ---- 追い出し ----

    #[test]
    fn test_eviction_k2() {
        // K=2 で 3 種類目の値を挿入すると最小が追い出される
        let t = TopKSet::<i64, 2>::new().inserted(3).inserted(1).inserted(2);
        assert_eq!(t.len(), 2);
        assert_eq!(t.nth(0), Some(3));
        assert_eq!(t.nth(1), Some(2));
    }

    #[test]
    fn test_no_change_when_value_smaller_than_all() {
        // 最小未満の値を挿入しても変化しない
        let t = TopKSet::<i64, 2>::new().inserted(5).inserted(3);
        let t2 = t.inserted(1);
        assert_eq!(t2.nth(0), Some(5));
        assert_eq!(t2.nth(1), Some(3));
        assert_eq!(t2.len(), 2);
    }

    #[test]
    fn test_no_change_when_value_duplicate_of_existing() {
        // 既存の値と同じ値を挿入しても変化しない（重複排除）
        let t = TopKSet::<i64, 2>::new().inserted(5).inserted(3);
        let t2 = t.inserted(3);
        assert_eq!(t2.nth(0), Some(5));
        assert_eq!(t2.nth(1), Some(3));
        assert_eq!(t2.len(), 2);
    }

    #[test]
    fn test_eviction_replace_min() {
        // 最小より大きく最小とは異なる値を挿入すると最小を置き換える
        let t = TopKSet::<i64, 2>::new().inserted(5).inserted(3);
        let t2 = t.inserted(4);
        assert_eq!(t2.nth(0), Some(5));
        assert_eq!(t2.nth(1), Some(4));
        assert_eq!(t2.len(), 2);
    }

    // ---- マージ ----

    #[test]
    fn test_merged() {
        let a: TopKSet<i64, 3> = [5, 3].iter().copied().collect();
        let b: TopKSet<i64, 3> = [4, 2, 1].iter().copied().collect();
        let c = a.merged(b);
        assert_eq!(c.nth(0), Some(5));
        assert_eq!(c.nth(1), Some(4));
        assert_eq!(c.nth(2), Some(3));
        assert_eq!(c.len(), 3);
    }

    #[test]
    fn test_merged_empty() {
        let a = TopKSet::<i64, 2>::new();
        let b = TopKSet::<i64, 2>::new();
        let c = a.merged(b);
        assert!(c.is_empty());
    }

    #[test]
    fn test_merged_duplicate_across() {
        // 両方に同じ値がある場合はマージ後も 1 スロットのみ
        let a: TopKSet<i64, 3> = [5, 3].iter().copied().collect();
        let b: TopKSet<i64, 3> = [5, 3].iter().copied().collect();
        let c = a.merged(b);
        assert_eq!(c.iter().collect::<Vec<_>>(), vec![5, 3]);
        assert_eq!(c.len(), 2);
    }

    // ---- エッジケース ----

    #[test]
    fn test_k1() {
        // K=1 での動作
        let mut t = TopKSet::<i64, 1>::new();
        t.insert(3);
        assert_eq!(t.nth(0), Some(3));
        t.insert(5);
        assert_eq!(t.nth(0), Some(5));
        t.insert(2);
        assert_eq!(t.nth(0), Some(5));
        assert_eq!(t.len(), 1);
    }

    #[test]
    fn test_all_same_value() {
        // 全要素が同一値 → len=1 になる
        let t: TopKSet<i64, 3> = [7, 7, 7, 7].iter().copied().collect();
        assert_eq!(t.len(), 1);
        assert_eq!(t.iter().collect::<Vec<_>>(), vec![7]);
    }

    // ---- イテレータ ----

    #[test]
    fn test_iter() {
        let t: TopKSet<i64, 3> = [5, 3, 1, 4, 2].iter().copied().collect();
        assert_eq!(t.iter().collect::<Vec<_>>(), vec![5, 4, 3]);
    }

    #[test]
    fn test_into_iter() {
        let t: TopKSet<i64, 3> = [5, 3, 1].iter().copied().collect();
        let v: Vec<i64> = t.into_iter().collect();
        assert_eq!(v, vec![5, 3, 1]);
        // Copy なのでループ後も t は使用可
        assert_eq!(t.len(), 3);
    }

    #[test]
    fn test_from_iterator() {
        let t: TopKSet<i64, 3> = vec![1, 5, 3, 2, 4].into_iter().collect();
        assert_eq!(t.iter().collect::<Vec<_>>(), vec![5, 4, 3]);
    }

    #[test]
    fn test_from_iterator_with_duplicates() {
        // 重複を含むイテレータ → 重複は除かれる
        let t: TopKSet<i64, 3> = vec![5, 3, 5, 1, 3, 2, 4].into_iter().collect();
        assert_eq!(t.iter().collect::<Vec<_>>(), vec![5, 4, 3]);
    }

    #[test]
    fn test_default() {
        let t = TopKSet::<i64, 3>::default();
        assert!(t.is_empty());
        assert_eq!(t.len(), 0);
    }

    #[test]
    fn test_debug() {
        let t_empty = TopKSet::<i64, 3>::new();
        assert_eq!(format!("{:?}", t_empty), "{}");

        let t_one = TopKSet::<i64, 3>::unit(3);
        assert_eq!(format!("{:?}", t_one), "{3}");

        let t: TopKSet<i64, 3> = [5, 3, 1].iter().copied().collect();
        assert_eq!(format!("{:?}", t), "{5, 3, 1}");
    }

    // ---- ランダムテスト ----

    /// BTreeSet から降順で上位 K 個を取り出すヘルパー
    fn top_k_from_btree(bt: &BTreeSet<i64>, k: usize) -> Vec<i64> {
        bt.iter().rev().take(k).copied().collect()
    }

    /// 2 つの BTreeSet の和集合から降順で上位 K 個を取り出すヘルパー
    fn top_k_from_btree_union(a: &BTreeSet<i64>, b: &BTreeSet<i64>, k: usize) -> Vec<i64> {
        let merged: BTreeSet<i64> = a.union(b).copied().collect();
        top_k_from_btree(&merged, k)
    }

    #[test]
    #[ignore]
    fn test_random_topk_vs_btree() {
        let mut rng = SmallRng::from_os_rng();

        for k_type in [1, 2, 3, 5] {
            match k_type {
                1 => test_random_topk_inner::<1>(&mut rng),
                2 => test_random_topk_inner::<2>(&mut rng),
                3 => test_random_topk_inner::<3>(&mut rng),
                5 => test_random_topk_inner::<5>(&mut rng),
                _ => unreachable!(),
            }
        }
    }

    fn test_random_topk_inner<const K: usize>(rng: &mut SmallRng) {
        for _ in 0..200 {
            let mut topk = TopKSet::<i64, K>::new();
            let mut bt = BTreeSet::new();

            for _ in 0..200 {
                match rng.random_range(0..5) {
                    0 => {
                        // insert
                        let v = rng.random_range(0..10);
                        topk.insert(v);
                        bt.insert(v);
                        assert_eq!(topk.iter().collect::<Vec<_>>(), top_k_from_btree(&bt, K));
                    }
                    1 => {
                        // inserted
                        let v = rng.random_range(0..10);
                        topk = topk.inserted(v);
                        bt.insert(v);
                        assert_eq!(topk.iter().collect::<Vec<_>>(), top_k_from_btree(&bt, K));
                    }
                    2 => {
                        // マージ: 別の TopKSet を merged で結合し、BTreeSet の和集合と比較
                        let len_b = rng.random_range(0..10);
                        let ys: Vec<i64> = (0..len_b).map(|_| rng.random_range(0..10)).collect();
                        let other: TopKSet<i64, K> = ys.iter().copied().collect();
                        let merged = topk.merged(other);

                        let other_bt: BTreeSet<i64> = ys.iter().copied().collect();
                        let expected = top_k_from_btree_union(&bt, &other_bt, K);
                        assert_eq!(merged.iter().collect::<Vec<_>>(), expected);
                    }
                    3 => {
                        // nth
                        let i = rng.random_range(0..K + 2);
                        let expected = top_k_from_btree(&bt, K);
                        assert_eq!(topk.nth(i), expected.get(i).copied());
                    }
                    4 => {
                        // 引数なしメソッドの検証: max, len, is_empty, iter
                        let expected = top_k_from_btree(&bt, K);
                        assert_eq!(topk.max(), expected.first().copied());
                        assert_eq!(topk.len(), expected.len());
                        assert_eq!(topk.is_empty(), expected.is_empty());
                        assert_eq!(topk.iter().collect::<Vec<_>>(), expected);
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
