use ac_library::LazySegtree;
// 問題文と制約は読みましたか？
// #[fastout]
use map_monoid_template::*;
#[allow(unused_variables)]
#[allow(clippy::module_inception)]
pub mod map_monoid_template {
    use ac_library::lazysegtree::MapMonoid;
    use ac_library::segtree::Monoid;
    use std::convert::Infallible;

    use crate::mod_neg_ext_int::{NEG_INF, NegExtInt};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeXxxMonoid(Infallible);
    impl Monoid for RangeXxxMonoid {
        // a,b,c の prefix sum の max を管理
        type S = [NegExtInt; 3];
        fn identity() -> Self::S {
            [NEG_INF; 3]
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            [0, 1, 2].map(|i| a[i].max(b[i]))
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Func {
        // swap して add する
        pub swap: [usize; 3],
        pub add: [NegExtInt; 3],
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeYyyRangeXxx(Infallible);
    impl MapMonoid for RangeYyyRangeXxx {
        type M = RangeXxxMonoid;
        type F = Func;
        fn identity_map() -> Self::F {
            Func {
                swap: [0, 1, 2],
                add: [NegExtInt::fin(0), NegExtInt::fin(0), NegExtInt::fin(0)],
            }
        }
        fn mapping(
            f: &Self::F,
            x: &<Self::M as ac_library::Monoid>::S,
        ) -> <Self::M as ac_library::Monoid>::S {
            [0, 1, 2].map(|i| x[f.swap[i]] + f.add[i])
        }
        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            let swap = [0, 1, 2].map(|i| f.swap[g.swap[i]]);
            let mut add0 = [NegExtInt::fin(0), NegExtInt::fin(0), NegExtInt::fin(0)];
            for i in 0..3 {
                add0[f.swap[i]] = g.add[i];
                // add0[i] = g.add[f.swap[i]];
            }
            let add = [0, 1, 2].map(|i| add0[i] + f.add[i]);
            Func { swap, add }
        }
    }
}
fn main() {
    input! {
        n: usize,
        nq: usize,
        a_s: [i64; n],
        b_s: [i64; n],
        c_s: [i64; n],
    }

    let pa = prefix_sum(&a_s);
    let pb = prefix_sum(&b_s);
    let pc = prefix_sum(&c_s);

    let mut seg = LazySegtree::<RangeYyyRangeXxx>::from(
        (0..=n)
            .map(|i| {
                [
                    NegExtInt::fin(pa[i]),
                    NegExtInt::fin(pb[i]),
                    NegExtInt::fin(pc[i]),
                ]
            })
            .collect_vec(),
    );

    for _ in 0..nq {
        input! {
            t: usize,
        }
        if t == 4 {
            input! {
                l: Usize1,
            }
            let al = seg.get(l)[0];

            let ans = seg.max_right(l, |max| max[0] <= al);
            println!("{}", ans);
        } else {
            input! {
                l: Usize1,
                r: Usize1,
            }
            let r = r + 1; // 半開区間にする

            // t=1: swap(a,b)
            // t=2: swap(b,c)
            // t=3: swap(c,a)

            if t == 1 {
                let al = seg.get(l)[0].get_fin();
                let ar = seg.get(r)[0].get_fin();
                let bl = seg.get(l)[1].get_fin();
                let br = seg.get(r)[1].get_fin();

                // (l, r]
                let f = Func {
                    swap: [1, 0, 2],
                    add: [fin(al - bl), fin(bl - al), fin(0)],
                };
                seg.apply_range(l + 1..=r, f);

                // (r, N]
                let f = Func {
                    swap: [0, 1, 2],
                    add: [
                        fin(-(ar - al) + (br - bl)),
                        fin((ar - al) - (br - bl)),
                        fin(0),
                    ],
                };
                seg.apply_range(r + 1..=n, f);
            }
            if t == 2 {
                let bl = seg.get(l)[1].get_fin();
                let br = seg.get(r)[1].get_fin();
                let cl = seg.get(l)[2].get_fin();
                let cr = seg.get(r)[2].get_fin();
                // (l, r]
                let f = Func {
                    swap: [0, 2, 1],
                    add: [fin(0), fin(bl - cl), fin(cl - bl)],
                };
                seg.apply_range(l + 1..=r, f);

                // (r, N]
                let f = Func {
                    swap: [0, 1, 2],
                    add: [
                        fin(0),
                        fin(-(br - bl) + (cr - cl)),
                        fin((br - bl) - (cr - cl)),
                    ],
                };
                seg.apply_range(r + 1..=n, f);
            }
            if t == 3 {
                let cl = seg.get(l)[2].get_fin();
                let cr = seg.get(r)[2].get_fin();
                let al = seg.get(l)[0].get_fin();
                let ar = seg.get(r)[0].get_fin();

                // (l, r]
                let f = Func {
                    swap: [2, 1, 0],
                    add: [fin(al - cl), fin(0), fin(cl - al)],
                };
                seg.apply_range(l + 1..=r, f);

                // (r, N]
                let f = Func {
                    swap: [0, 1, 2],
                    add: [
                        fin((cr - cl) - (ar - al)),
                        fin(0),
                        fin(-(cr - cl) + (ar - al)),
                    ],
                };
                seg.apply_range(r + 1..=n, f);
            }
        }
        // dbg!(lazy_segtree_to_vec(&mut seg, n + 1));
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
use mod_neg_ext_int::*;
pub mod mod_neg_ext_int {
    use ac_library::Monoid;
    use std::{
        cmp::Ordering,
        convert::Infallible,
        fmt,
        ops::{Add, AddAssign, Mul, Sub, SubAssign},
    };
    pub const NEG_INF: NegExtInt = NegExtInt::NEG_INF;
    pub fn fin(x: i64) -> NegExtInt {
        NegExtInt::fin(x)
    }
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct NegExtInt(i64);
    impl NegExtInt {
        pub const NEG_INF: Self = Self(i64::MIN);
        pub fn fin(x: i64) -> Self {
            Self(x)
        }
        pub fn get_fin(self) -> i64 {
            if self.is_fin() {
                self.0
            } else {
                panic!("called `NegExtInt::get_fin()` on a negative infinity")
            }
        }
        pub fn get_fin_or(self, default: i64) -> i64 {
            if self.is_fin() { self.0 } else { default }
        }
        #[inline]
        pub fn is_fin(self) -> bool {
            self.0 != i64::MIN
        }
        pub fn is_neg_inf(self) -> bool {
            self.0 == i64::MIN
        }
        pub fn to_option(self) -> Option<i64> {
            if self.is_fin() { Some(self.0) } else { None }
        }
        pub fn from_option(opt: Option<i64>) -> NegExtInt {
            match opt {
                Some(a) => Self(a),
                None => Self::NEG_INF,
            }
        }
        pub fn times(self, t: i64) -> Self {
            self * t
        }
    }
    impl Add for NegExtInt {
        type Output = NegExtInt;
        fn add(self, rhs: Self) -> Self::Output {
            if self.is_neg_inf() || rhs.is_neg_inf() {
                Self::NEG_INF
            } else {
                Self::fin(self.0 + rhs.0)
            }
        }
    }
    impl AddAssign for NegExtInt {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }
    impl Add<i64> for NegExtInt {
        type Output = NegExtInt;
        fn add(self, rhs: i64) -> Self::Output {
            if self.is_neg_inf() {
                Self::NEG_INF
            } else {
                Self::fin(self.0 + rhs)
            }
        }
    }
    impl AddAssign<i64> for NegExtInt {
        fn add_assign(&mut self, rhs: i64) {
            *self = *self + rhs;
        }
    }
    impl Sub<i64> for NegExtInt {
        type Output = NegExtInt;
        fn sub(self, rhs: i64) -> Self::Output {
            if self.is_neg_inf() {
                Self::NEG_INF
            } else {
                Self::fin(self.0 - rhs)
            }
        }
    }
    impl SubAssign<i64> for NegExtInt {
        fn sub_assign(&mut self, rhs: i64) {
            *self = *self - rhs;
        }
    }
    impl Mul<i64> for NegExtInt {
        type Output = NegExtInt;
        fn mul(self, rhs: i64) -> Self::Output {
            match rhs.cmp(&0) {
                Ordering::Less => panic!("multiplier must be non-negative."),
                Ordering::Equal => Self::fin(0),
                Ordering::Greater => {
                    if self.is_fin() {
                        Self::fin(self.0 * rhs)
                    } else {
                        Self::NEG_INF
                    }
                }
            }
        }
    }
    impl std::iter::Sum for NegExtInt {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            let mut s = 0;
            for x in iter {
                if x.is_neg_inf() {
                    return Self::NEG_INF;
                }
                s += x.0;
            }
            Self::fin(s)
        }
    }
    impl fmt::Display for NegExtInt {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.is_neg_inf() {
                write!(f, "-∞")
            } else {
                write!(f, "{}", self.0)
            }
        }
    }
    impl fmt::Debug for NegExtInt {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.is_neg_inf() {
                write!(f, "-∞")
            } else {
                write!(f, "{}", self.0)
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct NegExtIntAdditive(Infallible);
    impl Monoid for NegExtIntAdditive {
        type S = NegExtInt;
        fn identity() -> Self::S {
            NegExtInt::fin(0)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct NegExtIntMax(Infallible);
    impl Monoid for NegExtIntMax {
        type S = NegExtInt;
        fn identity() -> Self::S {
            NegExtInt::NEG_INF
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a.max(b)
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
                Unbounded => self.cumsum.len() - 1,
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
pub fn lazy_segtree_to_vec<F: ac_library::MapMonoid>(
    seg: &mut ac_library::LazySegtree<F>,
    len: usize,
) -> Vec<<F::M as ac_library::Monoid>::S> {
    (0..len).map(|i| seg.get(i)).collect()
}
