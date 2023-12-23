struct Grid {
    grid: Vec<Vec<u8>>,
    h: usize,
    w: usize,
}

impl Grid {
    fn new(grid: Vec<Vec<u8>>) -> Grid {
        let h = grid.len();
        let w = grid[0].len();
        Grid { grid, h, w }
    }

    fn is_within(&self, pos: Pos<i64>) -> bool {
        let h = self.h as i64;
        let w = self.w as i64;
        0 <= pos.y && pos.y < h && 0 <= pos.x && pos.x < w
    }

    fn at(&self, pos: Pos<i64>) -> &u8 {
        if self.is_within(pos) {
            self.grid.at(pos)
        } else {
            &b'_'
        }
    }

    fn is_green(&self, pos: Pos<i64>) -> bool {
        *self.at(pos) == b'#'
    }

    fn is_red(&self, pos: Pos<i64>) -> bool {
        *self.at(pos) == b'.'
    }

    fn encode(&self, pos: Pos<i64>) -> usize {
        (pos.y * self.w as i64 + pos.x) as usize
    }

    // fn at_mut(&mut self, pos: Pos<i64>) -> &mut u8 {
    //     self.grid.at_mut(pos)
    // }
}
//#[derive_readable]
struct Problem {
    grid: Grid,
}

use ac_library::ModInt998244353 as Mint;

