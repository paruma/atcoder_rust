use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_quadratic_add_range_sum::*;")]
pub mod range_quadratic_add_range_sum {
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
        pub sum_sq_idx: T,
    }
    impl<T> RangeSum<T>
    where
        T: From<i64> + Copy + Mul<Output = T>,
    {
        pub fn unit(x: T, idx: i64) -> RangeSum<T> {
            let idx: T = idx.into();
            RangeSum {
                sum: x,
                len: 1,
                sum_idx: idx,
                sum_sq_idx: idx * idx,
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
                sum_sq_idx: 0.into(),
            }
        }
        fn binary_operation(a: &RangeSum<T>, b: &RangeSum<T>) -> RangeSum<T> {
            RangeSum {
                sum: a.sum + b.sum,
                len: a.len + b.len,
                sum_idx: a.sum_idx + b.sum_idx,
                sum_sq_idx: a.sum_sq_idx + b.sum_sq_idx,
            }
        }
    }

    // x ↦ x + coef0 + coef1 * i + coef2 * i * i
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Quadratic<T> {
        coef0: T,
        coef1: T,
        coef2: T,
    }

    pub struct RangeQuadraticAddRangeSum<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for RangeQuadraticAddRangeSum<T>
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T> + From<i64>,
    {
        type M = ValueLenSum<T>;
        type F = Quadratic<T>;

        fn identity_map() -> Quadratic<T> {
            Quadratic {
                coef0: 0.into(),
                coef1: 0.into(),
                coef2: 0.into(),
            }
        }

        // 2つの2次関数加算を合成
        fn composition(f_outer: &Quadratic<T>, f_inner: &Quadratic<T>) -> Quadratic<T> {
            // x ↦ (x + a_0 + a_1 i + a_2 i^2) + b_0 + b_1 i + b_2 i^2
            //    = x + (a_0 + b_0) + (a_1 + b_1) i + (a_2 + b_2) i^2
            Quadratic {
                coef0: f_outer.coef0 + f_inner.coef0,
                coef1: f_outer.coef1 + f_inner.coef1,
                coef2: f_outer.coef2 + f_inner.coef2,
            }
        }

        fn mapping(f: &Quadratic<T>, x: &RangeSum<T>) -> RangeSum<T> {
            // sum(xs[i] + a_0 + a_1 i + a_2 i^2) = sum xs[i] + a_0 n + a_1 sum i + a_2 sum i^2
            RangeSum {
                sum: x.sum + f.coef0 * x.len.into() + f.coef1 * x.sum_idx + f.coef2 * x.sum_sq_idx,
                len: x.len,
                sum_idx: x.sum_idx,
                sum_sq_idx: x.sum_sq_idx,
            }
        }
    }

    pub struct RangeQuadraticAddRangeSumSegtree<T>
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T> + From<i64>,
    {
        segtree: LazySegtree<RangeQuadraticAddRangeSum<T>>,
        len: usize,
    }

    impl<T> RangeQuadraticAddRangeSumSegtree<T>
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T> + From<i64>,
    {
        pub fn new(xs: &[T]) -> RangeQuadraticAddRangeSumSegtree<T> {
            let xs = xs
                .iter()
                .copied()
                .enumerate()
                .map(|(i, x)| RangeSum::unit(x, i as i64))
                .collect_vec();
            let len = xs.len();
            RangeQuadraticAddRangeSumSegtree {
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
        /// i in l..r に対して、self[i] += coef0 + coef1 * (i - l) + coef2 * (i - l)^2を計算する
        pub fn apply_range_quadratic_add<R>(&mut self, range: R, coef0: T, coef1: T, coef2: T)
        where
            R: RangeBounds<usize>,
        {
            use std::ops::Bound;
            let l = match range.start_bound() {
                Bound::Included(val) => *val,
                Bound::Excluded(val) => *val + 1,
                Bound::Unbounded => 0,
            };
            let l: T = (l as i64).into();
            let new_coef0 = coef0 - coef1 * l + coef2 * l * l;
            let new_coef1 = coef1 - coef2 * l * 2.into();
            let new_coef2 = coef2;
            let quad = Quadratic {
                coef0: new_coef0,
                coef1: new_coef1,
                coef2: new_coef2,
            };
            self.segtree.apply_range(range, quad);
        }

        pub fn to_vec(&mut self) -> Vec<T> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}

#[cfg(test)]
pub mod test_range_quadratic_add_range_sum {
    use ac_library::ModInt998244353;

    use super::range_quadratic_add_range_sum::RangeQuadraticAddRangeSumSegtree;

    type Mint = ModInt998244353;

    #[test]
    fn test_new_and_get() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeQuadraticAddRangeSumSegtree::<i64>::new(&xs);
        assert_eq!(segtree.get(0), 10);
        assert_eq!(segtree.get(2), 30);
        assert_eq!(segtree.get(4), 50);
    }

    #[test]
    fn test_set() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeQuadraticAddRangeSumSegtree::<i64>::new(&xs);
        segtree.set(0, 5);
        assert_eq!(segtree.to_vec(), vec![5, 20, 30, 40, 50]);
        segtree.set(4, 45);
        assert_eq!(segtree.to_vec(), vec![5, 20, 30, 40, 45]);
    }

    #[test]
    fn test_all_sum() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeQuadraticAddRangeSumSegtree::<i64>::new(&xs);
        assert_eq!(segtree.all_sum(), 150);
        segtree.set(0, 5);
        assert_eq!(segtree.all_sum(), 145);
    }

    #[test]
    fn test_range_sum() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeQuadraticAddRangeSumSegtree::<i64>::new(&xs);
        assert_eq!(segtree.range_sum(1..4), 90); // 20 + 30 + 40
        segtree.set(2, 15);
        assert_eq!(segtree.range_sum(1..4), 75); // 20 + 15 + 40
    }

    #[test]
    fn test_apply_range_linear_add_normal() {
        let xs = vec![0, 0, 0, 0, 0];
        let mut segtree = RangeQuadraticAddRangeSumSegtree::<i64>::new(&xs);
        // Apply init=10, diff=2 to range 1..4 (indices 1, 2, 3)
        // i=1: 10 + 2 * (1 - 1) = 10
        // i=2: 10 + 2 * (2 - 1) = 12
        // i=3: 10 + 2 * (3 - 1) = 14
        segtree.apply_range_quadratic_add(1..4, 10, 2, 0);
        assert_eq!(segtree.to_vec(), vec![0, 10, 12, 14, 0]);

        // Apply init=1, diff=1 to range 0..5 (indices 0, 1, 2, 3, 4)
        // i=0: 1 + 1 * (0 - 0) = 1
        // i=1: 1 + 1 * (1 - 0) = 2
        // i=2: 1 + 1 * (2 - 0) = 3
        // i=3: 1 + 1 * (3 - 0) = 4
        // i=4: 1 + 1 * (4 - 0) = 5
        segtree.apply_range_quadratic_add(0..5, 1, 1, 0);
        assert_eq!(segtree.to_vec(), vec![1, 12, 15, 18, 5]);
    }

    #[test]
    fn test_apply_range_add_with_const_add() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeQuadraticAddRangeSumSegtree::<i64>::new(&xs);
        segtree.apply_range_quadratic_add(1..4, 5, 0, 0);
        assert_eq!(segtree.to_vec(), vec![10, 25, 35, 45, 50]);
        segtree.apply_range_quadratic_add(0..3, -10, 0, 0);
        assert_eq!(segtree.to_vec(), vec![0, 15, 25, 45, 50]);
    }

    #[test]
    fn test_to_vec() {
        let xs = vec![0, 1, 2, 3, 4, 5];
        let mut segtree = RangeQuadraticAddRangeSumSegtree::<i64>::new(&xs);
        assert_eq!(segtree.to_vec(), vec![0, 1, 2, 3, 4, 5]);
        segtree.apply_range_quadratic_add(1..4, 10, 0, 0);
        assert_eq!(segtree.to_vec(), vec![0, 11, 12, 13, 4, 5]);
    }

    #[test]
    fn test_modint() {
        let xs = vec![Mint::new(1), Mint::new(2), Mint::new(3)];
        let mut segtree = RangeQuadraticAddRangeSumSegtree::<Mint>::new(&xs);
        segtree.apply_range_quadratic_add(0..3, Mint::new(1), Mint::new(0), Mint::new(0));
        assert_eq!(
            segtree.to_vec(),
            vec![Mint::new(2), Mint::new(3), Mint::new(4)]
        );
    }

    #[test]
    fn test_apply_range_quadratic_add() {
        let xs = vec![0, 0, 0, 0, 0];
        let mut segtree = RangeQuadraticAddRangeSumSegtree::<i64>::new(&xs);
        // Apply coef0=1, coef1=1, coef2=1 to range 0..5
        // i=0: 0 + 1 + 1*0 + 1*0^2 = 1
        // i=1: 0 + 1 + 1*1 + 1*1^2 = 3
        // i=2: 0 + 1 + 1*2 + 1*2^2 = 7
        // i=3: 0 + 1 + 1*3 + 1*3^2 = 13
        // i=4: 0 + 1 + 1*4 + 1*4^2 = 21
        segtree.apply_range_quadratic_add(0..5, 1, 1, 1);
        assert_eq!(segtree.to_vec(), vec![1, 3, 7, 13, 21]);

        // Apply coef0=10, coef1=-2, coef2=1 to range 1..4 (indices 1, 2, 3)
        // Current: [1, 3, 7, 13, 21]
        // i=1: 3 + 10 + (-2)*(1-1) + 1*(1-1)^2 = 13
        // i=2: 7 + 10 + (-2)*(2-1) + 1*(2-1)^2 = 7 + 10 - 2 + 1 = 16
        // i=3: 13 + 10 + (-2)*(3-1) + 1*(3-1)^2 = 13 + 10 - 4 + 4 = 23
        segtree.apply_range_quadratic_add(1..4, 10, -2, 1);
        assert_eq!(segtree.to_vec(), vec![1, 13, 16, 23, 21]);
    }

    #[ignore]
    #[test]
    fn test_random_quadratic_add() {
        use rand::{rngs::SmallRng, Rng, SeedableRng};

        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.gen_range(1..=20);
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.gen_range(-100..=100)).collect();
            let mut segtree = RangeQuadraticAddRangeSumSegtree::<i64>::new(&naive_vec);

            for _ in 0..100 {
                // 100 random operations per set
                let op_type = rng.gen_range(0..5); // 5 operations

                match op_type {
                    0 => {
                        // set(p, x)
                        let p = rng.gen_range(0..n);
                        let x = rng.gen_range(-100..=100);
                        naive_vec[p] = x;
                        segtree.set(p, x);
                    }
                    1 => {
                        // apply_range_quadratic_add(range, coef0, coef1, coef2)
                        let l = rng.gen_range(0..=n);
                        let r = rng.gen_range(l..=n);

                        let coef0 = rng.gen_range(-50..=50);
                        let coef1 = rng.gen_range(-10..=10);
                        let coef2 = rng.gen_range(-5..=5);

                        for i in l..r {
                            let diff_idx = i as i64 - l as i64;
                            naive_vec[i] += coef0 + coef1 * diff_idx + coef2 * diff_idx * diff_idx;
                        }
                        segtree.apply_range_quadratic_add(l..r, coef0, coef1, coef2);
                    }
                    2 => {
                        // get(p)
                        let p = rng.gen_range(0..n);
                        assert_eq!(segtree.get(p), naive_vec[p], "get({}) failed", p);
                    }
                    3 => {
                        // range_sum(range)
                        let l = rng.gen_range(0..=n);
                        let r = rng.gen_range(l..=n);

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
