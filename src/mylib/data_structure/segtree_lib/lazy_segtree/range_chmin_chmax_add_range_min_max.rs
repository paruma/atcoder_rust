use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_chmin_chmax_add_range_min_max::*;")]
pub mod range_chmin_chmax_add_range_min_max {
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::iter::{Product, Sum};
    use std::marker::PhantomData;
    use std::ops::{Add, Mul, RangeBounds};

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

    // Action for RangeChminChmaxAddRangeMinMax
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ChminChmaxAddAction<T> {
        chmin_val: T,
        chmax_val: T,
        add_val: T,
    }

    impl<T> ChminChmaxAddAction<T>
    where
        T: Copy + Ord + Bounded + Sum,
    {
        pub fn new_chmin(val: T) -> Self {
            Self {
                chmin_val: val,
                chmax_val: T::min_value(),
                add_val: std::iter::empty::<T>().sum(),
            }
        }

        pub fn new_chmax(val: T) -> Self {
            Self {
                chmin_val: T::max_value(),
                chmax_val: val,
                add_val: std::iter::empty::<T>().sum(),
            }
        }

        pub fn new_add(val: T) -> Self {
            Self {
                chmin_val: T::max_value(),
                chmax_val: T::min_value(),
                add_val: val,
            }
        }

        pub fn new_update(val: T) -> Self {
            Self {
                chmin_val: val,
                chmax_val: val,
                add_val: std::iter::empty::<T>().sum(),
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeChminChmaxAddRangeMinMax<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for RangeChminChmaxAddRangeMinMax<T>
    where
        T: Copy + Ord + Bounded + Add<Output = T> + Mul<Output = T> + Sum + Product,
    {
        type M = RangeMinMaxMonoid<T>;
        type F = ChminChmaxAddAction<T>;

        fn identity_map() -> Self::F {
            ChminChmaxAddAction {
                chmin_val: T::max_value(),
                chmax_val: T::min_value(),
                add_val: std::iter::empty::<T>().sum(),
            }
        }

        // f: new action, g: old action
        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            let new_add = g.add_val + f.add_val;

            let new_chmin_tmp = if g.chmin_val == T::max_value() {
                T::max_value()
            } else {
                g.chmin_val + f.add_val
            };

            let new_chmax_tmp = if g.chmax_val == T::min_value() {
                T::min_value()
            } else {
                g.chmax_val + f.add_val
            };

            let new_chmin = new_chmin_tmp.clamp(f.chmax_val, f.chmin_val);
            let new_chmax = new_chmax_tmp.clamp(f.chmax_val, f.chmin_val);

            ChminChmaxAddAction {
                chmin_val: new_chmin,
                chmax_val: new_chmax,
                add_val: new_add,
            }
        }

        fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
            let RangeMinMax { min, max } = *x;
            if min > max {
                return *x;
            }
            let new_min = (min + f.add_val).clamp(f.chmax_val, f.chmin_val);
            let new_max = (max + f.add_val).clamp(f.chmax_val, f.chmin_val);
            RangeMinMax {
                min: new_min,
                max: new_max,
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
    pub struct RangeChminChmaxAddRangeMinMaxSegtree<T>
    where
        T: Copy + Ord + Bounded + Add<Output = T> + Mul<Output = T> + Sum + Product,
    {
        segtree: LazySegtree<RangeChminChmaxAddRangeMinMax<T>>,
        len: usize,
    }

    impl<T> RangeChminChmaxAddRangeMinMaxSegtree<T>
    where
        T: Copy + Ord + Bounded + Add<Output = T> + Mul<Output = T> + Sum + Product,
    {
        pub fn new(n: usize) -> Self {
            let xs = vec![std::iter::empty::<T>().sum(); n];
            Self::from_slice(&xs)
        }

        pub fn from_slice(xs: &[T]) -> Self {
            let len = xs.len();
            let vec = xs.iter().map(|&x| RangeMinMax::unit(x)).collect::<Vec<_>>();
            Self {
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

        pub fn range_chmin<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree
                .apply_range(range, ChminChmaxAddAction::new_chmin(x))
        }

        pub fn range_chmax<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree
                .apply_range(range, ChminChmaxAddAction::new_chmax(x))
        }

        pub fn range_add<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree
                .apply_range(range, ChminChmaxAddAction::new_add(x))
        }

        pub fn range_update<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree
                .apply_range(range, ChminChmaxAddAction::new_update(x))
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
pub mod test_range_chmin_chmax_add_range_min_max {
    use super::range_chmin_chmax_add_range_min_max::{
        RangeChminChmaxAddRangeMinMaxSegtree, RangeMinMax,
    };

    #[test]
    fn test_new_and_get() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAddRangeMinMaxSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.get(0), 10);
        assert_eq!(segtree.get(2), 30);
        assert_eq!(segtree.get(4), 50);
    }

    #[test]
    fn test_range_min_max() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAddRangeMinMaxSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.range_minmax(1..4), RangeMinMax { min: 20, max: 40 });
        assert_eq!(segtree.range_min(1..4), 20);
        assert_eq!(segtree.range_max(1..4), 40);
    }

    #[test]
    fn test_all_minmax() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAddRangeMinMaxSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.all_minmax(), RangeMinMax { min: 10, max: 50 });
        assert_eq!(segtree.all_min(), 10);
        assert_eq!(segtree.all_max(), 50);

