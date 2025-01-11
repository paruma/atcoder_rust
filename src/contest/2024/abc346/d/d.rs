//#[derive_readable]
struct Problem {
    n: usize,
    s: Vec<usize>,
    cs: Vec<i64>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            s: Bytes,
            cs: [i64; n],
        }
        let s = s.iter().copied().map(|x| (x - b'0') as usize).collect_vec();
        Problem { n, s, cs }
    }
    fn solve(&self) -> Answer {
        let n = self.n;
        let s = &self.s;

        let mut dp = vec![vec![vec![i64::MAX; 2]; 2]; n + 1];

        dp[0][0][0] = 0;
        dp[0][0][1] = 0;

        for i in 0..n {
            // 00, 11 がないパターンdp[i+1][0][-]
            for last in [0_usize, 1] {
                let cost = if s[i] == last { 0 } else { self.cs[i] };
                dp[i + 1][0][last] = dp[i][0][1 - last] + cost;
            }

            if i == 0 {
                continue;
            }
            // 00, 11 があるパターン dp[i+1][1][-]
            for last in [0_usize, 1] {
                let cost = if s[i] == last { 0 } else { self.cs[i] };
                dp[i + 1][1][last] = i64::min(dp[i][0][last], dp[i][1][1 - last]) + cost;
            }
        }
        let ans = i64::min(dp[n][1][0], dp[n][1][1]);
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // 累積和を使う
        // 良い文字はたかだか2(N-1)通りくらいしかないので、全部のコストを計算する。
        let n = self.n;
        let s = &self.s;
        let cs = &self.cs;

        let cost01 = (0..n)
            .map(|i| {
                // i % 2 にする
                if s[i] != i % 2 {
                    cs[i]
                } else {
                    0
                }
            })
            .collect_vec();

        let cost10 = (0..n)
            .map(|i| {
                // (i + 1) % 2 にする
                if s[i] != (i + 1) % 2 {
                    cs[i]
                } else {
                    0
                }
            })
            .collect_vec();

        let cost01_cumsum = CumSum::new(&cost01);
        let cost10_cumsum = CumSum::new(&cost10);

        // 前半01, 後半10
        let score0 = (0..(n - 1))
            .map(|i| {
                // 0..=i は cost01
                // (i+1)..n は cost10 を使う
                cost01_cumsum.range_sum(0..=i) + cost10_cumsum.range_sum((i + 1)..n)
            })
            .min()
            .unwrap();

        // 前半10, 後半01
        let score1 = (0..(n - 1))
            .map(|i| {
                // 0..=i は cost01
                // (i+1)..n は cost10 を使う
                cost10_cumsum.range_sum(0..=i) + cost01_cumsum.range_sum((i + 1)..n)
            })
            .min()
            .unwrap();

        let ans = i64::min(score0, score1);

        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        // DP 解き直し
        let n = self.n;
        let s = &self.s;
        let cs = &self.cs;

        // dp[i][c][x] = [0, i) を見たとき、隣り合った2文字が一致している箇所が c 個あって、s[i-1] が x のもの
        let mut dp = vec![[[i64::MAX; 2]; 2]; n + 1];

        for x in [0, 1] {
            dp[1][0][x] = (s[0] != x) as i64 * cs[0];
            dp[1][1][x] = i64::MAX;
        }

        for i in 2..=n {
            for x in [0, 1] {
                dp[i][0][x] = dp[i - 1][0][1 - x] + (s[i - 1] != x) as i64 * cs[i - 1];
                dp[i][1][x] = i64::min(dp[i - 1][0][x], dp[i - 1][1][1 - x])
                    + (s[i - 1] != x) as i64 * cs[i - 1];
            }
        }

        let ans = i64::min(dp[n][1][0], dp[n][1][1]);

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
    Problem::read().solve3().print();
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
    pub fn prefix_sum(xs: &[i64]) -> Vec<i64> {
        let mut prefix_sum = vec![0; xs.len() + 1];
        for i in 1..xs.len() + 1 {
            prefix_sum[i] = prefix_sum[i - 1] + xs[i - 1];
        }
        prefix_sum
    }
    use std::ops::{Bound, Range, RangeBounds};
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
        fn open(&self, range: impl RangeBounds<usize>) -> Range<usize> {
            use Bound::Excluded;
            use Bound::Included;
            use Bound::Unbounded;
            let begin = match range.start_bound() {
                Unbounded => 0,
                Included(&x) => x,
                Excluded(&x) => x + 1,
            };
            let end = match range.end_bound() {
                Excluded(&x) => x,
                Included(&x) => x + 1,
                Unbounded => self.cumsum.len() - 1,
            };
            begin..end
        }
        /// 計算量: O(1)
        pub fn range_sum(&self, range: impl RangeBounds<usize>) -> i64 {
            let range = self.open(range);
            self.cumsum[range.end] - self.cumsum[range.start]
        }
        pub fn prefix_sum(&self, end: usize) -> i64 {
            self.cumsum[end]
        }
        pub fn suffix_sum(&self, begin: usize) -> i64 {
            self.cumsum[self.cumsum.len() - 1] - self.cumsum[begin]
        }
    }
}
