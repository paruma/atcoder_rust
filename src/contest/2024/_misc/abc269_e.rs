// インタラクティブ問題
trait IInteractive {
    fn ask(&mut self, a: i64, b: i64, c: i64, d: i64) -> Option<i64>;
}

struct StdinInteractive;
impl IInteractive for StdinInteractive {
    fn ask(&mut self, a: i64, b: i64, c: i64, d: i64) -> Option<i64> {
        println_flush!("? {} {} {} {}", a, b, c, d);
        input_interactive! {
            t: i64
        }
        if t == -1 {
            None
        } else {
            Some(t)
        }
    }
}

struct TestInteractive {
    rook_pos_list: Vec<(i64, i64)>,
    cnt_ask: i64,
}
impl IInteractive for TestInteractive {
    fn ask(&mut self, a: i64, b: i64, c: i64, d: i64) -> Option<i64> {
        if self.cnt_ask > 20 {
            return None;
        }
        // [a, b] × [c, d] にあるルークの数を数える
        let ans = self
            .rook_pos_list
            .iter()
            .filter(|(i, j)| (a..=b).contains(i) && (c..=d).contains(j))
            .count();
        self.cnt_ask += 1;

        Some(ans as i64)
    }
}

impl TestInteractive {
    fn new(rook_pos_list: Vec<(i64, i64)>) -> Self {
        TestInteractive {
            rook_pos_list,
            cnt_ask: 0,
        }
    }
}

fn solve<T: IInteractive>(asker: &mut T, n: i64) -> (i64, i64) {
    // 行を特定
    let row = {
        let mut i0 = 1;
        let mut i1 = n;
        while i1 - i0 >= 1 {
            let mid = (i0 + i1) / 2;
            let cnt = asker.ask(i0, mid, 1, n).unwrap();
            if cnt == mid - i0 + 1 {
                // [mid+1, i1] に空きがある
                i0 = mid + 1;
            } else {
                // [i0, mid] に空きがある
                i1 = mid;
            }
        }

        i0
    };

    // 列を特定
    let col = {
        let mut i0 = 1;
        let mut i1 = n;
        while i1 - i0 >= 1 {
            let mid = (i0 + i1) / 2;
            let cnt = asker.ask(1, n, i0, mid).unwrap();
            if cnt == mid - i0 + 1 {
                // [mid+1, i1] に空きがある
                i0 = mid + 1;
            } else {
                // [i0, mid] に空きがある
                i1 = mid;
            }
        }
        i0
    };

    (row, col)
}

fn main() {
    input_interactive! {
        n: i64,
    }
    let (row, col) = solve(&mut StdinInteractive, n);
    println_flush!("! {} {}", row, col);
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_problem() {
        let rook_pos_list = vec![(1, 2), (2, 1)];
        let n = 3;
        let mut asker = TestInteractive::new(rook_pos_list);
        let ans = solve(&mut asker, n);
        dbg!(asker.cnt_ask);
        dbg!(ans);
    }

    #[test]
    fn test_problem_heavy() {
        let n = 1000;
        let rook_pos_list = (0..n - 1)
            .map(|i| {
                let row = (i + 10) % n + 1;
                let col = (i + 20) % n + 1;
                (row, col)
            })
            .collect_vec();
        let mut asker = TestInteractive::new(rook_pos_list);
        let ans = solve(&mut asker, n);
        dbg!(asker.cnt_ask);
        dbg!(ans);
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
