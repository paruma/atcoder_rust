// トレースバックをしないバージョン
// トレースバックしないのはそれはそれで面倒。
// ただ意味としてはわかりやすい。

use itertools::iproduct;
use ndarray::{Array, Array2};
use proconio::input;
use proconio::marker::Bytes;

fn read() -> (Vec<u8>, Vec<u8>) {
    input! {
        seq1: Bytes,
        seq2: Bytes
    }
    (seq1, seq2)
}

fn sim(ch1: u8, ch2: u8) -> i32 {
    if ch1 == b'-' || ch2 == b'-' {
        -5
    } else if ch1 == ch2 {
        1
    } else {
        -3
    }
}

#[allow(dead_code)]
fn score(seq1_aligned: &[u8], seq2_aligned: &[u8]) -> i32 {
    assert!(seq1_aligned.len() == seq2_aligned.len());

    (0..seq1_aligned.len())
        .map(|i| sim(seq1_aligned[i], seq2_aligned[i]))
        .sum::<i32>()
}

#[allow(dead_code)]
fn max<T: Ord>(v1: T, v2: T, v3: T) -> T {
    v1.max(v2.max(v3))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    Change,
    Insert,
    Delete,
    Nothing, //dp[[0,0]]用
}

impl Default for Op {
    fn default() -> Self {
        Self::Nothing
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
struct DPElem {
    score: i32,
    op: Op,
}

fn solve(seq1: &[u8], seq2: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let dp_width = seq1.len() + 1;
    let dp_height = seq2.len() + 1;

    let mut dp: Array2<DPElem> = Array::default((dp_width, dp_height));
    //let mut dp: Array2<DPElem> = Array::from_shape_fn((dp_width, dp_height), |_| DPElem::default());
    //と書いても多分同じ。
    //`Array::from_shape_simple_fn` (第2引数で0引数関数を渡せる)は0.15.3にはあるけど、0.13.0にはない。

    // x=0の列とy=0の行を計算
    dp[[0, 0]] = DPElem {
        score: 0,
        op: Op::Nothing,
    };

    // ここ、itertoolsのiproduct!を使う方法もある。

    // マクロでmfor!(y in 0..dp_height, x in dp_width)とかかけたらいいんだけどなぁ。
    // 2重forより可読性上がってるのかよくわからないなぁ。
    for (y, x) in iproduct!(0..dp_height, 0..dp_width) {
        if x == 0 && y == 0 {
            continue;
        }
        let score1 = (!(x == 0 || y == 0)).then(|| DPElem {
            score: dp[[y - 1, x - 1]].score + sim(seq1[x - 1], seq2[y - 1]),
            op: Op::Change,
        });
        let score2 = (y != 0).then(|| DPElem {
            score: dp[[y - 1, x]].score + sim(b'-', seq2[y - 1]),
            op: Op::Insert,
        });
        let score3 = (x != 0).then(|| DPElem {
            score: dp[[y, x - 1]].score + sim(seq1[x - 1], b'-'),
            op: Op::Delete,
        });
        dp[[y, x]] = *[score1, score2, score3]
            .iter()
            .flatten()
            .max_by_key(|&x| x.score)
            .unwrap();
    }

    // 逆走
    let mut seq1_aligned: Vec<u8> = Vec::new();
    let mut seq2_aligned: Vec<u8> = Vec::new();

    let mut x = dp_width - 1;
    let mut y = dp_height - 1;

    while !(x == 0 && y == 0) {
        match dp[[y, x]].op {
            Op::Change => {
                // seq1[x-1]をseq2[y-1]にChange
                seq1_aligned.push(seq1[x - 1]);
                seq2_aligned.push(seq2[y - 1]);
                x -= 1;
                y -= 1;
            }
            Op::Insert => {
                // seq2[y-1]をInsert
                seq1_aligned.push(b'-');
                seq2_aligned.push(seq2[y - 1]);
                y -= 1;
            }
            Op::Delete => {
                // seq1[x-1]をDelete
                seq1_aligned.push(seq1[x - 1]);
                seq2_aligned.push(b'-');
                x -= 1;
            }
            Op::Nothing => {
                panic!();
            }
        };
    }

    seq1_aligned.reverse();
    seq2_aligned.reverse();
    (seq1_aligned, seq2_aligned)
}

fn output(seq1: &[u8], seq2: &[u8]) {
    println!("{}", String::from_utf8(seq1.to_vec()).unwrap());
    println!("{}", String::from_utf8(seq2.to_vec()).unwrap());
}

fn main() {
    let (seq1, seq2) = read();
    let (seq1_aligned, seq2_aligned) = solve(&seq1, &seq2);
    output(&seq1_aligned, &seq2_aligned);
}
