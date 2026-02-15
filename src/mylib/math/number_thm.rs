use cargo_snippet::snippet;

#[snippet]
/// n の正の約数を列挙する。
///
/// # 計算量
/// O(sqrt(n))
pub fn divisors(n: i64) -> Vec<i64> {
    use num::Integer;
    use num_integer::Roots;

    assert!(n >= 1);
    let mut retval: Vec<i64> = Vec::new();
    for i in 1..=n.sqrt() {
        if n.is_multiple_of(&i) {
            retval.push(i);
            if i * i != n {
                retval.push(n / i);
            }
        }
    }

    retval
}

#[snippet]
/// n が素数かどうか判定する
///
/// # 計算量
/// O(sqrt(n))
pub fn is_prime(n: i64) -> bool {
    use num::Integer;
    use num_integer::Roots;

    if n <= 1 {
        return false;
    }
    for i in 2..=n.sqrt() {
        if n.is_multiple_of(&i) {
            return false;
        }
    }
    true
}

#[snippet]
/// n を素因数分解する。素数とその指数のペアのリストを返す。
///
/// # 計算量
/// O(sqrt(n))
pub fn prime_factorize(n: i64) -> Vec<(i64, i64)> {
    use num::Integer;
    use num_integer::Roots;

    assert!(n >= 1);
    let mut res = Vec::new();
    let mut n = n;
    for i in 2..=n.sqrt() {
        if n.is_multiple_of(&i) {
            // n を i で割れるだけ割る
            let mut cnt = 0;
            while n.is_multiple_of(&i) {
                n /= i;
                cnt += 1;
            }
            res.push((i, cnt));
        }
    }
    if n != 1 {
        res.push((n, 1));
    }
    res
}

#[snippet(include = "prime_factorize")]
/// 0 から n - 1 までの数で n と互いに素とになる個数を求める(オイラーのトーシェント関数)
///
/// # 計算量
/// O(sqrt(n))
pub fn euler_phi(n: i64) -> i64 {
    // n = p[1]^{e[1]} * ... * p[k]^{e[k]} と素因数分解できるとき
    // euler_phi(n) = n * ((p[1] - 1)/p[1]) * ... * ((p[k] - 1)/p[k]) で表せる。
    assert!(n >= 1);
    let pf = prime_factorize(n);
    let mut res = n;
    for (p, _) in pf {
        res = res / p * (p - 1);
    }

    res
}

#[snippet(prefix = "use eratosthenes_sieve::*;")]
pub mod eratosthenes_sieve {
    /// エラトステネスのふるいを用いて素数判定を行う。
    ///
    /// メモリ効率のため `is_prime_list` のみを持つ。
    #[derive(Clone, Debug)]
    pub struct EratosthenesSieve {
        is_prime_list: Vec<bool>,
    }

    impl EratosthenesSieve {
        /// [0, n] の区間でエラトステネスのふるいをする
        ///
        /// # 計算量
        /// O(n log(log(n)))
        pub fn new(n: usize) -> Self {
            let mut is_prime_list = vec![true; n + 1];
            is_prime_list[0] = false;
            is_prime_list[1] = false;
            for p in 2..=n {
                if !is_prime_list[p] {
                    continue;
                }
                for q in (p * 2..=n).step_by(p) {
                    is_prime_list[q] = false;
                }
            }
            Self { is_prime_list }
        }

        /// n が素数かどうか判定する
        ///
        /// # 計算量
        /// O(1)
        pub fn is_prime(&self, n: usize) -> bool {
            self.is_prime_list[n]
        }
    }

    /// エラトステネスのふるいを用いて、最小素因数（min prime factor）を管理する。
    ///
    /// 素因数分解や約数列挙を高速に行うことができる。
    #[derive(Clone, Debug)]
    pub struct EratosthenesSieveMinFactor {
        min_factor_list: Vec<usize>,
    }

    impl EratosthenesSieveMinFactor {
        /// [0, n] の区間でエラトステネスのふるいをする
        ///
        /// # 計算量
        /// O(n log(log(n)))
        pub fn new(n: usize) -> Self {
            let mut min_factor_list = vec![0; n + 1];
            for p in 2..=n {
                if min_factor_list[p] != 0 {
                    continue;
                }
                for q in (p..=n).step_by(p) {
                    if min_factor_list[q] == 0 {
                        // 0 は初期値（未決定）であることを表す
                        min_factor_list[q] = p;
                    }
                }
            }
            Self { min_factor_list }
        }

