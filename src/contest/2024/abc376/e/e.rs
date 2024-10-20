//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    k: usize,
    xs: Vec<i64>,
    ys: Vec<i64>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            k: usize,
            xs: [i64; n],
            ys: [i64; n],
        }
        Problem { n, k, xs, ys }
    }

    fn solve(&self) -> Answer {
        let n = self.n;
        let k = self.k;
        let xys = izip!(self.xs.iter().copied(), self.ys.iter().copied())
            .sorted_by_key(|(x, y)| (*y, *x))
            .collect_vec();

        let xs = xys.iter().copied().map(|(x, _)| x).collect_vec();
        let ys = xys.iter().copied().map(|(_, y)| y).collect_vec();

        let mut xs_bag = xs
            .iter()
            .copied()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect::<BTreeSet<(i64, usize)>>();

        let mut ys_bag_current = ys[0..k - 1]
            .iter()
            .copied()
            .enumerate()
            .map(|(i, y)| (y, i))
            .collect::<BTreeSet<(i64, usize)>>();

        let mut ys_bag_current_sum = ys_bag_current.iter().copied().map(|(y, _)| y).sum::<i64>();

        // ys[k - 1..].iter().copied().enumerate() だとダメ
        let mut ys_bag_cand = (k - 1..n).map(|i| (ys[i], i)).collect::<BTreeSet<_>>();

        let mut ans = i64::MAX;

        while xs_bag.len() >= k {
            let (x, i) = *xs_bag.iter().max().unwrap();
            let max = x;
            let sum = if ys_bag_current.contains(&(ys[i], i)) {
                //
                if ys_bag_cand.is_empty() {
                    break;
                }
                let (y, _) = ys_bag_cand.iter().min().unwrap();
                ys_bag_current_sum + y
            } else {
                ys_bag_current_sum + ys[i]
            };
            ans = ans.min(max * sum);

            xs_bag.remove(&(x, i));
            ys_bag_cand.remove(&(ys[i], i));

            if ys_bag_current.remove(&(ys[i], i)) {
                ys_bag_current_sum -= ys[i];
                if ys_bag_cand.is_empty() {
                    break;
                }
                let (y, yi) = *ys_bag_cand.iter().min().unwrap();
                ys_bag_current.insert((y, yi));
                ys_bag_cand.remove(&(y, yi));
                ys_bag_current_sum += y;
            }
        }

        Answer { ans }
    }

    fn solve_wrong(&self) -> Answer {
        let xys = izip!(self.xs.iter().copied(), self.ys.iter().copied())
            .sorted_by_key(|(x, y)| (*y, *x))
            .collect_vec();

        let xs = xys.iter().copied().map(|(x, _)| x).collect_vec();
        let ys = xys.iter().copied().map(|(_, y)| y).collect_vec();
        let cummax_xs = CumMax::new(&xs);
        let cumsum_ys = CumSum::new(&ys);

        let n = self.n;
        let k = self.k;
        let ans = (k - 1..n)
            .map(|i| {
                // [0, k-1) と i を使う
                let max = i64::max(cummax_xs.prefix_max(k - 1), xs[i]);
                let sum = cumsum_ys.prefix_sum(k - 1) + ys[i];
                max * sum
            })
            .min()
            .unwrap();

        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        let k = self.k;
        let n = self.n;
        let xs = &self.xs;
        let ys = &self.ys;

        let ans = (0..n)
            .combinations(k)
            .map(|s| {
                let max = s.iter().copied().map(|i| xs[i]).max().unwrap();
                let sum = s.iter().copied().map(|i| ys[i]).sum::<i64>();
                max * sum
            })
            .min()
            .unwrap();
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
    input! {
        t: usize
    }
    for _ in 0..t {
        Problem::read().solve().print();
    }
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
        let n = rng.gen_range(1..=3);
        let k = rng.gen_range(1..=n);
        let xs = (0..n).map(|_| rng.gen_range(1..10)).collect_vec();
        let ys = (0..n).map(|_| rng.gen_range(1..10)).collect_vec();
        let p = Problem { n, k, xs, ys };
        println!("{:?}", &p);
        p
    }

    #[allow(unreachable_code)]
    #[test]
    fn test_with_naive() {
        let num_tests = 1000;
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
use std::collections::BTreeSet;
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
use cum_monoid::*;
pub mod cum_monoid {
    use ac_library::{Max, Min, Monoid};
    pub struct CumMonoid<M>
    where
        M: Monoid,
    {
        prefix_prod: Vec<M::S>,
        suffix_prod: Vec<M::S>,
    }
    impl<M> CumMonoid<M>
    where
        M: Monoid,
    {
        pub fn new(xs: &[M::S]) -> CumMonoid<M> {
            let mut prefix_prod = vec![M::identity(); xs.len() + 1];
            let mut suffix_prod = vec![M::identity(); xs.len() + 1];
            for i in 0..xs.len() {
                prefix_prod[i + 1] = M::binary_operation(&prefix_prod[i], &xs[i]);
            }
            for i in (0..xs.len()).rev() {
                suffix_prod[i] = M::binary_operation(&xs[i], &suffix_prod[i + 1]);
            }
            CumMonoid {
                prefix_prod,
                suffix_prod,
            }
        }
        /// [0, i) の総積 (前から累積)
        pub fn prefix_prod(&self, i: usize) -> M::S {
            self.prefix_prod[i].clone()
        }
        /// [i, n) の総積 (後ろから累積)
        pub fn suffix_prod(&self, i: usize) -> M::S {
            self.suffix_prod[i].clone()
        }
        /// [0, i), [i + 1, n) の区間で総積を取る
        pub fn prod_without1(&self, i: usize) -> M::S {
            M::binary_operation(&self.prefix_prod[i], &self.suffix_prod[i + 1])
        }
        pub fn prod_without_range(&self, l: usize, r: usize) -> M::S {
            M::binary_operation(&self.prefix_prod[l], &self.suffix_prod[r])
        }
    }
    pub struct CumMin {
        cum: CumMonoid<Min<i64>>,
    }
    impl CumMin {
        pub fn new(xs: &[i64]) -> CumMin {
            CumMin {
                cum: CumMonoid::new(xs),
            }
        }
        /// [0, i) の総積 (前から累積)
        pub fn prefix_min(&self, i: usize) -> i64 {
            self.cum.prefix_prod(i)
        }
        /// [i, n) の総積 (後ろから累積)
        pub fn suffix_min(&self, i: usize) -> i64 {
            self.cum.suffix_prod(i)
        }
        /// [0, i), [i + 1, n) の区間で総積を取る
        pub fn min_without1(&self, i: usize) -> i64 {
            self.cum.prod_without1(i)
        }
        pub fn min_without_range(&self, l: usize, r: usize) -> i64 {
            self.cum.prod_without_range(l, r)
        }
    }
    pub struct CumMax {
        cum: CumMonoid<Max<i64>>,
    }
    impl CumMax {
        pub fn new(xs: &[i64]) -> CumMax {
            CumMax {
                cum: CumMonoid::new(xs),
            }
        }
        /// [0, i) の総積 (前から累積)
        pub fn prefix_max(&self, i: usize) -> i64 {
            self.cum.prefix_prod(i)
        }
        /// [i, n) の総積 (後ろから累積)
        pub fn suffix_max(&self, i: usize) -> i64 {
            self.cum.suffix_prod(i)
        }
        /// [0, i), [i + 1, n) の区間で総積を取る
        pub fn max_without1(&self, i: usize) -> i64 {
            self.cum.prod_without1(i)
        }
        pub fn max_without_range(&self, l: usize, r: usize) -> i64 {
            self.cum.prod_without_range(l, r)
        }
    }
}
