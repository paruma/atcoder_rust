use crate::mylib::math::modint_u64::modint_u64::ModInt2305843009213693951;
use cargo_snippet::snippet;

#[snippet(prefix = "use rolling_hash::*;", include = "modint_u64")]
pub mod rolling_hash {
    type Mint = super::ModInt2305843009213693951;

    pub fn generate_random_base() -> i64 {
        use rand::{rngs::SmallRng, Rng, SeedableRng};
        let mut rng = SmallRng::from_entropy();
        rng.gen_range(2..Mint::modulus() as i64)
    }

    #[derive(Clone, Debug)]
    pub struct RollingHash {
        hash_list: Vec<Mint>, // hash_list[i] = xs[0..i] のハッシュ値
        pow_list: Vec<Mint>,  // pow_list[i] = base^i
        length: usize,
    }

    impl RollingHash {
        /// # Arguments
        /// * `xs` - 値は1以上にする。0があると違う長さが同じハッシュ値になってしまう可能性が高まる。
        ///          char を i64 にする場合は `ch as i64` のように変換するとよい。
        /// * `base` - generate_random_base() で乱数生成した値を使う
        ///
        /// # Examples
        /// ```
        /// use atcoder_rust::mylib::rolling_hash0::rolling_hash::*;
        /// use atcoder_rust::mylib::rolling_hash0::rolling_hash::generate_random_base;
        ///
        /// let chars = ['a', 'b', 'a', 'b', 'a'];
        /// let xs = chars.iter().copied().map(|ch| ch as i64).collect::<Vec<_>>();
        /// let base = generate_random_base();
        /// let rh = RollingHash::new(&xs, base);
        /// assert!(rh.hash(0, 3) == rh.hash(2, 5));
        /// ```
        pub fn new(xs: &[i64], base: i64) -> Self {
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

    pub fn generate_random_base() -> i64 {
        use rand::{rngs::SmallRng, Rng, SeedableRng};
        let mut rng = SmallRng::from_entropy();
        rng.gen_range(2..Mint::modulus() as i64)
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct RollingHash {
        hash: Mint,
        base: Mint,
    }
    impl RollingHash {
        pub fn get_hash(&self) -> u64 {
            self.hash.val()
        }

        /// # Arguments
        /// * `base` - generate_random_base() で乱数生成した値を使う
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

// 未テスト (ABC391 B は通ってる)
/// 2次元ローリングハッシュ
pub mod rolling_hash_2d {

    type Mint = super::ModInt2305843009213693951;

    pub fn generate_random_base() -> i64 {
        use rand::{rngs::SmallRng, Rng, SeedableRng};
        let mut rng = SmallRng::from_entropy();
        rng.gen_range(2..Mint::modulus() as i64)
    }

    #[derive(Clone, Debug)]
    pub struct RollingHash2D {
        hash_list: Vec<Vec<Mint>>, // hash_list[i][j] = xss[0..i][0..j] のハッシュ値
        pow0_list: Vec<Mint>,      // pow_list[i] = base0^i
        pow1_list: Vec<Mint>,      // pow_list[i] = base1^i
        height: usize,
        width: usize,
    }

    impl RollingHash2D {
        /// # Arguments
        /// * `base0` - generate_random_base() で乱数生成した値を使う
        /// * `base1` - generate_random_base() で乱数生成した値を使う
        pub fn new(xss: &[Vec<i64>], base0: i64, base1: i64) -> Self {
            // base > 0 とする
            let base0 = Mint::new(base0);
            let base1 = Mint::new(base1);
            let height = xss.len();
            let width = xss[0].len();
            let mut hash_list = vec![vec![Mint::new(0); width + 1]; height + 1];
            let mut pow0_list = vec![Mint::new(1); height + 1];
            let mut pow1_list = vec![Mint::new(1); width + 1];

            for i in 0..height {
                pow0_list[i + 1] = pow0_list[i] * base0;
            }

            for i in 0..width {
                pow1_list[i + 1] = pow1_list[i] * base1;
            }

            for y in 0..height {
                for x in 0..width {
                    hash_list[y + 1][x + 1] = hash_list[y][x + 1] * base0
                        + hash_list[y + 1][x] * base1
                        - hash_list[y][x] * base0 * base1
                        + xss[y][x]
                }
            }
            Self {
                hash_list,
                pow0_list,
                pow1_list,
                height,
                width,
            }
        }

        pub fn hash(
            &self,
            row_begin: usize,
            row_end: usize,
            col_begin: usize,
            col_end: usize,
        ) -> u64 {
            let x = self.hash_list[row_end][col_end]
                - self.hash_list[row_begin][col_end] * self.pow0_list[row_end - row_begin]
                - self.hash_list[row_end][col_begin] * self.pow1_list[col_end - col_begin]
                + self.hash_list[row_begin][col_begin]
                    * self.pow0_list[row_end - row_begin]
                    * self.pow1_list[col_end - col_begin];
            x.val()
        }

        pub fn width(&self) -> usize {
            self.width
        }

        pub fn height(&self) -> usize {
            self.height
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
