#[allow(clippy::module_inception)]
pub mod treap_multiset {
    use std::cmp::Ordering;
    use std::ops::{Bound, Range, RangeBounds};

    use rand::{Rng, SeedableRng, rngs::SmallRng};

    type Link<T> = Option<Box<Node<T>>>;

    #[derive(Clone, Debug)]
    struct Node<T> {
        value: T,
        count: usize,
        total: usize,
        distinct: usize,
        priority: u64,
        left: Link<T>,
        right: Link<T>,
    }

    impl<T> Node<T> {
        fn total(link: &Link<T>) -> usize {
            link.as_ref().map_or(0, |node| node.total)
        }

        fn distinct(link: &Link<T>) -> usize {
            link.as_ref().map_or(0, |node| node.distinct)
        }

        fn recompute(&mut self) {
            self.total = Self::total(&self.left) + self.count + Self::total(&self.right);
            self.distinct = Self::distinct(&self.left) + 1 + Self::distinct(&self.right);
        }

        fn rotate_right(mut self: Box<Self>) -> Box<Self> {
            let mut left = self.left.take().unwrap();
            self.left = left.right.take();
            self.recompute();
            left.right = Some(self);
            left.recompute();
            left
        }

        fn rotate_left(mut self: Box<Self>) -> Box<Self> {
            let mut right = self.right.take().unwrap();
            self.right = right.left.take();
            self.recompute();
            right.left = Some(self);
            right.recompute();
            right
        }

        // 左の全キーが右の全キーより小さいとき、優先度の高い根を残して結合します。
        fn merge(left: Link<T>, right: Link<T>) -> Link<T> {
            match (left, right) {
                (Some(mut left), Some(mut right)) => {
                    if left.priority >= right.priority {
                        left.right = Self::merge(left.right.take(), Some(right));
                        left.recompute();
                        Some(left)
                    } else {
                        right.left = Self::merge(Some(left), right.left.take());
                        right.recompute();
                        Some(right)
                    }
                }
                (left, right) => left.or(right),
            }
        }
    }

    impl<T: Ord> Node<T> {
        fn insert(link: &mut Link<T>, value: T, count: usize, rng: &mut SmallRng) {
            let Some(node) = link else {
                *link = Some(Box::new(Self {
                    value,
                    count,
                    total: count,
                    distinct: 1,
                    priority: rng.random(),
                    left: None,
                    right: None,
                }));
                return;
            };

            match value.cmp(&node.value) {
                Ordering::Less => {
                    Self::insert(&mut node.left, value, count, rng);
                    if node.left.as_ref().unwrap().priority > node.priority {
                        *link = link.take().map(Self::rotate_right);
                        return;
                    }
                }
                Ordering::Greater => {
                    Self::insert(&mut node.right, value, count, rng);
                    if node.right.as_ref().unwrap().priority > node.priority {
                        *link = link.take().map(Self::rotate_left);
                        return;
                    }
                }
                Ordering::Equal => node.count += count,
            }
            node.recompute();
        }

        fn remove_up_to(link: &mut Link<T>, value: &T, count: usize) -> usize {
            let Some(node) = link else {
                return 0;
            };

            let removed = match value.cmp(&node.value) {
                Ordering::Less => Self::remove_up_to(&mut node.left, value, count),
                Ordering::Greater => Self::remove_up_to(&mut node.right, value, count),
                Ordering::Equal => {
                    let removed = count.min(node.count);
                    node.count -= removed;
                    if node.count == 0 {
                        *link = Self::merge(node.left.take(), node.right.take());
                        return removed;
                    }
                    removed
                }
            };
            node.total -= removed;
            node.distinct = Self::distinct(&node.left) + 1 + Self::distinct(&node.right);
            removed
        }
    }

    /// キーの事前登録が不要な、順位を扱える多重集合。
    ///
    /// 異なる値ごとに 1 ノードを持ち、空間計算量は $O(N)$ です。
    /// $N$ は異なる値の数です。期待計算量は Treap のランダムな優先度に対するもので、
    /// 木が偏ると探索や更新に最悪 $O(N)$ かかります。
    #[derive(Clone, Debug)]
    pub struct TreapMultiSet<T> {
        root: Link<T>,
        rng: SmallRng,
    }

    impl<T> Default for TreapMultiSet<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T> TreapMultiSet<T> {
        /// 空の多重集合を作ります。
        ///
        /// # 計算量
        /// $O(1)$。
        pub fn new() -> Self {
            Self {
                root: None,
                rng: SmallRng::from_os_rng(),
            }
        }

