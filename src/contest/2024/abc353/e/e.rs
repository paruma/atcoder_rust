//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    strs: Vec<Vec<u8>>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            strs: [Bytes; n]
        }
        Problem { n, strs }
    }
    fn solve(&self) -> Answer {
        // 解法
        // 接頭辞の寄与を考える
        // 各接頭辞の数を数えるために、接頭辞をロリハにした。
        let rhs = self
            .strs
            .iter()
            .map(|str| {
                RollingHash::new(
                    &str.iter()
                        .copied()
                        .map(|ch| (ch - b'a' + 1) as i64)
                        .collect_vec(),
                    125,
                )
            })
            .collect_vec();

        // 各接頭辞のロリハとそのカウント
        let cnts = rhs
            .iter()
            .flat_map(|rh| (1..=rh.len()).map(|end| rh.hash(0, end)))
            .counts();

        // 正方形全体
        let term1 = cnts.values().map(|c| c * c).sum::<usize>();

        // 対角線
        let term2 = self.strs.iter().map(|str| str.len()).sum::<usize>();
        let ans = (term1 - term2) / 2;
        let ans = ans as i64;

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // 解法1 の平面走査バージョン
        let rhs = self
            .strs
            .iter()
            .map(|str| {
                RollingHash::new(
                    &str.iter()
                        .copied()
                        .map(|ch| (ch - b'a' + 1) as i64)
                        .collect_vec(),
                    125,
                )
            })
            .collect_vec();

        let mut ans = 0;
        let mut bag = HashBag::new();
        for rh in rhs {
            for end in 1..=rh.len() {
                let hash = rh.hash(0, end);
                ans += bag.contains(&hash);
                bag.insert(hash);
            }
        }

        let ans = ans as i64;

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
    Problem::read().solve2().print();
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

use hashbag::HashBag;
// ====== import ======
#[allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
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
    pub struct RollingHash {
        hash_list: Vec<ModInt261M1>,
        pow_list: Vec<ModInt261M1>,
        len: usize,
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
            let len = xs.len();
            Self {
                hash_list,
                pow_list,
                len,
            }
        }
        pub fn hash(&self, begin: usize, end: usize) -> i64 {
            let x = self.hash_list[end] - self.hash_list[begin] * self.pow_list[end - begin];
            x.val
        }

        pub fn len(&self) -> usize {
            self.len
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }
}
