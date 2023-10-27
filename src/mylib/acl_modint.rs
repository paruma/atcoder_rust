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
