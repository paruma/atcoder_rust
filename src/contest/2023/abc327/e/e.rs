//#[derive_readable]
struct Problem {
    n_contest: usize,
    performance_list: Vec<i64>,
}

fn calc_rate(qs: &[i64]) -> f64 {
    let mut pow09 = 1.0;
    let mut pow09_sum = 0.0;
    let mut bunshi_sum = 0.0;
    for q in qs.iter().rev() {
        pow09_sum += pow09;
        bunshi_sum += pow09 * (*q as f64);
        pow09 *= 0.9;
    }
    bunshi_sum / pow09_sum - 1200.0 / (qs.len() as f64).sqrt()
}

const NEG_INF: f64 = -1000000000000000000000000000000000.0;
struct Dp {
    dp: Vec<Vec<f64>>,
}
impl Dp {
    fn new(n_contest: usize) -> Dp {
        Dp { dp: vec![vec![NEG_INF; n_contest + 1]; n_contest + 1] }
    }

    fn at(&self, all_cnt: usize, join_cnt: usize) -> f64 {
        // all_cnt: 今までのコンテストの数
        // join_cnt: 参加したコンテストの数
        if all_cnt < join_cnt {
            // panic!("何かがおかしい");
            NEG_INF
        } else {
            self.dp[all_cnt][join_cnt]
        }
    }

    fn at_mut(&mut self, all_cnt: usize, join_cnt: usize) -> &mut f64 {
        &mut self.dp[all_cnt][join_cnt]
    }
}

fn pow09_list(n: usize) -> Vec<f64> {
    let mut pow09 = 1.0;
    let mut buf: Vec<f64> = vec![1.0];
    for _ in 0..n {
        pow09 *= 0.9;
        buf.push(pow09)
    }
    buf
}

impl Problem {
    fn read() -> Problem {
        input! {
            n_contest: usize,
            performance_list: [i64; n_contest],
        }
        Problem { n_contest, performance_list }
    }
    fn solve(&self) -> Answer {
        let Problem { n_contest, performance_list } = self;
        let performance_list_rev = performance_list.iter().copied().rev().collect_vec();

        let pow09_list = pow09_list(*n_contest + 3);
        let mut dp = Dp::new(*n_contest);
        *dp.at_mut(0, 0) = 0.0;

        for (contest_i, &perf) in performance_list_rev.iter().enumerate() {
            for join_cnt in 0..=(contest_i + 1) {
                // join_cnt が負にならないように注意
                let no_choose = dp.at(contest_i, join_cnt);
                let choose = if join_cnt == 0 {
                    NEG_INF
                } else {
                    //
                    // dbg!(pow09_list[join_cnt - 1]);
                    // dbg!(perf);
                    dp.at(contest_i, join_cnt - 1) + pow09_list[join_cnt - 1] * (perf as f64)
                };
                *dp.at_mut(contest_i + 1, join_cnt) = f64::max(no_choose, choose);
            }
        }

        //print_vec(&dp.dp[*n_contest]);
        let pow09_sum_list = {
            let mut buf = vec![0.0; pow09_list.len() + 1];
            for (i, &x) in pow09_list.iter().enumerate() {
                buf[i + 1] = buf[i] + x;
            }

            buf
        };
        let ans = (1..=*n_contest)
            .map(|join_cnt| {
                dp.at(*n_contest, join_cnt) / pow09_sum_list[join_cnt]
                    - 1200.0 / (join_cnt as f64).sqrt()
            })
            .reduce(f64::max)
            .unwrap();
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Answer {
    ans: f64,
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

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
        //println!("{}", calc_rate(&[1000, 1200]));
        // print_vec(&pow09_list(5));
        let pow09_list = pow09_list(5);

        let pow09_sum_list = {
            let mut buf = vec![0.0; pow09_list.len() + 1];
            for (i, &x) in pow09_list.iter().enumerate() {
                buf[i + 1] = buf[i] + x;
            }

            buf
        };
        print_vec(&pow09_sum_list);
    }
}

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
