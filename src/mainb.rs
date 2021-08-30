use proconio::input;
use proconio::marker::Bytes;
use std::cmp;

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

fn score(seq1_aligned: &Vec<u8>, seq2_aligned: &Vec<u8>) -> i32 {
    assert!(seq1_aligned.len() == seq2_aligned.len());
    (0..seq1_aligned.len())
        .map(|i| sim(seq1_aligned[i], seq2_aligned[i]))
        .sum::<i32>()
}

fn solve(seq1: &Vec<u8>, seq2: &Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let dp_width = seq1.len() + 1;
    let dp_height = seq2.len() + 1;
    let mut dp: Vec<Vec<i32>> = vec![vec![0; dp_width]; dp_height];

    // x=0の列とy=0の行を計算
    for y in 0..dp_height {
        dp[y][0] = y as i32 * (-5);
    }
    for x in 0..dp_width {
        dp[0][x] = x as i32 * (-5);
    }

    for y in 1..dp_height {
        for x in 1..dp_width {
            let score1 = dp[y - 1][x - 1] + if seq1[x - 1] == seq2[y - 1] { 1 } else { -3 };
            let score2 = dp[y - 1][x] - 5;
            let score3 = dp[y][x - 1] - 5;
            dp[y][x] = cmp::max(cmp::max(score1, score2), score3);
        }
    }

    // 逆走
    let mut seq1_aligned: Vec<u8> = Vec::new();
    let mut seq2_aligned: Vec<u8> = Vec::new();

    let mut x = dp_width - 1;
    let mut y = dp_height - 1;

    while !(x == 0 && y == 0) {
        let score1 = if x == 0 || y == 0 {
            None
        } else {
            Some(dp[y - 1][x - 1] + if seq1[x - 1] == seq2[y - 1] { 1 } else { -3 })
        };
        let score2 = if y == 0 { None } else { Some(dp[y - 1][x] - 5) };
        let score3 = if x == 0 { None } else { Some(dp[y][x - 1] - 5) };

        if Some(dp[y][x]) == score1 {
            seq1_aligned.push(seq1[x - 1]);
            seq2_aligned.push(seq2[y - 1]);
            x -= 1;
            y -= 1;
        } else if Some(dp[y][x]) == score2 {
            seq1_aligned.push(b'-');
            seq2_aligned.push(seq2[y - 1]);
            y -= 1;
        } else if Some(dp[y][x]) == score3 {
            seq1_aligned.push(seq1[x - 1]);
            seq2_aligned.push(b'-');
            x -= 1;
        }
    }
    seq1_aligned.reverse();
    seq2_aligned.reverse();
    (seq1_aligned, seq2_aligned)
}

fn output(seq1: &Vec<u8>, seq2: &Vec<u8>) {
    println!("{}", String::from_utf8(seq1.clone()).unwrap());
    println!("{}", String::from_utf8(seq2.clone()).unwrap());
}

fn main() {
    let (seq1, seq2) = read();
    let (seq1_aligned, seq2_aligned) = solve(&seq1, &seq2);
    output(&seq1_aligned, &seq2_aligned);
}
