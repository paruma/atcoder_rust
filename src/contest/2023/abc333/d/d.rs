#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Edge {
    from: Usize1,
    to: Usize1,
}
impl Edge {
    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }
    pub fn rev(&self) -> Self {
        Self {
            from: self.to,
            to: self.from,
        }
    }
}
pub fn make_adj(n_vertex: usize, edges: &[Edge]) -> Vec<Vec<Edge>> {
    let mut adj = vec![vec![]; n_vertex];
    for &e in edges {
        adj[e.from].push(e);
        adj[e.to].push(e.rev());
    }
    adj
}
struct Problem {
    nv: usize,
    edges: Vec<Edge>,
}

struct DfsGraph1<'a> {
    adj: &'a Vec<Vec<Edge>>,
    visited: Vec<bool>,
    dp: Vec<i64>,
}

impl DfsGraph1<'_> {
    fn new(adj: &Vec<Vec<Edge>>) -> DfsGraph1<'_> {
        // adj.len() は グラフの頂点の数
        DfsGraph1 {
            adj,
            visited: vec![false; adj.len()],
            dp: vec![0_i64; adj.len()],
        }
    }
    /// 計算量: O(頂点の数 + 辺の数)
    fn exec(&mut self, v: usize) {
        // 行きがけ
        self.visited[v] = true;

        for &edge in &self.adj[v] {
            if !self.visited[edge.to] {
                self.exec(edge.to);
            }
        }
        // 帰りがけ

        // この場合分け不要
        if self.adj[v].len() == 1 {
            // 葉っぱ
            self.dp[v] = 1;
        } else {
            self.dp[v] = self.adj[v]
                .iter()
                .copied()
                .map(|e| self.dp[e.to])
                .sum::<i64>()
                + 1;
        }
    }
}

fn post_order(adj: &Vec<Vec<Edge>>, init: usize) -> Vec<usize> {
    struct DfsGraph<'a> {
        adj: &'a Vec<Vec<Edge>>,
        visited: Vec<bool>,
        post_order: Vec<usize>,
    }

    impl DfsGraph<'_> {
        fn new(adj: &Vec<Vec<Edge>>) -> DfsGraph<'_> {
            // adj.len() は グラフの頂点の数
            DfsGraph {
                adj,
                visited: vec![false; adj.len()],
                post_order: vec![],
            }
        }
        /// 計算量: O(頂点の数 + 辺の数)
        fn exec(&mut self, v: usize) {
            // 行きがけ
            self.visited[v] = true;

            for &edge in &self.adj[v] {
                if !self.visited[edge.to] {
                    self.exec(edge.to);
                }
            }
            // 帰りがけ
            self.post_order.push(v);
        }
    }
    let mut dfs = DfsGraph::new(adj);
    dfs.exec(init);
    dfs.post_order
}

impl Problem {
    fn read() -> Problem {
        input! {
            nv: usize,
            edges: [Edge; nv-1],
        }
        Problem { nv, edges }
    }
    fn solve(&self) -> Answer {
        let nv = self.nv;
        let edges = &self.edges;
        let adj = make_adj(nv, &edges);
        let mut dfs = DfsGraph1::new(&adj);
        dfs.exec(0);

        let next_0_dp = adj[0].iter().copied().map(|e| dfs.dp[e.to]).collect_vec();

        let ans = next_0_dp.iter().sum::<i64>() - next_0_dp.iter().copied().max().unwrap() + 1;
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // DFS 帰りがけ順を取得してからDPをする
        let nv = self.nv;
        let edges = &self.edges;
        let adj = make_adj(nv, &edges);
        let post_order = post_order(&adj, 0);
        let mut dp = vec![0_i64; nv];
        for v in post_order {
            dp[v] = adj[v].iter().copied().map(|e| dp[e.to]).sum::<i64>() + 1;
        }

        let next_0_dp = adj[0].iter().copied().map(|e| dp[e.to]).collect_vec();

        let ans = next_0_dp.iter().sum::<i64>() - next_0_dp.iter().copied().max().unwrap() + 1;
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: i64,
}

impl Answer {
    fn print(&self) {
        println!("{}", self.ans);
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
