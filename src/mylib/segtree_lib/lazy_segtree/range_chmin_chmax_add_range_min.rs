use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_chmin_chmax_add_range_min::*;")]
pub mod range_chmin_chmax_add_range_min {
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::marker::PhantomData;
    use std::ops::{Add, Mul, RangeBounds};

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

    // Action for RangeChminChmaxAddRangeMin
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ChminChmaxAddAction<T> {
        chmin_val: T,
        chmax_val: T,
        add_val: T,
    }

    impl<T> ChminChmaxAddAction<T>
    where
        T: Copy + Ord + Bounded + From<i64>,
    {
        pub fn new_chmin(val: T) -> Self {
            Self {
                chmin_val: val,
                chmax_val: T::min_value(),
                add_val: T::from(0),
            }
        }

        pub fn new_chmax(val: T) -> Self {
            Self {
                chmin_val: T::max_value(),
                chmax_val: val,
                add_val: T::from(0),
            }
        }

        pub fn new_add(val: T) -> Self {
            Self {
                chmin_val: T::max_value(),
                chmax_val: T::min_value(),
                add_val: val,
            }
        }
    }

    pub struct RangeChminChmaxAddRangeMin<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for RangeChminChmaxAddRangeMin<T>
    where
        T: Copy + Ord + From<i64> + Bounded + Add<Output = T> + Mul<Output = T>,
    {
        type M = RangeMin<T>;
        type F = ChminChmaxAddAction<T>;

        fn identity_map() -> Self::F {
            ChminChmaxAddAction {
                chmin_val: T::max_value(),
                chmax_val: T::min_value(),
                add_val: T::from(0),
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
            (*x + f.add_val).clamp(f.chmax_val, f.chmin_val)
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

    pub struct RangeChminChmaxAddRangeMinSegtree<T>
    where
        T: Copy + Ord + From<i64> + Bounded + Add<Output = T> + Mul<Output = T>,
    {
        segtree: LazySegtree<RangeChminChmaxAddRangeMin<T>>,
        len: usize,
    }

    impl<T> RangeChminChmaxAddRangeMinSegtree<T>
    where
        T: Copy + Ord + From<i64> + Bounded + Add<Output = T> + Mul<Output = T>,
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

        pub fn apply_range_chmin<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree
                .apply_range(range, ChminChmaxAddAction::new_chmin(x))
        }

        pub fn apply_range_chmax<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree
                .apply_range(range, ChminChmaxAddAction::new_chmax(x))
        }

        pub fn apply_range_add<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree
                .apply_range(range, ChminChmaxAddAction::new_add(x))
        }

        pub fn to_vec(&mut self) -> Vec<T> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}

#[cfg(test)]
pub mod test_range_chmin_chmax_add_range_min {
    use super::range_chmin_chmax_add_range_min::RangeChminChmaxAddRangeMinSegtree;