impl Problem {
    fn read() -> Problem {
        input! {
            h: usize,
            w: usize,
            grid: [Bytes; h],
        }
        let grid = Grid::new(grid);
        Problem { grid }
    }
    fn solve(&self) -> Answer {
        let grid = &self.grid;
        let w = grid.w;
        let h = grid.h;
        let mut uf = DisjointSetUnionRollback::new(w * h);

        for y in 0..h {
            for x in 0..w {
                let pos = Pos::new(x as i64, y as i64);
                if grid.is_red(pos) {
                    continue;
                }

                for next in DIR4_LIST.iter().copied().map(|d| d + pos) {
                    if grid.is_green(next) {
                        uf.unite(grid.encode(pos), grid.encode(next));
                    }
                }
            }
        }
        let cnt_red = iproduct!(0..h, 0..w)
            .filter(|&(y, x)| {
                let pos = Pos::new(x as i64, y as i64);
                grid.is_red(pos)
            })
            .count();

        uf.snapshot();

        let base_cnt = (uf.get_all_groups().len() - (cnt_red - 1)) as i64;

        let mut cnt = 0; // 答えの分母

        for y in 0..h {
            for x in 0..w {
                let pos = Pos::new(x as i64, y as i64);
                if grid.is_red(pos) {
                    let mut unite_cnt = 0;

                    for next in DIR4_LIST.iter().copied().map(|d| d + pos) {
                        if grid.is_green(next) && uf.unite(grid.encode(pos), grid.encode(next)) {
                            unite_cnt += 1;
                        }
                    }
                    // (x, y) を緑にしたときの連結成分数
                    let cnt_sub = base_cnt - unite_cnt;
                    cnt += cnt_sub;

                    uf.rollback_snapshot()

                    // lg!(unite_cnt);
                    // for _ in 0..unite_cnt {
                    //     uf.undo();
                    // }
                }
            }
        }

        let ans = Mint::new(cnt) / Mint::new(cnt_red as i64);
        let ans = ans.val() as i64;
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

// ====== import ======
use itertools::iproduct;
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

use pos::*;
pub mod pos {
    use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Pos<T> {
        pub x: T,
        pub y: T,
    }
    impl<T> Pos<T> {
        pub fn new(x: T, y: T) -> Pos<T> {
            Pos { x, y }
        }
    }
    impl<T: Mul<Output = T> + Copy> Pos<T> {
        pub fn scala_mul(self, rhs: T) -> Pos<T> {
            Pos::new(self.x * rhs, self.y * rhs)
        }
    }
    impl<T: Add<Output = T> + Mul<Output = T> + Copy> Pos<T> {
        pub fn norm_square(self) -> T {
            self.x * self.x + self.y * self.y
        }
    }
    impl<T: Add<Output = T> + Copy> Add for Pos<T> {
        type Output = Pos<T>;
        fn add(self, rhs: Self) -> Self::Output {
            Pos::new(self.x + rhs.x, self.y + rhs.y)
        }
    }
    impl<T: Sub<Output = T> + Copy> Sub for Pos<T> {
        type Output = Pos<T>;
        fn sub(self, rhs: Self) -> Self::Output {
            Pos::new(self.x - rhs.x, self.y - rhs.y)
        }
    }
    impl<T: Neg<Output = T>> Neg for Pos<T> {
        type Output = Self;
        fn neg(self) -> Self::Output {
            Pos::new(-self.x, -self.y)
        }
    }
    impl<T: num_traits::Zero + Copy> num_traits::Zero for Pos<T> {
        fn zero() -> Self {
            Pos::new(T::zero(), T::zero())
        }
        fn is_zero(&self) -> bool {
            self.x.is_zero() && self.y.is_zero()
        }
    }
    impl<T: Add<Output = T> + Copy> AddAssign for Pos<T> {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }
    impl<T: Sub<Output = T> + Copy> SubAssign for Pos<T> {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }
    pub const DIR8_LIST: [Pos<i64>; 8] = [
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 1 },
        Pos { x: 1, y: 0 },
        Pos { x: 1, y: -1 },
        Pos { x: 0, y: -1 },
        Pos { x: -1, y: -1 },
        Pos { x: -1, y: 0 },
        Pos { x: -1, y: 1 },
    ];
    pub const DIR4_LIST: [Pos<i64>; 4] =
        [Pos { x: 0, y: 1 }, Pos { x: 1, y: 0 }, Pos { x: 0, y: -1 }, Pos { x: -1, y: 0 }];
}

use undo_uf::DisjointSetUnionRollback;
use vec_vec_at::*;
pub mod vec_vec_at {
    use super::pos::*;
    use easy_ext::ext;
    #[ext]
    impl<T> Vec<Vec<T>> {
        pub fn at(&self, pos: Pos<i64>) -> &T {
            &self[pos.y as usize][pos.x as usize]
        }
        pub fn at_mut(&mut self, pos: Pos<i64>) -> &mut T {
            &mut self[pos.y as usize][pos.x as usize]
        }
    }
}

// https://github.uesugi.app/competitive-library/doc/src/competitive_library/structure/disjoint_set_union_undo.rs.html#1-183
pub mod undo_uf {
    //! Union find undo
    use std::collections::{HashMap, HashSet, VecDeque};
    #[derive(Debug, Clone)]
    enum Node {
        Root(usize, usize),
        Child(usize),
    }
    /// UnionFind
    /// 経路圧縮を行わないことで undo を可能にする
    #[derive(Clone, Debug)]
    pub struct DisjointSetUnionRollback {
        uf: Vec<Node>,
        history: VecDeque<(usize, Node)>,
        restore_point: Option<usize>,
    }

    impl DisjointSetUnionRollback {
        /// 要素数 n の dsu を構築する
        #[inline]
        pub fn new(n: usize) -> DisjointSetUnionRollback {
            DisjointSetUnionRollback {
                uf: vec![Node::Root(1, 1); n],
                history: VecDeque::new(),
                restore_point: None,
            }
        }

        /// 根を取得
        /// 経路圧縮を行わない
        #[inline]
        pub fn root(&self, target: usize) -> usize {
            match self.uf[target] {
                Node::Root(_, _) => target,
                Node::Child(par) => self.root(par),
            }
        }

