// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        xs: [i64; n],
    }
    use ac_library::ModInt998244353 as Mint;

    let mut acc = vec![Mint::new(1), Mint::new(-1)];

    for &x in &xs {
        let mut ys = vec![Mint::new(0); (x + 1) as usize];
        ys[0] = Mint::new(1);
        ys[x as usize] = Mint::new(-1);
        acc = ac_library::convolution::convolution(&acc, &ys);
    }

    let mut init = vec![Mint::new(0); acc.len()];
    init[0] = Mint::new(1);

    let ans = bostan_mori(&init, &acc, (m) as u64);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }

    /// 間違っていたら false を返す
    fn process_one_test(rng: &mut SmallRng) -> bool {
        // ==== 問題を作る ====
        let n = rng.random_range(1..=10);
        let xs = (0..n).map(|_| rng.random_range(0..10)).collect_vec();

        // ==== 解く ====
        let main_ans = xs.len();
        let naive_ans = 1;

        // ==== 間違っていたら報告をする ====
        if main_ans != naive_ans {
            // 問題を出力
            println!("{:?}", (n, xs));
            println!("main ans : {:?}", main_ans);
            println!("naive ans: {:?}", naive_ans);
            return false;
        }
        true
    }

    #[allow(unreachable_code)]
    #[test]
    #[ignore]
    fn test_with_naive() {
        let num_tests = 100;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut cnt_wrong = 0;
        let mut rng = SmallRng::seed_from_u64(42);
        // let mut rng = SmallRng::from_os_rng();
        for _ in 0..num_tests {
            let is_ok = process_one_test(&mut rng);
            if !is_ok {
                cnt_wrong += 1;
            }
            if cnt_wrong >= max_wrong_case {
                break;
            }
        }
        if cnt_wrong > 0 {
            println!("{} cases are wrong.", cnt_wrong);
            panic!();
        }
    }
}

// ====== import ======
#[allow(unused_imports)]
use {
    itertools::{Itertools, chain, iproduct, izip},
    proconio::{
        derive_readable, fastout, input,
        marker::{Bytes, Chars, Usize1},
    },
    rand::{Rng, SeedableRng, rngs::SmallRng, seq::SliceRandom},
    std::{
        cmp::Reverse,
        collections::{BinaryHeap, HashMap, HashSet},
    },
};

// ====== output func ======
#[allow(unused_imports)]
use print_util::*;
pub mod print_util {
    use itertools::Itertools;
    use proconio::fastout;
    #[fastout]
    pub fn print_vec<T: std::fmt::Display>(arr: &[T]) {
        for a in arr {
            println!("{}", a);
        }
    }
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Display>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{}", x)).join(" ");
            println!("{}", msg);
        }
    }
    pub fn print_bytes(bytes: &[u8]) {
        let msg = String::from_utf8(bytes.to_vec()).unwrap();
        println!("{}", msg);
    }
    pub fn print_chars(chars: &[char]) {
        let msg = chars.iter().collect::<String>();
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec_bytes(vec_bytes: &[Vec<u8>]) {
        for row in vec_bytes {
            let msg = String::from_utf8(row.to_vec()).unwrap();
            println!("{}", msg);
        }
    }
    #[fastout]
    pub fn print_vec_chars(vec_chars: &[Vec<char>]) {
        for row in vec_chars {
            let msg = row.iter().collect::<String>();
            println!("{}", msg);
        }
    }
    pub fn print_yesno(ans: bool) {
        let msg = if ans { "Yes" } else { "No" };
        println!("{}", msg);
    }
}

// ====== snippet ======

use ac_library::{Modulus, StaticModInt, convolution};

