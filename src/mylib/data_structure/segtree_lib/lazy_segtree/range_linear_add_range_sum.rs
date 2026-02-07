use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_linear_add_range_sum::*;")]
pub mod range_linear_add_range_sum {
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::iter::{Product, Sum};
    use std::marker::PhantomData;
    use std::ops::{Add, Div, Mul, RangeBounds, Sub};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeSum<T> {
        pub sum: T,
        pub len: i64,
        pub sum_idx: T,
        // sum_sq_idx を持てば2次関数加算もできる
    }
    impl<T> RangeSum<T>
    where
        T: Copy + Mul<i64, Output = T> + Product,
    {
        pub fn unit(x: T, idx: i64) -> RangeSum<T> {
            RangeSum {
                sum: x,
                len: 1,
                sum_idx: std::iter::empty::<T>().product::<T>() * idx,
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ValueLenSum<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for ValueLenSum<T>
    where
        T: Copy + Add<Output = T> + Sum,
    {
        type S = RangeSum<T>;
        fn identity() -> RangeSum<T> {
            RangeSum {
                sum: std::iter::empty::<T>().sum(),
                len: 0,
                sum_idx: std::iter::empty::<T>().sum(),
            }
        }
        fn binary_operation(a: &RangeSum<T>, b: &RangeSum<T>) -> RangeSum<T> {
            RangeSum {
                sum: a.sum + b.sum,
                len: a.len + b.len,
                sum_idx: a.sum_idx + b.sum_idx,
            }
        }
    }

    // x ↦ x + intercept + slope * i, 等差数列
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Linear<T> {
        intercept: T,
        slope: T,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeLinearAddRangeSum<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for RangeLinearAddRangeSum<T>
    where
        T: Copy
            + Add<Output = T>
            + Mul<Output = T>
            + Mul<i64, Output = T>
            + Div<Output = T>
            + Sub<Output = T>
            + Sum
            + Product,
    {
        type M = ValueLenSum<T>;
        type F = Linear<T>;

        fn identity_map() -> Linear<T> {
            Linear {
                intercept: std::iter::empty::<T>().sum(),
                slope: std::iter::empty::<T>().sum(),
            }
        }

        // 2つの１次関数加算を合成
        fn composition(f_outer: &Linear<T>, f_inner: &Linear<T>) -> Linear<T> {
            // x ↦ (x + c + di) + a + bi = x + (a + c) + (b + d)i
            Linear {
                intercept: f_outer.intercept + f_inner.intercept,
                slope: f_outer.slope + f_inner.slope,
            }
        }

        fn mapping(f: &Linear<T>, x: &RangeSum<T>) -> RangeSum<T> {
            // sum(xs[i] + a + bi) = sum xs[i] + na + b sum i
            RangeSum {
                sum: x.sum + f.intercept * x.len + f.slope * x.sum_idx,
                len: x.len,
                sum_idx: x.sum_idx,
            }
        }
    }

    #[derive(Clone)]
    pub struct RangeLinearAddRangeSumSegtree<T>
    where
        T: Copy
            + Add<Output = T>
            + Mul<Output = T>
            + Mul<i64, Output = T>
            + Div<Output = T>
            + Sub<Output = T>
            + Sum
            + Product,
    {
        segtree: LazySegtree<RangeLinearAddRangeSum<T>>,
        len: usize,
    }

    impl<T> RangeLinearAddRangeSumSegtree<T>
    where
        T: Copy
            + Add<Output = T>
            + Mul<Output = T>
            + Mul<i64, Output = T>
            + Div<Output = T>
            + Sub<Output = T>
            + Sum
            + Product,
    {
        pub fn new(n: usize) -> Self {
            let xs = vec![std::iter::empty::<T>().sum(); n];
            Self::from_slice(&xs)
        }

        pub fn from_slice(xs: &[T]) -> RangeLinearAddRangeSumSegtree<T> {
            let xs = xs
                .iter()
                .copied()
                .enumerate()
                .map(|(i, x)| RangeSum::unit(x, i as i64))
                .collect_vec();
            let len = xs.len();
            RangeLinearAddRangeSumSegtree {
                segtree: LazySegtree::from(xs),
                len,
            }
        }

        pub fn set(&mut self, p: usize, x: T) {
            self.segtree.set(p, RangeSum::unit(x, p as i64));
        }

        pub fn get(&mut self, p: usize) -> T {
            self.segtree.get(p).sum
        }

        pub fn range_sum<R>(&mut self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).sum
        }

        pub fn all_sum(&self) -> T {
            self.segtree.all_prod().sum
        }

        /// range が l..r であるとする。
        /// `i` in `l..r` に対して、`self[i] += init + diff * (i - l)` を計算する
        pub fn range_linear_add<R>(&mut self, range: R, init: T, diff: T)
        where
            R: RangeBounds<usize>,
        {
            use std::ops::Bound;
            let l = match range.start_bound() {
                Bound::Included(val) => *val,
                Bound::Excluded(val) => *val + 1,
                Bound::Unbounded => 0,
            };
            let intercept = init - diff * (l as i64);
            let linear = Linear {
                intercept,
                slope: diff,
            };
            self.segtree.apply_range(range, linear);
        }

        /// 左端 `l` を固定し、区間 `[l, r)` での総和が述語 `g` を満たすような最大の `r` を返します。
        ///
        /// # 計算量
        /// O(log N)
        pub fn max_right<G>(&mut self, l: usize, g: G) -> usize
        where
            G: Fn(T) -> bool,
        {
            self.segtree.max_right(l, |x| g(x.sum))
        }

        /// 右端 `r` を固定し、区間 `[l, r)` での総和が述語 `g` を満たすような最小の `l` を返します。
        ///
        /// # 計算量
        /// O(log N)
        pub fn min_left<G>(&mut self, r: usize, g: G) -> usize
        where
            G: Fn(T) -> bool,
        {
            self.segtree.min_left(r, |x| g(x.sum))
        }

        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            self.len
        }
        pub fn to_vec(&mut self) -> Vec<T> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}

#[cfg(test)]
pub mod test_range_linear_add_range_sum {
    use ac_library::ModInt998244353;

    use super::range_linear_add_range_sum::RangeLinearAddRangeSumSegtree;

    type Mint = ModInt998244353;

    #[test]
    fn test_new_and_get() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeLinearAddRangeSumSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.get(0), 10);
        assert_eq!(segtree.get(2), 30);
        assert_eq!(segtree.get(4), 50);
    }

    #[test]
    fn test_set() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeLinearAddRangeSumSegtree::<i64>::from_slice(&xs);
        segtree.set(0, 5);
        assert_eq!(segtree.to_vec(), vec![5, 20, 30, 40, 50]);
        segtree.set(4, 45);
        assert_eq!(segtree.to_vec(), vec![5, 20, 30, 40, 45]);
    }

