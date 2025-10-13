use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use counts_fx::*;")]
pub mod counts_fx {
    use rustc_hash::FxHashMap;

    pub trait IteratorCountsFx: Iterator + Sized {
        fn counts_fx(self) -> FxHashMap<Self::Item, usize>
        where
            Self: Sized,
            Self::Item: Eq + std::hash::Hash,
        {
            let mut counts = FxHashMap::default();
            self.for_each(|item| *counts.entry(item).or_default() += 1);
            counts
        }
    }
    impl<T: Iterator> IteratorCountsFx for T {}
}

#[cfg(test)]
mod test {
    use rustc_hash::FxHashMap;

    use super::counts_fx::*;

    #[test]
    fn test_counts_fx() {
        let xs: Vec<i64> = vec![1, 3, 2, 3, 4];
        let expected = [(1, 1), (2, 1), (3, 2), (4, 1)]
            .iter()
            .copied()
            .collect::<FxHashMap<i64, usize>>();
        let actual = xs.iter().copied().counts_fx();

        assert_eq!(expected, actual);
    }
}
