pub fn find_target_by_z_algorithm<T: Ord + Clone>(target: &[T], pattern: &[T]) -> Vec<usize> {
    use ac_library::z_algorithm_arbitrary;
    use itertools::Itertools;

    let pt = [pattern, target].concat();
    let z_arr = z_algorithm_arbitrary(&pt);
    z_arr[pattern.len()..]
        .iter()
        .positions(|l| *l >= pattern.len())
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use rand::{Rng, SeedableRng, rngs::SmallRng};

    #[test]
    fn test_basic() {
        let target = "abracadabra".chars().collect_vec();
        let pattern = "abra".chars().collect_vec();

        // 期待される正しい一致位置
        let expected = vec![0, 7];
        let actual = find_target_by_z_algorithm(&target, &pattern);

        assert_eq!(
            actual, expected,
            "Basic case failed: expected {:?}, got {:?}",
            expected, actual
        );
    }

    #[test]
    fn test_edge_cases() {
        let target = "ab".chars().collect_vec();
        let pattern = "abc".chars().collect_vec();

        // パターンのほうが長い場合は空であるべき
        let expected: Vec<usize> = vec![];
        let actual = find_target_by_z_algorithm(&target, &pattern);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_empty_cases() {
        // target が空
        let target: Vec<char> = vec![];
        let pattern = vec!['a'];
        let actual = find_target_by_z_algorithm(&target, &pattern);
        assert!(actual.is_empty());

        // pattern が空
        let target = vec!['a', 'b'];
        let pattern: Vec<char> = vec![];
        let actual = find_target_by_z_algorithm(&target, &pattern);
        // 仕様を確認するための println (テスト失敗時のみ表示)
        println!("target='ab', pattern='' actual: {:?}", actual);

        // 両方空
        let target: Vec<char> = vec![];
        let pattern: Vec<char> = vec![];
        let actual = find_target_by_z_algorithm(&target, &pattern);
        println!("target='', pattern='' actual: {:?}", actual);
    }

    #[test]
    #[ignore]
    fn test_random() {
        let mut rng = SmallRng::seed_from_u64(42);
        for _ in 0..100 {
            let n = rng.random_range(1..50);
            let target: Vec<u8> = (0..n).map(|_| rng.random_range(b'a'..b'd')).collect();

            let m = rng.random_range(1..10);
            let pattern: Vec<u8> = (0..m).map(|_| rng.random_range(b'a'..b'd')).collect();

            // 愚直な検索による期待値
            let expected = target
                .windows(pattern.len())
                .enumerate()
                .filter(|(_, window)| *window == pattern)
                .map(|(i, _)| i)
                .collect_vec();

            let actual = find_target_by_z_algorithm(&target, &pattern);

            assert_eq!(
                actual, expected,
                "Random case failed: target={:?}, pattern={:?}, expected={:?}, got={:?}",
                target, pattern, expected, actual
            );
        }
    }
}
