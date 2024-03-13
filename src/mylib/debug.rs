use cargo_snippet::snippet;

#[snippet(prefix = "use print_arr::*;")]
pub mod print_arr {
    use ndarray::{Array2, Array3};
    use proconio::fastout;

    #[fastout]
    pub fn print_arr<T: std::fmt::Debug>(arr: &[T]) {
        for a in arr {
            print!("{:?} ", a);
        }
        println!();
    }

    #[fastout]
    pub fn print_arr2<T: std::fmt::Debug>(arr: &Array2<T>) {
        for i in 0..arr.nrows() {
            for j in 0..arr.ncols() {
                print!("{:?} ", arr[[i, j]]);
            }
            println!();
        }
    }

    #[fastout]
    pub fn print_arr3<T: std::fmt::Debug>(arr: &Array3<T>) {
        let shape = arr.shape();
        for i in 0..shape[0] {
            for j in 0..shape[1] {
                for k in 0..shape[2] {
                    print!("{:?} ", arr[[i, j, k]]);
                }
                println!();
            }
            println!();
        }
    }
}

#[snippet(prefix = "use print_vec::*;")]
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

// https://github.com/ngtkana/ac-adapter-rs/blob/main/libs/lg/src/lib.rs
#[snippet(prefix = "use lg::*;")]
pub mod lg {
    use std::borrow::Borrow;
    use std::fmt;
    use std::iter::once;

    /// Print the values with the line number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use lg::*;
    /// let x = 42;
    /// let y = 43;
    /// lg!(x);
    /// lg!(x, y);
    /// lg!(42, x, 43, y);
    /// ```
    #[macro_export]
    macro_rules! lg {
    (@contents $head:expr $(, $tail:expr)*) => {{
        $crate::__lg_internal!($head);
        $(
            eprint!(",");
            $crate::__lg_internal!($tail);
        )*
        eprintln!();
    }};
    ($($expr:expr),* $(,)?) => {{
        eprint!("{}\u{276f}", line!());
        $crate::lg!(@contents $($expr),*)
    }};
}

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __lg_internal {
        ($value:expr) => {{
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
    macro_rules! rows {
    {
        $index_label:literal,
        $(@offset $offset:expr,)?
        $(@verticalbar $verticalbar:expr,)*
        $($(@$label:literal =>)? $values:expr),* $(,)?
    } => {{
        #![allow(unused_assignments)]
        let mut rows = $crate::Rows::default();
        rows.line_number(line!());
        $(rows.offset($offset);)?
        $(rows.verticalbar($verticalbar);)*
        rows.index_label($index_label);
        $({
            let mut label = stringify!($values).to_string();
            if label.starts_with("&") {
                label = label[1..].to_string();
            }
            $({
                let label_: &'static str = $label;
                label = label_.to_string();
            })?
            rows.row(label, $values);
        })*
        eprintln!("{}", rows.to_string_table());
    }};
}

    /// Print the 2D array with the line number.
    ///
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
    macro_rules! table {
    {
        $(@$name:literal => )? $values:expr $(,)?
    } => {{
        #![allow(unused_assignments)]
        let mut name = stringify!($values).to_string();
        if name.starts_with("&") {
            name = name[1..].to_string();
        }
        $({
            let name_: &'static str = $name;
            name = name_.to_string();
        })?
        let mut rows = $crate::Rows::default();
        rows.line_number(line!());
        rows.table_name(name);
        #[allow(array_into_iter)]
        for (i, row) in $values.into_iter().enumerate() {
            rows.row(i.to_string(), row);
        }
        eprintln!("{}", rows.to_string_table());
    }};
}

    #[doc(hidden)]
    pub fn __quiet(s: impl AsRef<str>) -> String {
        s.as_ref()
            .replace("340282366920938463463374607431768211455", "*") // u128
            .replace("170141183460469231731687303715884105727", "*") // i128
            .replace("18446744073709551615", "*") // u64
            .replace("9223372036854775807", "*") // i64
            .replace("-9223372036854775808", "*") // i64
            .replace("4294967295", "*") // u32
            .replace("2147483647", "*") // i32
            .replace("-2147483648", "*") // i32
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

            // Heading
            gray(f)?;
            write!(
                f,
                "{}",
                head.to_string(label_width, &value_width, verticalbar_count, true)
            )?;
            resetln(f)?;

            // Body
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

#[cfg(test)]
mod tests {
    use ndarray::{Array, Array2, Array3};

    use super::print_arr::*;
    use super::print_vec::*;

    #[test]
    fn test_print_arr() {
        let arr: Vec<i64> = vec![3; 4];

        print_arr(&arr);
    }

    #[test]
    fn test_print_arr2() {
        let arr: Array2<i64> = Array::from_shape_fn((2, 3), |_| 3);

        print_arr2(&arr);
    }

    #[test]
    fn test_print_arr3() {
        let arr: Array3<i64> = Array::from_shape_fn((2, 3, 4), |_| 3);

        print_arr3(&arr);
    }

    #[test]
    fn test_print_vec() {
        let arr = vec![1, 2, 3];
        print_vec(&arr);
    }

    #[test]
    fn test_print_vec_1line() {
        let arr = vec![1, 2, 3];
        print_vec_1line(&arr);
    }

    #[test]
    fn test_print_vec2() {
        let arr = vec![vec![1, 2, 3], vec![4, 5, 6]];
        print_vec2(&arr);
    }

    #[test]
    fn test_print_bytes() {
        let bs = b"hoge";
        print_bytes(bs);
    }

    #[test]
    fn test_print_vec_bytes() {
        let bs = vec![b"hoge".to_vec(), b"fuga".to_vec()];
        print_vec_bytes(&bs);
    }
}
