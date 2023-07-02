use std::io;

fn read() -> (String, i64) {
    let mut msg = String::new();
    let mut signeture_str = String::new();
    io::stdin().read_line(&mut msg).unwrap();
    io::stdin().read_line(&mut signeture_str).unwrap();
    let msg = msg.trim().to_string();
    let signeture = signeture_str.trim().parse::<i64>().unwrap();
    (msg, signeture)
}

fn alphabet_index(ch: u8) -> u8 {
    ch - b'a' + 1
}

fn hash(text: &str) -> i64 {
    // text は英小文字のみ
    let x: i64 = text
        .as_bytes()
        .iter()
        .map(|ch| alphabet_index(*ch) as i64)
        .sum();
    x % 100 + 1
}

fn mod_pow(base: i64, exponent: i64, modulus: i64) -> i64 {
    if exponent == 0 {
        return 1;
    }
    let half = mod_pow(base, exponent / 2, modulus);
    if exponent % 2 == 0 {
        (half * half) % modulus
    } else {
        (((base * half) % modulus) * half) % modulus
    }
}

// ax + by = gcd(a,b) の解を1つ求める。
// 解を(x, y) としたとき、(x, y, gcd(a,b))  を返す。
fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    assert!(a >= 0);
    assert!(b >= 0);
    if b == 0 {
        // ax = a
        return (1, 0, a);
    }
    // a = qb + r とする（q, rは商とあまり）
    let q = a / b;
    let r = a % b;
    // (qb + r)x + by = gcd(a,b) と変形でき、
    // b(qx + y) + rx = gcd(a,b) と変形できる。
    // ↑から、qx + y  と x を求める。
    let (qx_plus_y, x, d) = ext_gcd(b, r);
    let y = qx_plus_y - q * x;
    assert_eq!(a * x + b * y, d);
    (x, y, d)
}

// 検証鍵
struct PublicKey {
    n: i64,
    k: i64,
}

// 署名鍵
struct PrivateKey {
    n: i64,
    k: i64,
}

impl PrivateKey {
    pub fn sign(self, msg: &str) -> i64 {
        let hashed_msg = hash(msg);
        mod_pow(hashed_msg, self.k, self.n)
    }
}

impl PublicKey {
    pub fn is_valid(self, msg: &str, signature: i64) -> bool {
        let hashed_msg = hash(msg);
        let expected_hashed_msg = mod_pow(signature, self.k, self.n);
        hashed_msg == expected_hashed_msg
    }
}

fn main() {
    let (msg, signature) = read();
    let public_key = PublicKey { k: 23, n: 221 };
    let is_valid = public_key.is_valid(&msg, signature);
    let ans = if is_valid { "Yes" } else { "No" };
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("algorithm"), 4);

        let public_key = PublicKey { k: 23, n: 221 };
        assert!(public_key.is_valid("algorithm", 166))
    }
}
