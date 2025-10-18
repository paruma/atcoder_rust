#[derive(Debug, Clone)]
struct SubProblem {
    s: Vec<u8>,
    xs: Vec<u8>,
    ys: Vec<u8>,
}

impl SubProblem {
    fn read() -> SubProblem {
        input! {
            s: Bytes,
            xs: Bytes,
            ys: Bytes,
        }
        SubProblem { s, xs, ys }
    }
    fn solve(&self) -> bool {
        let base = ModInt261M1::new(50004);
        let s = self
            .s
            .iter()
            .copied()
            .map(|x| {
                let v = (x - b'a' + 1) as i64;
                RollingHash::unit(50004)(v)
            })
            .collect_vec();

        let s_len = s.len();

        let t_len = {
            let x_cnt_s = self.xs.iter().copied().filter(|x| *x == 0).count();
            let x_cnt_t = self.xs.iter().copied().filter(|x| *x == 1).count();

            let y_cnt_s = self.ys.iter().copied().filter(|x| *x == 0).count();
            let y_cnt_t = self.ys.iter().copied().filter(|x| *x == 1).count();

            let coef = x_cnt_t - y_cnt_t;
            let cst = y_cnt_s - x_cnt_s;
            if cst == 0 {
                0
            } else if coef == 0 || cst % coef != 0 {
                return false;
            } else {
                cst / coef
            }
        };

        // tの長さを求める

        let x_lens = self
            .xs
            .iter()
            .copied()
            .map(|x| (if x == b'0' { s_len } else { t_len }) as i64)
            .collect_vec();

        let y_lens = self
            .ys
            .iter()
            .copied()
            .map(|x| (if x == b'0' { s_len } else { t_len }) as i64)
            .collect_vec();

        let x_len_cumsum = CumSum::new(&x_lens);
        let y_len_cumsum = CumSum::new(&y_lens);

        let s = Segtree::<RollingHashConcat>::from(s);

        // s の最小周期を求める。
        let s_loop = {
            (1..=s_len)
                .find(|&l| {
                    if s_len % l != 0 {
                        return false;
                    }
                    let e = s_len / l;
                    let x = s.prod(0..l);

                    RollingHashConcat::pow(&x, e) == s.all_prod()
                })
                .unwrap()
        };

        // rhs 係数
        // rhs 定数
        // lhs 係数
        // lhs 係数

        let lhs_coef = self
            .xs
            .iter()
            .copied()
            .enumerate()
            .map(|(i, x)| {
                if x == b'0' {
                    // Sを使う
                    ModInt261M1::new(0)
                } else {
                    // Tを使う
                    ModInt261M1::pow(base, x_len_cumsum.range_sum(0..i) as usize)
                        * s.all_prod().get_hash_m()
                }
            })
            .fold(ModInt261M1::new(0), |acc, x| acc + x);

        let lhs_const = self
            .xs
            .iter()
            .copied()
            .enumerate()
            .map(|(i, x)| {
                if x == b'0' {
                    // Sを使う
                    ModInt261M1::pow(base, x_len_cumsum.range_sum(0..i) as usize)
                        * s.all_prod().get_hash_m()
                } else {
                    // Tを使う
                    ModInt261M1::new(0)
                }
            })
            .fold(ModInt261M1::new(0), |acc, x| acc + x);

        let rhs_coef = self
            .ys
            .iter()
            .copied()
            .enumerate()
            .map(|(i, x)| {
                if x == b'0' {
                    // Sを使う
                    ModInt261M1::new(0)
                } else {
                    // Tを使う
                    ModInt261M1::pow(base, y_len_cumsum.range_sum(0..i) as usize)
                        * s.all_prod().get_hash_m()
                }
            })
            .fold(ModInt261M1::new(0), |acc, x| acc + x);

        let rhs_const = self
            .ys
            .iter()
            .copied()
            .enumerate()
            .map(|(i, x)| {
                if x == b'0' {
                    // Sを使う
                    ModInt261M1::pow(base, y_len_cumsum.range_sum(0..i) as usize)
                        * s.all_prod().get_hash_m()
                } else {
                    // Tを使う
                    ModInt261M1::new(0)
                }
            })
            .fold(ModInt261M1::new(0), |acc, x| acc + x);

        // t のロリハを求める

        let trh =
            (rhs_const - lhs_const) * ModInt261M1::pow(lhs_coef - rhs_coef, (MOD - 2) as usize);

        let s_loop_rh = s.prod(0..s_loop);
        dbg!(trh);
        dbg!(s_loop_rh.get_hash());

        (0..5 * 100_000).any(|l| RollingHashConcat::pow(&s_loop_rh, l).get_hash() == trh.val)
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> usize {
        todo!();
    }
}

//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    ts: Vec<SubProblem>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
        }
        let ts = (0..n).map(|_| SubProblem::read()).collect_vec();
        Problem { n, ts }
    }
    fn solve(&self) -> Answer {
        let ans = self.ts.iter().map(|t| t.solve()).collect_vec();
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
    ans: Vec<bool>,
}

