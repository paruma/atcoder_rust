//#[derive_readable]
struct Problem {
    n: usize,
    xs: Vec<i64>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            xs: [i64; n]
        }
        Problem { n, xs }
    }
    fn solve(&self) -> Answer {
        let xs = &self.xs;
        let n = self.n;
        let xs_sorted = xs.iter().copied().sorted().collect_vec();
        let cumsum = cumsum::CumSum::new(&xs_sorted);
        let ans = xs
            .iter()
            .copied()
            .map(|x| {
                // xs の中で x より大きいもので最小の添え字
                let idx = upper_bound(&xs_sorted, x);
                cumsum.get_interval_sum(idx, n)
            })
            .collect_vec();

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // 解説解法 O(n)
        let xs = &self.xs;
        /*
        [1, 4, 1, 4, 2] から
        {
            4 => [1,3],
            2 => [2,4],
            1 => [0],
        }
        を得る
        */
        let value_to_idxes = xs
            .iter()
            .copied()
            .enumerate()
            .into_group_map_by(|(_i, x)| *x)
            .into_iter()
            .map(|(i, v)| (i, v.into_iter().map(|(i, _x)| i).collect_vec()))
            .collect::<HashMap<_, _>>();
        let mut sum = 0;
        let mut ans = vec![0; self.n];

        for (x, idxes) in value_to_idxes.into_iter().sorted_by_key(|x| Reverse(x.0)) {
            for &i in &idxes {
                ans[i] = sum;
            }
            sum += x * (idxes.len() as i64);
        }

        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<i64>,
}

impl Answer {
    fn print(&self) {
        print_vec_1line(&self.ans);
    }
}

fn main() {
    Problem::read().solve2().print();
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

use std::{cmp::Reverse, collections::HashMap};

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
        pub fn new(xs: &Vec<i64>) -> CumSum {
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

/// 二分探索をする
/// ```text
/// ng ng ng ok ok ok
///          ↑ここの引数の値を返す
/// ```
/// 計算量: O(log(|ok - ng|))
/// ## Arguments
/// * ok != ng
/// * |ok - ng| <= 2^63 - 1, |ok + ng| <= 2^63 - 1
/// * p の定義域について
///     * ng < ok の場合、p は区間 ng..ok で定義されている。
///     * ok < ng の場合、p は区間 ok..ng で定義されている。
/// * p の単調性について
///     * ng < ok の場合、p は単調増加
///     * ok < ng の場合、p は単調減少
/// ## Return
/// * ng < ok の場合: I = { i in ng..ok | p(i) == true } としたとき
///     * I が空でなければ、min I を返す。
///     * I が空ならば、ok を返す。
/// * ok < ng の場合: I = { i in ok..ng | p(i) == true } としたとき
///     * I が空でなければ、max I を返す。
///     * I が空ならば、ok を返す。
pub fn bin_search<F>(mut ok: i64, mut ng: i64, p: F) -> i64
where
    F: Fn(i64) -> bool,
{
    assert!(ok != ng);
    assert!(ok.checked_sub(ng).is_some());
    assert!(ok.checked_add(ng).is_some());
    while num::abs(ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        assert!(mid != ok);
        assert!(mid != ng);
        if p(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}
/// 指定された要素より大きい値が現れる最初の位置を返す。
/// 計算量: O(log(|xs|))
/// ## Arguments
/// * xs: 単調増加
///     * 単調増加でなくても、 `|i| xs[i] > key` が単調ならOK
/// ## Return
/// `I = {i in 0..xs.len() | xs[i] > key}` としたとき、`min I` を返す。
/// ただし、`I` が空の場合は `xs.len()` を返す
/// 戻り値は、区間 `0..=xs.len()` の間で返る。
pub fn upper_bound<T: PartialOrd>(xs: &[T], key: T) -> usize {
    let pred = |i: i64| xs[i as usize] > key;
    bin_search(xs.len() as i64, -1_i64, pred) as usize
}