        /// 対象の木をマージ
        /// 経路圧縮を行わないため変更されるノード数は高々2
        /// 変更箇所をスタックで保存
        #[inline]
        pub fn unite(&mut self, x: usize, y: usize) -> bool {
            let rx = self.root(x);
            let ry = self.root(y);
            if rx == ry {
                return false;
            }
            self.history.push_back((rx, self.uf[rx].clone()));
            self.history.push_back((ry, self.uf[ry].clone()));
            let size_x = self.size(rx);
            let size_y = self.size(ry);
            let rank_x = self.rank(rx);
            let rank_y = self.rank(ry);
            let (i, j) = if rank_x > rank_y { (rx, ry) } else { (ry, rx) };
            self.uf[i] =
                Node::Root(size_x + size_y, (rank_x.min(rank_y) + 1).max(rank_x.max(rank_y)));
            self.uf[j] = Node::Child(i);

            true
        }

        /// 同じ木に存在するか
        #[inline]
        pub fn is_same(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
        }

        /// 所属する木のサイズ
        pub fn size(&mut self, x: usize) -> usize {
            let root = self.root(x);
            match self.uf[root] {
                Node::Root(size, _) => size,
                Node::Child(_) => 1,
            }
        }
        /// 所属する木のランク
        #[inline]
        pub fn rank(&mut self, x: usize) -> usize {
            let root = self.root(x);
            match self.uf[root] {
                Node::Root(_, rank) => rank,
                Node::Child(_) => 1,
            }
        }

        /// unite 操作の undo
        #[inline]
        pub fn undo(&mut self) {
            for _ in 0..2 {
                let (index, node) = self.history.pop_back().unwrap();
                self.uf[index] = node;
            }
        }

        /// 現時点の状態を保存
        /// 復元には rollback_snapshot
        #[inline]
        pub fn snapshot(&mut self) {
            self.restore_point = Some(self.history.len() >> 1);
        }

        /// 現時点での保存されている操作回数を返す
        #[inline]
        pub fn get_history_length(&self) -> usize {
            self.history.len() >> 1
        }

        /// rollback_snapshot で保存された状態へ復元
        #[inline]
        pub fn rollback_snapshot(&mut self) {
            self.rollback(self.restore_point.unwrap());
        }

        /// 復元
        /// 任意のタイミングで get_history_length を実行し取得した 値を使用する
        #[inline]
        pub fn rollback(&mut self, n: usize) {
            assert!(self.history.len() >= n << 1);

            while self.history.len() > n << 1 {
                self.undo();
            }
        }

        /// 同じ木に含まれるノードを返す
        #[inline]
        pub fn get_same_group(&mut self, x: usize) -> HashSet<usize> {
            let root = self.root(x);
            let mut g = HashSet::new();
            for i in 0..self.uf.len() {
                if root == self.root(i) {
                    g.insert(i);
                }
            }
            g
        }

        /// 全ノードを返却
        #[inline]
        pub fn get_all_groups(&mut self) -> HashMap<usize, HashSet<usize>> {
            let mut map: HashMap<usize, HashSet<usize>> = HashMap::new();
            for i in 0..self.uf.len() {
                let root = self.root(i);

                map.entry(root).or_insert_with(HashSet::new).insert(i);
            }
            map
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_dsu_rollback() {
            let mut dsu = DisjointSetUnionRollback::new(6);

            dsu.unite(0, 1);
            assert!(dsu.is_same(0, 1));
            dsu.unite(1, 2);
            assert!(dsu.is_same(0, 2));
            assert_eq!(dsu.size(0), 3);
            assert!(!dsu.is_same(0, 3));
            dsu.snapshot();
            dsu.unite(0, 3);
            dsu.unite(3, 4);
            dsu.unite(4, 5);
            assert_eq!(dsu.size(5), 6);
            assert!(dsu.is_same(0, 5));
            dsu.undo();
            assert!(!dsu.is_same(0, 5));
            dsu.rollback_snapshot();
            assert!(dsu.is_same(0, 2));
            assert_eq!(dsu.size(0), 3);
            assert!(!dsu.is_same(0, 3));
            dsu.rollback(0);
            assert!(!dsu.is_same(0, 1));
            assert_eq!(dsu.get_history_length(), 0);
        }
    }
}
