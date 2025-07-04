use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use default_hash_map::*;")]
pub mod default_hash_map {
    use std::hash::Hash;

    use std::{
        borrow::Borrow,
        collections::{
            hash_map::{Iter, IterMut, Keys, Values, ValuesMut},
            HashMap,
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

        pub fn keys(&self) -> Keys<K, V> {
            self.raw.keys()
        }

        pub fn values(&self) -> Values<K, V> {
            self.raw.values()
        }

        pub fn values_mut(&mut self) -> ValuesMut<K, V> {
            self.raw.values_mut()
        }

        pub fn iter(&self) -> Iter<K, V> {
            self.raw.iter()
        }

        pub fn iter_mut(&mut self) -> IterMut<K, V> {
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

    #[allow(clippy::eq_op)]
    #[test]
    fn test() {
        let mut map: DefaultHashMap<i64, i64> = DefaultHashMap::new(0);
        dbg!(map[-1]);

        map[100] = 10;
        assert_eq!(map[100], 10);
        assert_eq!(map[10], 0);
        map[100] = 11;
        assert_eq!(map[100], 11);
    }
}
