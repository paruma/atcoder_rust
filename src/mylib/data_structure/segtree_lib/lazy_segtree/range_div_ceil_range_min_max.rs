use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_div_ceil_range_min_max::*;")]
pub mod range_div_ceil_range_min_max {
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use num_integer::Integer;
    use std::convert::Infallible;
    use std::ops::RangeBounds;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct RangeMinMax {
        pub min: i64,
        pub max: i64,
    }

    impl RangeMinMax {
        pub fn new(min: i64, max: i64) -> Self {
            Self { min, max }
        }

        pub fn unit(x: i64) -> Self {
            Self { min: x, max: x }
        }
    }

    pub struct RangeMinMaxMonoid(Infallible);
    impl Monoid for RangeMinMaxMonoid {
        type S = RangeMinMax;
        fn identity() -> RangeMinMax {
            RangeMinMax {
                min: i64::MAX,
                max: i64::MIN,
            }
        }
        fn binary_operation(a: &RangeMinMax, b: &RangeMinMax) -> RangeMinMax {
            RangeMinMax {
                min: i64::min(a.min, b.min),
                max: i64::max(a.max, b.max),
            }
        }
    }

    pub struct RangeDivCeilRangeMinMax(Infallible);
    impl MapMonoid for RangeDivCeilRangeMinMax {
        type M = RangeMinMaxMonoid;
        type F = i64; // 正の値のみを想定

        fn identity_map() -> i64 {
            1
        }
        fn composition(a: &i64, b: &i64) -> i64 {
            a.saturating_mul(*b)
        }

        fn mapping(f: &i64, x: &RangeMinMax) -> RangeMinMax {
            let min_v = if x.min == i64::MAX {
                i64::MAX
            } else {
                Integer::div_ceil(&x.min, f)
            };
            let max_v = if x.max == i64::MIN {
                i64::MIN
            } else {
                Integer::div_ceil(&x.max, f)
            };
            RangeMinMax {
                min: min_v,
                max: max_v,
            }
        }
    }

    /// 区間切り上げ除算と区間最小値・最大値取得を行う遅延セグメント木。
    ///
    /// 以下の操作をサポートします。
    /// - **区間切り上げ除算**: 区間 `[l, r)` の各要素 `a_i` を `ceil(a_i / x)` に更新します。
    /// - **区間最小値・最大値取得**: 区間 `[l, r)` の要素の最小値と最大値 `(min, max)` を取得します。
    ///
    /// # 制約
    /// - 更新に使う値 `x` （割る数）は正の整数である必要があります。
    /// - セグメント木に乗せるデータは正負どちらでも問題ありません。
    pub struct RangeDivCeilRangeMinMaxSegtree {
        segtree: LazySegtree<RangeDivCeilRangeMinMax>,
        len: usize,
    }

    impl RangeDivCeilRangeMinMaxSegtree {
        pub fn new(xs: &[i64]) -> RangeDivCeilRangeMinMaxSegtree {
            let len = xs.len();
            let initial_data: Vec<RangeMinMax> = xs.iter().map(|&x| RangeMinMax::unit(x)).collect();
            RangeDivCeilRangeMinMaxSegtree {
                segtree: LazySegtree::from(initial_data),
                len,
            }
        }

        pub fn set(&mut self, p: usize, x: i64) {
            self.segtree.set(p, RangeMinMax::unit(x));
        }

        pub fn get(&mut self, p: usize) -> i64 {
            self.segtree.get(p).min
        }

        pub fn range_min<R>(&mut self, range: R) -> i64
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).min
        }

        pub fn range_max<R>(&mut self, range: R) -> i64
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).max
        }

        pub fn all_min(&self) -> i64 {
            self.segtree.all_prod().min
        }

        pub fn all_max(&self) -> i64 {
            self.segtree.all_prod().max
        }

        /// A[p] <- ceil(A[p] / x) を計算する
        pub fn apply_div_ceil(&mut self, p: usize, x: i64) {
            self.segtree.apply(p, x)
        }

        /// p in range に対して A[p] <- ceil(A[p] / x) を計算する
        pub fn apply_range_div_ceil<R>(&mut self, range: R, x: i64)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, x)
        }

        pub fn max_right<G>(&mut self, l: usize, g: G) -> usize
        where
            G: Fn(RangeMinMax) -> bool,
        {
            self.segtree.max_right(l, g)
        }

        pub fn min_left<G>(&mut self, r: usize, g: G) -> usize
        where
            G: Fn(RangeMinMax) -> bool,
        {
            self.segtree.min_left(r, g)
        }

        pub fn to_vec(&mut self) -> Vec<i64> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}

#[cfg(test)]
mod test_range_div_ceil_range_min_max {
    use super::range_div_ceil_range_min_max::{RangeDivCeilRangeMinMaxSegtree, RangeMinMax};
    use num_integer::Integer;

    #[test]
    fn test_new_and_get() {
        let rm = RangeMinMax::new(1, 2);
        assert_eq!(rm.min, 1);
        assert_eq!(rm.max, 2);

        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeDivCeilRangeMinMaxSegtree::new(&xs);
        assert_eq!(segtree.get(0), 10);
        assert_eq!(segtree.get(2), 30);
        assert_eq!(segtree.get(4), 50);
    }

