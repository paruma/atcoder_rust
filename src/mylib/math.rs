use cargo_snippet::snippet;

#[snippet(prefix = "use math_tools::*;")]
pub mod math_tools {
    use std::collections::HashMap;

    use num::Integer;
    use num_integer::Roots;

    pub fn divisor(n: i64) -> Vec<i64> {
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

    pub fn is_prime(n: i64) -> bool {
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

    pub fn prime_factorize(n: i64) -> HashMap<i64, i64> {
        assert!(n >= 1);
        let mut cnt_table: HashMap<i64, i64> = HashMap::new();
        let mut n = n;
        for i in 2..=n.sqrt() {
            if n.is_multiple_of(&i) {
                // n を i で割れるだけ割る
                let mut cnt = 0;
                while n.is_multiple_of(&i) {
                    n /= i;
                    cnt += 1;
                }
                cnt_table.insert(i, cnt);
            }
        }
        if n != 1 {
            cnt_table.insert(n, 1);
        }
        cnt_table
    }

    pub fn euler_phi(n: i64) -> i64 {
        // n = p[1]^{e[1]} * ... * p[k]^{e[k]} と素因数分解できるとき
        // euler_phi(n) = n * ((p[1] - 1)/p[1]) * ... * ((p[k] - 1)/p[k]) で表せる。
        assert!(n >= 1);
        let pf = prime_factorize(n);
        let mut res = n;
        for p in pf.keys() {
            res = res / p * (p - 1);
        }

        res
    }

    fn frac0<T>(n: T, acc: T) -> T
    where
        T: std::ops::Sub<Output = T> + std::ops::Mul + num::Zero + num::One + Copy,
    {
        if n.is_zero() {
            acc
        } else {
            frac0(n - T::one(), n * acc)
        }
    }

    pub fn frac<T>(n: T) -> T
    where
        T: std::ops::Sub<Output = T> + std::ops::Mul + num::Zero + num::One + Copy,
    {
        frac0(n, T::one())
    }

    pub fn permutation<T>(n: T, k: T) -> T
    where
        T: std::ops::Sub<Output = T>
            + std::ops::Mul
            + std::ops::Div<Output = T>
            + num::Zero
            + num::One
            + Copy,
    {
        // n!/(n-k)!
        frac(n) / frac(n - k)
    }

    pub fn comb<T>(n: T, k: T) -> T
    where
        T: std::ops::Sub<Output = T>
            + std::ops::Mul
            + std::ops::Div<Output = T>
            + num::Zero
            + num::One
            + Copy,
    {
        // n!/(k!(n-k)!)
        frac(n) / frac(n - k) / frac(k)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use maplit::hashmap;

    use super::math_tools::*;

    #[test]
    fn test_divisor() {
        let test = |n: i64, ans: &[i64]| {
            let mut divisor = divisor(n);
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
        .map(|(n, expected_is_prime)| TestCase { n, expected_is_prime });

        for test_case in test_case_list {
            test_case.check();
        }
    }

    #[test]
    fn test_prime_factorize() {
        struct TestCase {
            n: i64,
            expected: HashMap<i64, i64>,
        }
        impl TestCase {
            fn check(&self) {
                assert_eq!(prime_factorize(self.n), self.expected);
            }
        }
        let test_case_list = [
            (1, HashMap::new()),
            (2, hashmap! {2=> 1}),
            (3, hashmap! {3=> 1}),
            (4, hashmap! {2=> 2}),
            (12, hashmap! {2=> 2, 3=>1}),
            (720, hashmap! {2=> 4, 3=>2, 5=>1}),
        ]
        .map(|(n, expected)| TestCase { n, expected });

        for test_case in test_case_list {
            test_case.check();
        }

        prime_factorize(12).values().map(|cnt| cnt + 1).product::<i64>();
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

        prime_factorize(12).values().map(|cnt| cnt + 1).product::<i64>();
    }
    #[test]
    fn test_frac() {
        use crate::mylib::modint_field::rf::*;
        assert_eq!(frac(5), 120);
        assert_eq!(frac(RF::new(5)), RF::new(120));
    }

    #[test]
    fn test_permutation() {
        use crate::mylib::modint_field::rf::*;
        assert_eq!(permutation(5, 3), 60); //5*4*3=60
        assert_eq!(permutation(RF::new(5), RF::new(3)), RF::new(60));
    }
}
