#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Query {
    left: Usize1,
    right: Usize1,
}
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    nq: usize,
    xs: Vec<char>,
    qs: Vec<Query>,
}

struct SubProblem {
    n: usize,
    ind1: Segtree<Additive<i64>>,
    ind2: Segtree<Additive<i64>>,
    ind_slash: Segtree<Additive<i64>>,
}

impl SubProblem {
    fn new(xs: &[char]) -> SubProblem {
        let ind1 = xs
            .iter()
            .copied()
            .map(|ch| (ch == '1') as i64)
            .collect_vec();
        let ind2 = xs
            .iter()
            .copied()
            .map(|ch| (ch == '2') as i64)
            .collect_vec();
        let ind_slash = xs
            .iter()
            .copied()
            .map(|ch| (ch == '/') as i64)
            .collect_vec();
        let n = xs.len();
        let ind1 = Segtree::<Additive<i64>>::from(ind1);
        let ind2 = Segtree::<Additive<i64>>::from(ind2);
        let ind_slash = Segtree::<Additive<i64>>::from(ind_slash);
        SubProblem {
            n,
            ind1,
            ind2,
            ind_slash,
        }
    }

    /// xs[p..] を先頭から見て '1' を len 回スキップした直後の添字を求める
    fn skip_1s(&self, p: usize, len: i64) -> Option<usize> {
        if len == 0 {
            return Some(p);
        }
        let right = self.ind1.max_right(p, |sum| *sum < len);
        if right == self.n {
            None
        } else {
            Some(right + 1)
        }
    }

    /// xs[p..] を先頭から見て '/' を len 回スキップした直後の添字を求める
    fn skip_slashes(&self, p: usize, len: i64) -> Option<usize> {
        if len == 0 {
            return Some(p);
        }
        let right = self.ind_slash.max_right(p, |sum| *sum < len);
        if right == self.n {
            None
        } else {
            Some(right + 1)
        }
    }

    /// xs[p..] を先頭から見て '2' を len 回スキップした直後の添字を求める
    fn skip_2s(&self, p: usize, len: i64) -> Option<usize> {
        if len == 0 {
            return Some(p);
        }
        let right = self.ind2.max_right(p, |sum| *sum < len);
        if right == self.n {
            None
        } else {
            Some(right + 1)
        }
    }
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            nq: usize,
            xs: Chars,
            qs: [Query; nq],
        }
        Problem { n, nq, xs, qs }
    }

    fn solve(&self) -> Answer {
        let n = self.n;
        let xs = &self.xs;
        let sub = SubProblem::new(xs);

        let ans = self
            .qs
            .iter()
            .copied()
            .map(|q| {
                let len = bin_search(-1, n as i64, |len| {
                    (|| {
                        let cur1 = sub.skip_1s(q.left, len)?;
                        let cur2 = sub.skip_slashes(cur1, 1)?;
                        let cur3 = sub.skip_2s(cur2, len)?;
                        Some([cur1, cur2, cur3].iter().all(|c| *c <= q.right + 1))
                    })()
                    .unwrap_or(false)
                });
                if len == -1 {
                    0
                } else {
                    (2 * len + 1) as usize
                }
            })
            .collect_vec();
        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        todo!();
        // let ans = 0;
        // Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<usize>,
}

impl Answer {
    fn print(&self) {
        print_vec(&self.ans);
    }
}

fn main() {
    Problem::read().solve().print();
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

    #[allow(dead_code)]
    #[derive(Debug)]
    struct WrongTestCase {
        problem: Problem,
        main_ans: Answer,
        naive_ans: Answer,
    }

    #[allow(dead_code)]
    fn check(p: &Problem) -> Option<WrongTestCase> {
        let main_ans = p.solve();
        let naive_ans = p.solve_naive();
        if main_ans != naive_ans {
            Some(WrongTestCase {
                problem: p.clone(),
                main_ans,
                naive_ans,
            })
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn make_random_problem(rng: &mut SmallRng) -> Problem {
        todo!()
        // let n = rng.random_range(1..=10);
        // let p = Problem { _a: n };
        // println!("{:?}", &p);
        // p
    }

    #[allow(unreachable_code)]
    #[test]
    fn test_with_naive() {
        let num_tests = 0;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut rng = SmallRng::seed_from_u64(42);
        // let mut rng = SmallRng::from_os_rng();
        let mut wrong_cases: Vec<WrongTestCase> = vec![];
        for _ in 0..num_tests {
            let p = make_random_problem(&mut rng);
            let result = check(&p);
            if let Some(wrong_test_case) = result {
                wrong_cases.push(wrong_test_case);
            }
            if wrong_cases.len() >= max_wrong_case {
                break;
            }
        }

        if !wrong_cases.is_empty() {
            for t in &wrong_cases {
                println!("{:?}", t.problem);
                println!("main ans : {:?}", t.main_ans);
                println!("naive ans: {:?}", t.naive_ans);
                println!();
            }
            println!("{} cases are wrong.", wrong_cases.len());
            panic!();
        }
    }
}

use ac_library::{Additive, Max, Segtree};
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
pub fn bin_search<F>(mut ok: i64, mut ng: i64, mut p: F) -> i64
where
    F: FnMut(i64) -> bool,
{
    debug_assert!(ok != ng);
    debug_assert!(ok.checked_sub(ng).is_some());
    debug_assert!(ok.checked_add(ng).is_some());
    while num::abs(ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        debug_assert!(mid != ok);
        debug_assert!(mid != ng);
        if p(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}
