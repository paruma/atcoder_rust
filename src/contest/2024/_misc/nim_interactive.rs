/*
Nim のインタラクティブ問題
整数 N と整数列 (A_1, ..., A_N) が与えられます。
あなたはジャッジシステムと次のゲームで対戦します。

> 石の山が N 個あり、i番目の山には A_i 個の山が積まれています
> 先行から交互の以下の操作を繰り返します
> * 空でない山を1つ選び、その山から1個以上の好きな個数の石を取り除く

あなたは先行か後攻の一方を選んでください。
そして、選んだほうの手番でジャッジシステムとゲームをして勝利してください。

[制約]
* 1 <= N <= 2*10^5
* 1 <= A_i <= 10^9

[入力例]
N
A_1, ..., A_N

参考: https://atcoder.jp/contests/abc278/tasks/abc278_g
*/

enum Response {
    YouWin,
    YouLose,
    Operate { mountain: usize, n_subtraction: u64 },
}

trait IInteractive {
    fn select_first(&mut self);
    fn select_second(&mut self) -> Response;
    fn operate(&mut self, mountain: usize, n_subtraction: u64) -> Response;
}

struct StdinInteractive;
impl StdinInteractive {
    fn get_response(&self) -> Response {
        input_interactive! {
            a: i64,
            b: i64,
        }
        match (a, b) {
            (0, 0) => Response::YouWin,
            (-1, -1) => Response::YouLose,
            (a, b) => Response::Operate {
                mountain: (a - 1) as usize,
                n_subtraction: b as u64,
            },
        }
    }
}
impl IInteractive for StdinInteractive {
    fn select_first(&mut self) {
        println_flush!("First");
    }

    fn select_second(&mut self) -> Response {
        println_flush!("Second");
        self.get_response()
    }

    fn operate(&mut self, mountain: usize, n_subtraction: u64) -> Response {
        println_flush!("{} {}", mountain + 1, n_subtraction);
        self.get_response()
    }
}

struct TestInteractive {
    xs: Vec<u64>,
}

impl TestInteractive {
    fn new(xs: Vec<u64>) -> TestInteractive {
        TestInteractive { xs }
    }

    fn judge_operate(&mut self) -> Response {
        // とりあえずランダムに操作をする

        if self.xs.iter().copied().all(|x| x == 0) {
            return Response::YouWin;
        }

        use rand::{rngs::SmallRng, seq::SliceRandom, *};
        let mut rng = SmallRng::from_entropy();

        let cand_mountains = self.xs.iter().copied().positions(|x| x > 0).collect_vec();
        let mountain = cand_mountains[rng.gen_range(0..cand_mountains.len())];

        let n_subtraction = rng.gen_range(1..=self.xs[mountain]);

        self.xs[mountain] -= n_subtraction;

        Response::Operate {
            mountain,
            n_subtraction,
        }
    }
}

impl IInteractive for TestInteractive {
    // ジャッジシステムは後手
    fn select_first(&mut self) {}

    // ジャッジシステムは先手
    fn select_second(&mut self) -> Response {
        self.judge_operate()
    }

    // あなたの操作を受け取り、ジャッジシステムが操作する
    fn operate(&mut self, mountain: usize, n_subtraction: u64) -> Response {
        // 不正な操作
        if mountain >= self.xs.len() || n_subtraction > self.xs[mountain] {
            return Response::YouLose;
        }
        self.xs[mountain] -= n_subtraction;
        self.judge_operate()
    }
}

fn calc_nim_sum(xs: &[u64]) -> u64 {
    xs.iter().copied().fold(0, |acc, x| acc ^ x)
}

fn calc_msb(x: u64) -> u32 {
    // most significant bit
    63 - x.leading_zeros()
}

fn solve<T: IInteractive>(asker: &mut T, _n: usize, xs: &[u64]) {
    let mut xs = xs.to_vec();
    let nim_sum = calc_nim_sum(&xs);
    if nim_sum == 0 {
        let response = asker.select_second();
        match response {
            Response::YouWin => return,
            Response::YouLose => panic!(),
            Response::Operate {
                mountain,
                n_subtraction,
            } => {
                xs[mountain] -= n_subtraction;
            }
        }
    } else {
        asker.select_first();
    }

    // 最適な操作（山の操作と取るコインの数）を求める

    loop {
        let nim_sum = calc_nim_sum(&xs);
        assert!(nim_sum != 0);
        let msb = calc_msb(nim_sum);

        // todo: 高速化
        // msb が 1 のものを xs から探す
        let mountain = xs
            .iter()
            .copied()
            .position(|x| (x >> msb) & 1 == 1)
            .unwrap();

        // xs[mountain] を xs[mountain] ^ nim_sum にしたい
        let n_subtraction = xs[mountain] - (xs[mountain] ^ nim_sum);
        xs[mountain] -= n_subtraction;

        // dbg!(xs.iter().copied().join(" "));

        let response = asker.operate(mountain, n_subtraction);
        match response {
            Response::YouWin => return,
            Response::YouLose => panic!("負け"),
            Response::Operate {
                mountain,
                n_subtraction,
            } => {
                xs[mountain] -= n_subtraction;
            }
        }
    }
}

fn main() {
    input_interactive! {
        n: usize,
        xs: [u64; n],
    }
    solve(&mut StdinInteractive, n, &xs);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_problem() {
        for _ in 0..10 {
            let xs = vec![1, 2, 3];
            let n = xs.len();
            let mut asker = TestInteractive::new(xs.clone());
            solve(&mut asker, n, &xs);
        }
    }

    #[test]
    fn test_problem2() {
        use rand::{rngs::SmallRng, seq::SliceRandom, *};
        let mut rng = SmallRng::from_entropy();

        for _ in 0..10 {
            let n = 6;
            let xs = (0..n).map(|_| rng.gen_range(1..=10)).collect_vec();
            let n = xs.len();
            let mut asker = TestInteractive::new(xs.clone());
            solve(&mut asker, n, &xs);
        }
    }
}

use std::io::{stdout, Stdin, Write};

// ====== import ======
use proconio::input_interactive;

#[allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout,
    marker::{Bytes, Chars, Usize1},
};
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet};

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
