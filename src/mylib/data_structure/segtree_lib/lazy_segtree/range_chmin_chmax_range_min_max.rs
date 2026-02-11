use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_chmin_chmax_range_min_max::*;")]
pub mod range_chmin_chmax_range_min_max {
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::iter::{Product, Sum};
    use std::marker::PhantomData;
    use std::ops::{Add, Mul, RangeBounds};

    fn zero<T: Sum>() -> T {
        std::iter::empty::<T>().sum()
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeMinMax<T> {
        pub min: T,
        pub max: T,
    }

    impl<T> RangeMinMax<T> {
        pub fn new(min: T, max: T) -> Self {
            Self { min, max }
        }

        pub fn unit(x: T) -> Self
        where
            T: Copy,
        {
            Self { min: x, max: x }
        }
    }

    // Range min/max query monoid
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeMinMaxMonoid<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for RangeMinMaxMonoid<T>
    where
        T: Copy + Ord + Bounded,
    {
        type S = RangeMinMax<T>;
        fn identity() -> Self::S {
            RangeMinMax {
                min: T::max_value(),
                max: T::min_value(),
            }
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            RangeMinMax {
                min: a.min.min(b.min),
                max: a.max.max(b.max),
            }
        }
    }

    // Action for RangeChminChmaxRangeMinMax
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ChminChmaxAction<T> {
        chmin_val: T,
        chmax_val: T,
    }

    impl<T> ChminChmaxAction<T>
    where
        T: Copy + Ord + Bounded,
    {
        pub fn new_chmin(val: T) -> Self {
            Self {
                chmin_val: val,
                chmax_val: T::min_value(),
            }
        }

        pub fn new_chmax(val: T) -> Self {
            Self {
                chmin_val: T::max_value(),
                chmax_val: val,
            }
        }

        pub fn new_update(val: T) -> Self {
            Self {
                chmin_val: val,
                chmax_val: val,
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeChminChmaxRangeMinMax<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for RangeChminChmaxRangeMinMax<T>
    where
        T: Copy + Ord + Bounded + Add<Output = T> + Mul<Output = T> + Sum + Product,
    {
        type M = RangeMinMaxMonoid<T>;
        type F = ChminChmaxAction<T>;

        fn identity_map() -> Self::F {
            ChminChmaxAction {
                chmin_val: T::max_value(),
                chmax_val: T::min_value(),
            }
        }

        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            let res_chmin = g.chmin_val.clamp(f.chmax_val, f.chmin_val);
            let res_chmax = g.chmax_val.clamp(f.chmax_val, f.chmin_val);

            ChminChmaxAction {
                chmin_val: res_chmin,
                chmax_val: res_chmax,
            }
        }

        fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
            RangeMinMax {
                min: x.min.clamp(f.chmax_val, f.chmin_val),
                max: x.max.clamp(f.chmax_val, f.chmin_val),
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

    #[derive(Clone)]
    pub struct RangeChminChmaxRangeMinMaxSegtree<T>
    where
        T: Copy + Ord + Bounded + Add<Output = T> + Mul<Output = T> + Sum + Product,
    {
        segtree: LazySegtree<RangeChminChmaxRangeMinMax<T>>,
        len: usize,
    }

    impl<T> RangeChminChmaxRangeMinMaxSegtree<T>
    where
        T: Copy + Ord + Bounded + Add<Output = T> + Mul<Output = T> + Sum + Product,
    {
        pub fn new(n: usize) -> Self {
            let xs = vec![zero(); n];
            Self::from_slice(&xs)
        }

        pub fn from_slice(xs: &[T]) -> RangeChminChmaxRangeMinMaxSegtree<T> {
            let len = xs.len();
            let vec = xs.iter().map(|&x| RangeMinMax::unit(x)).collect::<Vec<_>>();
            RangeChminChmaxRangeMinMaxSegtree {
                segtree: LazySegtree::from(vec),
                len,
            }
        }

        pub fn set(&mut self, p: usize, x: T) {
            self.segtree.set(p, RangeMinMax::unit(x));
        }

        pub fn get(&mut self, p: usize) -> T {
            self.segtree.get(p).min
        }

        pub fn range_minmax<R>(&mut self, range: R) -> RangeMinMax<T>
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range)
        }

        pub fn range_min<R>(&mut self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).min
        }

        pub fn range_max<R>(&mut self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).max
        }

        pub fn all_minmax(&self) -> RangeMinMax<T> {
            self.segtree.all_prod()
        }

        pub fn all_min(&self) -> T {
            self.segtree.all_prod().min
        }

        pub fn all_max(&self) -> T {
            self.segtree.all_prod().max
        }

        pub fn chmin(&mut self, p: usize, x: T) {
            self.segtree.apply(p, ChminChmaxAction::new_chmin(x))
        }

        pub fn chmax(&mut self, p: usize, x: T) {
            self.segtree.apply(p, ChminChmaxAction::new_chmax(x))
        }

        pub fn range_chmin<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree
                .apply_range(range, ChminChmaxAction::new_chmin(x))
        }

        pub fn range_chmax<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree
                .apply_range(range, ChminChmaxAction::new_chmax(x))
        }

        pub fn range_update<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree
                .apply_range(range, ChminChmaxAction::new_update(x))
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
pub mod test_range_chmin_chmax_range_min_max {
    use super::range_chmin_chmax_range_min_max::{RangeChminChmaxRangeMinMaxSegtree, RangeMinMax};

