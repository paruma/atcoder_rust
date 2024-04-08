//#[derive_readable]

use ac_library::ModInt998244353 as Mint;

struct Problem {
    n: usize,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
        }
        Problem { n }
    }
    fn solve(&self) -> Answer {
        let n = self.n;
        let mut dp = vec![vec![Mint::new(0); 2]; n + 1];
        dp[0][0] = 1.into();
        dp[0][1] = 0.into();
        dp[1][0] = 0.into();
        dp[1][1] = 1.into();

        for i in 1..n {
            let prob = Mint::new(n + i).inv() * n;
            // let prob = Mint::new(n - i)
            //     / Mint::new(n)
            //     / (Mint::new(1) - (Mint::new(i) / n) * (Mint::new(i) / n));

            dp[i + 1][0] = (-prob + 1) * dp[i][0] + prob * dp[i][1];
            dp[i + 1][1] = -dp[i + 1][0] + 1;
        }

        let n_inv = Mint::new(n).inv();

        let ans_sente = (1..n)
            .map(|i| {
                dp[i][0] * i * n_inv
                    / (Mint::new(1) - (Mint::new(i) * n_inv) * (Mint::new(i) * n_inv))
                    + dp[i][1] * i * i * n_inv * n_inv
                        / (Mint::new(1) - (Mint::new(i) * n_inv) * (Mint::new(i) * n_inv))
            })
            .sum::<Mint>();

        let ans_gote = (1..n)
            .map(|i| {
                dp[i][1] * i * n_inv
                    / (Mint::new(1) - (Mint::new(i) * n_inv) * (Mint::new(i) * n_inv))
                    + dp[i][0] * i * i * n_inv * n_inv
                        / (Mint::new(1) - (Mint::new(i) * n_inv) * (Mint::new(i) * n_inv))
            })
            .sum::<Mint>();

        let ans_sente = ans_sente.val();
        let ans_gote = ans_gote.val();
        Answer {
            ans_sente,
            ans_gote,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans_sente: u32,
    ans_gote: u32,
}

impl Answer {
    fn print(&self) {
        println!("{} {}", self.ans_sente, self.ans_gote);
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
        let x = Mint::new(174692763);
        let y = Mint::new(324429416);
        dbg!(x.to_rational_str());
        dbg!(y.to_rational_str());
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

use modint_to_rational::*;
pub mod modint_to_rational {
    use num_rational::Rational64;
    pub trait ToRational {
        fn to_rational(&self) -> Option<Rational64>;
        fn to_rational_str(&self) -> String {
            self.to_rational()
                .map(|x| x.to_string())
                .unwrap_or("cannot reconstruct".to_string())
        }
    }
    impl ToRational for ac_library::ModInt998244353 {
        /// 注意: 1000 * 2000 = 2*10^6 の計算をしている
        fn to_rational(&self) -> Option<Rational64> {
            if self.val() == 0 {
                return Some(Rational64::new(0, 1));
            }
            for denom in 1..1000 {
                let denom_inv = Self::new(denom).inv();
                for numer in -1000..1000 {
                    if *self == denom_inv * Self::new(numer) {
                        return Some(Rational64::new(numer, denom));
                    }
                }
            }
            None
        }
    }
}
