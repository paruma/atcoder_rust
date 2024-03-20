//#[derive_readable]
struct Problem {
    n: usize,
    c: i64,
    xs: Vec<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct RangeSumMinMax {
    prefix_sum_max: i64,
    internal_sum_max: i64,
    suffix_sum_max: i64,
    prefix_sum_min: i64,
    internal_sum_min: i64,
    suffix_sum_min: i64,
    sum: i64,
}

impl RangeSumMinMax {
    fn unit(x: i64) -> RangeSumMinMax {
        RangeSumMinMax {
            prefix_sum_max: x,
            internal_sum_max: x,
            suffix_sum_max: x,
            prefix_sum_min: x,
            internal_sum_min: x,
            suffix_sum_min: x,
            sum: x,
        }
    }

    fn range_sum_max(&self) -> i64 {
        self.internal_sum_max
    }

    fn range_sum_min(&self) -> i64 {
        self.internal_sum_min
    }

    fn range_sum(&self) -> i64 {
        self.sum
    }
}

struct Concat(Infallible);
impl Monoid for Concat {
    type S = RangeSumMinMax;
    fn identity() -> Self::S {
        RangeSumMinMax {
            prefix_sum_max: 0,
            internal_sum_max: 0,
            suffix_sum_max: 0,
            prefix_sum_min: 0,
            internal_sum_min: 0,
            suffix_sum_min: 0,
            sum: 0,
        }
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        RangeSumMinMax {
            prefix_sum_max: i64::max(a.prefix_sum_max, a.sum + b.prefix_sum_max),
            internal_sum_max: {
                *[
                    a.internal_sum_max,
                    b.internal_sum_max,
                    a.suffix_sum_max + b.prefix_sum_max,
                ]
                .iter()
                .max()
                .unwrap()
            },
            suffix_sum_max: i64::max(b.suffix_sum_max, b.sum + a.suffix_sum_max),
            prefix_sum_min: i64::min(a.prefix_sum_min, a.sum + b.prefix_sum_min),
            internal_sum_min: {
                *[
                    a.internal_sum_min,
                    b.internal_sum_min,
                    a.suffix_sum_min + b.prefix_sum_min,
                ]
                .iter()
                .min()
                .unwrap()
            },
            suffix_sum_min: i64::min(b.suffix_sum_min, b.sum + a.suffix_sum_min),
            sum: a.sum + b.sum,
        }
    }
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            c: i64,
            xs: [i64; n],
        }
        Problem { n, c, xs }
    }

    fn range_max(&self) -> i64 {
        let mut dp_left = vec![0; self.n + 1]; // dp_left[i] = [0, i - 1) での区間和最大値
        let mut dp_right = vec![0; self.n + 1]; // dp_right[i] = [0, i) での suffix_sum 最大値

        dp_left[0] = 0;
        dp_right[0] = 0;

        for i in 0..self.n {
            dp_left[i + 1] = i64::max(dp_left[i], dp_right[i]);
            dp_right[i + 1] = i64::max(self.xs[i], self.xs[i] + dp_right[i]);
        }
        i64::max(dp_left[self.n], dp_right[self.n])
    }

    fn range_min(&self) -> i64 {
        let mut dp_left = vec![0; self.n + 1];
        let mut dp_right = vec![0; self.n + 1];

        dp_left[0] = 0;
        dp_right[0] = 0;

        for i in 0..self.n {
            dp_left[i + 1] = i64::min(dp_left[i], dp_right[i]);
            dp_right[i + 1] = i64::min(self.xs[i], self.xs[i] + dp_right[i]);
        }
        i64::min(dp_left[self.n], dp_right[self.n])
    }

    fn solve(&self) -> Answer {
        // DP による解法
        let sum = self.xs.iter().sum::<i64>();
        let c = self.c;
        let ans = if c >= 1 {
            sum + self.range_max() * (c - 1)
        } else {
            sum + self.range_min() * (c - 1)
        };

        Answer { ans }
    }
}

