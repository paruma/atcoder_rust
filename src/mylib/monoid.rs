use cargo_snippet::snippet;

#[snippet(prefix = "use extend_acl_monoid::*;")]
pub mod extend_acl_monoid {
    use ac_library::Monoid;

    pub trait MonoidExtPow: Monoid {
        /// base^n を求める
        fn pow(base: &Self::S, n: usize) -> Self::S {
            let mut base = base.clone();
            let mut ans = Self::identity();
            let mut n = n;

            while n > 0 {
                if n & 1 == 1 {
                    ans = Self::binary_operation(&ans, &base);
                }
                base = Self::binary_operation(&base, &base);
                n >>= 1;
            }
            ans
        }
    }

    impl<T> MonoidExtPow for T where T: Monoid {}
}

#[snippet(prefix = "use cum_monoid::*;")]
pub mod cum_monoid {
    use ac_library::{Max, Min, Monoid};

    pub struct CumMonoid<M>
    where
        M: Monoid,
    {
        prefix_prod: Vec<M::S>, // prefix_sum[i]: [0, i) の総積 (前から累積するがどこから取るか)
        suffix_prod: Vec<M::S>, // suffix_sum[i]: [i, n) の総積 (後ろから累積するがどこまで取るか)
    }

    impl<M> CumMonoid<M>
    where
        M: Monoid,
    {
        pub fn new(xs: &[M::S]) -> CumMonoid<M> {
            let mut prefix_prod = vec![M::identity(); xs.len() + 1];
            let mut suffix_prod = vec![M::identity(); xs.len() + 1];
            for i in 0..xs.len() {
                prefix_prod[i + 1] = M::binary_operation(&prefix_prod[i], &xs[i]);
            }
            for i in (0..xs.len()).rev() {
                suffix_prod[i] = M::binary_operation(&xs[i], &suffix_prod[i + 1]);
            }

            CumMonoid {
                prefix_prod,
                suffix_prod,
            }
        }

        /// [0, i) の総積 (前から累積)
        pub fn prefix_prod(&self, i: usize) -> M::S {
            self.prefix_prod[i].clone()
        }

        /// [i, n) の総積 (後ろから累積)
        pub fn suffix_prod(&self, i: usize) -> M::S {
            self.suffix_prod[i].clone()
        }

        /// [0, i), [i + 1, n) の区間で総積を取る
        pub fn prod_without1(&self, i: usize) -> M::S {
            M::binary_operation(&self.prefix_prod[i], &self.suffix_prod[i + 1])
        }

        // [0, l), [r, n) の区間で総積を取る
        pub fn prod_without_range(&self, l: usize, r: usize) -> M::S {
            M::binary_operation(&self.prefix_prod[l], &self.suffix_prod[r])
        }
    }

    pub struct CumMin {
        cum: CumMonoid<Min<i64>>,
    }

    impl CumMin {
        pub fn new(xs: &[i64]) -> CumMin {
            CumMin {
                cum: CumMonoid::new(xs),
            }
        }

        /// [0, i) の総積 (前から累積)
        pub fn prefix_min(&self, i: usize) -> i64 {
            self.cum.prefix_prod(i)
        }

        /// [i, n) の総積 (後ろから累積)
        pub fn suffix_min(&self, i: usize) -> i64 {
            self.cum.suffix_prod(i)
        }

        /// [0, i), [i + 1, n) の区間で総積を取る
        pub fn min_without1(&self, i: usize) -> i64 {
            self.cum.prod_without1(i)
        }

        // [0, l), [r, n) の区間で総積を取る
        pub fn min_without_range(&self, l: usize, r: usize) -> i64 {
            self.cum.prod_without_range(l, r)
        }
    }

    pub struct CumMax {
        cum: CumMonoid<Max<i64>>,
    }

    impl CumMax {
        pub fn new(xs: &[i64]) -> CumMax {
            CumMax {
                cum: CumMonoid::new(xs),
            }
        }

        /// [0, i) の総積 (前から累積)
        pub fn prefix_max(&self, i: usize) -> i64 {
            self.cum.prefix_prod(i)
        }

        /// [i, n) の総積 (後ろから累積)
        pub fn suffix_max(&self, i: usize) -> i64 {
            self.cum.suffix_prod(i)
        }

