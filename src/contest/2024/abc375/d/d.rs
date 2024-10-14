//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    xs: Vec<u8>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            xs: Bytes,
        }
        Problem { xs }
    }

    fn solve(&self) -> Answer {
        // 解法: 3文字の回文の端っこの文字を固定する
        // \sum_{i=0}^{N-2} \sum_{j=i+1}^{N-1} 1[S_i = S_j] (j - i + 1) を解く話になる。
        // 計算量は O(N + アルファベットの数)
        let xs = &self.xs;
        // indexes[ch_i] == ch_i 番目のアルファベットが現れる場所(添字)のリスト
        let mut idxeses = vec![vec![]; 26];
        for (i, x) in xs.iter().copied().enumerate() {
            idxeses[(x - b'A') as usize].push(i);
        }
        let ans = idxeses
            .iter()
            .map(|idxes| {
                //
                let mut cnt = 0;
                let mut idx_sum = 0;

                for (i, idx) in idxes.iter().copied().enumerate() {
                    if i == 0 {
                        idx_sum += idx; // NOTE: これを忘れてた
                        continue;
                    }
                    let addition = i * (idx - 1) - idx_sum;
                    cnt += addition;
                    idx_sum += idx;
                }

                cnt as i64
            })
            .sum::<i64>();

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // 解法: 3文字の回文の真ん中のインデックスを固定する
        // 計算量は O(N * アルファベットの数)
        let xs = &self.xs;
        let n = xs.len();

        // indicators[ch_i][i] = if xs[i] == (ch_i番目のアルファベット) {1} else {0}
        let indicators = {
            let mut indicator = vec![vec![0; n]; 26];

            for (i, x) in xs.iter().copied().enumerate() {
                indicator[(x - b'A') as usize][i] = 1;
            }

            indicator
        };

        // indicator_cumsums[ch_i].range_sum(begin..end) == xs[begin..end] にある ch_i番目のアルファベットの数
        let indicator_cumsums = indicators.iter().map(|ind| CumSum::new(ind)).collect_vec();

        let ans = (0..n)
            .map(|mid| {
                // 3文字の回文の真ん中の index が mid であるとき、1文字目と3文字目の index の取り方は何通りあるか？
                // 各アルファベットに対して、1文字目と3文字目にそのアルファベットが現れるような場合の数を求めて足し合わせる。
                indicator_cumsums
                    .iter()
                    .map(|cumsum| cumsum.range_sum(..mid) * cumsum.range_sum(mid + 1..))
                    .sum::<i64>()
            })
            .sum::<i64>();

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
