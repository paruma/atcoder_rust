use ac_library::LazySegtree;
use map_monoid_template::*;
#[allow(unused_variables)]
#[allow(clippy::module_inception)]
pub mod map_monoid_template {
    use super::mod_neg_ext_int::*;
    use ac_library::lazysegtree::MapMonoid;
    use ac_library::segtree::Monoid;
    use std::convert::Infallible;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct RangeXxx {
        pub len: usize,
        pub cnt_head: usize, // 表の数
        pub max_x: NegExtInt,
    }
    impl RangeXxx {
        pub fn unit(x: i64, is_head: bool) -> Self {
            Self {
                len: 1,
                cnt_head: if is_head { 1 } else { 0 },
                max_x: if is_head { fin(x) } else { NEG_INF },
            }
        }
    }
    pub struct RangeXxxMonoid(Infallible);
    impl Monoid for RangeXxxMonoid {
        type S = RangeXxx;
        fn identity() -> Self::S {
            RangeXxx {
                len: 0,
                cnt_head: 0,
                max_x: NEG_INF,
            }
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            RangeXxx {
                len: a.len + b.len,
                cnt_head: a.cnt_head + b.cnt_head,
                max_x: NegExtInt::max(a.max_x, b.max_x),
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Action {
        pub add: i64,
        pub reset: bool,
        pub flip: bool,
    }
    pub struct RangeYyyRangeXxx(Infallible);
    impl MapMonoid for RangeYyyRangeXxx {
        type M = RangeXxxMonoid;
        type F = Action;
        fn identity_map() -> Self::F {
            Action {
                add: 0,
                reset: false,
                flip: false,
            }
        }
        fn mapping(
            f: &Self::F,
            x: &<Self::M as ac_library::Monoid>::S,
        ) -> <Self::M as ac_library::Monoid>::S {
            if f.flip {
                RangeXxx {
                    len: x.len,
                    cnt_head: x.len - x.cnt_head,
                    max_x: if x.len == x.cnt_head {
                        NEG_INF
                    } else {
                        fin(f.add)
                    },
                }
            } else {
                RangeXxx {
                    len: x.len,
                    cnt_head: x.cnt_head,
                    max_x: if x.cnt_head == 0 {
                        NEG_INF
                    } else {
                        fin(f.add) + if f.reset { fin(0) } else { x.max_x }
                    },
                }
            }
        }
        fn composition(f1: &Self::F, f2: &Self::F) -> Self::F {
            Self::F {
                add: f1.add + if f1.reset { 0 } else { f2.add },
                reset: f1.reset || f2.reset,
                flip: f1.flip ^ f2.flip,
            }
        }
    }
}

define_queries! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Query: usize {
        1 => Add { l: Usize1, r: Usize1, x: i64 },
        2 => Flip { l: Usize1, r: Usize1 },
        3 => Max { l: Usize1, r: Usize1 },
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        qs: [Query; q],
    }

    // 1: 表, 0: 裏
    let mut seg = LazySegtree::<RangeYyyRangeXxx>::from(vec![RangeXxx::unit(0, true); n]);
    for &q in &qs {
        match q {
            Query::Add { l, r, x } => {
                seg.apply_range(
                    l..=r,
                    Action {
                        add: x,
                        reset: false,
                        flip: false,
                    },
                );
            }
            Query::Flip { l, r } => {
                seg.apply_range(
                    l..=r,
                    Action {
                        add: 0,
                        reset: true,
                        flip: true,
                    },
                );
            }
            Query::Max { l, r } => {
                let ans = seg.prod(l..=r).max_x.get_fin_or(0);
                println!("{}", ans);
            }
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
        collections::{BinaryHeap, HashMap, HashSet},
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
        let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Display>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{}", x)).join(" ");
            println!("{}", msg);
        }
    }
    pub fn print_bytes(bytes: &[u8]) {
        let msg = String::from_utf8(bytes.to_vec()).unwrap();
        println!("{}", msg);
    }
    pub fn print_chars(chars: &[char]) {
        let msg = chars.iter().collect::<String>();
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec_bytes(vec_bytes: &[Vec<u8>]) {
        for row in vec_bytes {
            let msg = String::from_utf8(row.to_vec()).unwrap();
            println!("{}", msg);
        }
    }
    #[fastout]
    pub fn print_vec_chars(vec_chars: &[Vec<char>]) {
        for row in vec_chars {
            let msg = row.iter().collect::<String>();
            println!("{}", msg);
        }
    }
    pub fn print_yesno(ans: bool) {
        let msg = if ans { "Yes" } else { "No" };
        println!("{}", msg);
    }
}

// ====== snippet ======
#[macro_use]
pub mod define_queries {
    /// クエリ形式の入力を proconio::input! で読み込める enum を定義するマクロ。
    /// 出典： https://zenn.dev/magurofly/articles/6ee845bd5e385e
    /// # 利用例
    /// ```
    /// use mylib::define_queries;
    /// use proconio::marker::Usize1;
    /// define_queries! {
    ///     #[derive(Debug, PartialEq)]
    ///     enum Query: usize {
    ///         1 => Add { a: i64, b: i64 },
    ///         2 => Show { k: Usize1 },
    ///     }
    /// }
    /// ```
    #[macro_export]
    macro_rules ! define_queries {($ ($ (# [$ attr : meta ] ) * enum $ enum_name : ident : $ sig : ty {$ ($ pattern : pat => $ variant : ident $ ({$ ($ name : ident : $ marker : ty $ (, ) ? ) ,* } ) ? $ (, ) ? ) ,* } ) * ) => {$ ($ (# [$ attr ] ) * enum $ enum_name {$ ($ variant $ ({$ ($ name : <$ marker as proconio :: source :: Readable >:: Output ) ,* } ) ? ) ,* } impl proconio :: source :: Readable for $ enum_name {type Output = Self ; fn read < R : std :: io :: BufRead , S : proconio :: source :: Source < R >> (source : & mut S ) -> Self {#! [allow (unreachable_patterns ) ] match <$ sig as proconio :: source :: Readable >:: read (source ) {$ ($ pattern => $ enum_name ::$ variant $ ({$ ($ name : <$ marker as proconio :: source :: Readable >:: read (source ) ) ,* } ) ? ) ,* , _ => unreachable ! () } } } ) * } }
}
use mod_neg_ext_int::*;
pub mod mod_neg_ext_int {
    use ac_library::Monoid;
    use std::{
        cmp::Ordering,
        convert::Infallible,
        fmt,
        ops::{Add, AddAssign, Sub, SubAssign},
    };
    pub const NEG_INF: NegExtInt = NegExtInt::NEG_INF;
    pub fn fin(x: i64) -> NegExtInt {
        NegExtInt::fin(x)
    }
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
            match t.cmp(&0) {
                Ordering::Less => panic!("t must be non-negative."),
                Ordering::Equal => Self(0),
                Ordering::Greater => {
                    if self.is_fin() {
                        Self(self.0 * t)
                    } else {
                        Self::NEG_INF
                    }
                }
            }
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
