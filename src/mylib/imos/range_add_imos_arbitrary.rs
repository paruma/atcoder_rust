use cargo_snippet::snippet;

#[snippet(prefix = "use range_add_imos_arbitrary::*;", include = "ab_group")]
#[allow(clippy::module_inception)]
pub mod range_add_imos_arbitrary {
    use crate::ab_group::ab_group::AbGroup;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct RangeAddImosArbitrary<G: AbGroup> {
        n: usize,
        diff: Vec<G::S>,
    }

    impl<G: AbGroup> RangeAddImosArbitrary<G> {
        pub fn new(n: usize) -> Self {
            let mut diff = Vec::with_capacity(n + 1);
            for _ in 0..=n {
                diff.push(G::zero());
            }
            Self { n, diff }
        }

        pub fn range_add(&mut self, range: impl std::ops::RangeBounds<usize>, x: G::S) {
            let range = open_range_bounds(range, self.n);
            let l = range.start;
            let r = range.end;

            assert!(l <= r && r <= self.n);

            self.diff[l] = G::add(&self.diff[l], &x);
            self.diff[r] = G::sub(&self.diff[r], &x);
        }

        pub fn to_vec(mut self) -> Vec<G::S> {
            if self.n == 0 {
                return Vec::new();
            }
            for i in 1..self.n {
                self.diff[i] = G::add(&self.diff[i - 1], &self.diff[i]);
            }
            self.diff.truncate(self.n);
            self.diff
        }

        pub fn add(&mut self, p: usize, x: G::S) {
            self.range_add(p..(p + 1), x);
        }

        pub fn from_slice(xs: &[G::S]) -> Self {
            let n = xs.len();
            let mut diff = Vec::with_capacity(n + 1);
            if n == 0 {
                diff.push(G::zero());
            } else {
                diff.push(xs[0].clone());
                for i in 1..n {
                    diff.push(G::sub(&xs[i], &xs[i - 1]));
                }
                diff.push(G::neg(&xs[n - 1]));
            }
            Self { n, diff }
        }
    }

    fn open_range_bounds(
        range: impl std::ops::RangeBounds<usize>,
        len: usize,
    ) -> std::ops::Range<usize> {
        use std::ops::Bound::{Excluded, Included, Unbounded};
        let l = match range.start_bound() {
            Unbounded => 0,
            Included(&x) => x,
            Excluded(&x) => x + 1,
        };
        let r = match range.end_bound() {
            Unbounded => len,
            Included(&x) => x + 1,
            Excluded(&x) => x,
        };
        l..r
    }
}

#[cfg(test)]
mod tests {
    use super::range_add_imos_arbitrary::*;
    use crate::ab_group::ab_group::AdditiveAbGroup;
    use rand::{Rng, SeedableRng};

    #[test]
    fn test_range_add_imos_arbitrary_basic() {
        type G = AdditiveAbGroup<i64>;
        let n = 10;
        let mut imos = RangeAddImosArbitrary::<G>::new(n);
        assert_eq!(imos.clone().to_vec(), vec![0; n]);
        imos.range_add(2..5, 5);
        assert_eq!(imos.clone().to_vec(), vec![0, 0, 5, 5, 5, 0, 0, 0, 0, 0]);
        imos.range_add(0..3, -3);
        assert_eq!(imos.clone().to_vec(), vec![-3, -3, 2, 5, 5, 0, 0, 0, 0, 0]);
        imos.range_add(8..10, 10);
        assert_eq!(
            imos.clone().to_vec(),
            vec![-3, -3, 2, 5, 5, 0, 0, 0, 10, 10]
        );
        imos.range_add(5..5, 100);
        assert_eq!(
            imos.clone().to_vec(),
            vec![-3, -3, 2, 5, 5, 0, 0, 0, 10, 10]
        );
        imos.range_add(1..=3, 1);
        assert_eq!(
            imos.clone().to_vec(),
            vec![-3, -2, 3, 6, 5, 0, 0, 0, 10, 10]
        );
        imos.add(0, 10);
        assert_eq!(imos.clone().to_vec(), vec![7, -2, 3, 6, 5, 0, 0, 0, 10, 10]);

        // Cover open_range_bounds branches
        use std::ops::Bound;
        let mut imos = RangeAddImosArbitrary::<G>::new(5);
        // Excluded start, Excluded end
        imos.range_add((Bound::Excluded(0), Bound::Excluded(4)), 1);
        assert_eq!(imos.to_vec(), vec![0, 1, 1, 1, 0]);
    }

    #[test]
    fn test_empty() {
        type G = AdditiveAbGroup<i64>;
        let mut imos = RangeAddImosArbitrary::<G>::new(0);
        assert_eq!(imos.clone().to_vec(), Vec::<i64>::new());
        imos.range_add(0..0, 10);
        assert_eq!(imos.to_vec(), Vec::<i64>::new());
    }

    #[test]
    fn test_from_slice() {
        type G = AdditiveAbGroup<i64>;
        let xs = vec![1, 2, 3, 4, 5];
        let imos = RangeAddImosArbitrary::<G>::from_slice(&xs);
        assert_eq!(imos.to_vec(), xs);

        let empty_xs = Vec::<i64>::new();
        let empty_imos = RangeAddImosArbitrary::<G>::from_slice(&empty_xs);
        assert_eq!(empty_imos.to_vec(), empty_xs);

        let single_xs = vec![10];
        let single_imos = RangeAddImosArbitrary::<G>::from_slice(&single_xs);
        assert_eq!(single_imos.to_vec(), single_xs);
    }

    #[test]
    fn test_random_range_add_imos_arbitrary() {
        type G = AdditiveAbGroup<i64>;
        let mut rng = rand::rngs::SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.random_range(1..=100);
            let mut naive_vec = vec![0i64; n];
            let mut imos = RangeAddImosArbitrary::<G>::new(n);

            for _ in 0..100 {
                let l = rng.random_range(0..=n);
                let r = rng.random_range(l..=n);
                let x = rng.random_range(-1000..=1000);

                for i in l..r {
                    naive_vec[i] += x;
                }
                imos.range_add(l..r, x);
            }
            assert_eq!(imos.to_vec(), naive_vec);
        }
    }
}
