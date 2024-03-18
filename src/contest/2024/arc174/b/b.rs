struct TestCase {
    xs: Vec<i64>,
    ps: Vec<i64>,
}
struct Problem {
    test_cases: Vec<TestCase>,
}

impl TestCase {
    fn solve(&self) -> i64 {
        let n_review = self.xs.iter().sum::<i64>();
        let rating_sum = self
            .xs
            .iter()
            .copied()
            .enumerate()
            .map(|(i, x)| x * (i as i64 + 1))
            .sum::<i64>();

        // 賄賂不要なケース
        if rating_sum >= n_review * 3 {
            // rating_sum / n_review が平均評価
            return 0;
        }

        let x4 = self.xs[3];
        let x5 = self.xs[4];

        let p4 = self.ps[3];
        let p5 = self.ps[4];

        // 4を買う
        let cnt4 = 3 * n_review - rating_sum;
        let price4 = p4 * cnt4;

        // 5を買う
        let cnt5 = (3 * n_review - rating_sum).div_ceil(&2);
        let price5 = p5 * cnt5;

        i64::min(price4, price5)
    }
}

impl Problem {
    fn read() -> Problem {
        input! {
            n_cases: usize,
        }
        let test_cases = (0..n_cases)
            .map(|_| {
                input! {
                    xs: [i64; 5],
                    ps: [i64; 5],
                }
                TestCase { xs, ps }
            })
            .collect_vec();
        Problem { test_cases }
    }
    fn solve(&self) -> Answer {
        let ans = self.test_cases.iter().map(|t| t.solve()).collect_vec();

        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<i64>,
}

impl Answer {
    fn print(&self) {
        print_vec(&self.ans);
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
    }
}

// ====== import ======
#[allow(unused_imports)]
use itertools::Itertools;
use num::Integer;
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
