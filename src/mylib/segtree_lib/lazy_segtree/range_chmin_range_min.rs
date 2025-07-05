use cargo_snippet::snippet;

#[snippet(prefix = "use range_chmin_range_min::*;")]
pub mod range_chmin_range_min {
    // range chmax range max や range chmin range max なども同様に作れる
    use ac_library::lazysegtree::MapMonoid;
    use ac_library::Min;
    use std::convert::Infallible;
    

    pub struct RangeChminRangeMin(Infallible);
    impl MapMonoid for RangeChminRangeMin {
        type M = Min<usize>;
        type F = usize;
        fn identity_map() -> Self::F {
            usize::MAX
        }
        fn mapping(
            f: &Self::F,
            x: &<Self::M as ac_library::Monoid>::S,
        ) -> <Self::M as ac_library::Monoid>::S {
            *f.min(x)
        }
        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            *f.min(g)
        }
    }
}

#[cfg(test)]
pub mod test_range_chmin_range_min {
    use ac_library::LazySegtree;

    use super::range_chmin_range_min::RangeChminRangeMin;

    #[test]
    fn test_sample_of_lazy_segtree() {
        // range chmin range min の遅延セグ木の使用例
        let xs = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let mut segtree: LazySegtree<RangeChminRangeMin> = LazySegtree::from(xs);

        segtree.apply_range(3..7, 5); // [0, 1, 2, 5, 5, 5, 5, 7, 8, 9]
        assert_eq!(segtree.prod(2..5), 2);
        assert_eq!(segtree.prod(5..8), 5);
    }
}