impl Answer {
    fn print(&self) {
        for &x in &self.ans {
            print_yesno(x)
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

use ac_library::{FenwickTree, Segtree};
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
use monoid_rolling_hash::*;
pub mod monoid_rolling_hash {
    use std::convert::Infallible;
    pub const MOD: i64 = (1 << 61) - 1;
    const MOD_I128: i128 = (1 << 61) - 1;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ModInt261M1 {
        pub val: i64,
    }
    impl ModInt261M1 {
        #[inline]
        pub fn new(val: i64) -> Self {
            Self { val }
        }

        pub fn pow(base: Self, n: usize) -> Self {
            let mut base = base.clone();
            let mut ans = Self::new(1);
            let mut n = n;
            while n > 0 {
                if n & 1 == 1 {
                    ans = ans * base;
                }
                base = base * base;
                n >>= 1;
            }
            ans
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
    use ac_library::Monoid;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct RollingHash {
        hash: ModInt261M1,
        base: ModInt261M1,
    }
    impl RollingHash {
        pub fn get_hash(&self) -> i64 {
            self.hash.val
        }
        pub fn get_hash_m(&self) -> ModInt261M1 {
            self.hash
        }
        pub fn unit(base: i64) -> impl (Fn(i64) -> RollingHash) {
            move |x| RollingHash {
                hash: ModInt261M1::new(x),
                base: ModInt261M1::new(base),
            }
        }
        pub fn new(hash: i64, base: i64) -> Self {
            Self {
                hash: ModInt261M1::new(hash),
                base: ModInt261M1::new(base),
            }
        }
    }
    pub struct RollingHashConcat(Infallible);
    impl Monoid for RollingHashConcat {
        type S = RollingHash;
        fn identity() -> Self::S {
            RollingHash {
                hash: ModInt261M1::new(0),
                base: ModInt261M1::new(1),
            }
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            RollingHash {
                hash: a.hash * b.base + b.hash,
                base: a.base * b.base,
            }
        }
    }
}

use extend_acl_monoid::*;
pub mod extend_acl_monoid {
    use ac_library::Monoid;
    pub trait MonoidExtPow: Monoid {
        /// base^n を求める
        fn pow(base: &Self::S, n: usize) -> Self::S {
            let mut base = base.clone();
            let mut ans = Self::identity();
            let mut n = n;
            while n > 0 {
                if n & 1 == 1 {
                    ans = Self::binary_operation(&ans, &base);
                }
                base = Self::binary_operation(&base, &base);
                n >>= 1;
            }
            ans
        }
    }
    impl<T> MonoidExtPow for T where T: Monoid {}
}
use cumsum::*;
pub mod cumsum {
    pub fn prefix_sum(xs: &[i64]) -> Vec<i64> {
        let mut prefix_sum = vec![0; xs.len() + 1];
        for i in 1..xs.len() + 1 {
            prefix_sum[i] = prefix_sum[i - 1] + xs[i - 1];
        }
        prefix_sum
    }
    use std::ops::{Bound, Range, RangeBounds};
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct CumSum {
        pub cumsum: Vec<i64>,
    }
    impl CumSum {
        /// 計算量: O(|xs|)
        pub fn new(xs: &[i64]) -> CumSum {
            let mut cumsum = vec![0; xs.len() + 1];
            for i in 1..xs.len() + 1 {
                cumsum[i] = cumsum[i - 1] + xs[i - 1];
            }
            CumSum { cumsum }
        }
        fn open(&self, range: impl RangeBounds<usize>) -> Range<usize> {
            use Bound::Excluded;
            use Bound::Included;
            use Bound::Unbounded;
            let begin = match range.start_bound() {
                Unbounded => 0,
                Included(&x) => x,
                Excluded(&x) => x + 1,
            };
            let end = match range.end_bound() {
                Excluded(&x) => x,
                Included(&x) => x + 1,
                Unbounded => self.cumsum.len() - 1,
            };
            begin..end
        }
        /// 計算量: O(1)
        pub fn range_sum(&self, range: impl RangeBounds<usize>) -> i64 {
            let range = self.open(range);
            self.cumsum[range.end] - self.cumsum[range.start]
        }
        pub fn prefix_sum(&self, end: usize) -> i64 {
            self.cumsum[end]
        }
        pub fn suffix_sum(&self, begin: usize) -> i64 {
            self.cumsum[self.cumsum.len() - 1] - self.cumsum[begin]
        }
    }
}
