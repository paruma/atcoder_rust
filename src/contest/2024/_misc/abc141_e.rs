#[derive_readable]
#[derive(Debug)]
struct Problem {
    n: usize,
    s: Bytes,
}

impl Problem {
    fn read() -> Problem {
        input! {
            p: Problem
        }
        p
    }

    fn solve(&self) -> Answer {
        let n = self.n;
        let s = &self.s;
        let si64 = s
            .iter()
            .copied()
            .map(|ch| (ch - b'A' + 1) as i64)
            .collect_vec();
        let rh = RollingHash::new(&si64, 100);
        let pred = |len: i64| {
            let len = len as usize;
            // begin を動かしたときの s[begin..begin+len] の重複判定を行う
            let mut hash_to_begin: HashMap<i64, usize> = HashMap::new();
            for begin in 0..n - len + 1 {
                let hash = rh.hash(begin, begin + len);
                if let Some(&prev_begin) = hash_to_begin.get(&hash) {
                    if prev_begin + len <= begin {
                        // 前の区間と重ならない
                        return true;
                    }
                } else {
                    hash_to_begin.insert(hash, begin);
                }
            }
            false
        };
        let ans = bin_search(0, n as i64, pred);
        Answer { ans }
    }

    #[allow(dead_code)]

    fn solve_tle(&self) -> Answer {
        // これTLEする
        let n = self.n;
        let s = &self.s;
        let is_within = |i: usize| i < n;
        let si64 = s
            .iter()
            .copied()
            .map(|ch| (ch - b'A' + 1) as i64)
            .collect_vec();
        let rh = RollingHash::new(&si64, 100);
        let pred = |len: i64| {
            let len = len as usize;
            (0..n)
                .flat_map(|l1| (l1 + len..n).map(move |l2| (l1, l2)))
                .any(|(l1, l2)| {
                    // l1..l1+len と l2..l2+len での計算が全体的には重複していて無駄
                    // O(n)種類しかないのにO(n^2)回計算している
                    is_within(l2 + len - 1) && rh.hash(l1, l1 + len) == rh.hash(l2, l2 + len)
                })
        };
        let ans = bin_search(0, n as i64, pred);
        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive2(&self) -> Answer {
        let n = self.n;
        let s = &self.s;
        let pred = |len: i64| {
            let len = len as usize;
            (0..n)
                .flat_map(|l1| (l1 + len..n).map(move |l2| (l1, l2)))
                .any(|(l1, l2)| s.get(l1..l1 + len) == s.get(l2..l2 + len))
        };
        let ans = bin_search(0, n as i64, pred);
        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        let n = self.n;
        let s = &self.s;
        // 実際には二分探索をする
        let ans = (1..n)
            .filter(|len| {
                // l1 + len <= l2
                // s[l1..l1+len]  == s[l2..l2+len]
                // を満たすl1, l2 が存在するか
                (0..n)
                    .flat_map(|l1| (l1 + len..n).map(move |l2| (l1, l2)))
                    .any(|(l1, l2)| s.get(l1..l1 + len) == s.get(l2..l2 + len))
            })
            .max()
            .unwrap_or(0) as i64;
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

    fn test_random() {
        // let mut rng = SmallRng::from_entropy();
        // let n = rng.gen_range(1..=10);
        // let p = Problem { _a: n };
        // dbg!(&p);
        // assert_eq!(p.solve(), p.solve_naive());
    }

    #[test]
    fn test_random_all() {
        for _ in 0..100 {
            test_random();
        }
    }
}

use std::collections::HashMap;

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

use rolling_hash::*;
pub mod rolling_hash {
    const MOD: i128 = (1 << 61) - 1;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct ModIntPow261M1 {
        val: i128,
    }
    impl ModIntPow261M1 {
        #[inline]
        pub fn new(val: i128) -> Self {
            let val = (val + MOD) % MOD;
            Self {
                val: val.rem_euclid(MOD),
            }
        }
    }
    impl std::ops::Add for ModIntPow261M1 {
        type Output = Self;
        #[inline]
        fn add(self, rhs: Self) -> Self::Output {
            Self::new(self.val + rhs.val)
        }
    }
    impl std::ops::Sub for ModIntPow261M1 {
        type Output = Self;
        #[inline]
        fn sub(self, rhs: Self) -> Self::Output {
            Self::new(self.val - rhs.val)
        }
    }
    impl std::ops::Mul for ModIntPow261M1 {
        type Output = Self;
        #[inline]
        fn mul(self, rhs: Self) -> Self::Output {
            Self::new(self.val * rhs.val)
        }
    }
    pub struct RollingHash {
        hash_list: Vec<ModIntPow261M1>,
        pow_list: Vec<ModIntPow261M1>,
    }
    impl RollingHash {
        pub fn new(xs: &[i64], base: i64) -> Self {
            let base = ModIntPow261M1::new(base as i128);
            let mut hash_list = vec![ModIntPow261M1::new(0); xs.len() + 1];
            let mut pow_list = vec![ModIntPow261M1::new(1); xs.len() + 1];
            for i in 0..xs.len() {
                hash_list[i + 1] = hash_list[i] * base + ModIntPow261M1::new(xs[i] as i128);
                pow_list[i + 1] = pow_list[i] * base;
            }
            Self {
                hash_list,
                pow_list,
            }
        }
        pub fn hash(&self, begin: usize, end: usize) -> i64 {
            let x = self.hash_list[end] - self.hash_list[begin] * self.pow_list[end - begin];
            x.val as i64
        }
    }
}