        /// 重複を含む要素数を返します。
        ///
        /// # 計算量
        /// $O(1)$。
        pub fn len(&self) -> usize {
            Node::total(&self.root)
        }

        /// 異なる値の数を返します。
        ///
        /// # 計算量
        /// $O(1)$。
        pub fn set_len(&self) -> usize {
            Node::distinct(&self.root)
        }

        /// 空かどうかを返します。
        ///
        /// # 計算量
        /// $O(1)$。
        pub fn is_empty(&self) -> bool {
            self.root.is_none()
        }

        /// すべての要素を削除します。
        ///
        /// # 計算量
        /// $O(N)$。
        pub fn clear(&mut self) {
            self.root = None;
        }

        /// 重複を含めて、0 始まりで小さい方から `n` 番目の要素を返します。
        ///
        /// 範囲外なら `None` を返します。
        ///
        /// # 計算量
        /// 期待 $O(\log N)$。
        pub fn nth_min(&self, mut n: usize) -> Option<&T> {
            let mut node = self.root.as_deref();
            while let Some(current) = node {
                let left_total = Node::total(&current.left);
                if n < left_total {
                    node = current.left.as_deref();
                } else {
                    n -= left_total;
                    if n < current.count {
                        return Some(&current.value);
                    }
                    n -= current.count;
                    node = current.right.as_deref();
                }
            }
            None
        }

        /// 重複を含めて、0 始まりで大きい方から `n` 番目の要素を返します。
        ///
        /// 範囲外なら `None` を返します。
        ///
        /// # 計算量
        /// 期待 $O(\log N)$。
        pub fn nth_max(&self, n: usize) -> Option<&T> {
            self.nth_min(self.len().checked_sub(n)?.checked_sub(1)?)
        }

        /// 最小の要素を返します。空なら `None` を返します。
        ///
        /// # 計算量
        /// 期待 $O(\log N)$。
        pub fn min(&self) -> Option<&T> {
            self.nth_min(0)
        }

        /// 最大の要素を返します。空なら `None` を返します。
        ///
        /// # 計算量
        /// 期待 $O(\log N)$。
        pub fn max(&self) -> Option<&T> {
            self.nth_max(0)
        }
    }

    impl<T: Ord> TreapMultiSet<T> {
        /// 要素を 1 個追加します。
        ///
        /// # 計算量
        /// 期待 $O(\log N)$。
        pub fn insert(&mut self, value: T) {
            self.insert_many(value, 1);
        }

        /// 指定した値を `count` 個追加します。
        ///
        /// # Panics
        /// 追加後の総個数が `usize` に収まらない場合にパニックします。
        ///
        /// # 計算量
        /// 期待 $O(\log N)$。
        pub fn insert_many(&mut self, value: T, count: usize) {
            if count == 0 {
                return;
            }
            self.len()
                .checked_add(count)
                .expect("多重集合の要素数が usize の上限を超えました");
            Node::insert(&mut self.root, value, count, &mut self.rng);
        }

        /// 要素を 1 個削除し、削除できたかを返します。
        ///
        /// # 計算量
        /// 期待 $O(\log N)$。
        pub fn remove1(&mut self, value: &T) -> bool {
            self.remove_up_to(value, 1) != 0
        }

        /// 要素を最大 `count` 個削除し、削除した個数を返します。
        ///
        /// # 計算量
        /// 期待 $O(\log N)$。
        pub fn remove_up_to(&mut self, value: &T, count: usize) -> usize {
            if count == 0 {
                return 0;
            }
            Node::remove_up_to(&mut self.root, value, count)
        }

        /// 指定した値をすべて削除し、削除できたかを返します。
        ///
        /// # 計算量
        /// 期待 $O(\log N)$。
        pub fn remove_all(&mut self, value: &T) -> bool {
            self.remove_up_to(value, usize::MAX) != 0
        }

        /// 指定した値の出現回数を返します。
        ///
        /// # 計算量
        /// 期待 $O(\log N)$。
        pub fn count(&self, value: &T) -> usize {
            let mut node = self.root.as_deref();
            while let Some(current) = node {
                node = match value.cmp(&current.value) {
                    Ordering::Less => current.left.as_deref(),
                    Ordering::Greater => current.right.as_deref(),
                    Ordering::Equal => return current.count,
                };
            }
            0
        }

        /// 指定した値が含まれるかを返します。
        ///
        /// # 計算量
        /// 期待 $O(\log N)$。
        pub fn contains(&self, value: &T) -> bool {
            self.count(value) != 0
        }

