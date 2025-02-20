//#[derive_readable]
struct Problem {
    n: usize,
    t: Vec<u8>,
    ss: Vec<Vec<u8>>,
}

struct SubProblem {
    t: Vec<u8>,
    t_rev: Vec<u8>,
}

impl SubProblem {
    pub fn new(t: &[u8]) -> Self {
        SubProblem { t: t.to_vec(), t_rev: t.iter().copied().rev().collect_vec() }
    }
    pub fn cnt_from_left(&self, s: &[u8]) -> usize {
        Self::cnt_from_left_sub(&self.t, s)
    }

    pub fn cnt_from_right(&self, s: &[u8]) -> usize {
        let s_rev = s.iter().copied().rev().collect_vec();
        Self::cnt_from_left_sub(&self.t_rev, &s_rev)
    }

    fn cnt_from_left_sub(t: &[u8], s: &[u8]) -> usize {
        let mut t_iter = t.iter().peekable();
        let mut s_iter = s.iter().peekable();
        let mut cnt = 0;

        while s_iter.peek().is_some() && t_iter.peek().is_some() {
            if s_iter.peek() == t_iter.peek() {
                cnt += 1;
                s_iter.next();
                t_iter.next();
            } else {
                s_iter.next();
            }
        }
        cnt
    }
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            t: Bytes,
            ss: [Bytes; n],
        }
        Problem { n, t, ss }
    }
    fn solve(&self) -> Answer {
        let sub = SubProblem::new(&self.t);
        let mut xs = self.ss.iter().map(|s| sub.cnt_from_left(s) as i64).collect_vec();
        let mut ys = self.ss.iter().map(|s| sub.cnt_from_right(s) as i64).collect_vec();
        let len = self.t.len() as i64;
        xs.sort();
        ys.sort();
        // x in xs と y in ys に対して、x + y >= len となるような (x, y) の組の数を求める
        let ans = xs
            .iter()
            .map(|&x| {
                // x を固定して、y >= len - x となるような y の数を求める
                // [lower_bound(&ys, len -x), ys.len()) がそのような y のインデックスの範囲
                ys.len() as i64 - lower_bound(&ys, len - x) as i64
            })
            .sum();
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        let sub = SubProblem::new(&self.t);
        let xs = self.ss.iter().map(|s| sub.cnt_from_left(s) as i64).collect_vec();
        let ys = self.ss.iter().map(|s| sub.cnt_from_right(s) as i64).collect_vec();
        let len = self.t.len() as i64;

        let ans = iproduct!(xs, ys).map(|(x, y)| x + y).filter(|k| *k >= len).count() as i64;
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
        let sub_problem = SubProblem::new(b"bac");
        assert_eq!(sub_problem.cnt_from_left(b"abba"), 2);
        assert_eq!(sub_problem.cnt_from_right(b"abca"), 2);
    }
}

// ====== import ======
use itertools::iproduct;
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

/// 二分探索をする
/// ```text
/// ng ng ng ok ok ok
///          ↑ここの引数の値を返す
/// ```
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
/// 指定された要素以上の値が現れる最初の位置を返す。
/// ## Arguments
/// * xs: 単調増加
///     * 単調増加でなくても、 `|i| xs[i] >= key` が単調ならOK
/// ## Return
/// `I = {i in 0..xs.len() | xs[i] >= key}` としたとき、`min I` を返す。
/// ただし、`I` が空の場合は `xs.len()` を返す
/// 戻り値は、区間 `0..=xs.len()` の間で返る。
pub fn lower_bound<T: PartialOrd>(xs: &[T], key: T) -> usize {
    let pred = |i: i64| xs[i as usize] >= key;
    bin_search(xs.len() as i64, -1_i64, pred) as usize
}
