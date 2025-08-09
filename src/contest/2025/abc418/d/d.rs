fn main() {
    input! {
        n: usize,
        xs: Chars,
    }
    let xs = xs.iter().copied().map(|x| (x == '1') as i64).collect_vec();
    let ss = prefix_sum(&xs); // 長さが n+1
    let ind_even = ss
        .iter()
        .copied()
        .map(|x| (x % 2 == 0) as i64)
        .collect_vec();
    let ind_odd = ss
        .iter()
        .copied()
        .map(|x| (x % 2 == 1) as i64)
        .collect_vec();

    // 添字 even 抜き出し
    let ind_even_even = (0..n + 1)
        .filter(|&i| i % 2 == 0)
        .map(|i| ind_even[i])
        .collect_vec();
    let ind_even_odd = (0..n + 1)
        .filter(|&i| i % 2 == 1)
        .map(|i| ind_even[i])
        .collect_vec();

    // 添字 odd 抜き出し
    let ind_odd_even = (0..n + 1)
        .filter(|&i| i % 2 == 0)
        .map(|i| ind_odd[i])
        .collect_vec();
    let ind_odd_odd = (0..n + 1)
        .filter(|&i| i % 2 == 1)
        .map(|i| ind_odd[i])
        .collect_vec();

    let ind_even_even_cumsum = CumSum::new(&ind_even_even);
    let ind_even_odd_cumsum = CumSum::new(&ind_even_odd);
    let ind_odd_even_cumsum = CumSum::new(&ind_odd_even);
    let ind_odd_odd_cumsum = CumSum::new(&ind_odd_odd);

    let ans: i64 = (0..n)
        .map(|j| {
            if j % 2 == 0 {
                if ss[j + 1] % 2 == 0 {
                    // i が偶数, S_i 奇数
                    // ind_odd_even_cumsum
                    let term1 = ind_odd_even_cumsum.range_sum(0..j / 2 + 1);

                    // i が奇数, S_i 偶数
                    let term2 = ind_even_odd_cumsum.range_sum(0..j / 2);

                    term1 + term2
                } else {
                    // i が偶数, S_i 偶数
                    let term1 = ind_even_even_cumsum.range_sum(0..j / 2 + 1);

                    // i が奇数, S_i 奇数
                    let term2 = ind_odd_odd_cumsum.range_sum(0..j / 2);
                    term1 + term2
                }
            } else {
                if ss[j + 1] % 2 == 0 {
                    // i が偶数, S_i 偶数
                    // ind_odd_even_cumsum
                    let term1 = ind_even_even_cumsum.range_sum(0..j / 2 + 1);

                    // i が奇数, S_i 奇数
                    let term2 = ind_odd_odd_cumsum.range_sum(0..j / 2 + 1);
                    term1 + term2
                } else {
                    // i が偶数, S_i 奇数
                    let term1 = ind_odd_even_cumsum.range_sum(0..j / 2 + 1);

                    // i が奇数, S_i 偶数
                    let term2 = ind_even_odd_cumsum.range_sum(0..j / 2 + 1);

                    term1 + term2
                }
            }
        })
        .sum::<i64>();
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }
}

// ====== import ======
#[allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet};

// ====== output func ======
#[allow(unused_imports)]
use print_util::*;
pub mod print_util {
    use itertools::Itertools;
    use proconio::fastout;
    #[fastout]
    pub fn print_vec<T: std::fmt::Display>(arr: &[T]) {
        for a in arr {
            println!("{}", a);
        }
    }
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Display>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{}", x)).join(" ");
            println!("{}", msg);
        }
    }
    pub fn print_bytes(bytes: &[u8]) {
        let msg = String::from_utf8(bytes.to_vec()).unwrap();
        println!("{}", msg);
    }
    pub fn print_chars(chars: &[char]) {
        let msg = chars.iter().collect::<String>();
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec_bytes(vec_bytes: &[Vec<u8>]) {
        for row in vec_bytes {
            let msg = String::from_utf8(row.to_vec()).unwrap();
            println!("{}", msg);
        }
    }
    #[fastout]
    pub fn print_vec_chars(vec_chars: &[Vec<char>]) {
        for row in vec_chars {
            let msg = row.iter().collect::<String>();
            println!("{}", msg);
        }
    }
    pub fn print_yesno(ans: bool) {
        let msg = if ans { "Yes" } else { "No" };
        println!("{}", msg);
    }
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
    #[derive(Clone, Debug, PartialEq, Eq)]
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
