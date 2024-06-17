//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    xs: Vec<usize>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            xs: [usize; n],
        }
        Problem { n, xs }
    }
    fn solve(&self) -> Answer {
        // [解法]
        // ソートすると Σ_i Σ_j xs[j]/xs[i] を求めれば良いとなる。
        // i のループでは xs を平方分割して、大きい方は BIT を使って計算し、小さい方は愚直に計算をする。

        let n = self.n;
        let xs = &self.xs.iter().copied().sorted().collect_vec();

        let xs_max = xs.iter().copied().max().unwrap();
        let xs_max_sqrt = xs_max.sqrt();

        // term1s[denom] には、今まで見た x in xs に対する x/denom の和を記録する
        let mut term1s = vec![0; xs_max_sqrt + 1];
        // bit[rumor] = 今まで見た x in xs のうち x == rumor となる x のカウント
        let mut bit = FenwickTree::new(xs_max + 1, 0);

        // 各 i に対して、sum_j xs[j]/xs[i] を保存する
        let mut buf = vec![];

        // 全体としての計算量は O(n √xs_max log(xs_max))
        for i in (0..n).rev() {
            if xs[i] > xs_max_sqrt {
                // xs[i] が小さすぎると、計算量が大きくなってしまう。
                // xs[i] が小さい場合は別の方法で計算をする。
                // O(√xs_max log(xs_max))
                let v = (0..=xs_max / xs[i])
                    .map(|range_i| {
                        let begin = usize::min(range_i * xs[i], xs_max);
                        let end = usize::min((range_i + 1) * xs[i], xs_max + 1);
                        bit.sum(begin..end) * range_i
                    })
                    .sum::<usize>();
                buf.push(v)
            } else {
                buf.push(term1s[xs[i]])
            }

            bit.add(xs[i], 1);
            // 計算量 O(√xs_max)
            // 上の処理と同じくらいの計算量になるように平方分割をした
            for denom in 1..=xs_max_sqrt {
                term1s[denom] += xs[i] / denom;
            }
        }

        let ans = buf.iter().sum::<usize>() as i64;
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // [解法]
        // ソートすると Σ_i Σ_j xs[j]/xs[i] を求めれば良いとなる。
        // 以下のような式変形をして、計算をする。
        //
        // sum_{i=1}^{N-1} sum_{j=i+1}^{N} A_j/A_i
        // = sum_{i=1}^{N-1} sum_{j=1}^{N} A_j/A_i
        //   - sum_{i=1}^{N-1} sum_{j=1}^{i} A_j/A_i
        //
        // 第1項では、調和級数ぽいループをして計算する。

        let xs = &self.xs.iter().copied().sorted().collect_vec();

        let xs_max = xs.iter().copied().max().unwrap();

        // cnts[k] = #[x in xs | x == k]
        let cnts = {
            let mut cnts = vec![0; xs_max + 1];
            for &x in xs {
                cnts[x] += 1;
            }
            cnts
        };
        let cnts_cumsum = CumSum::new(&cnts);

        // 各 d in 1..=xs_max に対して、sum_{x in xs} x/d を求める。
        // 計算量は xs_max/1 + xs_max/2 + ... + xs_max/xs_max = O(xs_max log(xs_max))
        let sum_div_ceil_list = (0..=xs_max)
            .map(|d| {
                if d == 0 {
                    0
                } else {
                    // 計算量 O(xs_max / d) (一応 log もつく)
                    (0..xs_max / d + 1)
                        .map(|range_i| {
                            let begin = range_i * d;
                            let end = ((range_i + 1) * d).min(xs_max + 1);
                            cnts_cumsum.range_sum(begin..end) * (range_i as i64)
                        })
                        .sum::<i64>() as usize
                }
            })
            .collect_vec();

        // sum_{i=1}^{N-1} sum_{j=i+1}^{N} A_j/A_i
        // = sum_{i=1}^{N-1} sum_{j=1}^{N} A_j/A_i    ...... term1
        //   - sum_{i=1}^{N-1} sum_{j=1}^{i} A_j/A_i  ...... term2
        //
        // term2 について
        // sum_{i=1}^{N-1} sum_{j=1}^{i} A_j/A_i は A_i = A_j で j <= i となる (i, j)の数
        // cnts を使えば計算できる。

        let term1 = xs
            .iter()
            .copied()
            // 前計算なしに毎回 sum_{r in xs} r/x を計算してると、例えば x = 1 となる x in xs が多いときに困る。
            .map(|x| sum_div_ceil_list[x])
            .sum::<usize>();

        let term2 = cnts
            .iter()
            .copied()
            .map(|cnt| {
                let cnt = cnt as usize;
                cnt * (cnt + 1) / 2
            })
            .sum::<usize>();

        let ans = term1 - term2;
        let ans = ans as i64;

        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        // 解法2のメモ化バージョン
        // 解法２ではあらかじめ計算していたが、解法3では必要になってから計算をする（都度メモ化する）

        let xs = &self.xs.iter().copied().sorted().collect_vec();

        let xs_max = xs.iter().copied().max().unwrap();

        // cnts[k] = #[x in xs | x == k]
        let cnts = {
            let mut cnts = vec![0; xs_max + 1];
            for &x in xs {
                cnts[x] += 1;
            }
            cnts
        };
        let cnts_cumsum = CumSum::new(&cnts);
        // (0..n).map(|j| xs[j]/d).sum() を求める

        let mut memo = vec![None; xs_max + 1];

        let calc = |d: usize| -> usize {
            if let Some(ans) = memo[d] {
                return ans;
            }

            let ans = (0..)
                .step_by(d)
                .take_while(|&x| x <= xs_max)
                .map(|begin| {
                    let end = usize::min(begin + d, xs_max + 1);
                    cnts_cumsum.range_sum(begin..end) as usize * (begin / d)
                })
                .sum();
            memo[d] = Some(ans);
            ans
        };

        let term1 = xs
            .iter()
            .copied()
            // 前計算なしに毎回 sum_{r in xs} r/x を計算してると、例えば x = 1 となる x in xs が多いときに困る。
            .map(calc)
            .sum::<usize>();

        let term2 = cnts
            .iter()
            .copied()
            .map(|cnt| {
                let cnt = cnt as usize;
                cnt * (cnt + 1) / 2
            })
            .sum::<usize>();

        let ans = term1 - term2;
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
    Problem::read().solve3().print();
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

use ac_library::FenwickTree;
// ====== import ======
#[allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
use num_integer::Roots;
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
