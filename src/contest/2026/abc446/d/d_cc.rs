// 座標圧縮解法
fn main() {
    input! {
        n: usize,
        xs: [i64; n],
    }

    let space = xs.iter().copied().flat_map(|x| [x - 1, x]).collect_vec();
    let cc = CoordinateCompression::new(&space);

    let mut dp = vec![NEG_INF; cc.space_size()];

    for i in 0..n {
        let left = dp[cc.compress(xs[i] - 1)];
        dp[cc.compress(xs[i])] = (left + 1).max(fin(1));
    }

    let ans: i64 = dp.iter().copied().max().unwrap().get_fin();
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
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
use coordinate_compression::*;
#[allow(clippy::module_inception)]
pub mod coordinate_compression {
    use itertools::Itertools;
    use superslice::Ext;
    #[derive(Debug, Clone)]
    pub struct CoordinateCompression<T> {
        space: Vec<T>,
    }
    impl<T: Ord + Copy> CoordinateCompression<T> {
        /// 与えられた要素から座標圧縮空間を構築する。
        /// # 計算量
        /// O(N log N) (N = |space|)
        pub fn new(space: &[T]) -> Self {
            let space = space.iter().copied().sorted().dedup().collect_vec();
            Self { space }
        }
        /// 与えられた値を座標圧縮したインデックスを返す。
        /// 値が空間に存在しない場合はパニックする。
        /// # 計算量
        /// O(log N) (N = space_size)
        pub fn compress(&self, x: T) -> usize {
            self.space.binary_search(&x).unwrap()
        }
        /// 座標圧縮前の空間のうち x 以下である最大の値を座標圧縮したものを返す
        /// # 計算量
        /// O(log N) (N = space_size)
        pub fn compress_floor(&self, x: T) -> usize {
            self.space.upper_bound(&x) - 1
        }
        /// 座標圧縮前の空間のうち x 以上である最小の値を座標圧縮したものを返す
        /// # 計算量
        /// O(log N) (N = space_size)
        pub fn compress_ceil(&self, x: T) -> usize {
            self.space.lower_bound(&x)
        }
        /// 与えられた各要素を座標圧縮した結果を返す。
        /// # 計算量
        /// O(M log N) (M = |xs|, N = space_size)
        pub fn compress_vec(&self, xs: &[T]) -> Vec<usize> {
            xs.iter().map(|&x| self.compress(x)).collect_vec()
        }
        /// 指定された範囲内の値に対応する座標圧縮後のインデックス範囲を [begin, end) で返す。
        /// # 計算量
        /// O(log N) (N = space_size)
        pub fn compress_range(
            &self,
            range: impl std::ops::RangeBounds<T>,
        ) -> std::ops::Range<usize> {
            use std::ops::Bound::*;
            let begin = match range.start_bound() {
                Unbounded => 0,
                Included(&x) => self.space.lower_bound(&x),
                Excluded(&x) => self.space.upper_bound(&x),
            };
            let end = match range.end_bound() {
                Unbounded => self.space.len(),
                Included(&x) => self.space.upper_bound(&x),
                Excluded(&x) => self.space.lower_bound(&x),
            };
            begin..end
        }
        /// 座標圧縮されたインデックスから元の値を復元する。
        /// # 計算量
        /// O(1)
        pub fn decompress(&self, i: usize) -> T {
            self.space[i]
        }
        /// 座標圧縮後の空間の大きさ（要素数）を返す。
        pub fn space_size(&self) -> usize {
            self.space.len()
        }
    }
}
