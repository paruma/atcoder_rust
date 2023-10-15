//#[derive_readable]
struct Problem {
    n: usize,
    t: Vec<u8>,
    ss: Vec<Vec<u8>>,
}

fn cnt_from_left(t: &[u8], s: &[u8]) -> usize {
    let mut sq = Queue::new();
    let mut cnt = 0;

    // これが良くなかった
    /*
    for c in t {
        tq.push(*c)
    }
    */
    let mut t_idx = 0;

    for c in s {
        sq.push(*c)
    }

    loop {
        if t_idx == t.len() || sq.is_empty() {
            break;
        }
        if t[t_idx] == *sq.peek().unwrap() {
            cnt += 1;
            t_idx += 1;
            sq.pop();
        } else {
            sq.pop();
        }
    }
    cnt
}

fn cnt_from_right(t: &[u8], s: &[u8]) -> usize {
    let tr: Vec<u8> = t.iter().copied().rev().collect_vec();
    let sr = s.iter().copied().rev().collect_vec();
    cnt_from_left(&tr, &sr)
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
        let t_rev = self.t.iter().copied().rev().collect_vec();
        let mut xs = self.ss.iter().map(|s| cnt_from_left(&self.t, s) as i64).collect_vec();
        let mut ys = self
            .ss
            .iter()
            .map(|s| cnt_from_left(&t_rev, &s.iter().copied().rev().collect_vec()) as i64)
            .collect_vec();
        let len = self.t.len() as i64;
        xs.sort();
        ys.sort();
        let ans = xs.iter().map(|x| ys.len() as i64 - lower_bound(&ys, len - x) as i64).sum();
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        let mut xs = self.ss.iter().map(|s| cnt_from_left(&self.t, s) as i64).collect_vec();
        let mut ys = self.ss.iter().map(|s| cnt_from_right(&self.t, s) as i64).collect_vec();
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
        // assert_eq!(cnt_from_left(b"bac", b"abba"), 2);
        assert_eq!(cnt_from_right(b"cba", b"abba"), 2);
        assert_eq!(1 + 1, 2);
    }
}

use std::collections::VecDeque;

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
use mod_queue::*;
pub mod mod_queue {
    use std::collections::VecDeque;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Queue<T> {
        raw: VecDeque<T>,
    }
    impl<T> Queue<T> {
        pub fn new() -> Self {
            Queue { raw: VecDeque::new() }
        }
        pub fn push(&mut self, value: T) {
            self.raw.push_front(value)
        }
        pub fn pop(&mut self) -> Option<T> {
            self.raw.pop_back()
        }
        pub fn peek(&self) -> Option<&T> {
            self.raw.back()
        }
        pub fn is_empty(&self) -> bool {
            self.raw.is_empty()
        }
    }
    impl<T> Default for Queue<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}

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
