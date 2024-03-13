//#[derive_readable]
struct Problem {
    n: usize,
    xs: Vec<i64>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            xs: [i64; n],
        }
        Problem { n, xs }
    }
    fn solve(&self) -> Answer {
        /*
        方針
        mod 10^9 は mod 2^9 と mod 2^5 に分ける。
        n^n mod 2^9 は lcm(2^9, ϕ(2^9)) = 4×2^9 周期になることを使う
         */
        let pow_2_9: usize = usize::pow(2, 9);
        let pow_5_9: usize = usize::pow(5, 9);

        let mut x_to_n_mod2: Vec<Vec<usize>> = vec![vec![]; pow_2_9];
        let mut x_to_n_mod5: Vec<Vec<usize>> = vec![vec![]; pow_5_9];

        for n in (0..pow_2_9).filter(|i| i % 2 != 0) {
            let x = pow_mod(n as i64, n as i64, pow_2_9 as u32) as usize;
            x_to_n_mod2[x].push(n);
        }

        for n in (0..4 * pow_5_9).filter(|i| i % 5 != 0) {
            let x = pow_mod(n as i64, n as i64, pow_5_9 as u32) as usize;
            x_to_n_mod5[x].push(n);
        }

        let ans = self
            .xs
            .iter()
            .copied()
            .map(|x| {
                //
                let x = x as usize;
                iproduct!(&x_to_n_mod2[x % pow_2_9], &x_to_n_mod5[x % pow_5_9])
                    .filter_map(|(&n2, &n5)| {
                        let n5 = n5 % pow_5_9;
                        let cand0 = ac_library::crt(
                            &[n2 as i64, n5 as i64],
                            &[pow_2_9 as i64, pow_5_9 as i64],
                        )
                        .0;
                        if ac_library::pow_mod(cand0, cand0, (pow_2_9 * pow_5_9) as u32) == x as u32
                        {
                            Some(cand0)
                        } else {
                            None
                        }
                    })
                    .min()
                    .unwrap()
            })
            .collect_vec();

        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<i64>,
}

impl Answer {
    fn print(&self) {
        print_vec(&self.ans)
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

use ac_library::pow_mod;
use itertools::iproduct;
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
