use cargo_snippet::snippet;

#[snippet(prefix = "use lis::*;")]
#[allow(clippy::module_inception)]
pub mod lis {
    use ac_library::{Max, Segtree};
    use itertools::Itertools;

    /// 各要素を末尾とする最長増加部分列 (LIS) の長さを求める
    ///
    /// 戻り値の `i` 番目の要素は、`xs[i]` を末尾とする LIS の長さを表す。
    ///
    /// # 計算量
    /// O(N log N)
    pub fn lis_array<T: Ord>(xs: &[T]) -> Vec<usize> {
        let n = xs.len();
        if n == 0 {
            return vec![];
        }
        let sorted = xs.iter().sorted().dedup().collect_vec();
        // xs を座標圧縮したもの
        let rank = xs
            .iter()
            .map(|x| sorted.binary_search(&x).unwrap())
            .collect_vec();

        let mut dp = vec![0; n];
        let mut seg = Segtree::<Max<usize>>::new(sorted.len());
        for (i, &x) in rank.iter().enumerate() {
            dp[i] = seg.prod(..x) + 1;
            if seg.get(x) < dp[i] {
                seg.set(x, dp[i]);
            }
        }

        dp
    }
}

#[cfg(test)]
mod tests {
    use super::lis::*;
    use itertools::Itertools;

    #[test]
    fn test_lis() {
        assert_eq!(lis_array(&[1, 3, 5, 2, 4, 6]), vec![1, 2, 3, 2, 3, 4]);
        assert_eq!(lis_array(&[1, 1, 1]), vec![1, 1, 1]);
        assert_eq!(lis_array(&[5, 4, 3, 2, 1]), vec![1, 1, 1, 1, 1]);
        assert_eq!(lis_array::<i32>(&[]), vec![]);
        assert_eq!(lis_array(&[10]), vec![1]);
        assert_eq!(lis_array(&[1, 2, 3, 1, 2, 3]), vec![1, 2, 3, 1, 2, 3]);
        assert_eq!(
            lis_array(&[3, 1, 4, 1, 5, 9, 2, 6, 5]),
            vec![1, 1, 2, 1, 3, 4, 2, 4, 3]
        );
    }

    #[test]
    #[ignore]
    fn test_random_lis() {
        use rand::Rng;
        let mut rng = rand::rng();
        for _ in 0..1000 {
            let n = rng.random_range(0..=50);
            let xs = (0..n).map(|_| rng.random_range(0..=50)).collect_vec();
            let expected = lis_array_naive(&xs);
            let actual = lis_array(&xs);
            assert_eq!(actual, expected, "xs: {:?}", xs);
        }
    }

    fn lis_array_naive<T: Ord>(xs: &[T]) -> Vec<usize> {
        let n = xs.len();
        if n == 0 {
            return vec![];
        }
        let mut dp = vec![1; n];
        for i in 0..n {
            for j in 0..i {
                if xs[j] < xs[i] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        dp
    }
}