        /// [0, i), [i + 1, n) の区間で総積を取る
        pub fn max_without1(&self, i: usize) -> i64 {
            self.cum.prod_without1(i)
        }

        // [0, l), [r, n) の区間で総積を取る
        pub fn max_without_range(&self, l: usize, r: usize) -> i64 {
            self.cum.prod_without_range(l, r)
        }
    }
}

#[snippet(prefix = "use monoid_bitwise::*;")]
pub mod monoid_bitwise {
    use std::{
        convert::Infallible,
        marker::PhantomData,
        ops::{BitAnd, BitOr, BitXor, Not},
    };

    use ac_library::Monoid;
    use num_traits::Zero;

    pub struct BitwiseOr<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for BitwiseOr<S>
    where
        S: Copy + BitOr<Output = S> + Zero,
    {
        type S = S;
        fn identity() -> Self::S {
            S::zero()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a | *b
        }
    }

    pub struct BitwiseAnd<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for BitwiseAnd<S>
    where
        S: Copy + BitAnd<Output = S> + Not<Output = S> + Zero,
    {
        type S = S;
        fn identity() -> Self::S {
            !S::zero()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a & *b
        }
    }

    pub struct BitwiseXor<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for BitwiseXor<S>
    where
        S: Copy + BitXor<Output = S> + Zero,
    {
        type S = S;
        fn identity() -> Self::S {
            S::zero()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a ^ *b
        }
    }
}

#[snippet(prefix = "use monoid_gcd_lcm::*;")]
pub mod monoid_gcd_lcm {
    use std::{convert::Infallible, marker::PhantomData};

    use ac_library::Monoid;
    use num_integer::Integer;

    pub struct Gcd<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for Gcd<S>
    where
        S: Integer + Clone,
    {
        type S = S;
        fn identity() -> Self::S {
            S::zero()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a.gcd(b)
        }
    }

    pub struct Lcm<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for Lcm<S>
    where
        S: Integer + Clone,
    {
        type S = S;
        fn identity() -> Self::S {
            S::one()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a.lcm(b)
        }
    }
}

#[snippet(prefix = "use monoid_modint::*;")]
pub mod monoid_modint {
    use std::{convert::Infallible, marker::PhantomData};

    use ac_library::{Modulus, Monoid, StaticModInt};

    pub struct MintAdditive<Mod>(Infallible, PhantomData<fn() -> Mod>);
    impl<Mod> Monoid for MintAdditive<Mod>
    where
        Mod: Modulus,
    {
        type S = StaticModInt<Mod>;
        fn identity() -> Self::S {
            StaticModInt::raw(0)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a + b
        }
    }

    pub struct MintMultiplicative<Mod>(Infallible, PhantomData<fn() -> Mod>);
    impl<Mod> Monoid for MintMultiplicative<Mod>
    where
        Mod: Modulus,
    {
        type S = StaticModInt<Mod>;
        fn identity() -> Self::S {
            StaticModInt::raw(1)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a * b
        }
    }
}

#[snippet(prefix = "use monoid_affine::*;")]
pub mod monoid_affine {
    use std::{
        convert::Infallible,
        marker::PhantomData,
        ops::{Add, Mul},
    };

    use ac_library::Monoid;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct AffineTransform<T> {
        slope: T,
        intercept: T,
    }
    impl<T> AffineTransform<T> {
        pub fn new(slope: T, intercept: T) -> Self {
            Self { slope, intercept }
        }

        pub fn apply(&self, x: T) -> T
        where
            T: Copy + Mul<Output = T> + Add<Output = T>,
        {
            self.slope * x + self.intercept
        }

        pub fn identity() -> Self
        where
            T: From<i64>,
        {
            Self {
                slope: 1.into(),
                intercept: 0.into(),
            }
        }

        pub fn composite(&self, rhs: &Self) -> Self
        where
            T: Copy + Mul<Output = T> + Add<Output = T>,
        {
            Self {
                slope: self.slope * rhs.slope,
                intercept: self.slope * rhs.intercept + self.intercept,
            }
        }
    }

    pub struct AffineComposition<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for AffineComposition<T>
    where
        T: Copy + From<i64> + Add<Output = T> + Mul<Output = T>,
    {
        type S = AffineTransform<T>;
        fn identity() -> Self::S {
            AffineTransform::identity()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a.composite(b)
        }
    }
}

