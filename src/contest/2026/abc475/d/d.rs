// 問題文と制約は読みましたか？
// #[fastout]
fn main() {
    input! {
        xs: Chars,
    }

    let sieve = EratosthenesSieve::new(10_000_000);

    let alphabet = xs.iter().copied().unique().collect_vec();
    let mut to_index = HashMap::new();
    let kind = alphabet.len();

    for i in 0..kind {
        to_index.insert(alphabet[i], i);
    }

    let ans = (0..10)
        .permutations(kind)
        .flat_map(|perm| {
            let num_chars = xs
                .iter()
                .copied()
                .map(|x| perm[to_index[&x]])
                .rev()
                .collect_vec();
            if *num_chars.last().unwrap() == 0 {
                return None;
            }

            let num = from_digits_le(&num_chars, 10);
            sieve.is_prime(num as usize).then_some(num)
        })
        .next();

    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
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
use eratosthenes_sieve::*;
pub mod eratosthenes_sieve {
    /// エラトステネスのふるいを用いて素数判定を行う。
    /// メモリ効率のため `is_prime_list` のみを持つ。
    #[derive(Clone, Debug)]
    pub struct EratosthenesSieve {
        is_prime_list: Vec<bool>,
    }
    impl EratosthenesSieve {
        /// [0, n] の区間でエラトステネスのふるいをする
        /// # 計算量
        /// O(n log(log(n)))
        pub fn new(n: usize) -> Self {
            let mut is_prime_list = vec![true; n + 1];
            is_prime_list[0] = false;
            is_prime_list[1] = false;
            for p in 2..=n {
                if !is_prime_list[p] {
                    continue;
                }
                for q in (p * 2..=n).step_by(p) {
                    is_prime_list[q] = false;
                }
            }
            Self { is_prime_list }
        }
        /// n が素数かどうか判定する
        /// # 計算量
        /// O(1)
        pub fn is_prime(&self, n: usize) -> bool {
            self.is_prime_list[n]
        }
    }
    /// エラトステネスのふるいを用いて、最小素因数（min prime factor）を管理する。
    /// 素因数分解や約数列挙を高速に行うことができる。
    #[derive(Clone, Debug)]
    pub struct EratosthenesSieveMinFactor {
        min_factor_list: Vec<usize>,
    }
    impl EratosthenesSieveMinFactor {
        /// [0, n] の区間でエラトステネスのふるいをする
        /// # 計算量
        /// O(n log(log(n)))
        pub fn new(n: usize) -> Self {
            let mut min_factor_list = vec![0; n + 1];
            for p in 2..=n {
                if min_factor_list[p] != 0 {
                    continue;
                }
                for q in (p..=n).step_by(p) {
                    if min_factor_list[q] == 0 {
                        min_factor_list[q] = p;
                    }
                }
            }
            Self { min_factor_list }
        }
        /// n が素数かどうか判定する
        /// # 計算量
        /// O(1)
        pub fn is_prime(&self, n: usize) -> bool {
            n >= 2 && self.min_factor_list[n] == n
        }
        /// n を素因数分解する。素数とその指数のペアのリストを返す。
        /// # 計算量
        /// O(log n)
        pub fn prime_factorize(&self, n: usize) -> Vec<(usize, usize)> {
            let mut n = n;
            let mut res = Vec::new();
            while n > 1 {
                let p = self.min_factor_list[n];
                let mut exp = 0;
                while self.min_factor_list[n] == p {
                    n /= p;
                    exp += 1;
                }
                res.push((p, exp));
            }
            res
        }
        /// n の正の約数を列挙する。
        /// # 計算量
        /// O(nの約数の個数)
        pub fn divisors(&self, n: usize) -> Vec<usize> {
            let pf = self.prime_factorize(n);
            let divisor_count = pf.iter().map(|&(_, e)| e + 1).product();
            let mut res = Vec::with_capacity(divisor_count);
            res.push(1);
            for (p, e) in pf {
                let n = res.len();
                for i in 0..n {
                    let mut tmp = 1;
                    for _ in 0..e {
                        tmp *= p;
                        res.push(res[i] * tmp);
                    }
                }
            }
            res
        }
    }
}
use digit::*;
#[allow(clippy::module_inception)]
pub mod digit {
    /// n の base 進数を Little Endian で表す
    /// 例:
    /// - `to_digits_le_vec(123, 10) == vec![3, 2, 1]`
    /// - `to_digits_le_vec(0, 10) == vec![]`
    pub fn to_digits_le_vec(mut n: i64, base: i64) -> Vec<i64> {
        assert!(n >= 0);
        assert!(base >= 2);
        if n == 0 {
            return vec![];
        }
        let mut res = vec![];
        while n > 0 {
            res.push(n % base);
            n /= base;
        }
        res
    }
    /// n の base 進数を Little Endian で生成するイテレータ
    /// 例:
    /// - `to_digits_le_iter(123, 10).collect::<Vec<_>>() == vec![3, 2, 1]`
    /// - `to_digits_le_iter(0, 10).collect::<Vec<_>>() == vec![]`
    pub fn to_digits_le_iter(n: i64, base: i64) -> impl Iterator<Item = i64> {
        assert!(n >= 0);
        assert!(base >= 2);
        DigitsLeIterator { n, base }
    }
    struct DigitsLeIterator {
        n: i64,
        base: i64,
    }
    impl Iterator for DigitsLeIterator {
        type Item = i64;
        fn next(&mut self) -> Option<Self::Item> {
            if self.n == 0 {
                return None;
            }
            let digit = self.n % self.base;
            self.n /= self.base;
            Some(digit)
        }
    }
    /// Little Endian で表された各桁から、数値を評価する
    /// 例:
    /// - `from_digits_le(&[3, 2, 1], 10) == 123`
    /// - `from_digits_le(&[], 10) == 0`
    pub fn from_digits_le(digits: &[i64], base: i64) -> i64 {
        assert!(base >= 2);
        debug_assert!(digits.iter().all(|&d| (0..base).contains(&d)));
        digits.iter().rfold(0, |acc, &d| acc * base + d)
    }
    /// x を base 進数で表した際の桁数を返す
    /// 例:
    /// - `count_digits(123, 10) == 3`
    /// - `count_digits(0, 10) == 0`
    pub fn count_digits(mut x: i64, base: i64) -> usize {
        assert!(x >= 0);
        assert!(base >= 2);
        if x == 0 {
            return 0;
        }
        let mut count = 0;
        while x > 0 {
            x /= base;
            count += 1;
        }
        count
    }
    /// 2つの数値を指定された基数で連結する。
    /// `count_digits(b, base)` が 0 (すなわち `b == 0`) の場合、`a` をそのまま返す。
    /// 例:
    /// - `concat_digits(123, 45, 10) == 12345`
    /// - `concat_digits(123, 0, 10) == 123`
    pub fn concat_digits(a: i64, b: i64, base: i64) -> i64 {
        assert!(a >= 0);
        assert!(b >= 0);
        assert!(base >= 2);
        let digits = count_digits(b, base);
        if digits == 0 {
            return a;
        }
        let mut p = 1;
        for _ in 0..digits {
            p *= base;
        }
        a * p + b
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
    /// # use mylib::*;
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
    macro_rules ! rows {{$ index_label : literal , $ (@ offset $ offset : expr_2021 , ) ? $ (@ verticalbar $ verticalbar : expr_2021 , ) * $ ($ (@$ label : literal => ) ? $ values : expr_2021 ) ,* $ (, ) ? } => {{#! [allow (unused_assignments ) ] let mut rows = $ crate :: Rows :: default () ; rows . line_number (line ! () ) ; $ (rows . offset ($ offset ) ; ) ? $ (rows . verticalbar ($ verticalbar ) ; ) * rows . index_label ($ index_label ) ; $ ({let mut label = stringify ! ($ values ) . to_string () ; if label . starts_with ("&" ) {label = label [1 .. ] . to_string () ; } $ ({let label_ : &'static str = $ label ; label = label_ . to_string () ; } ) ? rows . row (label , $ values ) ; } ) * eprintln ! ("{}" , rows . to_string_table () ) ; } } ; }
    /// Print the 2D array with the line number.
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
