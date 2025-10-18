//#[derive_readable]
#[derive(Clone, Debug, PartialEq, Eq)]
struct Problem {
    n: usize,
    xs: Vec<i64>,
}

use ac_library::ModInt998244353 as Mint;

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            xs: [i64; n]
        }
        Problem { n, xs }
    }
    fn solve(&self) -> Answer {
        let n = self.n;
        let xs = &self.xs;
        let cnt_digits_pow10 = xs
            .iter()
            .copied()
            .map(|x| {
                let cnt = to_base_n_value(x, 10).len();
                // 10^cnt
                Mint::new(10).pow(cnt as u64)
            })
            .collect_vec();

        let cnt_digits_pow10_cumsum = CumSum::new(&cnt_digits_pow10);

        let term1 = {
            (0..n)
                .map(|i| Mint::new(i as i64) * Mint::new(xs[i]))
                .sum::<Mint>()
        };
        let term2 = {
            (0..n)
                .map(|i| -> ac_library::StaticModInt<ac_library::Mod998244353> {
                    Mint::new(xs[i]) * cnt_digits_pow10_cumsum.range_sum(i + 1..)
                })
                .sum::<Mint>()
        };
        let ans = term1 + term2;
        let ans = ans.val() as i64;
        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        let n = self.n;
        let mut sum: Mint = Mint::new(0);
        let xs = &self.xs;

        for i in 0..n - 1 {
            for j in i + 1..n {
                let c = xs[i].to_string() + xs[j].to_string().as_str();
                let c = c.parse::<i64>().unwrap();
                sum += Mint::new(c);
            }
        }
        let ans = sum.val() as i64;
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
        let n = rng.random_range(2..=10000);
        let xs = (0..n)
            .map(|_| {
                let x = rng.random_range(0..=8);
                let y = rng.random_range(0..=9);
                let z = 10_i64.pow(x as u32) * y;
                if z == 0 {
                    1
                } else {
                    z
                }
            })
            .collect_vec();
        let p = Problem { n, xs };
        // println!("{:?}", &p);
        p
    }

    #[allow(unreachable_code)]
    #[test]
    fn test_with_naive() {
        let num_tests = 5;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut rng = SmallRng::seed_from_u64(43);
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
// ====== snippet ======
use cumsum::*;
pub mod cumsum {
    use ac_library::ModInt998244353 as Mint;
    use std::ops::{Bound, Range, RangeBounds};
    pub struct CumSum {
        pub cumsum: Vec<Mint>,
    }
    impl CumSum {
        /// 計算量: O(|xs|)
        pub fn new(xs: &[Mint]) -> CumSum {
            let mut cumsum = vec![0.into(); xs.len() + 1];
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
        pub fn range_sum(&self, range: impl RangeBounds<usize>) -> Mint {
            let range = self.open(range);
            self.cumsum[range.end] - self.cumsum[range.start]
        }
        pub fn prefix_sum(&self, end: usize) -> Mint {
            self.cumsum[end]
        }
        pub fn suffix_sum(&self, begin: usize) -> Mint {
            self.cumsum[self.cumsum.len() - 1] - self.cumsum[begin]
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
