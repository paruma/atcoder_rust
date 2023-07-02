fn rot13(ch: u8) -> u8 {
    if !ch.is_ascii_lowercase() {
        return ch;
    }
    let ch_a = b'a';
    (ch - ch_a + 13) % 26 + ch_a
}

fn solve(text: &str) -> String {
    let bytes: Vec<u8> = text.as_bytes().to_vec();
    let bytes_processed: Vec<u8> = bytes.iter().map(|ch| rot13(*ch)).collect();
    String::from_utf8(bytes_processed).unwrap()
}

fn main() {
    let text = "cebtenzzvat vf abg zntvp. vg vf fbzrguvat lbh hfr naq znxr vagb lbhe bja gbby. jrypbzr gb nytb-zrgubq!";
    let ans = solve(text);
    println!("{}", ans);
}