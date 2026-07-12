// インタラクティブ問題
trait IInteractive {
    fn ask(&mut self, i: usize) -> bool;
}

struct StdinInteractive;
impl IInteractive for StdinInteractive {
    fn ask(&mut self, i: usize) -> bool {
        // 1オリジンで出力する場合は+1するのを忘れずに
        println_flush!("? {}", i + 1);
        input_interactive! {
            ans: String
        }
        ans == "Yes"
    }
}

/// テスト用のスタブ実装
struct TestInteractive {
    xs: Vec<i64>,
    cnt_ask: usize,
}
impl IInteractive for TestInteractive {
    fn ask(&mut self, i: usize) -> bool {
        // 質問が正当かどうかを確認すること
        // assert!(i < self.xs.len());
        true
    }
}

impl TestInteractive {
    fn new(xs: Vec<i64>) -> Self {
        TestInteractive { xs, cnt_ask: 0 }
    }
}

fn solve<T: IInteractive>(asker: &mut T, n: usize) -> i64 {
    -2
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

    use rand::{Rng, SeedableRng, rngs::SmallRng};

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_problem() {
        let xs = vec![0, 7, 15, 100];
        let n = 4;
        let mut asker = TestInteractive::new(xs);
        let ans = solve(&mut asker, n);
        dbg!(asker.cnt_ask);
        dbg!(ans);
    }

    fn solve_naive(xs: &[i64]) -> i64 {
        -2
    }

    /// 間違っていたら false を返す
    fn process_one_test(rng: &mut SmallRng) -> bool {
        // ==== 問題を作る ====
        let n = rng.random_range(1..=10);
        let xs = (0..n).map(|_| rng.random_range(0..10)).collect_vec();

        // ==== 解く ====
        // let mut asker = TestInteractive::new(xs.clone());
        // let main_ans = solve(&mut asker, n);
        // let naive_ans = solve_naive(&xs);
        let main_ans = -2;
        let naive_ans = -2;

        // ==== 間違っていたら報告をする ====
        // 必要なら質問回数のチェックもする。例: asker.cnt_ask > 2 * n
        if main_ans != naive_ans {
            // 問題を出力
            println!("{:?}", (n, xs));
            println!("main ans : {:?}", main_ans);
            println!("naive ans: {:?}", naive_ans);
            return false;
        }
        true
    }
    #[allow(unreachable_code)]
    #[test]
    #[ignore]
    fn test_with_naive() {
        let num_tests = 100;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut cnt_wrong = 0;
        let mut rng = SmallRng::seed_from_u64(42);
        // let mut rng = SmallRng::from_os_rng();
        for _ in 0..num_tests {
            let is_ok = process_one_test(&mut rng);
            if !is_ok {
                cnt_wrong += 1;
            }
            if cnt_wrong >= max_wrong_case {
                break;
            }
        }
        if cnt_wrong > 0 {
            println!("{} cases are wrong.", cnt_wrong);
            panic!();
        }
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
use print_util::*;
pub mod print_util {
    use itertools::Itertools;
    use proconio::fastout;

    #[fastout]
    pub fn print_vec<T: std::fmt::Display>(arr: &[T]) {
        for a in arr {
            println!("{}", a);
        }
    }

    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
        println!("{}", arr.iter().join(" "));
    }

    #[fastout]
    pub fn print_vec2<T: std::fmt::Display, R: AsRef<[T]>>(arr: &[R]) {
        for row in arr {
            println!("{}", row.as_ref().iter().join(" "));
        }
    }

    pub fn print_bytes(bytes: &[u8]) {
        println!("{}", std::str::from_utf8(bytes).unwrap());
    }

    pub fn print_chars(chars: &[char]) {
        println!("{}", chars.iter().collect::<String>());
    }

    #[fastout]
    pub fn print_vec_bytes<R: AsRef<[u8]>>(vec_bytes: &[R]) {
        for row in vec_bytes {
            println!("{}", std::str::from_utf8(row.as_ref()).unwrap());
        }
    }

    #[fastout]
    pub fn print_vec_chars<R: AsRef<[char]>>(vec_chars: &[R]) {
        for row in vec_chars {
            println!("{}", row.as_ref().iter().collect::<String>());
        }
    }

    pub fn print_yesno(ans: bool) {
        println!("{}", if ans { "Yes" } else { "No" });
    }
}

// インタラクティブ問題では、標準出力の後に flush することが求められる。
// なぜか flush に失敗した場合は unwrap をしてプログラムを終了するようにしている
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

// ====== snippet ======
