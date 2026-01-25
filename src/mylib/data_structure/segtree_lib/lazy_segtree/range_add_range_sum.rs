use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_add_range_sum::*;")]
pub mod range_add_range_sum {
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeAddRangeSum<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for RangeAddRangeSum<T>
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + From<i64>,
    {
        type M = ValueLenSum<T>;
        type F = T;

        fn identity_map() -> T {
            0.into()
        }
        fn composition(a: &T, b: &T) -> T {
            *a + *b
        }

        fn mapping(f: &T, x: &RangeSum<T>) -> RangeSum<T> {
            RangeSum {
                sum: x.sum + *f * x.len.into(),
                len: x.len,
            }
        }
    }

    #[derive(Clone)]
    pub struct RangeAddRangeSumSegtree<T>
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + From<i64>,
    {
        segtree: LazySegtree<RangeAddRangeSum<T>>,
        len: usize,
    }

    impl<T> RangeAddRangeSumSegtree<T>
    where
        T: Copy + Add<Output = T> + Mul<Output = T> + From<i64>,
    {
        pub fn new(n: usize) -> Self {
            let xs = vec![0.into(); n];
            Self::from_slice(&xs)
        }

        pub fn from_slice(xs: &[T]) -> RangeAddRangeSumSegtree<T> {
            let xs = xs.iter().copied().map(RangeSum::unit).collect_vec();
            let len = xs.len();
            RangeAddRangeSumSegtree {
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

        pub fn add(&mut self, p: usize, x: T) {
            self.segtree.apply(p, x)
        }

        pub fn range_add<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, x)
        }

        /// 左端 `l` を固定し、区間 `[l, r)` での総和が述語 `g` を満たすような最大の `r` を返します。
        ///
        /// # 計算量
        /// O(log N)
        pub fn max_right<G>(&mut self, l: usize, g: G) -> usize
        where
            G: Fn(T) -> bool,
        {
            self.segtree.max_right(l, |x| g(x.sum))
        }

        /// 右端 `r` を固定し、区間 `[l, r)` での総和が述語 `g` を満たすような最小の `l` を返します。
        ///
        /// # 計算量
        /// O(log N)
        pub fn min_left<G>(&mut self, r: usize, g: G) -> usize
        where
            G: Fn(T) -> bool,
        {
            self.segtree.min_left(r, |x| g(x.sum))
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
pub mod test_range_add_range_sum {
    use ac_library::ModInt998244353;

    use super::range_add_range_sum::RangeAddRangeSumSegtree;

    type Mint = ModInt998244353;

    #[test]
    fn test_new_and_get() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeAddRangeSumSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.get(0), 10);
        assert_eq!(segtree.get(2), 30);
        assert_eq!(segtree.get(4), 50);
    }

    #[test]
    fn test_set() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeAddRangeSumSegtree::<i64>::from_slice(&xs);
        segtree.set(0, 5);
        assert_eq!(segtree.to_vec(), vec![5, 20, 30, 40, 50]);
        segtree.set(4, 45);
        assert_eq!(segtree.to_vec(), vec![5, 20, 30, 40, 45]);
    }

