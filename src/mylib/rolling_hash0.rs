use crate::mylib::math::modint_u64::modint_u64::ModInt2305843009213693951;
use cargo_snippet::snippet;

#[snippet(prefix = "use rolling_hash::*;", include = "modint_u64")]
pub mod rolling_hash {

    type Mint = super::ModInt2305843009213693951;

    #[derive(Clone, Debug)]
    pub struct RollingHash {
        hash_list: Vec<Mint>, // hash_list[i] = xs[0..i] のハッシュ値
        pow_list: Vec<Mint>,  // pow_list[i] = base^i
        length: usize,
    }

    impl RollingHash {
        pub fn new(xs: &[i64], base: i64) -> Self {
            // base > 0 とする
            let base = Mint::new(base);
            let mut hash_list = vec![Mint::new(0); xs.len() + 1];
            let mut pow_list = vec![Mint::new(1); xs.len() + 1];
            for i in 0..xs.len() {
                hash_list[i + 1] = hash_list[i] * base + Mint::new(xs[i]);
                pow_list[i + 1] = pow_list[i] * base;
            }
            let length = xs.len();
            Self {
                hash_list,
                pow_list,
                length,
            }
        }

        pub fn hash(&self, begin: usize, end: usize) -> u64 {
            let x = self.hash_list[end] - self.hash_list[begin] * self.pow_list[end - begin];
            x.val()
        }

        pub fn len(&self) -> usize {
            self.length
        }

        pub fn is_empty(&self) -> bool {
            self.length == 0
        }
    }
}

#[snippet(prefix = "use monoid_rolling_hash::*;", include = "modint_u64")]
pub mod monoid_rolling_hash {
    use ac_library::Monoid;
    use std::convert::Infallible;

    type Mint = super::ModInt2305843009213693951;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct RollingHash {
        hash: Mint,
        base: Mint,
    }
    impl RollingHash {
        pub fn get_hash(&self) -> u64 {
            self.hash.val()
        }

        pub fn unit(base: i64) -> impl (Fn(i64) -> RollingHash) {
            move |x| RollingHash {
                hash: Mint::new(x),
                base: Mint::new(base),
            }
        }

        pub fn new(hash: i64, base: i64) -> Self {
            Self {
                hash: Mint::new(hash),
                base: Mint::new(base),
            }
        }
    }

    pub struct RollingHashConcat(Infallible);
    impl Monoid for RollingHashConcat {
        type S = RollingHash;
        fn identity() -> Self::S {
            RollingHash {
                hash: Mint::new(0),
                base: Mint::new(1),
            }
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            RollingHash {
                hash: a.hash * b.base + b.hash,
                base: a.base * b.base,
            }
        }
    }
}
#[cfg(test)]
mod tests_rolling_hash {
    use super::rolling_hash::*;

    #[test]
    fn test_rolling_hash_normal() {
        let xs = vec![1, 2, 3, 4, 1, 2, 3];
        let hash = RollingHash::new(&xs, 100);

        assert_eq!(hash.hash(0, 3), 10203);
        assert_eq!(hash.hash(0, 3), hash.hash(4, 7));
        assert_ne!(hash.hash(0, 3), hash.hash(3, 6)); // [1,2,3] != [4,1,2]
        assert_eq!(hash.hash(0, 0), 0); // [1,2,3] != [4,1,2]
        assert_eq!(hash.len(), 7);
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_rolling_hash_empty() {
        let hash = RollingHash::new(&[], 100);

        assert_eq!(hash.hash(0, 0), 0);
        assert_eq!(hash.len(), 0);
        assert!(hash.is_empty());
    }
}

#[cfg(test)]
mod tests_monoid_rolling_hash {
    use super::monoid_rolling_hash::*;
    use ac_library::Monoid;

    #[test]
    fn test_monoid_rolling_hash() {
        type M = RollingHashConcat;
        let rh1: RollingHash = RollingHash::new(7.into(), 25.into()); // 1 * 5 + 2
        let rh2: RollingHash = RollingHash::new(3.into(), 5.into());
        assert_eq!(
            M::binary_operation(&rh1, &rh2),
            RollingHash::new(38, 125) // 1 * 5^2 + 2 * 5 + 3
        );
        assert_eq!(M::binary_operation(&rh1, &M::identity()), rh1)
    }
}
