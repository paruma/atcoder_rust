use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_linear_add_range_sum::*;")]
pub mod range_linear_add_range_sum {
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
        // sum_sq_idx を持てば2次関数加算もできる
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

    pub struct RangeLinearAddRangeSum<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for RangeLinearAddRangeSum<T>
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T> + From<i64>,
    {
        type M = ValueLenSum<T>;
        type F = Linear<T>;

        fn identity_map() -> Linear<T> {
            Linear {
                intercept: 0.into(),
                slope: 0.into(),
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
                sum: x.sum + f.intercept * x.len.into() + f.slope * x.sum_idx,
                len: x.len,
                sum_idx: x.sum_idx,
            }
        }
    }

    pub struct RangeLinearAddRangeSumSegtree<T>
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T> + From<i64>,
    {
        segtree: LazySegtree<RangeLinearAddRangeSum<T>>,
        len: usize,
    }

    impl<T> RangeLinearAddRangeSumSegtree<T>
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T> + From<i64>,
    {
        pub fn new(xs: &[T]) -> RangeLinearAddRangeSumSegtree<T> {
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
        /// i in l..r に対して、self[i] += init + diff * (l - i) を計算する
        pub fn apply_range_linear_add<R>(&mut self, range: R, init: T, diff: T)
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
            self.segtree.apply_range(range, linear);
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
        let mut segtree = RangeLinearAddRangeSumSegtree::<i64>::new(&xs);
        assert_eq!(segtree.get(0), 10);
        assert_eq!(segtree.get(2), 30);
        assert_eq!(segtree.get(4), 50);
    }

    #[test]
    fn test_set() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeLinearAddRangeSumSegtree::<i64>::new(&xs);
        segtree.set(0, 5);
        assert_eq!(segtree.to_vec(), vec![5, 20, 30, 40, 50]);
        segtree.set(4, 45);
        assert_eq!(segtree.to_vec(), vec![5, 20, 30, 40, 45]);
    }

    #[test]
    fn test_all_sum() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeLinearAddRangeSumSegtree::<i64>::new(&xs);
        assert_eq!(segtree.all_sum(), 150);
        segtree.set(0, 5);
        assert_eq!(segtree.all_sum(), 145);
    }

    #[test]
    fn test_range_sum() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeLinearAddRangeSumSegtree::<i64>::new(&xs);
        assert_eq!(segtree.range_sum(1..4), 90); // 20 + 30 + 40
        segtree.set(2, 15);
        assert_eq!(segtree.range_sum(1..4), 75); // 20 + 15 + 40
    }

    #[test]
    fn test_apply_range_linear_add_with_diff() {
        let xs = vec![0, 0, 0, 0, 0];
        let mut segtree = RangeLinearAddRangeSumSegtree::<i64>::new(&xs);
        // Apply init=10, diff=2 to range 1..4 (indices 1, 2, 3)
        // i=1: 10 + 2 * (1 - 1) = 10
        // i=2: 10 + 2 * (2 - 1) = 12
        // i=3: 10 + 2 * (3 - 1) = 14
        segtree.apply_range_linear_add(1..4, 10, 2);
        assert_eq!(segtree.to_vec(), vec![0, 10, 12, 14, 0]);

        // Apply init=1, diff=1 to range 0..5 (indices 0, 1, 2, 3, 4)
        // i=0: 1 + 1 * (0 - 0) = 1
        // i=1: 1 + 1 * (1 - 0) = 2
        // i=2: 1 + 1 * (2 - 0) = 3
        // i=3: 1 + 1 * (3 - 0) = 4
        // i=4: 1 + 1 * (4 - 0) = 5
        segtree.apply_range_linear_add(0..5, 1, 1);
        assert_eq!(segtree.to_vec(), vec![1, 12, 15, 18, 5]);
    }

    #[test]
    fn test_apply_range_add_with_zero_diff() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeLinearAddRangeSumSegtree::<i64>::new(&xs);
        segtree.apply_range_linear_add(1..4, 5, 0);
        assert_eq!(segtree.to_vec(), vec![10, 25, 35, 45, 50]);
        segtree.apply_range_linear_add(0..3, -10, 0);
        assert_eq!(segtree.to_vec(), vec![0, 15, 25, 45, 50]);
    }

    #[test]
    fn test_to_vec() {
        let xs = vec![0, 1, 2, 3, 4, 5];
        let mut segtree = RangeLinearAddRangeSumSegtree::<i64>::new(&xs);
        assert_eq!(segtree.to_vec(), vec![0, 1, 2, 3, 4, 5]);
        segtree.apply_range_linear_add(1..4, 10, 0);
        assert_eq!(segtree.to_vec(), vec![0, 11, 12, 13, 4, 5]);
    }

    #[test]
    fn test_modint() {
        let xs = vec![Mint::new(1), Mint::new(2), Mint::new(3)];
        let mut segtree = RangeLinearAddRangeSumSegtree::<Mint>::new(&xs);
        segtree.apply_range_linear_add(0..3, Mint::new(1), Mint::new(0));
        assert_eq!(
            segtree.to_vec(),
            vec![Mint::new(2), Mint::new(3), Mint::new(4)]
        );
    }

    #[test]
    #[ignore]
    fn test_random_linear_add() {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            // 100回のテストケース
            let n = rng.gen_range(1..=100);
            let mut xs: Vec<i64> = (0..n).map(|_| rng.gen_range(-100..=100)).collect();
            let mut segtree = RangeLinearAddRangeSumSegtree::<i64>::new(&xs);

            for _ in 0..10 {
                // 各テストケースで10回の操作
                let l = rng.gen_range(0..n);
                let r = rng.gen_range(l + 1..=n);
                let init = rng.gen_range(-50..=50);
                let diff = rng.gen_range(-10..=10);

                // 愚直な更新
                for i in l..r {
                    xs[i] += init + diff * (i as i64 - l as i64);
                }

                // セグメントツリーの更新
                segtree.apply_range_linear_add(l..r, init, diff);

                // 結果の比較
                assert_eq!(
                    segtree.to_vec(),
                    xs,
                    "Failed on range {}:{} with init={}, diff={}",
                    l,
                    r,
                    init,
                    diff
                );
            }
        }
    }
}