    #[test]
    fn test_new_and_get() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAddRangeMinSegtree::<i64>::new(&xs);
        assert_eq!(segtree.get(0), 10);
        assert_eq!(segtree.get(2), 30);
        assert_eq!(segtree.get(4), 50);
    }

    #[test]
    fn test_range_min() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAddRangeMinSegtree::<i64>::new(&xs);
        assert_eq!(segtree.range_min(1..4), 20);
    }

    #[test]
    fn test_apply_range_chmin() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAddRangeMinSegtree::<i64>::new(&xs);
        segtree.apply_range_chmin(1..4, 15);
        assert_eq!(segtree.to_vec(), vec![10, 15, 15, 15, 50]);
    }

    #[test]
    fn test_apply_range_chmax() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAddRangeMinSegtree::<i64>::new(&xs);
        segtree.apply_range_chmax(1..4, 35);
        assert_eq!(segtree.to_vec(), vec![10, 35, 35, 40, 50]);
    }

    #[test]
    fn test_apply_range_add() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAddRangeMinSegtree::<i64>::new(&xs);
        segtree.apply_range_add(1..4, 5);
        assert_eq!(segtree.to_vec(), vec![10, 25, 35, 45, 50]);
    }

    #[test]
    fn test_chmin_add() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAddRangeMinSegtree::<i64>::new(&xs);
        segtree.apply_range_chmin(1..4, 25); // [10, 20, 25, 25, 50]
        segtree.apply_range_add(0..3, 5); // [15, 25, 30, 25, 50]
        assert_eq!(segtree.to_vec(), vec![15, 25, 30, 25, 50]);
    }

    #[test]
    fn test_add_chmin() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAddRangeMinSegtree::<i64>::new(&xs);
        segtree.apply_range_add(0..3, 5); // [15, 25, 35, 40, 50]
        segtree.apply_range_chmin(1..4, 28); // [15, 25, 28, 28, 50]
        assert_eq!(segtree.to_vec(), vec![15, 25, 28, 28, 50]);
    }

    #[test]
    fn test_chmax_add() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAddRangeMinSegtree::<i64>::new(&xs);
        segtree.apply_range_chmax(1..4, 25); // [10, 25, 30, 40, 50]
        segtree.apply_range_add(0..3, 5); // [15, 30, 35, 40, 50]
        assert_eq!(segtree.to_vec(), vec![15, 30, 35, 40, 50]);
    }

    #[test]
    fn test_add_chmax() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAddRangeMinSegtree::<i64>::new(&xs);
        segtree.apply_range_add(0..3, 5); // [15, 25, 35, 40, 50]
        segtree.apply_range_chmax(1..4, 30); // [15, 30, 35, 40, 50]
        assert_eq!(segtree.to_vec(), vec![15, 30, 35, 40, 50]);
    }

    #[test]
    fn test_chmin_chmax_add() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAddRangeMinSegtree::<i64>::new(&xs);
        segtree.apply_range_chmin(0..5, 35); // [10, 20, 30, 35, 35]
        segtree.apply_range_chmax(0..5, 15); // [15, 20, 30, 35, 35]
        segtree.apply_range_add(0..5, 3); // [18, 23, 33, 38, 38]
        assert_eq!(segtree.to_vec(), vec![18, 23, 33, 38, 38]);
    }

    #[ignore]
    #[test]
    fn test_random_chmin_chmax_add_min() {
        use rand::{rngs::SmallRng, Rng, SeedableRng};

        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.gen_range(1..=20);
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.gen_range(-100..=100)).collect();
            let mut segtree = RangeChminChmaxAddRangeMinSegtree::<i64>::new(&naive_vec);

            for _ in 0..100 {
                // 100 random operations per set
                let op_type = rng.gen_range(0..6); // 6 operations

                match op_type {
                    0 => {
                        // set(p, x)
                        if n == 0 {
                            continue;
                        }
                        let p = rng.gen_range(0..n);
                        let x = rng.gen_range(-100..=100);
                        naive_vec[p] = x;
                        segtree.set(p, x);
                    }
                    1 => {
                        // apply_range_chmin(range, x)
                        if n == 0 {
                            continue;
                        }
                        let mut p1 = rng.gen_range(0..=n);
                        let mut p2 = rng.gen_range(0..=n);
                        if p1 == p2 {
                            continue;
                        }
                        if p1 > p2 {
                            std::mem::swap(&mut p1, &mut p2);
                        }
                        let l = p1;
                        let r = p2;

                        let x = rng.gen_range(-50..=50);

                        for i in l..r {
                            naive_vec[i] = naive_vec[i].min(x);
                        }
                        segtree.apply_range_chmin(l..r, x);
                    }
                    2 => {
                        // apply_range_chmax(range, x)
                        if n == 0 {
                            continue;
                        }
                        let mut p1 = rng.gen_range(0..=n);
                        let mut p2 = rng.gen_range(0..=n);
                        if p1 == p2 {
                            continue;
                        }
                        if p1 > p2 {
                            std::mem::swap(&mut p1, &mut p2);
                        }
                        let l = p1;
                        let r = p2;

                        let x = rng.gen_range(-50..=50);

                        for i in l..r {
                            naive_vec[i] = naive_vec[i].max(x);
                        }
                        segtree.apply_range_chmax(l..r, x);
                    }
                    3 => {
                        // apply_range_add(range, x)
                        if n == 0 {
                            continue;
                        }
                        let mut p1 = rng.gen_range(0..=n);
                        let mut p2 = rng.gen_range(0..=n);
                        if p1 == p2 {
                            continue;
                        }
                        if p1 > p2 {
                            std::mem::swap(&mut p1, &mut p2);
                        }
                        let l = p1;
                        let r = p2;

                        let x = rng.gen_range(-50..=50);

                        for i in l..r {
                            naive_vec[i] += x;
                        }
                        segtree.apply_range_add(l..r, x);
                    }
                    4 => {
                        // range_min(range)
                        if n == 0 {
                            continue;
                        }
                        let mut p1 = rng.gen_range(0..=n);
                        let mut p2 = rng.gen_range(0..=n);
                        if p1 > p2 {
                            std::mem::swap(&mut p1, &mut p2);
                        }
                        let l = p1;
                        let r = p2;

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
                    5 => {
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
