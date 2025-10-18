fn normalize(p: Pos) -> Pos {
    let mut x = p.x;
    let mut y = p.y;
    if x < 0 {
        x *= -1;
        y *= -1;
    }
    if x == 0 {
        y = y.abs();
    }
    let gcd = num_integer::gcd(x.abs(), y.abs());
    x /= gcd;
    y /= gcd;
    Pos::new(x, y)
}

fn solve_sub(n: usize, ps: &[Pos], origin_i: usize) -> Option<(i64, i64, i64)> {
    let origin = ps[origin_i];
    // FxHashMap
    // let mut cnts: FxHashMap<Pos, usize> = FxHashMap::default();
    // for d in (0..n)
    //     .filter(|&i| i != origin_i)
    //     .map(|i| normalize(ps[i] - origin))
    // {
    //     *cnts.entry(d).or_insert(0) += 1;
    // }

    // 普通のHashMap
    let cnts = (0..n)
        .filter(|&i| i != origin_i)
        .map(|i| normalize(ps[i] - origin))
        .counts(); // ここが重そう

    // let dir = (0..n)
    //     .filter(|&i| i != origin_i)
    //     .map(|i| normalize(ps[i] - origin))
    //     .sorted()
    //     .dedup_with_count()
    //     .filter(|(cnt, _)| *cnt >= n / 2)
    //     .map(|(_, d)| d)
    //     .next()?;

    // lg!(origin_i);
    // lg!(origin);
    // lg!(&cnts);

    let dir = cnts
        .iter()
        .find(|(d, cnt)| **cnt >= n / 2)
        .map(|(d, _)| *d)?;

    // origin を通って 法線ベクトルが (dir.y, -dir.x) となるようなものを求める

    let a = dir.y;
    let b = -dir.x;
    let c = -(a * origin.x + b * origin.y);

    Some((a, b, c))
}

fn main() {
    input! {
        n: usize,
        ps: [PosXY; n],
    }
    //let mut rng = SmallRng::seed_from_u64(43);
    let mut rng = SmallRng::from_entropy();
    let ans: Option<(i64, i64, i64)> = (0..10)
        .filter_map(|_| {
            //
            let origin_i = rng.gen_range(0..n);
            solve_sub(n, &ps, origin_i)
        })
        .next();
    if let Some((a, b, c)) = ans {
        println!("Yes");
        println!("{} {} {}", a, b, c);
    } else {
        println!("No");
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
        let n = rng.gen_range(1..=10);
        let xs = (0..n).map(|_| rng.gen_range(0..10)).collect_vec();

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
        // let mut rng = SmallRng::from_entropy();
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
    itertools::{chain, iproduct, izip, Itertools},
    proconio::{
        derive_readable, fastout, input,
        marker::{Bytes, Chars, Usize1},
    },
    rand::{rngs::SmallRng, seq::SliceRandom, Rng, SeedableRng},
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
use {pos::*, rustc_hash::FxHashMap};
pub mod pos {
    use std::io::BufRead;
    use std::iter::Sum;
    use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Pos {
        pub x: i64,
        pub y: i64,
    }
    impl Pos {
        pub fn new(x: i64, y: i64) -> Pos {
            Pos { x, y }
        }
    }
    impl Pos {
        pub fn scala_mul(self, rhs: i64) -> Pos {
            Pos::new(self.x * rhs, self.y * rhs)
        }
    }
    impl Pos {
        pub fn inner_product(self, rhs: Self) -> i64 {
            self.x * rhs.x + self.y * rhs.y
        }
        pub fn norm_square(self) -> i64 {
            self.inner_product(self)
        }
    }
    impl Add for Pos {
        type Output = Pos;
        fn add(self, rhs: Self) -> Self::Output {
            Pos::new(self.x + rhs.x, self.y + rhs.y)
        }
    }
    impl Sub for Pos {
        type Output = Pos;
        fn sub(self, rhs: Self) -> Self::Output {
            Pos::new(self.x - rhs.x, self.y - rhs.y)
        }
    }
    impl Neg for Pos {
        type Output = Self;
        fn neg(self) -> Self::Output {
            Pos::new(-self.x, -self.y)
        }
    }
    impl Sum for Pos {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Pos::new(0, 0), |acc, x| acc + x)
        }
    }
    impl<'a> Sum<&'a Pos> for Pos {
        fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
            iter.fold(Pos::new(0, 0), |a, b| a + *b)
        }
    }
    impl num_traits::Zero for Pos {
        fn zero() -> Self {
            Pos::new(0, 0)
        }
        fn is_zero(&self) -> bool {
            self.x.is_zero() && self.y.is_zero()
        }
    }
    impl AddAssign for Pos {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }
    impl SubAssign for Pos {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }
    use std::fmt::{Debug, Error, Formatter};
    impl Debug for Pos {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            f.write_fmt(format_args!("({}, {})", self.x, self.y))?;
            Ok(())
        }
    }
    use proconio::source::{Readable, Source};
    pub enum PosXY {}
    impl Readable for PosXY {
        type Output = Pos;
        fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Pos {
            let x = i64::read(source);
            let y = i64::read(source);
            Pos::new(x, y)
        }
    }
    pub enum PosYX {}
    impl Readable for PosYX {
        type Output = Pos;
        fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Pos {
            let y = i64::read(source);
            let x = i64::read(source);
            Pos::new(x, y)
        }
    }
    /// 1-indexed で与えられた座標(YX)
    pub enum PosYX1 {}
    impl Readable for PosYX1 {
        type Output = Pos;
        fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Pos {
            let y = i64::read(source) - 1;
            let x = i64::read(source) - 1;
            Pos::new(x, y)
        }
    }
    pub const DIR8_LIST: [Pos; 8] = [
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 1 },
        Pos { x: 1, y: 0 },
        Pos { x: 1, y: -1 },
        Pos { x: 0, y: -1 },
        Pos { x: -1, y: -1 },
        Pos { x: -1, y: 0 },
        Pos { x: -1, y: 1 },
    ];
    pub const DIR4_LIST: [Pos; 4] = [
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 0 },
        Pos { x: 0, y: -1 },
        Pos { x: -1, y: 0 },
    ];
    impl Pos {
        pub fn around4_pos_iter(self) -> impl Iterator<Item = Pos> {
            DIR4_LIST.iter().copied().map(move |d| self + d)
        }
        pub fn around8_pos_iter(self) -> impl Iterator<Item = Pos> {
            DIR8_LIST.iter().copied().map(move |d| self + d)
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
