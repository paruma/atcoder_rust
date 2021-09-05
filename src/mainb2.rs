use ndarray::{Array, Array2};
use proconio::input;
use proconio::marker::Bytes;

mod test;

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

fn solve(seq1: &[u8], seq2: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let dp_width = seq1.len() + 1;
    let dp_height = seq2.len() + 1;

    let mut dp: Array2<i32> = Array::zeros((dp_width, dp_height));

    // x=0の列とy=0の行を計算
    dp[[0, 0]] = 0;


    // ここ、itertoolsのiproduct!を使う方法もある。
    for y in 0..dp_height {
        for x in 0..dp_width {
            if x == 0 && y == 0 {
                continue;
            }
            let score1 =
                (!(x == 0 || y == 0)).then(|| dp[[y - 1, x - 1]] + sim(seq1[x - 1], seq2[y - 1]));
            let score2 = (y != 0).then(|| dp[[y - 1, x]] + sim(b'-', seq2[y - 1]));
            let score3 = (x != 0).then(|| dp[[y, x - 1]] + sim(seq1[x - 1], b'-'));
            //dp[[y, x]] = max(score1, score2, score3);
            dp[[y, x]] = *[score1, score2, score3].iter().flatten().max().unwrap();
        }
    }

    // 逆走
    let mut seq1_aligned: Vec<u8> = Vec::new();
    let mut seq2_aligned: Vec<u8> = Vec::new();

    let mut x = dp_width - 1;
    let mut y = dp_height - 1;

    while !(x == 0 && y == 0) {
        let score1 =
            (!(x == 0 || y == 0)).then(|| dp[[y - 1, x - 1]] + sim(seq1[x - 1], seq2[y - 1]));
        let score2 = (y != 0).then(|| dp[[y - 1, x]] + sim(b'-', seq2[y - 1]));
        let score3 = (x != 0).then(|| dp[[y, x - 1]] + sim(seq1[x - 1], b'-'));

        if Some(dp[[y, x]]) == score1 {
            seq1_aligned.push(seq1[x - 1]);
            seq2_aligned.push(seq2[y - 1]);
            x -= 1;
            y -= 1;
        } else if Some(dp[[y, x]]) == score2 {
            seq1_aligned.push(b'-');
            seq2_aligned.push(seq2[y - 1]);
            y -= 1;
        } else if Some(dp[[y, x]]) == score3 {
            seq1_aligned.push(seq1[x - 1]);
            seq2_aligned.push(b'-');
            x -= 1;
        }
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

    test::test();
}
