// 問題文と制約は読みましたか？
// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        xs: [Usize1; n],
        lrs: [(Usize1, Usize1); m],
    }
    let mut xs = RangeArgminmaxSegtree::from_slice(&xs);
    for (l, r) in lrs {
        let p = xs.range_argminmax(l..=r);

        xs.set(p.argmin_left, p.max);
        xs.set(p.argmax_left, p.min);
    }

    let ans = xs.to_vec().iter().copied().map(|x| x + 1).collect_vec();
    print_vec_1line(&ans);
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
use range_argminmax_segtree::*;
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
pub mod range_argminmax_segtree {
    use super::{BoundedAbove, BoundedBelow};
    use ac_library::{Monoid, Segtree};
    use itertools::Itertools;
    use std::cmp::Ordering;
    use std::convert::Infallible;
    use std::marker::PhantomData;
    use std::ops::RangeBounds;
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeArgminmax<T> {
        pub min: T,
        pub max: T,
        pub argmin_left: usize,
        pub argmin_right: usize,
        pub argmax_left: usize,
        pub argmax_right: usize,
    }
    impl<T> RangeArgminmax<T>
    where
        T: BoundedAbove + BoundedBelow + Copy + Ord,
    {
        fn from_value(index: usize, value: T) -> Self {
            Self {
                min: value,
                max: value,
                argmin_left: index,
                argmin_right: index,
                argmax_left: index,
                argmax_right: index,
            }
        }
        fn identity() -> Self {
            Self {
                min: T::max_value(),
                max: T::min_value(),
                argmin_left: usize::MAX,
                argmin_right: usize::MIN,
                argmax_left: usize::MAX,
                argmax_right: usize::MIN,
            }
        }
        fn binary_operation(left: &Self, right: &Self) -> Self {
            let min = std::cmp::min(left.min, right.min);
            let max = std::cmp::max(left.max, right.max);
            let argmin_left = match left.min.cmp(&right.min) {
                Ordering::Less => left.argmin_left,
                Ordering::Equal => left.argmin_left.min(right.argmin_left),
                Ordering::Greater => right.argmin_left,
            };
            let argmin_right = match left.min.cmp(&right.min) {
                Ordering::Less => left.argmin_right,
                Ordering::Equal => left.argmin_right.max(right.argmin_right),
                Ordering::Greater => right.argmin_right,
            };
            let argmax_left = match left.max.cmp(&right.max) {
                Ordering::Less => right.argmax_left,
                Ordering::Equal => left.argmax_left.min(right.argmax_left),
                Ordering::Greater => left.argmax_left,
            };
            let argmax_right = match left.max.cmp(&right.max) {
                Ordering::Less => right.argmax_right,
                Ordering::Equal => left.argmax_right.max(right.argmax_right),
                Ordering::Greater => left.argmax_right,
            };
            Self {
                min,
                max,
                argmin_left,
                argmin_right,
                argmax_left,
                argmax_right,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct RangeArgminmaxMonoid<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for RangeArgminmaxMonoid<T>
    where
        T: BoundedAbove + BoundedBelow + Copy + Ord,
    {
        type S = RangeArgminmax<T>;
        fn identity() -> Self::S {
            RangeArgminmax::identity()
        }
        fn binary_operation(left: &Self::S, right: &Self::S) -> Self::S {
            RangeArgminmax::binary_operation(left, right)
        }
    }
    /// 区間の最小値・最大値と、それぞれの最左・最右添字を管理するセグメント木。
    ///
    /// 値だけを点更新し、添字は構築時の位置に固定する。
    #[derive(Clone)]
    pub struct RangeArgminmaxSegtree<T>
    where
        T: BoundedAbove + BoundedBelow + Copy + Ord,
    {
        segtree: Segtree<RangeArgminmaxMonoid<T>>,
        len: usize,
    }
    impl<T> RangeArgminmaxSegtree<T>
    where
        T: BoundedAbove + BoundedBelow + Copy + Ord,
    {
        /// 配列からセグメント木を構築する。
        pub fn from_slice(xs: &[T]) -> Self {
            let len = xs.len();
            let values = xs
                .iter()
                .copied()
                .enumerate()
                .map(|(index, value)| RangeArgminmax::from_value(index, value))
                .collect_vec();
            Self {
                segtree: Segtree::<RangeArgminmaxMonoid<T>>::from(values),
                len,
            }
        }
        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            self.len
        }
        /// p 番目の値を x に更新する。
        pub fn set(&mut self, p: usize, x: T) {
            self.segtree.set(p, RangeArgminmax::from_value(p, x));
        }
        /// p 番目の値を取得する。
        pub fn get(&self, p: usize) -> T {
            self.segtree.get(p).min
        }
        /// range の最小値・最大値と、それぞれの最左・最右添字を取得する。
        pub fn range_argminmax<R>(&self, range: R) -> RangeArgminmax<T>
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range)
        }
        /// range の最小値を取得する。
        pub fn range_min<R>(&self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).min
        }
        /// range の最大値を取得する。
        pub fn range_max<R>(&self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).max
        }
        /// range で最小値をとる最左の添字を取得する。
        ///
        /// 空区間では `usize::MAX` を返す。
        pub fn range_argmin_left<R>(&self, range: R) -> usize
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).argmin_left
        }
        /// range で最小値をとる最右の添字を取得する。
        ///
        /// 空区間では `usize::MIN` を返す。
        pub fn range_argmin_right<R>(&self, range: R) -> usize
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).argmin_right
        }
        /// range で最大値をとる最左の添字を取得する。
        ///
        /// 空区間では `usize::MAX` を返す。
        pub fn range_argmax_left<R>(&self, range: R) -> usize
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).argmax_left
        }
        /// range で最大値をとる最右の添字を取得する。
        ///
        /// 空区間では `usize::MIN` を返す。
        pub fn range_argmax_right<R>(&self, range: R) -> usize
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).argmax_right
        }
        /// 全要素の最小値を取得する。
        pub fn all_min(&self) -> T {
            self.segtree.all_prod().min
        }
        /// 全要素の最大値を取得する。
        pub fn all_max(&self) -> T {
            self.segtree.all_prod().max
        }
        /// 全要素で最小値をとる最左の添字を取得する。
        ///
        /// 空のセグメント木では `usize::MAX` を返す。
        pub fn all_argmin_left(&self) -> usize {
            self.segtree.all_prod().argmin_left
        }
        /// 全要素で最小値をとる最右の添字を取得する。
        ///
        /// 空のセグメント木では `usize::MIN` を返す。
        pub fn all_argmin_right(&self) -> usize {
            self.segtree.all_prod().argmin_right
        }
        /// 全要素で最大値をとる最左の添字を取得する。
        ///
        /// 空のセグメント木では `usize::MAX` を返す。
        pub fn all_argmax_left(&self) -> usize {
            self.segtree.all_prod().argmax_left
        }
        /// 全要素で最大値をとる最右の添字を取得する。
        ///
        /// 空のセグメント木では `usize::MIN` を返す。
        pub fn all_argmax_right(&self) -> usize {
            self.segtree.all_prod().argmax_right
        }
        /// セグメント木上の二分探索。
        ///
        /// [l, r) の最小値 min と最大値 max に対して f(&min, &max) が true となる最大の r を返す。
        ///
        /// # 前提条件
        /// * `l <= n`
        /// * 空区間の `(min, max)` に対して `f` が `true`
        /// * `f` は単調である。
        pub fn max_right<F>(&self, l: usize, f: F) -> usize
        where
            F: Fn(&T, &T) -> bool,
        {
            self.segtree.max_right(l, |value| f(&value.min, &value.max))
        }
        /// セグメント木上の二分探索。
        ///
        /// [l, r) の最小値 min と最大値 max に対して f(&min, &max) が true となる最小の l を返す。
        ///
        /// # 前提条件
        /// * `r <= n`
        /// * 空区間の `(min, max)` に対して `f` が `true`
        /// * `f` は単調である。
        pub fn min_left<F>(&self, r: usize, f: F) -> usize
        where
            F: Fn(&T, &T) -> bool,
        {
            self.segtree.min_left(r, |value| f(&value.min, &value.max))
        }
        /// p 番目の値を `min(current, x)` に更新する。
        pub fn chmin(&mut self, p: usize, x: T) {
            self.set(p, std::cmp::min(self.get(p), x));
        }
        /// p 番目の値を `max(current, x)` に更新する。
        pub fn chmax(&mut self, p: usize, x: T) {
            self.set(p, std::cmp::max(self.get(p), x));
        }
        /// 現在の値を Vec として返す。
        pub fn to_vec(&self) -> Vec<T> {
            (0..self.len).map(|index| self.get(index)).collect_vec()
        }
    }
}
