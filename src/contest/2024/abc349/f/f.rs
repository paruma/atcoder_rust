//#[derive_readable]
#[derive(Debug)]
struct Problem {
    n: usize,
    m: i64,
    xs: Vec<i64>,
}

#[allow(clippy::question_mark)]
fn multi_lcm(xs: &[i64]) -> Option<i64> {
    let mut ans = 1;
    for x in xs.iter() {
        let gcd = gcd(ans, *x);
        let tmp = (ans / gcd).checked_mul(*x);
        if tmp.is_none() {
            return None;
        }
        ans = tmp.unwrap();
    }
    Some(ans)
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            m: i64,
            xs: [i64; n],
        }
        Problem { n, m, xs }
    }
    fn solve_pre_opt(&self) -> Answer {
        // 最適化前
        use ac_library::ModInt998244353 as Mint;
        let m = self.m;
        let xs = &self.xs;
        struct Info {
            p: i64,
            e: u32,
            v: i64, // p^e
        }
        impl Info {
            fn new(p: i64, e: u32) -> Self {
                let v = p.pow(e);
                Self { p, e, v }
            }
        }
        // 素因数分解の情報
        let m_fact = prime_factorize(m)
            .into_iter()
            .sorted_by_key(|(p, _e)| *p)
            .map(|(p, e)| Info::new(p, e as u32))
            .collect_vec();

        // xs のうち絶対に選択しないものは除外する
        // m が x の倍数でない場合は x を lcm に含めると必ず m にならないので除外する。
        let xs = xs.iter().copied().filter(|x| m % x == 0).collect_vec();

        // m = {p_1}^{e_1} * {p_2}^{e_2} * ... * {p_k_^{e_k} とする。
        // lcm を取るというのは素因数分解したときの各指数に max を取るという操作に対応する。
        // lcm を取って m にするというのは各iに対して、p_i の指数を e_i にするということに対応する。
        // p_i の指数 が e_i になっているかどうかを状態として持つことを考える。
        // x in xs に対して、x の素因数分解に p_i^{e_i} (指数がちょうど e_i) を含んでいるときに 1、そうでないときに0を立てたビット列を作る
        // 2つの値の lcm を取るというのは、このビット列の or に対応する。
        // 最終的に lcm を取って全ビット1にできたら、lcm が m になる。
        let xs_bit = xs
            .iter()
            .copied()
            .map(|x| {
                m_fact
                    .iter()
                    .map(|f| if x % f.v == 0 { 1 } else { 0 })
                    .fold(0, |acc, x| acc << 1 | x)
            })
            .collect_vec();
        let n = xs.len();

        // 空列はカウントに含めないことになっているので、いい感じに場合分けする
        if m == 1 {
            let ans = Mint::new(2).pow(xs.len() as u64) - 1;
            return Answer {
                ans: ans.val() as i64,
            };
        }

        // dp[k][b]: xs[0..k] で、揃った素因数が b というビット列で表現される場合の数

        let mut dp = vec![vec![Mint::new(0); 1 << m_fact.len()]; n + 1];
        dp[0][0] = if xs.is_empty() { 0.into() } else { 1.into() };

        // 配る
        for k in 0..n {
            for b in 0..1 << m_fact.len() {
                let addition = dp[k][b];
                // 選択しない
                dp[k + 1][b] += addition;

                // 選択する
                dp[k + 1][b | xs_bit[k]] += addition;
            }
        }
        let ans = dp[n][(1 << m_fact.len()) - 1].val() as i64;
        Answer { ans }
    }
    fn solve(&self) -> Answer {
        // 最適化後
        use ac_library::ModInt998244353 as Mint;
        let m = self.m;
        let xs = &self.xs;
        struct Info {
            p: i64,
            e: u32,
            v: i64, // p^e
        }
        impl Info {
            fn new(p: i64, e: u32) -> Self {
                let v = p.pow(e);
                Self { p, e, v }
            }
        }
        // 素因数分解の情報
        let m_fact = prime_factorize(m)
            .into_iter()
            .sorted_by_key(|(p, _e)| *p)
            .map(|(p, e)| Info::new(p, e as u32))
            .collect_vec();

        // xs のうち絶対に選択しないものは除外する
        let xs = xs.iter().copied().filter(|x| m % x == 0).collect_vec();
        let xs_bit = xs
            .iter()
            .copied()
            .map(|x| {
                m_fact
                    .iter()
                    .map(|f| if x % f.v == 0 { 1 } else { 0 })
                    .fold(0, |acc, x| acc << 1 | x)
            })
            .collect_vec();
        let n = xs.len();

        if m == 1 {
            let ans = Mint::new(2).pow(xs.len() as u64) - 1;
            return Answer {
                ans: ans.val() as i64,
            };
        }

        // dp[b]: 揃った素因数が b というビット列で表現される場合の数

        let mut dp = vec![Mint::new(0); 1 << m_fact.len()];
        dp[0] = if xs.is_empty() { 0.into() } else { 1.into() };

        for k in 0..n {
            let mut ndp = dp.clone();
            let xs_bit_k = xs_bit[k];
            for (b, cnt) in dp.iter().enumerate() {
                // ndp[b | xs_bit[k]] += dp[b]; の最適化
                unsafe {
                    *ndp.get_unchecked_mut(b | xs_bit_k) += cnt;
                }
            }
            dp = ndp;
        }

        let ans = dp[(1 << m_fact.len()) - 1].val() as i64;
        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        let ans = self
            .xs
            .iter()
            .copied()
            .powerset()
            .filter(|xs_sub| !xs_sub.is_empty())
            .map(|xs_sub| multi_lcm(&xs_sub))
            .filter(|x| *x == Some(self.m))
            .count()
            % 998244353;
        let ans = ans as i64;
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
        let mut rng = SmallRng::from_entropy();
        let n = 10;
        let m = rng.gen_range(1..=20); // 30とかでもテストすると良いかも
        let xs = (0..n).map(|_| rng.gen_range(1..=20)).collect_vec();

        let p = Problem { n, m, xs };
        println!("{:?}", &p);
        p
    }

    #[test]
    fn test_with_naive() {
        // 手動でテストを作るのもOK
        for _ in 0..10000 {
            let p = make_random_problem();
            check(&p);
        }
    }
}

// ====== import ======
#[allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
use num_integer::gcd;
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
        pub fn is_prime(&self, n: usize) -> bool {
            self.is_prime_list[n]
        }
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