    #[test]
    fn test_range_min_max() {
        let xs = vec![10, 50, 30, 40, 20];
        let mut segtree = RangeDivCeilRangeMinMaxSegtree::new(&xs);
        assert_eq!(segtree.range_min(1..4), 30);
        assert_eq!(segtree.range_max(1..4), 50);
        assert_eq!(segtree.range_min(2..5), 20);
        assert_eq!(segtree.range_max(2..5), 40);
    }

    #[test]
    fn test_apply_div_ceil() {
        let xs = vec![10, 20, 30];
        let mut segtree = RangeDivCeilRangeMinMaxSegtree::new(&xs);
        segtree.apply_div_ceil(1, 2); // 20/2 = 10
        assert_eq!(segtree.get(1), 10);
        segtree.apply_div_ceil(2, 4); // 30/4 = 7.5 -> 8
        assert_eq!(segtree.get(2), 8);
    }

    #[test]
    fn test_apply_range_div_ceil() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeDivCeilRangeMinMaxSegtree::new(&xs);
        segtree.apply_range_div_ceil(1..4, 3);
        // 20/3 = 6.66->7, 30/3 = 10, 40/3 = 13.33->14
        // [10, 7, 10, 14, 50]
        assert_eq!(segtree.to_vec(), vec![10, 7, 10, 14, 50]);
        assert_eq!(segtree.range_min(1..4), 7);
        assert_eq!(segtree.range_max(1..4), 14);
    }

    #[test]
    fn test_negative_values() {
        // -5 / 2 = -2.5 -> -2 (ceil)
        let xs = vec![-10, -5, 0, 5, 10];
        let mut segtree = RangeDivCeilRangeMinMaxSegtree::new(&xs);
        segtree.apply_range_div_ceil(0..5, 2);
        // -10/2=-5, -5/2=-2, 0/2=0, 5/2=3, 10/2=5
        assert_eq!(segtree.to_vec(), vec![-5, -2, 0, 3, 5]);
        assert_eq!(segtree.range_min(0..5), -5);
        assert_eq!(segtree.range_max(0..5), 5);
    }

    #[test]
    fn test_composition_overflow() {
        let xs = vec![100, 200];
        let mut segtree = RangeDivCeilRangeMinMaxSegtree::new(&xs);

        // 10^12 * 10^12 will overflow i64.
        // With saturating_mul, it becomes i64::MAX.
        let large_val = 1_000_000_000_000i64; // 10^12

        // Apply large_val twice. Composition will be large_val * large_val -> saturates to i64::MAX
        segtree.apply_range_div_ceil(0..2, large_val);
        segtree.apply_range_div_ceil(0..2, large_val);

        // ceil(100 / i64::MAX) = 1
        assert_eq!(segtree.to_vec(), vec![1, 1]);
    }

    #[test]
    fn test_max_right_min_left() {
        let xs = vec![1, 10, 5, 20, 3];
        let mut segtree = RangeDivCeilRangeMinMaxSegtree::new(&xs);
        // max_right: [0, r) で max が 10 以下の最大の r -> [0, 3) max=10, [0, 4) max=20
        assert_eq!(segtree.max_right(0, |m| m.max <= 10), 3);
        // min_left: [l, 5) で max が 10 以下の最小の l -> [4, 5) max=3, [3, 5) max=20
        assert_eq!(segtree.min_left(5, |m| m.max <= 10), 4);
    }

    #[ignore]
    #[test]
    fn test_random_div_ceil() {
        use rand::{Rng, SeedableRng, rngs::SmallRng};

        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.random_range(1..=20);
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(-1000..=1000)).collect();
            let mut segtree = RangeDivCeilRangeMinMaxSegtree::new(&naive_vec);

            for _ in 0..100 {
                let op_type = rng.random_range(0..5);

                match op_type {
                    0 => {
                        // set(p, x)
                        let p = rng.random_range(0..n);
                        let x = rng.random_range(-1000..=1000);
                        naive_vec[p] = x;
                        segtree.set(p, x);
                    }
                    1 => {
                        // apply_range_div_ceil(range, x)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        if l == r {
                            continue;
                        }
                        // 稀に巨大な数を使ってオーバーフロー(合成の飽和)を誘発させる
                        let x = if rng.random_bool(0.1) {
                            rng.random_range(1..=1_000_000_000_000_000_000)
                        } else {
                            rng.random_range(1..=10)
                        };

                        for i in l..r {
                            naive_vec[i] = Integer::div_ceil(&naive_vec[i], &x);
                        }
                        segtree.apply_range_div_ceil(l..r, x);
                    }
                    2 => {
                        // get(p)
                        let p = rng.random_range(0..n);
                        assert_eq!(segtree.get(p), naive_vec[p]);
                    }
                    3 => {
                        // range_min_max(range)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let expected_max =
                            naive_vec[l..r].iter().copied().max().unwrap_or(i64::MIN);
                        let expected_min =
                            naive_vec[l..r].iter().copied().min().unwrap_or(i64::MAX);
                        assert_eq!(segtree.range_min(l..r), expected_min);
                        assert_eq!(segtree.range_max(l..r), expected_max);
                    }
                    4 => {
                        // all_minmax()
                        let expected_max = naive_vec.iter().copied().max().unwrap_or(i64::MIN);
                        let expected_min = naive_vec.iter().copied().min().unwrap_or(i64::MAX);
                        assert_eq!(segtree.all_min(), expected_min);
                        assert_eq!(segtree.all_max(), expected_max);
                    }
                    _ => unreachable!(),
                }
            }
            assert_eq!(segtree.to_vec(), naive_vec);
        }
    }
}
