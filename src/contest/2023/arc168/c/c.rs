//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    k: usize,
    s: Vec<u8>,
}
fn solve_sub_naive(k: usize, a: usize, b: usize, c: usize) -> usize {
    let n = a + b + c;
    let comb2_list: Vec<(usize, usize)> = (0..n).tuple_combinations().collect_vec();

    let xs_origin = [(a, b'A'), (b, b'B'), (c, b'C')]
        .iter()
        .copied()
        .flat_map(|(cnt, ch)| std::iter::repeat(ch).take(cnt))
        .collect_vec();

    let mut set = HashSet::new();
    set.insert(xs_origin.clone()); // multi_cartesian_product は0個の直積をするとバグるので、ここでケアしておく。

    for ops in (0..=k).flat_map(|n_times| {
        std::iter::repeat(&comb2_list)
            .take(n_times)
            .multi_cartesian_product()
    }) {
        let mut xs = xs_origin.clone();

        for &(i, j) in ops {
            xs.swap(i, j)
        }
        set.insert(xs);
    }
    for x in &set {
        let msg = String::from_utf8(x.clone()).unwrap();
        dbg!(msg);
    }
    set.len()
}
use ac_library::ModInt998244353 as Mint;

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            k: usize,
            s: Bytes,
        }
        Problem { n, k, s }
    }
    fn solve(&self) -> Answer {
        todo!()
    }

    fn solve_wrong3(&self) -> Answer {
        // AB置換、BC置換、CA置換、AB置換→BC置換、BC置換→CA置換、CA置換→AB置換 で場合分け
        // ABC ってパターンで長さ3巡回置換2通りが3通りとカウントされてしまった
        let cnt_ch = |target: u8| self.s.iter().copied().filter(|&ch| ch == target).count();
        let n = self.n;
        let k = self.k;
        let na = cnt_ch(b'A');
        let nb = cnt_ch(b'B');
        let nc = cnt_ch(b'C');
        let comb = Comb::new(usize::max(n, k));

        let min3 = |a: usize, b: usize, c: usize| a.min(b).min(c);

        let cnt_empty = Mint::new(1);

        let cnt_cycle1 = |na: usize, nb: usize| -> Mint {
            (1..=min3(na, nb, k))
                .map(|i| comb.comb(na, i) * comb.comb(nb, i))
                .sum::<Mint>()
        };

        let cnt_ab = cnt_cycle1(na, nb);
        let cnt_bc = cnt_cycle1(nb, nc);
        let cnt_ca = cnt_cycle1(nc, na);

        let cnt_cycle2 = |na: usize, nb: usize, nc: usize| -> Mint {
            let mut sum = Mint::new(0);
            for cycle_ab in 1..=min3(na, nb, k) {
                let cnt_ab = comb.comb(na, cycle_ab) * comb.comb(nb, cycle_ab);
                let k = k - cycle_ab;

                for cycle_bc in 1..=min3(nb, nc, k) {
                    let cnt_bc = comb.comb(nb, cycle_bc) * comb.comb(nc, cycle_bc);
                    let value = cnt_ab * cnt_bc;
                    sum += value;
                }
            }
            sum
        };

        let cnt_ab_bc = cnt_cycle2(na, nb, nc);
        let cnt_bc_ca = cnt_cycle2(nb, nc, na);
        let cnt_ca_ab = cnt_cycle2(nc, na, nb);

        dbg!(cnt_empty);
        dbg!(cnt_ab);
        dbg!(cnt_bc);
        dbg!(cnt_ca);
        dbg!(cnt_ab_bc);
        dbg!(cnt_bc_ca);
        dbg!(cnt_ca_ab);

        let ans = cnt_empty + cnt_ab + cnt_bc + cnt_ca + cnt_ab_bc + cnt_bc_ca + cnt_ca_ab;
        let ans = ans.val() as i64;
        Answer { ans }
    }

    fn solve_wrong2(&self) -> Answer {
        // 5つの巡回置換 AB, BC, CA, ABC, CBA が何回起きるかを考える
        // AABBC で2つの操作で1つの結果が生まれてしまうのでWA
        // AABBC → BABAC → BBCAA
        // AABBC → ABBAC → BBCAA
        let cnt_ch = |target: u8| self.s.iter().copied().filter(|&ch| ch == target).count();
        let n = self.n;
        let k = self.k;
        let na = cnt_ch(b'A');
        let nb = cnt_ch(b'B');
        let nc = cnt_ch(b'C');

        let comb = Comb::new(usize::max(n, k));
        let mut ans = Mint::new(0);
        for cycle_ab in 0..=k {
            if cycle_ab > na || cycle_ab > nb {
                break;
            }
            let cnt_ab = comb.comb(na, cycle_ab) * comb.comb(nb, cycle_ab);

            let na = na - cycle_ab;
            let nb = nb - cycle_ab;
            for cycle_bc in 0..=k {
                if cycle_bc > nb || cycle_bc > nc || cycle_ab + cycle_bc > k {
                    break;
                }
                let cnt_bc = comb.comb(nb, cycle_bc) * comb.comb(nc, cycle_bc);
                let nb = nb - cycle_bc;
                let nc = nc - cycle_bc;
                for cycle_ca in 0..=k {
                    if cycle_ca > nc || cycle_ca > na || cycle_ab + cycle_bc + cycle_ca > k {
                        break;
                    }
                    let cnt_ca = comb.comb(nc, cycle_ca) * comb.comb(na, cycle_ca);
                    let nc = nc - cycle_ca;
                    let na = na - cycle_ca;
                    for cycle_abc in 0..=k / 2 {
                        if cycle_abc > na
                            || cycle_abc > nb
                            || cycle_abc > nc
                            || cycle_ab + cycle_bc + cycle_ca + cycle_abc * 2 > k
                        {
                            break;
                        }
                        let cnt_abc = comb.comb(na, cycle_abc)
                            * comb.comb(nb, cycle_abc)
                            * comb.comb(nc, cycle_abc);
                        let na = na - cycle_abc;
                        let nb = nb - cycle_abc;
                        let nc = nc - cycle_abc;
                        for cycle_cba in 0..=k / 2 {
                            if cycle_cba > na
                                || cycle_cba > nb
                                || cycle_cba > nc
                                || cycle_ab + cycle_bc + cycle_ca + cycle_abc * 2 + cycle_cba * 2
                                    > k
                            {
                                break;
                            }
                            let cnt_cba = comb.comb(na, cycle_cba)
                                * comb.comb(nb, cycle_cba)
                                * comb.comb(nc, cycle_cba);
                            let value = cnt_ab * cnt_bc * cnt_ca * cnt_abc * cnt_cba;
                            ans += value;
                            let msg = format!(
                                "{} {} {} {} {}: {}",
                                cycle_ab, cycle_bc, cycle_ca, cycle_abc, cycle_cba, value
                            );
                            dbg!(msg);
                        }
                    }
                }
            }
        }
        let ans = ans.val() as i64;
        Answer { ans }
    }

    fn solve_wrong(&self) -> Answer {
        // 解法: 長さ2の巡回置換と長さ3の巡回置換を消費していく。
        // AABBC みたいなケースで、BBAAC を4回数えてしまうのでWA
        let cnt_ch = |target: u8| self.s.iter().copied().filter(|&ch| ch == target).count();
        let n = self.n;
        let k = self.k;
        let na = cnt_ch(b'A');
        let nb = cnt_ch(b'B');
        let nc = cnt_ch(b'C');
        let mut memo = vec![HashMap::<(i64, i64, i64), Mint>::new(); k + 1];

        fn rec(
            k: usize,
            a: i64,
            b: i64,
            c: i64,
            memo: &mut [HashMap<(i64, i64, i64), Mint>],
        ) -> Mint {
            if a < 0 || b < 0 || c < 0 {
                return Mint::new(0);
            }
            if k == 0 {
                return Mint::new(1);
            }
            if k == 1 {
                return Mint::new(a * b + b * c + c * a);
            }
            if let Some(ans) = memo[k].get(&(a, b, c)).copied() {
                return ans;
            }

            let ans = {
                let term1 = rec(k - 2, a - 1, b - 1, c - 1, memo) * 2 * a * b * c;
                let term2 = rec(k - 1, a - 1, b - 1, c, memo) * a * b;
                let term3 = rec(k - 1, a, b - 1, c - 1, memo) * b * c;
                let term4 = rec(k - 1, a - 1, b, c - 1, memo) * c * a;
                term1 + term2 + term3 + term4
            };
            memo[k].insert((a, b, c), ans);
            ans
        }

        let ans = (0..=k)
            .map(|i| rec(i, na as i64, nb as i64, nc as i64, &mut memo))
            .sum::<Mint>();
        dbg!(memo);
        let ans = ans.val() as i64;
        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        let cnt_ch = |target: u8| self.s.iter().copied().filter(|&ch| ch == target).count();
        let k = self.k;
        let na = cnt_ch(b'A');
        let nb = cnt_ch(b'B');
        let nc = cnt_ch(b'C');

        let ans = solve_sub_naive(k, na, nb, nc);
        let ans = ans as i64;
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
    // Problem::read().solve_naive().print();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};

    #[test]
    fn test_problem() {
        lg!(solve_sub_naive(1, 1, 1, 0));
        lg!(solve_sub_naive(1, 1, 1, 1));
        lg!(solve_sub_naive(1, 1, 1, 2));
        lg!(solve_sub_naive(1, 1, 1, 3));
        lg!(solve_sub_naive(1, 1, 1, 4));
        lg!("");
        lg!(solve_sub_naive(1, 1, 1, 1));
        lg!(solve_sub_naive(2, 1, 1, 1));
        lg!(solve_sub_naive(3, 1, 1, 1));
        lg!(solve_sub_naive(4, 1, 1, 1));
        lg!(solve_sub_naive(5, 1, 1, 1));
        lg!("");
        lg!(solve_sub_naive(1, 2, 1, 1));
        lg!(solve_sub_naive(2, 2, 1, 1));
        lg!(solve_sub_naive(3, 2, 1, 1));
        lg!(solve_sub_naive(4, 2, 1, 1));
        lg!(solve_sub_naive(5, 2, 1, 1));
        lg!("");
        lg!(solve_sub_naive(1, 2, 3, 1));
        lg!(solve_sub_naive(2, 2, 3, 1));
        lg!(solve_sub_naive(3, 2, 3, 1));
        lg!(solve_sub_naive(4, 2, 3, 1));
        lg!(solve_sub_naive(5, 2, 3, 1));
        lg!("");
        // wolfram alpha で実験すると aについてk次式になっているのがわかる。
        lg!(solve_sub_naive(1, 2, 3, 2));
        lg!(solve_sub_naive(2, 2, 3, 2));
        lg!(solve_sub_naive(3, 2, 3, 2));
        lg!(solve_sub_naive(4, 2, 3, 2));
        lg!(solve_sub_naive(5, 2, 3, 2));
        lg!("");
        lg!(solve_sub_naive(1, 2, 3, 3));
        lg!(solve_sub_naive(2, 2, 3, 3));
        lg!(solve_sub_naive(3, 2, 3, 3));
        lg!(solve_sub_naive(4, 2, 3, 3));
        lg!(solve_sub_naive(5, 2, 3, 3));
        lg!("");
        lg!(solve_sub_naive(4, 0, 0, 5));
        lg!(solve_sub_naive(2, 2, 2, 3));
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    struct WrongTestCase {
        problem: Problem,
        main_ans: Answer,
        naive_ans: Answer,
    }

    #[allow(dead_code)]
    fn check(p: &Problem) -> Option<WrongTestCase> {
        let main_ans = p.solve();
        let naive_ans = p.solve_naive();
        if main_ans != naive_ans {
            Some(WrongTestCase {
                problem: p.clone(),
                main_ans,
                naive_ans,
            })
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn make_random_problem(rng: &mut SmallRng) -> Problem {
        let n = rng.gen_range(1..=6);
        let k = rng.gen_range(1..=5);
        let s = (0..n).map(|_| b"ABC"[rng.gen_range(0..3)]).collect_vec();
        let p = Problem { n, k, s };
        println!("{:?}", &p);
        p
    }

    #[allow(unreachable_code)]
    #[test]
    fn test_with_naive() {
        let num_tests = 1000;
        let max_wrong_case = 50; // この件数間違いが見つかったら打ち切り
        let mut rng = SmallRng::seed_from_u64(42);
        // let mut rng = SmallRng::from_entropy();
        let mut wrong_cases: Vec<WrongTestCase> = vec![];
        for _ in 0..num_tests {
            let p = make_random_problem(&mut rng);
            let result = check(&p);
            if let Some(wrong_test_case) = result {
                wrong_cases.push(wrong_test_case);
            }
            if wrong_cases.len() >= max_wrong_case {
                break;
            }
        }

        if !wrong_cases.is_empty() {
            for t in &wrong_cases {
                println!("{:?}", t.problem);
                println!("main ans : {:?}", t.main_ans);
                println!("naive ans: {:?}", t.naive_ans);
                println!();
            }
            println!("{} cases are wrong.", wrong_cases.len());
            panic!();
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
use mod_combinatorics::*;
pub mod mod_combinatorics {
    use ac_library::ModInt998244353 as Mint;
    pub struct Comb {
        fac: Vec<Mint>,
        invfac: Vec<Mint>,
    }
    impl Comb {
        pub fn new(max_val: usize) -> Self {
            let mut inv = vec![Mint::new(0); max_val + 1];
            let mut fac = vec![Mint::new(0); max_val + 1];
            let mut invfac = vec![Mint::new(0); max_val + 1];
            fac[0] = 1.into();
            fac[1] = 1.into();
            invfac[0] = 1.into();
            invfac[1] = 1.into();
            inv[1] = 1.into();
            let modulus = Mint::modulus() as usize;
            for i in 2..=max_val {
                inv[i] = -inv[modulus % i] * Mint::new(modulus / i);
                fac[i] = fac[i - 1] * Mint::new(i);
                invfac[i] = invfac[i - 1] * inv[i];
            }
            Self { fac, invfac }
        }
        pub fn comb(&self, n: usize, k: usize) -> Mint {
            if n < k {
                0.into()
            } else {
                self.fac[n] * self.invfac[k] * self.invfac[n - k]
            }
        }
        pub fn perm(&self, n: usize, k: usize) -> Mint {
            if n < k {
                0.into()
            } else {
                self.fac[n] * self.invfac[n - k]
            }
        }
        pub fn factorial(&self, n: usize) -> Mint {
            self.fac[n]
        }
        pub fn inv_factorial(&self, n: usize) -> Mint {
            self.invfac[n]
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
