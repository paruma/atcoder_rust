//#[derive_readable]

use ac_library::ModInt998244353 as Mint;
struct Problem {
    n: i64,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: i64,
        }
        Problem { n }
    }
    fn solve(&self) -> Answer {
        let n = self.n as usize;
        let mut dp = vec![vec![Mint::new(0); n + 2]; n + 2];

        dp[0][0] = Mint::new(1);
        let inv2 = Mint::new(2).inv();

        for i in 1..n {
            dp[i][0] = {
                let mut tmp = Mint::new(0);
                let mut inv2pow = inv2;
                let mut pow2 = Mint::new(2);
                for j in 0..i {
                    tmp += dp[i - 1][j] * inv2pow;
                    inv2pow *= inv2;
                    pow2 *= Mint::new(2);
                }

                tmp * pow2 / (pow2 - 1)
            };
            for j in (1..=i).rev() {
                let tmp = (dp[i - 1][j] + dp[i][(j + 1) % (i + 1)]) * inv2;
                dp[i][j] += tmp;
            }
        }

        let ans = dp[n - 1]
            .iter()
            .copied()
            .rev()
            .map(|x| x.val())
            .collect_vec()[2..]
            .to_vec();
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<u32>,
}

impl Answer {
    fn print(&self) {
        print_vec_1line(&self.ans);
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
