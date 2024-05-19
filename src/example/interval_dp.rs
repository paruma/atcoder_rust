#![allow(dead_code)]

fn slimes(n: usize, xs: &[i64]) -> i64 {
    // Educational DP Contest N - Slimes https://atcoder.jp/contests/dp/tasks/dp_n
    struct Rec {
        n: usize,
        xs: Vec<i64>,
        xs_cumsum: Vec<i64>,
    }

    impl Rec {
        fn new(n: usize, xs: &[i64]) -> Self {
            let xs = xs.to_vec();
            let mut xs_cumsum = vec![0; xs.len() + 1];
            for i in 0..xs.len() {
                xs_cumsum[i + 1] = xs[i] + xs_cumsum[i]
            }
            Self { n, xs, xs_cumsum }
        }

        fn rec(&self, begin: usize, end: usize, dp: &mut [Vec<Option<i64>>]) -> i64 {
            if let Some(ans) = dp[begin][end] {
                return ans;
            }

            let ans = if begin == end {
                0
            } else if end - begin <= 1 {
                0
            } else {
                (begin + 1..end)
                    .map(|mid| self.rec(begin, mid, dp) + self.rec(mid, end, dp))
                    .min()
                    .unwrap()
                    + (self.xs_cumsum[end] - self.xs_cumsum[begin])
            };
            dp[begin][end] = Some(ans);
            ans
        }
    }
    let mut dp = vec![vec![None; n + 1]; n + 1];
    Rec::new(n, xs).rec(0, n, &mut dp)
}

#[cfg(test)]
mod tests {
    use super::slimes;

    #[test]
    fn test_slimes() {
        assert_eq!(slimes(4, &[10, 20, 30, 40]), 190);
    }
}