    #[test]
    fn test_all_sum() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeLinearAddRangeSumSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.all_sum(), 150);
        segtree.set(0, 5);
        assert_eq!(segtree.all_sum(), 145);
    }

    #[test]
    fn test_range_sum() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeLinearAddRangeSumSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.range_sum(1..4), 90); // 20 + 30 + 40
        segtree.set(2, 15);
        assert_eq!(segtree.range_sum(1..4), 75); // 20 + 15 + 40
    }

    #[test]
    fn test_range_linear_add_with_diff() {
        let xs = vec![0, 0, 0, 0, 0];
        let mut segtree = RangeLinearAddRangeSumSegtree::<i64>::from_slice(&xs);
        // Apply init=10, diff=2 to range 1..4 (indices 1, 2, 3)
        // i=1: 10 + 2 * (1 - 1) = 10
        // i=2: 10 + 2 * (2 - 1) = 12
        // i=3: 10 + 2 * (3 - 1) = 14
        segtree.range_linear_add(1..4, 10, 2);
        assert_eq!(segtree.to_vec(), vec![0, 10, 12, 14, 0]);

        // Apply init=1, diff=1 to range 0..5 (indices 0, 1, 2, 3, 4)
        // i=0: 1 + 1 * (0 - 0) = 1
        // i=1: 1 + 1 * (1 - 0) = 2
        // i=2: 1 + 1 * (2 - 0) = 3
        // i=3: 1 + 1 * (3 - 0) = 4
        // i=4: 1 + 1 * (4 - 0) = 5
        segtree.range_linear_add(0..5, 1, 1);
        assert_eq!(segtree.to_vec(), vec![1, 12, 15, 18, 5]);
    }

    #[test]
    fn test_range_add_with_zero_diff() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeLinearAddRangeSumSegtree::<i64>::from_slice(&xs);
        segtree.range_linear_add(1..4, 5, 0);
        assert_eq!(segtree.to_vec(), vec![10, 25, 35, 45, 50]);
        segtree.range_linear_add(0..3, -10, 0);
        assert_eq!(segtree.to_vec(), vec![0, 15, 25, 45, 50]);
    }

    #[test]
    fn test_to_vec() {
        let xs = vec![0, 1, 2, 3, 4, 5];
        let mut segtree = RangeLinearAddRangeSumSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.to_vec(), vec![0, 1, 2, 3, 4, 5]);
        segtree.range_linear_add(1..4, 10, 0);
        assert_eq!(segtree.to_vec(), vec![0, 11, 12, 13, 4, 5]);
    }

    #[test]
    fn test_modint() {
        let xs = vec![Mint::new(1), Mint::new(2), Mint::new(3)];
        let mut segtree = RangeLinearAddRangeSumSegtree::<Mint>::from_slice(&xs);
        segtree.range_linear_add(0..3, Mint::new(1), Mint::new(0));
        assert_eq!(
            segtree.to_vec(),
            vec![Mint::new(2), Mint::new(3), Mint::new(4)]
        );
    }

    #[test]
    fn test_max_right_min_left() {
        let xs = vec![1, 1, 1, 1, 1];
        let mut segtree = RangeLinearAddRangeSumSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.max_right(0, |s| s <= 3), 3);
        assert_eq!(segtree.min_left(5, |s| s <= 3), 2);
    }

    #[ignore]
    #[test]
    fn test_random_linear_add() {
        use rand::{Rng, SeedableRng, rngs::SmallRng};

        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.random_range(1..=20);
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
            let mut segtree = RangeLinearAddRangeSumSegtree::<i64>::from_slice(&naive_vec);

            for _ in 0..100 {
                // 100 random operations per set
                let op_type = rng.random_range(0..5); // 5 operations

                match op_type {
                    0 => {
                        // set(p, x)
                        let p = rng.random_range(0..n);
                        let x = rng.random_range(-100..=100);
                        naive_vec[p] = x;
                        segtree.set(p, x);
                    }
                    1 => {
                        // range_linear_add(range, init, diff)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);

                        let init = rng.random_range(-50..=50);
                        let diff = rng.random_range(-10..=10);

                        for i in l..r {
                            naive_vec[i] += init + diff * (i as i64 - l as i64);
                        }
                        segtree.range_linear_add(l..r, init, diff);
                    }
                    2 => {
                        // get(p)
                        let p = rng.random_range(0..n);
                        assert_eq!(segtree.get(p), naive_vec[p], "get({}) failed", p);
                    }
                    3 => {
                        // range_sum(range)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);

                        let expected_sum: i64 = naive_vec[l..r].iter().sum();
                        assert_eq!(
                            segtree.range_sum(l..r),
                            expected_sum,
                            "range_sum({}..{}) failed",
                            l,
                            r
                        );
                    }
                    4 => {
                        // all_sum()
                        let expected_sum: i64 = naive_vec.iter().sum();
                        assert_eq!(segtree.all_sum(), expected_sum, "all_sum() failed");
                    }
                    _ => unreachable!(),
                }
            }

            // Final check
            assert_eq!(segtree.to_vec(), naive_vec, "final to_vec() check failed");
        }
    }
}
