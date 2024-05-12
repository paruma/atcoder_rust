//#[derive_readable]
struct Problem {
    n: usize,
    exchange_cost: i64,
    update_cost: i64,
    xs: Vec<u8>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            exchange_cost: i64,
            update_cost: i64,
            xs: Bytes,
        }
        Problem {
            n,
            exchange_cost,
            update_cost,
            xs,
        }
    }
    fn solve(&self) -> Answer {
        // 解説AC (解説そのまま)
        let n = self.n;
        let exchange_cost = self.exchange_cost;
        let update_cost = self.update_cost;
        let xs = &self.xs;

        let mut xs = xs
            .iter()
            .copied()
            .map(|ch| match ch {
                b'(' => 1_i64,
                b')' => -1_i64,
                _ => unreachable!(),
            })
            .collect_vec();

        let cnt_1 = xs.iter().copied().filter(|&x| x == 1).count();
        let cnt_m1 = xs.iter().copied().filter(|&x| x == -1).count();

        let mut cnt_update = 0;

        if cnt_m1 > cnt_1 {
            // 左側の -1 を 1 にする
            cnt_update = (cnt_m1 - cnt_1) / 2;
            for x in xs.iter_mut().filter(|x| **x == -1).take(cnt_update) {
                *x = 1;
            }
        } else {
            // cnt_1 >= cnt_m1
            // 右側の 1 を -1 にする
            cnt_update = (cnt_1 - cnt_m1) / 2;
            for x in xs.iter_mut().rev().filter(|x| **x == 1).take(cnt_update) {
                *x = -1;
            }
        }

        let exchange_cost = i64::min(exchange_cost, update_cost * 2);
        let cumsum_xs = CumSum::new(&xs).cumsum;
        let min_cumsum_xs = cumsum_xs.iter().copied().min().unwrap();

        let cnt_exchange = if min_cumsum_xs >= 0 {
            0
        } else {
            div_ceil(-min_cumsum_xs, 2)
        };

        let ans = cnt_exchange * exchange_cost + (cnt_update as i64) * update_cost;
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
    Problem::read().solve().print();
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
use num_integer::div_ceil;
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
