//#[derive_readable]
struct Problem {
    n_case: usize,
    cases: Vec<TestCase>,
}

struct TestCase {
    k: i64,
}

fn to_decimal(x: i64) -> Vec<i64> {
    let mut res = vec![];
    let mut x = x;
    while x > 0 {
        res.push(x % 10);
        x /= 10;
    }
    res.reverse();
    res
}

struct Dp {
    dp: Vec<Vec<i64>>,
}

impl Dp {
    fn new(n_digit: usize) -> Self {
        Self {
            dp: vec![vec![0; 2]; n_digit + 1],
        }
    }

    fn at(&self, digit_i: usize, smaller: bool) -> &i64 {
        &self.dp[digit_i][smaller as usize]
    }

    fn at_mut(&mut self, digit_i: usize, smaller: bool) -> &mut i64 {
        &mut self.dp[digit_i][smaller as usize]
    }
}

impl TestCase {
    fn is_neq_num(x: i64) -> bool {
        let k = to_decimal(x);
        k.iter().tuple_windows().all(|(x, y)| x != y)
    }

    fn cnt_neq_num(x: i64) -> i64 {
        // [1, x] での neq number の数を数える。
        // k個あるとき、x が neq number なら x は k 番目の neq number である。
        let k = to_decimal(x);
        let mut dp = Dp::new(k.len());

        *dp.at_mut(0, false) = 1;

        for digit_i in 0..k.len() {
            let dp_true = *dp.at(digit_i, true);
            let dp_false = *dp.at(digit_i, false);

            // 00..00 の場合は0から9のどれを追加してもよい(10通り)
            // それ以外の場合は末尾の値以外の値を追加(9通り)
            if digit_i != 0 {
                *dp.at_mut(digit_i + 1, true) += 10 + (dp_true - 1) * 9;
            }

            *dp.at_mut(digit_i + 1, true) += if digit_i == 0 {
                dp_false * k[digit_i]
            } else if (0..k[digit_i]).contains(&k[digit_i - 1]) {
                dp_false * (k[digit_i] - 1)
            } else {
                dp_false * k[digit_i]
            };

            if digit_i == 0 || k[digit_i - 1] != k[digit_i] {
                *dp.at_mut(digit_i + 1, false) += dp_false;
            }
        }

        *dp.at(k.len(), true) + dp.at(k.len(), false) - 1
    }

    fn solve(&self) -> i64 {
        // 解法: 桁DP & 二分探索
        let prod = |x: i64| {
            // 答え(k番目のneq number) 以上かどうか
            Self::cnt_neq_num(x) >= self.k
        };
        // 10^17
        bin_search(100_000_000_000_000_000, 0, prod)
    }

    fn cnt_neq_num2(x: i64) -> i64 {
        // [1, x] の neq number の数を求める
        // コメントでは、x=382 を例にする

        let ds = to_decimal(x + 1);
        let ndigits = ds.len();

        // [1, 100) での neq number の数
        // = [1, 10) での neq number + [10, 100) での neq number
        // = 9 + 9^2
        let term1 = (1..ndigits).map(|i| i64::pow(9, i as u32)).sum::<i64>();

        // [100, 383) での neq number の数
        let term2 = {
            let mut cnt = 0;

            for i in 0..ndigits {
                // i = 0 のときは、[100, 300) の neq number の数 (2 * 9^2)
                // i = 1 のときは、[300, 380) の neq number の数 (7 * 9^1)
                // i = 2 のときは、[380, 383) の neq number の数 (3 * 9^0)
                // を求める
                let pow9 = i64::pow(9, (ndigits - i - 1) as u32);

                let addition = if i == 0 {
                    (ds[i] - 1) * pow9
                } else if ds[i - 1] < ds[i] {
                    (ds[i] - 1) * pow9
                } else {
                    ds[i] * pow9
                };

                cnt += addition;

                // 別ケースで x = 3325 のときは、i = 2 以降は 0通り (33の時点で neq number ではないので)
                if i != 0 && ds[i - 1] == ds[i] {
                    break;
                }
            }
            cnt
        };

        term1 + term2
    }

    fn is_neq_number(x: i64) -> bool {
        to_decimal(x)
            .iter()
            .copied()
            .tuple_windows()
            .all(|(a, b)| a != b)
    }

    fn cnt_neq_num_naive(x: i64) -> i64 {
        (1..=x).filter(|&y| Self::is_neq_num(y)).count() as i64
    }

    fn solve2(&self) -> i64 {
        // 解法: 二分探索 (x以下の neq number の数を頑張って数える)
        let prod = |x: i64| {
            // 答え(k番目のneq number) 以上かどうか
            Self::cnt_neq_num2(x) >= self.k
        };
        // 10^17
        bin_search(100_000_000_000_000_000, 0, prod)
    }

