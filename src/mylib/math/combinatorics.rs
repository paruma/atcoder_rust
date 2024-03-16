use cargo_snippet::snippet;

#[snippet(prefix = "use mod_combinatorics::*;")]
pub mod mod_combinatorics {
    use ac_library::ModInt998244353 as Mint;

    pub struct Comb {
        fac: Vec<Mint>,
        invfac: Vec<Mint>,
    }

    impl Comb {
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
            if n < k {
                0.into()
            } else {
                self.fac[n] * self.invfac[k] * self.invfac[n - k]
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::mod_combinatorics::*;

    #[test]
    fn test_comb() {
        {
            let comb = Comb::new(10);
            assert_eq!(comb.comb(5, 3), 10.into());
            assert_eq!(comb.comb(5, 0), 1.into());
            assert_eq!(comb.comb(5, 5), 1.into());
            assert_eq!(comb.comb(10, 5), 252.into());
            assert_eq!(comb.comb(10, 500), 0.into());
        }
        {
            let comb = Comb::new(10000);
            assert_eq!(comb.comb(10000, 5000), 156178480.into());
        }
    }
}
