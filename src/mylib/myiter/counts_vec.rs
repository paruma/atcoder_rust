use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use counts_vec::*;")]
pub mod counts_vec {
    pub trait IteratorCountsVec: Iterator<Item = usize> + Sized {
        fn counts_vec(self, size: usize) -> Vec<i64> {
            let mut counts = vec![0; size];
            self.for_each(|item| counts[item] += 1);
            counts
        }
    }
    impl<T: Iterator<Item = usize>> IteratorCountsVec for T {}
}

#[cfg(test)]
mod test {
    use super::counts_vec::*;

    #[test]
    fn test_counts_vec() {
        let xs: Vec<usize> = vec![1, 3, 2, 3, 4];
        let expected = vec![0, 1, 1, 2, 1];
        let actual = xs.iter().copied().counts_vec(5);

        assert_eq!(expected, actual);
    }
}
