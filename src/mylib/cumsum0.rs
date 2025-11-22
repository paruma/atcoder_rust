use cargo_snippet::snippet;

#[snippet(prefix = "use cumsum::*;")]
pub mod cumsum {
    pub fn prefix_sum(xs: &[i64]) -> Vec<i64> {
        let mut prefix_sum = vec![0; xs.len() + 1];
        for i in 1..xs.len() + 1 {
            prefix_sum[i] = prefix_sum[i - 1] + xs[i - 1];
        }
        prefix_sum
    }

    use std::ops::{Bound, Range, RangeBounds};

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct CumSum {
        pub cumsum: Vec<i64>,
    }

    impl CumSum {
        /// # 計算量
        /// O(|xs|)
        pub fn new(xs: &[i64]) -> CumSum {
            let mut cumsum = vec![0; xs.len() + 1];
            for i in 1..xs.len() + 1 {
                cumsum[i] = cumsum[i - 1] + xs[i - 1];
            }
            CumSum { cumsum }
        }

        fn open(&self, range: impl RangeBounds<usize>) -> Range<usize> {
            use Bound::Excluded;
            use Bound::Included;
            use Bound::Unbounded;
            let begin = match range.start_bound() {
                Unbounded => 0,
                Included(&x) => x,
                Excluded(&x) => x + 1,
            };
            let end = match range.end_bound() {
                Excluded(&x) => x,
                Included(&x) => x + 1,
                Unbounded => self.cumsum.len() - 1, // xs.len() == self.cumsum.len() - 1
            };
            begin..end
        }

        /// 区間 `[begin, end)` の要素の和を計算します。
        ///
        /// # 計算量
        /// O(1)
        pub fn range_sum(&self, range: impl RangeBounds<usize>) -> i64 {
            let range = self.open(range);
            self.cumsum[range.end] - self.cumsum[range.start]
        }

        /// 区間 `[0, end)` での和を計算します。
        ///
        /// # 計算量
        /// O(1)
        pub fn prefix_sum(&self, end: usize) -> i64 {
            self.cumsum[end]
        }

        /// 区間 `[begin, n)` の要素の和を計算します。（`n` は元の配列の長さ）
        ///
        /// # 計算量
        /// O(1)
        pub fn suffix_sum(&self, begin: usize) -> i64 {
            self.cumsum[self.cumsum.len() - 1] - self.cumsum[begin]
        }

