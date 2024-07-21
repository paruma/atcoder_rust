//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    k: usize,
    s: Vec<char>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            k: usize,
            s: Chars
        }
        Problem { n, k, s }
    }
    fn solve(&self) -> Answer {
        // 解法: next_permutation を使う
        let n = self.n;
        let k = self.k;
        use permutohedron::LexicalPermutation;

        let mut cnt = 0;

        let mut s = self.s.clone();
        s.sort();

        while {
            let is_ok = s.windows(k).all(|t_sub| {
                // t_sub が回文でないことを調べる
                //t_sub != t_sub.iter().copied().rev().collect_vec() // こう書くと実行時間500ms
                !t_sub.iter().eq(t_sub.iter().rev()) // こう書くと実行時間50ms
            });

            cnt += is_ok as i64;

            s.next_permutation()
        } {}

        let ans = cnt;

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // permutations & 重複カウントを階乗で割る
        let s = &self.s;
        let k = self.k;
        let numerator = s
            .iter()
            .copied()
            .permutations(s.len())
            .filter(|t| {
                t.windows(k).all(|t_sub| {
                    // t_sub が回文でないことを調べる
                    t_sub != t_sub.iter().copied().rev().collect_vec()
                })
            })
            .count();

        let denominator = {
            let cnts = s.iter().copied().counts().values().copied().collect_vec();

            cnts.iter()
                .map(|c| (1..=*c).product::<usize>())
                .product::<usize>()
        };

        let ans = numerator / denominator;
        let ans = ans as i64;

        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        // permutations & unique (1500ms)
        // unique 内部では HashMap を使っているので、O(n) で unique は計算できるが、
        // 動的メモリ確保をしているので定数倍が重い
        let s = &self.s;
        let k = self.k;
        let ans = s
            .iter()
            .copied()
            .permutations(s.len())
            .filter(|t| {
                t.windows(k).all(|t_sub| {
                    // t_sub が回文でないことを調べる
                    !t_sub.iter().eq(t_sub.iter().rev())
                })
            })
            .unique()
            .count();

        let ans = ans as i64;
        Answer { ans }
    }

    fn solve4(&self) -> Answer {
        // next_permutations & ロリハ
        // 794ms O(N! N) なのにとても遅い (愚直の O(N! NK) は 50ms)
        // N! 回動的に配列確保しているのが遅い原因ぽさそう。
        let n = self.n;
        let k = self.k;
        use permutohedron::LexicalPermutation;

        let mut cnt = 0;

        let mut s = self.s.clone();
        s.sort();

        while {
            let s_i64 = s
                .iter()
                .copied()
                .map(|ch| (ch as i64) - ('a' as i64))
                .collect_vec();
            let s_rev = s_i64.iter().copied().rev().collect_vec();
            let s_rh = RollingHash::new(&s_i64, 363);
            let s_rev_rh = RollingHash::new(&s_rev, 363);
            let is_ok = (0..=n - k).all(|begin| {
                let end = begin + k;
                s_rh.hash(begin, end) != s_rev_rh.hash(n - end, n - begin)
            });

            cnt += is_ok as i64;

            s.next_permutation()
        } {}

        let ans = cnt;

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
use rolling_hash::*;
pub mod rolling_hash {
    const MOD: i64 = (1 << 61) - 1;
    const MOD_I128: i128 = (1 << 61) - 1;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct ModInt261M1 {
        val: i64,
    }
    impl ModInt261M1 {
        #[inline]
        pub fn new(val: i64) -> Self {
            Self { val }
        }
    }
    impl std::ops::Add for ModInt261M1 {
        type Output = Self;
        #[inline]
        fn add(self, rhs: Self) -> Self::Output {
            let mut x = self.val + rhs.val;
            if x >= MOD {
                x -= MOD;
            }
            Self::new(x)
        }
    }
    impl std::ops::Sub for ModInt261M1 {
        type Output = Self;
        #[inline]
        fn sub(self, rhs: Self) -> Self::Output {
            let mut x = MOD + self.val - rhs.val;
            if x >= MOD {
                x -= MOD;
            }
            Self::new(x)
        }
    }
    impl std::ops::Mul for ModInt261M1 {
        type Output = Self;
        #[inline]
        fn mul(self, rhs: Self) -> Self::Output {
            let x = (self.val as i128) * (rhs.val as i128);
            let mut x = ((x >> 61) + (x & MOD_I128)) as i64;
            if x >= MOD {
                x -= MOD;
            }
            Self::new(x)
        }
    }
    #[derive(Clone, Debug)]
    pub struct RollingHash {
        hash_list: Vec<ModInt261M1>,
        pow_list: Vec<ModInt261M1>,
        length: usize,
    }
    impl RollingHash {
        pub fn new(xs: &[i64], base: i64) -> Self {
            let base = ModInt261M1::new(base);
            let mut hash_list = vec![ModInt261M1::new(0); xs.len() + 1];
            let mut pow_list = vec![ModInt261M1::new(1); xs.len() + 1];
            for i in 0..xs.len() {
                hash_list[i + 1] = hash_list[i] * base + ModInt261M1::new(xs[i]);
                pow_list[i + 1] = pow_list[i] * base;
            }
            let length = xs.len();
            Self {
                hash_list,
                pow_list,
                length,
            }
        }
        pub fn hash(&self, begin: usize, end: usize) -> i64 {
            let x = self.hash_list[end] - self.hash_list[begin] * self.pow_list[end - begin];
            x.val
        }
        pub fn len(&self) -> usize {
            self.length
        }
        pub fn is_empty(&self) -> bool {
            self.length == 0
        }
    }
}