#[snippet(prefix = "use dynamic_monoid::*;")]
pub mod dynamic_monoid {
    pub trait DynamicMonoid {
        type S: Clone;
        fn identity(&self) -> Self::S;
        fn binary_operation(&self, a: &Self::S, b: &Self::S) -> Self::S;

        /// base^n を求める
        fn pow(&self, base: &Self::S, n: usize) -> Self::S {
            let mut base = base.clone();
            let mut ans = self.identity();
            let mut n = n;

            while n > 0 {
                if n & 1 == 1 {
                    ans = self.binary_operation(&ans, &base);
                }
                base = self.binary_operation(&base, &base);
                n >>= 1;
            }
            ans
        }
    }
}

#[snippet(prefix = "use monoid_transform::*;", include = "dynamic_monoid")]
pub mod monoid_transform {
    use itertools::Itertools;

    use super::dynamic_monoid::DynamicMonoid;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Transform {
        n: usize,
    }
    impl Transform {
        pub fn new(n: usize) -> Self {
            Self { n }
        }
    }
    impl DynamicMonoid for Transform {
        type S = Vec<usize>;
        fn identity(&self) -> Self::S {
            (0..self.n).collect_vec()
        }

        fn binary_operation(&self, a: &Self::S, b: &Self::S) -> Self::S {
            (0..self.n).map(|i| a[b[i]]).collect_vec()
        }
    }
}

#[snippet(prefix = "use monoid_matrix_mul::*;", include = "dynamic_monoid")]
pub mod monoid_matrix_mul {
    use itertools::Itertools;

    use super::dynamic_monoid::DynamicMonoid;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct MatrixMul {
        size: usize,
    }
    impl MatrixMul {
        pub fn new(size: usize) -> Self {
            Self { size }
        }
    }
    impl DynamicMonoid for MatrixMul {
        type S = Vec<Vec<i64>>;
        fn identity(&self) -> Self::S {
            (0..self.size)
                .map(|y| {
                    (0..self.size)
                        .map(move |x| if x == y { 1 } else { 0 })
                        .collect_vec()
                })
                .collect_vec()
        }

        fn binary_operation(&self, a: &Self::S, b: &Self::S) -> Self::S {
            (0..self.size)
                .map(|y| {
                    (0..self.size)
                        .map(move |x| (0..self.size).map(|k| a[y][k] * b[k][x]).sum())
                        .collect_vec()
                })
                .collect_vec()
        }
    }
}

#[cfg(test)]
mod test_extend_acl_monoid {
    use ac_library::Multiplicative;

    use super::extend_acl_monoid::*;

    #[test]
    fn test_monoid_pow() {
        type M = Multiplicative<i64>;
        assert_eq!(M::pow(&3, 4), 81);
        assert_eq!(M::pow(&3, 0), 1);
    }
}
#[cfg(test)]
mod test_cum_monoid {
    use ac_library::Additive;

    use super::cum_monoid::*;

    #[test]
    fn test_cum_monoid() {
        type M = Additive<i64>;
        // 正常系
        {
            let xs = [1, 2, 3, 4, 5, 6];
            let cum = CumMonoid::<M>::new(&xs);
            assert_eq!(cum.prefix_prod(0), 0);
            assert_eq!(cum.prefix_prod(3), xs[0] + xs[1] + xs[2]);
            assert_eq!(
                cum.prefix_prod(6),
                xs[0] + xs[1] + xs[2] + xs[3] + xs[4] + xs[5]
            );
            assert_eq!(
                cum.suffix_prod(0),
                xs[0] + xs[1] + xs[2] + xs[3] + xs[4] + xs[5]
            );
            assert_eq!(cum.suffix_prod(4), xs[4] + xs[5]);
            assert_eq!(cum.suffix_prod(6), 0);

            assert_eq!(cum.prod_without1(2), xs[0] + xs[1] + xs[3] + xs[4] + xs[5]);
            assert_eq!(cum.prod_without_range(2, 4), xs[0] + xs[1] + xs[4] + xs[5]);
            assert_eq!(cum.prod_without_range(0, 6), 0);
        }

        // 空列
        {
            let xs = [];
            let cum = CumMonoid::<M>::new(&xs);
            assert_eq!(cum.prefix_prod(0), 0);
            assert_eq!(cum.suffix_prod(0), 0);
            // cum.prod_without1(0) これはエラー
            assert_eq!(cum.prod_without_range(0, 0), 0);
        }
    }

