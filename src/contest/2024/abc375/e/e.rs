#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Player {
    team: usize,
    power: usize,
}
#[derive(Debug, Clone)]

struct Problem {
    n: usize,
    ps: Vec<Player>,
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! chmin {
    ($a: expr, $b: expr) => {
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    };
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            ps: [Player; n],
        }
        Problem { n, ps }
    }
    fn solve(&self) -> Answer {
        // 解法: dp[何人見た？][チーム1の強さ][チーム2の強さ] = 変更人数 というDPをする
        // そのままだと MLE になるので、next DP にする
        let n = self.n;
        let ps = &self.ps;
        let power_sum = ps.iter().copied().map(|p| p.power).sum::<usize>();

        if power_sum % 3 != 0 {
            return Answer { ans: None };
        }

        // NOTE: ここは power_sum/3 + 1 で良かった（そうすると MLE が避けられる）
        let mut dp = vec![vec![usize::MAX; power_sum + 1]; power_sum + 1];

        dp[0][0] = 0;

        for i in 0..n {
            let mut next_dp = vec![vec![usize::MAX; power_sum + 1]; power_sum + 1];

            for p1 in 0..=power_sum {
                for p2 in 0..=power_sum {
                    if dp[p1][p2] == usize::MAX {
                        continue;
                    }
                    // NOTE: next_dp[p1 + ps[i].power] は配列外参照しそうでやばい（実際には continue のおかげで配列外参照は発生しない）
                    // dp配列の長さを power_sum/3 + 1 で取る場合は配列外参照のケアが必要になる
                    chmin!(
                        next_dp[p1 + ps[i].power][p2],
                        dp[p1][p2] + (ps[i].team != 1) as usize
                    );
                    chmin!(
                        next_dp[p1][p2 + ps[i].power],
                        dp[p1][p2] + (ps[i].team != 2) as usize
                    );
                    chmin!(next_dp[p1][p2], dp[p1][p2] + (ps[i].team != 3) as usize);
                }
            }
            dp = next_dp;
        }
        let ans = dp[power_sum / 3][power_sum / 3];
        let ans = if ans == usize::MAX { None } else { Some(ans) };

        Answer { ans }
    }

    fn solve_naive(&self) -> Answer {
        // MLE する
        let n = self.n;
        let ps = &self.ps;
        let power_sum = ps.iter().copied().map(|p| p.power).sum::<usize>();

        if power_sum % 3 != 0 {
            return Answer { ans: None };
        }

        let mut dp = vec![vec![vec![usize::MAX; power_sum + 1]; power_sum + 1]; n + 1];

        dp[0][0][0] = 0;

        for i in 0..n {
            for p1 in 0..=power_sum {
                for p2 in 0..=power_sum {
                    if dp[i][p1][p2] == usize::MAX {
                        continue;
                    }

                    chmin!(
                        dp[i + 1][p1 + ps[i].power][p2],
                        dp[i][p1][p2] + (ps[i].team != 1) as usize
                    );
                    chmin!(
                        dp[i + 1][p1][p2 + ps[i].power],
                        dp[i][p1][p2] + (ps[i].team != 2) as usize
                    );
                    chmin!(
                        dp[i + 1][p1][p2],
                        dp[i][p1][p2] + (ps[i].team != 3) as usize
                    );
                }
            }
        }
        let ans = dp[n][power_sum / 3][power_sum / 3];
        let ans = if ans == usize::MAX { None } else { Some(ans) };
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Option<usize>,
}

impl Answer {
    fn print(&self) {
        if let Some(ans) = self.ans {
            println!("{}", ans);
        } else {
            println!("{}", -1);
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
