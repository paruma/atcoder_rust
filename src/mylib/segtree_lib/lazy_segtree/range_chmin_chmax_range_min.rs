use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_chmin_chmax_range_min::*;")]
pub mod range_chmin_chmax_range_min {
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

    // Action for RangeChminChmaxRangeMin
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
    }

    pub struct RangeChminChmaxRangeMin<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for RangeChminChmaxRangeMin<T>
    where
        T: Copy + Ord + From<i64> + Bounded + Add<Output = T> + Mul<Output = T>,
    {
        type M = RangeMin<T>;
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
            (*x).clamp(f.chmax_val, f.chmin_val)
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

    pub struct RangeChminChmaxRangeMinSegtree<T>
    where
        T: Copy + Ord + From<i64> + Bounded + Add<Output = T> + Mul<Output = T>,
    {
        segtree: LazySegtree<RangeChminChmaxRangeMin<T>>,
        len: usize,
    }

    impl<T> RangeChminChmaxRangeMinSegtree<T>
    where
        T: Copy + Ord + From<i64> + Bounded + Add<Output = T> + Mul<Output = T>,
    {
        pub fn new(xs: &[T]) -> RangeChminChmaxRangeMinSegtree<T> {
            let len = xs.len();
            RangeChminChmaxRangeMinSegtree {
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

        pub fn apply_chmin(&mut self, p: usize, x: T) {
            self.segtree.apply(p, ChminChmaxAction::new_chmin(x))
        }

        pub fn apply_chmax(&mut self, p: usize, x: T) {
            self.segtree.apply(p, ChminChmaxAction::new_chmax(x))
        }

        pub fn apply_range_chmin<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree
                .apply_range(range, ChminChmaxAction::new_chmin(x))
        }

        pub fn apply_range_chmax<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree
                .apply_range(range, ChminChmaxAction::new_chmax(x))
        }

        pub fn to_vec(&mut self) -> Vec<T> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}

#[cfg(test)]
pub mod test_range_chmin_chmax_range_min {
    use super::range_chmin_chmax_range_min::RangeChminChmaxRangeMinSegtree;

    #[test]
    fn test_new_and_get() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxRangeMinSegtree::<i64>::new(&xs);
        assert_eq!(segtree.get(0), 10);
        assert_eq!(segtree.get(2), 30);
        assert_eq!(segtree.get(4), 50);
    }

    #[test]
    fn test_set() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxRangeMinSegtree::<i64>::new(&xs);
        segtree.set(0, 5);
        assert_eq!(segtree.to_vec(), vec![5, 20, 30, 40, 50]);
        segtree.set(4, 45);
        assert_eq!(segtree.to_vec(), vec![5, 20, 30, 40, 45]);
    }

    #[test]
    fn test_all_min() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxRangeMinSegtree::<i64>::new(&xs);
        assert_eq!(segtree.all_min(), 10);
        segtree.set(0, 5);
        assert_eq!(segtree.all_min(), 5);
    }

    #[test]
    fn test_range_min() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxRangeMinSegtree::<i64>::new(&xs);
        assert_eq!(segtree.range_min(1..4), 20); // 20, 30, 40
        segtree.set(2, 15);
        assert_eq!(segtree.range_min(1..4), 15); // 20, 15, 40
    }

    #[test]
    fn test_apply_chmin() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxRangeMinSegtree::<i64>::new(&xs);
        segtree.apply_chmin(1, 15);
        assert_eq!(segtree.to_vec(), vec![10, 15, 30, 40, 50]);
        segtree.apply_chmin(1, 25); // No change
        assert_eq!(segtree.to_vec(), vec![10, 15, 30, 40, 50]);
    }

    #[test]
    fn test_apply_chmax() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxRangeMinSegtree::<i64>::new(&xs);
        segtree.apply_chmax(1, 25);
        assert_eq!(segtree.to_vec(), vec![10, 25, 30, 40, 50]);
        segtree.apply_chmax(1, 15); // No change
        assert_eq!(segtree.to_vec(), vec![10, 25, 30, 40, 50]);
    }

    #[test]
    fn test_apply_range_chmin() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxRangeMinSegtree::<i64>::new(&xs);
        segtree.apply_range_chmin(1..4, 15);
        assert_eq!(segtree.to_vec(), vec![10, 15, 15, 15, 50]);
        segtree.apply_range_chmin(0..3, 5);
        assert_eq!(segtree.to_vec(), vec![5, 5, 5, 15, 50]);
    }

    #[test]
    fn test_apply_range_chmax() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxRangeMinSegtree::<i64>::new(&xs);
        segtree.apply_range_chmax(1..4, 35);
        assert_eq!(segtree.to_vec(), vec![10, 35, 35, 40, 50]);
        segtree.apply_range_chmax(0..3, 100);
        assert_eq!(segtree.to_vec(), vec![100, 100, 100, 40, 50]);
    }

    #[test]
    fn test_to_vec() {
        let xs = vec![0, 1, 2, 3, 4, 5];
        let mut segtree = RangeChminChmaxRangeMinSegtree::<i64>::new(&xs);
        assert_eq!(segtree.to_vec(), vec![0, 1, 2, 3, 4, 5]);
        segtree.apply_range_chmin(1..4, -1);
        assert_eq!(segtree.to_vec(), vec![0, -1, -1, -1, 4, 5]);
        segtree.apply_range_chmax(1..4, 100);
        assert_eq!(segtree.to_vec(), vec![0, 100, 100, 100, 4, 5]);
    }

    #[test]
    fn test_chmin_chmax_interaction() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxRangeMinSegtree::<i64>::new(&xs);

        // Apply chmin then chmax
        segtree.apply_range_chmin(0..5, 25); // [10, 20, 25, 25, 25] -> [10, 20, 25, 25, 25]
        segtree.apply_range_chmax(0..5, 15); // [10, 20, 25, 25, 25] -> [15, 20, 25, 25, 25]
        assert_eq!(segtree.to_vec(), vec![15, 20, 25, 25, 25]);

        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxRangeMinSegtree::<i64>::new(&xs);

        // Apply chmax then chmin
        segtree.apply_range_chmax(0..5, 15); // [10, 20, 30, 40, 50] -> [15, 20, 30, 40, 50]
        segtree.apply_range_chmin(0..5, 25); // [15, 20, 30, 40, 50] -> [15, 20, 25, 25, 25]
        assert_eq!(segtree.to_vec(), vec![15, 20, 25, 25, 25]);
    }

    #[test]
    fn test_chmin_chmax_interaction2() {
        let xs = vec![30, 30, 30, 30];
        let mut segtree = RangeChminChmaxRangeMinSegtree::<i64>::new(&xs);

        // Apply chmin then chmax
        segtree.apply_range_chmax(0..3, 40);
        segtree.apply_range_chmin(0..3, 20);
        assert_eq!(segtree.to_vec(), vec![20, 20, 20, 30]);
    }

    #[ignore]
    #[test]
    fn test_random_chmin_chmax_min() {
        use rand::{rngs::SmallRng, Rng, SeedableRng};

        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.gen_range(1..=20);
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.gen_range(-100..=100)).collect();
            let mut segtree = RangeChminChmaxRangeMinSegtree::<i64>::new(&naive_vec);

            for _ in 0..100 {
                // 100 random operations per set
                let op_type = rng.gen_range(0..7); // 7 operations

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
                        // apply_chmin(p, x)
                        if n == 0 {
                            continue;
                        }
                        let p = rng.gen_range(0..n);
                        let x = rng.gen_range(-50..=50);
                        naive_vec[p] = naive_vec[p].min(x);
                        segtree.apply_chmin(p, x);
                    }
                    2 => {
                        // apply_chmax(p, x)
                        if n == 0 {
                            continue;
                        }
                        let p = rng.gen_range(0..n);
                        let x = rng.gen_range(-50..=50);
                        naive_vec[p] = naive_vec[p].max(x);
                        segtree.apply_chmax(p, x);
                    }
                    3 => {
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
                    4 => {
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
                    5 => {
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
                    6 => {
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
