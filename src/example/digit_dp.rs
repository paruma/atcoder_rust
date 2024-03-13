// https://atcoder.jp/contests/tdpc/tasks/tdpc_number

use ac_library::ModInt1000000007 as Mint;
struct Problem {
    d: i64,
    k: Vec<i64>,
}

struct Dp {
    dp: Vec<Vec<Vec<Mint>>>,
}

impl Dp {
    fn new(n_digit: usize, d: i64) -> Self {
        Self {
            dp: vec![vec![vec![Mint::new(0); d as usize]; 2]; n_digit + 1],
        }
    }

    fn at(&self, digit_i: usize, smaller: bool, rem: i64) -> &Mint {
        &self.dp[digit_i][smaller as usize][rem as usize]
    }

    fn at_mut(&mut self, digit_i: usize, smaller: bool, rem: i64) -> &mut Mint {
        &mut self.dp[digit_i][smaller as usize][rem as usize]
    }
}

impl Problem {
    fn read() -> Problem {
        input! {
            d: i64,
            k: Bytes,
        }

        let k = k.iter().copied().map(|x| (x - b'0') as i64).collect_vec();
        Problem { k, d }
    }
    fn solve(&self) -> Answer {
        let mut dp = Dp::new(self.k.len(), self.d);

        *dp.at_mut(0, false, 0) = 1.into();
        for digit_i in 0..self.k.len() {
            for rem in 0..self.d {
                let dp_false = *dp.at(digit_i, false, rem);
                let dp_true = *dp.at(digit_i, true, rem);

                for x in 0..10 {
                    *dp.at_mut(digit_i + 1, true, (rem + x) % self.d) += dp_true;
                }

                for x in 0..self.k[digit_i] {
                    *dp.at_mut(digit_i + 1, true, (rem + x) % self.d) += dp_false;
                }

                *dp.at_mut(digit_i + 1, false, (rem + self.k[digit_i]) % self.d) += dp_false;
            }
        }
        let ans = dp.at(self.k.len(), true, 0) + dp.at(self.k.len(), false, 0) - Mint::new(1);
        let ans = ans.val() as i64;
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
        assert_eq!(
            Problem {
                d: 3,
                k: vec![1, 0, 0],
            }
            .solve()
            .ans,
            33
        );
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
