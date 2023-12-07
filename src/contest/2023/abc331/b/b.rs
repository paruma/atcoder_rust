#[derive_readable]
struct Problem {
    want_lb: i64,
    small_price: i64,
    medium_price: i64,
    large_price: i64,
}

impl Problem {
    fn read() -> Problem {
        input! {
            p: Problem
        }
        p
    }
    fn solve(&self) -> Answer {
        let small_size = 6; // eggs_in_small とかのほうが置かった？
        let medium_size = 8;
        let large_size = 12;

        let ans = iproduct!(
            0..=num_integer::Integer::div_ceil(&self.want_lb, &small_size),
            0..=num_integer::Integer::div_ceil(&self.want_lb, &medium_size),
            0..=num_integer::Integer::div_ceil(&self.want_lb, &large_size)
        )
        .filter(|(n_small, n_medium, n_large)| {
            n_small * small_size + n_medium * medium_size + n_large * large_size >= self.want_lb
        })
        .map(|(n_small, n_medium, n_large)| {
            n_small * self.small_price + n_medium * self.medium_price + n_large * self.large_price
        })
        .min()
        .unwrap();
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
use itertools::iproduct;
#[allow(unused_imports)]
use itertools::Itertools;
use num_integer::Integer;
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
