use cargo_snippet::snippet;

#[snippet(prefix = "use trie::*;")]
#[allow(clippy::module_inception)]
pub mod trie {
    // ABC 353 E - Yet Another Sigma Problemhttps://atcoder.jp/contests/abc353/tasks/abc353_e
    // では、[usize; 26] を使うよりも BTreeMap<usize, usize> を使うほうが若干早かったので、 本ライブラリでは BTreeMap を使う

    use std::collections::BTreeMap;

    #[derive(Clone, Debug)]
    pub struct Trie<T> {
        nexts: Vec<BTreeMap<T, usize>>,
    }

    impl<T: Ord + Copy> Trie<T> {
        pub fn new() -> Self {
            Self {
                nexts: vec![BTreeMap::new()],
            }
        }

        pub fn insert(&mut self, xs: &[T]) {
            let mut cur_node = 0;
            for &x in xs {
                if !self.nexts[cur_node].contains_key(&x) {
                    let new_node = self.nexts.len();
                    self.nexts[cur_node].insert(x, new_node);
                    self.nexts.push(BTreeMap::new());
                }
                cur_node = self.nexts[cur_node][&x];
            }
        }

        pub fn next(&self, cur: usize, x: T) -> Option<usize> {
            self.nexts[cur].get(&x).copied()
        }

        pub fn node_path(&self, xs: &[T]) -> Vec<usize> {
            let mut cur_node = 0;
            let mut path = vec![cur_node];
            for &x in xs {
                let Some(&next_node) = self.nexts[cur_node].get(&x) else {
                    break;
                };
                cur_node = next_node;
                path.push(cur_node);
            }
            path
        }
    }

    impl<T: Ord + Copy> Default for Trie<T> {
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
        let mut trie = Trie::default();
        trie.insert(&[1, 2, 3]);
        assert_eq!(trie.node_path(&[1, 2, 3]).len(), 4);
    }

    #[test]
    fn test_trie_basic() {
        let mut trie = Trie::new();
        trie.insert(&[0, 1, 2]);
        trie.insert(&[0, 1, 3]);
        trie.insert(&[1, 2]);

        // node_path の検証
        let path1 = trie.node_path(&[0, 1, 2]);
        let path2 = trie.node_path(&[0, 1, 3]);
        let path3 = trie.node_path(&[1, 2]);

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
    fn test_trie_empty() {
        let mut trie = Trie::new();
        trie.insert(&[]);
        assert_eq!(trie.node_path(&[]), vec![0]);
        assert_eq!(trie.next(0, 0), None);
    }

    #[test]
    fn test_trie_not_found() {
        let mut trie = Trie::new();
        trie.insert(&[0, 1, 2]);

        // 途中で存在しないパス
        let path = trie.node_path(&[0, 2]);
        assert_eq!(path.len(), 2); // [0, (0->0) のノード] まで辿れるはずだが 0->2 はないので [0] だけか？
        // 実装を確認:
        // cur_node = 0, x = 0 -> next_node = trie.nexts[0][0], path = [0, next_node]
        // cur_node = next_node, x = 2 -> trie.nexts[next_node][2] は None -> break
        // 結果 path = [0, next_node]
        assert_eq!(path.len(), 2);
        assert_eq!(path[0], 0);
    }
}
