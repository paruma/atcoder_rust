//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: i64,
}

fn is_palindrome(k: i64) -> bool {
    let s = to_base_n_value(k, 10);
    s == s.iter().copied().rev().collect_vec()
}

fn rev_number(k: i64) -> i64 {
    let mut s = to_base_n_value(k, 10);
    s.reverse();
    eval_base_n_value(&s, 10)
}

fn contains0(k: i64) -> bool {
    let s = to_base_n_value(k, 10);
    s.contains(&0)
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: i64,
        }
        Problem { n }
    }
    fn solve(&self) -> Answer {
        let n = self.n;

        if n == 1 {
            return Answer {
                ans: Some("1".to_string()),
            };
        }

        // 0が入っている数字は除く（問題文）
        let mut divisors = divisors(n)
            .iter()
            .copied()
            .filter(|x| !contains0(*x) && *x != 1)
            .flat_map(|x| {
                let cnt = std::iter::successors(Some(x), move |acc| {
                    acc.checked_mul(x).filter(|x| (n % x == 0))
                })
                .count();
                std::iter::repeat(x).take(cnt)
            })
            .collect_vec();

        //dbg!(divisors.iter().copied().sorted().collect_vec());

        divisors.push(1);

        let palindrome_set = divisors
            .iter()
            .copied()
            .filter(|x| is_palindrome(*x))
            .collect::<HashSet<_>>();

        let having_pair_num = divisors
            .iter()
            .copied()
            .filter(|x| divisors.contains(&rev_number(*x)))
            .collect_vec();

        // dbg!(palindrome_set);
        // dbg!(having_pair_num);

        for &mid in &palindrome_set {
            let mut dp = vec![HashMap::<i64, i64>::new(); having_pair_num.len() + 1];
            dp[0].insert(mid, 0); // 0 はダミー

            for i in 0..having_pair_num.len() {
                // having_pair_num[i] を選ぶ or 選ばない
                let mut next_dp = HashMap::new();
                for &x in dp[i].keys() {
                    *next_dp.entry(x).or_insert(0) = x;
                    //next_dp.insert(x, x);
                    //let next = x * having_pair_num[i] * rev_number(having_pair_num[i]);
                    let next = x
                        .checked_mul(having_pair_num[i])
                        .and_then(|y| y.checked_mul(rev_number(having_pair_num[i])));
                    // n は next の倍数である必要がある
                    if let Some(next) = next {
                        if n % next == 0 {
                            *next_dp.entry(next).or_insert(0) = x;
                        }
                    }
                }
                dp[i + 1] = next_dp;
            }

            //dbg!(&dp[having_pair_num.len()]);
            if dp[having_pair_num.len()].contains_key(&n) {
                // dp の復元
                let mut current = n;
                let mut buf = vec![];
                for i in (0..having_pair_num.len()).rev() {
                    let prev = dp[i + 1][&current];
                    if current != prev {
                        buf.push(i);
                        current = prev;
                    }
                }

                // dbg!(buf);
                // dbg!(&having_pair_num);
                // dbg!(&having_pair_num.len());

                let mut factor = vec![];

                for &i in &buf {
                    if having_pair_num[i] == 1 {
                        continue;
                    }
                    factor.push(having_pair_num[i]);
                }
                if mid != 1 {
                    factor.push(mid);
                }

                for i in buf.iter().copied().rev() {
                    if having_pair_num[i] == 1 {
                        continue;
                    }
                    factor.push(rev_number(having_pair_num[i]));
                }

                // dbg!(&factor);

                let ans = factor.iter().copied().map(|x| x.to_string()).join("*");

                // dbg!(&ans);

                return Answer { ans: Some(ans) };
            }
        }

        let ans = None;
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // 解法: メモ化再帰
        let n = self.n;
        let divisors = divisors(n);
        let having_pair_num = divisors
            .iter()
            .copied()
            .filter(|x| divisors.contains(&rev_number(*x)) && !contains0(*x))
            .collect_vec();

        fn rec(
            k: i64,
            memo: &mut HashMap<i64, Option<VecDeque<i64>>>,
            having_pair_num: &[i64],
        ) -> Option<VecDeque<i64>> {
            if let Some(ans) = memo.get(&k) {
                return ans.clone();
            }
            let ans = if is_palindrome(k) && !contains0(k) {
                Some(VecDeque::from([k]))
            } else {
                let mut ans = None;
                for &x in having_pair_num {
                    if x == 1 {
                        continue;
                    }
                    let rev_x = rev_number(x);
                    if k % x == 0 && (k / x) % rev_x == 0 {
                        let sub_opt = rec(k / x / rev_x, memo, having_pair_num);
                        if let Some(sub) = sub_opt {
                            let mut sub = sub;
                            sub.push_front(x);
                            sub.push_back(rev_x);
                            ans = Some(sub);
                            break;
                        }
                    }
                }

                ans
            };
            memo.insert(k, ans.clone());
            ans
        }

        let ans = rec(n, &mut HashMap::new(), &having_pair_num);
        let ans = ans.map(|ans| ans.iter().copied().map(|x| x.to_string()).join("*"));
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
    ans: Option<String>,
}

