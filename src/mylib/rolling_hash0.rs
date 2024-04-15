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

    pub struct RollingHash {
        hash_list: Vec<ModInt261M1>, // hash_list[i] = xs[0..i] のハッシュ値
        pow_list: Vec<ModInt261M1>,  // pow_llst[i] = base^i
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
            Self {
                hash_list,
                pow_list,
            }
        }

        pub fn hash(&self, begin: usize, end: usize) -> i64 {
            let x = self.hash_list[end] - self.hash_list[begin] * self.pow_list[end - begin];
            x.val
        }
    }
}
#[cfg(test)]
mod tests {
    use super::rolling_hash::*;

    #[allow(clippy::eq_op)]
    #[test]
    fn test_rolling_hash() {
        let xs = vec![1, 2, 3, 4, 1, 2, 3];
        let hash = RollingHash::new(&xs, 100);

        assert_eq!(hash.hash(0, 3), 10203);
        assert_eq!(hash.hash(0, 3), hash.hash(4, 7));
        assert_ne!(hash.hash(0, 3), hash.hash(3, 6)); // [1,2,3] != [4,1,2]
        assert_eq!(hash.hash(0, 0), 0); // [1,2,3] != [4,1,2]
    }
}
