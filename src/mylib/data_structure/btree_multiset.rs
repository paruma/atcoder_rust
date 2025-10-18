use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use btree_multiset::*;")]
pub mod btree_multiset {
    use std::{
        borrow::Borrow,
        collections::{BTreeMap, btree_map::Range},
        ops::RangeBounds,
    };

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct BTreeMultiSet<T> {
        map: BTreeMap<T, usize>,
        length: usize,
    }

    impl<T> Default for BTreeMultiSet<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T> BTreeMultiSet<T> {
        pub const fn new() -> BTreeMultiSet<T> {
            BTreeMultiSet {
                map: BTreeMap::new(),
                length: 0,
            }
        }
        pub fn range<R>(&self, range: R) -> Range<'_, T, usize>
        where
            T: Ord,
            R: RangeBounds<T>,
        {
            self.map.range(range)
        }

        pub fn iter(&self) -> impl Iterator<Item = &T> {
            self.map
                .iter()
                .flat_map(|(e, cnt)| std::iter::repeat(e).take(*cnt))
        }

        pub fn set_iter(&self) -> impl Iterator<Item = (&T, usize)> {
            self.map.iter().map(|(e, cnt)| (e, *cnt))
        }

        pub fn insert(&mut self, value: T)
        where
            T: Ord,
        {
            *self.map.entry(value).or_insert(0) += 1;
            self.length += 1;
        }

        pub fn remove1<Q>(&mut self, value: &Q) -> bool
        where
            T: Borrow<Q> + Ord,
            Q: ?Sized + Ord,
        {
            if let Some(cnt) = self.map.get_mut(value) {
                *cnt -= 1;
                if *cnt == 0 {
                    self.map.remove(value);
                }
                self.length -= 1;
                return true;
            }
            false
        }

        pub fn remove_all<Q>(&mut self, value: &Q) -> bool
        where
            T: Borrow<Q> + Ord,
            Q: ?Sized + Ord,
        {
            if let Some(cnt) = self.map.get(value) {
                self.length -= cnt;
                self.map.remove(value);
                return true;
            }
            false
        }

        pub fn len(&self) -> usize {
            self.length
        }

        pub fn set_len(&self) -> usize {
            self.map.len()
        }

        pub fn is_empty(&self) -> bool {
            self.length == 0
        }

        pub fn count<Q>(&self, value: &Q) -> usize
        where
            T: Borrow<Q> + Ord,
            Q: ?Sized + Ord,
        {
            self.map.get(value).copied().unwrap_or(0)
        }

        pub fn contains<Q>(&self, value: &Q) -> bool
        where
            T: Borrow<Q> + Ord,
            Q: ?Sized + Ord,
        {
            self.map.contains_key(value)
        }
    }
    impl<T: Ord> FromIterator<T> for BTreeMultiSet<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> BTreeMultiSet<T> {
            let mut set = BTreeMultiSet::new();
            for x in iter {
                set.insert(x);
            }
            set
        }
    }
}

#[cfg(test)]
mod tests {
    use super::btree_multiset::BTreeMultiSet;

