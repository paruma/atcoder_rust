//#[derive_readable]
#[derive(Debug)]
struct Problem {
    n: usize,
    xs: Vec<usize>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            xs: [Usize1; n]
        }
        Problem { n, xs }
    }
    fn solve(&self) -> Answer {
        let n = self.n;
        let mut xs = self.xs.clone();
        let mut idx_map = vec![0_usize; n];
        for (i, &x) in xs.iter().enumerate() {
            idx_map[x] = i;
        }

        let mut ans: Vec<(usize, usize)> = vec![];

        for v in 0..n {
            if idx_map[v] != v {
                // 01234567
                // 2....0.. を交換する場合
                // もともと0の場所
                // 古いxs[0] の値
                let old_v_idx = idx_map[v]; //5
                let old_xs_v = xs[v]; //2

                idx_map[old_xs_v] = old_v_idx;
                idx_map[v] = v;
                xs.swap(v, old_v_idx);
                ans.push((v, old_v_idx));
            }
        }

        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        todo!();
        // let ans = 0;
        // Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<(usize, usize)>,
}

impl Answer {
    fn print(&self) {
        println!("{}", self.ans.len());
        for &(x, y) in &self.ans {
            println!("{} {}", x + 1, y + 1);
        }
    }
}

fn main() {
    Problem::read().solve().print();
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

    fn check(p: &Problem) {
        assert_eq!(p.solve(), p.solve_naive());
    }

    fn make_random_problem() -> Problem {
        todo!()
        // let mut rng = SmallRng::from_entropy();
        // let n = rng.gen_range(1..=10);
        // let p = Problem { _a: n };
        // println!("{:?}", &p);
        // p
    }

    #[test]
    fn test_with_naive() {
        // 手動でテストを作るのもOK
        for _ in 0..100 {
            // let p = make_random_problem();
            // check(&p);
        }
    }
}

// ====== import ======
#[allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet};

// ====== output func ======
#[allow(unused_imports)]
use print_vec::*;
pub mod print_vec {

    use itertools::Itertools;
    use proconio::fastout;
    #[fastout]
    pub fn print_vec<T: std::fmt::Debug>(arr: &[T]) {
        for a in arr {
            println!("{:?}", a);
        }
    }
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Debug>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{:?}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Debug>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{:?}", x)).join(" ");
            println!("{}", msg);
        }
    }
    pub fn print_bytes(bytes: &[u8]) {
        let msg = String::from_utf8(bytes.to_vec()).unwrap();
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec_bytes(vec_bytes: &[Vec<u8>]) {
        for row in vec_bytes {
            let msg = String::from_utf8(row.to_vec()).unwrap();
            println!("{}", msg);
        }
    }
}

#[allow(unused)]
fn print_yesno(ans: bool) {
    let msg = if ans { "Yes" } else { "No" };
    println!("{}", msg);
}

// ====== snippet ======
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
    macro_rules ! lg {(@ contents $ head : expr $ (, $ tail : expr ) * ) => {{$ crate :: __lg_internal ! ($ head ) ; $ (eprint ! ("," ) ; $ crate :: __lg_internal ! ($ tail ) ; ) * eprintln ! () ; } } ; ($ ($ expr : expr ) ,* $ (, ) ? ) => {{eprint ! ("{}\u{276f}" , line ! () ) ; $ crate :: lg ! (@ contents $ ($ expr ) ,* ) } } ; }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __lg_internal {
        ($ value : expr ) => {{
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
    macro_rules ! rows {{$ index_label : literal , $ (@ offset $ offset : expr , ) ? $ (@ verticalbar $ verticalbar : expr , ) * $ ($ (@$ label : literal => ) ? $ values : expr ) ,* $ (, ) ? } => {{#! [allow (unused_assignments ) ] let mut rows = $ crate :: Rows :: default () ; rows . line_number (line ! () ) ; $ (rows . offset ($ offset ) ; ) ? $ (rows . verticalbar ($ verticalbar ) ; ) * rows . index_label ($ index_label ) ; $ ({let mut label = stringify ! ($ values ) . to_string () ; if label . starts_with ("&" ) {label = label [1 .. ] . to_string () ; } $ ({let label_ : &'static str = $ label ; label = label_ . to_string () ; } ) ? rows . row (label , $ values ) ; } ) * eprintln ! ("{}" , rows . to_string_table () ) ; } } ; }
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
    macro_rules ! table {{$ (@$ name : literal => ) ? $ values : expr $ (, ) ? } => {{#! [allow (unused_assignments ) ] let mut name = stringify ! ($ values ) . to_string () ; if name . starts_with ("&" ) {name = name [1 .. ] . to_string () ; } $ ({let name_ : &'static str = $ name ; name = name_ . to_string () ; } ) ? let mut rows = $ crate :: Rows :: default () ; rows . line_number (line ! () ) ; rows . table_name (name ) ; # [allow (array_into_iter ) ] for (i , row ) in $ values . into_iter () . enumerate () {rows . row (i . to_string () , row ) ; } eprintln ! ("{}" , rows . to_string_table () ) ; } } ; }
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
