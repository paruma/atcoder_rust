use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_chmin_range_min::*;")]
pub mod range_chmin_range_min {
    // range chmax range max や range chmin range max なども同様に作れる
    use ac_library::LazySegtree;
    use ac_library::Min;
    use ac_library::lazysegtree::MapMonoid;
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::ops::RangeBounds;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeChminRangeMin(Infallible);
    impl MapMonoid for RangeChminRangeMin {
        type M = Min<i64>;
        type F = i64;
        fn identity_map() -> Self::F {
            i64::MAX
        }
        fn mapping(
            f: &Self::F,
            x: &<Self::M as ac_library::Monoid>::S,
        ) -> <Self::M as ac_library::Monoid>::S {
            (*f).min(*x)
        }
        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            (*f).min(*g)
        }
    }

    #[derive(Clone)]
    pub struct RangeChminRangeMinSegtree {
        segtree: LazySegtree<RangeChminRangeMin>,
        len: usize,
    }

    impl RangeChminRangeMinSegtree {
                pub fn new(n: usize) -> Self {
            let xs = vec![0; n];
            Self::from_slice(&xs)
        }

        pub fn from_slice(xs: &[i64]) -> RangeChminRangeMinSegtree {
            let len = xs.len();
            RangeChminRangeMinSegtree {
                segtree: LazySegtree::from(xs.to_vec()),
                len,
            }
        }

        pub fn set(&mut self, p: usize, x: i64) {
            self.segtree.set(p, x);
        }

        pub fn get(&mut self, p: usize) -> i64 {
            self.segtree.get(p)
        }

        pub fn range_min<R>(&mut self, range: R) -> i64
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range)
        }

        pub fn all_min(&self) -> i64 {
            self.segtree.all_prod()
        }

        pub fn chmin(&mut self, p: usize, x: i64) {
            self.segtree.apply(p, x)
        }

        pub fn range_chmin<R>(&mut self, range: R, x: i64)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, x)
        }

        /// 左端 `l` を固定し、区間 `[l, r)` での最小値が述語 `g` を満たすような最大の `r` を返します。
        ///
        /// # 計算量
        /// O(log N)
        pub fn max_right<G>(&mut self, l: usize, g: G) -> usize
        where
            G: Fn(i64) -> bool,
        {
            self.segtree.max_right(l, g)
        }

        /// 右端 `r` を固定し、区間 `[l, r)` での最小値が述語 `g` を満たすような最小の `l` を返します。
        ///
        /// # 計算量
        /// O(log N)
        pub fn min_left<G>(&mut self, r: usize, g: G) -> usize
        where
            G: Fn(i64) -> bool,
        {
            self.segtree.min_left(r, g)
        }

        
        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            self.len
        }
        pub fn to_vec(&mut self) -> Vec<i64> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}

#[cfg(test)]
pub mod test_range_chmin_range_min {
    use super::range_chmin_range_min::RangeChminRangeMinSegtree;

    #[test]
    fn test_new_and_get() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminRangeMinSegtree::from_slice(&xs);
        assert_eq!(segtree.get(0), 10);
        assert_eq!(segtree.get(2), 30);
        assert_eq!(segtree.get(4), 50);
    }

    #[test]
    fn test_set() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminRangeMinSegtree::from_slice(&xs);
        segtree.set(0, 5);
        assert_eq!(segtree.to_vec(), vec![5, 20, 30, 40, 50]);
        segtree.set(4, 45);
        assert_eq!(segtree.to_vec(), vec![5, 20, 30, 40, 45]);
    }

    #[test]
    fn test_all_min() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminRangeMinSegtree::from_slice(&xs);
        assert_eq!(segtree.all_min(), 10);
        segtree.set(0, 5);
        assert_eq!(segtree.all_min(), 5);
    }

    #[test]
    fn test_range_min() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminRangeMinSegtree::from_slice(&xs);
        assert_eq!(segtree.range_min(1..4), 20);
        segtree.set(2, 15);
        assert_eq!(segtree.range_min(1..4), 15);
    }

    #[test]
    fn test_chmin() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminRangeMinSegtree::from_slice(&xs);
        segtree.chmin(1, 15);
        assert_eq!(segtree.to_vec(), vec![10, 15, 30, 40, 50]);
        segtree.chmin(1, 25); // No change
        assert_eq!(segtree.to_vec(), vec![10, 15, 30, 40, 50]);
    }

    #[test]
    fn test_range_chmin() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminRangeMinSegtree::from_slice(&xs);
        segtree.range_chmin(2..5, 25);
        assert_eq!(segtree.to_vec(), vec![10, 20, 25, 25, 25]);
        segtree.range_chmin(0..3, 5);
        assert_eq!(segtree.to_vec(), vec![5, 5, 5, 25, 25]);
    }

    #[test]
    fn test_max_right_min_left() {
        let xs = vec![5, 4, 3, 2, 1];
        let mut segtree = RangeChminRangeMinSegtree::from_slice(&xs);
        assert_eq!(segtree.max_right(0, |m| m >= 3), 3);
        assert_eq!(segtree.min_left(5, |m| m >= 3), 5);
        assert_eq!(segtree.min_left(3, |m| m >= 3), 0);
    }

    #[test]
    fn test_to_vec() {
        let xs = vec![0, 1, 2, 3, 4, 5];
        let mut segtree = RangeChminRangeMinSegtree::from_slice(&xs);
        assert_eq!(segtree.to_vec(), vec![0, 1, 2, 3, 4, 5]);
        segtree.range_chmin(1..4, 100);
        assert_eq!(segtree.to_vec(), vec![0, 1, 2, 3, 4, 5]); // No change as 100 is greater
        segtree.range_chmin(1..4, -1);
        assert_eq!(segtree.to_vec(), vec![0, -1, -1, -1, 4, 5]);
    }

    #[ignore]
    #[test]
    fn test_random_chmin_min() {
        use rand::{Rng, SeedableRng, rngs::SmallRng};

        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.random_range(1..=20);
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
            let mut segtree = RangeChminRangeMinSegtree::from_slice(&naive_vec);

            for _ in 0..100 {
                // 100 random operations per set
                let op_type = rng.random_range(0..5); // 5 operations

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
                        // range_chmin(range, x)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);

                        let x = rng.random_range(-50..=50);

                        for i in l..r {
                            naive_vec[i] = naive_vec[i].min(x);
                        }
                        segtree.range_chmin(l..r, x);
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
