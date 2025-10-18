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
* 1 <= A_i <= 2*10^5
* 1 <= Σ A_i <= 2*10^5
* N, A_i は整数

[入出力]
最初に以下の入力を受け取ってください。

```
N
A_1, ..., A_N
```

まず、あなたは先手か後手を選択します。先手の場合は `First`、後手の場合は `Second` を出力してください。

その後、ゲームが直ちに開始されるので、あなたはゲームが終了するまで入出力を利用してジャッジシステムと対話をして、ゲームに勝利してください。

あなたは手番が回ってきたら、操作を1つ決めます。i番目の山からx個石を取り除く場合は次の形式で出力してください。
ただし、操作が不可能な場合は (i, x) = (0, 0) として出力してください。

```
i x
```

ジャッジシステムの手番では、ジャッジシステムが以下の形式で整数の組 (i, x) を出力します。
```
i x
```

ここで
(i, x) は次の 3 種類のいずれかであることが保証されます。

* (i, x) = (0, 0) の場合：ジャッジシステムは操作を行えなくなったことを意味します。つまり、あなたはゲームに勝利しました。
* (i, x) = (−1, −1) の場合：あなたは 1 つ前に非合法な操作をしたか、あるいは (0, 0) を出力したことを意味します。つまり、あなたはゲームに敗北しました。
* それ以外の場合：ジャッジシステムは i番目からx個のコインを取る操作を行ったことを意味します。ここでジャッジシステムが選んだ操作は合法であることが保証されます。

ジャッジが (i, x) = (0, 0) または (i, x) = (−1, −1) を返した場合、ゲームはすでに終了しています。
この場合、プログラムをただちに終了してください。

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
    set: BTreeSet<usize>,
}

impl TestInteractive {
    fn new(xs: Vec<u64>) -> TestInteractive {
        let set = (0..xs.len()).collect();
        TestInteractive { xs, set }
    }

    fn judge_operate(&mut self) -> Response {
        // とりあえずランダムに操作をする

        if self.set.is_empty() {
            return Response::YouWin;
        }

        use rand::{rngs::SmallRng, seq::SliceRandom, *};
        let mut rng = SmallRng::from_os_rng();

        let mountain = self.set.iter().next().copied().unwrap();
        assert!(self.xs[mountain] != 0);

        let n_subtraction = rng.random_range(1..=self.xs[mountain]);

        self.xs[mountain] -= n_subtraction;
        if self.xs[mountain] == 0 {
            self.set.remove(&mountain);
        }

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
        if mountain >= self.xs.len() || n_subtraction > self.xs[mountain] || n_subtraction == 0 {
            return Response::YouLose;
        }
        self.xs[mountain] -= n_subtraction;
        if self.xs[mountain] == 0 {
            self.set.remove(&mountain);
        }
        self.judge_operate()
    }
}

/// Nim 和を計算する
fn calc_nim_sum(xs: &[u64]) -> u64 {
    xs.iter().copied().fold(0, |acc, x| acc ^ x)
}

