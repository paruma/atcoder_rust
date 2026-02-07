use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_add_range_min::*;")]
pub mod range_add_range_min {
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::iter::Sum;
    use std::marker::PhantomData;
    use std::ops::{Add, RangeBounds};

    fn zero<T: Sum>() -> T {
        std::iter::empty::<T>().sum()
    }

    // Range minimum query monoid
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
        T: Copy,
    {
        pub fn new(val: T) -> Self {
            Self { add_val: val }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeAddRangeMin<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for RangeAddRangeMin<T>
    where
        T: Copy + Ord + Bounded + Add<Output = T> + Sum,
    {
        type M = RangeMin<T>;
        type F = AddAction<T>;

        fn identity_map() -> Self::F {
            AddAction { add_val: zero() }
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

    #[derive(Clone)]
    pub struct RangeAddRangeMinSegtree<T>
    where
        T: Copy + Ord + Bounded + Add<Output = T> + Sum,
    {
        segtree: LazySegtree<RangeAddRangeMin<T>>,
        len: usize,
    }

    impl<T> RangeAddRangeMinSegtree<T>
    where
        T: Copy + Ord + Bounded + Add<Output = T> + Sum,
    {
        pub fn new(n: usize) -> Self {
            let xs = vec![zero(); n];
            Self::from_slice(&xs)
        }

        pub fn from_slice(xs: &[T]) -> Self {
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

        pub fn range_add<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, AddAction::new(x))
        }

        /// 左端 `l` を固定し、区間 `[l, r)` での最小値が述語 `g` を満たすような最大の `r` を返します。
        ///
        /// # 計算量
        /// O(log N)
        pub fn max_right<G>(&mut self, l: usize, g: G) -> usize
        where
            G: Fn(T) -> bool,
        {
            self.segtree.max_right(l, g)
        }

        /// 右端 `r` を固定し、区間 `[l, r)` での最小値が述語 `g` を満たすような最小の `l` を返します。
        ///
        /// # 計算量
        /// O(log N)
        pub fn min_left<G>(&mut self, r: usize, g: G) -> usize
        where
            G: Fn(T) -> bool,
        {
            self.segtree.min_left(r, g)
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
pub mod test_range_add_range_min {
    use super::range_add_range_min::RangeAddRangeMinSegtree;

    #[test]
    fn test_new_and_get() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeAddRangeMinSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.get(0), 10);
        assert_eq!(segtree.get(2), 30);
        assert_eq!(segtree.get(4), 50);
    }

    #[test]
    fn test_range_min() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeAddRangeMinSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.range_min(1..4), 20);
    }

    #[test]
    fn test_range_add() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeAddRangeMinSegtree::<i64>::from_slice(&xs);
        segtree.range_add(1..4, 5);
        assert_eq!(segtree.to_vec(), vec![10, 25, 35, 45, 50]);
        assert_eq!(segtree.range_min(1..4), 25);
    }

    #[test]
    fn test_to_vec() {
        let xs = vec![0, 1, 2, 3, 4, 5];
        let mut segtree = RangeAddRangeMinSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.to_vec(), vec![0, 1, 2, 3, 4, 5]);
        segtree.range_add(1..4, 10);
        assert_eq!(segtree.to_vec(), vec![0, 11, 12, 13, 4, 5]);
    }

    #[test]
    fn test_all_min() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeAddRangeMinSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.all_min(), 10);
        segtree.range_add(0..5, -5);
        assert_eq!(segtree.all_min(), 5);
    }

    #[test]
    fn test_max_right_min_left() {
        let xs = vec![5, 4, 3, 2, 1];
        let mut segtree = RangeAddRangeMinSegtree::<i64>::from_slice(&xs);
        // max_right: [0, r) で min が 3 以上の最大の r
        assert_eq!(segtree.max_right(0, |m| m >= 3), 3);
        // min_left: [l, 5) で min が 3 以上の最小の l (identity i64::MAX >= 3 は true)
        // [2, 1] は 3 以上ではないので、l=5, 4, 3... と見ていって止まる
        // 実際には [3, 4, 5] の min は 1 or 2 なので g(min) >= 3 は false。
        // 右端 5 から左へ: [4..5] min=1 (false), なので 5。
        assert_eq!(segtree.min_left(5, |m| m >= 3), 5);
        // [0..3] min=3 (true) なので、右端 3 から左へなら 0 までいける
        assert_eq!(segtree.min_left(3, |m| m >= 3), 0);
    }

    #[test]
    fn test_identity_mapping() {
        let xs: Vec<i64> = vec![];
        let mut segtree = RangeAddRangeMinSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.all_min(), i64::MAX);
        segtree.range_add(0..0, 100);
        assert_eq!(segtree.all_min(), i64::MAX);
    }

    #[ignore]
    #[test]
    fn test_random_add_min() {
        use rand::{Rng, SeedableRng, rngs::SmallRng};

        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.random_range(1..=20);
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
            let mut segtree = RangeAddRangeMinSegtree::<i64>::from_slice(&naive_vec);

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
                        // range_add(range, x)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);

                        let x = rng.random_range(-50..=50);

                        for i in l..r {
                            naive_vec[i] += x;
                        }
                        segtree.range_add(l..r, x);
                    }
                    2 => {
                        // get(p)
                        let p = rng.random_range(0..n);
                        assert_eq!(segtree.get(p), naive_vec[p], "get({}) failed", p);
                    }
                    3 => {
                        // range_min(range)
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