        fn rank(&self, value: &T, inclusive: bool) -> usize {
            let mut rank = 0;
            let mut node = self.root.as_deref();
            while let Some(current) = node {
                let left_total = Node::total(&current.left);
                node = match current.value.cmp(value) {
                    Ordering::Less => {
                        rank += left_total + current.count;
                        current.right.as_deref()
                    }
                    Ordering::Equal => {
                        return rank + left_total + if inclusive { current.count } else { 0 };
                    }
                    Ordering::Greater => current.left.as_deref(),
                };
            }
            rank
        }

        fn rank_range<R: RangeBounds<T>>(&self, range: R) -> Range<usize> {
            let start = match range.start_bound() {
                Bound::Unbounded => 0,
                Bound::Included(value) => self.rank(value, false),
                Bound::Excluded(value) => self.rank(value, true),
            };
            let end = match range.end_bound() {
                Bound::Unbounded => self.len(),
                Bound::Included(value) => self.rank(value, true),
                Bound::Excluded(value) => self.rank(value, false),
            };
            start..end.max(start)
        }

        /// 範囲内の最小の要素を返します。
        ///
        /// # 計算量
        /// 期待 $O(\log N)$。
        pub fn min_in_range<R: RangeBounds<T>>(&self, range: R) -> Option<&T> {
            self.nth_min_in_range(0, range)
        }

        /// 範囲内の最大の要素を返します。
        ///
        /// # 計算量
        /// 期待 $O(\log N)$。
        pub fn max_in_range<R: RangeBounds<T>>(&self, range: R) -> Option<&T> {
            self.nth_max_in_range(0, range)
        }

        /// 範囲内で、重複を含めて 0 始まりで小さい方から `n` 番目の要素を返します。
        ///
        /// # 計算量
        /// 期待 $O(\log N)$。
        pub fn nth_min_in_range<R: RangeBounds<T>>(&self, n: usize, range: R) -> Option<&T> {
            let indices = self.rank_range(range);
            if n < indices.len() {
                self.nth_min(indices.start + n)
            } else {
                None
            }
        }

        /// 範囲内で、重複を含めて 0 始まりで大きい方から `n` 番目の要素を返します。
        ///
        /// # 計算量
        /// 期待 $O(\log N)$。
        pub fn nth_max_in_range<R: RangeBounds<T>>(&self, n: usize, range: R) -> Option<&T> {
            let indices = self.rank_range(range);
            if n < indices.len() {
                self.nth_min(indices.end - 1 - n)
            } else {
                None
            }
        }

        /// 範囲内に含まれる要素の個数を重複込みで返します。
        ///
        /// # 計算量
        /// 期待 $O(\log N)$。
        pub fn count_in_range<R: RangeBounds<T>>(&self, range: R) -> usize {
            self.rank_range(range).len()
        }

        /// 範囲内に要素があるかを返します。
        ///
        /// # 計算量
        /// 期待 $O(\log N)$。
        pub fn contains_in_range<R: RangeBounds<T>>(&self, range: R) -> bool {
            self.count_in_range(range) != 0
        }

        /// 最小の要素を 1 個取り出します。
        ///
        /// # 計算量
        /// 期待 $O(\log N)$（値の複製を定数時間とします）。
        pub fn pop_min(&mut self) -> Option<T>
        where
            T: Clone,
        {
            let value = self.min()?.clone();
            self.remove1(&value);
            Some(value)
        }

        /// 最大の要素を 1 個取り出します。
        ///
        /// # 計算量
        /// 期待 $O(\log N)$（値の複製を定数時間とします）。
        pub fn pop_max(&mut self) -> Option<T>
        where
            T: Clone,
        {
            let value = self.max()?.clone();
            self.remove1(&value);
            Some(value)
        }
    }

    impl<T: Ord> FromIterator<T> for TreapMultiSet<T> {
        /// すべての要素を順に追加して構築します。
        ///
        /// # 計算量
        /// 期待 $O(M \log N)$（$M$ は入力の要素数）。
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut set = Self::new();
            for value in iter {
                set.insert(value);
            }
            set
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Bound::{Excluded, Included, Unbounded};

    use rand::{Rng, SeedableRng, rngs::SmallRng};

    use super::treap_multiset::TreapMultiSet;
    use crate::data_structure::btree_multiset::btree_multiset::BTreeMultiSet;

