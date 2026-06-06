#![allow(dead_code)]

use ac_library::{crt, inv_mod};
use num_integer::Integer;

/// ax ≡ b (mod m) を解く。
/// - 解が存在する場合、解を x ≡ r (mod m') としたとき (r, m') を返す。
/// - 解なしの場合は None。
fn solve_linear_congruence(a: i64, b: i64, m: i64) -> Option<(i64, i64)> {
    let g = a.gcd(&m);
    if b % g != 0 {
        return None;
    }
    let (a2, b2, m2) = (a / g, b / g, m / g);
    let a_inv = inv_mod(a2, m2);
    Some((a_inv * b2 % m2, m2))
}

/// x ≡ a (mod m) かつ x ≡ b (mod n) を解く。
/// - 解が存在する場合、解を x ≡ r (mod l) としたとき (r, l) を返す。
/// - 解なしの場合は None を返す
fn solve_simultaneous_congruences(a: i64, m: i64, b: i64, n: i64) -> Option<(i64, i64)> {
    let (r, modulus) = crt(&[a, b], &[m, n]);
    if modulus == 0 {
        None
    } else {
        Some((r, modulus))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_linear_congruence() {
        // 3x ≡ 4 (mod 5) -> x ≡ 3 (mod 5)
        assert_eq!(solve_linear_congruence(3, 4, 5), Some((3, 5)));
        // 6x ≡ 9 (mod 15) -> 2x ≡ 3 (mod 5) -> x ≡ 4 (mod 5)
        assert_eq!(solve_linear_congruence(6, 9, 15), Some((4, 5)));
        // 2x ≡ 1 (mod 4) -> no solution
        assert_eq!(solve_linear_congruence(2, 1, 4), None);
    }

    #[test]
    fn test_solve_simultaneous_congruences() {
        // x ≡ 2 (mod 3), x ≡ 3 (mod 5) -> x ≡ 8 (mod 15)
        assert_eq!(solve_simultaneous_congruences(2, 3, 3, 5), Some((8, 15)));
        // x ≡ 1 (mod 4), x ≡ 2 (mod 6) -> no solution (x must be odd and even)
        assert_eq!(solve_simultaneous_congruences(1, 4, 2, 6), None);
        // x ≡ 1 (mod 4), x ≡ 3 (mod 6) -> x ≡ 9 (mod 12)
        assert_eq!(solve_simultaneous_congruences(1, 4, 3, 6), Some((9, 12)));
    }
}