        segtree.range_add(.., 5);
        assert_eq!(segtree.all_min(), 15);
        assert_eq!(segtree.all_max(), 55);
    }

    #[test]
    fn test_range_chmin() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAddRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.range_chmin(1..4, 15);
        assert_eq!(segtree.to_vec(), vec![10, 15, 15, 15, 50]);
    }

    #[test]
    fn test_range_chmax() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAddRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.range_chmax(1..4, 35);
        assert_eq!(segtree.to_vec(), vec![10, 35, 35, 40, 50]);
    }

    #[test]
    fn test_range_add() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAddRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.range_add(1..4, 5);
        assert_eq!(segtree.to_vec(), vec![10, 25, 35, 45, 50]);
    }

    #[test]
    fn test_chmin_add() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAddRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.range_chmin(1..4, 25); // [10, 20, 25, 25, 50]
        segtree.range_add(0..3, 5); // [15, 25, 30, 25, 50]
        assert_eq!(segtree.to_vec(), vec![15, 25, 30, 25, 50]);
    }

    #[test]
    fn test_add_chmin() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAddRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.range_add(0..3, 5); // [15, 25, 35, 40, 50]
        segtree.range_chmin(1..4, 28); // [15, 25, 28, 28, 50]
        assert_eq!(segtree.to_vec(), vec![15, 25, 28, 28, 50]);
    }

    #[test]
    fn test_chmax_add() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAddRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.range_chmax(1..4, 25); // [10, 25, 30, 40, 50]
        segtree.range_add(0..3, 5); // [15, 30, 35, 40, 50]
        assert_eq!(segtree.to_vec(), vec![15, 30, 35, 40, 50]);
    }

    #[test]
    fn test_add_chmax() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAddRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.range_add(0..3, 5); // [15, 25, 35, 40, 50]
        segtree.range_chmax(1..4, 30); // [15, 30, 35, 40, 50]
        assert_eq!(segtree.to_vec(), vec![15, 30, 35, 40, 50]);
    }

    #[test]
    fn test_chmin_chmax_add() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAddRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.range_chmin(0..5, 35); // [10, 20, 30, 35, 35]
        segtree.range_chmax(0..5, 15); // [15, 20, 30, 35, 35]
        segtree.range_add(0..5, 3); // [18, 23, 33, 38, 38]
        assert_eq!(segtree.to_vec(), vec![18, 23, 33, 38, 38]);
    }

    #[test]
    fn test_range_update() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAddRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.range_update(1..4, 5);
        assert_eq!(segtree.to_vec(), vec![10, 5, 5, 5, 50]);
    }

    #[test]
    fn test_update_add() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAddRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.range_update(1..4, 5); // [10, 5, 5, 5, 50]
        segtree.range_add(2..5, 10); // [10, 5, 15, 15, 60]
        assert_eq!(segtree.to_vec(), vec![10, 5, 15, 15, 60]);
    }

    #[test]
    fn test_add_update() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAddRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.range_add(2..5, 10); // [10, 20, 40, 50, 60]
        segtree.range_update(1..4, 5); // [10, 5, 5, 5, 60]
        assert_eq!(segtree.to_vec(), vec![10, 5, 5, 5, 60]);
    }

    #[ignore]
    #[test]
    fn test_random_chmin_chmax_add_min_max() {
        use rand::{Rng, SeedableRng, rngs::SmallRng};

        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.random_range(1..=20);
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
            let mut segtree = RangeChminChmaxAddRangeMinMaxSegtree::<i64>::from_slice(&naive_vec);

            for _ in 0..100 {
                // 100 random operations per set
                let op_type = rng.random_range(0..7); // 7 operations

                match op_type {
                    0 => {
                        // set(p, x)
                        let p = rng.random_range(0..n);
                        let x = rng.random_range(-100..=100);
                        naive_vec[p] = x;
                        segtree.set(p, x);
                    }
                    1 => {
                        // range_chmin(range, x)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);

                        let x = rng.random_range(-50..=50);

                        for i in l..r {
                            naive_vec[i] = naive_vec[i].min(x);
                        }
                        segtree.range_chmin(l..r, x);
                    }
                    2 => {
                        // range_chmax(range, x)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);

                        let x = rng.random_range(-50..=50);

                        for i in l..r {
                            naive_vec[i] = naive_vec[i].max(x);
                        }
                        segtree.range_chmax(l..r, x);
                    }
                    3 => {
                        // range_add(range, x)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);

                        let x = rng.random_range(-50..=50);

                        for i in l..r {
                            naive_vec[i] += x;
                        }
                        segtree.range_add(l..r, x);
                    }
                    4 => {
                        // range_update(range, x)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);

                        let x = rng.random_range(-50..=50);

                        for i in l..r {
                            naive_vec[i] = x;
                        }
                        segtree.range_update(l..r, x);
                    }
                    5 => {
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
                    6 => {
                        // all_minmax()
                        let expected_min = naive_vec.iter().copied().min().unwrap_or(i64::MAX);
                        assert_eq!(segtree.all_min(), expected_min, "all_min() failed");
                        let expected_max = naive_vec.iter().copied().max().unwrap_or(i64::MIN);
                        assert_eq!(segtree.all_max(), expected_max, "all_max() failed");
                    }
                    _ => unreachable!(),
                }
            }

            // Final check
            assert_eq!(segtree.to_vec(), naive_vec, "final to_vec() check failed");
        }
    }
}