    #[test]
    fn counts_and_removal() {
        let mut set = TreapMultiSet::default();
        assert!(set.is_empty());
        assert_eq!(set.pop_min(), None);
        assert!(!set.remove1(&5));
        set.insert_many(5, 0);
        assert!(set.is_empty());

        set.insert_many(5, 100);
        set.insert_many(5, 2);
        set.insert(2);
        set.insert(9);
        assert_eq!(set.len(), 104);
        assert_eq!(set.set_len(), 3);
        assert_eq!(set.count(&5), 102);
        assert_eq!(set.remove_up_to(&5, 0), 0);
        assert_eq!(set.remove_up_to(&5, 101), 101);
        assert!(set.remove1(&5));
        assert!(!set.contains(&5));
        assert_eq!(set.set_len(), 2);
        assert_eq!(set.pop_max(), Some(9));
        assert_eq!(set.pop_min(), Some(2));
        assert_eq!(set.pop_max(), None);

        set.insert_many(5, usize::MAX);
        assert_eq!(set.nth_min(usize::MAX - 1), Some(&5));
        assert_eq!(set.nth_max(usize::MAX), None);
        assert!(set.remove_all(&5));
        assert!(!set.remove_all(&5));
        set.insert(3);
        set.clear();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
        assert_eq!(set.set_len(), 0);
    }

    #[test]
    #[should_panic(expected = "多重集合の要素数が usize の上限を超えました")]
    fn rejects_count_overflow() {
        let mut set = TreapMultiSet::new();
        set.insert_many(5, usize::MAX);
        set.insert(2);
    }

    #[test]
    fn rank_and_bounds() {
        let set: TreapMultiSet<_> = [9, 5, 2, 5].into_iter().collect();
        assert_eq!(set.min(), Some(&2));
        assert_eq!(set.max(), Some(&9));
        assert_eq!(set.nth_min(1), Some(&5));
        assert_eq!(set.nth_min(2), Some(&5));
        assert_eq!(set.nth_max(2), Some(&5));
        assert_eq!(set.nth_min(4), None);
        assert_eq!(set.count_in_range(2..9), 3);
        assert_eq!(set.count_in_range((Excluded(5), Excluded(5))), 0);
        assert_eq!(set.min_in_range((Excluded(2), Included(9))), Some(&5));
        assert_eq!(set.max_in_range(..9), Some(&5));
        assert_eq!(set.nth_min_in_range(1, 5..), Some(&5));
        assert_eq!(set.nth_max_in_range(2, (Unbounded, Included(5))), Some(&2));
        assert_eq!(set.nth_min_in_range(usize::MAX, ..), None);
        assert_eq!(set.nth_max_in_range(usize::MAX, ..), None);
        assert!(set.contains_in_range(5..=5));
        assert!(!set.contains_in_range(6..9));
    }

    #[test]
    fn random_updates_and_queries_match_btree() {
        let mut rng = SmallRng::seed_from_u64(42);
        for trial in 0..8 {
            let mut treap = TreapMultiSet::new();
            let mut btree = BTreeMultiSet::new();
            for step in 0..300 {
                let key = rng.random_range(-8..=8);
                let count = rng.random_range(0..=4);
                let context = format!("trial={trial}, step={step}, key={key}, count={count}");
                match rng.random_range(0..5) {
                    0 => {
                        treap.insert_many(key, count);
                        btree.insert_many(key, count);
                    }
                    1 => assert_eq!(treap.remove1(&key), btree.remove1(&key), "{context}"),
                    2 => assert_eq!(
                        treap.remove_up_to(&key, count),
                        btree.remove_up_to(&key, count),
                        "{context}"
                    ),
                    3 => assert_eq!(treap.remove_all(&key), btree.remove_all(&key), "{context}"),
                    _ => {
                        treap.insert(key);
                        btree.insert(key);
                    }
                }
                assert_eq!(treap.len(), btree.len(), "{context}");
                assert_eq!(treap.set_len(), btree.set_len(), "{context}");
                assert_eq!(treap.count(&key), btree.count(&key), "{context}");

                for n in 0..=btree.len() {
                    assert_eq!(treap.nth_min(n), btree.nth_min(n), "{context}, n={n}");
                }

                let end = rng.random_range(key..=9);
                let range = match rng.random_range(0..5) {
                    0 => (Included(key), Included(end)),
                    1 => (Included(key), Excluded(end)),
                    2 => (Excluded(key), Included(end)),
                    3 => (Excluded(key), Unbounded),
                    _ => (Unbounded, Excluded(end)),
                };
                assert_eq!(
                    treap.count_in_range(range),
                    btree.range(range).map(|(_, count)| count).sum::<usize>(),
                    "{context}, range={range:?}"
                );
            }
        }
    }
}