impl Problem {
    // solve2
    fn solve2(&self) -> Answer {
        // モノイドによる解法
        let xs_info = self
            .xs
            .iter()
            .copied()
            .map(RangeSumMinMax::unit)
            .fold(Concat::identity(), |acc, x| {
                Concat::binary_operation(&acc, &x)
            });
        let c = self.c;
        let ans = if c >= 1 {
            xs_info.range_sum() + xs_info.range_sum_max() * (c - 1)
        } else {
            xs_info.range_sum() + xs_info.range_sum_min() * (c - 1)
        };

        Answer { ans }
    }
}

impl Problem {
    // solve3
    fn range_max3(&self) -> i64 {
        let mut dp_internal_sum = vec![0; self.n + 1]; // dp_internal_sum[i] = [0, i) での区間和最大値
        let mut dp_suffix_sum = vec![0; self.n + 1]; // dp_suffix_sum[i] = [0, i) での suffix_sum 最大値

        dp_internal_sum[0] = 0;
        dp_suffix_sum[0] = 0;

        for i in 0..self.n {
            dp_internal_sum[i + 1] = i64::max(dp_internal_sum[i], dp_suffix_sum[i] + self.xs[i]);
            dp_suffix_sum[i + 1] = i64::max(self.xs[i], dp_suffix_sum[i] + self.xs[i]);
        }
        dp_internal_sum[self.n]
    }

    fn range_min3(&self) -> i64 {
        let mut dp_internal_sum = vec![0; self.n + 1];
        let mut dp_suffix_sum = vec![0; self.n + 1];

        dp_internal_sum[0] = 0;
        dp_suffix_sum[0] = 0;

        for i in 0..self.n {
            dp_internal_sum[i + 1] = i64::min(dp_internal_sum[i], dp_suffix_sum[i] + self.xs[i]);
            dp_suffix_sum[i + 1] = i64::min(self.xs[i], dp_suffix_sum[i] + self.xs[i]);
        }
        dp_internal_sum[self.n]
    }

    fn solve3(&self) -> Answer {
        // DP による解法2
        let sum = self.xs.iter().sum::<i64>();
        let c = self.c;
        let ans = if c >= 1 {
            sum + self.range_max3() * (c - 1)
        } else {
            sum + self.range_min3() * (c - 1)
        };

        Answer { ans }
    }
}
impl Problem {
    fn range_max4(&self) -> i64 {
        let cumsum = CumSum::new(&self.xs).cumsum;
        // 区間和は cumsum[end] - cumsum[begin] (begin <= end) の形で求まる
        // つまり、max{cumsum[end] - cumsum[begin] | begin <= end} を求めれば良い
        // そのために、end を固定して、min{cumsum[begin] | begin <= end} を求める
        // ABC331 E の主菜全探索と同じ考え方 (end 全探索)

        let cumsum_cummin = CumMonoid::<Min<i64>>::new(&cumsum);

        (0..=self.n) // end の範囲なので、n を含める
            .map(|end| {
                // cumsum[end] - min {cumsum[begin] | begin <= end}
                cumsum[end] - cumsum_cummin.prefix_prod(end + 1)
            })
            .max()
            .unwrap()
    }

    fn range_min4(&self) -> i64 {
        let cumsum = CumSum::new(&self.xs).cumsum;

        let cumsum_cummax = CumMonoid::<Max<i64>>::new(&cumsum);

        (0..=self.n)
            .map(|end| cumsum[end] - cumsum_cummax.prefix_prod(end + 1))
            .min()
            .unwrap()
    }
    fn solve4(&self) -> Answer {
        // 累積和から求める

        let sum = self.xs.iter().sum::<i64>();
        let c = self.c;
        let ans = if c >= 1 {
            sum + self.range_max4() * (c - 1)
        } else {
            sum + self.range_min4() * (c - 1)
        };

        Answer { ans }
    }
}

