use proconio::input;
use proconio::marker::Bytes;

fn read() -> (usize, Vec<Vec<u8>>) {
    input! {
        m: usize,
        seq_vec: [Bytes; m]
    }
    (m, seq_vec)
}


fn solve(m: usize, seq_vec: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    seq_vec.clone()
}

fn output(seq_vec: &Vec<Vec<u8>>) {
    // ここらへんのiter, into_iterの話がぜんぜんわからないなぁ。
    seq_vec
        .iter()
        .for_each(|seq| println!("{}", String::from_utf8(seq.clone()).unwrap()));
}

fn main() {
    let (m, seq_vec) = read();
    let seq_aligned_vec = solve(m, &seq_vec);
    output(&seq_aligned_vec);
}
