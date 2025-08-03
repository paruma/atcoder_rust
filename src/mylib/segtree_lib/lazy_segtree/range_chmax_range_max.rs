use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_chmax_range_max::*;")]
pub mod range_chmax_range_max {
    use ac_library::lazysegtree::MapMonoid;
    use ac_library::LazySegtree;
    use ac_library::Max;
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::ops::RangeBounds;

    pub struct RangeChmaxRangeMax(Infallible);
    impl MapMonoid for RangeChmaxRangeMax {
        type M = Max<i64>;
        type F = i64;
        fn identity_map() -> Self::F {
            i64::MIN
        }
        fn mapping(
            f: &Self::F,
            x: &<Self::M as ac_library::Monoid>::S,
        ) -> <Self::M as ac_library::Monoid>::S {
            (*f).max(*x)
        }
        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            (*f).max(*g)
        }
    }

    pub struct RangeChmaxRangeMaxSegtree {
        segtree: LazySegtree<RangeChmaxRangeMax>,
        len: usize,
    }

    impl RangeChmaxRangeMaxSegtree {
        pub fn new(xs: &[i64]) -> RangeChmaxRangeMaxSegtree {
            let len = xs.len();
            RangeChmaxRangeMaxSegtree {
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

        pub fn range_max<R>(&mut self, range: R) -> i64
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range)
        }

        pub fn all_max(&self) -> i64 {
            self.segtree.all_prod()
        }

        pub fn apply_chmax(&mut self, p: usize, x: i64) {
            self.segtree.apply(p, x)
        }

        pub fn apply_range_chmax<R>(&mut self, range: R, x: i64)
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
pub mod test_range_chmax_range_max {
    use super::range_chmax_range_max::RangeChmaxRangeMaxSegtree;

    #[test]
    fn test_new_and_get() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChmaxRangeMaxSegtree::new(&xs);
        assert_eq!(segtree.get(0), 10);
        assert_eq!(segtree.get(2), 30);
        assert_eq!(segtree.get(4), 50);
    }

    #[test]
    fn test_set() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChmaxRangeMaxSegtree::new(&xs);
        segtree.set(0, 5);
        assert_eq!(segtree.to_vec(), vec![5, 20, 30, 40, 50]);
        segtree.set(4, 45);
        assert_eq!(segtree.to_vec(), vec![5, 20, 30, 40, 45]);
    }

    #[test]
    fn test_all_max() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChmaxRangeMaxSegtree::new(&xs);
        assert_eq!(segtree.all_max(), 50);
        segtree.set(0, 55);
        assert_eq!(segtree.all_max(), 55);
    }

    #[test]
    fn test_range_max() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChmaxRangeMaxSegtree::new(&xs);
        assert_eq!(segtree.range_max(1..4), 40);
        segtree.set(2, 45);
        assert_eq!(segtree.range_max(1..4), 45);
    }

    #[test]
    fn test_apply_chmax() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChmaxRangeMaxSegtree::new(&xs);
        segtree.apply_chmax(1, 25);
        assert_eq!(segtree.to_vec(), vec![10, 25, 30, 40, 50]);
        segtree.apply_chmax(1, 15); // No change
        assert_eq!(segtree.to_vec(), vec![10, 25, 30, 40, 50]);
    }

    #[test]
    fn test_apply_range_chmax() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChmaxRangeMaxSegtree::new(&xs);
        segtree.apply_range_chmax(0..3, 35);
        assert_eq!(segtree.to_vec(), vec![35, 35, 35, 40, 50]);
        segtree.apply_range_chmax(2..5, 60);
        assert_eq!(segtree.to_vec(), vec![35, 35, 60, 60, 60]);
    }

    #[test]
    fn test_to_vec() {
        let xs = vec![0, 1, 2, 3, 4, 5];
        let mut segtree = RangeChmaxRangeMaxSegtree::new(&xs);
        assert_eq!(segtree.to_vec(), vec![0, 1, 2, 3, 4, 5]);
        segtree.apply_range_chmax(1..4, -1);
        assert_eq!(segtree.to_vec(), vec![0, 1, 2, 3, 4, 5]); // No change as -1 is smaller
        segtree.apply_range_chmax(1..4, 100);
        assert_eq!(segtree.to_vec(), vec![0, 100, 100, 100, 4, 5]);
    }
}
