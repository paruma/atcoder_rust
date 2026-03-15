use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use topk_map::*;")]
pub mod topk_map {
    use std::fmt;

    /// キーが大きい方から最大 K 個のエントリを保持するマップ（同一キーの重複なし）。
    ///
    /// ヒープを使用せず、スタック上の固定長配列で動作する。
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TopKMap<Key, Val, const K: usize> {
        // 不変条件:
        // 1. buf[0..len] は Key の狭義降順でソートされている（buf[0].0 > buf[1].0 > ...）
        // 2. len <= K
        // 3. buf[len..K] はすべて (Key::default(), Val::default())（未使用スロットは常にデフォルト値）
        buf: [(Key, Val); K],
        len: usize,
    }

    /// キーが大きい方から最大 2 個のエントリを保持するマップ。
    pub type Top2Map<Key, Val> = TopKMap<Key, Val, 2>;
    /// キーが大きい方から最大 3 個のエントリを保持するマップ。
    pub type Top3Map<Key, Val> = TopKMap<Key, Val, 3>;
    /// キーが大きい方から最大 4 個のエントリを保持するマップ。
    pub type Top4Map<Key, Val> = TopKMap<Key, Val, 4>;
    /// キーが大きい方から最大 5 個のエントリを保持するマップ。
    pub type Top5Map<Key, Val> = TopKMap<Key, Val, 5>;

    impl<Key, Val, const K: usize> TopKMap<Key, Val, K>
    where
        Key: Ord + Copy + Default,
        Val: Copy + Default,
    {
        /// 空の TopKMap を作成する。
        ///
        /// 計算量は $O(K)$。
        pub fn new() -> Self {
            Self {
                buf: [(Key::default(), Val::default()); K],
                len: 0,
            }
        }

        /// エントリを 1 つだけ含む TopKMap を作成する。
        ///
        /// 計算量は $O(K)$。
        pub fn unit(key: Key, value: Val) -> Self {
            Self::new().inserted(key, value)
        }

        /// エントリを 1 つ追加する。同一キーが存在する場合は値を更新する（後勝ち）。
        ///
        /// 計算量は $O(K)$。
        pub fn insert(&mut self, key: Key, value: Val) {
            // 挿入位置を見つける（狭義降順なので、key 以上の最初の位置）
            let pos = self.buf[..self.len]
                .iter()
                .position(|&(k, _)| key >= k)
                .unwrap_or(self.len);

            // 同一キーが存在する場合は値を更新して終了（後勝ち）
            if pos < self.len && self.buf[pos].0 == key {
                self.buf[pos].1 = value;
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
            self.buf[pos] = (key, value);
        }

        /// エントリを 1 つ追加した新しい TopKMap を返す。同一キーが存在する場合は値を更新する（後勝ち）。
        ///
        /// 計算量は $O(K)$。
        #[must_use]
        pub fn inserted(self, key: Key, value: Val) -> Self {
            let mut result = self;
            result.insert(key, value);
            result
        }

        /// other の全エントリを追加する。同一キーが存在する場合は other の値が採用される（後勝ち）。
        ///
        /// 計算量は $O(K^2)$。
        pub fn merge(&mut self, other: Self) {
            for i in 0..other.len {
                self.insert(other.buf[i].0, other.buf[i].1);
            }
        }

        /// other の全エントリを追加した新しい TopKMap を返す。同一キーが存在する場合は other の値が採用される（後勝ち）。
        ///
        /// 計算量は $O(K^2)$。
        #[must_use]
        pub fn merged(self, other: Self) -> Self {
            let mut result = self;
            result.merge(other);
            result
        }

        /// i 番目に大きいキーのエントリ `(Key, Val)` を返す（0-indexed）。
        ///
        /// i >= len の場合は None を返す。計算量は $O(1)$。
        pub fn nth(&self, i: usize) -> Option<(Key, Val)> {
            if i < self.len {
                Some(self.buf[i])
            } else {
                None
            }
        }

        /// i 番目に大きいキーを返す（0-indexed）。
        ///
        /// i >= len の場合は None を返す。計算量は $O(1)$。
        pub fn nth_key(&self, i: usize) -> Option<Key> {
            self.nth(i).map(|(k, _)| k)
        }

        /// i 番目に大きいキーに対応する値を返す（0-indexed）。
        ///
        /// i >= len の場合は None を返す。計算量は $O(1)$。
        pub fn nth_value(&self, i: usize) -> Option<Val> {
            self.nth(i).map(|(_, v)| v)
        }

        /// 最大キーのエントリを返す。
        ///
        /// `nth(0)` と同じ。計算量は $O(1)$。
        pub fn max(&self) -> Option<(Key, Val)> {
            self.nth(0)
        }

        /// 最大キーを返す。
        ///
        /// `nth_key(0)` と同じ。計算量は $O(1)$。
        pub fn max_key(&self) -> Option<Key> {
            self.nth_key(0)
        }

        /// 最大キーに対応する値を返す。
        ///
        /// `nth_value(0)` と同じ。計算量は $O(1)$。
        pub fn max_value(&self) -> Option<Val> {
            self.nth_value(0)
        }

        /// 保持しているエントリ数を返す。
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

        /// 保持しているエントリのイテレータを返す（Key 降順）。
        pub fn iter(&self) -> impl Iterator<Item = (Key, Val)> + '_ {
            self.buf[..self.len].iter().copied()
        }

        /// 保持しているキーのイテレータを返す（Key 降順）。
        pub fn keys(&self) -> impl Iterator<Item = Key> + '_ {
            self.iter().map(|(k, _)| k)
        }

