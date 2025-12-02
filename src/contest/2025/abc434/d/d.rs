// #[fastout]
#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Cloud {
    u: Usize1,
    d: Usize1,
    l: Usize1,
    r: Usize1,
}
fn main() {
    input! {
        n: usize,
        clouds: [Cloud; n]
    }

    const H: usize = 2000;
    const W: usize = 2000;
    // const H: usize = 10;
    // const W: usize = 10;
    // dbg!("H, W を戻す");

    let mut imos = Imos2D::new(0, (H + 1) as i64, 0, (W + 1) as i64);

    for &c in &clouds {
        imos.add(c.u as i64, c.l as i64, 1);
        imos.add(c.u as i64, (c.r + 1) as i64, -1);
        imos.add((c.d + 1) as i64, c.l as i64, -1);
        imos.add((c.d + 1) as i64, (c.r + 1) as i64, 1);
    }
    imos.summation(0, 1);
    imos.summation(1, 0);

    let all = {
        let mut all = 0_i64;
        for y in 0..H {
            for x in 0..W {
                if imos.get(y as i64, x as i64) > 0 {
                    all += 1;
                }
            }
        }
        all
    };

    let inds = {
        let mut inds = vec![vec![0; W]; H];

        for y in 0..H {
            for x in 0..W {
                inds[y][x] = (imos.get(y as i64, x as i64) == 1) as i64;
            }
        }
        inds
    };
    // table! {&inds};

    let inds_cum = CumSum2D::new(&inds);

    let ans: Vec<i64> = clouds
        .iter()
        .copied()
        .map(|c| {
            let removed_cnt = inds_cum.rect_sum(c.u..=c.d, c.l..=c.r);
            // dbg!(removed_cnt);
            (W * H) as i64 - (all - removed_cnt)
        })
        .collect_vec();
    print_vec(&ans);
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
use imos_2d::*;
/// 2次元いもす法を扱うモジュール
/// # Examples
/// ```
/// use atcoder_rust::mylib::imos::imos_2d::*;
/// let (h, w) = (4, 5);
/// // 1回差分を取るので、各次元の end を1つ余分に確保する
/// let mut imos = Imos2D::new(0, h + 1, 0, w + 1);
/// // 矩形領域 [1, 3) x [2, 4) に 1 を加算
/// imos.add(1, 2, 1);
/// imos.add(1, 4, -1);
/// imos.add(3, 2, -1);
/// imos.add(3, 4, 1);
/// // x方向、y方向の順に累積和を計算
/// imos.summation(0, 1);
/// imos.summation(1, 0);
/// assert_eq!(imos.get(1, 2), 1);
/// assert_eq!(imos.get(2, 3), 1);
/// assert_eq!(imos.get(0, 0), 0);
/// ```
pub mod imos_2d {
    /// 2次元いもす法のための構造体
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Imos2D {
        raw: Vec<Vec<i64>>,
        row_begin: i64,
        row_end: i64,
        col_begin: i64,
        col_end: i64,
    }
    impl Imos2D {
        /// `[row_begin, row_end) x [col_begin, col_end)` の矩形領域を対象とする `Imos2D` を生成する。
        /// `summation` を適用する方向や回数に応じて、`end` を広めに確保する必要がある。
        /// # Panics
        /// `begin >= end` となる次元がある場合、デバッグビルドではパニックする。
        pub fn new(row_begin: i64, row_end: i64, col_begin: i64, col_end: i64) -> Self {
            debug_assert!(row_begin < row_end);
            debug_assert!(col_begin < col_end);
            let height = (row_end - row_begin) as usize;
            let width = (col_end - col_begin) as usize;
            let raw = vec![vec![0; width]; height];
            Self {
                raw,
                row_begin,
                row_end,
                col_begin,
                col_end,
            }
        }
        fn is_within(&self, row: i64, col: i64) -> bool {
            (self.row_begin..self.row_end).contains(&row)
                && (self.col_begin..self.col_end).contains(&col)
        }
        /// `(row, col)` の要素の値を取得する。
        /// `summation` 実行前は差分が、実行後は累積和が返される。
        /// # Panics
        /// `(row, col)` が範囲外の場合、デバッグビルドではパニックする。
        pub fn get(&self, row: i64, col: i64) -> i64 {
            if cfg!(debug_assertions) && !self.is_within(row, col) {
                panic!(
                    "index out of bounds: the domain is [{}, {}) × [{}, {}) but the index is ({}, {})",
                    self.row_begin, self.row_end, self.col_begin, self.col_end, row, col
                );
            }
            self.raw[(row - self.row_begin) as usize][(col - self.col_begin) as usize]
        }
        /// `(row, col)` の要素に `val` を加算する。
        /// 矩形領域 `[r1, r2) x [c1, c2)` に値を加算するには、4点の `add` を呼び出す。
        /// # Panics
        /// `(row, col)` が範囲外の場合、デバッグビルドではパニックする。
        pub fn add(&mut self, row: i64, col: i64, val: i64) {
            if cfg!(debug_assertions) && !self.is_within(row, col) {
                panic!(
                    "index out of bounds: the domain is [{}, {}) × [{}, {}) but the index is ({}, {})",
                    self.row_begin, self.row_end, self.col_begin, self.col_end, row, col
                );
            }
            self.raw[(row - self.row_begin) as usize][(col - self.col_begin) as usize] += val;
        }
        /// `(d_row, d_col)` 方向の差分の累積和を計算する。
        /// - `(1, 0)`: y方向の累積和
        /// - `(0, 1)`: x方向の累積和
        /// - `(1, 1)`: 右下方向の累積和
        /// # Panics
        /// `(d_row, d_col) == (0, 0)` の場合、デバッグビルドではパニックする。
        pub fn summation(&mut self, d_row: i64, d_col: i64) {
            debug_assert_ne!((d_row, d_col), (0, 0));
            let height = self.row_end - self.row_begin;
            let width = self.col_end - self.col_begin;
            if d_row > 0 || (d_row == 0 && d_col > 0) {
                for row in 0..height {
                    for col in 0..width {
                        let prev_row = row - d_row;
                        let prev_col = col - d_col;
                        if (0..height).contains(&prev_row) && (0..width).contains(&prev_col) {
                            self.raw[row as usize][col as usize] +=
                                self.raw[prev_row as usize][prev_col as usize]
                        }
                    }
                }
            } else {
                for row in (0..height).rev() {
                    for col in (0..width).rev() {
                        let prev_row = row - d_row;
                        let prev_col = col - d_col;
                        if (0..height).contains(&prev_row) && (0..width).contains(&prev_col) {
                            self.raw[row as usize][col as usize] +=
                                self.raw[prev_row as usize][prev_col as usize]
                        }
                    }
                }
            }
        }
    }
}
use cumsum_2d::*;
#[allow(clippy::module_inception)]
pub mod cumsum_2d {
    use std::ops::{Bound, Range, RangeBounds};
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct CumSum2D {
        pub cumsum: Vec<Vec<i64>>,
    }
    impl CumSum2D {
        pub fn new(xss: &[Vec<i64>]) -> CumSum2D {
            if xss.is_empty() {
                return CumSum2D {
                    cumsum: vec![vec![0]],
                };
            }
            let height = xss.len();
            let width = xss[0].len();
            let mut cumsum = vec![vec![0; width + 1]; height + 1];
            for y in 1..height + 1 {
                for x in 1..width + 1 {
                    cumsum[y][x] = cumsum[y - 1][x] + cumsum[y][x - 1] - cumsum[y - 1][x - 1]
                        + xss[y - 1][x - 1];
                }
            }
            CumSum2D { cumsum }
        }
        pub fn rect_sum(
            &self,
            y_range: impl RangeBounds<usize>,
            x_range: impl RangeBounds<usize>,
        ) -> i64 {
            let y_len = self.cumsum.len() - 1;
            let x_len = self.cumsum[0].len() - 1;
            let y_range = open(y_range, y_len);
            let x_range = open(x_range, x_len);
            let y1 = y_range.start;
            let y2 = y_range.end;
            let x1 = x_range.start;
            let x2 = x_range.end;
            self.cumsum[y2][x2] - self.cumsum[y2][x1] - self.cumsum[y1][x2] + self.cumsum[y1][x1]
        }
    }
    fn open(range: impl RangeBounds<usize>, len: usize) -> Range<usize> {
        let begin = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x + 1,
        };
        let end = match range.end_bound() {
            Bound::Excluded(&x) => x,
            Bound::Included(&x) => x + 1,
            Bound::Unbounded => len,
        };
        begin..end
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
    /// # use lg::*;
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
    /// # use lg::*;
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
    /// # use lg::*;
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
