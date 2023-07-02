fn rot_right(ch: u8, shift: u8) -> u8 {
    if !ch.is_ascii_lowercase() {
        return ch;
    }
    let ch_a = b'a';
    (ch - ch_a + shift) % 26 + ch_a
}

fn rot_left(ch: u8, shift: u8) -> u8 {
    if !ch.is_ascii_lowercase() {
        return ch;
    }
    let ch_a = b'a';
    (ch - ch_a + 26 * 2 - shift) % 26 + ch_a
}

fn alphabet_index(ch: u8) -> u8 {
    ch - b'a' + 1
}

fn encrypt_block1(block: &[u8], k1: u8, k2: u8) -> Vec<u8> {
    // block は長さ4であることを前提とする

    let x3 = rot_right(block[2], k1 + alphabet_index(block[0]));
    let x4 = rot_right(block[3], k2 + alphabet_index(block[1]));
    vec![x3, x4, block[0], block[1]]
}
fn encrypt_block5(block: &[u8], k1: u8, k2: u8) -> Vec<u8> {
    (0..5).fold(Vec::from(block), |acc, _| encrypt_block1(&acc, k1, k2))
}

fn encrypt(text: &str, k1: u8, k2: u8) -> String {
    let text = text.as_bytes().to_vec();
    let encrypted_text = text
        .chunks(4)
        .flat_map(|b| encrypt_block5(b, k1, k2))
        .collect::<Vec<u8>>();
    String::from_utf8(encrypted_text).unwrap()
}

fn decrypt_block1(block: &[u8], k1: u8, k2: u8) -> Vec<u8> {
    // block は長さ4であることを前提とする
    let x0 = rot_left(block[0], k1 + alphabet_index(block[2]));
    let x1 = rot_left(block[1], k2 + alphabet_index(block[3]));
    vec![block[2], block[3], x0, x1]
}
fn decrypt_block5(block: &[u8], k1: u8, k2: u8) -> Vec<u8> {
    (0..5).fold(Vec::from(block), |acc, _| decrypt_block1(&acc, k1, k2))
}

fn decrypt(text: &str, k1: u8, k2: u8) -> String {
    let text = text.as_bytes().to_vec();
    let encrypted_text = text
        .chunks(4)
        .flat_map(|b| decrypt_block5(b, k1, k2))
        .collect::<Vec<u8>>();
    String::from_utf8(encrypted_text).unwrap()
}

fn main() {
    let text = "wajwnsglkajoglrwaxwnjoin";
    let ans = decrypt(text, 12, 17);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        assert_eq!(
            encrypt_block5("abab".as_bytes(), 2, 3),
            "kjvk".as_bytes().to_vec()
        );

        assert_eq!(encrypt("ababcdcd", 2, 3), "kjvkkjla");
    }

    #[test]
    fn test_02() {
        assert_eq!(decrypt_block1("dgab".as_bytes(), 2, 3), "abab".as_bytes());
        assert_eq!(decrypt("kjvkkjla", 2, 3), "ababcdcd");
    }
}
