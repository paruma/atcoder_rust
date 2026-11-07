// 問題文と制約は読みましたか？
// #[fastout]
fn main() {
    input! {
        n: usize,
        xs: [i64; n],
    }

    let psum = prefix_sum(&xs);

    let mut seg = RangeAddRangeArgminmaxSegtree::from_slice(&psum);

    let mut ans = seg.all_max() - seg.all_min() + 1;

    for i in 0..n {
        seg.range_add(i + 1.., -2 * xs[i]);

        let sub = seg.all_max() - seg.all_min() + 1;
        ans = ans.max(sub);

        seg.range_add(i + 1.., 2 * xs[i]);
    }

    println!("{}", ans);
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
// 使用したテンプレート・スニペットの公開元:
// https://github.com/paruma/atcoder_rust
use min_max_monoid::*;
use range_add_range_argminmax::*;
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

    macro_rules! impl_bounded {
        ($($ty:ty),*) => {
            $(
                impl BoundedBelow for $ty {
                    #[inline]
                    fn min_value() -> Self {
                        Self::MIN
                    }
                }

                impl BoundedAbove for $ty {
                    #[inline]
                    fn max_value() -> Self {
                        Self::MAX
                    }
                }
            )*
        };
    }

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

    macro_rules! impl_bounded_tuples {
        ($head:ident) => {};
        ($head:ident, $($tail:ident),*) => {
            impl<$head, $($tail),*> BoundedBelow for ($head, $($tail),*)
            where
                $head: BoundedBelow,
                $($tail: BoundedBelow),*
            {
                #[inline]
                fn min_value() -> Self {
                    ($head::min_value(), $($tail::min_value()),*)
                }
            }

            impl<$head, $($tail),*> BoundedAbove for ($head, $($tail),*)
            where
                $head: BoundedAbove,
                $($tail: BoundedAbove),*
            {
                #[inline]
                fn max_value() -> Self {
                    ($head::max_value(), $($tail::max_value()),*)
                }
            }

            impl_bounded_tuples!($($tail),*);
        };
        () => {};
    }

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
pub mod range_add_range_argminmax {
    use super::{BoundedAbove, BoundedBelow};
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use std::cmp::Ordering;
    use std::convert::Infallible;
    use std::iter::Sum;
    use std::marker::PhantomData;
    use std::ops::{Add, RangeBounds};

    fn zero<T: Sum>() -> T {
        std::iter::empty::<T>().sum()
    }

    #[derive(Clone, Copy, Debug)]
    struct RangeArgminmax<T> {
        min: T,
        max: T,
        argmin_left: usize,
        argmin_right: usize,
        argmax_left: usize,
        argmax_right: usize,
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
            let min = left.min.min(right.min);
            let max = left.max.max(right.max);
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
    struct AddAction<T> {
        add_value: T,
    }

    impl<T: Copy> AddAction<T> {
        fn new(add_value: T) -> Self {
            Self { add_value }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct RangeAddRangeArgminmax<T>(Infallible, PhantomData<fn() -> T>);

    impl<T> MapMonoid for RangeAddRangeArgminmax<T>
    where
        T: BoundedAbove + BoundedBelow + Copy + Ord + Add<Output = T> + Sum,
    {
        type M = RangeArgminmaxMonoid<T>;
        type F = AddAction<T>;

        fn identity_map() -> Self::F {
            AddAction::new(zero())
        }

        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            AddAction::new(g.add_value + f.add_value)
        }

        fn mapping(f: &Self::F, value: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
            if value.argmin_left == usize::MAX {
                *value
            } else {
                RangeArgminmax {
                    min: value.min + f.add_value,
                    max: value.max + f.add_value,
                    ..*value
                }
            }
        }
    }

    /// 区間加算と、区間の最小値・最大値およびそれぞれの最左・最右添字を管理する遅延セグメント木。
    #[derive(Clone)]
    pub struct RangeAddRangeArgminmaxSegtree<T>
    where
        T: BoundedAbove + BoundedBelow + Copy + Ord + Add<Output = T> + Sum,
    {
        segtree: LazySegtree<RangeAddRangeArgminmax<T>>,
        len: usize,
    }

    impl<T> RangeAddRangeArgminmaxSegtree<T>
    where
        T: BoundedAbove + BoundedBelow + Copy + Ord + Add<Output = T> + Sum,
    {
        /// 配列から遅延セグメント木を構築する。
        pub fn from_slice(xs: &[T]) -> Self {
            let len = xs.len();
            let values = xs
                .iter()
                .copied()
                .enumerate()
                .map(|(index, value)| RangeArgminmax::from_value(index, value))
                .collect_vec();
            Self {
                segtree: LazySegtree::from(values),
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
        pub fn get(&mut self, p: usize) -> T {
            self.segtree.get(p).min
        }

        /// range の最小値を取得する。
        pub fn range_min<R: RangeBounds<usize>>(&mut self, range: R) -> T {
            self.segtree.prod(range).min
        }

        /// range の最大値を取得する。
        pub fn range_max<R: RangeBounds<usize>>(&mut self, range: R) -> T {
            self.segtree.prod(range).max
        }

        /// range で最小値をとる最左の添字を取得する。
        ///
        /// 空区間では `usize::MAX` を返す。
        pub fn range_argmin_left<R: RangeBounds<usize>>(&mut self, range: R) -> usize {
            self.segtree.prod(range).argmin_left
        }

        /// range で最小値をとる最右の添字を取得する。
        ///
        /// 空区間では `usize::MIN` を返す。
        pub fn range_argmin_right<R: RangeBounds<usize>>(&mut self, range: R) -> usize {
            self.segtree.prod(range).argmin_right
        }

        /// range で最大値をとる最左の添字を取得する。
        ///
        /// 空区間では `usize::MAX` を返す。
        pub fn range_argmax_left<R: RangeBounds<usize>>(&mut self, range: R) -> usize {
            self.segtree.prod(range).argmax_left
        }

        /// range で最大値をとる最右の添字を取得する。
        ///
        /// 空区間では `usize::MIN` を返す。
        pub fn range_argmax_right<R: RangeBounds<usize>>(&mut self, range: R) -> usize {
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

        /// range の各要素に x を加算する。
        pub fn range_add<R: RangeBounds<usize>>(&mut self, range: R, x: T) {
            self.segtree.apply_range(range, AddAction::new(x));
        }

        /// セグメント木上の二分探索。
        ///
        /// [l, r) の最小値 min と最大値 max に対して f(&min, &max) が true となる最大の r を返す。
        ///
        /// # 前提条件
        /// * `l <= n`
        /// * 空区間の `(min, max)` に対して `f` が `true`
        /// * `f` は単調である。
        pub fn max_right<F>(&mut self, l: usize, f: F) -> usize
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
        pub fn min_left<F>(&mut self, r: usize, f: F) -> usize
        where
            F: Fn(&T, &T) -> bool,
        {
            self.segtree.min_left(r, |value| f(&value.min, &value.max))
        }

        /// p 番目の値を `min(current, x)` に更新する。
        pub fn chmin(&mut self, p: usize, x: T) {
            let current = self.get(p);
            self.set(p, current.min(x));
        }

        /// p 番目の値を `max(current, x)` に更新する。
        pub fn chmax(&mut self, p: usize, x: T) {
            let current = self.get(p);
            self.set(p, current.max(x));
        }

        /// 現在の値を Vec として返す。
        pub fn to_vec(&mut self) -> Vec<T> {
            (0..self.len).map(|index| self.get(index)).collect_vec()
        }
    }
}
use cumsum::*;
#[allow(clippy::module_inception)]
pub mod cumsum {
    pub fn prefix_sum(xs: &[i64]) -> Vec<i64> {
        let mut prefix_sum = vec![0; xs.len() + 1];
        for i in 1..xs.len() + 1 {
            prefix_sum[i] = prefix_sum[i - 1] + xs[i - 1];
        }
        prefix_sum
    }

    use std::ops::{Bound, Range, RangeBounds};

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct CumSum {
        pub cumsum: Vec<i64>,
    }

    impl CumSum {
        /// # 計算量
        /// O(|xs|)
        pub fn new(xs: &[i64]) -> CumSum {
            let mut cumsum = vec![0; xs.len() + 1];
            for i in 1..xs.len() + 1 {
                cumsum[i] = cumsum[i - 1] + xs[i - 1];
            }
            CumSum { cumsum }
        }

        fn open(&self, range: impl RangeBounds<usize>) -> Range<usize> {
            use Bound::Excluded;
            use Bound::Included;
            use Bound::Unbounded;
            let begin = match range.start_bound() {
                Unbounded => 0,
                Included(&x) => x,
                Excluded(&x) => x + 1,
            };
            let end = match range.end_bound() {
                Excluded(&x) => x,
                Included(&x) => x + 1,
                Unbounded => self.cumsum.len() - 1, // xs.len() == self.cumsum.len() - 1
            };
            begin..end
        }

        /// 区間 `[begin, end)` の要素の和を計算します。
        ///
        /// # 計算量
        /// O(1)
        pub fn range_sum(&self, range: impl RangeBounds<usize>) -> i64 {
            let range = self.open(range);
            self.cumsum[range.end] - self.cumsum[range.start]
        }

        /// 区間 `[0, end)` での和を計算します。
        ///
        /// # 計算量
        /// O(1)
        pub fn prefix_sum(&self, end: usize) -> i64 {
            self.cumsum[end]
        }

        /// 区間 `[begin, n)` の要素の和を計算します。（`n` は元の配列の長さ）
        ///
        /// # 計算量
        /// O(1)
        pub fn suffix_sum(&self, begin: usize) -> i64 {
            self.cumsum[self.cumsum.len() - 1] - self.cumsum[begin]
        }

        /// `f(sum(l..r))` が `true` となる最大の `r in [l, n]` を見つける。
        ///
        /// `n` は元の配列の長さ。
        ///
        /// # 前提条件
        /// * `l <= n`
        /// * `f(0)` が `true`
        /// * `f` は単調である。
        ///     * `f(sum(l..i))` が `true` => `f(sum(l..j))` が `true` for all `l <= j <= i`.
        ///
        /// # 計算量
        /// O(log n)
        pub fn max_right<F>(&self, l: usize, mut f: F) -> usize
        where
            F: FnMut(i64) -> bool,
        {
            let n = self.cumsum.len() - 1;
            assert!(l <= n);
            assert!(f(0), "f(0) must be true");

            if f(self.range_sum(l..n)) {
                return n;
            }

            let mut ok = l;
            let mut ng = n + 1;

            while ng - ok > 1 {
                let mid = ok + (ng - ok) / 2;
                if f(self.range_sum(l..mid)) {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok
        }

        /// `f(sum(l..r))` が `true` となる最小の `l in [0, r]` を見つける。
        ///
        /// # 前提条件
        /// * `r <= n`
        /// * `f(0)` が `true`
        /// * `f` は単調である。
        ///     * `f(sum(i..r))` が `true` => `f(sum(j..r))` が `true` for all `i <= j <= r`.
        ///
        /// # 計算量
        /// O(log r)
        pub fn min_left<F>(&self, r: usize, mut f: F) -> usize
        where
            F: FnMut(i64) -> bool,
        {
            let n = self.cumsum.len() - 1;
            assert!(r <= n);
            assert!(f(0), "f(0) must be true");

            if f(self.range_sum(0..r)) {
                return 0;
            }

            let mut ok = r;
            let mut ng = 0;

            while ok - ng > 1 {
                let mid = ng + (ok - ng) / 2;
                if f(self.range_sum(mid..r)) {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok
        }
    }
}
