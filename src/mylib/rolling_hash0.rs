use cargo_snippet::snippet;

#[snippet(prefix = "use rolling_hash::*;")]
pub mod rolling_hash {
    const MOD: i64 = (1 << 61) - 1; // 2^61 -1
    const MOD_I128: i128 = (1 << 61) - 1; // 2^61 -1

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct ModInt261M1 {
        val: i64,
    }

    impl ModInt261M1 {
        #[inline]
        pub fn new(val: i64) -> Self {
            // require: 0 <= val < 2^61
            Self { val }
        }
    }

    impl std::ops::Add for ModInt261M1 {
        type Output = Self;

        #[inline]
        fn add(self, rhs: Self) -> Self::Output {
            let mut x = self.val + rhs.val;
            if x >= MOD {
                x -= MOD;
            }
            Self::new(x)
        }
    }

    impl std::ops::Sub for ModInt261M1 {
        type Output = Self;

        #[inline]
        fn sub(self, rhs: Self) -> Self::Output {
            let mut x = MOD + self.val - rhs.val;
            if x >= MOD {
                x -= MOD;
            }
            Self::new(x)
        }
    }

    impl std::ops::Mul for ModInt261M1 {
        type Output = Self;

        #[inline]
        fn mul(self, rhs: Self) -> Self::Output {
            let x = (self.val as i128) * (rhs.val as i128);
            let mut x = ((x >> 61) + (x & MOD_I128)) as i64;

            if x >= MOD {
                x -= MOD;
            }
            Self::new(x)
        }
    }

    #[derive(Clone, Debug)]
    pub struct RollingHash {
        hash_list: Vec<ModInt261M1>, // hash_list[i] = xs[0..i] のハッシュ値
        pow_list: Vec<ModInt261M1>,  // pow_llst[i] = base^i
        length: usize,
    }

    impl RollingHash {
        pub fn new(xs: &[i64], base: i64) -> Self {
            // base > 0 とする
            let base = ModInt261M1::new(base);
            let mut hash_list = vec![ModInt261M1::new(0); xs.len() + 1];
            let mut pow_list = vec![ModInt261M1::new(1); xs.len() + 1];
            for i in 0..xs.len() {
                hash_list[i + 1] = hash_list[i] * base + ModInt261M1::new(xs[i]);
                pow_list[i + 1] = pow_list[i] * base;
            }
            let length = xs.len();
            Self {
                hash_list,
                pow_list,
                length,
            }
        }

        pub fn hash(&self, begin: usize, end: usize) -> i64 {
            let x = self.hash_list[end] - self.hash_list[begin] * self.pow_list[end - begin];
            x.val
        }

        pub fn len(&self) -> usize {
            self.length
        }

        pub fn is_empty(&self) -> bool {
            self.length == 0
        }
    }
}

#[snippet(prefix = "use monoid_rolling_hash::*;")]
pub mod monoid_rolling_hash {
    use std::convert::Infallible;

    const MOD: i64 = (1 << 61) - 1; // 2^61 -1
    const MOD_I128: i128 = (1 << 61) - 1; // 2^61 -1

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct ModInt261M1 {
        val: i64,
    }

    impl ModInt261M1 {
        #[inline]
        pub fn new(val: i64) -> Self {
            // require: 0 <= val < 2^61
            Self { val }
        }
    }

    impl std::ops::Add for ModInt261M1 {
        type Output = Self;

        #[inline]
        fn add(self, rhs: Self) -> Self::Output {
            let mut x = self.val + rhs.val;
            if x >= MOD {
                x -= MOD;
            }
            Self::new(x)
        }
    }

    impl std::ops::Sub for ModInt261M1 {
        type Output = Self;

        #[inline]
        fn sub(self, rhs: Self) -> Self::Output {
            let mut x = MOD + self.val - rhs.val;
            if x >= MOD {
                x -= MOD;
            }
            Self::new(x)
        }
    }

    impl std::ops::Mul for ModInt261M1 {
        type Output = Self;

        #[inline]
        fn mul(self, rhs: Self) -> Self::Output {
            let x = (self.val as i128) * (rhs.val as i128);
            let mut x = ((x >> 61) + (x & MOD_I128)) as i64;

            if x >= MOD {
                x -= MOD;
            }
            Self::new(x)
        }
    }
    use ac_library::Monoid;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct RollingHash {
        hash: ModInt261M1,
        base: ModInt261M1,
    }
    impl RollingHash {
        pub fn get_hash(&self) -> i64 {
            self.hash.val
        }

        pub fn unit(base: i64) -> impl (Fn(i64) -> RollingHash) {
            move |x| RollingHash {
                hash: ModInt261M1::new(x),
                base: ModInt261M1::new(base),
            }
        }

        pub fn new(hash: i64, base: i64) -> Self {
            Self {
                hash: ModInt261M1::new(hash),
                base: ModInt261M1::new(base),
            }
        }
    }

    pub struct RollingHashConcat(Infallible);
    impl Monoid for RollingHashConcat {
        type S = RollingHash;
        fn identity() -> Self::S {
            RollingHash {
                hash: ModInt261M1::new(0),
                base: ModInt261M1::new(1),
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
