// 23:37
//#[derive_readable]
enum Query {
    Add(i64, i64),
    Remove(i64),
}

struct Problem {
    n: usize,
    xs: Vec<i64>,
    nq: usize,
    qs: Vec<Query>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            xs: [i64; n],
            nq: usize,
        }

        let qs = (0..nq)
            .map(|_| {
                input! {
                    query_type: usize
                }

                if query_type == 1 {
                    input! {
                        x: i64,
                        y: i64
                    }
                    Query::Add(x, y)
                } else {
                    input! {
                        x: i64
                    }
                    Query::Remove(x)
                }
            })
            .collect_vec();

        Problem { n, xs, nq, qs }
    }
    fn solve(&self) -> Answer {
        let ans = 0;
        let xs = &self.xs;
        let mut next: HashMap<i64, i64> = HashMap::new();
        let mut prev: HashMap<i64, i64> = HashMap::new();
        // -1 を先頭を表すダミーとする。
        // -2 を末尾を表すダミーとする。

        next.insert(-1, xs[0]);
        prev.insert(xs[0], -1);
        for i in 0..xs.len() - 1 {
            next.insert(xs[i], xs[i + 1]);
            prev.insert(xs[i + 1], xs[i]);
        }

        next.insert(xs[xs.len() - 1], -2);
        prev.insert(-2, xs[xs.len() - 1]);

        for q in &self.qs {
            match q {
                Query::Add(x, y) => {
                    let old_next_x = next[x];
                    *prev.get_mut(&old_next_x).unwrap() = *y;
                    *next.get_mut(x).unwrap() = *y;
                    next.insert(*y, old_next_x);
                    prev.insert(*y, *x);
                }
                Query::Remove(x) => {
                    // lg!(*x);
                    // lg!(&prev);
                    // lg!(&next);
                    let old_prev_x = prev[x];
                    let old_next_x = next[x];
                    prev.remove(x);
                    next.remove(x);

                    *next.get_mut(&old_prev_x).unwrap() = old_next_x;
                    *prev.get_mut(&old_next_x).unwrap() = old_prev_x;
                }
            }
        }

        let mut ans = vec![];
        let mut val = -1;

        loop {
            val = next[&val];
            if val == -2 {
                break;
            }
            ans.push(val);
        }

        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<i64>,
}

impl Answer {
    fn print(&self) {
        print_vec_1line(&self.ans);
    }
}

fn main() {
    Problem::read().solve().print();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }
}

use std::collections::HashMap;

// ====== import ======
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};

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
#[allow(unused_imports)]
use lg::*;
// https://github.com/ngtkana/ac-adapter-rs/blob/main/libs/lg/src/lib.rs
pub mod lg {
    use std::borrow::Borrow;
    use std::fmt;
    use std::marker::PhantomData;

    #[macro_export]
    macro_rules! lg {
    (@contents $head:expr $(, $tail:expr)*) => {{
        $crate::__lg_variable!($head);
        $(
            eprint!(",");
            $crate::__lg_variable!($tail);
        )*
        eprintln!();
    }};
    ($($expr:expr),* $(,)?) => {{
        eprint!("{}❯", line!());
        $crate::lg!(@contents $($expr),*)
    }};
}

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __lg_variable {
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

    #[macro_export]
    macro_rules! table {
        ($value:expr) => {{
            $crate::Table::new($value).title(stringify!($value))
        }};
    }

    #[doc(hidden)]
    pub fn __quiet(s: impl AsRef<str>) -> String {
        s.as_ref()
            .replace("18446744073709551615", "*") // u64
            .replace("9223372036854775807", "*") // i64
            .replace("-9223372036854775808", "*") // i64
            .replace("4294967295", "*") // u32
            .replace("2147483647", "*") // i32
            .replace("-2147483648", "*") // i32
            .replace("None", "*")
            .replace("Some", "")
    }

    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Table<T, Row, Storage> {
        __marker: PhantomData<(T, Row)>,
        title: String,
        storage: Storage,
        index_width: usize,
        column_width: usize,
        heading_newlines: usize,
    }

    /// Format a two dimensional container in a table style.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use lg::{table, Table};
    /// let a = vec![vec![0, 1, 2], vec![3, 4, 5]];
    ///
    /// eprintln!(
    ///     "{}",
    ///     table(&a) // Either a or &a is ok.
    ///         .heading_newlines(1) // Default: 1
    ///         .index_width(1) // Default: 2
    ///         .column_width(2), // Default: 3
    /// );
    /// ```
    ///
    ///
    /// # Automatic quieting
    ///
    /// ```
    /// # use lg::{table, Table};
    /// eprintln!("{}", table(&[[0, 2147483647, 2], [3, 4, 5],]),);
    /// ```
    pub fn table<T: Clone + fmt::Debug, Row: AsRef<[T]>, Storage: AsRef<[Row]>>(
        storage: Storage,
    ) -> Table<T, Row, Storage> {
        Table::new(storage)
    }
    impl<T, Row, Storage> Table<T, Row, Storage>
    where
        T: Clone + fmt::Debug,
        Row: AsRef<[T]>,
        Storage: AsRef<[Row]>,
    {
        pub fn new(storage: Storage) -> Self {
            Self {
                __marker: PhantomData,
                title: String::new(),
                storage,
                column_width: 3,
                index_width: 2,
                heading_newlines: 1,
            }
        }

        pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
            self.title = title.into();
            self
        }

        pub fn index_width(&mut self, index_width: usize) -> &mut Self {
            self.index_width = index_width;
            self
        }

        pub fn column_width(&mut self, column_width: usize) -> &mut Self {
            self.column_width = column_width;
            self
        }

        pub fn heading_newlines(&mut self, heading_newlines: usize) -> &mut Self {
            self.heading_newlines = heading_newlines;
            self
        }
    }
    impl<T, Row, Storage> fmt::Display for Table<T, Row, Storage>
    where
        T: Clone + fmt::Debug,
        Row: AsRef<[T]>,
        Storage: AsRef<[Row]>,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let Self {
                __marker: _,
                ref title,
                ref storage,
                index_width,
                column_width,
                heading_newlines,
            } = *self;
            for _ in 0..heading_newlines {
                writeln!(f)?;
            }
            writeln!(f, "{}❯ {}", line!(), title)?;
            let ncols = storage.as_ref()[0].as_ref().len();
            write!(f, "\x1b[48;2;127;127;127;37m")?;
            write!(f, "{}|", " ".repeat(index_width))?;
            for j in 0..ncols {
                write!(f, "{j:column_width$}")?;
            }
            writeln!(f, "\x1b[0m")?;
            for (i, row) in storage.as_ref().iter().enumerate() {
                write!(f, "{:index_width$}|", i, index_width = index_width)?;
                for value in row.as_ref() {
                    write!(f, "{:>column_width$}", __quiet(format!("{:?}", value)),)?;
                }
                writeln!(f)?;
            }
            Ok(())
        }
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

// ====== snippet ======
