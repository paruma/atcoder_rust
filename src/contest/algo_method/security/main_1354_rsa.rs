/// ax + by = gcd(a,b) の解を1つ求める。
/// 解を(x, y) としたとき、(x, y, gcd(a,b))  を返す。
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
    // (qb + r)x + by = gcd(a,b) と変形でき。
    // b(qx + y) + rx = gcd(a,b) と変形できる。
    // ↑から、qx + y  と x を求める。
    let (qx_plus_y, x, d) = ext_gcd(b, r);
    let y = qx_plus_y - q * x;
    assert_eq!(a * x + b * y, d);
    (x, y, d)
}

fn mod_pow(base: i64, exponent: i64, modulus: i64) -> i64 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exponent = exponent;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }

        base = (base * base) % modulus;
        exponent /= 2;
    }

    result
}

fn mod_pow2(base: i64, exponent: i64, modulus: i64) -> i64 {
    if exponent == 0 {
        return 1;
    }
    let half = mod_pow2(base, exponent / 2, modulus);
    if exponent % 2 == 0 {
        (half * half) % modulus
    } else {
        (((base * half) % modulus) * half) % modulus
    }
}

fn main() {
    let p = 127;
    let q = 211;
    let n = p * q;
    let k1 = 101; // (p-1)*(q-1) と互いに素
    let (k2, _, _) = ext_gcd(k1, (p - 1) * (q - 1));
    let k2 = k2.rem_euclid((p - 1) * (q - 1));

    let encrypted = 10280;
    let decrypted = mod_pow2(encrypted, k2, n);
    println!("{}", decrypted);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ext_gcd() {
        // 3x + 4y = 1 → x=3, y=-2など
        let (x, y, d) = ext_gcd(3, 4);
        print!("{}, {}", x, y);
        assert_eq!(d, 1);
        assert_eq!(3 * x + 4 * y, d);

        // 4x + 6y = 2
        let (x, y, d) = ext_gcd(4, 6);
        assert_eq!(d, 2);
        assert_eq!(4 * x + 6 * y, d);
    }

    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow(2, 10, 1000), 24);
        assert_eq!(mod_pow2(2, 10, 1000), 24);
    }
}
