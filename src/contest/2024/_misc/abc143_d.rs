//#[derive_readable]
#[derive(Debug)]
struct Problem {
    n: usize,
    ls: Vec<u32>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            ls: [u32; n],
        }
        Problem { n, ls }
    }
    fn solve(&self) -> Answer {
        // O(N^3) (1711ms)
        let ans = self
            .ls
            .iter()
            .copied()
            .tuple_combinations()
            .filter(|&(a, b, c)| a < b + c && b < c + a && c < a + b)
            .count() as i64;
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // O(N^3), スタック領域を使う(1706ms, ヒープ領域を使うものと全然変わらない)
        let mut ls_slice = [0_u32; 2000];
        ls_slice[..self.n].copy_from_slice(&self.ls);
        let ls_slice = &ls_slice[..self.n];
        let ans = ls_slice
            .iter()
            .copied()
            .tuple_combinations()
            .filter(|&(a, b, c)| a < b + c && b < c + a && c < a + b)
            .count() as i64;
        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        // O(N^3) ソートする
        // a <= b <= c ならば、a < b + c と b < c + a は自動で成り立つ。 c < a + b のみ確認すれば良い
        let ls_sorted = self.ls.iter().copied().sorted().collect_vec();
        let ans = ls_sorted
            .iter()
            .copied()
            .tuple_combinations()
            .filter(|&(a, b, c)| c < a + b)
            .count() as i64;
        Answer { ans }
    }

    fn solve4(&self) -> Answer {
        // O(N^2 log N) 想定解
        // a <= b <= c ならば、a < b + c と b < c + a は自動で成り立つ。 c < a + b のみ確認すれば良い
        let ls_sorted = self.ls.iter().copied().sorted().collect_vec();
        let n = self.n;
        let ans = ls_sorted
            .iter()
            .copied()
            .enumerate()
            .tuple_combinations()
            .map(|((_ai, a), (bi, b))| {
                //0
                let ci = bin_search(bi as i64, n as i64, |ci| {
                    let c = ls_sorted[ci as usize];
                    c < a + b
                });
                ci - (bi as i64)
            })
            .sum::<i64>();
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
    ans: i64,
}

impl Answer {
    fn print(&self) {
        println!("{}", self.ans);
    }
}

fn main() {
    Problem::read().solve4().print();
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

    fn check(p: &Problem) {
        assert_eq!(p.solve(), p.solve_naive());
    }

    fn make_random_problem() -> Problem {
        todo!()
        // let mut rng = SmallRng::from_os_rng();
        // let n = rng.random_range(1..=10);
        // let p = Problem { _a: n };
        // println!("{:?}", &p);
        // p
    }

    #[test]
    fn test_with_naive() {
        // 手動でテストを作るのもOK
        for _ in 0..100 {
            // let p = make_random_problem();
            // check(&p);
        }
    }
}

// ====== import ======
#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, izip};
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};
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
