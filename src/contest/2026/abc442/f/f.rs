// #[fastout]
fn main() {
    input! {
        n: usize,
        grid: [Bytes; n],
    }
    // 初手 next DP

    let mut dp = vec![fin(0); n + 1];

    for r in 0..n {
        let mut next_dp = vec![INF; n + 1];
        let ind_white = grid[r]
            .iter()
            .copied()
            .map(|ch| (ch == b'.') as i64)
            .collect_vec();
        let ind_black = grid[r]
            .iter()
            .copied()
            .map(|ch| (ch == b'#') as i64)
            .collect_vec();
        let cum_white = CumSum::new(&ind_white);
        let cum_black = CumSum::new(&ind_black);

        let tmp_cummin = CumMonoid::<ExtIntMin>::new(&dp);

        for k in 0..=n {
            let change_bw = cum_black.range_sum(0..n - k);
            let change_wb = cum_white.range_sum(n - k..n);
            let cnt_change = change_bw + change_wb;
            next_dp[k] = tmp_cummin.prefix_prod(k + 1) + cnt_change;
        }
        dp = next_dp;
    }
    let ans: i64 = dp.iter().copied().min().unwrap().get_fin();
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
        /// # 計算量
        /// O(1)
        pub fn range_sum(&self, range: impl RangeBounds<usize>) -> i64 {
            let range = self.open(range);
            self.cumsum[range.end] - self.cumsum[range.start]
        }
        /// 区間 `[0, end)` での和を計算します。
        /// # 計算量
        /// O(1)
        pub fn prefix_sum(&self, end: usize) -> i64 {
            self.cumsum[end]
        }
        /// 区間 `[begin, n)` の要素の和を計算します。（`n` は元の配列の長さ）
        /// # 計算量
        /// O(1)
        pub fn suffix_sum(&self, begin: usize) -> i64 {
            self.cumsum[self.cumsum.len() - 1] - self.cumsum[begin]
        }
        /// `f(sum(l..r))` が `true` となる最大の `r in [l, n]` を見つける。
        /// `n` は元の配列の長さ。
        /// `f` は単調でなければならない。
        /// `f(sum(l..i))` が `true` => `f(sum(l..j))` が `true` for all `l <= j <= i`.
        /// # Panics
        /// `l > n` の場合にパニックする。
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
        /// `f` は単調でなければならない。
        /// `f(sum(i..r))` が `true` => `f(sum(j..r))` が `true` for all `i <= j <= r`.
        /// `r > n` の場合にパニックする。
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
use mod_ext_int::*;
pub mod mod_ext_int {
    use ac_library::Monoid;
    use std::{
        cmp::Ordering,
        convert::Infallible,
        fmt,
        ops::{Add, AddAssign, Sub, SubAssign},
    };
    pub const INF: ExtInt = ExtInt::INF;
    pub fn fin(x: i64) -> ExtInt {
        ExtInt::fin(x)
    }
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ExtInt(i64);
    impl ExtInt {
        pub const INF: Self = Self(i64::MAX);
        pub fn fin(x: i64) -> Self {
            Self(x)
        }
        pub fn get_fin(self) -> i64 {
            if self.is_fin() {
                self.0
            } else {
                panic!("called `ExtInt::get_fin()` on a infinity")
            }
        }
        pub fn get_fin_or(self, default: i64) -> i64 {
            if self.is_fin() { self.0 } else { default }
        }
        #[inline]
        pub fn is_fin(self) -> bool {
            self.0 != i64::MAX
        }
        pub fn is_inf(self) -> bool {
            self.0 == i64::MAX
        }
        pub fn to_option(self) -> Option<i64> {
            if self.is_fin() { Some(self.0) } else { None }
        }
        pub fn from_option(opt: Option<i64>) -> ExtInt {
            match opt {
                Some(a) => Self(a),
                None => Self::INF,
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
                        Self::INF
                    }
                }
            }
        }
    }
    impl Add for ExtInt {
        type Output = ExtInt;
        fn add(self, rhs: Self) -> Self::Output {
            if self.is_inf() || rhs.is_inf() {
                Self::INF
            } else {
                Self::fin(self.0 + rhs.0)
            }
        }
    }
    impl AddAssign for ExtInt {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }
    impl Add<i64> for ExtInt {
        type Output = ExtInt;
        fn add(self, rhs: i64) -> Self::Output {
            if self.is_inf() {
                Self::INF
            } else {
                Self::fin(self.0 + rhs)
            }
        }
    }
    impl AddAssign<i64> for ExtInt {
        fn add_assign(&mut self, rhs: i64) {
            *self = *self + rhs;
        }
    }
    impl Sub<i64> for ExtInt {
        type Output = ExtInt;
        fn sub(self, rhs: i64) -> Self::Output {
            if self.is_inf() {
                Self::INF
            } else {
                Self::fin(self.0 - rhs)
            }
        }
    }
    impl SubAssign<i64> for ExtInt {
        fn sub_assign(&mut self, rhs: i64) {
            *self = *self - rhs;
        }
    }
    impl std::iter::Sum for ExtInt {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            let mut s = 0;
            for x in iter {
                if x.is_inf() {
                    return Self::INF;
                }
                s += x.0;
            }
            Self::fin(s)
        }
    }
    impl fmt::Display for ExtInt {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.is_inf() {
                write!(f, "+∞")
            } else {
                write!(f, "{}", self.0)
            }
        }
    }
    impl fmt::Debug for ExtInt {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.is_inf() {
                write!(f, "+∞")
            } else {
                write!(f, "{}", self.0)
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ExtIntAdditive(Infallible);
    impl Monoid for ExtIntAdditive {
        type S = ExtInt;
        fn identity() -> Self::S {
            ExtInt::fin(0)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ExtIntMin(Infallible);
    impl Monoid for ExtIntMin {
        type S = ExtInt;
        fn identity() -> Self::S {
            ExtInt::INF
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a.min(b)
        }
    }
}
use lg::*;
pub mod lg {
    use std::borrow::Borrow;
    use std::fmt;
    use std::iter::once;
    /// Print the values with the line number.
    /// # Examples
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
use cum_monoid::*;
pub mod cum_monoid {
    use ac_library::{Max, Min, Monoid};
    pub struct CumMonoid<M>
    where
        M: Monoid,
    {
        prefix_prod: Vec<M::S>,
        suffix_prod: Vec<M::S>,
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
        pub fn max_without_range(&self, l: usize, r: usize) -> i64 {
            self.cum.prod_without_range(l, r)
        }
    }
}