    #[test]
    fn test_cum_min() {
        // 正常系
        let max = |xs: &[i64]| xs.iter().copied().max().unwrap();
        {
            let xs = [1, 2, 3, 4, 5, 6];
            let cum = CumMax::new(&xs);
            assert_eq!(cum.prefix_max(0), i64::MIN);
            assert_eq!(cum.prefix_max(3), max(&xs[..3]));
            assert_eq!(cum.prefix_max(6), max(&xs[..6]));
            assert_eq!(cum.suffix_max(0), max(&xs[0..]));
            assert_eq!(cum.suffix_max(4), max(&xs[4..]));
            assert_eq!(cum.suffix_max(6), i64::MIN);

            assert_eq!(cum.max_without1(2), max(&[0, 1, 3, 4, 5].map(|i| xs[i])));
            assert_eq!(
                cum.max_without_range(2, 4),
                max(&[0, 1, 4, 5].map(|i| xs[i]))
            );
            assert_eq!(cum.max_without_range(0, 6), i64::MIN);
        }

        // 空列
        {
            let xs = [];
            let cum = CumMax::new(&xs);
            assert_eq!(cum.prefix_max(0), i64::MIN);
            assert_eq!(cum.suffix_max(0), i64::MIN);
            // cum.max_without1(0) これはエラー
            assert_eq!(cum.max_without_range(0, 0), i64::MIN);
        }
    }

    #[test]
    fn test_cum_max() {
        // 正常系
        let min = |xs: &[i64]| xs.iter().copied().min().unwrap();
        {
            let xs = [1, 2, 3, 4, 5, 6];
            let cum = CumMin::new(&xs);
            assert_eq!(cum.prefix_min(0), i64::MAX);
            assert_eq!(cum.prefix_min(3), min(&xs[..3]));
            assert_eq!(cum.prefix_min(6), min(&xs[..6]));
            assert_eq!(cum.suffix_min(0), min(&xs[0..]));
            assert_eq!(cum.suffix_min(4), min(&xs[4..]));
            assert_eq!(cum.suffix_min(6), i64::MAX);

            assert_eq!(cum.min_without1(2), min(&[0, 1, 3, 4, 5].map(|i| xs[i])));
            assert_eq!(
                cum.min_without_range(2, 4),
                min(&[0, 1, 4, 5].map(|i| xs[i]))
            );
            assert_eq!(cum.min_without_range(0, 6), i64::MAX);
        }

        // 空列
        {
            let xs = [];
            let cum = CumMin::new(&xs);
            assert_eq!(cum.prefix_min(0), i64::MAX);
            assert_eq!(cum.suffix_min(0), i64::MAX);
            // cum.min_without1(0) これはエラー
            assert_eq!(cum.min_without_range(0, 0), i64::MAX);
        }
    }
}
#[cfg(test)]
mod test_monoid_bitwise {
    use ac_library::Monoid;

    use super::monoid_bitwise::*;

    #[test]
    fn test_monoid_bitwise_or() {
        type M = BitwiseOr<i64>;
        assert_eq!(M::binary_operation(&0b0110, &0b0011), 0b0111);
        assert_eq!(M::binary_operation(&0b0110, &M::identity()), 0b0110);
    }
    #[test]
    fn test_monoid_bitwise_and() {
        type M = BitwiseAnd<i64>;
        assert_eq!(M::binary_operation(&0b0110, &0b0011), 0b0010);
        assert_eq!(M::binary_operation(&0b0110, &M::identity()), 0b0110);
    }
    #[test]
    fn test_monoid_bitwise_xor() {
        type M = BitwiseXor<i64>;
        assert_eq!(M::binary_operation(&0b0110, &0b0011), 0b0101);
        assert_eq!(M::binary_operation(&0b0110, &M::identity()), 0b0110);
    }
}
#[cfg(test)]
mod test_monoid_gcd_lcm {
    use ac_library::Monoid;

    use super::monoid_gcd_lcm::*;

    #[test]
    fn test_monoid_gcd() {
        type M = Gcd<i64>;
        assert_eq!(M::binary_operation(&12, &8), 4);
        assert_eq!(M::binary_operation(&12, &M::identity()), 12);
    }

