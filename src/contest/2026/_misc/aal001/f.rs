use ac_library::Segtree;
// 問題文と制約は読みましたか？
// #[fastout]
use monoid_template::*;
#[allow(unused_variables)]
pub mod monoid_template {
    use ac_library::segtree::Monoid;
    use std::convert::Infallible;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct RangeXxx {
        pub prefix_val: i64,
        pub prefix_cnt: usize,
        pub suffix_val: i64,
        pub suffix_cnt: usize,
        pub internal_cnt: usize,
        pub len: usize,
        pub all_eq: Option<i64>,
    }
    impl RangeXxx {
        pub fn unit(x: i64) -> Self {
            Self {
                prefix_val: x,
                prefix_cnt: 1,
                suffix_val: x,
                suffix_cnt: 1,
                internal_cnt: 1,
                len: 1,
                all_eq: Some(x),
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeXxxMonoid(Infallible);
    impl Monoid for RangeXxxMonoid {
        type S = RangeXxx;
        fn identity() -> Self::S {
            RangeXxx {
                prefix_val: 0,
                prefix_cnt: 0,
                suffix_val: 0,
                suffix_cnt: 0,
                internal_cnt: 0,
                len: 0,
                all_eq: Some(0),
            }
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            if a.internal_cnt == 0 {
                return *b;
            }
            if b.internal_cnt == 0 {
                return *a;
            }

            let internal = [
                a.internal_cnt,
                b.internal_cnt,
                if a.suffix_val == b.prefix_val {
                    a.suffix_cnt + b.prefix_cnt
                } else {
                    0
                },
            ]
            .iter()
            .copied()
            .max()
            .unwrap();

            RangeXxx {
                prefix_val: a.prefix_val,
                prefix_cnt: if a.all_eq == Some(b.prefix_val) {
                    a.len + b.prefix_cnt
                } else {
                    a.prefix_cnt
                },
                suffix_val: b.suffix_val,
                suffix_cnt: if b.all_eq == Some(a.suffix_val) {
                    a.suffix_cnt + b.len
                } else {
                    b.suffix_cnt
                },
                internal_cnt: internal,
                len: a.len + b.len,
                all_eq: if a.all_eq == b.all_eq { a.all_eq } else { None },
            }
        }
    }
}
fn main() {
    input! {
        n: usize,
        q: usize,
        xs: [i64; n],
    }

    let mut s =
        Segtree::<RangeXxxMonoid>::from(xs.iter().copied().map(RangeXxx::unit).collect_vec());

    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                i: Usize1,
                x: i64
            }
            s.set(i, RangeXxx::unit(x));
        }
        if t == 2 {
            input! {
                l: Usize1,
                r: Usize1
            }
            let ans = s.prod(l..=r).internal_cnt;
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
pub fn segtree_to_vec<M: ac_library::Monoid>(
    seg: &ac_library::Segtree<M>,
    len: usize,
) -> Vec<M::S> {
    (0..len).map(|i| seg.get(i)).collect()
}

use lg::*;
pub mod lg {
    use std::borrow::Borrow;
    use std::fmt;
    use std::iter::once;
    /// Print the values with the line number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use mylib::*;
    /// let x = 42;
    /// let y = 43;
    /// lg!(x);
    /// lg!(x, y);
    /// lg!(42, x, 43, y);
    /// ```
    #[macro_export]
    macro_rules ! lg {(@ contents $ head : expr_2021 $ (, $ tail : expr_2021 ) * ) => {{$ crate :: __lg_internal ! ($ head ) ; $ (eprint ! ("," ) ; $ crate :: __lg_internal ! ($ tail ) ; ) * eprintln ! () ; } } ; ($ ($ expr : expr_2021 ) ,* $ (, ) ? ) => {{eprint ! ("{}\u{276f}" , line ! () ) ; $ crate :: lg ! (@ contents $ ($ expr ) ,* ) } } ; }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __lg_internal {
        ($ value : expr_2021 ) => {{
            match $value {
                head => {
                    eprint!(
                        " {} = {}",
                        stringify!($value),
                        $crate::__quiet(format!("{:?}", &head))
                    );
                }
            }
        }};
    }
    /// Print many 1D arrays side-by-side with the line number.
    ///
    /// # Examples
    /// ```rust
    /// # use mylib::*;
    /// let a = [1, 2, 3];
    /// let b = [4, 5, 6];
    /// let c = [7, 8, 9];
    /// rows! {
    ///   "id", // the name of the index
    ///   @"a" => a,
    ///   b,
    ///   @"c" => c,
    /// }
    /// ```
    #[macro_export]
    macro_rules ! rows {{$ index_label : literal , $ (@ offset $ offset : expr_2021 , ) ? $ (@ verticalbar $ verticalbar : expr_2021 , ) * $ ($ (@$ label : literal => ) ? $ values : expr_2021 ) ,* $ (, ) ? } => {{#! [allow (unused_assignments ) ] let mut rows = $ crate :: Rows :: default () ; rows . line_number (line ! () ) ; $ (rows . offset ($ offset ) ; ) ? $ (rows . verticalbar ($ verticalbar ) ; ) * rows . index_label ($ index_label ) ; $ ({let mut label = stringify ! ($ values ) . to_string () ; if label . starts_with ("&" ) {label = label [1 .. ] . to_string () ; } $ ({let label_ : &'static str = $ label ; label = label_ . to_string () ; } ) ? rows . row (label , $ values ) ; } ) * eprintln ! ("{}" , rows . to_string_table () ) ; } } ; }
    /// Print the 2D array with the line number.
    ///
    /// # Examples
    /// ```rust
    /// # use mylib::*;
    /// let a = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    /// table! {
    ///    @"a" => a,
    /// }
    /// table! {
    ///   a,
    /// }
    /// ```
    #[macro_export]
    macro_rules ! table {{$ (@$ name : literal => ) ? $ values : expr_2021 $ (, ) ? } => {{#! [allow (unused_assignments ) ] let mut name = stringify ! ($ values ) . to_string () ; if name . starts_with ("&" ) {name = name [1 .. ] . to_string () ; } $ ({let name_ : &'static str = $ name ; name = name_ . to_string () ; } ) ? let mut rows = $ crate :: Rows :: default () ; rows . line_number (line ! () ) ; rows . table_name (name ) ; # [allow (array_into_iter ) ] for (i , row ) in $ values . into_iter () . enumerate () {rows . row (i . to_string () , row ) ; } eprintln ! ("{}" , rows . to_string_table () ) ; } } ; }
    #[doc(hidden)]
    pub fn __quiet(s: impl AsRef<str>) -> String {
        s.as_ref()
            .replace("340282366920938463463374607431768211455", "*")
            .replace("170141183460469231731687303715884105727", "*")
            .replace("18446744073709551615", "*")
            .replace("9223372036854775807", "*")
            .replace("-9223372036854775808", "*")
            .replace("4294967295", "*")
            .replace("2147483647", "*")
            .replace("-2147483648", "*")
            .replace("None", "*")
            .replace("Some", "")
            .replace("true", "#")
            .replace("false", ".")
            .replace(['"', '\''], "")
    }
    #[doc(hidden)]
    #[derive(Default)]
    pub struct Rows {
        line_number: String,
        index_label: String,
        offset: usize,
        verticalbars: Vec<usize>,
        table_name: String,
        rows: Vec<Row>,
    }
    impl Rows {
        pub fn line_number(&mut self, line_number: u32) -> &mut Self {
            self.line_number = format!("{}", line_number);
            self
        }
        pub fn index_label(&mut self, index_label: impl Into<String>) -> &mut Self {
            self.index_label = index_label.into();
            self
        }
        pub fn offset(&mut self, offset: usize) -> &mut Self {
            self.offset = offset;
            self
        }
        pub fn verticalbar(&mut self, verticalbar: impl IntoIterator<Item = usize>) -> &mut Self {
            self.verticalbars.extend(verticalbar);
            self
        }
        pub fn table_name(&mut self, table_name: impl Into<String>) -> &mut Self {
            self.table_name = table_name.into();
            self
        }
        pub fn row(
            &mut self,
            label: impl Into<String>,
            values: impl IntoIterator<Item = impl fmt::Debug>,
        ) -> &mut Self {
            self.rows.push(Row {
                label: label.into(),
                values: values
                    .into_iter()
                    .map(|value| __quiet(format!("{:?}", value)))
                    .collect(),
            });
            self
        }
        pub fn to_string_table(self) -> StringTable {
            let Self {
                line_number,
                index_label,
                offset,
                verticalbars,
                table_name,
                rows,
            } = self;
            let w = rows
                .iter()
                .map(|row| row.values.len())
                .max()
                .unwrap_or_default();
            let mut verticalbar_count = vec![0; w + 1];
            for &v in &verticalbars {
                if (offset..=offset + w).contains(&v) {
                    verticalbar_count[v - offset] += 1;
                }
            }
            StringTable {
                head: StringRow {
                    label: format!(
                        "{line_number}❯ {table_name}{index_label}",
                        index_label = if index_label.is_empty() {
                            String::new()
                        } else {
                            format!("[{}]", index_label)
                        }
                    ),
                    values: (offset..offset + w)
                        .map(|index| index.to_string())
                        .collect(),
                },
                body: rows
                    .iter()
                    .map(|row| StringRow {
                        label: row.label.clone(),
                        values: row.values.clone(),
                    })
                    .collect(),
                verticalbar_count,
            }
        }
    }
    struct Row {
        label: String,
        values: Vec<String>,
    }
    #[doc(hidden)]
    pub struct StringTable {
        head: StringRow,
        body: Vec<StringRow>,
        verticalbar_count: Vec<usize>,
    }
    impl fmt::Display for StringTable {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let Self {
                head,
                body,
                verticalbar_count,
            } = self;
            let w = body
                .iter()
                .map(|row| row.values.len())
                .max()
                .unwrap_or_default();
            let label_width = once(head.label.chars().count())
                .chain(body.iter().map(|row| row.label.chars().count()))
                .max()
                .unwrap();
            let value_width = (0..w)
                .map(|j| {
                    once(j.to_string().len())
                        .chain(
                            body.iter()
                                .map(|row| row.values.get(j).map_or(0, |s| s.chars().count())),
                        )
                        .max()
                        .unwrap()
                })
                .collect::<Vec<_>>();
            gray(f)?;
            write!(
                f,
                "{}",
                head.to_string(label_width, &value_width, verticalbar_count, true)
            )?;
            resetln(f)?;
            for row in body {
                write!(
                    f,
                    "{}",
                    row.to_string(label_width, &value_width, verticalbar_count, false)
                )?;
                writeln!(f)?;
            }
            Ok(())
        }
    }
    struct StringRow {
        label: String,
        values: Vec<String>,
    }
    impl StringRow {
        fn to_string(
            &self,
            label_width: usize,
            value_width: &[usize],
            varticalbars_count: &[usize],
            label_align_left: bool,
        ) -> String {
            let Self { label, values } = self;
            let w = value_width.len();
            let mut s = String::new();
            s.push_str(&if label_align_left {
                format!("{label:<label_width$} |")
            } else {
                format!("{label:^label_width$} |")
            });
            for j in 0..w {
                let value_width = value_width[j];
                s.push_str("|".repeat(varticalbars_count[j]).as_str());
                if varticalbars_count[j] == 0 && j != 0 && value_width <= 1 {
                    s.push(' ');
                }
                match values.get(j) {
                    Some(value) => {
                        s.push_str(&format!(" {value:>value_width$}",));
                    }
                    None => {
                        s.push_str(" ".repeat(value_width + 1).as_str());
                    }
                }
            }
            s
        }
    }
    const GRAY: &str = "\x1b[48;2;127;127;127;37m";
    const RESET: &str = "\x1b[0m";
    fn gray(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{GRAY}")
    }
    fn resetln(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{RESET}")
    }
    /// Format a iterator of [`bool`]s.
    pub fn bools<B, I>(iter: I) -> String
    where
        B: Borrow<bool>,
        I: IntoIterator<Item = B>,
    {
        format!(
            "[{}]",
            iter.into_iter()
                .map(|b| ['.', '#'][usize::from(*(b.borrow()))])
                .collect::<String>(),
        )
    }
}
