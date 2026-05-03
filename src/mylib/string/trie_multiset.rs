use crate::string::trie::trie::TrieCore;
use cargo_snippet::snippet;

#[snippet(prefix = "use trie_multiset::*;", include = "trie")]
#[allow(clippy::module_inception)]
/// トライ木を用いたマルチセット（多重集合）の実装。
///
/// 挿入された各数列（文字列）の個数をノードごとに管理する。
pub mod trie_multiset {
    use super::TrieCore;

    /// トライ木を用いたマルチセットの実装。
    ///
    /// `TrieCore` を内部に持ち、各終端ノードに対応する数列の出現回数を保持する。
    pub struct TrieMultiset<T> {
        trie: TrieCore<T>,
        cnts: Vec<usize>, // cnts[v]: v を末端ノードとして持つ文字列の数
    }

    impl<T: Ord + Copy> TrieMultiset<T> {
        /// 空のトライ木マルチセットを構築する。
        ///
        /// # 計算量
        /// O(1)
        pub fn new() -> Self {
            Self {
                trie: TrieCore::new(),
                cnts: vec![0],
            }
        }

        /// 数列 `xs` をマルチセットに挿入する。
        ///
        /// # 計算量
        /// O(|xs| log Σ)
        pub fn insert(&mut self, xs: &[T]) {
            self.trie.insert(xs);
            self.cnts.resize(self.trie.num_nodes(), 0);
            let last_node = self.trie.find_node(xs).unwrap();
            self.cnts[last_node] += 1;
        }

        /// 数列 `xs` がマルチセットに含まれている個数を返す。
        ///
        /// # 計算量
        /// O(|xs| log Σ)
        pub fn count_contains(&self, xs: &[T]) -> usize {
            self.trie
                .find_node(xs)
                .map(|last_node| self.cnts[last_node])
                .unwrap_or(0)
        }

        /// トライ木に含まれるノードの総数を返す。
        ///
        /// # 計算量
        /// O(1)
        pub fn num_nodes(&self) -> usize {
            self.trie.num_nodes()
        }

        /// 数列 `xs` を辿る際に通過するノード ID のリストを返す。
        /// 途中で遷移できなくなった場合は、そこまでのノード ID リストを返す。
        ///
        /// # 計算量
        /// O(|xs| log Σ)
        pub fn trace_nodes(&self, xs: &[T]) -> Vec<usize> {
            self.trie.trace_nodes(xs)
        }
    }

    impl<T: Ord + Copy> Default for TrieMultiset<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::trie_multiset::*;

    #[test]
    fn test_basic() {
        let mut tm = TrieMultiset::new();
        tm.insert(&[0, 1]);
        tm.insert(&[0, 1]);
        tm.insert(&[0, 2]);

        assert_eq!(tm.count_contains(&[0, 1]), 2);
        assert_eq!(tm.count_contains(&[0, 2]), 1);
        assert_eq!(tm.count_contains(&[0]), 0);
        assert_eq!(tm.count_contains(&[1]), 0);
        assert_eq!(tm.num_nodes(), 4); // root, 0, 0-1, 0-2
    }

    #[test]
    fn test_default() {
        let tm = TrieMultiset::<i32>::default();
        assert_eq!(tm.num_nodes(), 1);
    }

    #[test]
    fn test_empty_string() {
        let mut tm = TrieMultiset::new();
        tm.insert(&[]);
        assert_eq!(tm.count_contains(&[]), 1);
        assert_eq!(tm.count_contains(&[0]), 0);

        tm.insert(&[]);
        assert_eq!(tm.count_contains(&[]), 2);
    }

    #[test]
    fn test_trace_nodes() {
        let mut tm = TrieMultiset::new();
        tm.insert(&[0, 1, 2]);
        let path = tm.trace_nodes(&[0, 1]);
        assert_eq!(path.len(), 3);
        assert_eq!(tm.trace_nodes(&[0, 1, 2]).len(), 4);
        assert_eq!(tm.trace_nodes(&[0, 3]).len(), 2);
    }

    #[test]
    fn test_random() {
        use rand::{Rng, SeedableRng, rngs::SmallRng};
        use std::collections::HashMap;

        let mut rng = SmallRng::seed_from_u64(42);
        for _ in 0..50 {
            let mut tm = TrieMultiset::new();
            let mut map = HashMap::new();

            let n = rng.random_range(1..50);
            for _ in 0..n {
                let len = rng.random_range(0..10);
                let xs: Vec<u8> = (0..len).map(|_| rng.random_range(0..5)).collect();
                tm.insert(&xs);
                *map.entry(xs).or_insert(0) += 1;
            }

            for (xs, &count) in &map {
                assert_eq!(tm.count_contains(xs), count);
            }

            for _ in 0..100 {
                let len = rng.random_range(0..10);
                let xs: Vec<u8> = (0..len).map(|_| rng.random_range(0..5)).collect();
                let expected = map.get(&xs).copied().unwrap_or(0);
                assert_eq!(tm.count_contains(&xs), expected);
            }
        }
    }
}
