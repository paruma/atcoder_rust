use cargo_snippet::snippet;

#[snippet(prefix = "use comb::*;")]
#[allow(clippy::module_inception)]
pub mod comb {
    pub mod i64_combinatorics {
        /// 組み合わせ $nCk$ を計算する。
        ///
        /// 計算量: $O(K)$
        pub fn comb(n: i64, k: i64) -> i64 {
            if n < 0 || k < 0 || n < k {
                return 0;
            }
            (1..=k).fold(1, |acc, i| acc * (n - i + 1) / i)
        }
        /// 順列 $nPk$ を計算する。
        ///
        /// 計算量: $O(K)$
        pub fn perm(n: i64, k: i64) -> i64 {
            if n < 0 || k < 0 || n < k {
                return 0;
            }
            (n - k + 1..=n).product()
        }
        /// 階乗 $n!$ を計算する。
        ///
        /// 計算量: $O(N)$
        pub fn factorial(n: i64) -> i64 {
            if n < 0 { 0 } else { (1..=n).product() }
        }
    }

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
            /// 計算量: $O(N)$
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
                    inv[i] = -inv[modulus % i] * Mint::new(modulus / i);
                    fac[i] = fac[i - 1] * Mint::new(i);
                    invfac[i] = invfac[i - 1] * inv[i];
                }
                Self { fac, invfac }
            }
            pub fn comb(&self, n: usize, k: usize) -> Mint {
                assert!(
                    n < self.fac.len(),
                    "index out of range (n={}, max_val={})",
                    n,
                    self.fac.len() - 1
                );
                if n < k {
                    0.into()
                } else {
                    self.fac[n] * self.invfac[k] * self.invfac[n - k]
                }
            }
            pub fn perm(&self, n: usize, k: usize) -> Mint {
                assert!(
                    n < self.fac.len(),
                    "index out of range (n={}, max_val={})",
                    n,
                    self.fac.len() - 1
                );
                if n < k {
                    0.into()
                } else {
                    self.fac[n] * self.invfac[n - k]
                }
            }
            pub fn factorial(&self, n: usize) -> Mint {
                assert!(
                    n < self.fac.len(),
                    "index out of range (n={}, max_val={})",
                    n,
                    self.fac.len() - 1
                );
                self.fac[n]
            }
            pub fn inv_factorial(&self, n: usize) -> Mint {
                assert!(
                    n < self.invfac.len(),
                    "index out of range (n={}, max_val={})",
                    n,
                    self.invfac.len() - 1
                );
                self.invfac[n]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::comb::{i64_combinatorics, mod_combinatorics::Comb};
    use ac_library::ModInt998244353 as Mint;

    #[test]
    fn test_i64_combinatorics() {
        assert_eq!(i64_combinatorics::comb(5, 3), 10);
        assert_eq!(i64_combinatorics::comb(3, 5), 0);
        assert_eq!(i64_combinatorics::comb(-1, 0), 0);
        assert_eq!(i64_combinatorics::perm(5, 3), 60);
        assert_eq!(i64_combinatorics::perm(3, 5), 0);
        assert_eq!(i64_combinatorics::perm(3, -1), 0);
        assert_eq!(i64_combinatorics::factorial(5), 120);
        assert_eq!(i64_combinatorics::factorial(-1), 0);
    }

    #[test]
    fn test_mod_combinatorics() {
        let comb = Comb::<Mint>::new(10);
        assert_eq!(comb.comb(5, 3), Mint::new(10));
        assert_eq!(comb.comb(3, 5), Mint::new(0));
        assert_eq!(comb.perm(5, 3), Mint::new(60));
        assert_eq!(comb.perm(3, 5), Mint::new(0));
        assert_eq!(comb.factorial(5), Mint::new(120));
        assert_eq!(comb.inv_factorial(5) * comb.factorial(5), Mint::new(1));
    }

    #[test]
    #[should_panic(expected = "index out of range (n=10, max_val=5)")]
    fn test_comb_out_of_range() {
        Comb::<Mint>::new(5).comb(10, 0);
    }

    #[test]
    #[should_panic(expected = "index out of range (n=10, max_val=5)")]
    fn test_perm_out_of_range() {
        Comb::<Mint>::new(5).perm(10, 0);
    }

    #[test]
    #[should_panic(expected = "index out of range (n=10, max_val=5)")]
    fn test_factorial_out_of_range() {
        Comb::<Mint>::new(5).factorial(10);
    }

    #[test]
    #[should_panic(expected = "index out of range (n=10, max_val=5)")]
    fn test_inv_factorial_out_of_range() {
        Comb::<Mint>::new(5).inv_factorial(10);
    }
}
