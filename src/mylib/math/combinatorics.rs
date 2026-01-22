use cargo_snippet::snippet;
#[snippet(prefix = "use mod_combinatorics::*;")]
pub mod i64_combinatorics {
    /// 組み合わせ nCk を計算する。
    ///
    /// 計算量: O(k)
    pub fn comb(n: i64, k: i64) -> i64 {
        if n < 0 || k < 0 || n < k {
            return 0;
        }
        // comb(n, k) = (n - k + 1) / k * comb(n, k - 1) を使う
        (1..=k).fold(1, |acc, i| acc * (n - i + 1) / i)
    }

    /// 順列 nPk を計算する。
    ///
    /// 計算量: O(k)
    pub fn perm(n: i64, k: i64) -> i64 {
        if n < 0 || k < 0 || n < k {
            return 0;
        }
        (n - k + 1..=n).product::<i64>()
    }

    /// 階乗 n! を計算する。
    ///
    /// 計算量: O(n)
    pub fn factorial(n: i64) -> i64 {
        if n < 0 {
            return 0;
        }
        (1..=n).product::<i64>()
    }
}

#[snippet(prefix = "use mod_combinatorics::*;")]
pub mod mod_combinatorics {
    use ac_library::modint::ModIntBase;

    #[derive(Clone, Debug)]
    pub struct Comb<Mint: ModIntBase> {
        fac: Vec<Mint>,
        invfac: Vec<Mint>,
    }

    impl<Mint: ModIntBase> Comb<Mint> {
        /// 階乗とその逆元を `max_val` まで前計算する。
        ///
        /// 計算量: O(max_val)
        pub fn new(max_val: usize) -> Self {
            let mut inv = vec![Mint::new(0); max_val + 1];
            let mut fac = vec![Mint::new(0); max_val + 1];
            let mut invfac = vec![Mint::new(0); max_val + 1];

            fac[0] = 1.into();
            fac[1] = 1.into();

            invfac[0] = 1.into();
            invfac[1] = 1.into();

            inv[1] = 1.into();

            let modulus = Mint::modulus() as usize;

            for i in 2..=max_val {
                // modulus = (modulus / i) * i + (modulus % i) なので
                // inv[i] = -inv[modulus % i] * (modulus / i) となる
                inv[i] = -inv[modulus % i] * Mint::new(modulus / i);
                fac[i] = fac[i - 1] * Mint::new(i);
                invfac[i] = invfac[i - 1] * inv[i];
            }

            Self { fac, invfac }
        }

        pub fn comb(&self, n: usize, k: usize) -> Mint {
            if n < k {
                0.into()
            } else {
                self.fac[n] * self.invfac[k] * self.invfac[n - k]
            }
        }

        pub fn perm(&self, n: usize, k: usize) -> Mint {
            if n < k {
                0.into()
            } else {
                self.fac[n] * self.invfac[n - k]
            }
        }

        pub fn factorial(&self, n: usize) -> Mint {
            self.fac[n]
        }

        pub fn inv_factorial(&self, n: usize) -> Mint {
            self.invfac[n]
        }
    }
}

#[cfg(test)]
mod tests_i64_combinatorics {

    use super::i64_combinatorics::*;

    #[test]
    fn test_comb() {
        assert_eq!(comb(5, 3), 10);
        assert_eq!(comb(5, 0), 1);
        assert_eq!(comb(5, 5), 1);
        assert_eq!(comb(10, 500), 0);
        assert_eq!(comb(0, 0), 1);
    }

    #[test]
    fn test_perm() {
        assert_eq!(perm(5, 3), 60);
        assert_eq!(perm(5, 0), 1);
        assert_eq!(perm(5, 5), 120);
        assert_eq!(perm(0, 0), 1);
        assert_eq!(perm(3, 5), 0);
        assert_eq!(perm(-1, 3), 0);
        assert_eq!(perm(5, -1), 0);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(-1), 0);
    }
}

#[cfg(test)]
mod tests_mod_combinatorics {

    use super::mod_combinatorics::*;
    use ac_library::ModInt998244353 as Mint;

    #[test]
    fn test_comb() {
        {
            let comb = Comb::<Mint>::new(10);
            assert_eq!(comb.comb(5, 3), 10.into());
            assert_eq!(comb.comb(5, 0), 1.into());
            assert_eq!(comb.comb(5, 5), 1.into());
            assert_eq!(comb.comb(10, 5), 252.into());
            assert_eq!(comb.comb(10, 500), 0.into());
            assert_eq!(comb.comb(0, 0), 1.into());
        }
        {
            let comb = Comb::<Mint>::new(10000);
            assert_eq!(comb.comb(10000, 5000), 156178480.into());
        }
    }

    #[test]
    fn test_perm() {
        {
            let comb = Comb::<Mint>::new(10);
            assert_eq!(comb.perm(5, 3), 60.into());
            assert_eq!(comb.perm(5, 0), 1.into());
            assert_eq!(comb.perm(5, 5), 120.into());
            assert_eq!(comb.perm(0, 0), 1.into());
            assert_eq!(comb.perm(3, 4), 0.into());
            assert_eq!(comb.perm(3, 5), 0.into());
        }
        {
            let comb = Comb::<Mint>::new(10000);
            assert_eq!(comb.perm(10000, 5000), 709300690.into());
        }
    }

    #[test]
    fn test_factorial() {
        {
            let comb = Comb::<Mint>::new(5);
            assert_eq!(comb.factorial(0), 1.into());
            assert_eq!(comb.factorial(1), 1.into());
            assert_eq!(comb.factorial(5), 120.into());
        }
        {
            let comb = Comb::<Mint>::new(10000);
            assert_eq!(comb.factorial(10000), 777990065.into());
        }
    }

    #[test]
    fn test_inv_factorial() {
        {
            let comb = Comb::<Mint>::new(5);
            assert_eq!(comb.inv_factorial(0), 1.into());
            assert_eq!(comb.inv_factorial(1), 1.into());
            assert_eq!(comb.inv_factorial(5), Mint::new(120).inv());
        }
        {
            let comb = Comb::<Mint>::new(10000);
            assert_eq!(comb.inv_factorial(10000), Mint::new(777990065).inv());
        }
    }
}
