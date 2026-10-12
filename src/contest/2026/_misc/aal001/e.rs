use ac_library::Segtree;
// 問題文と制約は読みましたか？
// #[fastout]

use monoid_template::*;
#[allow(unused_variables)]
pub mod monoid_template {
    use ac_library::ModInt998244353 as Mint;

    use ac_library::segtree::Monoid;
    use std::convert::Infallible;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct RangeXxx {
        pub r: Mint,
        pub g: Mint,
        pub b: Mint,
        pub rg: Mint,
        pub gb: Mint,
        pub rgb: Mint,
    }
    impl RangeXxx {
        pub fn unit(x: i64, ch: char) -> Self {
            Self {
                r: Mint::new((ch == 'R') as usize) * x,
                g: Mint::new((ch == 'G') as usize) * x,
                b: Mint::new((ch == 'B') as usize) * x,
                rg: Mint::new(0),
                gb: Mint::new(0),
                rgb: Mint::new(0),
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeXxxMonoid(Infallible);
    impl Monoid for RangeXxxMonoid {
        type S = RangeXxx;
        fn identity() -> Self::S {
            RangeXxx {
                r: Mint::new(0),
                g: Mint::new(0),
                b: Mint::new(0),
                rg: Mint::new(0),
                gb: Mint::new(0),
                rgb: Mint::new(0),
            }
        }
        fn binary_operation(x: &Self::S, y: &Self::S) -> Self::S {
            let r = x.r + y.r;
            let g = x.g + y.g;
            let b = x.b + y.b;
            let rg = x.rg + y.rg + x.r * y.g;
            let gb = x.gb + y.gb + x.g * y.b;
            let rgb = x.rgb + y.rgb + x.rg * y.b + x.r * y.gb;
            RangeXxx {
                r,
                g,
                b,
                rg,
                gb,
                rgb,
            }
        }
    }
}
fn main() {
    input! {
        n: usize,
        q: usize,
        xs: [i64; n],
        ss: Chars,
    }

    let mut s =
        Segtree::<RangeXxxMonoid>::from((0..n).map(|i| RangeXxx::unit(xs[i], ss[i])).collect_vec());

    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                i: Usize1,
                x: i64
            }
            s.set(i, RangeXxx::unit(x, ss[i]));
        }
        if t == 2 {
            input! {
                l: Usize1,
                r: Usize1
            }
            let ans = s.prod(l..=r).rgb;
            println!("{}", ans);
        }
    }
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

    /// 間違っていたら false を返す
    fn process_one_test(rng: &mut SmallRng) -> bool {
        // ==== 問題を作る ====
        let n = rng.random_range(1..=10);
        let xs = (0..n).map(|_| rng.random_range(0..10)).collect_vec();

        // ==== 解く ====
        let main_ans = xs.len();
        let naive_ans = 1;

        // ==== 間違っていたら報告をする ====
        if main_ans != naive_ans {
            // 問題を出力
            println!("{:?}", (n, xs));
            println!("main ans : {:?}", main_ans);
            println!("naive ans: {:?}", naive_ans);
            return false;
        }
        true
    }

    #[allow(unreachable_code)]
    #[test]
    #[ignore]
    fn test_with_naive() {
        let num_tests = 100;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut cnt_wrong = 0;
        let mut rng = SmallRng::seed_from_u64(42);
        // let mut rng = SmallRng::from_os_rng();
        for _ in 0..num_tests {
            let is_ok = process_one_test(&mut rng);
            if !is_ok {
                cnt_wrong += 1;
            }
            if cnt_wrong >= max_wrong_case {
                break;
            }
        }
        if cnt_wrong > 0 {
            println!("{} cases are wrong.", cnt_wrong);
            panic!();
        }
    }
}

// ====== import ======
#[allow(unused_imports)]
use {
    itertools::{Itertools, chain, iproduct, izip},
    proconio::{
        derive_readable, fastout, input,
        marker::{Bytes, Chars, Usize1},
    },
    rand::{Rng, SeedableRng, rngs::SmallRng, seq::SliceRandom},
    std::{
        cmp::Reverse,
        collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet},
    },
};

// ====== output func ======
#[allow(unused_imports)]
use print_util::*;
pub mod print_util {
    use itertools::Itertools;
    use proconio::fastout;

    #[fastout]
    pub fn print_vec<T: std::fmt::Display>(arr: &[T]) {
        for a in arr {
            println!("{}", a);
        }
    }

    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
        println!("{}", arr.iter().join(" "));
    }

    #[fastout]
    pub fn print_vec2<T: std::fmt::Display, R: AsRef<[T]>>(arr: &[R]) {
        for row in arr {
            println!("{}", row.as_ref().iter().join(" "));
        }
    }

    pub fn print_bytes(bytes: &[u8]) {
        println!("{}", std::str::from_utf8(bytes).unwrap());
    }

    pub fn print_chars(chars: &[char]) {
        println!("{}", chars.iter().collect::<String>());
    }

    #[fastout]
    pub fn print_vec_bytes<R: AsRef<[u8]>>(vec_bytes: &[R]) {
        for row in vec_bytes {
            println!("{}", std::str::from_utf8(row.as_ref()).unwrap());
        }
    }

    #[fastout]
    pub fn print_vec_chars<R: AsRef<[char]>>(vec_chars: &[R]) {
        for row in vec_chars {
            println!("{}", row.as_ref().iter().collect::<String>());
        }
    }

    pub fn print_yesno(ans: bool) {
        println!("{}", if ans { "Yes" } else { "No" });
    }
}

