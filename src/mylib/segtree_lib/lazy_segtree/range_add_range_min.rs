use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_add_range_min::*;")]
pub mod range_add_range_min {
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::marker::PhantomData;
    use std::ops::{Add, RangeBounds};

    // Range minimum query monoid
    pub struct RangeMin<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for RangeMin<T>
    where
        T: Copy + Ord + Bounded,
    {
        type S = T;
        fn identity() -> T {
            T::max_value()
        }
        fn binary_operation(a: &T, b: &T) -> T {
            *a.min(b)
        }
    }

    // Action for RangeAddRangeMin
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct AddAction<T> {
        add_val: T,
    }

    impl<T> AddAction<T>
    where
        T: Copy + From<i64>,
    {
        pub fn new(val: T) -> Self {
            Self { add_val: val }
        }
    }

    pub struct RangeAddRangeMin<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for RangeAddRangeMin<T>
    where
        T: Copy + Ord + From<i64> + Bounded + Add<Output = T>,
    {
        type M = RangeMin<T>;
        type F = AddAction<T>;

        fn identity_map() -> Self::F {
            AddAction {
                add_val: T::from(0),
            }
        }

        // f: new action, g: old action
        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            AddAction {
                add_val: g.add_val + f.add_val,
            }
        }

        fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
            if *x == T::max_value() {
                T::max_value()
            } else {
                *x + f.add_val
            }
        }
    }

    pub trait Bounded {
        fn min_value() -> Self;
        fn max_value() -> Self;
    }

    impl Bounded for i64 {
        fn min_value() -> Self {
            i64::MIN
        }
        fn max_value() -> Self {
            i64::MAX
        }
    }

    pub struct RangeAddRangeMinSegtree<T>
    where
        T: Copy + Ord + From<i64> + Bounded + Add<Output = T>,
    {
        segtree: LazySegtree<RangeAddRangeMin<T>>,
        len: usize,
    }

    impl<T> RangeAddRangeMinSegtree<T>
    where
        T: Copy + Ord + From<i64> + Bounded + Add<Output = T>,
    {
        pub fn new(xs: &[T]) -> Self {
            let len = xs.len();
            Self {
                segtree: LazySegtree::from(xs.to_vec()),
                len,
            }
        }

        pub fn set(&mut self, p: usize, x: T) {
            self.segtree.set(p, x);
        }

        pub fn get(&mut self, p: usize) -> T {
            self.segtree.get(p)
        }

        pub fn range_min<R>(&mut self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range)
        }

        pub fn all_min(&self) -> T {
            self.segtree.all_prod()
        }

        pub fn apply_range_add<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, AddAction::new(x))
        }

        pub fn to_vec(&mut self) -> Vec<T> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}

#[cfg(test)]
pub mod test_range_add_range_min {
    use super::range_add_range_min::RangeAddRangeMinSegtree;

    #[test]
    fn test_new_and_get() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeAddRangeMinSegtree::<i64>::new(&xs);
        assert_eq!(segtree.get(0), 10);
        assert_eq!(segtree.get(2), 30);
        assert_eq!(segtree.get(4), 50);
    }

    #[test]
    fn test_range_min() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeAddRangeMinSegtree::<i64>::new(&xs);
        assert_eq!(segtree.range_min(1..4), 20);
    }

    #[test]
    fn test_apply_range_add() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeAddRangeMinSegtree::<i64>::new(&xs);
        segtree.apply_range_add(1..4, 5);
        assert_eq!(segtree.to_vec(), vec![10, 25, 35, 45, 50]);
        assert_eq!(segtree.range_min(1..4), 25);
    }

    #[test]
    fn test_to_vec() {
        let xs = vec![0, 1, 2, 3, 4, 5];
        let mut segtree = RangeAddRangeMinSegtree::<i64>::new(&xs);
        assert_eq!(segtree.to_vec(), vec![0, 1, 2, 3, 4, 5]);
        segtree.apply_range_add(1..4, 10);
        assert_eq!(segtree.to_vec(), vec![0, 11, 12, 13, 4, 5]);
    }

    #[test]
    fn test_all_min() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeAddRangeMinSegtree::<i64>::new(&xs);
        assert_eq!(segtree.all_min(), 10);
        segtree.apply_range_add(0..5, -5);
        assert_eq!(segtree.all_min(), 5);
    }

    #[test]
    fn test_identity_mapping() {
        let xs: Vec<i64> = vec![];
        let mut segtree = RangeAddRangeMinSegtree::<i64>::new(&xs);
        assert_eq!(segtree.all_min(), i64::MAX);
        segtree.apply_range_add(0..0, 100);
        assert_eq!(segtree.all_min(), i64::MAX);
    }

    #[ignore]
    #[test]
    fn test_random_add_min() {
        use rand::{rngs::SmallRng, Rng, SeedableRng};

        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.gen_range(1..=20);
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.gen_range(-100..=100)).collect();
            let mut segtree = RangeAddRangeMinSegtree::<i64>::new(&naive_vec);

            for _ in 0..100 {
                // 100 random operations per set
                let op_type = rng.gen_range(0..5);

                match op_type {
                    0 => {
                        // set(p, x)
                        let p = rng.gen_range(0..n);
                        let x = rng.gen_range(-100..=100);
                        naive_vec[p] = x;
                        segtree.set(p, x);
                    }
                    1 => {
                        // apply_range_add(range, x)
                        let l = rng.gen_range(0..=n);
                        let r = rng.gen_range(l..=n);

                        let x = rng.gen_range(-50..=50);

                        for i in l..r {
                            naive_vec[i] += x;
                        }
                        segtree.apply_range_add(l..r, x);
                    }
                    2 => {
                        // get(p)
                        let p = rng.gen_range(0..n);
                        assert_eq!(segtree.get(p), naive_vec[p], "get({}) failed", p);
                    }
                    3 => {
                        // range_min(range)
                        let l = rng.gen_range(0..=n);
                        let r = rng.gen_range(l..=n);

                        let expected_min =
                            naive_vec[l..r].iter().copied().min().unwrap_or(i64::MAX);
                        assert_eq!(
                            segtree.range_min(l..r),
                            expected_min,
                            "range_min({}..{}) failed",
                            l,
                            r
                        );
                    }
                    4 => {
                        // all_min()
                        let expected_min = naive_vec.iter().copied().min().unwrap_or(i64::MAX);
                        assert_eq!(segtree.all_min(), expected_min, "all_min() failed");
                    }
                    _ => unreachable!(),
                }
            }

            // Final check
            assert_eq!(segtree.to_vec(), naive_vec, "final to_vec() check failed");
        }
    }
}
