//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    k: usize,
    ps: Vec<usize>,
}

fn f(xs: &[u32]) -> u32 {
    let n = xs.len();
    let k = n / 2;

    (0..n - k + 1)
        .map(|begin| {
            let end = begin + k;
            let min = xs[begin..end].iter().min().unwrap();
            let max = xs[begin..end].iter().max().unwrap();
            max - min
        })
        .min()
        .unwrap()
}

fn g(xs: &[u32]) -> u32 {
    let n = xs.len();
    let k = n / 2;

    (0..n - k + 1)
        .map(|begin| {
            let end = begin + k;
            let min = xs[begin..end].iter().copied().min().unwrap();
            let max = xs[begin..end].iter().copied().max().unwrap();
            max - min
        })
        .min()
        .unwrap()
}

fn _main() {
    let xs = (0..200_000_u32).collect_vec();
    println!("{}", f(&xs));
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            k: usize,
            ps: [Usize1; n],
        }
        Problem { n, k, ps }
    }
    #[allow(clippy::redundant_clone)]
    fn solve(&self) -> Answer {
        // セグ木を使った解法
        let n = self.n;
        let k = self.k;
        let ps = &self.ps;
        let ps_inv = {
            let mut ps_inv = vec![0; n];
            for (i, &p) in ps.iter().enumerate() {
                ps_inv[p] = i;
            }
            ps_inv
        };

        let seg_min = Segtree::<Min<usize>>::from(ps_inv.clone());
        let seg_max = Segtree::<Max<usize>>::from(ps_inv.clone());

        let ans = (0..n - k + 1)
            .map(|begin| {
                let end = begin + k;

                let max = seg_max.prod(begin..end);
                let min = seg_min.prod(begin..end);
                max - min
            })
            .min()
            .unwrap();

        let ans = ans as i64;

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // BTreeSetを使った解法
        let n = self.n;
        let k = self.k;
        let ps = &self.ps;
        let ps_inv = {
            let mut ps_inv = vec![0; n];
            for (i, &p) in ps.iter().enumerate() {
                ps_inv[p] = i;
            }
            ps_inv
        };

        let mut set = ps_inv[0..k - 1].iter().copied().collect::<BTreeSet<_>>();

        let mut cand = vec![];

        for begin in 0..n - k + 1 {
            let end = begin + k;
            set.insert(ps_inv[end - 1]);

            let min = set.iter().copied().next().unwrap();
            let max = set.iter().copied().next_back().unwrap();

            cand.push(max - min);

            set.remove(&ps_inv[begin]);
        }

        let ans = cand.iter().copied().min().unwrap() as i64;

        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        // BTreeSetを使った解法 (拡張ライブラリ使用)
        let n = self.n;
        let k = self.k;
        let ps = &self.ps;
        let ps_inv = {
            let mut ps_inv = vec![0; n];
            for (i, &p) in ps.iter().enumerate() {
                ps_inv[p] = i;
            }
            ps_inv
        };

        let mut set = ps_inv[0..k - 1].iter().copied().collect::<BTreeSet<_>>();

        let mut cand = vec![];

        for begin in 0..n - k + 1 {
            let end = begin + k;
            set.insert(ps_inv[end - 1]);

            let min = set.all_min().unwrap();
            let max = set.all_max().unwrap();

            cand.push(max - min);

            set.remove(&ps_inv[begin]);
        }

        let ans = cand.iter().copied().min().unwrap() as i64;

        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        // 愚直 (TLE)
        // 計算量: (N-K)*K  (最大 10^10)
        // N, K<=2*10^5

        let n = self.n;
        let k = self.k;
        let ps = &self.ps;
        let ps_inv = {
            let mut ps_inv = vec![0; n];
            for (i, &p) in ps.iter().enumerate() {
                ps_inv[p] = i as u32;
            }
            ps_inv
        };

        let ans = (0..n - k + 1)
            .map(|begin| {
                let end = begin + k;
                let (min, max) = ps_inv[begin..end]
                    .iter()
                    .copied()
                    .fold((std::u32::MAX, 0), |(min, max), x| (min.min(x), max.max(x)));

                max - min
            })
            .min()
            .unwrap() as i64;

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
    Problem::read().solve_naive().print();
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
    fn make_random_problem() -> Problem {
        todo!()
        // let mut rng = SmallRng::from_entropy();
        // let n = rng.gen_range(1..=10);
        // let p = Problem { _a: n };
        // println!("{:?}", &p);
        // p
    }

    #[allow(unreachable_code)]
    #[test]
    fn test_with_naive() {
        return;
        let num_tests = 1000;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut wrong_cases: Vec<WrongTestCase> = vec![];
        for _ in 0..num_tests {
            let p = make_random_problem();
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

use ac_library::{Max, Min, Segtree};
// ====== import ======
#[allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};
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
use btree_set_ext::*;
#[allow(clippy::module_inception)]
pub mod btree_set_ext {
    use easy_ext::ext;
    use std::{collections::BTreeSet, ops::RangeBounds};
    #[ext(BTreeSetExt)]
    impl<T> BTreeSet<T>
    where
        T: Ord,
    {
        pub fn all_min(&self) -> Option<&T> {
            self.iter().next()
        }
        pub fn all_max(&self) -> Option<&T> {
            self.iter().next_back()
        }
        /// range との共通部分の中での最小値を返す
        pub fn range_min<R>(&self, range: R) -> Option<&T>
        where
            R: RangeBounds<T>,
        {
            self.range(range).next()
        }
        /// range との共通部分の中での最大値を返す
        pub fn range_max<R>(&self, range: R) -> Option<&T>
        where
            R: RangeBounds<T>,
        {
            self.range(range).next_back()
        }
    }
}