// ====== snippet ======
use min_max_monoid::*;
use range_min_segtree::*;
#[allow(clippy::module_inception)]
pub mod min_max_monoid {
    use ac_library::Monoid;
    use std::convert::Infallible;
    use std::marker::PhantomData;
    pub trait BoundedBelow {
        fn min_value() -> Self;
    }
    pub trait BoundedAbove {
        fn max_value() -> Self;
    }
    macro_rules ! impl_bounded {($ ($ ty : ty ) ,* ) => {$ (impl BoundedBelow for $ ty {# [inline ] fn min_value () -> Self {Self :: MIN } } impl BoundedAbove for $ ty {# [inline ] fn max_value () -> Self {Self :: MAX } } ) * } ; }
    impl_bounded!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
    );
    impl<T: BoundedAbove> BoundedBelow for std::cmp::Reverse<T> {
        #[inline]
        fn min_value() -> Self {
            std::cmp::Reverse(T::max_value())
        }
    }
    impl<T: BoundedBelow> BoundedAbove for std::cmp::Reverse<T> {
        #[inline]
        fn max_value() -> Self {
            std::cmp::Reverse(T::min_value())
        }
    }
    macro_rules ! impl_bounded_tuples {($ head : ident ) => {} ; ($ head : ident , $ ($ tail : ident ) ,* ) => {impl <$ head , $ ($ tail ) ,*> BoundedBelow for ($ head , $ ($ tail ) ,* ) where $ head : BoundedBelow , $ ($ tail : BoundedBelow ) ,* {# [inline ] fn min_value () -> Self {($ head :: min_value () , $ ($ tail :: min_value () ) ,* ) } } impl <$ head , $ ($ tail ) ,*> BoundedAbove for ($ head , $ ($ tail ) ,* ) where $ head : BoundedAbove , $ ($ tail : BoundedAbove ) ,* {# [inline ] fn max_value () -> Self {($ head :: max_value () , $ ($ tail :: max_value () ) ,* ) } } impl_bounded_tuples ! ($ ($ tail ) ,* ) ; } ; () => {} ; }
    impl_bounded_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    /// 辞書式順序で最小の要素を管理するモノイド (単位元は最大値)
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct MinMonoid<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for MinMonoid<T>
    where
        T: BoundedAbove + Ord + Clone,
    {
        type S = T;
        fn identity() -> Self::S {
            T::max_value()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            std::cmp::min(a, b).clone()
        }
    }
    /// 辞書式順序で最大の要素を管理するモノイド (単位元は最小値)
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct MaxMonoid<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for MaxMonoid<T>
    where
        T: BoundedBelow + Ord + Clone,
    {
        type S = T;
        fn identity() -> Self::S {
            T::min_value()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            std::cmp::max(a, b).clone()
        }
    }
}
#[allow(clippy::module_inception)]
pub mod range_min_segtree {
    use super::MinMonoid;
    use ac_library::{Monoid, Segtree};
    use itertools::Itertools;
    use std::ops::RangeBounds;
    /// ACL の Segtree を使用した区間最小セグメント木。
    ///
    /// 数値型 T に対して点更新・区間最小取得を行う。
    #[derive(Clone)]
    pub struct RangeMinSegtree<T>
    where
        MinMonoid<T>: Monoid<S = T>,
        T: Clone,
    {
        segtree: Segtree<MinMonoid<T>>,
        len: usize,
    }
    impl<T> RangeMinSegtree<T>
    where
        MinMonoid<T>: Monoid<S = T>,
        T: Copy + Ord,
    {
        /// 単位元で初期化されたセグメント木を構築する
        pub fn new(n: usize) -> Self {
            Self {
                segtree: Segtree::<MinMonoid<T>>::new(n),
                len: n,
            }
        }
        /// 配列からセグメント木を構築する
        pub fn from_slice(xs: &[T]) -> Self {
            let len = xs.len();
            Self {
                segtree: Segtree::<MinMonoid<T>>::from(xs.to_vec()),
                len,
            }
        }
        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            self.len
        }
        /// p 番目の要素を x に更新する
        pub fn set(&mut self, p: usize, x: T) {
            self.segtree.set(p, x);
        }
        /// p 番目の要素を取得する
        pub fn get(&self, p: usize) -> T {
            self.segtree.get(p)
        }
        /// range の最小値を取得する
        pub fn range_min<R>(&self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range)
        }
        /// 全要素の最小値を取得する
        pub fn all_min(&self) -> T {
            self.segtree.all_prod()
        }
        /// セグメント木上の二分探索。
        ///
        /// [l, r) の最小値 s について f(&s) が true となる最大の r を返す。
        ///
        /// # 前提条件
        /// * `l <= n`
        /// * 最小値の単位元に対して `f` が `true`
        /// * `f` は単調である。
        ///     * `f(min(l..i))` が `true` => `f(min(l..j))` が `true` for all `l <= j <= i`.
        pub fn max_right<F>(&self, l: usize, f: F) -> usize
        where
            F: Fn(&T) -> bool,
        {
            self.segtree.max_right(l, f)
        }
        /// セグメント木上の二分探索。
        ///
        /// [l, r) の最小値 s について f(&s) が true となる最小の l を返す。
        ///
        /// # 前提条件
        /// * `r <= n`
        /// * 最小値の単位元に対して `f` が `true`
        /// * `f` は単調である。
        ///     * `f(min(i..r))` が `true` => `f(min(j..r))` が `true` for all `i <= j <= r`.
        pub fn min_left<F>(&self, r: usize, f: F) -> usize
        where
            F: Fn(&T) -> bool,
        {
            self.segtree.min_left(r, f)
        }
        /// p 番目の要素を min(current, x) に更新する
        pub fn chmin(&mut self, p: usize, x: T) {
            let current = self.get(p);
            self.set(p, std::cmp::min(current, x));
        }
        /// 現在の状態を Vec として返す
        pub fn to_vec(&self) -> Vec<T> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}
