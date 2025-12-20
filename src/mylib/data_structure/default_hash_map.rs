use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use default_hash_map::*;")]
pub mod default_hash_map {
    use std::hash::Hash;

    use std::{
        borrow::Borrow,
        collections::{
            HashMap,
            hash_map::{Iter, IterMut, Keys, Values, ValuesMut},
        },
    };

    #[derive(Clone, Debug)]
    pub struct DefaultHashMap<K, V> {
        raw: HashMap<K, V>,
        default: V,
    }

    impl<K, V> DefaultHashMap<K, V> {
        pub fn new(default: V) -> DefaultHashMap<K, V> {
            DefaultHashMap {
                raw: HashMap::new(),
                default,
            }
        }
        pub fn from_hash_map(hash_map: HashMap<K, V>, default: V) -> DefaultHashMap<K, V> {
            DefaultHashMap {
                raw: hash_map,
                default,
            }
        }
        pub fn raw(&mut self) -> &mut HashMap<K, V> {
            &mut self.raw
        }

        pub fn keys(&self) -> Keys<'_, K, V> {
            self.raw.keys()
        }

        pub fn values(&self) -> Values<'_, K, V> {
            self.raw.values()
        }

        pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
            self.raw.values_mut()
        }

        pub fn iter(&self) -> Iter<'_, K, V> {
            self.raw.iter()
        }

        pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
            self.raw.iter_mut()
        }

        pub fn len(&mut self) -> usize {
            self.raw.len()
        }

        pub fn is_empty(&mut self) -> bool {
            self.raw.is_empty()
        }
    }
    impl<K, V> DefaultHashMap<K, V>
    where
        K: Eq + Hash,
    {
        pub fn get<Q>(&self, k: &Q) -> &V
        where
            K: Borrow<Q>,
            Q: ?Sized + Hash + Eq,
        {
            self.raw.get(k).unwrap_or(&self.default)
        }

        // Clone がない場合も使えるようにするといい気がする。
        pub fn get_mut(&mut self, k: K) -> &mut V
        where
            V: Clone,
        {
            self.raw.entry(k).or_insert(self.default.clone())
        }

        pub fn insert(&mut self, k: K, v: V) -> Option<V> {
            self.raw.insert(k, v)
        }

        pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
        where
            K: Borrow<Q>,
            Q: ?Sized + Hash + Eq,
        {
            self.raw.remove(k)
        }
    }

    impl<K, V> PartialEq for DefaultHashMap<K, V>
    where
        K: Eq + Hash,
        V: PartialEq,
    {
        fn eq(&self, other: &DefaultHashMap<K, V>) -> bool {
            self.raw == other.raw && self.default == other.default
        }
    }

    impl<K, V> Eq for DefaultHashMap<K, V>
    where
        K: Eq + Hash,
        V: Eq,
    {
    }

    impl<K, V> Default for DefaultHashMap<K, V>
    where
        V: Default,
    {
        fn default() -> DefaultHashMap<K, V> {
            DefaultHashMap::new(V::default())
        }
    }

    impl<K, V> std::ops::Index<K> for DefaultHashMap<K, V>
    where
        K: Eq + Hash,
    {
        type Output = V;

        #[inline]
        fn index(&self, key: K) -> &V {
            self.get(&key)
        }
    }

    impl<K, V> std::ops::IndexMut<K> for DefaultHashMap<K, V>
    where
        K: Eq + Hash,
        V: Clone,
    {
        #[inline]
        fn index_mut(&mut self, key: K) -> &mut V {
            self.get_mut(key)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::default_hash_map::DefaultHashMap;
    use std::collections::HashMap;

    #[test]
    fn test_basic_usage() {
        let mut map: DefaultHashMap<i64, i64> = DefaultHashMap::new(0);
        assert_eq!(map[100], 0);
        map[100] = 10;
        assert_eq!(map[100], 10);
        assert_eq!(map[10], 0);
        map[100] = 11;
        assert_eq!(map[100], 11);
    }

    #[test]
    fn test_from_hash_map() {
        let mut hm = HashMap::new();
        hm.insert(1, 10);
        let map = DefaultHashMap::from_hash_map(hm, -1);
        assert_eq!(map[1], 10);
        assert_eq!(map[2], -1);
    }

    #[test]
    fn test_raw() {
        let mut map = DefaultHashMap::new(0);
        map.insert(1, 10);
        let raw = map.raw();
        assert_eq!(raw.get(&1), Some(&10));
        raw.insert(2, 20);
        assert_eq!(map[2], 20);
    }

    #[test]
    fn test_iterators() {
        let mut map = DefaultHashMap::new(0);
        map.insert(1, 10);
        map.insert(2, 20);

        let keys: Vec<_> = map.keys().copied().collect();
        assert!(keys.contains(&1));
        assert!(keys.contains(&2));
        assert_eq!(keys.len(), 2);

        let values: Vec<_> = map.values().copied().collect();
        assert!(values.contains(&10));
        assert!(values.contains(&20));
        assert_eq!(values.len(), 2);

        for v in map.values_mut() {
            *v += 1;
        }
        assert_eq!(map[1], 11);
        assert_eq!(map[2], 21);

        let items: Vec<_> = map.iter().map(|(&k, &v)| (k, v)).collect();
        assert!(items.contains(&(1, 11)));
        assert!(items.contains(&(2, 21)));

        for (&k, v) in map.iter_mut() {
            if k == 1 {
                *v = 100;
            }
        }
        assert_eq!(map[1], 100);
    }

    #[test]
    fn test_metadata() {
        let mut map = DefaultHashMap::new(0);
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);

        map.insert(1, 10);
        assert!(!map.is_empty());
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_get_and_remove() {
        let mut map = DefaultHashMap::new(-1);
        map.insert(1, 10);
        assert_eq!(map.get(&1), &10);
        assert_eq!(map.get(&2), &-1);

        assert_eq!(map.remove(&1), Some(10));
        assert_eq!(map.remove(&1), None);
        assert_eq!(map.get(&1), &-1);
    }

    #[test]
    fn test_equality() {
        let map1 = DefaultHashMap::from_hash_map(HashMap::from([(1, 10)]), 0);
        let map2 = DefaultHashMap::from_hash_map(HashMap::from([(1, 10)]), 0);
        assert_eq!(map1, map2);

        let map3 = DefaultHashMap::from_hash_map(HashMap::from([(1, 11)]), 0);
        assert_ne!(map1, map3);

        let map4 = DefaultHashMap::from_hash_map(HashMap::from([(1, 10)]), 1);
        assert_ne!(map1, map4);
    }

    #[test]
    fn test_default_trait() {
        let map: DefaultHashMap<i32, i32> = DefaultHashMap::default();
        assert_eq!(map.get(&1), &0);
    }
}
