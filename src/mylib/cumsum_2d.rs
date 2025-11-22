use cargo_snippet::snippet;

#[snippet(prefix = "use cumsum_2d::*;")]
#[allow(clippy::module_inception)]
pub mod cumsum_2d {
    use std::ops::{Bound, Range, RangeBounds};

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct CumSum2D {
        pub cumsum: Vec<Vec<i64>>,
    }

    impl CumSum2D {
        pub fn new(xss: &[Vec<i64>]) -> CumSum2D {
            if xss.is_empty() {
                return CumSum2D {
                    cumsum: vec![vec![0]],
                };
            }

            let height = xss.len();
            let width = xss[0].len();
            let mut cumsum = vec![vec![0; width + 1]; height + 1];
            for y in 1..height + 1 {
                for x in 1..width + 1 {
                    cumsum[y][x] = cumsum[y - 1][x] + cumsum[y][x - 1] - cumsum[y - 1][x - 1]
                        + xss[y - 1][x - 1];
                }
            }
            CumSum2D { cumsum }
        }

        pub fn rect_sum(
            &self,
            x_range: impl RangeBounds<usize>,
            y_range: impl RangeBounds<usize>,
        ) -> i64 {
            let y_len = self.cumsum.len() - 1;
            let x_len = self.cumsum[0].len() - 1;
            let x_range = open(x_range, x_len);
            let y_range = open(y_range, y_len);

            let x1 = x_range.start;
            let x2 = x_range.end;
            let y1 = y_range.start;
            let y2 = y_range.end;
            self.cumsum[y2][x2] - self.cumsum[y2][x1] - self.cumsum[y1][x2] + self.cumsum[y1][x1]
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
mod test_cumsum_2d {
    use super::cumsum_2d::*;

    #[test]
    fn test_cumsum_2d_normal() {
        // [1 2]
        // [4 5]
        let xss = vec![vec![1, 2], vec![4, 5]];
        let cumsum = CumSum2D::new(&xss);
        assert_eq!(
            cumsum.cumsum,
            vec![vec![0, 0, 0], vec![0, 1, 3], vec![0, 5, 12]]
        );
        assert_eq!(cumsum.rect_sum(0..1, 0..2), xss[0][0] + xss[1][0]);
        assert_eq!(
            cumsum.rect_sum(0..=1, 0..=1),
            xss[0][0] + xss[0][1] + xss[1][0] + xss[1][1]
        );
        assert_eq!(cumsum.rect_sum(1..1, 1..1), 0);
        assert_eq!(cumsum.rect_sum(.., 1..=1), xss[1][0] + xss[1][1]);
    }

    #[test]
    fn test_cumsum_2d_empty() {
        let xss = vec![];
        let cumsum = CumSum2D::new(&xss);
        assert_eq!(cumsum.cumsum, vec![vec![0]]);
        assert_eq!(cumsum.rect_sum(0..0, 0..0), 0);
    }
}
