use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_linear_update_range_sum::*;")]
pub mod range_linear_update_range_sum {
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::marker::PhantomData;
    use std::ops::{Add, Div, Mul, RangeBounds, Sub};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeSum<T> {
        pub sum: T,
        pub len: i64,
        pub sum_idx: T,
        // sum_sq_idx を持てば2次関数更新もできる
    }
    impl<T> RangeSum<T>
    where
        T: From<i64>,
    {
        pub fn unit(x: T, idx: i64) -> RangeSum<T> {
            RangeSum {
                sum: x,
                len: 1,
                sum_idx: idx.into(),
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ValueLenSum<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for ValueLenSum<T>
    where
        T: Copy + Add<Output = T> + From<i64>,
    {
        type S = RangeSum<T>;
        fn identity() -> RangeSum<T> {
            RangeSum {
                sum: 0.into(),
                len: 0,
                sum_idx: 0.into(),
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
    pub struct RangeLinearUpdateRangeSum<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for RangeLinearUpdateRangeSum<T>
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T> + From<i64>,
    {
        type M = ValueLenSum<T>;
        type F = Option<Linear<T>>;

        fn identity_map() -> Option<Linear<T>> {
            None
        }

        fn composition(
            f_outer: &Option<Linear<T>>,
            f_inner: &Option<Linear<T>>,
        ) -> Option<Linear<T>> {
            if f_outer.is_some() {
                *f_outer
            } else {
                *f_inner
            }
        }

        fn mapping(f: &Option<Linear<T>>, x: &RangeSum<T>) -> RangeSum<T> {
            if let Some(f) = f {
                // sum(a + bi) = na + b sum i
                RangeSum {
                    sum: f.intercept * x.len.into() + f.slope * x.sum_idx,
                    len: x.len,
                    sum_idx: x.sum_idx,
                }
            } else {
                *x
            }
        }
    }

    #[derive(Clone)]
    pub struct RangeLinearUpdateRangeSumSegtree<T>
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T> + From<i64>,
    {
        segtree: LazySegtree<RangeLinearUpdateRangeSum<T>>,
        len: usize,
    }

    impl<T> RangeLinearUpdateRangeSumSegtree<T>
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T> + From<i64>,
    {
        pub fn new(n: usize) -> Self {
            let xs = vec![0.into(); n];
            Self::from_slice(&xs)
        }

        pub fn from_slice(xs: &[T]) -> RangeLinearUpdateRangeSumSegtree<T> {
            let xs = xs
                .iter()
                .copied()
                .enumerate()
                .map(|(i, x)| RangeSum::unit(x, i as i64))
                .collect_vec();
            let len = xs.len();
            RangeLinearUpdateRangeSumSegtree {
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
        pub fn apply_range_linear_update<R>(&mut self, range: R, init: T, diff: T)
        where
            R: RangeBounds<usize>,
        {
            use std::ops::Bound;
            let l = match range.start_bound() {
                Bound::Included(val) => *val,
                Bound::Excluded(val) => *val + 1,
                Bound::Unbounded => 0,
            };
            let intercept = init - diff * (l as i64).into();
            let linear = Linear {
                intercept,
                slope: diff,
            };
            self.segtree.apply_range(range, Some(linear));
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
pub mod test_range_linear_update_range_sum {
    use ac_library::ModInt998244353;

    use super::range_linear_update_range_sum::RangeLinearUpdateRangeSumSegtree;

    type Mint = ModInt998244353;

    #[test]
    fn test_new_and_get() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeLinearUpdateRangeSumSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.get(0), 10);
        assert_eq!(segtree.get(2), 30);
        assert_eq!(segtree.get(4), 50);
    }

    #[test]
    fn test_set() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeLinearUpdateRangeSumSegtree::<i64>::from_slice(&xs);
        segtree.set(0, 5);
        assert_eq!(segtree.to_vec(), vec![5, 20, 30, 40, 50]);
        segtree.set(4, 45);
        assert_eq!(segtree.to_vec(), vec![5, 20, 30, 40, 45]);
    }

    #[test]
    fn test_all_sum() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeLinearUpdateRangeSumSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.all_sum(), 150);
        segtree.set(0, 5);
        assert_eq!(segtree.all_sum(), 145);
    }

    #[test]
    fn test_range_sum() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeLinearUpdateRangeSumSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.range_sum(1..4), 90); // 20 + 30 + 40
        segtree.set(2, 15);
        assert_eq!(segtree.range_sum(1..4), 75); // 20 + 15 + 40
    }

    #[test]
    fn test_apply_range_linear_update_with_diff() {
        let xs = vec![0, 0, 0, 0, 0];
        let mut segtree = RangeLinearUpdateRangeSumSegtree::<i64>::from_slice(&xs);
        // Apply init=10, diff=2 to range 1..4 (indices 1, 2, 3)
        // i=1: 10 + 2 * (1 - 1) = 10
        // i=2: 10 + 2 * (2 - 1) = 12
        // i=3: 10 + 2 * (3 - 1) = 14
        segtree.apply_range_linear_update(1..4, 10, 2);
        assert_eq!(segtree.to_vec(), vec![0, 10, 12, 14, 0]);

        // Apply init=1, diff=1 to range 0..5 (indices 0, 1, 2, 3, 4)
        // i=0: 1 + 1 * (0 - 0) = 1
        // i=1: 1 + 1 * (1 - 0) = 2
        // i=2: 1 + 1 * (2 - 0) = 3
        // i=3: 1 + 1 * (3 - 0) = 4
        // i=4: 1 + 1 * (4 - 0) = 5
        segtree.apply_range_linear_update(0..5, 1, 1);
        assert_eq!(segtree.to_vec(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_apply_range_update_with_zero_diff() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeLinearUpdateRangeSumSegtree::<i64>::from_slice(&xs);
        segtree.apply_range_linear_update(1..4, 5, 0);
        assert_eq!(segtree.to_vec(), vec![10, 5, 5, 5, 50]);
        segtree.apply_range_linear_update(0..3, -10, 0);
        assert_eq!(segtree.to_vec(), vec![-10, -10, -10, 5, 50]);
    }

    #[test]
    fn test_to_vec() {
        let xs = vec![0, 1, 2, 3, 4, 5];
        let mut segtree = RangeLinearUpdateRangeSumSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.to_vec(), vec![0, 1, 2, 3, 4, 5]);
        segtree.apply_range_linear_update(1..4, 10, 0);
        assert_eq!(segtree.to_vec(), vec![0, 10, 10, 10, 4, 5]);
    }

    #[test]
    fn test_modint() {
        let xs = vec![Mint::new(1), Mint::new(2), Mint::new(3)];
        let mut segtree = RangeLinearUpdateRangeSumSegtree::<Mint>::from_slice(&xs);
        segtree.apply_range_linear_update(0..3, Mint::new(1), Mint::new(0));
        assert_eq!(
            segtree.to_vec(),
            vec![Mint::new(1), Mint::new(1), Mint::new(1)]
        );
    }

    #[test]
    fn test_max_right_min_left() {
        let xs = vec![1, 1, 1, 1, 1];
        let mut segtree = RangeLinearUpdateRangeSumSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.max_right(0, |s| s <= 3), 3);
        assert_eq!(segtree.min_left(5, |s| s <= 3), 2);
    }

    #[ignore]
    #[test]
    fn test_random_linear_update() {
        use rand::{Rng, SeedableRng, rngs::SmallRng};

        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.random_range(1..=20);
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
            let mut segtree = RangeLinearUpdateRangeSumSegtree::<i64>::from_slice(&naive_vec);

            for _ in 0..100 {
                // 100 random operations per set
                let op_type = rng.random_range(0..5);

                match op_type {
                    0 => {
                        // set(p, x)
                        let p = rng.random_range(0..n);
                        let x = rng.random_range(-100..=100);
                        naive_vec[p] = x;
                        segtree.set(p, x);
                    }
                    1 => {
                        // apply_range_linear_update(range, init, diff)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);

                        let init = rng.random_range(-50..=50);
                        let diff = rng.random_range(-10..=10);

                        for i in l..r {
                            naive_vec[i] = init + diff * (i as i64 - l as i64);
                        }
                        segtree.apply_range_linear_update(l..r, init, diff);
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
