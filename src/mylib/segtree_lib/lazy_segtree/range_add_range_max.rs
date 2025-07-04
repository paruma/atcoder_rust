use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_add_range_max::*;")]
pub mod range_add_range_max {
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::marker::PhantomData;
    use std::ops::{Add, RangeBounds};

    // Range maximum query monoid
    pub struct RangeMax<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for RangeMax<T>
    where
        T: Copy + Ord + Bounded,
    {
        type S = T;
        fn identity() -> T {
            T::min_value()
        }
        fn binary_operation(a: &T, b: &T) -> T {
            *a.max(b)
        }
    }

    // Action for RangeAddRangeMax
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

    pub struct RangeAddRangeMax<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for RangeAddRangeMax<T>
    where
        T: Copy + Ord + From<i64> + Bounded + Add<Output = T>,
    {
        type M = RangeMax<T>;
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
            if *x == T::min_value() {
                T::min_value()
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

    pub struct RangeAddRangeMaxSegtree<T>
    where
        T: Copy + Ord + From<i64> + Bounded + Add<Output = T>,
    {
        segtree: LazySegtree<RangeAddRangeMax<T>>,
        len: usize,
    }

    impl<T> RangeAddRangeMaxSegtree<T>
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

        pub fn range_max<R>(&mut self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range)
        }

        pub fn all_max(&self) -> T {
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
pub mod test_range_add_range_max {
    use super::range_add_range_max::RangeAddRangeMaxSegtree;

    #[test]
    fn test_new_and_get() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeAddRangeMaxSegtree::<i64>::new(&xs);
        assert_eq!(segtree.get(0), 10);
        assert_eq!(segtree.get(2), 30);
        assert_eq!(segtree.get(4), 50);
    }

    #[test]
    fn test_range_max() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeAddRangeMaxSegtree::<i64>::new(&xs);
        assert_eq!(segtree.range_max(1..4), 40);
    }

    #[test]
    fn test_apply_range_add() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeAddRangeMaxSegtree::<i64>::new(&xs);
        segtree.apply_range_add(1..4, 5);
        assert_eq!(segtree.to_vec(), vec![10, 25, 35, 45, 50]);
        assert_eq!(segtree.range_max(1..4), 45);
    }

    #[test]
    fn test_to_vec() {
        let xs = vec![0, 1, 2, 3, 4, 5];
        let mut segtree = RangeAddRangeMaxSegtree::<i64>::new(&xs);
        assert_eq!(segtree.to_vec(), vec![0, 1, 2, 3, 4, 5]);
        segtree.apply_range_add(1..4, 10);
        assert_eq!(segtree.to_vec(), vec![0, 11, 12, 13, 4, 5]);
    }

    #[test]
    fn test_all_max() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeAddRangeMaxSegtree::<i64>::new(&xs);
        assert_eq!(segtree.all_max(), 50);
        segtree.apply_range_add(0..5, -5);
        assert_eq!(segtree.all_max(), 45);
    }

    #[test]
    fn test_identity_mapping() {
        let xs: Vec<i64> = vec![];
        let mut segtree = RangeAddRangeMaxSegtree::<i64>::new(&xs);
        assert_eq!(segtree.all_max(), i64::MIN);
        segtree.apply_range_add(0..0, 100);
        assert_eq!(segtree.all_max(), i64::MIN);
    }
}
