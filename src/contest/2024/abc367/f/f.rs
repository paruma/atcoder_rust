#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Query {
    xl: Usize1,
    xr: Usize1,
    yl: Usize1,
    yr: Usize1,
}

#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    nq: usize,
    xs: Vec<usize>,
    ys: Vec<usize>,
    qs: Vec<Query>,
}

fn pow(base: i64, n: i64, m: i64) -> i64 {
    let mut base = base as i128;
    let m = m as i128;
    let mut ans = 1;
    let mut n = n;

    while n > 0 {
        if n & 1 == 1 {
            ans *= base;
            ans %= m;
        }
        base = base * base;
        ans %= m;
        n >>= 1;
    }
    ans as i64
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            nq: usize,
            xs: [Usize1; n],
            ys: [Usize1; n],
            qs: [Query; nq],
        }
        Problem { n, nq, xs, ys, qs }
    }
    fn solve(&self) -> Answer {
        // 謎ハッシュ x ↦ (998244853^x + 998244353) mod 10^9 + 7 を使った
        let n: usize = self.n;
        let nq: usize = self.nq;
        let xs: &Vec<usize> = &self.xs;
        let ys: &Vec<usize> = &self.ys;
        let qs: &Vec<Query> = &self.qs;
        let m = 1_000_000_007;
        //let m = 3;

        let offset = 998244353_i64;

        let xsh = xs
            .iter()
            .copied()
            .map(|x| (pow(998244853_i64, x as i64, m) + offset) % m)
            .collect_vec();

        let ysh = ys
            .iter()
            .copied()
            .map(|x| (pow(998244853_i64, x as i64, m) + offset) % m)
            .collect_vec();

        let xshc = xsh
            .iter()
            .copied()
            .scanl(0_i64, |acc, x| (*acc + x) % m)
            .collect_vec();

        let yshc = ysh
            .iter()
            .copied()
            .scanl(0_i64, |acc, x| (*acc + x) % m)
            .collect_vec();

        let ans = qs
            .iter()
            .copied()
            .map(|q| {
                let xsum = (xshc[q.xr + 1] - xshc[q.xl] + m) % m;
                let ysum = (yshc[q.yr + 1] - yshc[q.yl] + m) % m;
                xsum == ysum && q.xr - q.xl == q.yr - q.yl
            })
            .collect_vec();

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // Zobrist Hash を使う
        let n: usize = self.n;
        let nq: usize = self.nq;
        let xs: &Vec<usize> = &self.xs;
        let ys: &Vec<usize> = &self.ys;
        let qs: &Vec<Query> = &self.qs;

        type Mint = ModInt1000000000000000003;

        use rand::{rngs::SmallRng, *};
        // let mut rng = SmallRng::from_os_rng();
        let mut rng = SmallRng::seed_from_u64(42);

        let rands = (0..n)
            .map(|_| Mint::new(rng.random_range(0..998244353)))
            .collect_vec();

        let xsh = xs.iter().copied().map(|x| rands[x]).collect_vec();
        let ysh = ys.iter().copied().map(|x| rands[x]).collect_vec();

        let xshc = xsh
            .iter()
            .copied()
            .scanl(Mint::new(0), |acc, x| *acc + x)
            .collect_vec();

        let yshc = ysh
            .iter()
            .copied()
            .scanl(Mint::new(0), |acc, x| *acc + x)
            .collect_vec();

        let ans = qs
            .iter()
            .copied()
            .map(|q| {
                let xsum = xshc[q.xr + 1] - xshc[q.xl];
                let ysum = yshc[q.yr + 1] - yshc[q.yl];
                xsum == ysum //&& q.xr - q.xl == q.yr - q.yl
            })
            .collect_vec();

        Answer { ans }
    }
    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        todo!();
        // let ans = 0;
        // Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<bool>,
}

impl Answer {
    fn print(&self) {
        for x in &self.ans {
            print_yesno(*x);
        }
    }
}

