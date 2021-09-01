use proconio::input;
use proconio::marker::Bytes;

fn read() -> (usize, Vec<Vec<u8>>) {
    input! {
        m: usize,
        seq_vec: [Bytes; m]
    }
    (m, seq_vec)
}

fn complementary_char(ch: u8) -> u8 {
    match ch {
        b'A' => b'T',
        b'T' => b'A',
        b'G' => b'C',
        b'C' => b'G',
        other => other,
    }
}

fn reverse_complement_sequence(seq: Vec<u8>) -> Vec<u8> {
    seq.into_iter().rev().map(complementary_char).collect()
}

fn main() {
    let (_, seq_vec) = read();

    

    let ans = seq_vec.into_iter().map(reverse_complement_sequence);

    ans.for_each(|x| println!("{}", String::from_utf8(x).unwrap()));
}