        /// `f(sum(l..r))` が `true` となる最大の `r in [l, n]` を見つける。
        /// `n` は元の配列の長さ。
        ///
        /// `f` は単調でなければならない。
        /// `f(sum(l..i))` が `true` => `f(sum(l..j))` が `true` for all `l <= j <= i`.
        ///
        /// # Panics
        /// `l > n` の場合にパニックする。
        ///
        /// # 計算量
        /// O(log n)
        pub fn max_right<F>(&self, l: usize, mut f: F) -> usize
        where
            F: FnMut(i64) -> bool,
        {
            let n = self.cumsum.len() - 1;
            assert!(l <= n);
            assert!(f(0), "f(0) must be true");

            if f(self.range_sum(l..n)) {
                return n;
            }

            let mut ok = l;
            let mut ng = n + 1;

            while ng - ok > 1 {
                let mid = ok + (ng - ok) / 2;
                if f(self.range_sum(l..mid)) {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok
        }

        /// `f(sum(l..r))` が `true` となる最小の `l in [0, r]` を見つける。
        ///
        /// `f` は単調でなければならない。
        /// `f(sum(i..r))` が `true` => `f(sum(j..r))` が `true` for all `i <= j <= r`.
        ///
        // # Panics
        /// `r > n` の場合にパニックする。
        ///
        /// # 計算量
        /// O(log r)
        pub fn min_left<F>(&self, r: usize, mut f: F) -> usize
        where
            F: FnMut(i64) -> bool,
        {
            let n = self.cumsum.len() - 1;
            assert!(r <= n);
            assert!(f(0), "f(0) must be true");

            if f(self.range_sum(0..r)) {
                return 0;
            }

            let mut ok = r;
            let mut ng = 0;

            while ok - ng > 1 {
                let mid = ng + (ok - ng) / 2;
                if f(self.range_sum(mid..r)) {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok
        }
    }
}

#[snippet(prefix = "use cumsum_2d::*;")]
pub mod cumsum_2d {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct CumSum2D {
        pub cumsum: Vec<Vec<i64>>,
    }

    impl CumSum2D {
        pub fn new(xss: &[Vec<i64>]) -> CumSum2D {
            if xss.is_empty() {
                return CumSum2D {
                    cumsum: vec![vec![0]],
                };
            }

            let height = xss.len();
            let width = xss[0].len();
            let mut cumsum = vec![vec![0; width + 1]; height + 1];
            for y in 1..height + 1 {
                for x in 1..width + 1 {
                    cumsum[y][x] = cumsum[y - 1][x] + cumsum[y][x - 1] - cumsum[y - 1][x - 1]
                        + xss[y - 1][x - 1];
                }
            }
            CumSum2D { cumsum }
        }

        pub fn rect_sum(&self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> i64 {
            // [x1, x2) × [y1, y2) の範囲で総和を求める
            self.cumsum[y2][x2] - self.cumsum[y2][x1] - self.cumsum[y1][x2] + self.cumsum[y1][x1]
        }
    }
}

#[cfg(test)]
mod test {
    mod test_cumsum {
        use super::super::cumsum::*;

        #[test]
        fn test_prefix_sum_normal() {
            let xs = vec![1, 2, 3, 4];
            let prefix_sum = prefix_sum(&xs);
            assert_eq!(prefix_sum, vec![0, 1, 3, 6, 10]);
        }

        #[test]
        fn test_prefix_sum_empty() {
            let xs = vec![];
            let prefix_sum = prefix_sum(&xs);
            assert_eq!(prefix_sum, vec![0]);
        }

        #[test]
        fn test_cumsum_normal() {
            let xs = vec![1, 2, 3, 4];
            let cumsum = CumSum::new(&xs);
            assert_eq!(cumsum.cumsum, vec![0, 1, 3, 6, 10]);
            assert_eq!(cumsum.range_sum(1..3), xs[1] + xs[2]);
            assert_eq!(cumsum.range_sum(2..4), xs[2] + xs[3]);
            assert_eq!(cumsum.range_sum(2..2), 0);
            assert_eq!(cumsum.range_sum(2..), xs[2] + xs[3]);
            assert_eq!(cumsum.range_sum(..2), xs[0] + xs[1]);
            assert_eq!(cumsum.range_sum(..), xs[0] + xs[1] + xs[2] + xs[3]);
            assert_eq!(cumsum.range_sum(2..=3), xs[2] + xs[3]);

            assert_eq!(cumsum.prefix_sum(3), 6);
            assert_eq!(cumsum.suffix_sum(1), 9);
        }

        #[test]
        fn test_cumsum_binary_search() {
            let xs = vec![1, 2, 3, 4, 5];
            let cumsum = CumSum::new(&xs);

            // max_right
            // sum(1..r) <= 5
            // 1..1 -> 0
            // 1..2 -> 2
            // 1..3 -> 2+3=5
            // 1..4 -> 2+3+4=9
            assert_eq!(cumsum.max_right(1, |sum| sum <= 5), 3);
            assert_eq!(cumsum.max_right(1, |sum| sum <= 4), 2);
            assert_eq!(cumsum.max_right(1, |sum| sum < 5), 2);

            // sum(0..r) <= 10
            // 0..1 -> 1
            // 0..2 -> 1+2=3
            // 0..3 -> 1+2+3=6
            // 0..4 -> 1+2+3+4=10
            // 0..5 -> 1+2+3+4+5=15
            assert_eq!(cumsum.max_right(0, |sum| sum <= 10), 4);
            assert_eq!(cumsum.max_right(0, |sum| sum <= 9), 3);

            // all true
            assert_eq!(cumsum.max_right(0, |sum| sum <= 100), 5);

            // min_left
            // sum(l..4) <= 7
            // l=4 -> 0
            // l=3 -> 4
            // l=2 -> 3+4=7
            // l=1 -> 2+3+4=9
            assert_eq!(cumsum.min_left(4, |sum| sum <= 7), 2);
            assert_eq!(cumsum.min_left(4, |sum| sum < 7), 3);

            // sum(l..5) <= 15
            // l=5 -> 0
            // l=4 -> 5
            // l=3 -> 4+5=9
            // l=2 -> 3+4+5=12
            // l=1 -> 2+3+4+5=14
            // l=0 -> 1+2+3+4+5=15
            assert_eq!(cumsum.min_left(5, |sum| sum <= 15), 0);
            assert_eq!(cumsum.min_left(5, |sum| sum < 15), 1);
            assert_eq!(cumsum.min_left(5, |sum| sum <= 13), 2);

            // all true
            assert_eq!(cumsum.min_left(5, |sum| sum >= 0), 0);
        }

        #[test]
        #[should_panic(expected = "f(0) must be true")]
        fn test_max_right_panic() {
            let xs = vec![1, 2, 3, 4, 5];
            let cumsum = CumSum::new(&xs);
            // f(0) is false, should panic
            cumsum.max_right(2, |sum| sum < 0);
        }

        #[test]
        #[should_panic(expected = "f(0) must be true")]
        fn test_min_left_panic() {
            let xs = vec![1, 2, 3, 4, 5];
            let cumsum = CumSum::new(&xs);
            // f(0) is false, should panic
            cumsum.min_left(3, |sum| sum < 0);
        }

        #[test]
        fn test_cumsum_empty() {
            let xs = vec![];
            let cumsum = CumSum::new(&xs);
            assert_eq!(cumsum.cumsum, vec![0]);
            assert_eq!(cumsum.range_sum(..), 0);
            assert_eq!(cumsum.range_sum(..), 0);
            assert_eq!(cumsum.range_sum(0..0), 0);
            assert_eq!(cumsum.range_sum(..0), 0);
        }

        #[ignore]
        #[test]
        fn test_random_max_right() {
            use rand::{Rng, SeedableRng};

            let mut rng = rand::rngs::SmallRng::seed_from_u64(42);

            for _ in 0..100 {
                // 100 trials
                let n = rng.random_range(1..=50);
                // `max_right` の単調性のために、もとの配列の値は非負であるようにする
                let naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(0..=100)).collect();
                let cumsum = CumSum::new(&naive_vec);

                for _ in 0..100 {
                    // 100 operations
                    let l = rng.random_range(0..=n);
                    let total_sum: i64 = naive_vec.iter().sum();
                    let threshold = rng.random_range(0..=total_sum.saturating_add(100));

                    let f = |sum: i64| sum <= threshold;
                    assert!(f(0), "f(0) must be true for random test");

                    // Naive implementation
                    let mut expected = l;
                    for r in l..=n {
                        let current_sum: i64 = naive_vec[l..r].iter().sum();
                        if f(current_sum) {
                            expected = r;
                        } else {
                            break;
                        }
                    }

                    let actual = cumsum.max_right(l, f);
                    assert_eq!(
                        actual, expected,
                        "max_right failed for l={}, threshold={}\nvec: {:?}",
                        l, threshold, naive_vec
                    );
                }
            }
        }

        #[ignore]
        #[test]
        fn test_random_min_left() {
            use rand::{Rng, SeedableRng};

            let mut rng = rand::rngs::SmallRng::seed_from_u64(43); // 異なるシードを使用

            for _ in 0..100 {
                // 100 trials
                let n = rng.random_range(1..=50);
                // `min_left` の単調性のために、もとの配列の値は非負であるようにする
                let naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(0..=100)).collect();
                let cumsum = CumSum::new(&naive_vec);

                for _ in 0..100 {
                    // 100 operations
                    let r = rng.random_range(0..=n);
                    let total_sum: i64 = naive_vec.iter().sum();
                    let threshold = rng.random_range(0..=total_sum.saturating_add(100));

                    let f = |sum: i64| sum <= threshold;
                    assert!(f(0), "f(0) must be true for random test");

                    // Naive implementation
                    let expected = (0..=r)
                        .find(|&l_candidate| {
                            let current_sum: i64 = naive_vec[l_candidate..r].iter().sum();
                            f(current_sum)
                        })
                        .unwrap();

                    let actual = cumsum.min_left(r, f);
                    assert_eq!(
                        actual, expected,
                        "min_left failed for r={}, threshold={}\nvec: {:?}",
                        r, threshold, naive_vec
                    );
                }
            }
        }
    }

    mod test_cumsum_2d {
        use super::super::cumsum_2d::*;

        #[test]
        fn test_cumsum_2d_normal() {
            // [1 2]
            // [4 5]
            let xss = vec![vec![1, 2], vec![4, 5]];
            let cumsum = CumSum2D::new(&xss);
            assert_eq!(
                cumsum.cumsum,
                vec![vec![0, 0, 0], vec![0, 1, 3], vec![0, 5, 12]]
            );
            assert_eq!(cumsum.rect_sum((0, 0), (1, 2)), xss[0][0] + xss[1][0]);
            assert_eq!(
                cumsum.rect_sum((0, 0), (2, 2)),
                xss[0][0] + xss[0][1] + xss[1][0] + xss[1][1]
            );
            assert_eq!(cumsum.rect_sum((1, 1), (1, 1)), 0);
        }

        #[test]
        fn test_cumsum_2d_empty() {
            let xss = vec![];
            let cumsum = CumSum2D::new(&xss);
            assert_eq!(cumsum.cumsum, vec![vec![0]]);
            assert_eq!(cumsum.rect_sum((0, 0), (0, 0)), 0);
        }
    }
}
