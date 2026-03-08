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

    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
    pub enum Mod1e9 {}

    impl Modulus for Mod1e9 {
        const VALUE: u32 = 1_000_000_000;
        const HINT_VALUE_IS_PRIME: bool = false;

        fn butterfly_cache() -> &'static LocalKey<RefCell<Option<ButterflyCache<Self>>>> {
            thread_local! {
                static BUTTERFLY_CACHE: RefCell<Option<ButterflyCache<Mod1e9>>> = RefCell::default();
            }
            &BUTTERFLY_CACHE
        }
    }
    pub type ModInt1e9 = StaticModInt<Mod1e9>;
}

#[snippet(prefix = "use modint_to_rational::*;")]
pub mod modint_to_rational {
    use ac_library::modint::ModIntBase;
    use num_rational::Rational64;

    pub trait ToRational {
        fn to_rational(&self) -> Option<Rational64>;
        fn to_rational_str(&self) -> String {
            self.to_rational()
                .map(|x| x.to_string())
                .unwrap_or("cannot reconstruct".to_string())
        }
    }

    impl<M: ModIntBase> ToRational for M {
        /// modint を分数の形に復元する。デバッグ用。
        ///
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

#[snippet(prefix = "use dynamic_mod_int::*;")]
pub mod dynamic_mod_int {
    use ac_library::{Barrett, modint::DynamicModInt, modint::Id};

    /// 複数の DynamicModInt を同時に使用するための Id 実装群。
    ///
    /// `DynamicModInt<DefaultId>` は全てのインスタンスで同じ static `BARRETT` を共有するため、
    /// 複数の異なるモジュラスを同時に使用できない。このモジュールでは、各々が独立した
    /// `companion_barrett()` を持つ複数の `Id` 実装を提供し、複数の modint type を同時利用可能にする。
    ///
    /// # 使用例
    ///
    /// ```ignore
    /// use dynamic_mod_int::*;
    ///
    /// DynMint1::set_modulus(998244353);
    /// DynMint2::set_modulus(1000000007);
    ///
    /// let x = DynMint1::new(5);
    /// let y = DynMint2::new(3);
    /// // x と y は異なるモジュラスで独立に動作
    /// ```

    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
    pub enum Id1 {}
    impl Id for Id1 {
        fn companion_barrett() -> &'static Barrett {
            static BARRETT: Barrett = Barrett::new(998244353);
            &BARRETT
        }
    }
    pub type DynMint1 = DynamicModInt<Id1>;

    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
    pub enum Id2 {}
    impl Id for Id2 {
        fn companion_barrett() -> &'static Barrett {
            static BARRETT: Barrett = Barrett::new(1000000007);
            &BARRETT
        }
    }
    pub type DynMint2 = DynamicModInt<Id2>;

    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
    pub enum Id3 {}
    impl Id for Id3 {
        fn companion_barrett() -> &'static Barrett {
            static BARRETT: Barrett = Barrett::new(998244353);
            &BARRETT
        }
    }
    pub type DynMint3 = DynamicModInt<Id3>;

    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
    pub enum Id4 {}
    impl Id for Id4 {
        fn companion_barrett() -> &'static Barrett {
            static BARRETT: Barrett = Barrett::new(998244353);
            &BARRETT
        }
    }
    pub type DynMint4 = DynamicModInt<Id4>;

    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
    pub enum Id5 {}
    impl Id for Id5 {
        fn companion_barrett() -> &'static Barrett {
            static BARRETT: Barrett = Barrett::new(998244353);
            &BARRETT
        }
    }
    pub type DynMint5 = DynamicModInt<Id5>;
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
        sub(
            Mint::new(5) / Mint::new(12),
            Some(Rational64::new(5, 12)),
            "5/12".to_string(),
        );

        sub(Mint::new(4), Some(Rational64::new(4, 1)), "4".to_string());

        sub(Mint::new(1), Some(Rational64::new(1, 1)), "1".to_string());

        sub(Mint::new(0), Some(Rational64::new(0, 1)), "0".to_string());

        sub(
            Mint::new(-5) / Mint::new(12),
            Some(Rational64::new(-5, 12)),
            "-5/12".to_string(),
        );

        sub(
            Mint::new(100000) / Mint::new(654321),
            None,
            "cannot reconstruct".to_string(),
        );
    }

    #[test]
    fn test_dynamic_mod_int_independence() {
        use super::dynamic_mod_int::{DynMint1, DynMint2};

        // 異なるモジュラスで独立に設定
        DynMint1::set_modulus(7);
        DynMint2::set_modulus(11);

        // 各々が正しく計算できることを確認
        let x1 = DynMint1::new(5) + DynMint1::new(3); // (5 + 3) % 7 = 1
        let x2 = DynMint2::new(5) + DynMint2::new(3); // (5 + 3) % 11 = 8

        assert_eq!(x1.val(), 1);
        assert_eq!(x2.val(), 8);

        // 片方のモジュラスを変更しても、もう一方に影響しないことを確認
        DynMint1::set_modulus(13);
        let y1 = DynMint1::new(10) + DynMint1::new(5); // (10 + 5) % 13 = 2
        let y2 = DynMint2::new(10) + DynMint2::new(5); // (10 + 5) % 11 = 4

        assert_eq!(y1.val(), 2);
        assert_eq!(y2.val(), 4); // DynMint2 のモジュラスは依然として 11
    }

    #[test]
    fn test_dynamic_mod_int_many_ids() {
        use super::dynamic_mod_int::DynMint3;

        // DynMint3 も独立したモジュラスを持つことを確認
        DynMint3::set_modulus(5);
        let z = DynMint3::new(2) + DynMint3::new(4); // (2 + 4) % 5 = 1
        assert_eq!(z.val(), 1);
    }
}
