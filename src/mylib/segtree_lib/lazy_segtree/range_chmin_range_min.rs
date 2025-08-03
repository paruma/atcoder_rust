use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_chmin_range_min::*;")]
pub mod range_chmin_range_min {
    // range chmax range max や range chmin range max なども同様に作れる
    use ac_library::lazysegtree::MapMonoid;
    use ac_library::LazySegtree;
    use ac_library::Min;
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::ops::RangeBounds;

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

    pub struct RangeChminRangeMinSegtree {
        segtree: LazySegtree<RangeChminRangeMin>,
        len: usize,
    }

    impl RangeChminRangeMinSegtree {
        pub fn new(xs: &[i64]) -> RangeChminRangeMinSegtree {
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

        pub fn apply_chmin(&mut self, p: usize, x: i64) {
            self.segtree.apply(p, x)
        }

        pub fn apply_range_chmin<R>(&mut self, range: R, x: i64)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, x)
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
        let mut segtree = RangeChminRangeMinSegtree::new(&xs);
        assert_eq!(segtree.get(0), 10);
        assert_eq!(segtree.get(2), 30);
        assert_eq!(segtree.get(4), 50);
    }

    #[test]
    fn test_set() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminRangeMinSegtree::new(&xs);
        segtree.set(0, 5);
        assert_eq!(segtree.to_vec(), vec![5, 20, 30, 40, 50]);
        segtree.set(4, 45);
        assert_eq!(segtree.to_vec(), vec![5, 20, 30, 40, 45]);
    }

    #[test]
    fn test_all_min() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminRangeMinSegtree::new(&xs);
        assert_eq!(segtree.all_min(), 10);
        segtree.set(0, 5);
        assert_eq!(segtree.all_min(), 5);
    }

    #[test]
    fn test_range_min() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminRangeMinSegtree::new(&xs);
        assert_eq!(segtree.range_min(1..4), 20);
        segtree.set(2, 15);
        assert_eq!(segtree.range_min(1..4), 15);
    }

    #[test]
    fn test_apply_chmin() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminRangeMinSegtree::new(&xs);
        segtree.apply_chmin(1, 15);
        assert_eq!(segtree.to_vec(), vec![10, 15, 30, 40, 50]);
        segtree.apply_chmin(1, 25); // No change
        assert_eq!(segtree.to_vec(), vec![10, 15, 30, 40, 50]);
    }

    #[test]
    fn test_apply_range_chmin() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminRangeMinSegtree::new(&xs);
        segtree.apply_range_chmin(2..5, 25);
        assert_eq!(segtree.to_vec(), vec![10, 20, 25, 25, 25]);
        segtree.apply_range_chmin(0..3, 5);
        assert_eq!(segtree.to_vec(), vec![5, 5, 5, 25, 25]);
    }

    #[test]
    fn test_to_vec() {
        let xs = vec![0, 1, 2, 3, 4, 5];
        let mut segtree = RangeChminRangeMinSegtree::new(&xs);
        assert_eq!(segtree.to_vec(), vec![0, 1, 2, 3, 4, 5]);
        segtree.apply_range_chmin(1..4, 100);
        assert_eq!(segtree.to_vec(), vec![0, 1, 2, 3, 4, 5]); // No change as 100 is greater
        segtree.apply_range_chmin(1..4, -1);
        assert_eq!(segtree.to_vec(), vec![0, -1, -1, -1, 4, 5]);
    }
}
