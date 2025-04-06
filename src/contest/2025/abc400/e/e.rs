//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    xs: Vec<i64>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            xs: [i64; n],
        }
        Problem { n, xs }
    }

    fn solve(&self) -> Answer {
        let max = self.xs.iter().copied().max().unwrap() as usize;
        let max_sqrt = max.sqrt() * 2;
        let sieve = EratosthenesSieve::new(max_sqrt);

        let primes = (1..=max_sqrt).filter(|p| sieve.is_prime(*p)).collect_vec();
        let pow_primes = primes
            .iter()
            .flat_map(|&p| {
                std::iter::successors(Some(p), move |acc| Some(acc * p))
                    .take_while(|x| *x <= max_sqrt)
            })
            .sorted()
            .collect_vec();

        let num400s = {
            let mut buf: Vec<usize> = vec![];

            for i in 0..pow_primes.len() {
                if pow_primes[i] * pow_primes[i] * pow_primes[i] * pow_primes[i] > max {
                    break;
                }
                for j in i + 1..pow_primes.len() {
                    let val = pow_primes[i] * pow_primes[i] * pow_primes[j] * pow_primes[j];
                    if val > max {
                        break;
                    }
                    if sieve.min_factor(pow_primes[i]) != sieve.min_factor(pow_primes[j]) {
                        buf.push(val);
                    }
                }
            }
            buf.sort();
            buf
        };

        let ans = self
            .xs
            .iter()
            .copied()
            .map(|x| {
                //
                let idx = bin_search(0, num400s.len() as i64, |i| {
                    let i = i as usize;
                    x as usize >= num400s[i]
                }) as usize;

                num400s[idx]
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
        // let n = rng.gen_range(1..=10);
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
        // let mut rng = SmallRng::from_entropy();
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
use pathfinding::matrix::directions::E;
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet};
use superslice::Ext;

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
use eratosthenes_sieve::*;
pub mod eratosthenes_sieve {
    use std::collections::HashMap;
    pub struct EratosthenesSieve {
        is_prime_list: Vec<bool>,
        min_factor_list: Vec<Option<usize>>,
    }
    impl EratosthenesSieve {
        /// [0, n] の区間でエラトステネスのふるいをする
        /// 計算量: O(n log(log(n)))
        pub fn new(n: usize) -> Self {
            let mut is_prime_list = vec![true; n + 1];
            let mut min_factor_list = vec![None; n + 1];
            is_prime_list[0] = false;
            is_prime_list[1] = false;
            for p in 2..=n {
                if !is_prime_list[p] {
                    continue;
                }
                min_factor_list[p] = Some(p);
                for q in (p * 2..=n).step_by(p) {
                    is_prime_list[q] = false;
                    if min_factor_list[q].is_none() {
                        min_factor_list[q] = Some(p);
                    }
                }
            }
            Self {
                is_prime_list,
                min_factor_list,
            }
        }

        pub fn min_factor(&self, n: usize) -> usize {
            self.min_factor_list[n].unwrap()
        }
        /// 計算量: O(1)
        pub fn is_prime(&self, n: usize) -> bool {
            self.is_prime_list[n]
        }
        /// 計算量: O(log n)
        pub fn prime_factorize(&self, n: usize) -> HashMap<usize, usize> {
            let mut n = n;
            let mut cnt_table: HashMap<usize, usize> = HashMap::new();
            while n > 1 {
                let p = self.min_factor_list[n].unwrap();
                let mut exp = 0;
                while self.min_factor_list[n] == Some(p) {
                    n /= p;
                    exp += 1;
                }
                cnt_table.insert(p, exp);
            }
            cnt_table
        }
        /// 計算量: O(nの約数の個数)
        pub fn divisors(&self, n: usize) -> Vec<usize> {
            let mut res = vec![1];
            let pf = self.prime_factorize(n);
            for (p, e) in pf {
                for i in 0..res.len() {
                    let mut tmp = 1;
                    for _ in 0..e {
                        tmp *= p;
                        res.push(res[i] * tmp);
                    }
                }
            }
            res
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