    fn solve3(&self) -> i64 {
        // 解法: 答えを絞り込む (9分探索的な感じ)
        // AC できていない

        // 148 番目の neq number を求める場合は以下のように探索していけばよい。
        // 1, 10, 100 (桁をあわせる)
        // 100 (100の位をあわせる)
        // 110, 120, 130, 140, 150, 160, 170 (10の位をあわせる)
        // 170, 171, 172, 173, 174 (1の位をあわせる)
        // これらの値に対して [0, x) での neq number の数は求めやすい。
        // はじめて [0, x) での neq number の数が k になるような x を求めれば、
        // x - 1 が k番目の neq number となる。

        let ideal_k = self.k;

        // 桁数を決める
        // [1, 10) の neq number の数は 9個
        // [10, 100) の neq number の数は 9^2個
        // [100, 1000) の neq number の数は 9^3個
        // つまり、
        // [1, 1 + 9) 番目の neq number は 1桁
        // [1 + 9, 1 + 9 + 9^2) 番目の neq number は 2桁
        // [1 + 9 + 9^2, 1 + 9 + 9^2 + 9^3) 番目の neq number は 3桁
        let n_digits = (1..)
            .map(|i| {
                // 1 + 9 + ... + 9^{i-1}
                (0..i).map(|j| i64::pow(9, j)).sum::<i64>()
            })
            .position(|k| ideal_k < k)
            .unwrap();

        // [0, 10^n_digits) の neq number の数
        let mut current_k = (1..n_digits).map(|k| i64::pow(9, k as u32)).sum::<i64>();
        // 各桁の値を決める
        // dbg!(current_k);
        let mut digits = vec![0; n_digits];
        for di in 0..n_digits {
            // 上から di 桁目を考える
            let pow9 = i64::pow(9, (n_digits - di - 1) as u32);
            for i in 1..=9 {
                lg!(format!("{} {} {}", di, i, current_k));
                if di == 0 && i == 0 {
                    continue;
                }

                if (current_k..current_k + pow9).contains(&ideal_k) {
                    digits[di] = i;
                    break;
                }

                // 例: [0, 300) から [0, 400) に進めると、neq number の数は 81 増える
                // なぜならば [300, 400) に neq number は 81 あるので

                if di == 0 || digits[di - 1] != i {
                    current_k += pow9;
                }
            }
        }

        //assert_eq!(x_begin + 1, x_end);

        eval_base_n_value(&digits, 10) - 1
    }
}

impl Problem {
    fn read() -> Problem {
        input! {
            n_case: usize,
            cases: [i64; n_case],
        }
        let cases = cases.iter().copied().map(|k| TestCase { k }).collect_vec();
        Problem { n_case, cases }
    }

    fn solve(&self) -> Answer {
        let ans = self.cases.iter().map(|case| case.solve3()).collect_vec();
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<i64>,
}

impl Answer {
    fn print(&self) {
        print_vec(&self.ans);
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
        let cnt = (0..1000)
            .filter(|x| {
                let xd = to_decimal(*x);
                xd.iter().copied().tuple_windows().all(|(p, q)| p != q)
            })
            .inspect(|x| {
                dbg!(x);
            })
            .count();
        dbg!(cnt);
        dbg!(9 * 9 * 9);
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn test_problem2_1() {
        (1..100)
            .map(|k| {
                let p = TestCase { k };
                p.solve2()
            })
            .for_each(|x| {
                println!("{x}");
            })
    }

    #[test]
    fn test_problem2_2() {
        assert_eq!(TestCase::cnt_neq_num2(173), 148);
        assert_eq!(TestCase::cnt_neq_num2(110), 99);
        for i in 1..10000 {
            dbg!(i);
            assert_eq!(TestCase::cnt_neq_num2(i), TestCase::cnt_neq_num_naive(i));
        }
    }

    #[test]
    fn test_problem3_1() {
        // 0を含めたneq number を列挙する
        let neq_numbers = (0..100).filter(|i| TestCase::is_neq_num(*i)).collect_vec();

        for k in 1..100 {
            let ans = TestCase { k }.solve3();
            let expected = neq_numbers[k as usize];

            dbg!(k);
            assert_eq!(ans, expected);
        }
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

/// 二分探索をする
/// ```text
/// ng ng ng ok ok ok
///          ↑ここの引数の値を返す
/// ```
/// 計算量: O(log(|ok - ng|))
/// ## Arguments
/// * ok != ng
/// * |ok - ng| <= 2^63 - 1, |ok + ng| <= 2^63 - 1
/// * p の定義域について
///     * ng < ok の場合、p は区間 ng..ok で定義されている。
///     * ok < ng の場合、p は区間 ok..ng で定義されている。
/// * p の単調性について
///     * ng < ok の場合、p は単調増加
///     * ok < ng の場合、p は単調減少
/// ## Return
/// * ng < ok の場合: I = { i in ng..ok | p(i) == true } としたとき
///     * I が空でなければ、min I を返す。
///     * I が空ならば、ok を返す。
/// * ok < ng の場合: I = { i in ok..ng | p(i) == true } としたとき
///     * I が空でなければ、max I を返す。
///     * I が空ならば、ok を返す。
pub fn bin_search<F>(mut ok: i64, mut ng: i64, p: F) -> i64
where
    F: Fn(i64) -> bool,
{
    assert!(ok != ng);
    assert!(ok.checked_sub(ng).is_some());
    assert!(ok.checked_add(ng).is_some());
    while num::abs(ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        assert!(mid != ok);
        assert!(mid != ng);
        if p(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}
use positional_notation::*;
#[allow(clippy::module_inception)]
pub mod positional_notation {
    pub fn eval_base_n_value(xs: &[i64], base: i64) -> i64 {
        xs.iter().fold(0, |acc, &x| acc * base + x)
    }
    pub fn to_base_n_value(x: i64, base: i64) -> Vec<i64> {
        let mut ret = vec![];
        let mut x = x;
        while x > 0 {
            ret.push(x % base);
            x /= base;
        }
        ret.reverse();
        ret
    }
}