fn main() {
    Problem::read().solve2().print();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    struct WrongTestCase {
        problem: Problem,
        main_ans: Answer,
        naive_ans: Answer,
    }

    #[allow(dead_code)]
    fn check(p: &Problem) -> Option<WrongTestCase> {
        let main_ans = p.solve();
        let naive_ans = p.solve_naive();
        if main_ans != naive_ans {
            Some(WrongTestCase {
                problem: p.clone(),
                main_ans,
                naive_ans,
            })
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn make_random_problem(rng: &mut SmallRng) -> Problem {
        todo!()
        // let n = rng.random_range(1..=10);
        // let p = Problem { _a: n };
        // println!("{:?}", &p);
        // p
    }

    #[allow(unreachable_code)]
    #[test]
    fn test_with_naive() {
        let num_tests = 0;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut rng = SmallRng::seed_from_u64(42);
        // let mut rng = SmallRng::from_os_rng();
        let mut wrong_cases: Vec<WrongTestCase> = vec![];
        for _ in 0..num_tests {
            let p = make_random_problem(&mut rng);
            let result = check(&p);
            if let Some(wrong_test_case) = result {
                wrong_cases.push(wrong_test_case);
            }
            if wrong_cases.len() >= max_wrong_case {
                break;
            }
        }

        if !wrong_cases.is_empty() {
            for t in &wrong_cases {
                println!("{:?}", t.problem);
                println!("main ans : {:?}", t.main_ans);
                println!("naive ans: {:?}", t.naive_ans);
                println!();
            }
            println!("{} cases are wrong.", wrong_cases.len());
            panic!();
        }
    }
}

use ac_library::pow_mod;
// ====== import ======
#[allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet};

// ====== output func ======
#[allow(unused_imports)]
use print_vec::*;
pub mod print_vec {

    use itertools::Itertools;
    use proconio::fastout;
    #[fastout]
    pub fn print_vec<T: std::fmt::Debug>(arr: &[T]) {
        for a in arr {
            println!("{:?}", a);
        }
    }
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Debug>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{:?}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Debug>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{:?}", x)).join(" ");
            println!("{}", msg);
        }
    }
    pub fn print_bytes(bytes: &[u8]) {
        let msg = String::from_utf8(bytes.to_vec()).unwrap();
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec_bytes(vec_bytes: &[Vec<u8>]) {
        for row in vec_bytes {
            let msg = String::from_utf8(row.to_vec()).unwrap();
            println!("{}", msg);
        }
    }
}

#[allow(unused)]
fn print_yesno(ans: bool) {
    let msg = if ans { "Yes" } else { "No" };
    println!("{}", msg);
}

// ====== snippet ======

use scan_iter::*;
pub mod scan_iter {
    #[derive(Clone)]
    pub struct Scanl<I, B, F> {
        iter: I,
        state: Option<B>,
        f: F,
    }
    impl<I, B, F> Scanl<I, B, F> {
        fn new(iter: I, init: B, f: F) -> Scanl<I, B, F> {
            Scanl {
                iter,
                state: Some(init),
                f,
            }
        }
    }
    impl<I, B, F> Iterator for Scanl<I, B, F>
    where
        B: Copy,
        I: Iterator,
        F: FnMut(&mut B, I::Item) -> B,
    {
        type Item = B;
        #[inline]
        fn next(&mut self) -> Option<B> {
            let retval = self.state?;
            let a_opt = self.iter.next();
            self.state = self
                .state
                .and_then(|mut s| a_opt.map(|a| (self.f)(&mut s, a)));
            Some(retval)
        }
    }
    pub trait IteratorExtScanLeft: Iterator + Sized {
        fn scanl<B, F>(self, init: B, f: F) -> Scanl<Self, B, F>
        where
            Self: Sized,
            F: FnMut(&mut B, Self::Item) -> B,
        {
            Scanl::new(self, init, f)
        }
    }
    impl<T: Iterator> IteratorExtScanLeft for T {}
}

use modint_u64::*;
#[allow(clippy::module_inception)]
pub mod modint_u64 {
    use std::{
        convert::Infallible,
        fmt,
        hash::{Hash, Hasher},
        iter::{Product, Sum},
        marker::PhantomData,
        ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
        str::FromStr,
    };
    /// 2^61 -1
    pub type ModInt2305843009213693951 = StaticModInt<Mod2305843009213693951>;
    /// 10^18 + 3
    pub type ModInt1000000000000000003 = StaticModInt<Mod1000000000000000003>;
    pub type ModInt1000000007 = StaticModInt<Mod1000000007>;
    pub type ModInt998244353 = StaticModInt<Mod998244353>;
    /// Represents $\mathbb{Z}/m\mathbb{Z}$ where $m$ is a constant value.
    /// Corresponds to `atcoder::static_modint` in the original ACL.
    /// # Example
    /// ```
    /// use ac_library::ModInt1000000007 as Mint;
    /// use proconio::{input, source::once::OnceSource};
    /// input! {
    ///     from OnceSource::from("1000000006 2\n"),
    ///     a: Mint,
    ///     b: Mint,
    /// }
    /// println!("{}", a + b); // `1`
    /// ```
    #[derive(Copy, Clone, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct StaticModInt<M> {
        val: u64,
        phantom: PhantomData<fn() -> M>,
    }
    impl<M: Modulus> StaticModInt<M> {
        /// Returns the modulus, which is [`<M as Modulus>::VALUE`].
        /// Corresponds to `atcoder::static_modint::mod` in the original ACL.
        /// # Example
        /// ```
        /// use ac_library::ModInt1000000007 as Mint;
        /// assert_eq!(1_000_000_007, Mint::modulus());
        /// ```
        /// [`<M as Modulus>::VALUE`]: ../trait.Modulus.html#associatedconstant.VALUE
        #[inline(always)]
        pub fn modulus() -> u64 {
            M::VALUE
        }
        /// Creates a new `StaticModInt`.
        /// Takes [any primitive integer].
        /// Corresponds to the constructor of `atcoder::static_modint` in the original ACL.
        /// [any primitive integer]:  ../trait.RemEuclidU32.html
        #[inline]
        pub fn new<T: RemEuclidU64>(val: T) -> Self {
            Self::raw(val.rem_euclid_u64(M::VALUE))
        }
        /// Constructs a `StaticModInt` from a `val < Self::modulus()` without checking it.
        /// Corresponds to `atcoder::static_modint::raw` in the original ACL.
        /// # Constraints
        /// - `val` is less than `Self::modulus()`
        /// See [`ModIntBase::raw`] for more more details.
        /// [`ModIntBase::raw`]: ./trait.ModIntBase.html#tymethod.raw
        #[inline]
        pub fn raw(val: u64) -> Self {
            Self {
                val,
                phantom: PhantomData,
            }
        }
        /// Retruns the representative.
        /// Corresponds to `atcoder::static_modint::val` in the original ACL.
        #[inline]
        pub fn val(self) -> u64 {
            self.val
        }
        /// Returns `self` to the power of `n`.
        /// Corresponds to `atcoder::static_modint::pow` in the original ACL.
        #[inline]
        pub fn pow(self, n: u64) -> Self {
            <Self as ModIntBase>::pow(self, n)
        }
        /// Retruns the multiplicative inverse of `self`.
        /// Corresponds to `atcoder::static_modint::inv` in the original ACL.
        /// # Panics
        /// Panics if the multiplicative inverse does not exist.
        #[inline]
        pub fn inv(self) -> Self {
            if self.val() == 0 {
                panic!("attempt to divide by zero");
            }
            self.pow(M::VALUE - 2)
        }
    }
    /// These methods are implemented for the struct.
    /// You don't need to `use` `ModIntBase` to call methods of `StaticModInt`.
    impl<M: Modulus> ModIntBase for StaticModInt<M> {
        #[inline(always)]
        fn modulus() -> u64 {
            Self::modulus()
        }
        #[inline]
        fn raw(val: u64) -> Self {
            Self::raw(val)
        }
        #[inline]
        fn val(self) -> u64 {
            self.val()
        }
        #[inline]
        fn inv(self) -> Self {
            self.inv()
        }
    }
    /// Represents a modulus.
    /// # Example
    /// ```
    /// macro_rules! modulus {
    ///     ($($name:ident($value:expr, $is_prime:expr)),*) => {
    ///         $(
    ///             #[derive(Copy, Clone, Eq, PartialEq)]
    ///             enum $name {}
    ///             impl ac_library::modint::Modulus for $name {
    ///                 const VALUE: u32 = $value;
    ///                 const HINT_VALUE_IS_PRIME: bool = $is_prime;
    ///                 fn butterfly_cache() -> &'static ::std::thread::LocalKey<::std::cell::RefCell<::std::option::Option<ac_library::modint::ButterflyCache<Self>>>> {
    ///                     thread_local! {
    ///                         static BUTTERFLY_CACHE: ::std::cell::RefCell<::std::option::Option<ac_library::modint::ButterflyCache<$name>>> = ::std::default::Default::default();
    ///                     }
    ///                     &BUTTERFLY_CACHE
    ///                 }
    ///             }
    ///         )*
    ///     };
    /// }
    /// use ac_library::StaticModInt;
    /// modulus!(Mod101(101, true), Mod103(103, true));
    /// type Z101 = StaticModInt<Mod101>;
    /// type Z103 = StaticModInt<Mod103>;
    /// assert_eq!(Z101::new(101), Z101::new(0));
    /// assert_eq!(Z103::new(103), Z103::new(0));
    /// ```
    pub trait Modulus: 'static + Copy + Eq {
        const VALUE: u64;
        const HINT_VALUE_IS_PRIME: bool;
    }
    /// Represents $2^{61}-1 = 2305843009213693951$.
    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
    pub enum Mod2305843009213693951 {}
    impl Modulus for Mod2305843009213693951 {
        const VALUE: u64 = 2_305_843_009_213_693_951;
        const HINT_VALUE_IS_PRIME: bool = true;
    }
    /// Represents $10^{18}+3 = 1000000000000000003$.
    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
    pub enum Mod1000000000000000003 {}
    impl Modulus for Mod1000000000000000003 {
        const VALUE: u64 = 1_000_000_000_000_000_003;
        const HINT_VALUE_IS_PRIME: bool = true;
    }
    /// Represents $1000000007$.
    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
    pub enum Mod1000000007 {}
    impl Modulus for Mod1000000007 {
        const VALUE: u64 = 1_000_000_007;
        const HINT_VALUE_IS_PRIME: bool = true;
    }
    /// Represents $998244353$.
    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
    pub enum Mod998244353 {}
    impl Modulus for Mod998244353 {
        const VALUE: u64 = 998_244_353;
        const HINT_VALUE_IS_PRIME: bool = true;
    }
    /// A trait for [`StaticModInt`] and [`DynamicModInt`].
    /// Corresponds to `atcoder::internal::modint_base` in the original ACL.
    /// [`StaticModInt`]: ../struct.StaticModInt.html
    /// [`DynamicModInt`]: ../struct.DynamicModInt.html
    pub trait ModIntBase:
        Default
        + FromStr
        + From<i8>
        + From<i16>
        + From<i32>
        + From<i64>
        + From<i128>
        + From<isize>
        + From<u8>
        + From<u16>
        + From<u32>
        + From<u64>
        + From<u128>
        + From<usize>
        + Copy
        + Eq
        + Hash
        + fmt::Display
        + fmt::Debug
        + Neg<Output = Self>
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
    {
        /// Returns the modulus.
        /// Corresponds to `atcoder::static_modint::mod` and `atcoder::dynamic_modint::mod` in the original ACL.
        /// # Example
        /// ```
        /// use ac_library::modint::ModIntBase;
        /// fn f<Z: ModIntBase>() {
        ///     let _: u32 = Z::modulus();
        /// }
        /// ```
        fn modulus() -> u64;
        /// Constructs a `Self` from a `val < Self::modulus()` without checking it.
        /// Corresponds to `atcoder::static_modint::raw` and `atcoder::dynamic_modint::raw` in the original ACL.
        /// # Constraints
        /// - `val` is less than `Self::modulus()`
        /// **Note that all operations assume that inner values are smaller than the modulus.**
        /// If `val` is greater than or equal to `Self::modulus()`, the behaviors are not defined.
        /// ```should_panic
        /// use ac_library::ModInt1000000007 as Mint;
        /// let x = Mint::raw(1_000_000_007);
        /// let y = x + x;
        /// assert_eq!(0, y.val());
        /// ```
        /// ```text
        /// thread 'main' panicked at 'assertion failed: `(left == right)`
        ///   left: `0`,
        ///  right: `1000000007`', src/modint.rs:8:1
        /// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        /// ```
        /// # Example
        /// ```
        /// use ac_library::modint::ModIntBase;
        /// fn f<Z: ModIntBase>() -> Z {
        ///     debug_assert!(Z::modulus() >= 100);
        ///     let mut acc = Z::new(0);
        ///     for i in 0..100 {
        ///         if i % 3 == 0 {
        ///             // I know `i` is smaller than the modulus!
        ///             acc += Z::raw(i);
        ///         }
        ///     }
        ///     acc
        /// }
        /// ```
        fn raw(val: u64) -> Self;
        /// Retruns the representative.
        /// Corresponds to `atcoder::static_modint::val` and `atcoder::dynamic_modint::val` in the original ACL.
        /// # Example
        /// ```
        /// use ac_library::modint::ModIntBase;
        /// fn f<Z: ModIntBase>(x: Z) {
        ///     let _: u32 = x.val();
        /// }
        /// ```
        fn val(self) -> u64;
        /// Retruns the multiplicative inverse of `self`.
        /// Corresponds to `atcoder::static_modint::inv` and `atcoder::dynamic_modint::inv` in the original ACL.
        /// # Panics
        /// Panics if the multiplicative inverse does not exist.
        /// # Example
        /// ```
        /// use ac_library::modint::ModIntBase;
        /// fn f<Z: ModIntBase>(x: Z) {
        ///     let _: Z = x.inv();
        /// }
        /// ```
        fn inv(self) -> Self;
        /// Creates a new `Self`.
        /// Takes [any primitive integer].
        /// # Example
        /// ```
        /// use ac_library::modint::ModIntBase;
        /// fn f<Z: ModIntBase>() {
        ///     let _ = Z::new(1u32);
        ///     let _ = Z::new(1usize);
        ///     let _ = Z::new(-1i64);
        /// }
        /// ```
        /// [any primitive integer]:  ../trait.RemEuclidU32.html
        #[inline]
        fn new<T: RemEuclidU64>(val: T) -> Self {
            Self::raw(val.rem_euclid_u64(Self::modulus()))
        }
        /// Returns `self` to the power of `n`.
        /// Corresponds to `atcoder::static_modint::pow` and `atcoder::dynamic_modint::pow` in the original ACL.
        /// # Example
        /// ```
        /// use ac_library::modint::ModIntBase;
        /// fn f<Z: ModIntBase>() {
        ///     let _: Z = Z::new(2).pow(3);
        /// }
        /// ```
        #[inline]
        fn pow(self, mut n: u64) -> Self {
            let mut x = self;
            let mut r = Self::raw(1);
            while n > 0 {
                if n & 1 == 1 {
                    r *= x;
                }
                x *= x;
                n >>= 1;
            }
            r
        }
    }
    /// A trait for `{StaticModInt, DynamicModInt, ModIntBase}::new`.
    pub trait RemEuclidU64 {
        /// Calculates `self` $\bmod$ `modulus` losslessly.
        fn rem_euclid_u64(self, modulus: u64) -> u64;
    }
    macro_rules ! impl_rem_euclid_u64_for_small_signed {($ ($ ty : tt ) ,* ) => {$ (impl RemEuclidU64 for $ ty {# [inline ] fn rem_euclid_u64 (self , modulus : u64 ) -> u64 {(self as i128 ) . rem_euclid (i128 :: from (modulus ) ) as _ } } ) * } }
    impl_rem_euclid_u64_for_small_signed!(i8, i16, i32, i64, isize);
    impl RemEuclidU64 for i128 {
        #[inline]
        fn rem_euclid_u64(self, modulus: u64) -> u64 {
            self.rem_euclid(i128::from(modulus)) as _
        }
    }
    macro_rules ! impl_rem_euclid_u64_for_small_unsigned {($ ($ ty : tt ) ,* ) => {$ (impl RemEuclidU64 for $ ty {# [inline ] fn rem_euclid_u64 (self , modulus : u64 ) -> u64 {self as u64 % modulus } } ) * } }
    macro_rules ! impl_rem_euclid_u64_for_large_unsigned {($ ($ ty : tt ) ,* ) => {$ (impl RemEuclidU64 for $ ty {# [inline ] fn rem_euclid_u64 (self , modulus : u64 ) -> u64 {(self % (modulus as $ ty ) ) as _ } } ) * } }
    impl_rem_euclid_u64_for_small_unsigned!(u8, u16, u32, u64, usize);
    impl_rem_euclid_u64_for_large_unsigned!(u128);
    trait InternalImplementations: ModIntBase {
        #[inline]
        fn default_impl() -> Self {
            Self::raw(0)
        }
        #[inline]
        fn from_str_impl(s: &str) -> Result<Self, Infallible> {
            Ok(s.parse::<i64>()
                .map(Self::new)
                .unwrap_or_else(|_| todo!("parsing as an arbitrary precision integer?")))
        }
        #[inline]
        fn hash_impl(this: &Self, state: &mut impl Hasher) {
            this.val().hash(state)
        }
        #[inline]
        fn display_impl(this: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(&this.val(), f)
        }
        #[inline]
        fn debug_impl(this: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&this.val(), f)
        }
        #[inline]
        fn neg_impl(this: Self) -> Self {
            Self::sub_impl(Self::raw(0), this)
        }
        #[inline]
        fn add_impl(lhs: Self, rhs: Self) -> Self {
            let modulus = Self::modulus();
            let mut val = lhs.val() + rhs.val();
            if val >= modulus {
                val -= modulus;
            }
            Self::raw(val)
        }
        #[inline]
        fn sub_impl(lhs: Self, rhs: Self) -> Self {
            let modulus = Self::modulus();
            let mut val = lhs.val().wrapping_sub(rhs.val());
            if val >= modulus {
                val = val.wrapping_add(modulus)
            }
            Self::raw(val)
        }
        fn mul_impl(lhs: Self, rhs: Self) -> Self;
        #[inline]
        fn div_impl(lhs: Self, rhs: Self) -> Self {
            Self::mul_impl(lhs, rhs.inv())
        }
    }
    impl<M: Modulus> InternalImplementations for StaticModInt<M> {
        #[inline]
        fn mul_impl(lhs: Self, rhs: Self) -> Self {
            Self::raw((u128::from(lhs.val()) * u128::from(rhs.val()) % u128::from(M::VALUE)) as u64)
        }
    }
    macro_rules ! impl_basic_traits {() => {} ; (impl <$ generic_param : ident : $ generic_param_bound : tt > _ for $ self : ty ; $ ($ rest : tt ) * ) => {impl <$ generic_param : $ generic_param_bound > Default for $ self {# [inline ] fn default () -> Self {Self :: default_impl () } } impl <$ generic_param : $ generic_param_bound > FromStr for $ self {type Err = Infallible ; # [inline ] fn from_str (s : & str ) -> Result < Self , Infallible > {Self :: from_str_impl (s ) } } impl <$ generic_param : $ generic_param_bound , V : RemEuclidU64 > From < V > for $ self {# [inline ] fn from (from : V ) -> Self {Self :: new (from ) } } # [allow (clippy :: derive_hash_xor_eq ) ] impl <$ generic_param : $ generic_param_bound > Hash for $ self {# [inline ] fn hash < H : Hasher > (& self , state : & mut H ) {Self :: hash_impl (self , state ) } } impl <$ generic_param : $ generic_param_bound > fmt :: Display for $ self {# [inline ] fn fmt (& self , f : & mut fmt :: Formatter <'_ > ) -> fmt :: Result {Self :: display_impl (self , f ) } } impl <$ generic_param : $ generic_param_bound > fmt :: Debug for $ self {# [inline ] fn fmt (& self , f : & mut fmt :: Formatter <'_ > ) -> fmt :: Result {Self :: debug_impl (self , f ) } } impl <$ generic_param : $ generic_param_bound > Neg for $ self {type Output = $ self ; # [inline ] fn neg (self ) -> $ self {Self :: neg_impl (self ) } } impl <$ generic_param : $ generic_param_bound > Neg for &'_ $ self {type Output = $ self ; # [inline ] fn neg (self ) -> $ self {<$ self >:: neg_impl (* self ) } } impl_basic_traits ! ($ ($ rest ) * ) ; } ; }
    impl_basic_traits! {impl < M : Modulus > _ for StaticModInt < M > ; }
    macro_rules ! impl_bin_ops {() => {} ; (for <$ ($ generic_param : ident : $ generic_param_bound : tt ) ,*> <$ lhs_ty : ty > ~ <$ rhs_ty : ty > -> $ output : ty {{$ lhs_body : expr } ~ {$ rhs_body : expr } } $ ($ rest : tt ) * ) => {impl <$ ($ generic_param : $ generic_param_bound ) ,*> Add <$ rhs_ty > for $ lhs_ty {type Output = $ output ; # [inline ] fn add (self , rhs : $ rhs_ty ) -> $ output {<$ output >:: add_impl (apply ($ lhs_body , self ) , apply ($ rhs_body , rhs ) ) } } impl <$ ($ generic_param : $ generic_param_bound ) ,*> Sub <$ rhs_ty > for $ lhs_ty {type Output = $ output ; # [inline ] fn sub (self , rhs : $ rhs_ty ) -> $ output {<$ output >:: sub_impl (apply ($ lhs_body , self ) , apply ($ rhs_body , rhs ) ) } } impl <$ ($ generic_param : $ generic_param_bound ) ,*> Mul <$ rhs_ty > for $ lhs_ty {type Output = $ output ; # [inline ] fn mul (self , rhs : $ rhs_ty ) -> $ output {<$ output >:: mul_impl (apply ($ lhs_body , self ) , apply ($ rhs_body , rhs ) ) } } impl <$ ($ generic_param : $ generic_param_bound ) ,*> Div <$ rhs_ty > for $ lhs_ty {type Output = $ output ; # [inline ] fn div (self , rhs : $ rhs_ty ) -> $ output {<$ output >:: div_impl (apply ($ lhs_body , self ) , apply ($ rhs_body , rhs ) ) } } impl_bin_ops ! ($ ($ rest ) * ) ; } ; }
    macro_rules ! impl_assign_ops {() => {} ; (for <$ ($ generic_param : ident : $ generic_param_bound : tt ) ,*> <$ lhs_ty : ty > ~= <$ rhs_ty : ty > {_ ~= {$ rhs_body : expr } } $ ($ rest : tt ) * ) => {impl <$ ($ generic_param : $ generic_param_bound ) ,*> AddAssign <$ rhs_ty > for $ lhs_ty {# [inline ] fn add_assign (& mut self , rhs : $ rhs_ty ) {* self = * self + apply ($ rhs_body , rhs ) ; } } impl <$ ($ generic_param : $ generic_param_bound ) ,*> SubAssign <$ rhs_ty > for $ lhs_ty {# [inline ] fn sub_assign (& mut self , rhs : $ rhs_ty ) {* self = * self - apply ($ rhs_body , rhs ) ; } } impl <$ ($ generic_param : $ generic_param_bound ) ,*> MulAssign <$ rhs_ty > for $ lhs_ty {# [inline ] fn mul_assign (& mut self , rhs : $ rhs_ty ) {* self = * self * apply ($ rhs_body , rhs ) ; } } impl <$ ($ generic_param : $ generic_param_bound ) ,*> DivAssign <$ rhs_ty > for $ lhs_ty {# [inline ] fn div_assign (& mut self , rhs : $ rhs_ty ) {* self = * self / apply ($ rhs_body , rhs ) ; } } impl_assign_ops ! ($ ($ rest ) * ) ; } ; }
    #[inline]
    fn apply<F: FnOnce(X) -> O, X, O>(f: F, x: X) -> O {
        f(x)
    }
    impl_bin_ops! {for < M : Modulus > < StaticModInt < M > > ~ < StaticModInt < M > > -> StaticModInt < M > {{| x | x } ~ {| x | x } } for < M : Modulus > < StaticModInt < M > > ~ <&'_ StaticModInt < M > > -> StaticModInt < M > {{| x | x } ~ {|& x | x } } for < M : Modulus > <&'_ StaticModInt < M > > ~ < StaticModInt < M > > -> StaticModInt < M > {{|& x | x } ~ {| x | x } } for < M : Modulus > <&'_ StaticModInt < M > > ~ <&'_ StaticModInt < M > > -> StaticModInt < M > {{|& x | x } ~ {|& x | x } } for < M : Modulus , T : RemEuclidU64 > < StaticModInt < M > > ~ < T > -> StaticModInt < M > {{| x | x } ~ {StaticModInt ::< M >:: new } } }
    impl_assign_ops! {for < M : Modulus > < StaticModInt < M > > ~= < StaticModInt < M > > {_ ~= {| x | x } } for < M : Modulus > < StaticModInt < M > > ~= <&'_ StaticModInt < M > > {_ ~= {|& x | x } } for < M : Modulus , T : RemEuclidU64 > < StaticModInt < M > > ~= < T > {_ ~= {StaticModInt ::< M >:: new } } }
    macro_rules ! impl_folding {() => {} ; (impl <$ generic_param : ident : $ generic_param_bound : tt > $ trait : ident < _ > for $ self : ty {fn $ method : ident (_ ) -> _ {_ ($ unit : expr , $ op : expr ) } } $ ($ rest : tt ) * ) => {impl <$ generic_param : $ generic_param_bound > $ trait < Self > for $ self {# [inline ] fn $ method < S > (iter : S ) -> Self where S : Iterator < Item = Self >, {iter . fold ($ unit , $ op ) } } impl <'a , $ generic_param : $ generic_param_bound > $ trait <&'a Self > for $ self {# [inline ] fn $ method < S > (iter : S ) -> Self where S : Iterator < Item = &'a Self >, {iter . fold ($ unit , $ op ) } } impl_folding ! ($ ($ rest ) * ) ; } ; }
    impl_folding! {impl < M : Modulus > Sum < _ > for StaticModInt < M > {fn sum (_ ) -> _ {_ (Self :: raw (0 ) , Add :: add ) } } impl < M : Modulus > Product < _ > for StaticModInt < M > {fn product (_ ) -> _ {_ (Self :: raw (1 ) , Mul :: mul ) } } }
}