/// 一番左の 1 が何ビット目かを返す (MSB: most significant bit)
/// x が 0 だとエラーになる。
fn calc_msb(x: u64) -> u32 {
    63 - x.leading_zeros()
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct NimSumManager {
    xs: Vec<u64>,
    indices: [BTreeSet<usize>; 64], // 各 bit に対して 1 が立っている添字の集合を格納する
    nim_sum: u64,
}

impl NimSumManager {
    fn new(xs: &[u64]) -> Self {
        let mut indices: [BTreeSet<usize>; 64] = std::array::from_fn(|_| BTreeSet::new());
        for (i, x) in xs.iter().copied().enumerate() {
            for b in 0..64 {
                if (x >> b) & 1 == 1 {
                    indices[b].insert(i);
                }
            }
        }
        let xs = xs.to_vec();
        let nim_sum = calc_nim_sum(&xs);

        Self {
            xs,
            indices,
            nim_sum,
        }
    }

    fn at(&self, i: usize) -> u64 {
        self.xs[i]
    }

    /// Nim 和を 0 にする操作を探す
    /// find_to_zero() == (i, x) のとき、i番目の値を x にすると、Nim 和が 0 になる。
    /// x はもともとの i 番目の値よりも小さい。
    fn find_to_zero(&self) -> Option<(usize, u64)> {
        if self.nim_sum == 0 {
            None
        } else {
            let msb = calc_msb(self.nim_sum) as usize;
            let i = self.indices[msb].iter().next().copied().unwrap();

            let after = self.xs[i] ^ self.nim_sum;
            Some((i, after))
        }
    }

    /// i 番目の値を after にする
    fn operate(&mut self, i: usize, after: u64) {
        let before = self.xs[i];

        for b in 0..64 {
            let before_bit = (before >> b) & 1;
            let after_bit = (after >> b) & 1;
            if (before_bit, after_bit) == (0, 1) {
                self.indices[b].insert(i);
            }
            if (before_bit, after_bit) == (1, 0) {
                self.indices[b].remove(&i);
            }
        }
        self.nim_sum = self.nim_sum ^ before ^ after;
        self.xs[i] = after;
    }
}

fn solve<T: IInteractive>(asker: &mut T, _n: usize, xs: &[u64]) {
    let nim_sum = calc_nim_sum(xs);
    let mut nim_sum_manager = NimSumManager::new(xs);
    let mut xs = xs.to_vec();
    if nim_sum == 0 {
        let response = asker.select_second();
        match response {
            Response::YouWin => return,
            Response::YouLose => panic!(),
            Response::Operate {
                mountain,
                n_subtraction,
            } => {
                let after = xs[mountain] - n_subtraction;
                let after_grundy = after; // grundy数 = 石の数
                xs[mountain] = after;
                nim_sum_manager.operate(mountain, after_grundy);
            }
        }
    } else {
        asker.select_first();
    }

    loop {
        // 最適な操作（操作する山と操作後のコインの数）を求める
        let (mountain, after_grundy) = nim_sum_manager.find_to_zero().unwrap();
        // 石の数 = grundy 数 (個数制限 Nim などの場合は、今の局面と目標の grundy 数から次の局面が求められる構造体を作ると良さそう)
        let after = after_grundy;
        let n_subtraction = xs[mountain] - after;

        xs[mountain] = after;
        nim_sum_manager.operate(mountain, after_grundy);
        // dbg!(xs.iter().copied().join(" "));

        let response = asker.operate(mountain, n_subtraction);
        match response {
            Response::YouWin => return,
            Response::YouLose => panic!("負け"),
            Response::Operate {
                mountain,
                n_subtraction,
            } => {
                let after = xs[mountain] - n_subtraction;
                let after_grundy = after; // grundy数 = 石の数
                xs[mountain] = after;
                nim_sum_manager.operate(mountain, after_grundy);
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

    /// 小さなテスト
    #[test]
    fn test_problem() {
        for _ in 0..10 {
            let xs = vec![1, 2, 3];
            let n = xs.len();
            let mut asker = TestInteractive::new(xs.clone());
            solve(&mut asker, n, &xs);
        }
    }

    /// ランテス
    #[test]
    fn test_problem2() {
        use rand::{rngs::SmallRng, *};
        let mut rng = SmallRng::from_os_rng();

        for _ in 0..10 {
            let n = 2_000;
            let xs = (0..n).map(|_| rng.random_range(1..=10)).collect_vec();
            let n = xs.len();
            let mut asker = TestInteractive::new(xs.clone());
            solve(&mut asker, n, &xs);
        }
    }
}

use std::{
    collections::BTreeSet,
    io::{Write, stdout},
};

// ====== import ======
use proconio::input_interactive;

#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, izip};
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
