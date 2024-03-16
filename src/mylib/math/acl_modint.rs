use cargo_snippet::snippet;

#[snippet(prefix = "use static_mod_int::*;")]
pub mod static_mod_int {
    use std::{cell::RefCell, thread::LocalKey};

    use ac_library::{ButterflyCache, Modulus, StaticModInt};

    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
    pub enum Mod2 {}

    impl Modulus for Mod2 {
        const VALUE: u32 = 2;
        const HINT_VALUE_IS_PRIME: bool = true;

        fn butterfly_cache() -> &'static LocalKey<RefCell<Option<ButterflyCache<Self>>>> {
            thread_local! {
                static BUTTERFLY_CACHE: RefCell<Option<ButterflyCache<Mod2>>> = RefCell::default();
            }
            &BUTTERFLY_CACHE
        }
    }
    pub type ModInt2 = StaticModInt<Mod2>;
}

#[snippet(prefix = "use modint_to_rational::*;")]
pub mod modint_to_rational {
    use num_rational::Rational64;

    pub trait ToRational {
        fn to_rational(&self) -> Option<Rational64>;
        fn to_rational_str(&self) -> String {
            self.to_rational().map(|x| x.to_string()).unwrap_or("cannot reconstruct".to_string())
        }
    }

    impl ToRational for ac_library::ModInt998244353 {
        /// 注意: 1000 * 2000 = 2*10^6 の計算をしている
        fn to_rational(&self) -> Option<Rational64> {
            if self.val() == 0 {
                return Some(Rational64::new(0, 1));
            }

            for denom in 1..1000 {
                let denom_inv = Self::new(denom).inv();
                for numer in -1000..1000 {
                    if *self == denom_inv * Self::new(numer) {
                        return Some(Rational64::new(numer, denom));
                    }
                }
            }
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use num_rational::Rational64;

    use super::modint_to_rational::*;

    #[test]
    fn test_to_rational() {
        use ac_library::ModInt998244353 as Mint;
        fn sub(data: Mint, expected_rational: Option<Rational64>, expected_str: String) {
            let actual_rational = data.to_rational();
            let actual_str = data.to_rational_str();
            assert_eq!(actual_rational, expected_rational);
            assert_eq!(actual_str, expected_str);
        }
        sub(Mint::new(5) / Mint::new(12), Some(Rational64::new(5, 12)), "5/12".to_string());

        sub(Mint::new(4), Some(Rational64::new(4, 1)), "4".to_string());

        sub(Mint::new(1), Some(Rational64::new(1, 1)), "1".to_string());

        sub(Mint::new(0), Some(Rational64::new(0, 1)), "0".to_string());

        sub(Mint::new(-5) / Mint::new(12), Some(Rational64::new(-5, 12)), "-5/12".to_string());

        sub(Mint::new(100000) / Mint::new(654321), None, "cannot reconstruct".to_string());
    }
}
