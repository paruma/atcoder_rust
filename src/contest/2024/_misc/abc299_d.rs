// インタラクティブ問題
// ABC299 D - Find by Query
//https://atcoder.jp/contests/abc299/tasks/abc299_d
//#[derive_readable]
struct InteractiveIO<'a> {
    source: LineSource<StdinLock<'a>>,
}

impl<'a> InteractiveIO<'a> {
    fn new() -> Self {
        InteractiveIO {
            source: LineSource::new(stdin().lock()),
        }
    }

    fn read_init(&mut self) -> usize {
        input! {
            from &mut self.source,
            s: usize
        }
        s
    }

    fn ask(&mut self, i: usize) -> i64 {
        println!("? {}", i + 1);
        stdout().flush().unwrap();
        input! {
            from &mut self.source,
            s: i64
        }
        s
    }
}

fn main() {
    let mut io = InteractiveIO::new();
    let n = io.read_init();

    let mut i0 = 0; // s[i0] = 0
    let mut i1 = n - 1; // s[i1] = 1

    while i1 - i0 > 1 {
        let mid = (i0 + i1) / 2;
        let s_mid = io.ask(mid);
        if s_mid == 0 {
            i0 = mid;
        } else {
            //s_mid=1
            i1 = mid
        }
    }

    let ans = i0 + 1;
    println!("! {}", ans);
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

use std::io::{stdin, stdout, StdinLock, Write};

// ====== import ======
#[allow(unused_imports)]
use itertools::Itertools;
use proconio::source::line::LineSource;
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
