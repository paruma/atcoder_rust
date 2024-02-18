enum Query {
    Update {
        // x 文字目を c に変更する
        x: usize,
        c: u8,
    },
    Find {
        // l 文字目から r文字目までが回分か判定する
        l: usize,
        r: usize,
    },
}
//#[derive_readable]
struct Problem {
    str_len: usize,
    nq: usize,
    str: Vec<u8>,
    qs: Vec<Query>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            str_len: usize,
            nq: usize,
            str: Bytes,
        }
        let qs = (0..nq)
            .map(|_| {
                input! {
                    q: u8
                }
                if q == 1 {
                    input! {
                        x: Usize1,
                        c: Bytes
                    }
                    Query::Update { x, c: c[0] }
                } else {
                    input! {
                        l: Usize1,
                        r: Usize1
                    }
                    Query::Find { l, r }
                }
            })
            .collect_vec();
        Problem { str_len, nq, str, qs }
    }
    fn solve(&self) -> Answer {
        use ac_library::segtree::Segtree;
        use ac_library::ModInt998244353 as Mint;
        let Problem { str_len, nq, str, qs } = self;

        let ctoi = |c: u8| c - b'a';

        let base_list: [i64; 3] = [100_000_001, 321_432_543, 987_654_321];
        let make_rh = |ch: u8, base: i64| RollingHash::new(Mint::new(ctoi(ch)), Mint::new(base));

        let rev_i = |i: usize| str_len - 1 - i;

        // 文字列の正順と逆順
        let normal_rh =
            base_list.map(|base| str.iter().copied().map(|ch| make_rh(ch, base)).collect_vec());
        let reverse_rh = base_list
            .map(|base| str.iter().copied().rev().map(|ch| make_rh(ch, base)).collect_vec());
        let mut normal_seg_trees = normal_rh.map(Segtree::<RollingHashConcat<Mint>>::from);
        let mut reverse_seg_trees = reverse_rh.map(Segtree::<RollingHashConcat<Mint>>::from);

        let mut ans = vec![];
        for q in qs {
            match q {
                Query::Update { x, c } => {
                    for (seg_tree, base) in izip!(&mut normal_seg_trees, base_list) {
                        seg_tree.set(*x, make_rh(*c, base));
                    }
                    for (seg_tree, base) in izip!(&mut reverse_seg_trees, base_list) {
                        seg_tree.set(rev_i(*x), make_rh(*c, base));
                    }
                }
                Query::Find { l, r } => {
                    let is_palindrome = izip!(&normal_seg_trees, &reverse_seg_trees).all(
                        |(normal_seg_tree, reverse_seg_tree)| {
                            let rev_l = rev_i(*l);
                            let rev_r = rev_i(*r);
                            normal_seg_tree.prod(l..=r) == reverse_seg_tree.prod(rev_r..=rev_l)
                        },
                    );
                    ans.push(is_palindrome);
                }
            }
        }

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // ロリハの直積
        use ac_library::segtree::Segtree;
        use ac_library::ModInt998244353 as Mint;
        let Problem { str_len, nq, str, qs } = self;

        let ctoi = |c: u8| c - b'a';

        let base_list: [i64; 3] = [100_000_001, 321_432_543, 987_654_321];
        let make_rh =
            |ch: u8| base_list.map(|base| RollingHash::new(Mint::new(ctoi(ch)), Mint::new(base)));

        let rev_i = |i: usize| str_len - 1 - i;

        // 文字列の正順と逆順
        let normal_rh = str.iter().copied().map(|ch| make_rh(ch)).collect_vec();
        let reverse_rh = str.iter().copied().rev().map(|ch| make_rh(ch)).collect_vec();

        let mut normal_seg_tree = Segtree::<RollingHash3Concat<Mint>>::from(normal_rh);
        let mut reverse_seg_tree = Segtree::<RollingHash3Concat<Mint>>::from(reverse_rh);

        let mut ans = vec![];
        for q in qs {
            match q {
                Query::Update { x, c } => {
                    normal_seg_tree.set(*x, make_rh(*c));
                    reverse_seg_tree.set(rev_i(*x), make_rh(*c));
                }
                Query::Find { l, r } => {
                    let rev_l = rev_i(*l);
                    let rev_r = rev_i(*r);
                    let is_palindrome =
                        normal_seg_tree.prod(l..=r) == reverse_seg_tree.prod(rev_r..=rev_l);
                    ans.push(is_palindrome);
                }
            }
        }

        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<bool>,
}

impl Answer {
    fn print(&self) {
        for a in &self.ans {
            print_yesno(*a);
        }
    }
}

fn main() {
    Problem::read().solve2().print();
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

use ac_library::segtree;
use itertools::izip;
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
            iter.into_iter().map(|b| ['.', '#'][usize::from(*(b.borrow()))]).collect::<String>(),
        )
    }
}

// ====== snippet ======
use monoid_rolling_hash::*;
pub mod monoid_rolling_hash {
    use ac_library::Monoid;
    use std::{
        convert::Infallible,
        marker::PhantomData,
        ops::{Add, Mul},
    };
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RollingHash<T> {
        hash: T,
        base: T,
    }
    impl<T> RollingHash<T> {
        pub fn new(hash: T, base: T) -> Self {
            Self { hash, base }
        }
        pub fn identity() -> Self
        where
            T: From<i64>,
        {
            Self { hash: 0.into(), base: 1.into() }
        }
        pub fn concat(&self, rhs: &Self) -> Self
        where
            T: Copy + Mul<Output = T> + Add<Output = T>,
        {
            Self { hash: self.hash * rhs.base + rhs.hash, base: self.base * rhs.base }
        }
    }
    pub struct RollingHashConcat<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for RollingHashConcat<T>
    where
        T: Copy + From<i64> + Add<Output = T> + Mul<Output = T>,
    {
        type S = RollingHash<T>;
        fn identity() -> Self::S {
            RollingHash::identity()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a.concat(b)
        }
    }

    pub struct RollingHash3Concat<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for RollingHash3Concat<T>
    where
        T: Copy + From<i64> + Add<Output = T> + Mul<Output = T>,
    {
        type S = [RollingHash<T>; 3];
        fn identity() -> Self::S {
            [RollingHash::identity(); 3]
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            [0, 1, 2].map(|i| a[i].concat(&b[i]))
        }
    }
}