    #[test]
    fn test_new() {
        let set: BTreeMultiSet<i32> = BTreeMultiSet::new();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_insert() {
        let mut set = BTreeMultiSet::new();
        set.insert(1);
        assert!(!set.is_empty());
        assert_eq!(set.len(), 1);
        assert!(set.contains(&1));
        assert_eq!(set.count(&1), 1);

        set.insert(1);
        assert_eq!(set.len(), 2);
        assert_eq!(set.count(&1), 2);

        set.insert(2);
        assert_eq!(set.len(), 3);
        assert!(set.contains(&2));
        assert_eq!(set.count(&2), 1);
    }

    #[test]
    fn test_remove1() {
        let mut set: BTreeMultiSet<_> = vec![1, 1, 2].into_iter().collect();

        assert!(set.remove1(&1));
        assert_eq!(set.len(), 2);
        assert_eq!(set.count(&1), 1);

        assert!(set.remove1(&1));
        assert_eq!(set.len(), 1);
        assert_eq!(set.count(&1), 0);
        assert!(!set.contains(&1));

        assert!(set.remove1(&2));
        assert!(set.is_empty());

        // Test removing an element not in the set
        assert!(!set.remove1(&3));
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_remove_all() {
        let mut set: BTreeMultiSet<_> = vec![1, 1, 2, 2, 2, 3].into_iter().collect();

        assert!(set.remove_all(&1));
        assert_eq!(set.len(), 4);
        assert_eq!(set.count(&1), 0);
        assert!(!set.contains(&1));

        assert!(set.remove_all(&2));
        assert_eq!(set.len(), 1);
        assert_eq!(set.count(&2), 0);
        assert!(!set.contains(&2));

        assert!(set.remove_all(&3));
        assert_eq!(set.len(), 0);
        assert!(set.is_empty());

        // Test removing an element not in the set
        assert!(!set.remove_all(&4));
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_count() {
        let set: BTreeMultiSet<_> = vec![1, 1, 2].into_iter().collect();

        assert_eq!(set.count(&1), 2);
        assert_eq!(set.count(&2), 1);
        assert_eq!(set.count(&3), 0);
    }

    #[test]
    fn test_set_len() {
        let mut set: BTreeMultiSet<_> = vec![1, 1, 2].into_iter().collect();

        assert_eq!(set.len(), 3);
        assert_eq!(set.set_len(), 2);

        set.remove1(&1);
        assert_eq!(set.len(), 2);
        assert_eq!(set.set_len(), 2);

        set.remove1(&1);
        assert_eq!(set.len(), 1);
        assert_eq!(set.set_len(), 1);
    }

    #[test]
    fn test_contains() {
        let set: BTreeMultiSet<_> = vec![1, 1, 2].into_iter().collect();

        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(!set.contains(&3));
    }

    #[test]
    fn test_range() {
        let set: BTreeMultiSet<_> = vec![1, 2, 3].into_iter().collect();

        let range: Vec<_> = set.range(2..).collect();
        assert_eq!(range, vec![(&2, &1), (&3, &1)]);

        let range: Vec<_> = set.range(..3).collect();
        assert_eq!(range, vec![(&1, &1), (&2, &1)]);

        // Test range with elements out of bounds
        let range: Vec<_> = set.range(10..).collect();
        assert!(range.is_empty());
    }

    #[test]
    fn test_from_iter_and_iter() {
        let vec = vec![1, 1, 2, 3];
        let set: BTreeMultiSet<_> = vec.into_iter().collect();

        assert_eq!(set.len(), 4);
        assert_eq!(set.count(&1), 2);
        assert_eq!(set.count(&2), 1);
        assert_eq!(set.count(&3), 1);
    }

    #[test]
    fn test_iter() {
        let vec = vec![1, 1, 2, 3];
        let set: BTreeMultiSet<_> = vec.into_iter().collect();
        let elements: Vec<_> = set.iter().copied().collect();
        assert_eq!(elements, vec![1, 1, 2, 3]);
    }

    #[test]
    fn test_set_iter() {
        let vec = vec![1, 1, 2, 3];
        let set: BTreeMultiSet<_> = vec.into_iter().collect();
        let elements: Vec<_> = set.set_iter().collect();
        assert_eq!(elements, vec![(&1, 2), (&2, 1), (&3, 1)]);
    }

    #[test]
    fn test_is_empty() {
        let mut set: BTreeMultiSet<i32> = BTreeMultiSet::new();
        assert!(set.is_empty());

        set.insert(1);
        assert!(!set.is_empty());

        set.remove1(&1);
        assert!(set.is_empty());
    }

    #[test]
    fn test_edge_cases() {
        let mut set: BTreeMultiSet<i32> = BTreeMultiSet::new();

        // Test remove on an empty set
        assert!(!set.remove1(&1));

        // Test count on an empty set
        assert_eq!(set.count(&1), 0);

        // Test contains on an empty set
        assert!(!set.contains(&1));

        // Test range on an empty set
        let range: Vec<_> = set.range(1..).collect();
        assert!(range.is_empty());

        // Test insert with negative and zero values
        set.insert(-1);
        set.insert(0);
        assert!(set.contains(&-1));
        assert!(set.contains(&0));
    }
}
