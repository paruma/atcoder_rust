use crate::ab_group::ab_group::AbGroup;
use cargo_snippet::snippet;

#[snippet(prefix = "use cumsum_2d_arbitrary::*;", include = "ab_group")]
#[allow(clippy::module_inception)]
pub mod cumsum_2d_arbitrary {
    use super::AbGroup;
    use std::ops::{Bound, Range, RangeBounds};

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct CumSum2dArbitrary<G: AbGroup> {
        pub cumsum: Vec<Vec<G::S>>,
    }

    impl<G: AbGroup> CumSum2dArbitrary<G> {
        pub fn new(xss: &[Vec<G::S>]) -> CumSum2dArbitrary<G> {
            if xss.is_empty() {
                return CumSum2dArbitrary {
                    cumsum: vec![vec![G::zero()]],
                };
            }

            let height = xss.len();
            let width = xss[0].len();
            let mut cumsum = vec![vec![G::zero(); width + 1]; height + 1];
            for y in 1..height + 1 {
                for x in 1..width + 1 {
                    // cumsum[y][x] = cumsum[y-1][x] + cumsum[y][x-1] - cumsum[y-1][x-1] + val
                    let top = &cumsum[y - 1][x];
                    let left = &cumsum[y][x - 1];
                    let top_left = &cumsum[y - 1][x - 1];
                    let val = &xss[y - 1][x - 1];

                    let term1 = G::add(top, left);
                    let term2 = G::sub(&term1, top_left);
                    cumsum[y][x] = G::add(&term2, val);
                }
            }
            CumSum2dArbitrary { cumsum }
        }

        pub fn rect_sum(
            &self,
            y_range: impl RangeBounds<usize>,
            x_range: impl RangeBounds<usize>,
        ) -> G::S {
            let y_len = self.cumsum.len() - 1;
            let x_len = self.cumsum[0].len() - 1;
            let y_range = open(y_range, y_len);
            let x_range = open(x_range, x_len);

            let y1 = y_range.start;
            let y2 = y_range.end;
            let x1 = x_range.start;
            let x2 = x_range.end;

            // S[y2][x2] - S[y2][x1] - S[y1][x2] + S[y1][x1]
            let bottom_right = &self.cumsum[y2][x2];
            let bottom_left = &self.cumsum[y2][x1];
            let top_right = &self.cumsum[y1][x2];
            let top_left = &self.cumsum[y1][x1];

            let term1 = G::sub(bottom_right, bottom_left);
            let term2 = G::sub(top_right, top_left);
            G::sub(&term1, &term2)
        }
    }

    fn open(range: impl RangeBounds<usize>, len: usize) -> Range<usize> {
        let begin = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x + 1,
        };
        let end = match range.end_bound() {
            Bound::Excluded(&x) => x,
            Bound::Included(&x) => x + 1,
            Bound::Unbounded => len,
        };
        begin..end
    }
}

#[cfg(test)]
mod tests_cumsum_2d_arbitrary {
    use super::cumsum_2d_arbitrary::*;
    use crate::ab_group::ab_group::AdditiveAbGroup;
    use std::ops::Bound;

    #[test]
    fn test_cumsum_2d_arbitrary_additive() {
        type G = AdditiveAbGroup<i32>;
        // [1 2]
        // [4 5]
        let xss = vec![vec![1, 2], vec![4, 5]];
        let cumsum = CumSum2dArbitrary::<G>::new(&xss);

        assert_eq!(cumsum.rect_sum(0..2, 0..1), 1 + 4);
        assert_eq!(cumsum.rect_sum(.., ..), 1 + 2 + 4 + 5);

        // Custom bounds
        // Excluded start
        assert_eq!(
            cumsum.rect_sum((Bound::Excluded(0), Bound::Excluded(2)), ..),
            4 + 5
        );
        // Included end
        assert_eq!(
            cumsum.rect_sum(.., (Bound::Included(0), Bound::Included(0))),
            1 + 4
        );
    }

    #[test]
    fn test_cumsum_2d_arbitrary_empty() {
        type G = AdditiveAbGroup<i32>;
        let xss: Vec<Vec<i32>> = vec![];
        let cumsum = CumSum2dArbitrary::<G>::new(&xss);
        assert_eq!(cumsum.rect_sum(.., ..), 0);
    }
}