impl Problem {
    fn range_max5(&self) -> i64 {
        let xs = &self.xs;
        let n = self.xs.len();
        let mut dp = vec![0; n + 1];
        dp[0] = 0;

        for end in 0..n {
            dp[end + 1] = i64::max(dp[end] + xs[end], xs[end]);
        }
        *dp.iter().max().unwrap()
    }

    fn range_min5(&self) -> i64 {
        let xs = &self.xs;
        let n = self.xs.len();
        let mut dp = vec![0; n + 1];
        dp[0] = 0;

        for end in 0..n {
            dp[end + 1] = i64::min(dp[end] + xs[end], xs[end]);
        }
        *dp.iter().min().unwrap()
    }
    fn solve5(&self) -> Answer {
        // 累積和から求める

        let sum = self.xs.iter().sum::<i64>();
        let c = self.c;
        let ans = if c >= 1 {
            sum + self.range_max5() * (c - 1)
        } else {
            sum + self.range_min5() * (c - 1)
        };

        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: i64,
}

impl Answer {
    fn print(&self) {
        println!("{}", self.ans);
    }
}

fn main() {
    Problem::read().solve5().print();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }
}

use std::convert::Infallible;

use ac_library::{Max, Min, Monoid};
// ====== import ======
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};

// ====== output func ======
#[allow(unused_imports)]
use print_vec::*;
pub mod print_vec {
    use itertools::Itertools;
    use proconio::fastout;
    #[fastout]
    pub fn print_vec<T: std::fmt::Debug>(arr: &[T]) {
        for a in arr {
            println!("{:?}", a);
        }
    }
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Debug>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{:?}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Debug>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{:?}", x)).join(" ");
            println!("{}", msg);
        }
    }
    pub fn print_bytes(bytes: &[u8]) {
        let msg = String::from_utf8(bytes.to_vec()).unwrap();
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec_bytes(vec_bytes: &[Vec<u8>]) {
        for row in vec_bytes {
            let msg = String::from_utf8(row.to_vec()).unwrap();
            println!("{}", msg);
        }
    }
}

#[allow(unused)]
fn print_yesno(ans: bool) {
    let msg = if ans { "Yes" } else { "No" };
    println!("{}", msg);
}

// ====== snippet ======
use cumsum::*;
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
            self.cumsum[end] - self.cumsum[begin]
        }
    }
}

use cum_monoid::*;
pub mod cum_monoid {
    use ac_library::Monoid;
    pub struct CumMonoid<M>
    where
        M: Monoid,
    {
        prefix_prod: Vec<M::S>,
        suffix_prod: Vec<M::S>,
    }
    impl<M> CumMonoid<M>
    where
        M: Monoid,
    {
        pub fn new(xs: &[M::S]) -> CumMonoid<M> {
            let mut prefix_prod = vec![M::identity(); xs.len() + 1];
            let mut suffix_prod = vec![M::identity(); xs.len() + 1];
            for i in 0..xs.len() {
                prefix_prod[i + 1] = M::binary_operation(&prefix_prod[i], &xs[i]);
            }
            for i in (0..xs.len()).rev() {
                suffix_prod[i] = M::binary_operation(&xs[i], &suffix_prod[i + 1]);
            }
            CumMonoid {
                prefix_prod,
                suffix_prod,
            }
        }
        /// [0, i) の総積 (前から累積)
        pub fn prefix_prod(&self, i: usize) -> M::S {
            self.prefix_prod[i].clone()
        }
        /// [i, n) の総積 (後ろから累積)
        pub fn suffix_prod(&self, i: usize) -> M::S {
            self.suffix_prod[i].clone()
        }
        /// [0, i), [i + 1, n) の区間で総積を取る
        pub fn prod_without1(&self, i: usize) -> M::S {
            M::binary_operation(&self.prefix_prod[i], &self.suffix_prod[i + 1])
        }
        pub fn prod_without_range(&self, l: usize, r: usize) -> M::S {
            M::binary_operation(&self.prefix_prod[l], &self.suffix_prod[r])
        }
    }
}
