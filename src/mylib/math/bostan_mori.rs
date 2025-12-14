use ac_library::{Modulus, StaticModInt, convolution};

/// Bostan-Mori法
///
/// 有理関数 P(x)/Q(x) の N 次の係数を求める
///
/// # 引数
///
/// * `p` - 分子多項式 P(x) の係数ベクトル
/// * `q` - 分母多項式 Q(x) の係数ベクトル
/// * `n` - 求める係数の次数
///
/// # 戻り値
///
/// `[x^n] P(x)/Q(x)`
///
/// # 計算量
///
/// O(K log K log N), K は Q(x) の次数
pub fn bostan_mori<M: Modulus>(
    mut p: Vec<StaticModInt<M>>,
    mut q: Vec<StaticModInt<M>>,
    mut n: u64,
) -> StaticModInt<M> {
    while n > 0 {
        let k = q.len() - 1;
        if k == 0 {
            return if p.is_empty() {
                StaticModInt::new(0)
            } else {
                p[0] * q[0].inv()
            };
        }

        // Q(-x) を計算
        let mut q_neg_x = q.clone();
        for i in (1..q.len()).step_by(2) {
            q_neg_x[i] = -q_neg_x[i];
        }

        // P(x) * Q(-x) と Q(x) * Q(-x) を計算
        p = convolution(&p, &q_neg_x);
        q = convolution(&q, &q_neg_x);

        // P と Q の偶数次または奇数次の項を取り出す
        let mut p_new = Vec::new();
        if n % 2 == 1 {
            for i in (1..p.len()).step_by(2) {
                p_new.push(p[i]);
            }
        } else {
            for i in (0..p.len()).step_by(2) {
                p_new.push(p[i]);
            }
        }
        p = p_new;

        let mut q_new = Vec::new();
        for i in (0..q.len()).step_by(2) {
            q_new.push(q[i]);
        }
        q = q_new;

        n /= 2;
    }

    if p.is_empty() {
        StaticModInt::new(0)
    } else {
        p[0] * q[0].inv()
    }
}

/// 線形漸化式の第N項
///
/// a_n = c_0 a_{n-1} + c_1 a_{n-2} + ... + c_{k-1} a_{n-k} で定義される
/// 線形漸化式の第N項を求める
///
/// # 引数
///
/// * `initial_terms` - 初期値 a_0, a_1, ..., a_{k-1}
/// * `coeffs` - 係数 c_0, c_1, ..., c_{k-1}
/// * `n` - 求める項のインデックス (0-indexed)
///
/// # 戻り値
///
/// a_n
///
/// # 計算量
///
/// O(K log K log N), K は漸化式の次数 (coeffs.len())
pub fn nth_linearly_recurrent_sequence<M: Modulus>(
    initial_terms: &[StaticModInt<M>],
    coeffs: &[StaticModInt<M>],
    n: u64,
) -> StaticModInt<M> {
    if n < initial_terms.len() as u64 {
        return initial_terms[n as usize];
    }

    let k = coeffs.len();

    // 漸化式を計算するには k 個の初期項 (a_0, ..., a_{k-1}) が必要
    assert!(
        initial_terms.len() >= k,
        "初期項の数が漸化式の次数より不足しています。initial_terms.len()={}, 漸化式の次数coeffs.len()={}",
        initial_terms.len(),
        coeffs.len()
    );
    if k == 0 {
        return if n < initial_terms.len() as u64 {
            initial_terms[n as usize]
        } else {
            StaticModInt::new(0)
        };
    }

    // 特性多項式 Q(x) = 1 - c_0 x - c_1 x^2 - ... - c_{k-1} x^k
    let mut q: Vec<StaticModInt<M>> = vec![StaticModInt::raw(0); k + 1];
    q[0] = StaticModInt::new(1);
    for i in 0..k {
        q[i + 1] = -coeffs[i];
    }

    // P(x) を計算する。
    // 線形漸化式の母関数を A(x) とすると、A(x)Q(x) の x^k 以上の項は漸化式
    // a_n - Σ c_i a_{n-1-i} = 0 により全て0になる。
    // よって A(x)Q(x) = P(x) は k-1 次以下の多項式となる。
    // したがって A(x) = P(x)/Q(x) と表現でき、P(x) は A(x) の k 次未満の項と
    // Q(x) の積を x^k を法として計算することで求まる。
    // P(x) = (Σ_{i=0}^{k-1} a_i x^i) * Q(x) (mod x^k)
    let p_full = convolution(&initial_terms[..k], &q);
    let mut p = p_full.to_vec();
    p.truncate(k);

    bostan_mori(p, q, n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ac_library::Mod998244353;
    use rand::Rng;
    use rand::rngs::ThreadRng;

    // フィボナッチ数列: F_0 = 0, F_1 = 1, F_n = F_{n-1} + F_{n-2}
    // 係数: c_1 = 1, c_2 = 1
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
                nth_linearly_recurrent_sequence(&initial_terms, &coeffs, i as u64),
                fib_terms[i],
                "Fibonacci({}) failed",
                i
            );
        }

        // 大きなNでテスト
        // F_50 mod 998244353
        let n_50 = 50;
        let expected_f50 = Mint::new(607336789);
        assert_eq!(
            nth_linearly_recurrent_sequence(&initial_terms, &coeffs, n_50),
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
        // 期待値: 0, 1, 2, 3, 4, ... (a_n = n)
        let initial_terms = [Mint::new(0), Mint::new(1)];
        let coeffs = [Mint::new(2), Mint::new(-1)];

        for i in 0..10 {
            assert_eq!(
                nth_linearly_recurrent_sequence(&initial_terms, &coeffs, i as u64),
                Mint::new(i),
                "Simple test failed for n={}",
                i
            );
        }
    }

    // ランダムテスト用の愚直な実装
    fn naive_linear_recurrence<M: Modulus>(
        initial_terms: &[StaticModInt<M>],
        coeffs: &[StaticModInt<M>],
        n: u64,
    ) -> StaticModInt<M> {
        let k = initial_terms.len();
        if n < k as u64 {
            return initial_terms[n as usize];
        }

        let mut a = initial_terms.to_vec();
        for i in k..=n as usize {
            let mut sum = StaticModInt::new(0);
            for j in 0..coeffs.len() {
                sum += coeffs[j] * a[i - 1 - j];
            }
            a.push(sum);
        }
        a[n as usize]
    }

    #[test]
    #[ignore]
    fn test_bostan_mori_random() {
        type Mint = StaticModInt<Mod998244353>;
        let mut rng = ThreadRng::default();

        for _ in 0..100 {
            let k = rng.random_range(1..=5);

            let initial_terms: Vec<Mint> = (0..k)
                .map(|_| Mint::new(rng.random_range(0..998244353)))
                .collect();

            let coeffs: Vec<Mint> = (0..k)
                .map(|_| Mint::new(rng.random_range(0..998244353)))
                .collect();

            let n = rng.random_range(0..=50);

            let expected = naive_linear_recurrence(&initial_terms, &coeffs, n);
            let actual = nth_linearly_recurrent_sequence(&initial_terms, &coeffs, n);

            assert_eq!(
                actual, expected,
                "Random test failed for k={}, n={}, initial_terms={:?}, coeffs={:?}",
                k, n, &initial_terms, &coeffs
            );
        }
    }
}