    #[test]
    fn test_monoid_lcm() {
        type M = Lcm<i64>;
        assert_eq!(M::binary_operation(&12, &8), 24);
        assert_eq!(M::binary_operation(&12, &M::identity()), 12);
    }
}
#[cfg(test)]
mod test_monoid_modint {
    use ac_library::{Mod998244353, ModInt998244353, Monoid};

    use super::monoid_modint::*;

    #[test]
    fn test_monoid_mint_additive() {
        type Mint = ModInt998244353;
        type M = MintAdditive<Mod998244353>;
        assert_eq!(
            M::binary_operation(&Mint::new(3), &Mint::new(4)),
            Mint::new(7)
        );
        assert_eq!(
            M::binary_operation(&Mint::new(3), &M::identity()),
            Mint::new(3)
        );
    }

    #[test]
    fn test_monoid_multiplicative() {
        type Mint = ModInt998244353;
        type M = MintMultiplicative<Mod998244353>;
        assert_eq!(
            M::binary_operation(&Mint::new(3), &Mint::new(4)),
            Mint::new(12)
        );
        assert_eq!(
            M::binary_operation(&Mint::new(3), &M::identity()),
            Mint::new(3)
        );
    }
}
#[cfg(test)]
mod test_monoid_affine {
    use super::monoid_affine::*;
    use ac_library::{ModInt998244353, Monoid};
    #[test]
    fn test_monoid_affine() {
        type Mint = ModInt998244353;
        type M = AffineComposition<Mint>;
        let affine1: AffineTransform<Mint> = AffineTransform::new(3.into(), 5.into());
        let affine2: AffineTransform<Mint> = AffineTransform::new(5.into(), 2.into());
        // 3(5x + 2) + 5 = 15x + 11
        assert_eq!(
            M::binary_operation(&affine1, &affine2),
            AffineTransform::new(15.into(), 11.into())
        );
        assert_eq!(M::binary_operation(&affine1, &M::identity()), affine1)
    }
}

#[cfg(test)]
mod test_monoid_transform {

    use super::dynamic_monoid::DynamicMonoid;

    use super::monoid_transform::Transform;

    #[test]
    fn test_monoid_transform() {
        let transform = Transform::new(5);
        let f = vec![0, 1, 3, 2, 4];
        let g = vec![4, 3, 1, 1, 2];
        // f . g を作る
        // f[g[0]] = f[4] = 4
        // f[g[1]] = f[3] = 2
        // f[g[2]] = f[1] = 1
        // f[g[3]] = f[1] = 1
        // f[g[4]] = f[2] = 3
        assert_eq!(transform.binary_operation(&f, &g), vec![4, 2, 1, 1, 3]);
        assert_eq!(transform.binary_operation(&transform.identity(), &g), g);
        assert_eq!(transform.binary_operation(&f, &transform.identity()), f);

        assert_eq!(transform.pow(&vec![1, 2, 3, 4, 0], 0), vec![0, 1, 2, 3, 4]);
        assert_eq!(transform.pow(&vec![1, 2, 3, 4, 0], 1), vec![1, 2, 3, 4, 0]);
        assert_eq!(transform.pow(&vec![1, 2, 3, 4, 0], 2), vec![2, 3, 4, 0, 1]);
        assert_eq!(transform.pow(&vec![1, 2, 3, 4, 0], 3), vec![3, 4, 0, 1, 2]);
        assert_eq!(transform.pow(&vec![1, 2, 3, 4, 0], 4), vec![4, 0, 1, 2, 3]);
        assert_eq!(transform.pow(&vec![1, 2, 3, 4, 0], 5), vec![0, 1, 2, 3, 4]);
    }
}

#[cfg(test)]
mod test_monoid_matrix_mul {

    use super::dynamic_monoid::DynamicMonoid;

    use super::monoid_matrix_mul::MatrixMul;

    #[test]
    fn test_monoid_matrix_mul() {
        let transform = MatrixMul::new(2);
        let a = vec![vec![2, 3], vec![4, 5]];
        let b = vec![vec![6, 7], vec![8, 9]];

        assert_eq!(
            transform.binary_operation(&a, &b),
            vec![vec![36, 41], vec![64, 73]]
        );
        assert_eq!(transform.binary_operation(&transform.identity(), &a), a);
        assert_eq!(transform.binary_operation(&a, &transform.identity()), a);
    }
}