        /// n が素数かどうか判定する
        ///
        /// # 計算量
        /// O(1)
        pub fn is_prime(&self, n: usize) -> bool {
            n >= 2 && self.min_factor_list[n] == n
        }

        /// n を素因数分解する。素数とその指数のペアのリストを返す。
        ///
        /// # 計算量
        /// O(log n)
        pub fn prime_factorize(&self, n: usize) -> Vec<(usize, usize)> {
            let mut n = n;
            let mut res = Vec::new();
            while n > 1 {
                let p = self.min_factor_list[n];
                let mut exp = 0;
                while self.min_factor_list[n] == p {
                    n /= p;
                    exp += 1;
                }
                res.push((p, exp));
            }

            res
        }

        /// n の正の約数を列挙する。
        ///
        /// # 計算量
        /// O(nの約数の個数)
        pub fn divisors(&self, n: usize) -> Vec<usize> {
            let mut res = vec![1];
            let pf = self.prime_factorize(n);
            for (p, e) in pf {
                let n = res.len();
                for i in 0..n {
                    let mut tmp = 1;
                    for _ in 0..e {
                        tmp *= p;
                        res.push(res[i] * tmp);
                    }
                }
            }
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::eratosthenes_sieve::{EratosthenesSieve, EratosthenesSieveMinFactor};
    use super::*;

    #[test]
    fn test_divisor() {
        let test = |n: i64, ans: &[i64]| {
            let mut divisor = divisors(n);
            divisor.sort_unstable();
            assert_eq!(divisor, ans);
        };
        test(1, &[1]);
        test(2, &[1, 2]);
        test(16, &[1, 2, 4, 8, 16]);
        test(12, &[1, 2, 3, 4, 6, 12]);
    }

    #[test]
    fn test_is_prime() {
        struct TestCase {
            n: i64,
            expected_is_prime: bool,
        }
        impl TestCase {
            fn check(&self) {
                assert_eq!(is_prime(self.n), self.expected_is_prime);
            }
        }
        let test_case_list = [
            (-1, false),
            (0, false),
            (1, false),
            (2, true),
            (3, true),
            (4, false),
            (100, false),
            (101, true),
        ]
        .map(|(n, expected_is_prime)| TestCase {
            n,
            expected_is_prime,
        });

        for test_case in test_case_list {
            test_case.check();
        }
    }

    #[test]
    fn test_prime_factorize() {
        struct TestCase {
            n: i64,
            expected: Vec<(i64, i64)>,
        }
        impl TestCase {
            fn check(&self) {
                assert_eq!(prime_factorize(self.n), self.expected);
            }
        }
        let test_case_list = [
            (1, vec![]),
            (2, vec![(2, 1)]),
            (3, vec![(3, 1)]),
            (4, vec![(2, 2)]),
            (12, vec![(2, 2), (3, 1)]),
            (720, vec![(2, 4), (3, 2), (5, 1)]),
        ]
        .map(|(n, expected)| TestCase { n, expected });

        for test_case in test_case_list {
            test_case.check();
        }

        prime_factorize(12)
            .iter()
            .map(|&(_, cnt)| cnt + 1)
            .product::<i64>();
    }

    #[test]
    fn test_euler_ph() {
        struct TestCase {
            n: i64,
            expected: i64,
        }
        impl TestCase {
            fn check(&self) {
                assert_eq!(euler_phi(self.n), self.expected);
            }
        }
        let test_case_list = [(1, 1), (2, 1), (3, 2), (12, 4), (720, 192)]
            .map(|(n, expected)| TestCase { n, expected });

        for test_case in test_case_list {
            test_case.check();
        }

        prime_factorize(12)
            .iter()
            .map(|&(_, cnt)| cnt + 1)
            .product::<i64>();
    }

    #[test]
    fn test_eratosthenes_is_prime() {
        let sieve = EratosthenesSieve::new(12);
        assert!(!sieve.is_prime(0));
        assert!(!sieve.is_prime(1));
        assert!(sieve.is_prime(2));
        assert!(sieve.is_prime(3));
        assert!(!sieve.is_prime(4));
        assert!(sieve.is_prime(5));
        assert!(!sieve.is_prime(6));
        assert!(sieve.is_prime(7));
        assert!(!sieve.is_prime(8));
        assert!(!sieve.is_prime(9));
        assert!(!sieve.is_prime(10));
        assert!(sieve.is_prime(11));
        assert!(!sieve.is_prime(12));

        let sieve_mf = EratosthenesSieveMinFactor::new(12);
        assert!(!sieve_mf.is_prime(0));
        assert!(!sieve_mf.is_prime(1));
        assert!(sieve_mf.is_prime(2));
        assert!(sieve_mf.is_prime(3));
        assert!(!sieve_mf.is_prime(4));
        assert!(sieve_mf.is_prime(5));
        assert!(!sieve_mf.is_prime(6));
        assert!(sieve_mf.is_prime(7));
        assert!(!sieve_mf.is_prime(8));
        assert!(!sieve_mf.is_prime(9));
        assert!(!sieve_mf.is_prime(10));
        assert!(sieve_mf.is_prime(11));
        assert!(!sieve_mf.is_prime(12));
    }

    #[test]
    fn test_eratosthenes_prime_factorize() {
        let sieve = EratosthenesSieveMinFactor::new(100);
        assert_eq!(sieve.prime_factorize(1), vec![]);
        assert_eq!(sieve.prime_factorize(2), vec![(2, 1)]);
        assert_eq!(sieve.prime_factorize(3), vec![(3, 1)]);
        assert_eq!(sieve.prime_factorize(4), vec![(2, 2)]);
        assert_eq!(sieve.prime_factorize(12), vec![(2, 2), (3, 1)]);
        assert_eq!(sieve.prime_factorize(84), vec![(2, 2), (3, 1), (7, 1)]);
        assert_eq!(sieve.prime_factorize(97), vec![(97, 1)]);
    }

    #[test]
    #[ignore]
    fn test_eratosthenes_exhaustive() {
        let max_n = 1000;
        let sieve = EratosthenesSieve::new(max_n);
        let sieve_mf = EratosthenesSieveMinFactor::new(max_n);

        for n in 0..=max_n {
            let is_p = is_prime(n as i64);
            assert_eq!(sieve.is_prime(n), is_p, "Sieve is_prime failed for {}", n);
            assert_eq!(
                sieve_mf.is_prime(n),
                is_p,
                "SieveMinFactor is_prime failed for {}",
                n
            );

            if n >= 1 {
                let pf_naive = prime_factorize(n as i64)
                    .into_iter()
                    .map(|(p, e)| (p as usize, e as usize))
                    .collect_vec();
                assert_eq!(
                    sieve_mf.prime_factorize(n),
                    pf_naive,
                    "SieveMinFactor prime_factorize failed for {}",
                    n
                );

                let mut div_mf = sieve_mf.divisors(n);
                div_mf.sort_unstable();
                let mut div_naive = divisors(n as i64)
                    .into_iter()
                    .map(|d| d as usize)
                    .collect_vec();
                div_naive.sort_unstable();
                assert_eq!(
                    div_mf, div_naive,
                    "SieveMinFactor divisors failed for {}",
                    n
                );
            }
        }
    }

    #[test]
    fn test_eratosthenes_divisors() {
        let sieve = EratosthenesSieveMinFactor::new(100);
        let sort = |xs: Vec<usize>| xs.into_iter().sorted().collect_vec();
        assert_eq!(sort(sieve.divisors(1)), vec![1]);
        assert_eq!(sort(sieve.divisors(2)), vec![1, 2]);
        assert_eq!(sort(sieve.divisors(3)), vec![1, 3]);
        assert_eq!(sort(sieve.divisors(4)), vec![1, 2, 4]);
        assert_eq!(sort(sieve.divisors(12)), vec![1, 2, 3, 4, 6, 12]);
        assert_eq!(sort(sieve.divisors(27)), vec![1, 3, 9, 27]);
        assert_eq!(
            sort(sieve.divisors(72)),
            vec![1, 2, 3, 4, 6, 8, 9, 12, 18, 24, 36, 72]
        );

        assert_eq!(sort(sieve.divisors(97)), vec![1, 97]);
    }
}
