use cargo_snippet::snippet;

#[snippet(prefix = "use cumsum_2d::*;")]
pub mod cumsum_2d {
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

        pub fn rect_sum(&self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> i64 {
            // [x1, x2) × [y1, y2) の範囲で総和を求める
            self.cumsum[y2][x2] - self.cumsum[y2][x1] - self.cumsum[y1][x2] + self.cumsum[y1][x1]
        }
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
        assert_eq!(cumsum.rect_sum((0, 0), (1, 2)), xss[0][0] + xss[1][0]);
        assert_eq!(
            cumsum.rect_sum((0, 0), (2, 2)),
            xss[0][0] + xss[0][1] + xss[1][0] + xss[1][1]
        );
        assert_eq!(cumsum.rect_sum((1, 1), (1, 1)), 0);
    }

    #[test]
    fn test_cumsum_2d_empty() {
        let xss = vec![];
        let cumsum = CumSum2D::new(&xss);
        assert_eq!(cumsum.cumsum, vec![vec![0]]);
        assert_eq!(cumsum.rect_sum((0, 0), (0, 0)), 0);
    }
}