        /// 保持している値のイテレータを返す（Key 降順）。
        pub fn values(&self) -> impl Iterator<Item = Val> + '_ {
            self.iter().map(|(_, v)| v)
        }
    }

    impl<Key, Val, const K: usize> Default for TopKMap<Key, Val, K>
    where
        Key: Ord + Copy + Default,
        Val: Copy + Default,
    {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<Key, Val, const K: usize> FromIterator<(Key, Val)> for TopKMap<Key, Val, K>
    where
        Key: Ord + Copy + Default,
        Val: Copy + Default,
    {
        /// イテレータの各エントリを順に insert した結果と等価。同一キーは後勝ちで更新される。
        ///
        /// 計算量は $O(NK)$（N はイテレータの要素数）。
        fn from_iter<I: IntoIterator<Item = (Key, Val)>>(iter: I) -> Self {
            let mut result = Self::new();
            for (k, v) in iter {
                result.insert(k, v);
            }
            result
        }
    }

    impl<Key: Copy, Val: Copy, const K: usize> IntoIterator for TopKMap<Key, Val, K> {
        type Item = (Key, Val);
        type IntoIter = std::iter::Take<std::array::IntoIter<(Key, Val), K>>;

        fn into_iter(self) -> Self::IntoIter {
            self.buf.into_iter().take(self.len)
        }
    }

    impl<Key: fmt::Debug, Val: fmt::Debug, const K: usize> fmt::Debug for TopKMap<Key, Val, K> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            for (i, (k, v)) in self.buf[..self.len].iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}: {:?}", k, v)?;
            }
            write!(f, "}}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::topk_map::*;
    use rand::{Rng, SeedableRng, rngs::SmallRng};
    use std::collections::BTreeMap;

    // ---- 基本操作 ----

    #[test]
    fn test_new() {
        let t = TopKMap::<i64, i64, 3>::new();
        assert_eq!(t.len(), 0);
        assert!(t.is_empty());
        assert_eq!(t.nth(0), None);
        assert_eq!(t.max(), None);
    }

    #[test]
    fn test_unit() {
        let t = TopKMap::<i64, i64, 3>::unit(5, 10);
        assert_eq!(t.len(), 1);
        assert!(!t.is_empty());
        assert_eq!(t.nth(0), Some((5, 10)));
        assert_eq!(t.nth(1), None);
    }

    #[test]
    fn test_insert_basic() {
        let mut t = TopKMap::<i64, i64, 3>::new();
        t.insert(3, 30);
        t.insert(1, 10);
        t.insert(2, 20);
        assert_eq!(t.len(), 3);
        assert_eq!(t.nth(0), Some((3, 30)));
        assert_eq!(t.nth(1), Some((2, 20)));
        assert_eq!(t.nth(2), Some((1, 10)));
        assert_eq!(t.nth(3), None);
    }

    // ---- 重複キー更新（TopKMap 固有）----

    #[test]
    fn test_duplicate_key_update_len1() {
        // 同じキーを 2 回 insert すると len=1 で値が後勝ちになる
        let mut t = TopKMap::<i64, i64, 3>::new();
        t.insert(5, 10);
        t.insert(5, 20);
        assert_eq!(t.len(), 1);
        assert_eq!(t.nth(0), Some((5, 20)));
        assert_eq!(t.nth(1), None);
    }

    #[test]
    fn test_duplicate_key_update_k_times() {
        // 同じキーを K 回 insert しても len=1
        let mut t = TopKMap::<i64, i64, 3>::new();
        t.insert(5, 1);
        t.insert(5, 2);
        t.insert(5, 3);
        assert_eq!(t.len(), 1);
        assert_eq!(t.nth(0), Some((5, 3)));
    }

    #[test]
    fn test_distinct_keys_fill_k() {
        // 異なるキーを K 個 insert すると len=K
        let mut t = TopKMap::<i64, i64, 3>::new();
        t.insert(1, 10);
        t.insert(2, 20);
        t.insert(3, 30);
        assert_eq!(t.len(), 3);
    }

    #[test]
    fn test_update_existing_min_key() {
        // 追い出し対象キーと同じキーを insert すると値のみ更新される（追い出しなし）
        let mut t = TopKMap::<i64, i64, 2>::new();
        t.insert(5, 50);
        t.insert(3, 30);
        // 最小キー 3 を同じキーで更新
        t.insert(3, 99);
        assert_eq!(t.len(), 2);
        assert_eq!(t.nth(0), Some((5, 50)));
        assert_eq!(t.nth(1), Some((3, 99)));
    }

    // ---- 追い出し ----

    #[test]
    fn test_eviction_k2() {
        // K=2 で 3 種類目のキーを挿入すると最小キーが追い出される
        let t = TopKMap::<i64, i64, 2>::new()
            .inserted(3, 30)
            .inserted(1, 10)
            .inserted(2, 20);
        assert_eq!(t.len(), 2);
        assert_eq!(t.nth(0), Some((3, 30)));
        assert_eq!(t.nth(1), Some((2, 20)));
    }

    #[test]
    fn test_no_change_when_key_smaller_than_all() {
        // 最小キー未満のキーを挿入しても変化しない
        let t = TopKMap::<i64, i64, 2>::new()
            .inserted(5, 50)
            .inserted(3, 30);
        let t2 = t.inserted(1, 10);
        assert_eq!(t2.nth(0), Some((5, 50)));
        assert_eq!(t2.nth(1), Some((3, 30)));
        assert_eq!(t2.len(), 2);
    }

    #[test]
    fn test_update_existing_key_no_eviction() {
        // 既存の最小キーと同じキーを挿入すると値のみ更新される（追い出しなし）
        let t = TopKMap::<i64, i64, 2>::new()
            .inserted(5, 50)
            .inserted(3, 30);
        let t2 = t.inserted(3, 99);
        assert_eq!(t2.nth(0), Some((5, 50)));
        assert_eq!(t2.nth(1), Some((3, 99)));
        assert_eq!(t2.len(), 2);
    }

    // ---- マージ（後勝ち）----

    #[test]
    fn test_merged() {
        let a: TopKMap<i64, i64, 3> = [(5, 50), (3, 30)].into_iter().collect();
        let b: TopKMap<i64, i64, 3> = [(4, 40), (2, 20), (1, 10)].into_iter().collect();
        let c = a.merged(b);
        assert_eq!(c.nth(0), Some((5, 50)));
        assert_eq!(c.nth(1), Some((4, 40)));
        assert_eq!(c.nth(2), Some((3, 30)));
        assert_eq!(c.len(), 3);
    }

    #[test]
    fn test_merged_empty() {
        let a = TopKMap::<i64, i64, 2>::new();
        let b = TopKMap::<i64, i64, 2>::new();
        let c = a.merged(b);
        assert!(c.is_empty());
    }

    #[test]
    fn test_merged_other_wins_on_duplicate_key() {
        // other に存在するキーが self にも存在する場合、other の値が採用される
        let a: TopKMap<i64, i64, 3> = [(5, 50), (3, 30)].into_iter().collect();
        let b: TopKMap<i64, i64, 3> = [(5, 500), (3, 300)].into_iter().collect();
        let c = a.merged(b);
        assert_eq!(c.iter().collect::<Vec<_>>(), vec![(5, 500), (3, 300)]);
        assert_eq!(c.len(), 2);
    }

    #[test]
    fn test_merged_eviction() {
        // K 個を超える場合に正しく追い出される
        let a: TopKMap<i64, i64, 2> = [(5, 50), (3, 30)].into_iter().collect();
        let b: TopKMap<i64, i64, 2> = [(4, 40), (2, 20)].into_iter().collect();
        let c = a.merged(b);
        assert_eq!(c.nth(0), Some((5, 50)));
        assert_eq!(c.nth(1), Some((4, 40)));
        assert_eq!(c.len(), 2);
    }

    #[test]
    fn test_merged_associativity_same_key() {
        // self.merged(a).merged(b) と self.merged(a.merged(b)) で全エントリが一致する
        let base: TopKMap<i64, i64, 3> = [(5, 1)].into_iter().collect();
        let a: TopKMap<i64, i64, 3> = [(5, 2), (3, 30)].into_iter().collect();
        let b: TopKMap<i64, i64, 3> = [(5, 3), (4, 40)].into_iter().collect();
        let left = base.merged(a).merged(b);
        let right = base.merged(a.merged(b));
        assert_eq!(
            left.iter().collect::<Vec<_>>(),
            right.iter().collect::<Vec<_>>()
        );
        // 同一キー 5 の値は b が最後に適用されるので 3、key=4 と key=3 も保持される
        assert_eq!(
            left.iter().collect::<Vec<_>>(),
            vec![(5, 3), (4, 40), (3, 30)]
        );
    }

    // ---- 参照メソッド（TopKMap 固有）----

    #[test]
    fn test_nth_key_nth_value_consistency() {
        let t: TopKMap<i64, i64, 3> = [(5, 50), (3, 30), (1, 10)].into_iter().collect();
        for i in 0..4 {
            assert_eq!(t.nth_key(i), t.nth(i).map(|(k, _)| k));
            assert_eq!(t.nth_value(i), t.nth(i).map(|(_, v)| v));
        }
    }

    #[test]
    fn test_max_methods_consistency() {
        let t: TopKMap<i64, i64, 3> = [(5, 50), (3, 30), (1, 10)].into_iter().collect();
        assert_eq!(t.max(), t.nth(0));
        assert_eq!(t.max_key(), t.nth_key(0));
        assert_eq!(t.max_value(), t.nth_value(0));
    }

    #[test]
    fn test_max_empty() {
        let t = TopKMap::<i64, i64, 3>::new();
        assert_eq!(t.max(), None);
        assert_eq!(t.max_key(), None);
        assert_eq!(t.max_value(), None);
    }

    // ---- エッジケース ----

    #[test]
    fn test_k1() {
        // K=1 での動作
        let mut t = TopKMap::<i64, i64, 1>::new();
        t.insert(3, 30);
        assert_eq!(t.nth(0), Some((3, 30)));
        t.insert(5, 50);
        assert_eq!(t.nth(0), Some((5, 50)));
        t.insert(2, 20);
        assert_eq!(t.nth(0), Some((5, 50)));
        assert_eq!(t.len(), 1);
    }

    #[test]
    fn test_all_same_key() {
        // 全要素が同一キー → len=1 になり最後の値が保持される
        let t: TopKMap<i64, i64, 3> = [(7, 1), (7, 2), (7, 3), (7, 4)].into_iter().collect();
        assert_eq!(t.len(), 1);
        assert_eq!(t.iter().collect::<Vec<_>>(), vec![(7, 4)]);
    }

    // ---- イテレータ ----

    #[test]
    fn test_iter() {
        let t: TopKMap<i64, i64, 3> = [(5, 50), (3, 30), (1, 10), (4, 40), (2, 20)]
            .into_iter()
            .collect();
        assert_eq!(
            t.iter().collect::<Vec<_>>(),
            vec![(5, 50), (4, 40), (3, 30)]
        );
    }

    #[test]
    fn test_into_iter() {
        let t: TopKMap<i64, i64, 3> = [(5, 50), (3, 30), (1, 10)].into_iter().collect();
        let v: Vec<(i64, i64)> = t.into_iter().collect();
        assert_eq!(v, vec![(5, 50), (3, 30), (1, 10)]);
        // Copy なのでループ後も t は使用可
        assert_eq!(t.len(), 3);
    }

    #[test]
    fn test_from_iterator_with_duplicate_keys() {
        // 重複キーを含むイテレータ → 後勝ちで更新される
        let t: TopKMap<i64, &str, 3> =
            vec![(50, "alice"), (30, "bob"), (50, "charlie"), (40, "dave")]
                .into_iter()
                .collect();
        assert_eq!(t.nth(0), Some((50, "charlie")));
        assert_eq!(t.nth(1), Some((40, "dave")));
        assert_eq!(t.nth(2), Some((30, "bob")));
    }

    #[test]
    fn test_keys_values() {
        let t: TopKMap<i64, i64, 3> = [(5, 50), (3, 30), (1, 10)].into_iter().collect();
        assert_eq!(t.keys().collect::<Vec<_>>(), vec![5, 3, 1]);
        assert_eq!(t.values().collect::<Vec<_>>(), vec![50, 30, 10]);
    }

    #[test]
    fn test_default() {
        let t = TopKMap::<i64, i64, 3>::default();
        assert!(t.is_empty());
        assert_eq!(t.len(), 0);
    }

    #[test]
    fn test_debug() {
        let t_empty = TopKMap::<i64, i64, 3>::new();
        assert_eq!(format!("{:?}", t_empty), "{}");

        let t_one = TopKMap::<i64, i64, 3>::unit(3, 30);
        assert_eq!(format!("{:?}", t_one), "{3: 30}");

        let t: TopKMap<i64, i64, 3> = [(5, 50), (3, 30), (1, 10)].into_iter().collect();
        assert_eq!(format!("{:?}", t), "{5: 50, 3: 30, 1: 10}");
    }

    // ---- ランダムテスト ----

    /// BTreeMap から降順で上位 K 個のエントリを取り出すヘルパー
    fn top_k_from_btree<V: Copy>(bt: &BTreeMap<i64, V>, k: usize) -> Vec<(i64, V)> {
        bt.iter().rev().take(k).map(|(&k, &v)| (k, v)).collect()
    }

    /// 2 つの BTreeMap をマージ（後勝ち）して上位 K 個を取り出すヘルパー
    fn top_k_from_btree_merged<V: Copy>(
        a: &BTreeMap<i64, V>,
        b: &BTreeMap<i64, V>,
        k: usize,
    ) -> Vec<(i64, V)> {
        let mut merged = a.clone();
        merged.extend(b.iter().map(|(&k, &v)| (k, v)));
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
            let mut topk = TopKMap::<i64, i64, K>::new();
            let mut bt = BTreeMap::<i64, i64>::new();

            for _ in 0..200 {
                match rng.random_range(0..5) {
                    0 => {
                        // insert
                        let k = rng.random_range(0..10);
                        let v = rng.random_range(0..5);
                        topk.insert(k, v);
                        bt.insert(k, v);
                        assert_eq!(topk.iter().collect::<Vec<_>>(), top_k_from_btree(&bt, K));
                    }
                    1 => {
                        // inserted
                        let k = rng.random_range(0..10);
                        let v = rng.random_range(0..5);
                        topk = topk.inserted(k, v);
                        bt.insert(k, v);
                        assert_eq!(topk.iter().collect::<Vec<_>>(), top_k_from_btree(&bt, K));
                    }
                    2 => {
                        // マージ: 別の TopKMap を merged で結合し、BTreeMap のマージと比較
                        let len_b = rng.random_range(0..10);
                        let ys: Vec<(i64, i64)> = (0..len_b)
                            .map(|_| (rng.random_range(0..10), rng.random_range(0..5)))
                            .collect();
                        let other: TopKMap<i64, i64, K> = ys.iter().copied().collect();
                        let merged = topk.merged(other);

                        let other_bt: BTreeMap<i64, i64> = ys.iter().copied().collect();
                        let expected = top_k_from_btree_merged(&bt, &other_bt, K);
                        assert_eq!(merged.iter().collect::<Vec<_>>(), expected);
                    }
                    3 => {
                        // nth / nth_key / nth_value
                        let i = rng.random_range(0..K + 2);
                        let expected = top_k_from_btree(&bt, K);
                        assert_eq!(topk.nth(i), expected.get(i).copied());
                        assert_eq!(topk.nth_key(i), expected.get(i).map(|&(k, _)| k));
                        assert_eq!(topk.nth_value(i), expected.get(i).map(|&(_, v)| v));
                    }
                    4 => {
                        // 引数なしメソッドの検証: max, max_key, max_value, len, is_empty, iter
                        let expected = top_k_from_btree(&bt, K);
                        assert_eq!(topk.max(), expected.first().copied());
                        assert_eq!(topk.max_key(), expected.first().map(|&(k, _)| k));
                        assert_eq!(topk.max_value(), expected.first().map(|&(_, v)| v));
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
