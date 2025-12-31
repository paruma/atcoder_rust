use std::io;

fn read() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

fn alphabet_index(ch: u8) -> u8 {
    ch - b'a' + 1
}

fn hash(text: &str) -> i64 {
    // text は英小文字のみ
    text.as_bytes()
        .iter()
        .map(|ch| alphabet_index(*ch))
        .fold(1, |acc, x| acc * (x as i64))
}

fn main() {
    let s = read();
    let hashed = hash(&s);
    println!("{}", hashed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("algo"), 1260);
        assert_eq!(hash("zzzzzzzzzzzz"), 95428956661682176);
    }
}
