// インタラクティブ問題
// ABC299 D - Find by Query
//https://atcoder.jp/contests/abc299/tasks/abc299_d

trait IInteractive {
    fn ask(&self, i: usize) -> i64;
}

struct StdinInteractive;
impl IInteractive for StdinInteractive {
    fn ask(&self, i: usize) -> i64 {
        println_flush!("? {}", i + 1);
        input_interactive! {
            s: i64
        }
        s
    }
}

struct TestInteractive {
    xs: Vec<i64>,
}
impl IInteractive for TestInteractive {
    fn ask(&self, i: usize) -> i64 {
        self.xs[i]
    }
}

fn solve<T: IInteractive>(asker: T, n: usize) -> usize {
    let mut i0 = 0; // s[i0] = 0
    let mut i1 = n - 1; // s[i1] = 1

    while i1 - i0 > 1 {
        let mid = (i0 + i1) / 2;
        let s_mid = asker.ask(mid);
        if s_mid == 0 {
            i0 = mid;
        } else {
            //s_mid=1
            i1 = mid
        }
    }

    i0 + 1
}

fn main() {
    input_interactive! {
        n: usize,
    }
    let ans = solve(StdinInteractive, n);
    println_flush!("! {}", ans);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_problem() {
        let xs = vec![0, 0, 1, 0, 0, 1, 1];
        let n = xs.len();
        let asker = TestInteractive { xs: xs.clone() };
        let ans = solve(asker, n);
        assert!(xs[ans - 1] != xs[ans]);
    }
}

use std::io::{stdout, Write};

// ====== import ======
#[allow(unused_imports)]
use itertools::Itertools;
use proconio::input_interactive;
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

#[macro_export]
macro_rules! println_flush {
    () => {
        println!();
        stdout().flush().unwrap();
    };
    ($($arg:tt)*) => {{
        println!($($arg)*);
        stdout().flush().unwrap();
    }};
}
