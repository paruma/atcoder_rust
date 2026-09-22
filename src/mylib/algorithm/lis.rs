use cargo_snippet::snippet;

#[snippet(prefix = "use lis::*;")]
#[allow(clippy::module_inception)]
pub mod lis {
    use ac_library::{Max, Segtree};
    use itertools::Itertools;
    use superslice::Ext;

    /// 各要素を末尾とする最長増加部分列 (LIS) の長さを求める
    ///
    /// 戻り値の `i` 番目の要素は、`xs[i]` を末尾とする LIS の長さを表す。
    ///
    /// # 計算量
    /// O(N log N)
    pub fn lis_array<T: Ord>(xs: &[T]) -> Vec<i64> {
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

        // dp[i] = xs[0..=i] における i を末尾とした LIS 長
        let mut dp = vec![0; n];

        // seg[x] = x が LIS の末尾の値となるような LIS 長の最大値
        // そのような LIS が存在しない場合は i64::MIN
        let mut seg = Segtree::<Max<i64>>::new(sorted.len());
        for (i, &x) in rank.iter().enumerate() {
            let prev_lis = seg.prod(..x);
            dp[i] = if prev_lis == i64::MIN {
                // i を単独の LIS とする
                1
            } else {
                prev_lis + 1
            };
            if seg.get(x) < dp[i] {
                seg.set(x, dp[i]);
            }
        }

        dp
    }

    /// 最長増加部分列 (LIS) の長さを求める
    ///
    /// # 計算量
    /// O(N log N)
    pub fn lis_len<T: Ord>(xs: &[T]) -> i64 {
        // dp[l] = LIS が l+1 となるような末尾の値の最小値
        let mut dp = vec![];
        for x in xs {
            let i = dp.lower_bound(&x);
            if i < dp.len() {
                dp[i] = x;
            } else {
                dp.push(x);
            }
        }
        dp.len() as i64
    }
}

#[snippet(prefix = "use lis_restore::*;")]
#[allow(clippy::module_inception)]
pub mod lis_restore {
    use itertools::Itertools;

    use ac_library::{Segtree, segtree::Monoid};
    use std::convert::Infallible;
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct MaxI64Usize(Infallible);
    impl Monoid for MaxI64Usize {
        type S = (i64, usize);
        fn identity() -> Self::S {
            (i64::MIN, usize::MIN)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            (*a).max(*b)
        }
    }

    #[derive(Clone, Debug)]
    pub struct LisRestoreResult {
        pub lis_array: Vec<i64>,
        pub prev: Vec<Option<usize>>,
    }

    impl LisRestoreResult {
        /// LIS を1つ求める
        pub fn restore(&self) -> Vec<usize> {
            if self.lis_array.is_empty() {
                vec![]
            } else {
                let lis_len = self.lis_len();
                let lis_last = self.lis_array.iter().position(|l| *l == lis_len).unwrap();
                self.restore_with_last(lis_last)
            }
        }

        /// LIS の長さを求める
        pub fn lis_len(&self) -> i64 {
            self.lis_array.iter().copied().max().unwrap_or(0)
        }

        // xs[0..=last] で末尾が xs[last] の LIS を1つ求める
        pub fn restore_with_last(&self, last: usize) -> Vec<usize> {
            let mut path = std::iter::successors(Some(last), |acc| self.prev[*acc]).collect_vec();
            path.reverse();
            path
        }
    }

    pub fn lis_restore<T: Ord>(xs: &[T]) -> LisRestoreResult {
        let sorted = xs.iter().sorted().dedup().collect_vec();
        // xs を座標圧縮したもの
        let rank = xs
            .iter()
            .map(|x| sorted.binary_search(&x).unwrap())
            .collect_vec();

        let n = xs.len();

        let mut lis_array = vec![0; n];
        let mut prev = vec![None; n];
        // seg[x] = xを末尾としたときの LIS の (長さ, 末尾の添字)
        // LIS が存在しない場合は (i64::MIN, usize::MIN)
        let mut seg = Segtree::<MaxI64Usize>::from(vec![(i64::MIN, usize::MIN); sorted.len()]);
        for (i, &x) in rank.iter().enumerate() {
            let (prev_lis, prev_idx) = seg.prod(..x);
            if prev_lis == i64::MIN {
                lis_array[i] = 1;
                prev[i] = None;
            } else {
                lis_array[i] = prev_lis + 1;
                prev[i] = Some(prev_idx);
            }
            if seg.get(x).0 < lis_array[i] {
                seg.set(x, (lis_array[i], i));
            }
        }
        LisRestoreResult { lis_array, prev }
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithm::lis::lis_restore::lis_restore;

    use super::lis::*;
    use itertools::Itertools;

    #[test]
    fn test_lis_array() {
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
    fn test_lis_len() {
        assert_eq!(lis_len(&[1, 3, 5, 2, 4, 6]), 4);
        assert_eq!(lis_len(&[1, 1, 1]), 1);
        assert_eq!(lis_len(&[5, 4, 3, 2, 1]), 1);
        assert_eq!(lis_len::<i32>(&[]), 0);
        assert_eq!(lis_len(&[10]), 1);
        assert_eq!(lis_len(&[1, 2, 3, 1, 2, 3]), 3);
        assert_eq!(lis_len(&[3, 1, 4, 1, 5, 9, 2, 6, 5]), 4);
    }

    #[test]
    #[ignore]
    fn test_random_lis_array() {
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

    #[test]
    #[ignore]
    fn test_random_lis_len() {
        use rand::Rng;
        let mut rng = rand::rng();
        for _ in 0..1000 {
            let n = rng.random_range(0..=50);
            let xs = (0..n).map(|_| rng.random_range(0..=50)).collect_vec();
            let expected = lis_array_naive(&xs).into_iter().max().unwrap_or(0);
            let actual = lis_len(&xs);
            assert_eq!(actual, expected, "xs: {:?}", xs);
        }
    }

    fn lis_array_naive<T: Ord>(xs: &[T]) -> Vec<i64> {
        let n = xs.len();
        if n == 0 {
            return vec![];
        }
        let mut dp = vec![1_i64; n];
        for i in 0..n {
            for j in 0..i {
                if xs[j] < xs[i] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        dp
    }

    #[test]
    fn test_lis_restore() {
        let xs = [1, 3, 5, 2, 4, 6];
        let res = lis_restore(&xs);
        assert_eq!(res.lis_array, vec![1, 2, 3, 2, 3, 4]);
        assert_eq!(res.lis_len(), 4);

        let restored = res.restore();
        assert_eq!(restored.len() as i64, 4);
        assert_strictly_increasing_subsequence(&xs, &restored);

        let restored_with_last = res.restore_with_last(2);
        assert_eq!(restored_with_last.len() as i64, res.lis_array[2]);
        assert_eq!(restored_with_last.last(), Some(&2));
        assert_strictly_increasing_subsequence(&xs, &restored_with_last);

        let duplicate = lis_restore(&[1, 1, 1]);
        assert_eq!(duplicate.lis_array, vec![1, 1, 1]);
        assert_eq!(duplicate.restore().len() as i64, 1);

        let empty = lis_restore::<i32>(&[]);
        assert_eq!(empty.lis_array, vec![]);
        assert_eq!(empty.lis_len(), 0);
        assert_eq!(empty.restore(), vec![]);
    }

    fn assert_strictly_increasing_subsequence<T: Ord>(xs: &[T], indices: &[usize]) {
        assert!(
            indices
                .windows(2)
                .all(|pair| pair[0] < pair[1] && xs[pair[0]] < xs[pair[1]])
        );
    }
}