impl Answer {
    fn print(&self) {
        if let Some(ans) = &self.ans {
            println!("{}", ans);
        } else {
            println!("-1");
        }
    }
}

fn main() {
    Problem::read().solve2().print();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};

    fn verify(p: &Problem, ans: &Answer) {
        let ans = if let Some(ans) = &ans.ans {
            ans
        } else {
            return;
        };

        let is_palindrome = {
            let ans = ans.bytes().collect_vec();
            Iterator::eq(ans.iter(), ans.iter().rev())
        };
        assert!(is_palindrome);

        let x = ans
            .split('*')
            .map(|s| s.parse::<i64>().unwrap())
            .product::<i64>();

        assert_eq!(p.n, x);
    }

    #[test]
    fn test_problem() {
        // for i in 1..=100000 {
        //     let p = Problem { n: i };
        //     let ans = p.solve();
        //     println!("{}, {:?}", i, ans);
        //     verify(&p, &ans);
        // }
        for i in 0..50 {
            let n = 1 << i;
            let p = Problem { n };
            let ans = p.solve();
            println!("{}, {:?}", n, ans);
            verify(&p, &ans);
        }
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

use itertools::rev;
// ====== import ======
#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, izip};
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::Reverse;
use std::collections::VecDeque;
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
use mod_number_thm::*;
pub mod mod_number_thm {
    use num::Integer;
    use num_integer::Roots;
    use std::collections::HashMap;
    /// O(sqrt(n))
    pub fn divisors(n: i64) -> Vec<i64> {
        assert!(n >= 1);
        let mut retval: Vec<i64> = Vec::new();
        for i in 1..=n.sqrt() {
            if n.is_multiple_of(&i) {
                retval.push(i);
                if i * i != n {
                    retval.push(n / i);
                }
            }
        }
        retval
    }
    /// 計算量: O(sqrt(n))
    pub fn is_prime(n: i64) -> bool {
        if n <= 1 {
            return false;
        }
        for i in 2..=n.sqrt() {
            if n.is_multiple_of(&i) {
                return false;
            }
        }
        true
    }
    /// 計算量: O(sqrt(n))
    pub fn prime_factorize(n: i64) -> HashMap<i64, i64> {
        assert!(n >= 1);
        let mut cnt_table: HashMap<i64, i64> = HashMap::new();
        let mut n = n;
        for i in 2..=n.sqrt() {
            if n.is_multiple_of(&i) {
                let mut cnt = 0;
                while n.is_multiple_of(&i) {
                    n /= i;
                    cnt += 1;
                }
                cnt_table.insert(i, cnt);
            }
        }
        if n != 1 {
            cnt_table.insert(n, 1);
        }
        cnt_table
    }
    /// 計算量: O(sqrt(n))
    pub fn euler_phi(n: i64) -> i64 {
        assert!(n >= 1);
        let pf = prime_factorize(n);
        let mut res = n;
        for p in pf.keys() {
            res = res / p * (p - 1);
        }
        res
    }
    pub struct Eratosthenes {
        is_prime_list: Vec<bool>,
        min_factor_list: Vec<Option<usize>>,
    }
    impl Eratosthenes {
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

use positional_notation::*;
#[allow(clippy::module_inception)]
pub mod positional_notation {
    pub fn eval_base_n_value(xs: &[i64], base: i64) -> i64 {
        xs.iter().fold(0, |acc, &x| acc * base + x)
    }
    pub fn to_base_n_value(x: i64, base: i64) -> Vec<i64> {
        let mut ret = vec![];
        let mut x = x;
        while x > 0 {
            ret.push(x % base);
            x /= base;
        }
        ret.reverse();
        ret
    }
}
