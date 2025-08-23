use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_update_range_sum::*;")]
pub mod range_update_range_sum {
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::marker::PhantomData;
    use std::ops::{Add, Mul, RangeBounds};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeSum<T> {
        pub sum: T,
        pub len: i64,
    }
    impl<T> RangeSum<T> {
        pub fn unit(x: T) -> RangeSum<T> {
            RangeSum { sum: x, len: 1 }
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
            }
        }
        fn binary_operation(a: &RangeSum<T>, b: &RangeSum<T>) -> RangeSum<T> {
            RangeSum {
                sum: a.sum + b.sum,
                len: a.len + b.len,
            }
        }
    }

    pub struct RangeUpdateRangeSum<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for RangeUpdateRangeSum<T>
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + From<i64>,
    {
        type M = ValueLenSum<T>;
        type F = Option<T>; // None means no update, Some(val) means update to val

        fn identity_map() -> Option<T> {
            None
        }
        fn composition(a: &Option<T>, b: &Option<T>) -> Option<T> {
            if a.is_some() {
                *a
            } else {
                *b
            }
        }

        fn mapping(f: &Option<T>, x: &RangeSum<T>) -> RangeSum<T> {
            match f {
                Some(val) => RangeSum {
                    sum: *val * x.len.into(),
                    len: x.len,
                },
                None => *x,
            }
        }
    }

    pub struct RangeUpdateRangeSumSegtree<T>
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + From<i64>,
    {
        segtree: LazySegtree<RangeUpdateRangeSum<T>>,
        len: usize,
    }

    impl<T> RangeUpdateRangeSumSegtree<T>
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + From<i64>,
    {
        pub fn new(xs: &[T]) -> RangeUpdateRangeSumSegtree<T> {
            let xs = xs.iter().copied().map(RangeSum::unit).collect_vec();
            let len = xs.len();
            RangeUpdateRangeSumSegtree {
                segtree: LazySegtree::from(xs),
                len,
            }
        }

        pub fn set(&mut self, p: usize, x: T) {
            self.segtree.set(p, RangeSum::unit(x));
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

        pub fn apply_update(&mut self, p: usize, x: T) {
            self.segtree.apply(p, Some(x))
        }

        pub fn apply_range_update<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, Some(x))
        }

        pub fn to_vec(&mut self) -> Vec<T> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}

#[cfg(test)]
pub mod test_range_update_range_sum {
    use ac_library::ModInt998244353;

    use super::range_update_range_sum::RangeUpdateRangeSumSegtree;

    type Mint = ModInt998244353;

    #[test]
    fn test_new_and_get() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeUpdateRangeSumSegtree::<i64>::new(&xs);
        assert_eq!(segtree.get(0), 10);
        assert_eq!(segtree.get(2), 30);
        assert_eq!(segtree.get(4), 50);
    }

    #[test]
    fn test_set() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeUpdateRangeSumSegtree::<i64>::new(&xs);
        segtree.set(0, 5);
        assert_eq!(segtree.to_vec(), vec![5, 20, 30, 40, 50]);
        segtree.set(4, 45);
        assert_eq!(segtree.to_vec(), vec![5, 20, 30, 40, 45]);
    }

    #[test]
    fn test_all_sum() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeUpdateRangeSumSegtree::<i64>::new(&xs);
        assert_eq!(segtree.all_sum(), 150);
        segtree.set(0, 5);
        assert_eq!(segtree.all_sum(), 145);
    }

    #[test]
    fn test_range_sum() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeUpdateRangeSumSegtree::<i64>::new(&xs);
        assert_eq!(segtree.range_sum(1..4), 90); // 20 + 30 + 40
        segtree.set(2, 15);
        assert_eq!(segtree.range_sum(1..4), 75); // 20 + 15 + 40
    }

    #[test]
    fn test_apply_update() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeUpdateRangeSumSegtree::<i64>::new(&xs);
        segtree.apply_update(1, 5);
        assert_eq!(segtree.to_vec(), vec![10, 5, 30, 40, 50]);
        segtree.apply_update(1, 15);
        assert_eq!(segtree.to_vec(), vec![10, 15, 30, 40, 50]);
    }

    #[test]
    fn test_apply_range_update() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeUpdateRangeSumSegtree::<i64>::new(&xs);
        segtree.apply_range_update(1..4, 5);
        // assert_eq!(segtree.to_vec(), vec![10, 5, 5, 5, 50]);
        segtree.apply_range_update(1..4, 20);
        assert_eq!(segtree.to_vec(), vec![10, 20, 20, 20, 50]);
        segtree.apply_range_update(0..3, 100);
        assert_eq!(segtree.to_vec(), vec![100, 100, 100, 20, 50]);
    }

    #[test]
    fn test_to_vec() {
        let xs = vec![0, 1, 2, 3, 4, 5];
        let mut segtree = RangeUpdateRangeSumSegtree::<i64>::new(&xs);
        assert_eq!(segtree.to_vec(), vec![0, 1, 2, 3, 4, 5]);
        segtree.apply_range_update(1..4, 10);
        assert_eq!(segtree.to_vec(), vec![0, 10, 10, 10, 4, 5]);
    }

    #[test]
    fn test_modint() {
        let xs = vec![Mint::new(1), Mint::new(2), Mint::new(3)];
        let mut segtree = RangeUpdateRangeSumSegtree::<Mint>::new(&xs);
        segtree.apply_range_update(0..3, Mint::new(10));
        assert_eq!(
            segtree.to_vec(),
            vec![Mint::new(10), Mint::new(10), Mint::new(10)]
        );
    }

    #[ignore]
    #[test]
    fn test_random_update() {
        use rand::{rngs::SmallRng, Rng, SeedableRng};

        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.gen_range(1..=20);
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.gen_range(-100..=100)).collect();
            let mut segtree = RangeUpdateRangeSumSegtree::<i64>::new(&naive_vec);

            for _ in 0..100 {
                // 100 random operations per set
                let op_type = rng.gen_range(0..6);

                match op_type {
                    0 => {
                        // set(p, x)
                        let p = rng.gen_range(0..n);
                        let x = rng.gen_range(-100..=100);
                        naive_vec[p] = x;
                        segtree.set(p, x);
                    }
                    1 => {
                        // apply_range_update(range, x)
                        let mut p1 = rng.gen_range(0..=n);
                        let mut p2 = rng.gen_range(0..=n);
                        if p1 > p2 {
                            std::mem::swap(&mut p1, &mut p2);
                        }
                        let l = p1;
                        let r = p2;

                        let x = rng.gen_range(-50..=50);

                        for i in l..r {
                            naive_vec[i] = x;
                        }
                        segtree.apply_range_update(l..r, x);
                    }
                    2 => {
                        // get(p)
                        let p = rng.gen_range(0..n);
                        assert_eq!(segtree.get(p), naive_vec[p], "get({}) failed", p);
                    }
                    3 => {
                        // range_sum(range)
                        let mut p1 = rng.gen_range(0..=n);
                        let mut p2 = rng.gen_range(0..=n);
                        if p1 > p2 {
                            std::mem::swap(&mut p1, &mut p2);
                        }
                        let l = p1;
                        let r = p2;

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
                    5 => {
                        // apply_update(p, x)
                        let p = rng.gen_range(0..n);
                        let x = rng.gen_range(-100..=100);
                        naive_vec[p] = x;
                        segtree.apply_update(p, x);
                    }
                    _ => unreachable!(),
                }
            }

            // Final check
            assert_eq!(segtree.to_vec(), naive_vec, "final to_vec() check failed");
        }
    }
}
