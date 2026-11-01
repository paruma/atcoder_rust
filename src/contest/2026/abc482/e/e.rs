// 問題文と制約は読みましたか？
// #[fastout]
/// 無向グラフの辺集合から隣接リストを作成します。
///
/// # 計算量
/// O(V + E)
pub fn make_adj_from_undirected(n_vertex: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n_vertex];

    for &(from, to) in edges {
        adj[from].push(to);
        adj[to].push(from);
    }

    adj
}

// (src, dst) が辺で繋がっている前提で、src→dst 方向の部分木 (dst が根)の根からの距離を求める
fn calc_dist(n: usize, adj: &[Vec<usize>], src: usize, dst: usize) -> Vec<usize> {
    let mut open: Queue<usize> = Queue::new();
    let mut visited = vec![false; n];
    let mut dist = vec![usize::MAX; n];
    visited[dst] = true;
    visited[src] = true;
    open.push(dst);
    dist[dst] = 0;

    while let Some(current) = open.pop() {
        for &to in &adj[current] {
            if !visited[to] {
                visited[to] = true;
                open.push(to);
                dist[to] = dist[current] + 1;
            }
        }
    }

    dist
}

fn main() {
    input! {
        n: usize,
        es: [(Usize1, Usize1); n - 1],
        cs: [Usize1; n],
    }

    let adj = make_adj_from_undirected(n, &es);

    let (d, diam) = tree_diameter_no_weight(&es);
    let d = d as usize;

    let ans = if d % 2 == 1 {
        // 0 1 [2 3] 4 5 (d=5)
        let c1 = diam[d / 2];
        let c2 = diam[d / 2 + 1];

        // c2 → c1 方向の部分木のサイズ
        let cnt1 = {
            let dist = calc_dist(n, &adj, c2, c1);

            let mut cnt_by_color = vec![0; n];
            for v in 0..n {
                if dist[v] == d / 2 {
                    cnt_by_color[cs[v]] += 1;
                }
            }
            cnt_by_color
        };

        // c1 → c2 方向の部分木のサイズ
        let cnt2 = {
            let dist = calc_dist(n, &adj, c1, c2);

            let mut cnt_by_color = vec![0; n];
            for v in 0..n {
                if dist[v] == d / 2 {
                    cnt_by_color[cs[v]] += 1;
                }
            }

            cnt_by_color
        };
        (0..n).map(|c| cnt1[c] * cnt2[c]).sum::<usize>()
    } else {
        let c = diam[d / 2];

        // (頂点, 部分木のid(eid)) を格納する。部分木のidは格納しなくても eids から計算できたので不要だった
        let mut open: Queue<(usize, usize)> = Queue::new();
        let mut visited = vec![false; n];
        let mut dist = vec![usize::MAX; n];
        let mut eids = vec![usize::MAX; n]; // c からのどの部分木に属しているか
        visited[c] = true;
        dist[c] = 0;
        for (eid, init) in adj[c].iter().copied().enumerate() {
            open.push((init, eid));
            dist[init] = 1;
            visited[init] = true;
            eids[init] = eid;
        }

        while let Some((cur, eid)) = open.pop() {
            for &next in &adj[cur] {
                if !visited[next] {
                    visited[next] = true;
                    open.push((next, eid));
                    eids[next] = eid; // ここは eids[next] = eids[cur] でよかった
                    dist[next] = dist[cur] + 1;
                }
            }
        }

        let by_color = cs
            .iter()
            .copied()
            .enumerate()
            .fold(vec![vec![]; n], |mut acc, (v, c)| {
                acc[c].push(v);
                acc
            });

        by_color
            .iter()
            .map(|g| {
                // 計算量に注意！！！！
                // 部分木 (eid) ごとにカウント
                let mut cnts: HashMap<usize, usize> = HashMap::new();
                for &v in g {
                    if dist[v] == d / 2 {
                        *cnts.entry(eids[v]).or_default() += 1;
                    }
                }
                let cnts = cnts.values().copied().collect_vec();
                let sum = cnts.iter().copied().sum::<usize>();
                let sum_sq = cnts.iter().copied().map(|x| x * x).sum::<usize>();
                (sum * sum - sum_sq) / 2
            })
            .sum::<usize>()
    };

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
// 使用したテンプレート・スニペットの公開元:
// https://github.com/paruma/atcoder_rust
use mod_queue::*;
/// 木の直径を求める(直径の長さと直径を構成する頂点のリストを返す)
///
/// # Arguments
/// * `edges` - 辺の情報 (頂点, 頂点, コスト) のリスト
///
/// # Returns
/// `(直径の長さ, 直径を構成する頂点のリスト)`
///
/// # 計算量
/// O(V) (V は頂点数)
pub fn tree_diameter(edges: &[(usize, usize, i64)]) -> (i64, Vec<usize>) {
    let nv = edges.len() + 1;
    let adj = edges
        .iter()
        .copied()
        .fold(vec![vec![]; nv], |mut acc, (u, v, cost)| {
            acc[u].push((v, cost));
            acc[v].push((u, cost));
            acc
        });
    // init から最も遠い点までの距離と、init から最も遠い点までいくのに訪問する頂点のリストを返す
    fn bfs(adj: &[Vec<(usize, i64)>], init: usize) -> (i64, Vec<usize>) {
        let n = adj.len(); // 頂点の数

        let mut dist = vec![0; n];
        let mut prev = vec![None; n];
        let mut visited = vec![false; n];
        let mut open = Queue::new();
        open.push(init);
        visited[init] = true;

        while let Some(current) = open.pop() {
            for &(next, cost) in &adj[current] {
                if !visited[next] {
                    dist[next] = dist[current] + cost;
                    prev[next] = Some(current);
                    visited[next] = true;
                    open.push(next);
                }
            }
        }
        let (furthest, max_dist) = dist
            .iter()
            .copied()
            .enumerate()
            .max_by_key(|(_, d)| *d)
            .unwrap();
        // 復元
        let path: Vec<usize> = {
            let mut path: Vec<usize> =
                std::iter::successors(Some(furthest), |&i| prev[i]).collect();
            path.reverse();
            path
        };

        (max_dist, path)
    }

    // 頂点 0 から最も遠い点 x を求める
    let x = *bfs(&adj, 0).1.last().unwrap();

    // 頂点 x から最も遠い点 y までの距離を求める
    bfs(&adj, x)
}

pub mod mod_queue {
    use std::collections::VecDeque;

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct Queue<T> {
        raw: VecDeque<T>,
    }

    impl<T> Queue<T> {
        pub fn new() -> Self {
            Queue {
                raw: VecDeque::new(),
            }
        }
        pub fn push(&mut self, value: T) {
            self.raw.push_back(value)
        }

        pub fn pop(&mut self) -> Option<T> {
            self.raw.pop_front()
        }

        pub fn peek(&self) -> Option<&T> {
            self.raw.front()
        }

        pub fn is_empty(&self) -> bool {
            self.raw.is_empty()
        }

        pub fn len(&self) -> usize {
            self.raw.len()
        }
    }

    impl<T> Default for Queue<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}
/// 重みなし木の直径を求める(直径の長さと直径を構成する頂点のリストを返す)
///
/// # Arguments
/// * `edges` - 辺の情報 (頂点, 頂点) のリスト
///
/// # Returns
/// `(直径の長さ, 直径を構成する頂点のリスト)`
///
/// # 計算量
/// O(V) (V は頂点数)
pub fn tree_diameter_no_weight(edges: &[(usize, usize)]) -> (i64, Vec<usize>) {
    let edges: Vec<(usize, usize, i64)> = edges.iter().copied().map(|(u, v)| (u, v, 1)).collect();

    tree_diameter(&edges)
}
use lg::*;
// https://github.com/ngtkana/ac-adapter-rs/blob/main/libs/lg/src/lib.rs
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
    macro_rules! lg {
    (@contents $head:expr_2021 $(, $tail:expr_2021)*) => {{
        $crate::__lg_internal!($head);
        $(
            eprint!(",");
            $crate::__lg_internal!($tail);
        )*
        eprintln!();
    }};
    ($($expr:expr_2021),* $(,)?) => {{
        eprint!("{}\u{276f}", line!());
        $crate::lg!(@contents $($expr),*)
    }};
}

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __lg_internal {
        ($value:expr_2021) => {{
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
    macro_rules! rows {
    {
        $index_label:literal,
        $(@offset $offset:expr_2021,)?
        $(@verticalbar $verticalbar:expr_2021,)*
        $($(@$label:literal =>)? $values:expr_2021),* $(,)?
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
    macro_rules! table {
    {
        $(@$name:literal => )? $values:expr_2021 $(,)?
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
