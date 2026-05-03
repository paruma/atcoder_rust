use cargo_snippet::snippet;

#[snippet(prefix = "use trie::*;")]
#[allow(clippy::module_inception)]
/// トライ木（接頭辞木）の実装。
///
/// 数列（文字列）の集合を管理し、共通接頭辞の検索やノードのパス取得を効率的に行う。
pub mod trie {
    // ABC 353 E - Yet Another Sigma Problem (https://atcoder.jp/contests/abc353/tasks/abc353_e)
    // では、[usize; 26] を使うよりも BTreeMap<T, usize> を使うほうが若干高速であったため、
    // 本ライブラリでは遷移の管理に BTreeMap を採用している。

    use std::collections::BTreeMap;

    /// トライ木の実装。
    ///
    /// 各ノードは `BTreeMap` を用いて、次の文字に対する遷移先ノード ID を保持する。
    #[derive(Clone, Debug)]
    pub struct TrieCore<T> {
        children_list: Vec<BTreeMap<T, usize>>,
    }

    impl<T: Ord + Copy> TrieCore<T> {
        /// 空のトライ木を構築する。
        ///
        /// # 計算量
        /// O(1)
        pub fn new() -> Self {
            Self {
                children_list: vec![BTreeMap::new()],
            }
        }

        /// 指定したノード `node` が持つ子ノードへの遷移情報を取得する。
        ///
        /// # 計算量
        /// O(1)
        pub fn children(&self, node: usize) -> &BTreeMap<T, usize> {
            &self.children_list[node]
        }

        /// 数列 `xs` をトライ木に挿入する。
        ///
        /// # 計算量
        /// O(|xs| log Σ) （Σ はアルファベットサイズ、ここでは文字の種類数）
        pub fn insert(&mut self, xs: &[T]) {
            let mut cur_node = 0;
            for &x in xs {
                if !self.children_list[cur_node].contains_key(&x) {
                    let new_node = self.children_list.len();
                    self.children_list[cur_node].insert(x, new_node);
                    self.children_list.push(BTreeMap::new());
                }
                cur_node = self.children_list[cur_node][&x];
            }
        }

        /// 指定したノード `cur` から、文字 `x` による遷移先のノード ID を取得する。
        /// 遷移先が存在しない場合は `None` を返す。
        ///
        /// # 計算量
        /// O(log Σ)
        pub fn next(&self, cur: usize, x: T) -> Option<usize> {
            self.children_list[cur].get(&x).copied()
        }

        /// 数列 `xs` に対応する終端ノード ID を取得する。
        /// `xs` がトライ木に含まれない場合は `None` を返す。
        ///
        /// # 計算量
        /// O(|xs| log Σ)
        pub fn find_node(&self, xs: &[T]) -> Option<usize> {
            let mut cur_node = 0;
            for &x in xs {
                let next_node = self.next(cur_node, x)?;
                cur_node = next_node;
            }
            Some(cur_node)
        }

        /// 数列 `xs` を辿る際に通過するノード ID のリストを返す。
        /// 途中で遷移できなくなった場合は、そこまでのノード ID リストを返す。
        ///
        /// # 計算量
        /// O(|xs| log Σ)
        pub fn trace_nodes(&self, xs: &[T]) -> Vec<usize> {
            let mut cur_node = 0;
            let mut path = vec![cur_node];
            for &x in xs {
                let Some(next_node) = self.next(cur_node, x) else {
                    break;
                };
                cur_node = next_node;
                path.push(cur_node);
            }
            path
        }

        /// トライ木に含まれるノードの総数を返す。
        ///
        /// # 計算量
        /// O(1)
        pub fn num_nodes(&self) -> usize {
            self.children_list.len()
        }
    }

    impl<T: Ord + Copy> Default for TrieCore<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::trie::*;

    #[test]
    fn test_trie_default() {
        let mut trie = TrieCore::default();
        trie.insert(&[1, 2, 3]);
        assert_eq!(trie.trace_nodes(&[1, 2, 3]).len(), 4);
    }

    #[test]
    fn test_trie_basic() {
        let mut trie = TrieCore::new();
        trie.insert(&[0, 1, 2]);
        trie.insert(&[0, 1, 3]);
        trie.insert(&[1, 2]);

        // trace_nodes の検証
        let path1 = trie.trace_nodes(&[0, 1, 2]);
        let path2 = trie.trace_nodes(&[0, 1, 3]);
        let path3 = trie.trace_nodes(&[1, 2]);

        assert_eq!(path1.len(), 4);
        assert_eq!(path2.len(), 4);
        assert_eq!(path3.len(), 3);

        // 共通接頭辞 [0, 1] のノード ID が一致することを確認
        assert_eq!(path1[0], path2[0]);
        assert_eq!(path1[1], path2[1]);
        assert_eq!(path1[2], path2[2]);
        assert_ne!(path1[3], path2[3]);

        // next の検証
        assert_eq!(trie.next(path1[0], 0), Some(path1[1]));
        assert_eq!(trie.next(path1[1], 1), Some(path1[2]));
        assert_eq!(trie.next(path1[2], 2), Some(path1[3]));
        assert_eq!(trie.next(path1[2], 3), Some(path2[3]));
        assert_eq!(trie.next(path1[2], 4), None);
    }

    #[test]
    fn test_trie_children() {
        let mut trie = TrieCore::new();
        trie.insert(&[1, 2]);
        trie.insert(&[1, 3]);

        let root_children = trie.children(0);
        assert_eq!(root_children.len(), 1);
        let &node_1 = root_children.get(&1).unwrap();

        let node_1_children = trie.children(node_1);
        assert_eq!(node_1_children.len(), 2);
        assert!(node_1_children.contains_key(&2));
        assert!(node_1_children.contains_key(&3));
    }

    #[test]
    fn test_trie_empty() {
        let mut trie = TrieCore::new();
        // 空の挿入
        trie.insert(&[]);
        assert_eq!(trie.num_nodes(), 1);
        assert_eq!(trie.trace_nodes(&[]), vec![0]);

        // 空の木に対する検索
        assert_eq!(trie.find_node(&[1]), None);
    }

    #[test]
    fn test_trie_not_found() {
        let mut trie = TrieCore::new();
        trie.insert(&[0, 1, 2]);

        // 途中で存在しないパス
        let path = trie.trace_nodes(&[0, 2]);
        assert_eq!(path.len(), 2);
        assert_eq!(path[0], 0);
        assert_ne!(path[1], 0);
    }

    #[test]
    fn test_random() {
        use rand::{Rng, SeedableRng, rngs::SmallRng};
        use std::collections::HashSet;

        let mut rng = SmallRng::seed_from_u64(42);
        for _ in 0..50 {
            let mut trie = TrieCore::new();
            let mut inserted = HashSet::new();

            let n = rng.random_range(1..50);
            for _ in 0..n {
                let len = rng.random_range(0..10);
                let xs: Vec<u8> = (0..len).map(|_| rng.random_range(0..5)).collect();
                trie.insert(&xs);
                inserted.insert(xs);
            }

            for xs in &inserted {
                let path = trie.trace_nodes(xs);
                assert_eq!(path.len(), xs.len() + 1);

                // find_node が trace_nodes の最後の要素と一致することを確認
                assert_eq!(trie.find_node(xs), Some(*path.last().unwrap()));
            }
        }
    }
}