    #[test]
    fn test_all_sum() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeAddRangeSumSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.all_sum(), 150);
        segtree.set(0, 5);
        assert_eq!(segtree.all_sum(), 145);
    }

    #[test]
    fn test_range_sum() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeAddRangeSumSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.range_sum(1..4), 90); // 20 + 30 + 40
        segtree.set(2, 15);
        assert_eq!(segtree.range_sum(1..4), 75); // 20 + 15 + 40
    }

    #[test]
    fn test_add() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeAddRangeSumSegtree::<i64>::from_slice(&xs);
        segtree.add(1, 5);
        assert_eq!(segtree.to_vec(), vec![10, 25, 30, 40, 50]);
        segtree.add(1, -10);
        assert_eq!(segtree.to_vec(), vec![10, 15, 30, 40, 50]);
    }

    #[test]
    fn test_range_add() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeAddRangeSumSegtree::<i64>::from_slice(&xs);
        segtree.range_add(1..4, 5);
        assert_eq!(segtree.to_vec(), vec![10, 25, 35, 45, 50]);
        segtree.range_add(0..3, -10);
        assert_eq!(segtree.to_vec(), vec![0, 15, 25, 45, 50]);
    }

    #[test]
    fn test_to_vec() {
        let xs = vec![0, 1, 2, 3, 4, 5];
        let mut segtree = RangeAddRangeSumSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.to_vec(), vec![0, 1, 2, 3, 4, 5]);
        segtree.range_add(1..4, 10);
        assert_eq!(segtree.to_vec(), vec![0, 11, 12, 13, 4, 5]);
    }

    #[test]
    fn test_modint() {
        let xs = vec![Mint::new(1), Mint::new(2), Mint::new(3)];
        let mut segtree = RangeAddRangeSumSegtree::<Mint>::from_slice(&xs);
        segtree.range_add(0..3, Mint::new(1));
        assert_eq!(
            segtree.to_vec(),
            vec![Mint::new(2), Mint::new(3), Mint::new(4)]
        );
    }

    #[test]
    fn test_max_right_min_left() {
        // [1, 1, 1, 1, 1]
        let xs = vec![1, 1, 1, 1, 1];
        let mut segtree = RangeAddRangeSumSegtree::<i64>::from_slice(&xs);

        // max_right: 左端 0 から、和が 3 以下である最大の右端
        // sum(0..1)=1, sum(0..2)=2, sum(0..3)=3, sum(0..4)=4 (NG)
        // -> 右端は 3
        assert_eq!(segtree.max_right(0, |s| s <= 3), 3);
        assert_eq!(segtree.max_right(0, |s| s <= 0), 0);
        assert_eq!(segtree.max_right(0, |s| s < 10), 5);

        // min_left: 右端 5 から、和が 3 以下である最小の左端
        // sum(4..5)=1, sum(3..5)=2, sum(2..5)=3, sum(1..5)=4 (NG)
        // -> 左端は 2
        assert_eq!(segtree.min_left(5, |s| s <= 3), 2);
        assert_eq!(segtree.min_left(5, |s| s <= 0), 5);
        assert_eq!(segtree.min_left(5, |s| s < 10), 0);
    }

    #[ignore]
    #[test]
    fn test_random_add_sum() {
        use rand::{Rng, SeedableRng, rngs::SmallRng};

        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.random_range(1..=20);
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
            let mut segtree = RangeAddRangeSumSegtree::<i64>::from_slice(&naive_vec);

            for _ in 0..100 {
                // 100 random operations per set
                let op_type = rng.random_range(0..6); // 6 operations: set, add, range_add, get, range_sum, all_sum

                match op_type {
                    0 => {
                        // set(p, x)
                        let p = rng.random_range(0..n);
                        let x = rng.random_range(-100..=100);
                        naive_vec[p] = x;
                        segtree.set(p, x);
                    }
                    1 => {
                        // add(p, x)
                        let p = rng.random_range(0..n);
                        let x = rng.random_range(-50..=50);
                        naive_vec[p] += x;
                        segtree.add(p, x);
                    }
                    2 => {
                        // range_add(range, x)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);

                        let x = rng.random_range(-50..=50);

                        for i in l..r {
                            naive_vec[i] += x;
                        }
                        segtree.range_add(l..r, x);
                    }
                    3 => {
                        // get(p)
                        let p = rng.random_range(0..n);
                        assert_eq!(segtree.get(p), naive_vec[p], "get({}) failed", p);
                    }
                    4 => {
                        // range_sum(range)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);

                        let expected_sum: i64 = naive_vec[l..r].iter().sum();
                        assert_eq!(
                            segtree.range_sum(l..r),
                            expected_sum,
                            "range_sum({}..{}) failed",
                            l,
                            r
                        );
                    }
                    5 => {
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
