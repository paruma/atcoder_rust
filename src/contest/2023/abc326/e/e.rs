//#[derive_readable]
struct Problem {
    n: usize,
    xs: Vec<i64>,
}

use ac_library::ModInt998244353 as Mint;

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            xs: [i64; n]
        }
        Problem { n, xs }
    }
    fn solve(&self) -> Answer {
        let n = self.n;
        let xs = &self.xs;
        // dp[x] := xにいるときから追加で支給される金額の期待値
        // 樹形図を考えると見える。
        let mut dp = vec![Mint::new(0); n + 1];
        let n_inv = Mint::new(n as i64).inv();

        let mut sum = Mint::new(0);

        for i in (0..=n).rev() {
            dp[i] = n_inv * sum;
            if i != 0 {
                sum += dp[i] + xs[i - 1];
            }
        }

        let ans = dp[0].val() as i64;
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // 寄与を考える。
        // サイコロで y を引いて xs[y] 円受け取る確率を考える
        // TODO: まだACしていない
        let n = self.n;
        let xs = &self.xs;
        let mut prob = vec![Mint::new(0); n + 1];
        prob[0] = 0.into();
        let n_inv = Mint::new(n as i64).inv();
        for i in 1..=n {
            prob[i] = (prob[i - 1] + 1) * n_inv;
            dbg!(prob[i].to_rational_str());
        }
        let ans = (0..n).map(|i| prob[i + 1] * xs[i]).sum::<Mint>();
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
    Problem::read().solve2().print();
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
