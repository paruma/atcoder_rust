use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_add_range_max::*;")]
pub mod range_add_range_max {
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::iter::Sum;
    use std::marker::PhantomData;
    use std::ops::{Add, RangeBounds};

    fn zero<T: Sum>() -> T {
        std::iter::empty::<T>().sum()
    }

    // Range maximum query monoid
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
        T: Copy,
    {
        pub fn new(val: T) -> Self {
            Self { add_val: val }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeAddRangeMax<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for RangeAddRangeMax<T>
    where
        T: Copy + Ord + Bounded + Add<Output = T> + Sum,
    {
        type M = RangeMax<T>;
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

    #[derive(Clone)]
    pub struct RangeAddRangeMaxSegtree<T>
    where
        T: Copy + Ord + Bounded + Add<Output = T> + Sum,
    {
        segtree: LazySegtree<RangeAddRangeMax<T>>,
        len: usize,
    }

    impl<T> RangeAddRangeMaxSegtree<T>
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

        pub fn range_max<R>(&mut self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range)
        }

        pub fn all_max(&self) -> T {
            self.segtree.all_prod()
        }

        pub fn range_add<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, AddAction::new(x))
        }

        /// 左端 `l` を固定し、区間 `[l, r)` での最大値が述語 `g` を満たすような最大の `r` を返します。
        ///
        /// # 計算量
        /// O(log N)
        pub fn max_right<G>(&mut self, l: usize, g: G) -> usize
        where
            G: Fn(T) -> bool,
        {
            self.segtree.max_right(l, g)
        }

        /// 右端 `r` を固定し、区間 `[l, r)` での最大値が述語 `g` を満たすような最小の `l` を返します。
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
pub mod test_range_add_range_max {
    use super::range_add_range_max::RangeAddRangeMaxSegtree;

    #[test]
    fn test_new_and_get() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeAddRangeMaxSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.get(0), 10);
        assert_eq!(segtree.get(2), 30);
        assert_eq!(segtree.get(4), 50);
    }

    #[test]
    fn test_range_max() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeAddRangeMaxSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.range_max(1..4), 40);
    }

    #[test]
    fn test_range_add() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeAddRangeMaxSegtree::<i64>::from_slice(&xs);
        segtree.range_add(1..4, 5);
        assert_eq!(segtree.to_vec(), vec![10, 25, 35, 45, 50]);
        assert_eq!(segtree.range_max(1..4), 45);
    }

    #[test]
    fn test_to_vec() {
        let xs = vec![0, 1, 2, 3, 4, 5];
        let mut segtree = RangeAddRangeMaxSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.to_vec(), vec![0, 1, 2, 3, 4, 5]);
        segtree.range_add(1..4, 10);
        assert_eq!(segtree.to_vec(), vec![0, 11, 12, 13, 4, 5]);
    }

    #[test]
    fn test_all_max() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeAddRangeMaxSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.all_max(), 50);
        segtree.range_add(0..5, -5);
        assert_eq!(segtree.all_max(), 45);
    }

    #[test]
    fn test_max_right_min_left() {
        let xs = vec![1, 2, 3, 4, 5];
        let mut segtree = RangeAddRangeMaxSegtree::<i64>::from_slice(&xs);
        // max_right: [0, r) で max が 3 以下の最大の r
        assert_eq!(segtree.max_right(0, |m| m <= 3), 3);
        // min_left: [l, 5) で max が 3 以下の最小の l (identity i64::MIN <= 3 は true)
        // [4, 5] は 3 以下ではないので、l=5, 4, 3... と見ていって l=5, 4 で止まる？
        // 実際には [4, 5] の max は 5 なので g(5) <= 3 は false。
        // 右端 5 から左へ: [4..5] max=5 (false), なので 5。
        assert_eq!(segtree.min_left(5, |m| m <= 3), 5);
        // [0..3] max=3 (true) なので、右端 3 から左へなら 0 までいける
        assert_eq!(segtree.min_left(3, |m| m <= 3), 0);
    }

    #[test]
    fn test_identity_mapping() {
        let xs: Vec<i64> = vec![];
        let mut segtree = RangeAddRangeMaxSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.all_max(), i64::MIN);
        segtree.range_add(0..0, 100);
        assert_eq!(segtree.all_max(), i64::MIN);
    }

    #[ignore]
    #[test]
    fn test_random_add_max() {
        use rand::{Rng, SeedableRng, rngs::SmallRng};

        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.random_range(1..=20);
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
            let mut segtree = RangeAddRangeMaxSegtree::<i64>::from_slice(&naive_vec);

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
                        // range_max(range)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);

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
                    4 => {
                        // all_max()
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
