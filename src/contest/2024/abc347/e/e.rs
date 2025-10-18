// #[derive_readable]
#[derive(Debug)]
struct Problem {
    n: usize,
    nq: usize,
    xs: Vec<usize>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            nq: usize,
            xs: [Usize1; nq],
        }
        Problem { n, nq, xs }
    }
    fn solve(&self) -> Answer {
        let nq = self.nq;
        let xs = &self.xs;
        let mut s_size_list = vec![0; self.nq];
        let mut set = vec![false; self.n];
        // 各 x in 0..n がどの区間で合われたり消えたりするかを調べる。
        let mut appear_list: Vec<Vec<usize>> = vec![vec![]; self.n];

        // set を HashSet で持って、s_size_list は set.len() で計算すればよかった
        for (i, x) in xs.iter().enumerate() {
            if set[*x] {
                s_size_list[i] = s_size_list[i - 1] - 1;
                set[*x] = false;
            } else {
                if i == 0 {
                    s_size_list[i] = 1;
                } else {
                    s_size_list[i] = s_size_list[i - 1] + 1;
                }
                set[*x] = true;
            }

            appear_list[*x].push(i);
        }

        //s_size_list の累積和をとる

        let s_size_list_cumsum = CumSum::new(&s_size_list);

        let ans = appear_list
            .iter()
            .map(|appear_info| {
                if appear_info.len() % 2 == 0 {
                    (0..appear_info.len() / 2)
                        .map(|i| {
                            let begin = appear_info[i * 2];
                            let end = appear_info[i * 2 + 1];
                            s_size_list_cumsum.range_sum(begin, end)
                        })
                        .sum::<i64>()
                } else {
                    let x1 = (0..appear_info.len() / 2)
                        .map(|i| {
                            let begin = appear_info[i * 2];
                            let end = appear_info[i * 2 + 1];
                            s_size_list_cumsum.range_sum(begin, end)
                        })
                        .sum::<i64>();
                    let x2 = s_size_list_cumsum
                        .range_sum(*appear_info.last().unwrap(), s_size_list.len());
                    x1 + x2
                }
            })
            .collect_vec();

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // solve のリファクタリング(set を Vec<bool> ではなく、HashSet で持つ)
        let nq = self.nq;
        let xs = &self.xs;

        // これに fold をするのはやりすぎかもしれない（単純にブロック内に mut を閉じ込めるだけでも良かったかも）
        let (s_size_list, _) = xs.iter().enumerate().fold(
            (vec![0; self.nq], HashSet::<usize>::new()),
            |(mut acc_s_size_list, mut acc_set), (i, x)| {
                if acc_set.contains(x) {
                    acc_set.remove(x);
                } else {
                    acc_set.insert(*x);
                }
                acc_s_size_list[i] = acc_set.len() as i64;
                (acc_s_size_list, acc_set)
            },
        );

        // 各 x in 0..n がどの区間で合われたり消えたりするかを調べる。
        let appear_list = xs.iter().copied().enumerate().fold(
            vec![vec![]; self.n],
            |mut acc_appear_list, (i, x)| {
                acc_appear_list[x].push(i);
                acc_appear_list
            },
        );

        //s_size_list の累積和をとる
        let s_size_list_cumsum = CumSum::new(&s_size_list);

        let ans = appear_list
            .iter()
            .map(|appear_info| {
                if appear_info.len() % 2 == 0 {
                    (0..appear_info.len() / 2)
                        .map(|i| {
                            let begin = appear_info[i * 2];
                            let end = appear_info[i * 2 + 1];
                            s_size_list_cumsum.range_sum(begin, end)
                        })
                        .sum::<i64>()
                } else {
                    let x1 = (0..appear_info.len() / 2)
                        .map(|i| {
                            let begin = appear_info[i * 2];
                            let end = appear_info[i * 2 + 1];
                            s_size_list_cumsum.range_sum(begin, end)
                        })
                        .sum::<i64>();
                    let x2 = s_size_list_cumsum.suffix_sum(*appear_info.last().unwrap());
                    x1 + x2
                }
            })
            .collect_vec();

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
    ans: Vec<i64>,
}

impl Answer {
    fn print(&self) {
        print_vec_1line(&self.ans);
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

    fn check(p: &Problem) {
        assert_eq!(p.solve(), p.solve_naive());
    }

    fn make_random_problem() -> Problem {
        todo!()
        // let mut rng = SmallRng::from_os_rng();
        // let n = rng.random_range(1..=10);
        // let p = Problem { _a: n };
        // dbg!(&p);
        // p
    }

    #[test]
    fn test_with_naive() {
        // 手動でテストを作るのもOK
        for _ in 0..100 {
            // let p = make_random_problem();
            // check(&p);
        }
    }
}

use std::collections::HashSet;

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

use cumsum::*;
pub mod cumsum {
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
        /// 計算量: O(1)
        pub fn range_sum(&self, begin: usize, end: usize) -> i64 {
            self.cumsum[end] - self.cumsum[begin]
        }
        pub fn prefix_sum(&self, end: usize) -> i64 {
            self.cumsum[end]
        }
        pub fn suffix_sum(&self, begin: usize) -> i64 {
            self.cumsum[self.cumsum.len() - 1] - self.cumsum[begin]
        }
    }
}