// Bostan-Mori algorithm for linear recurrence relations
//
// Calculates the N-th term of a linear recurrence relation.
// The recurrence relation is defined by a characteristic polynomial Q(x) and initial terms.
//
// Let the recurrence be a_n = c_1 a_{n-1} + c_2 a_{n-2} + ... + c_k a_{n-k}
// The characteristic polynomial is Q(x) = 1 - c_1 x - c_2 x^2 - ... - c_k x^k.
//
// We want to find the N-th term of the sequence {a_n}.
// This can be expressed as the N-th coefficient of the rational function P(x)/Q(x),
// where P(x) is derived from the initial terms.
//
// The algorithm proceeds by repeatedly applying the identity:
// [x^N] P(x)/Q(x) = [x^(N/2)] P_0(x^2) / Q_0(x^2)  if N is even
//                   [x^(N/2)] P_1(x^2) / Q_1(x^2)  if N is odd
// where P_0, P_1, Q_0, Q_1 are derived using polynomial arithmetic and convolution.
//
// Specifically, let R(x) = Q(-x).
// Q_0(x^2) = Q(x) R(x)
// P_0(x^2) = P(x) R(x) (for even N)
// P_1(x^2) = (P(x) R(x)) / x (for odd N)
//
// All polynomials are represented as Vec<StaticModInt<M>> where the i-th element is the coefficient of x^i.

/// Bostan-Mori algorithm for calculating the N-th term of a linear recurrence relation.
///
/// `initial_terms`: The initial terms of the sequence, a_0, a_1, ..., a_{k-1}.
/// `coeffs`: The coefficients of the recurrence relation, c_1, c_2, ..., c_k.
///           a_n = c_1 a_{n-1} + c_2 a_{n-2} + ... + c_k a_{n-k}.
/// `n`: The index of the term to calculate (0-indexed).
///
/// Returns the N-th term of the sequence modulo M.
///
/// # Complexity
///
/// O(K log K log N), where K is the degree of the recurrence relation (coeffs.len()).
pub fn bostan_mori<M: Modulus>(
    p: &[StaticModInt<M>],
    q: &[StaticModInt<M>],
    mut n: u64,
) -> StaticModInt<M> {
    let k = q.len() - 1;
    let mut p = p.to_vec();
    let mut q = q.to_vec();

    // Main loop of Bostan-Mori algorithm
    while n > 0 {
        // Compute Q(-x)
        let mut q_neg_x: Vec<StaticModInt<M>> = q.clone();
        for i in (1..=k).step_by(2) {
            q_neg_x[i] = -q_neg_x[i];
        }

        // Compute Q_new(x^2) = Q(x) * Q(-x)
        let q_new_poly = convolution(&q, &q_neg_x);
        let mut q_new: Vec<StaticModInt<M>> = Vec::new();
        for i in (0..q_new_poly.len()).step_by(2) {
            q_new.push(q_new_poly[i]);
        }

        // Compute P_temp(x) = P(x) * Q(-x)
        let p_temp_poly = convolution(&p, &q_neg_x);
        let mut p_new: Vec<StaticModInt<M>> = Vec::new();

        if n % 2 == 0 {
            // N is even: P_new(x^2) = P_temp(x)
            for i in (0..p_temp_poly.len()).step_by(2) {
                p_new.push(p_temp_poly[i]);
            }
        } else {
            // N is odd: P_new(x^2) = P_temp(x) / x
            for i in (1..p_temp_poly.len()).step_by(2) {
                p_new.push(p_temp_poly[i]);
            }
        }

        p = p_new;
        q = q_new;
        n /= 2;

        // Truncate polynomials to degree k-1 for P and k for Q.
        // The degree of P_new should be less than k
        // The degree of Q_new should be k
        if p.len() > k {
            p.truncate(k);
        }
        if q.len() > k + 1 {
            q.truncate(k + 1);
        }
    }

    // When N becomes 0, P(x) will contain the terms of interest,
    // and Q(x) will be 1. The N-th term (which is now a_0) is p[0]/q[0].
    // Since q[0] will be 1, it's just p[0].
    p[0] * q[0].inv()
}
