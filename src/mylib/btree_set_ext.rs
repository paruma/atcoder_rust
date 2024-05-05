use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use btree_set_ext::*;")]
pub mod btree_set_ext {
    use std::{collections::BTreeSet, ops::RangeBounds};

    use easy_ext::ext;

    #[ext(BTreeSetExt)]
    impl<T> BTreeSet<T>
    where
        T: Ord,
    {
        // 計算量は O(1)？
        pub fn all_min(&self) -> Option<&T> {
            self.iter().next()
        }

        pub fn all_max(&self) -> Option<&T> {
            self.iter().next_back()
        }

        /// range との共通部分の中での最小値を返す
        pub fn range_min<R>(&self, range: R) -> Option<&T>
        where
            R: RangeBounds<T>,
        {
            self.range(range).next()
        }

        /// range との共通部分の中での最大値を返す
        pub fn range_max<R>(&self, range: R) -> Option<&T>
        where
            R: RangeBounds<T>,
        {
            self.range(range).next_back()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::btree_set_ext::*;

    #[test]
    fn test_btree_set_ext() {
        {
            let set = BTreeSet::from([5, 2, 3, 7, 10]);
            assert_eq!(set.all_min(), Some(&2));
            assert_eq!(set.all_max(), Some(&10));
            assert_eq!(set.range_min(..), Some(&2));
            assert_eq!(set.range_max(..), Some(&10));
            assert_eq!(set.range_min(4..=8), Some(&5)); // 4以上の最小値
            assert_eq!(set.range_max(4..=8), Some(&7)); // 8以下の最小値
            assert_eq!(set.range_min(5..=7), Some(&5)); // 5以上の最小値
            assert_eq!(set.range_max(5..=7), Some(&7)); // 7以下の最小値

            assert_eq!(set.range_min(8..=9), None);
            assert_eq!(set.range_max(8..=9), None);
        }
        {
            let set: BTreeSet<i32> = BTreeSet::new();
            assert_eq!(set.all_min(), None);
            assert_eq!(set.all_max(), None);
            assert_eq!(set.range_min(4..=8), None);
            assert_eq!(set.range_max(4..=8), None);
        }
    }
}
