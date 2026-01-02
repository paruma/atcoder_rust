pub mod acl_suffix_finder {
    use ac_library::suffix_array_arbitrary;
    use superslice::Ext;

    pub struct SuffixArrayFinder<'a, T: Ord> {
        target: &'a [T],
        sa: Vec<usize>,
    }

    impl<'a, T: Ord> SuffixArrayFinder<'a, T> {
        pub fn new(target: &'a [T]) -> Self {
            let sa = suffix_array_arbitrary(target);
            SuffixArrayFinder { target, sa }
        }

        pub fn find(&self, pattern: &[T]) -> &[usize] {
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
    use super::acl_suffix_finder::SuffixArrayFinder;
    use itertools::Itertools;
    use rand::{Rng, SeedableRng, rngs::SmallRng};

    #[test]
    fn test_basic() {
        let target = "abracadabra".chars().collect_vec();
        let finder = SuffixArrayFinder::new(&target);

        // "abra" は 0番目と 7番目に出現
        let pattern = "abra".chars().collect_vec();
        let matches = finder.find(&pattern).iter().copied().sorted().collect_vec();
        assert_eq!(matches, vec![0, 7]);

        // "a" は 0, 3, 5, 7, 10
        let pattern = "a".chars().collect_vec();
        let matches = finder.find(&pattern).iter().copied().sorted().collect_vec();
        assert_eq!(matches, vec![0, 3, 5, 7, 10]);

        // 存在しない
        let pattern = "z".chars().collect_vec();
        assert!(finder.find(&pattern).is_empty());
    }

    #[test]
    fn test_edge_cases() {
        // 空のターゲット
        let target: Vec<char> = vec![];
        let finder = SuffixArrayFinder::new(&target);
        let pattern = vec!['a'];
        assert!(finder.find(&pattern).is_empty());

        // ターゲットより長いパターン
        let target = vec!['a', 'b'];
        let finder = SuffixArrayFinder::new(&target);
        let pattern = vec!['a', 'b', 'c'];
        assert!(finder.find(&pattern).is_empty());

        // パターンが空（全ての接尾辞の接頭辞とみなされ、SA 全体が返る）
        let pattern: Vec<char> = vec![];
        // "ab" の SA は [0, 1] ("ab", "b")
        assert_eq!(finder.find(&pattern), &[0, 1]);
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

            let actual = finder.find(&pattern).iter().copied().sorted().collect_vec();

            assert_eq!(
                actual, expected,
                "Failed on target={:?}, pattern={:?}",
                target, pattern
            );
        }
    }
}
