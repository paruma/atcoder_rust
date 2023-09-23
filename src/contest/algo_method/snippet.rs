#[allow(unused_imports)]
use myio::*;
pub mod myio {
    use std::io;

    pub fn read_line() -> String {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        buffer.trim().to_string()
    }

    pub fn read_vec_i64() -> Vec<i64> {
        let buf = read_line();
        buf.trim()
            .split(' ')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    }

    pub fn read_i64_1() -> i64 {
        let buf = read_line();
        buf.parse::<i64>().unwrap()
    }

    pub fn read_i64_2() -> (i64, i64) {
        let ns = read_vec_i64();
        (ns[0], ns[1])
    }

    pub fn read_i64_3() -> (i64, i64, i64) {
        let ns = read_vec_i64();
        (ns[0], ns[1], ns[2])
    }

    pub fn read_i64_4() -> (i64, i64, i64, i64) {
        let ns = read_vec_i64();
        (ns[0], ns[1], ns[2], ns[3])
    }
}

fn pow(base: i64, exponent: i64) -> i64 {
    if exponent == 0 {
        return 1;
    }
    let half = pow(base, exponent / 2);
    if exponent % 2 == 0 {
        half * half
    } else {
        base * half * half
    }
}

fn pow_f64(base: f64, exponent: i64) -> f64 {
    if exponent == 0 {
        return 1.0;
    }
    let half = pow_f64(base, exponent / 2);
    if exponent % 2 == 0 {
        half * half
    } else {
        base * half * half
    }
}

// TODO: ACLを使うようにする
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

fn scanl<I, B, F>(iter: I, init: B, f: F) -> impl Iterator<Item = B>
where
    B: Clone + Copy,
    I: Iterator,
    I::Item: Clone,
    F: Fn(&B, I::Item) -> B,
{
    let mut iter = iter;
    let mut acc = init;
    std::iter::once(acc).chain(std::iter::from_fn(move || {
        iter.next().map(|x| {
            acc = f(&acc, x);
            acc
        })
    }))
}

fn frac(n: i64) -> i64 {
    (1..=n).product::<i64>()
}

fn permu(n: i64, r: i64) -> i64 {
    (n - r + 1..=n).product::<i64>()
}

fn comb(n: i64, r: i64) -> i64 {
    permu(n, r) / permu(r, r)
}

// 引数に範囲外が来ることは考慮されていない


struct EntryCount<T> {
    elem: T,
    prob: f64,
}
