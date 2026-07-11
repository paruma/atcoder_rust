// インタラクティブ問題
trait IInteractive {
    fn ask(&mut self, i: usize, j: usize) -> bool;
}

struct StdinInteractive;
impl IInteractive for StdinInteractive {
    fn ask(&mut self, i: usize, j: usize) -> bool {
        println_flush!("? {} {}", i + 1, j + 1);
        input_interactive! {
            ans: String
        }
        ans == "Yes"
    }
}

struct TestInteractive {
    xs: Vec<i64>,
    cnt_ask: usize,
}
impl IInteractive for TestInteractive {
    fn ask(&mut self, i: usize, j: usize) -> bool {
        self.xs[j] - self.xs[i] <= 10
    }
}

impl TestInteractive {
    // 10倍しておく
    fn new(xs: Vec<i64>) -> Self {
        TestInteractive { xs, cnt_ask: 0 }
    }
}

fn solve<T: IInteractive>(asker: &mut T, n: usize) -> usize {
    let mut begin = 0;
    let mut ans = 0;
    for end in 1..n {
        while begin < end && !asker.ask(begin, end) {
            begin += 1;
        }
        // dbg!(begin, end);

        ans += end - begin; // + 1 ？
    }
    ans
}

fn main() {
    input_interactive! {
        n: usize,
    }
    let ans = solve(&mut StdinInteractive, n);
    println_flush!("! {}", ans);
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_problem() {
        let xs = vec![0, 7, 15, 100];
        let n = 3;
        let mut asker = TestInteractive::new(xs);
        let ans = solve(&mut asker, n);
        dbg!(asker.cnt_ask);
        dbg!(ans);
    }
}

use std::io::{Write, stdout};

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
