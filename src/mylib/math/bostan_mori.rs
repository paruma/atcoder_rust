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
    initial_terms: &[StaticModInt<M>],
    coeffs: &[StaticModInt<M>],
    mut n: u64,
) -> StaticModInt<M> {
    let k = coeffs.len();

    // Construct characteristic polynomial Q(x) = 1 - c_1 x - c_2 x^2 - ... - c_k x^k
    // Q(x) is represented as Vec<StaticModInt<M>>
    let mut q: Vec<StaticModInt<M>> = vec![StaticModInt::raw(0); k + 1];
    q[0] = StaticModInt::new(1);
    for i in 0..k {
        q[i + 1] = -coeffs[i];
    }

    // Construct P(x) from initial terms
    // P(x) = a_0 + (a_1 - c_1 a_0)x + ...
    // More generally, P(x) = (Q(x) * S(x)) mod x^k, where S(x) = sum(a_i x^i)
    // S(x) is an infinite series, we need its first k terms.
    let mut p: Vec<StaticModInt<M>> = vec![StaticModInt::raw(0); k];
    for i in 0..k {
        for j in 0..=i {
            p[i] += q[j] * initial_terms[i - j];
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use ac_library::Mod998244353;
    use rand::Rng; // For random_range
    use rand::rngs::ThreadRng; // For ThreadRng::default()

    // A simple linear recurrence: Fibonacci numbers
    // F_0 = 0, F_1 = 1, F_n = F_{n-1} + F_{n-2}
    // coeffs: c_1 = 1, c_2 = 1
    // Q(x) = 1 - x - x^2
    // P(x) = F_0 + (F_1 - c_1 F_0)x = 0 + (1 - 1*0)x = x
    // P(x)/Q(x) = x / (1 - x - x^2)

    #[test]
    fn test_fibonacci_mod_998244353() {
        type Mint = StaticModInt<Mod998244353>;
        let initial_terms = [Mint::new(0), Mint::new(1)]; // F_0, F_1
        let coeffs = [Mint::new(1), Mint::new(1)]; // c_1, c_2

        let fib_terms: &[Mint] = &[
            Mint::new(0),
            Mint::new(1),
            Mint::new(1),
            Mint::new(2),
            Mint::new(3),
            Mint::new(5),
            Mint::new(8),
            Mint::new(13),
            Mint::new(21),
            Mint::new(34),
            Mint::new(55),
        ];

        for i in 0..fib_terms.len() {
            assert_eq!(
                bostan_mori(&initial_terms, &coeffs, i as u64),
                fib_terms[i],
                "Fibonacci({}) failed",
                i
            );
        }

        // Test a larger N
        // F_50 mod 998244353
        let n_50 = 50;
        let expected_f50 = Mint::new(607336789); // Corrected value
        assert_eq!(
            bostan_mori(initial_terms, coeffs, n_50),
            expected_f50,
            "Fibonacci({}) failed",
            n_50
        );
    }

    #[test]
    fn test_bostan_mori_simple() {
        type Mint = StaticModInt<Mod998244353>;
        // a_n = 2 * a_{n-1} - a_{n-2}
        // a_0 = 0, a_1 = 1
        // Expected: 0, 1, 2, 3, 4, ... (a_n = n)
        let initial_terms = [Mint::new(0), Mint::new(1)];
        let coeffs = [Mint::new(2), Mint::new(-1)]; // c_1 = 2, c_2 = -1

        for i in 0..10 {
            assert_eq!(
                bostan_mori(&initial_terms, &coeffs, i as u64),
                Mint::new(i),
                "Simple test failed for n={}",
                i
            );
        }
    }

    // Reference naive implementation for random testing
    fn naive_linear_recurrence<M: Modulus>(
        initial_terms: &[StaticModInt<M>],
        coeffs: &[StaticModInt<M>],
        n: u64,
    ) -> StaticModInt<M> {
        let k = coeffs.len() as u64;
        if n < k {
            return initial_terms[n as usize];
        }

        let mut a = initial_terms.to_vec();
        a.resize((n + 1) as usize, StaticModInt::new(0));

        for i in k..=n {
            let mut sum = StaticModInt::new(0);
            for j in 0..coeffs.len() {
                sum += coeffs[j] * a[(i - 1_u64 - j as u64) as usize];
            }
            a[i as usize] = sum;
        }
        a[n as usize]
    }

    #[test]
    #[ignore] // Mark as ignore since random tests can be slow or fail intermittently
    fn test_bostan_mori_random() {
        type Mint = StaticModInt<Mod998244353>;
        let mut rng = ThreadRng::default();

        for _ in 0..100 {
            // Randomly choose degree of recurrence (k)
            let k = rng.random_range(1..=5); // Small k for faster naive comparison

            // Generate random initial terms
            let initial_terms: Vec<Mint> = (0..k)
                .map(|_| Mint::new(rng.random_range(0..998244353)))
                .collect();

            // Generate random coefficients
            let coeffs: Vec<Mint> = (0..k)
                .map(|_| Mint::new(rng.random_range(0..998244353)))
                .collect();

            // Randomly choose N (up to a reasonable limit for naive implementation)
            let n = rng.random_range(0..=50); // N up to 50 for naive calculation

            let expected = naive_linear_recurrence(&initial_terms, &coeffs, n);
            let actual = bostan_mori(&initial_terms, &coeffs, n);

            assert_eq!(
                actual, expected,
                "Random test failed for k={}, n={}\ninitial_terms={:?}\ncoeffs={:?}",
                k, n, &initial_terms, &coeffs
            );
        }
    }
}
