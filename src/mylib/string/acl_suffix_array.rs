use cargo_snippet::snippet;

#[snippet(prefix = "use acl_suffix_array_finder::*;")]
pub mod acl_suffix_array_finder {
    use ac_library::suffix_array_arbitrary;
    use superslice::Ext;

    /// Suffix Array を用いた文字列検索を行う構造体
    #[derive(Clone, Debug)]
    pub struct SuffixArrayFinder<'a, T: Ord> {
        target: &'a [T],
        sa: Vec<usize>,
    }

    impl<'a, T: Ord> SuffixArrayFinder<'a, T> {
        /// 指定された `target` に対して SuffixArray を構築し、検索の準備を行う。
        ///
        /// 計算量は O(|T| log |T|)
        pub fn new(target: &'a [T]) -> Self {
            let sa = suffix_array_arbitrary(target);
            SuffixArrayFinder { target, sa }
        }

        /// `target` に出現する `pattern` の開始位置をすべて返す。
        /// 結果はソートされているとは限らない。
        ///
        /// 計算量は O(|P| log |T|)
        pub fn find_all(&self, pattern: &[T]) -> &[usize] {
            let range = self.sa.equal_range_by_key(&pattern, |&begin| {
                let suffix = &self.target[begin..];
                &suffix[..pattern.len().min(suffix.len())]
            });
            &self.sa[range]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::acl_suffix_array_finder::SuffixArrayFinder;
    use itertools::Itertools;
    use rand::{Rng, SeedableRng, rngs::SmallRng};

    #[test]
    fn test_basic() {
        let target = "abracadabra".chars().collect_vec();
        let finder = SuffixArrayFinder::new(&target);

        // "abra" は 0番目と 7番目に出現
        let pattern = "abra".chars().collect_vec();
        let matches = finder
            .find_all(&pattern)
            .iter()
            .copied()
            .sorted()
            .collect_vec();
        assert_eq!(matches, vec![0, 7]);

        // "a" は 0, 3, 5, 7, 10
        let pattern = "a".chars().collect_vec();
        let matches = finder
            .find_all(&pattern)
            .iter()
            .copied()
            .sorted()
            .collect_vec();
        assert_eq!(matches, vec![0, 3, 5, 7, 10]);

        // 存在しない
        let pattern = "z".chars().collect_vec();
        assert!(finder.find_all(&pattern).is_empty());
    }

    #[test]
    fn test_edge_cases() {
        // 空のターゲット
        let target: Vec<char> = vec![];
        let finder = SuffixArrayFinder::new(&target);
        let pattern = vec!['a'];
        assert!(finder.find_all(&pattern).is_empty());

        // ターゲットより長いパターン
        let target = vec!['a', 'b'];
        let finder = SuffixArrayFinder::new(&target);
        let pattern = vec!['a', 'b', 'c'];
        assert!(finder.find_all(&pattern).is_empty());

        // パターンが空（全ての接尾辞の接頭辞とみなされ、SA 全体が返る）
        let pattern: Vec<char> = vec![];
        // "ab" の SA は [0, 1] ("ab", "b")
        assert_eq!(finder.find_all(&pattern), &[0, 1]);
    }

    #[test]
    #[ignore]
    fn test_random() {
        let mut rng = SmallRng::seed_from_u64(42);
        for _ in 0..1000 {
            let n = rng.random_range(1..50);
            let target: Vec<u8> = (0..n).map(|_| rng.random_range(b'a'..b'd')).collect();
            let finder = SuffixArrayFinder::new(&target);

            let m = rng.random_range(1..10);
            let pattern: Vec<u8> = (0..m).map(|_| rng.random_range(b'a'..b'd')).collect();

            // 愚直な検索
            let expected = target
                .windows(pattern.len())
                .enumerate()
                .filter(|(_, window)| *window == pattern)
                .map(|(i, _)| i)
                .collect_vec();

            let actual = finder
                .find_all(&pattern)
                .iter()
                .copied()
                .sorted()
                .collect_vec();

            assert_eq!(
                actual, expected,
                "Failed on target={:?}, pattern={:?}",
                target, pattern
            );
        }
    }
}