    #[test]
    fn test_new_and_get() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxRangeMinMaxSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.get(0), 10);
        assert_eq!(segtree.get(2), 30);
        assert_eq!(segtree.get(4), 50);
    }

    #[test]
    fn test_set() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.set(0, 5);
        assert_eq!(segtree.to_vec(), vec![5, 20, 30, 40, 50]);
        segtree.set(4, 45);
        assert_eq!(segtree.to_vec(), vec![5, 20, 30, 40, 45]);
    }

    #[test]
    fn test_range_min_max() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxRangeMinMaxSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.range_minmax(1..4), RangeMinMax { min: 20, max: 40 });
        assert_eq!(segtree.range_min(1..4), 20);
        assert_eq!(segtree.range_max(1..4), 40);
    }

    #[test]
    fn test_all_min_max() {
        let xs = vec![10, 20, 30, 40, 50];
        let segtree = RangeChminChmaxRangeMinMaxSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.all_minmax(), RangeMinMax { min: 10, max: 50 });
        assert_eq!(segtree.all_min(), 10);
        assert_eq!(segtree.all_max(), 50);
    }

    #[test]
    fn test_chmin() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.chmin(1, 15);
        assert_eq!(segtree.to_vec(), vec![10, 15, 30, 40, 50]);
        segtree.chmin(1, 25); // No change
        assert_eq!(segtree.to_vec(), vec![10, 15, 30, 40, 50]);
    }

    #[test]
    fn test_chmax() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.chmax(1, 25);
        assert_eq!(segtree.to_vec(), vec![10, 25, 30, 40, 50]);
        segtree.chmax(1, 15); // No change
        assert_eq!(segtree.to_vec(), vec![10, 25, 30, 40, 50]);
    }

    #[test]
    fn test_range_chmin() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.range_chmin(1..4, 15);
        assert_eq!(segtree.to_vec(), vec![10, 15, 15, 15, 50]);
        segtree.range_chmin(0..3, 5);
        assert_eq!(segtree.to_vec(), vec![5, 5, 5, 15, 50]);
    }

    #[test]
    fn test_range_chmax() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.range_chmax(1..4, 35);
        assert_eq!(segtree.to_vec(), vec![10, 35, 35, 40, 50]);
        segtree.range_chmax(0..3, 100);
        assert_eq!(segtree.to_vec(), vec![100, 100, 100, 40, 50]);
    }

    #[test]
    fn test_range_update() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.range_update(1..4, 5);
        assert_eq!(segtree.to_vec(), vec![10, 5, 5, 5, 50]);
    }

    #[ignore]
    #[test]
    fn test_random_chmin_chmax_min_max() {
        use rand::{Rng, SeedableRng, rngs::SmallRng};

        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.random_range(1..=20);
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
            let mut segtree = RangeChminChmaxRangeMinMaxSegtree::<i64>::from_slice(&naive_vec);

            for _ in 0..100 {
                // 100 random operations per set
                let op_type = rng.random_range(0..9); // 9 operations

                match op_type {
                    0 => {
                        // set(p, x)
                        let p = rng.random_range(0..n);
                        let x = rng.random_range(-100..=100);
                        naive_vec[p] = x;
                        segtree.set(p, x);
                    }
                    1 => {
                        // chmin(p, x)
                        let p = rng.random_range(0..n);
                        let x = rng.random_range(-50..=50);
                        naive_vec[p] = naive_vec[p].min(x);
                        segtree.chmin(p, x);
                    }
                    2 => {
                        // chmax(p, x)
                        let p = rng.random_range(0..n);
                        let x = rng.random_range(-50..=50);
                        naive_vec[p] = naive_vec[p].max(x);
                        segtree.chmax(p, x);
                    }
                    3 => {
                        // range_chmin(range, x)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);

                        let x = rng.random_range(-50..=50);

                        for i in l..r {
                            naive_vec[i] = naive_vec[i].min(x);
                        }
                        segtree.range_chmin(l..r, x);
                    }
                    4 => {
                        // range_chmax(range, x)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);

                        let x = rng.random_range(-50..=50);

                        for i in l..r {
                            naive_vec[i] = naive_vec[i].max(x);
                        }
                        segtree.range_chmax(l..r, x);
                    }
                    5 => {
                        // range_update(range, x)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);

                        let x = rng.random_range(-50..=50);

                        for i in l..r {
                            naive_vec[i] = x;
                        }
                        segtree.range_update(l..r, x);
                    }
                    6 => {
                        // range_minmax(range)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);

                        let expected_min =
                            naive_vec[l..r].iter().copied().min().unwrap_or(i64::MAX);
                        assert_eq!(
                            segtree.range_min(l..r),
                            expected_min,
                            "range_min({}..{}) failed",
                            l,
                            r
                        );
                        let expected_max =
                            naive_vec[l..r].iter().copied().max().unwrap_or(i64::MIN);
                        assert_eq!(
                            segtree.range_max(l..r),
                            expected_max,
                            "range_max({}..{}) failed",
                            l,
                            r
                        );
                    }
                    7 => {
                        // all_minmax()
                        let expected_min = naive_vec.iter().copied().min().unwrap_or(i64::MAX);
                        assert_eq!(segtree.all_min(), expected_min, "all_min() failed");
                        let expected_max = naive_vec.iter().copied().max().unwrap_or(i64::MIN);
                        assert_eq!(segtree.all_max(), expected_max, "all_max() failed");
                    }
                    8 => {
                        // get(p)
                        let p = rng.random_range(0..n);
                        assert_eq!(segtree.get(p), naive_vec[p]);
                    }
                    _ => unreachable!(),
                }
            }

            // Final check
            assert_eq!(segtree.to_vec(), naive_vec, "final to_vec() check failed");
        }
    }
}
