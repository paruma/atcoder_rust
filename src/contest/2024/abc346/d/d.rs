//#[derive_readable]
struct Problem {
    n: usize,
    s: Vec<usize>,
    cs: Vec<i64>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            s: Bytes,
            cs: [i64; n],
        }
        let s = s.iter().copied().map(|x| (x - b'0') as usize).collect_vec();
        Problem { n, s, cs }
    }
    fn solve(&self) -> Answer {
        let n = self.n;
        let s = &self.s;

        let mut dp = vec![vec![vec![i64::MAX; 2]; 2]; n + 1];

        dp[0][0][0] = 0;
        dp[0][0][1] = 0;

        for i in 0..n {
            // 00, 11 がないパターンdp[i+1][0][-]
            for last in [0_usize, 1] {
                let cost = if s[i] == last { 0 } else { self.cs[i] };
                dp[i + 1][0][last] = dp[i][0][1 - last] + cost;
            }

            if i == 0 {
                continue;
            }
            // 00, 11 があるパターン dp[i+1][1][-]
            for last in [0_usize, 1] {
                let cost = if s[i] == last { 0 } else { self.cs[i] };
                dp[i + 1][1][last] = i64::min(dp[i][0][last], dp[i][1][1 - last]) + cost;
            }
        }
        let ans = i64::min(dp[n][1][0], dp[n][1][1]);
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

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
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
