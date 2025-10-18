//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: i64,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: i64,
        }
        Problem { n }
    }

    fn solve(&self) -> Answer {
        // i128 を使う
        let n = self.n as i128;
        let n_cbrt = n.cbrt();

        // x - y <= n_cbrt の場合

        for d in 1..=n_cbrt {
            let y = bin_search(0, n.sqrt() as i64, |y| {
                // オーバーフローが怖い
                let y = y as i128;
                // let t1 = (y + d).checked_pow(3);
                // let t2 = y.checked_pow(3);
                // if t1.is_none() || t2.is_none() {
                //     return false;
                // }
                (y + d) * (y + d) * (y + d) - y * y * y <= n
            });
            let y = y as i128;
            let x = y + d;
            if x > 0 && y > 0 && x * x * x - y * y * y == n {
                let x = x as i64;
                let y = y as i64;
                return Answer { ans: Some((x, y)) };
            }
        }

        // x^2 + xy + y^2 <= n / n_cbrt + 1 の場合
        // そんな場合は存在しないので計算不要だった
        // x - y > n_cbrt のとき、x > n_cbrt なので、x^2 + xy + y^2 >= cbrt^2 になる

        // let max_x = (n / n_cbrt + 1).sqrt();

        // for x in 1..=max_x {
        //     let y = (x * x * x - n).cbrt();
        //     if x > 0 && y > 0 && x * x * x - y * y * y == n {
        //         let x = x as i64;
        //         let y = y as i64;
        //         return Answer { ans: Some((x, y)) };
        //     }
        // }

        let ans = None;
        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        let n = self.n;
        for x in 1..=n {
            let y = (x * x * x - n).cbrt();
            if x > 0 && y > 0 && x * x * x - y * y * y == n {
                return Answer { ans: Some((x, y)) };
            }
        }
        Answer { ans: None }
        // let ans = 0;
        // Answer { ans }
    }

    fn check(&self, ans: &Answer) -> bool {
        if let Some((x, y)) = ans.ans {
            // ここオーバーフローする
            return x * x * x - y * y * y == self.n;
        }

        true
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Option<(i64, i64)>,
}

impl Answer {
    fn print(&self) {
        if let Some((x, y)) = self.ans {
            println!("{} {}", x, y);
        } else {
            println!("{}", -1);
        }
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
        let x: i64 = 1_000_000_000_000_000_000;

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
        if main_ans.ans.is_none() != naive_ans.ans.is_none() {
            return Some(WrongTestCase {
                problem: p.clone(),
                main_ans,
                naive_ans,
            });
        }

        if !p.check(&main_ans) {
            return Some(WrongTestCase {
                problem: p.clone(),
                main_ans,
                naive_ans,
            });
        }
        None
    }

    #[allow(dead_code)]
    fn make_random_problem(rng: &mut SmallRng) -> Problem {
        let n = rng.random_range(1..=99977273855577088);
        let p = Problem { n };
        println!("{:?}", &p);
        p
    }

    #[allow(unreachable_code)]
    #[test]
    fn test_with_naive() {
        let num_tests = 100;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut rng = SmallRng::seed_from_u64(45);
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

// ====== import ======
#[allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
use num_integer::Roots;
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
