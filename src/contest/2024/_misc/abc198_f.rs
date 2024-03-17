//#[derive_readable]
struct Problem {
    s: i64,
}

use ac_library::ModInt998244353 as Mint;

fn nc2(n: i64) -> Mint {
    let n = Mint::new(n);
    n * (n - 1) / 2
}

fn nc5(n: i64) -> Mint {
    let n = Mint::new(n);
    n * (n - 1) * (n - 2) * (n - 3) * (n - 4) / (5 * 4 * 3 * 2)
}

fn sub1(s: i64) -> Mint {
    // id
    nc5(s - 1)
}

fn sub2(s: i64) -> Mint {
    // 面に垂直の軸で90度回転
    match s % 4 {
        0 => {
            let n = Mint::new(s / 4 - 1);
            n * (n * 2 + 1)
        }
        1 => {
            let n = Mint::new((s - 1) / 4 - 1);
            n * 2 * (n + 1)
        }
        2 => {
            let n = Mint::new((s - 2) / 4);
            n * (n * 2 - 1)
        }
        3 => {
            let n = Mint::new((s - 3) / 4);
            n * n * 2
        }
        _ => unreachable!(),
    }
}

fn sub3(s: i64) -> Mint {
    // 面に垂直の軸で180度回転
    if s % 2 == 0 {
        let n = Mint::new(s / 2 - 2);
        n * (n + 1) * (n * 2 + 1) / 6
    } else {
        let n = Mint::new((s - 1) / 2 - 2);
        n * (n + 1) * (n + 2) / 3
    }
}

fn sub4(s: i64) -> Mint {
    // 120度回転
    if s % 3 != 0 {
        return Mint::new(0);
    }
    Mint::new(s) / 3 - 1
}

fn sub5(s: i64) -> Mint {
    // 斜めの軸で180度回転
    if s % 2 != 0 {
        return Mint::new(0);
    }
    nc2(s / 2 - 1)
}

impl Problem {
    fn read() -> Problem {
        input! {
            s: i64,
        }
        Problem { s }
    }
    fn solve(&self) -> Answer {
        let s = self.s;
        let ans = (sub1(s) + sub2(s) * 6 + sub3(s) * 3 + sub4(s) * 8 + sub5(s) * 6) / 24;
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
