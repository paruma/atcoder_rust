use cargo_snippet::snippet;

#[snippet(prefix = "use range_add_imos::*;")]
#[allow(clippy::module_inception)]
pub mod range_add_imos {
    /// いもす法（差分配列）を用いて、配列に対する区間加算クエリを効率的に処理するデータ構造です。
    /// 最終的な配列の状態を一度に計算する場合（オフライン処理）に特に有用です。
    ///
    /// 各区間加算操作はO(1)で、最終的な配列を構築するのにO(N)かかります。
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct RangeAddImos {
        n: usize,
        diff: Vec<i64>,
    }

    impl RangeAddImos {
        /// サイズ `n` の新しい `RangeAddImos` インスタンスを作成します。
        ///
        /// # 計算量
        /// O(n)
        pub fn new(n: usize) -> Self {
            Self {
                n,
                diff: vec![0; n + 1],
            }
        }

        /// 指定された `range` に `x` を加算します。
        ///
        /// # Panics
        /// 範囲が不正な場合にパニックします。
        ///
        /// # 計算量
        /// O(1)
        pub fn range_add(&mut self, range: impl std::ops::RangeBounds<usize>, x: i64) {
            let range = open_range_bounds(range, self.n);
            let l = range.start;
            let r = range.end;

            assert!(l <= r && r <= self.n);

            self.diff[l] += x;
            self.diff[r] -= x;
        }

        /// 差分配列から最終的な配列を構築します。
        ///
        /// # 計算量
        /// O(n)
        pub fn to_vec(mut self) -> Vec<i64> {
            if self.n == 0 {
                return Vec::new();
            }
            for i in 1..self.n {
                self.diff[i] += self.diff[i - 1];
            }
            self.diff.truncate(self.n);
            self.diff
        }

        /// 指定されたインデックス `p` に `x` を加算します。
        ///
        /// # Panics
        /// `p >= n` の場合にパニックする可能性があります。
        /// `range_add` の内部で範囲チェックが行われます。
        ///
        /// # 計算量
        /// O(1)
        pub fn add(&mut self, p: usize, x: i64) {
            self.range_add(p..(p + 1), x);
        }

        /// 配列スライスから`RangeAddImos`インスタンスを作成します。
        ///
        /// # 計算量
        /// O(n)
        pub fn from_slice(xs: &[i64]) -> Self {
            let n = xs.len();
            let mut diff = vec![0; n + 1];
            if n > 0 {
                diff[0] = xs[0];
                for i in 1..n {
                    diff[i] = xs[i] - xs[i - 1];
                }
            }
            Self { n, diff }
        }
    }

    fn open_range_bounds(
        range: impl std::ops::RangeBounds<usize>,
        len: usize,
    ) -> std::ops::Range<usize> {
        use std::ops::Bound::{Excluded, Included, Unbounded};
        let l = match range.start_bound() {
            Unbounded => 0,
            Included(&x) => x,
            Excluded(&x) => x + 1,
        };
        let r = match range.end_bound() {
            Unbounded => len,
            Included(&x) => x + 1,
            Excluded(&x) => x,
        };
        l..r
    }
}

#[cfg(test)]
mod tests {
    use super::range_add_imos::*;
    use rand::{Rng, SeedableRng};

    #[test]
    fn test_range_add_imos_basic() {
        let n = 10;
        let mut imos = RangeAddImos::new(n);
        // 初期状態はすべてゼロであるべき
        assert_eq!(imos.clone().to_vec(), vec![0; n]);
        // Add 5 to [2, 5)
        // Expected: [0, 0, 5, 5, 5, 0, 0, 0, 0, 0]
        imos.range_add(2..5, 5);
        assert_eq!(imos.clone().to_vec(), vec![0, 0, 5, 5, 5, 0, 0, 0, 0, 0]);
        // Add -3 to [0, 3)
        // Expected: [-3, -3, 2, 5, 5, 0, 0, 0, 0, 0]
        imos.range_add(0..3, -3);
        assert_eq!(imos.clone().to_vec(), vec![-3, -3, 2, 5, 5, 0, 0, 0, 0, 0]);
        // Add 10 to [8, 10)
        // Expected: [-3, -3, 2, 5, 5, 0, 0, 0, 10, 10]
        imos.range_add(8..10, 10);
        assert_eq!(
            imos.clone().to_vec(),
            vec![-3, -3, 2, 5, 5, 0, 0, 0, 10, 10]
        );
        // Add to empty range
        imos.range_add(5..5, 100);
        assert_eq!(
            imos.clone().to_vec(),
            vec![-3, -3, 2, 5, 5, 0, 0, 0, 10, 10]
        );

        // Add 1 to [1..=3] (indices 1, 2, 3)
        // Expected: [-3, -2, 3, 6, 5, 0, 0, 0, 10, 10]
        imos.range_add(1..=3, 1);
        assert_eq!(
            imos.clone().to_vec(),
            vec![-3, -2, 3, 6, 5, 0, 0, 0, 10, 10]
        );

        // Add 10 to index 0
        // Expected: [7, -2, 3, 6, 5, 0, 0, 0, 10, 10]
        imos.add(0, 10);
        assert_eq!(imos.clone().to_vec(), vec![7, -2, 3, 6, 5, 0, 0, 0, 10, 10]);
    }

    #[test]
    fn test_empty() {
        let mut imos = RangeAddImos::new(0);
        // 初期状態は空のVecであるべき
        assert_eq!(imos.clone().to_vec(), Vec::<i64>::new());
        imos.range_add(0..0, 10);
        assert_eq!(imos.to_vec(), Vec::<i64>::new());
    }

    #[test]
    fn test_from_slice() {
        let xs = vec![1, 2, 3, 4, 5];
        let imos = RangeAddImos::from_slice(&xs);
        assert_eq!(imos.to_vec(), xs);

        let empty_xs = Vec::<i64>::new();
        let empty_imos = RangeAddImos::from_slice(&empty_xs);
        assert_eq!(empty_imos.to_vec(), empty_xs);

        let single_xs = vec![10];
        let single_imos = RangeAddImos::from_slice(&single_xs);
        assert_eq!(single_imos.to_vec(), single_xs);
    }

    #[ignore]
    #[test]
    fn test_random_range_add_imos() {
        let mut rng = rand::rngs::SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.random_range(1..=100);
            let mut naive_vec = vec![0i64; n];
            let mut imos = RangeAddImos::new(n);

            for _ in 0..100 {
                let l = rng.random_range(0..=n);
                let r = rng.random_range(l..=n);
                let x = rng.random_range(-1000..=1000);

                // ナイーブな実装に適用
                for i in l..r {
                    naive_vec[i] += x;
                }
                // imos法に適用
                imos.range_add(l..r, x);
            }
            assert_eq!(imos.to_vec(), naive_vec);
        }
    }
}
