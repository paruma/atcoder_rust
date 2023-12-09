use cargo_snippet::snippet;

#[snippet(prefix = "use cumsum::*;")]
pub mod cumsum {
    pub struct CumSum {
        pub cumsum: Vec<i64>,
    }

    impl CumSum {
        /// 計算量: O(|xs|)
        pub fn new(xs: &[i64]) -> CumSum {
            let mut cumsum = vec![0; xs.len() + 1];
            for i in 1..xs.len() + 1 {
                cumsum[i] = cumsum[i - 1] + xs[i - 1];
            }
            CumSum { cumsum }
        }
        /// 計算量: O(1)
        pub fn get_interval_sum(&self, begin: usize, end: usize) -> i64 {
            // [begin, end) の間で総和を求める
            self.cumsum[end] - self.cumsum[begin]
        }
    }
}

#[snippet(prefix = "use cumsum_2d::*;")]
pub mod cumsum_2d {
    pub struct CumSum2D {
        pub cumsum: Vec<Vec<i64>>,
    }

    impl CumSum2D {
        pub fn new(xss: &[Vec<i64>]) -> CumSum2D {
            if xss.is_empty() {
                return CumSum2D { cumsum: vec![vec![0]] };
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

        pub fn get_rect_sum(&self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> i64 {
            // [x1, x2) × [y1, y2) の範囲で総和を求める
            self.cumsum[y2][x2] - self.cumsum[y2][x1] - self.cumsum[y1][x2] + self.cumsum[y1][x1]
        }
    }
}

#[cfg(test)]
mod test {

    use super::cumsum::*;
    use super::cumsum_2d::*;

    #[test]
    fn test_cumsum_normal() {
        let xs = vec![1, 2, 3, 4];
        let cumsum = CumSum::new(&xs);
        assert_eq!(cumsum.cumsum, vec![0, 1, 3, 6, 10]);
        assert_eq!(cumsum.get_interval_sum(1, 3), xs[1] + xs[2]);
        assert_eq!(cumsum.get_interval_sum(2, 4), xs[2] + xs[3]);
        assert_eq!(cumsum.get_interval_sum(2, 2), 0);
    }

    #[test]
    fn test_cumsum_empty() {
        let xs = vec![];
        let cumsum = CumSum::new(&xs);
        assert_eq!(cumsum.cumsum, vec![0]);
        assert_eq!(cumsum.get_interval_sum(0, 0), 0)
    }

    #[test]
    fn test_cumsum_2d_normal() {
        // [1 2]
        // [4 5]
        let xss = vec![vec![1, 2], vec![4, 5]];
        let cumsum = CumSum2D::new(&xss);
        assert_eq!(cumsum.cumsum, vec![vec![0, 0, 0], vec![0, 1, 3], vec![0, 5, 12]]);
        assert_eq!(cumsum.get_rect_sum((0, 0), (1, 2)), xss[0][0] + xss[1][0]);
        assert_eq!(
            cumsum.get_rect_sum((0, 0), (2, 2)),
            xss[0][0] + xss[0][1] + xss[1][0] + xss[1][1]
        );
        assert_eq!(cumsum.get_rect_sum((1, 1), (1, 1)), 0);
    }

    #[test]
    fn test_cumsum_2d_empty() {
        let xss = vec![];
        let cumsum = CumSum2D::new(&xss);
        assert_eq!(cumsum.cumsum, vec![vec![0]]);
        assert_eq!(cumsum.get_rect_sum((0, 0), (0, 0)), 0);
    }
}
