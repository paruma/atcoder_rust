use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_update_range_xor::*;")]
pub mod range_update_range_xor {
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::marker::PhantomData;
    use std::ops::{BitXor, RangeBounds};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeXor<T> {
        pub xor: T,
        pub len: i64,
    }
    impl<T> RangeXor<T> {
        pub fn unit(x: T) -> RangeXor<T> {
            RangeXor { xor: x, len: 1 }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ValueLenXor<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for ValueLenXor<T>
    where
        T: Copy + BitXor<Output = T> + From<u8>,
    {
        type S = RangeXor<T>;
        fn identity() -> RangeXor<T> {
            RangeXor {
                xor: 0.into(),
                len: 0,
            }
        }
        fn binary_operation(a: &RangeXor<T>, b: &RangeXor<T>) -> RangeXor<T> {
            RangeXor {
                xor: a.xor ^ b.xor,
                len: a.len + b.len,
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeUpdateRangeXor<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for RangeUpdateRangeXor<T>
    where
        T: Copy + BitXor<Output = T> + From<u8>,
    {
        type M = ValueLenXor<T>;
        type F = Option<T>; // None means no update, Some(val) means update to val

        fn identity_map() -> Option<T> {
            None
        }
        fn composition(a: &Option<T>, b: &Option<T>) -> Option<T> {
            if a.is_some() { *a } else { *b }
        }

        fn mapping(f: &Option<T>, x: &RangeXor<T>) -> RangeXor<T> {
            match f {
                Some(val) => RangeXor {
                    xor: if x.len % 2 == 1 { *val } else { 0.into() },
                    len: x.len,
                },
                None => *x,
            }
        }
    }

    #[derive(Clone)]
    pub struct RangeUpdateRangeXorSegtree<T>
    where
        T: Copy + BitXor<Output = T> + From<u8>,
    {
        segtree: LazySegtree<RangeUpdateRangeXor<T>>,
        len: usize,
    }

    impl<T> RangeUpdateRangeXorSegtree<T>
    where
        T: Copy + BitXor<Output = T> + From<u8>,
    {
        pub fn new(xs: &[T]) -> RangeUpdateRangeXorSegtree<T> {
            let xs = xs.iter().copied().map(RangeXor::unit).collect_vec();
            let len = xs.len();
            RangeUpdateRangeXorSegtree {
                segtree: LazySegtree::from(xs),
                len,
            }
        }

        pub fn set(&mut self, p: usize, x: T) {
            self.segtree.set(p, RangeXor::unit(x));
        }

        pub fn get(&mut self, p: usize) -> T {
            self.segtree.get(p).xor
        }

        pub fn range_xor<R>(&mut self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).xor
        }

        pub fn all_xor(&self) -> T {
            self.segtree.all_prod().xor
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

#[allow(clippy::identity_op)]
#[cfg(test)]
pub mod test_range_update_range_xor {

    use super::range_update_range_xor::RangeUpdateRangeXorSegtree;

    #[test]
    fn test_new_and_get() {
        let xs = vec![10u64, 20, 30, 40, 50];
        let mut segtree = RangeUpdateRangeXorSegtree::<u64>::new(&xs);
        assert_eq!(segtree.get(0), 10);
        assert_eq!(segtree.get(2), 30);
        assert_eq!(segtree.get(4), 50);
    }

    #[test]
    fn test_set() {
        let xs = vec![10u64, 20, 30, 40, 50];
        let mut segtree = RangeUpdateRangeXorSegtree::<u64>::new(&xs);
        segtree.set(0, 5);
        assert_eq!(segtree.to_vec(), vec![5, 20, 30, 40, 50]);
        segtree.set(4, 45);
        assert_eq!(segtree.to_vec(), vec![5, 20, 30, 40, 45]);
    }

    #[test]
    fn test_all_xor() {
        let xs = vec![10u64, 20, 30, 40, 50];
        let mut segtree = RangeUpdateRangeXorSegtree::<u64>::new(&xs);
        assert_eq!(segtree.all_xor(), 10 ^ 20 ^ 30 ^ 40 ^ 50);
        segtree.set(0, 5);
        assert_eq!(segtree.all_xor(), 5 ^ 20 ^ 30 ^ 40 ^ 50);
    }

    #[test]
    fn test_range_xor() {
        let xs = vec![10u64, 20, 30, 40, 50];
        let mut segtree = RangeUpdateRangeXorSegtree::<u64>::new(&xs);
        assert_eq!(segtree.range_xor(1..4), 20 ^ 30 ^ 40);
        segtree.set(2, 15);
        assert_eq!(segtree.range_xor(1..4), 20 ^ 15 ^ 40);
    }

    #[test]
    fn test_apply_update() {
        let xs = vec![10u64, 20, 30, 40, 50];
        let mut segtree = RangeUpdateRangeXorSegtree::<u64>::new(&xs);
        segtree.apply_update(1, 5);
        assert_eq!(segtree.to_vec(), vec![10, 5, 30, 40, 50]);
        segtree.apply_update(1, 15);
        assert_eq!(segtree.to_vec(), vec![10, 15, 30, 40, 50]);
    }

    #[test]
    fn test_apply_range_update() {
        let xs = vec![10u64, 20, 30, 40, 50];
        let mut segtree = RangeUpdateRangeXorSegtree::<u64>::new(&xs);
        segtree.apply_range_update(1..4, 5);
        assert_eq!(segtree.to_vec(), vec![10, 5, 5, 5, 50]);
        segtree.apply_range_update(1..4, 20);
        assert_eq!(segtree.to_vec(), vec![10, 20, 20, 20, 50]);
        segtree.apply_range_update(0..3, 100);
        assert_eq!(segtree.to_vec(), vec![100, 100, 100, 20, 50]);
    }

    #[test]
    fn test_to_vec() {
        let xs = vec![0u64, 1, 2, 3, 4, 5];
        let mut segtree = RangeUpdateRangeXorSegtree::<u64>::new(&xs);
        assert_eq!(segtree.to_vec(), vec![0, 1, 2, 3, 4, 5]);
        segtree.apply_range_update(1..4, 10);
        assert_eq!(segtree.to_vec(), vec![0, 10, 10, 10, 4, 5]);
    }

    #[test]
    fn test_with_zeros() {
        let xs = vec![1u64, 2, 0, 4, 5];
        let mut segtree = RangeUpdateRangeXorSegtree::<u64>::new(&xs);

        // Initial state
        assert_eq!(segtree.all_xor(), 1 ^ 2 ^ 0 ^ 4 ^ 5);
        assert_eq!(segtree.range_xor(0..2), 1 ^ 2);
        assert_eq!(segtree.range_xor(0..3), 1 ^ 2 ^ 0);
        assert_eq!(segtree.range_xor(2..5), 0 ^ 4 ^ 5);

        // Update a non-zero range to include zero
        segtree.apply_range_update(0..2, 0);
        assert_eq!(segtree.to_vec(), vec![0, 0, 0, 4, 5]);
        assert_eq!(segtree.all_xor(), 0 ^ 0 ^ 0 ^ 4 ^ 5);
        assert_eq!(segtree.range_xor(0..2), 0 ^ 0);
        assert_eq!(segtree.range_xor(3..5), 4 ^ 5);

        // Update a zero to a non-zero value
        segtree.set(2, 3);
        assert_eq!(segtree.to_vec(), vec![0, 0, 3, 4, 5]);
        assert_eq!(segtree.all_xor(), 0 ^ 0 ^ 3 ^ 4 ^ 5);
        assert_eq!(segtree.range_xor(2..5), 3 ^ 4 ^ 5);

        // Update a range with zero to a non-zero value
        segtree.apply_range_update(0..2, 1);
        assert_eq!(segtree.to_vec(), vec![1, 1, 3, 4, 5]);
        assert_eq!(segtree.all_xor(), 1 ^ 1 ^ 3 ^ 4 ^ 5);
    }

    #[ignore]
    #[test]
    fn test_random_update() {
        use rand::{Rng, SeedableRng, rngs::SmallRng};
        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.random_range(1..=20);
            let mut naive_vec: Vec<u64> = (0..n).map(|_| rng.random_range(0..=100)).collect();
            let mut segtree = RangeUpdateRangeXorSegtree::<u64>::new(&naive_vec);

            for _ in 0..100 {
                // 100 random operations per set
                let op_type = rng.random_range(0..6);

                match op_type {
                    0 => {
                        // set(p, x)
                        let p = rng.random_range(0..n);
                        let x = rng.random_range(0..=100);
                        naive_vec[p] = x;
                        segtree.set(p, x);
                    }
                    1 => {
                        // apply_range_update(range, x)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);

                        let x = rng.random_range(0..=100);

                        for i in l..r {
                            naive_vec[i] = x;
                        }
                        segtree.apply_range_update(l..r, x);
                    }
                    2 => {
                        // get(p)
                        let p = rng.random_range(0..n);
                        assert_eq!(segtree.get(p), naive_vec[p], "get({}) failed", p);
                    }
                    3 => {
                        // range_xor(range)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);

                        let mut expected_xor: u64 = 0;
                        for &val in naive_vec[l..r].iter() {
                            expected_xor ^= val;
                        }
                        assert_eq!(
                            segtree.range_xor(l..r),
                            expected_xor,
                            "range_xor({}..{}) failed",
                            l,
                            r
                        );
                    }
                    4 => {
                        // all_xor()
                        let mut expected_xor: u64 = 0;
                        for &val in naive_vec.iter() {
                            expected_xor ^= val;
                        }
                        assert_eq!(segtree.all_xor(), expected_xor, "all_xor() failed");
                    }
                    5 => {
                        // apply_update(p, x)
                        let p = rng.random_range(0..n);
                        let x = rng.random_range(0..=100);
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
