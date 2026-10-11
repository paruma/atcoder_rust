// 問題文と制約は読みましたか？
// #[fastout]
fn main() {
    input! {
        n: usize,
        ss: [i64; n],
        ts: [i64; n],
    }

    let ps = ss
        .iter()
        .copied()
        .enumerate()
        .sorted_by_key(|(_, v)| *v)
        .map(|(k, _)| k)
        .collect_vec();

    let qs = ts
        .iter()
        .copied()
        .enumerate()
        .sorted_by_key(|(_, v)| *v)
        .map(|(k, _)| k)
        .collect_vec();

    let pinvq = mul_of_permutation(&inv_of_permutation(&ps), qs.clone());

    let mut ft = RangeSumFenwickTreeI64::new(n);
    let mut invs = vec![0; n];

    for i in 0..n {
        invs[i] = ft.range_sum(pinvq[i]..);
        ft.add(pinvq[i], 1);
    }

    lg!(&invs);
    lg!(&pinvq);

    let ss = ss.iter().copied().sorted().collect_vec();

    lg!((0..n).map(|i| ts[pinvq[i]]).collect_vec());

    let ans = (0..n)
        .map(|i| dbg!(((ss[i] + invs[pinvq[i]]) - ts[pinvq[i]]).abs()))
        .sum::<i64>();

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
use symmetric_group::*;
#[allow(clippy::module_inception)]
pub mod symmetric_group {
    /// 置換を巡回置換の積で表したときの巡回置換のリストを返す。
    ///
    /// 例: `make_cycles(&[1, 0, 3, 2]) == vec![vec![0, 1], vec![2, 3]]`
    ///
    /// # 計算量
    /// O(N)
    pub fn make_cycles(ps: &[usize]) -> Vec<Vec<usize>> {
        let n = ps.len();
        let mut visited = vec![false; n];
        let mut cycles = vec![];
        for init in 0..n {
            if visited[init] {
                continue;
            }
            let mut cycle = vec![];
            let mut cur = init;
            while !visited[cur] {
                cycle.push(cur);
                visited[cur] = true;
                cur = ps[cur];
            }
            cycles.push(cycle);
        }
        cycles
    }
    /// 置換の積を計算する
    ///
    /// # 計算量
    /// O(N)
    pub fn mul_of_permutation(ps: &[usize], mut qs: Vec<usize>) -> Vec<usize> {
        for q in qs.iter_mut() {
            *q = ps[*q];
        }
        qs
    }
    /// 置換の `k` 乗を計算する
    ///
    /// # 計算量
    /// O(N)
    pub fn pow_of_permutation(ps: &[usize], k: u64) -> Vec<usize> {
        let n = ps.len();
        let mut ret = vec![0; n];
        let cycles = make_cycles(ps);
        for cycle in cycles {
            let len = cycle.len() as u64;
            let k = (k % len) as usize;
            for (i, &x) in cycle.iter().enumerate() {
                ret[x] = cycle[(i + k) % cycle.len()];
            }
        }
        ret
    }
    /// 逆置換を計算する
    ///
    /// # 計算量
    /// O(N)
    pub fn inv_of_permutation(ps: &[usize]) -> Vec<usize> {
        let n = ps.len();
        let mut ret = vec![0; n];
        for (i, &p) in ps.iter().enumerate() {
            ret[p] = i;
        }
        ret
    }
    /// 転倒数を計算する
    ///
    /// # 計算量
    /// O(N log N)
    pub fn inversion_number(ps: &[usize]) -> i64 {
        use ac_library::FenwickTree;
        let n = ps.len();
        let mut ft = FenwickTree::new(n, 0_i64);
        let mut ans = 0;
        for &p in ps {
            ans += ft.sum(p + 1..n);
            ft.add(p, 1);
        }
        ans
    }
}
use ab_group::*;
use range_sum_fenwick_tree::*;
#[allow(clippy::module_inception)]
pub mod ab_group {
    use std::{
        convert::Infallible,
        iter::Sum,
        marker::PhantomData,
        ops::{Add, Neg, Sub},
    };
    /// 可換群 (Abelian Group)
    pub trait AbGroup {
        type S: Clone;
        fn zero() -> Self::S;
        fn add(a: &Self::S, b: &Self::S) -> Self::S;
        fn neg(a: &Self::S) -> Self::S;
        fn sub(a: &Self::S, b: &Self::S) -> Self::S {
            Self::add(a, &Self::neg(b))
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct AdditiveAbGroup<T>(Infallible, PhantomData<fn() -> T>);
    impl<T: Sum + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy> AbGroup
        for AdditiveAbGroup<T>
    {
        type S = T;
        fn zero() -> Self::S {
            std::iter::empty().sum()
        }
        fn add(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
        fn neg(a: &Self::S) -> Self::S {
            -(*a)
        }
        fn sub(a: &Self::S, b: &Self::S) -> Self::S {
            *a - *b
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct XorAbGroup(Infallible);
    impl AbGroup for XorAbGroup {
        type S = u64;
        fn zero() -> Self::S {
            0
        }
        fn add(a: &Self::S, b: &Self::S) -> Self::S {
            *a ^ *b
        }
        fn neg(a: &Self::S) -> Self::S {
            *a
        }
        fn sub(a: &Self::S, b: &Self::S) -> Self::S {
            *a ^ *b
        }
    }
}
#[allow(clippy::module_inception)]
pub mod range_sum_fenwick_tree {
    use super::{AbGroup, AdditiveAbGroup};
    use std::ops::{Bound, RangeBounds};
    /// 可換群 (AbGroup) を用いた汎用的な Fenwick Tree (Binary Indexed Tree)。
    ///
    /// 0-indexed で実装されています。
    ///
    /// 基本的な加算・区間和クエリに加え、get/set や、二分探索 (max_right / min_left) を提供します。
    #[derive(Clone)]
    pub struct RangeSumFenwickTreeArbitrary<G: AbGroup> {
        n: usize,
        pub(crate) ary: Vec<G::S>,
    }
    /// i64 の加算群を用いた標準的な Fenwick Tree のエイリアス。
    pub type RangeSumFenwickTreeI64 = RangeSumFenwickTreeArbitrary<AdditiveAbGroup<i64>>;
    /// 任意の数値型 T の加算群を用いた Fenwick Tree のエイリアス。
    pub type RangeSumFenwickTree<T> = RangeSumFenwickTreeArbitrary<AdditiveAbGroup<T>>;
    pub type FenwickTree<T> = RangeSumFenwickTree<T>;
    impl<G: AbGroup> RangeSumFenwickTreeArbitrary<G> {
        /// サイズ `n` の Fenwick Tree を作成します。
        ///
        /// 要素はすべて `G::zero()` で初期化されます。
        ///
        /// # 計算量
        /// O(n)
        pub fn new(n: usize) -> Self {
            let ary = vec![G::zero(); n];
            RangeSumFenwickTreeArbitrary { n, ary }
        }
        /// 配列スライスから Fenwick Tree を作成します。
        ///
        /// # 計算量
        /// O(n)
        pub fn from_slice(slice: &[G::S]) -> Self {
            let n = slice.len();
            let mut ary = slice.to_vec();
            for i in 0..n {
                let j = i | (i + 1);
                if j < n {
                    let val_i = ary[i].clone();
                    ary[j] = G::add(&ary[j], &val_i);
                }
            }
            RangeSumFenwickTreeArbitrary { n, ary }
        }
        /// `[0, idx)` の区間の総和を計算します。
        ///
        /// # Panics
        /// `idx > n` の場合にパニックします。
        ///
        /// # 計算量
        /// O(log n)
        pub fn prefix_sum(&self, mut idx: usize) -> G::S {
            assert!(
                idx <= self.n,
                "RangeSumFenwickTreeArbitrary::prefix_sum: index out of bounds. idx: {}, n: {}",
                idx,
                self.n
            );
            let mut sum = G::zero();
            while idx > 0 {
                sum = G::add(&sum, &self.ary[idx - 1]);
                idx &= idx - 1;
            }
            sum
        }
        /// `idx` 番目の要素に `val` を加算（群の演算を適用）します。
        ///
        /// # Panics
        /// `idx >= n` の場合にパニックします。
        ///
        /// # 計算量
        /// O(log n)
        pub fn add(&mut self, mut idx: usize, val: G::S) {
            assert!(
                idx < self.n,
                "RangeSumFenwickTreeArbitrary::add: index out of bounds. idx: {}, n: {}",
                idx,
                self.n
            );
            let n = self.n;
            idx += 1;
            while idx <= n {
                self.ary[idx - 1] = G::add(&self.ary[idx - 1], &val);
                idx += idx & idx.wrapping_neg();
            }
        }
        fn resolve_range<R: RangeBounds<usize>>(&self, range: R) -> (usize, usize) {
            let l = match range.start_bound() {
                Bound::Included(&l) => l,
                Bound::Excluded(&l) => l + 1,
                Bound::Unbounded => 0,
            };
            let r = match range.end_bound() {
                Bound::Included(&r) => r + 1,
                Bound::Excluded(&r) => r,
                Bound::Unbounded => self.n,
            };
            (l, r)
        }
        /// 指定された範囲の区間和を計算します。
        ///
        /// # Panics
        /// 範囲が不正、または `n` を超える場合にパニックします。
        ///
        /// # 計算量
        /// O(log n)
        pub fn range_sum<R>(&self, range: R) -> G::S
        where
            R: RangeBounds<usize>,
        {
            let (l, r) = self.resolve_range(range);
            assert!(
                l <= r && r <= self.n,
                "RangeSumFenwickTreeArbitrary::range_sum: invalid range. l: {}, r: {}, n: {}",
                l,
                r,
                self.n
            );
            G::sub(&self.prefix_sum(r), &self.prefix_sum(l))
        }
        /// `l` を左端として、`f(sum(l..r))` が true になる最大の `r` を返します。
        ///
        /// # 前提条件
        /// * `l <= n`
        /// * `f(&G::zero())` が `true`
        /// * `f` は単調である。
        ///     * `f(sum(l..i))` が `true` => `f(sum(l..j))` が `true` for all `l <= j <= i`.
        ///
        /// # 計算量
        /// O(log n)
        pub fn max_right<F>(&self, l: usize, mut f: F) -> usize
        where
            F: FnMut(&G::S) -> bool,
        {
            assert!(
                l <= self.n,
                "RangeSumFenwickTreeArbitrary::max_right: index out of bounds. l: {}, n: {}",
                l,
                self.n
            );
            let zero = G::zero();
            assert!(
                f(&zero),
                "RangeSumFenwickTreeArbitrary::max_right: The predicate f(zero) must be true."
            );
            let val_l = self.prefix_sum(l);
            let mut r = 0;
            let mut current_val = G::zero();
            let mut k = if self.n == 0 {
                0
            } else {
                1 << (usize::BITS - 1 - self.n.leading_zeros())
            };
            while k > 0 {
                if r + k <= self.n {
                    let next_val = G::add(&current_val, &self.ary[r + k - 1]);
                    if r + k <= l || f(&G::sub(&next_val, &val_l)) {
                        r += k;
                        current_val = next_val;
                    }
                }
                k >>= 1;
            }
            r
        }
        /// `r` を右端として、`f(sum(l..r))` が true になる最小の `l` を返します。
        ///
        /// # 前提条件
        /// * `r <= n`
        /// * `f(&G::zero())` が `true`
        /// * `f` は単調である。
        ///     * `f(sum(i..r))` が `true` => `f(sum(j..r))` が `true` for all `i <= j <= r`.
        ///
        /// # 計算量
        /// O(log n)
        pub fn min_left<F>(&self, r: usize, mut f: F) -> usize
        where
            F: FnMut(&G::S) -> bool,
        {
            assert!(
                r <= self.n,
                "RangeSumFenwickTreeArbitrary::min_left: index out of bounds. r: {}, n: {}",
                r,
                self.n
            );
            let zero = G::zero();
            assert!(
                f(&zero),
                "RangeSumFenwickTreeArbitrary::min_left: The predicate f(zero) must be true."
            );
            let val_r = self.prefix_sum(r);
            if f(&val_r) {
                return 0;
            }
            let mut idx = 0;
            let mut current_val = G::zero();
            let mut k = if self.n == 0 {
                0
            } else {
                1 << (usize::BITS - 1 - self.n.leading_zeros())
            };
            while k > 0 {
                if idx + k <= r {
                    let next_val = G::add(&current_val, &self.ary[idx + k - 1]);
                    if !f(&G::sub(&val_r, &next_val)) {
                        idx += k;
                        current_val = next_val;
                    }
                }
                k >>= 1;
            }
            idx + 1
        }
        /// `idx` 番目の要素の値を取得します。
        ///
        /// # Panics
        /// `idx >= n` の場合にパニックします。
        ///
        /// # 計算量
        /// O(log n)
        pub fn get(&self, idx: usize) -> G::S {
            assert!(
                idx < self.n,
                "RangeSumFenwickTreeArbitrary::get: index out of bounds. idx: {}, n: {}",
                idx,
                self.n
            );
            self.range_sum(idx..=idx)
        }
        /// `idx` 番目の要素の値を `val` に設定します。
        ///
        /// # Panics
        /// `idx >= n` の場合にパニックします。
        ///
        /// # 計算量
        /// O(log n)
        pub fn set(&mut self, idx: usize, val: G::S) {
            assert!(
                idx < self.n,
                "RangeSumFenwickTreeArbitrary::set: index out of bounds. idx: {}, n: {}",
                idx,
                self.n
            );
            let old_val = self.get(idx);
            self.add(idx, G::sub(&val, &old_val));
        }
        /// Fenwick Tree の現在の状態を `Vec<G::S>` として返します。
        ///
        /// # 計算量
        /// O(n log n)
        pub fn to_vec(&self) -> Vec<G::S> {
            (0..self.n).map(|i| self.get(i)).collect()
        }
        /// 保持している要素数を返します。
        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            self.n
        }
    }
}
use lg::*;
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
    macro_rules ! rows {{$ index_label : literal , $ (@ offset $ offset : expr_2021 , ) ? $ (@ verticalbar $ verticalbar : expr_2021 , ) * $ ($ (@$ label : literal => ) ? $ values : expr_2021 ) ,* $ (, ) ? } => {{#! [allow (unused_assignments ) ] let mut rows = $ crate :: Rows :: default () ; rows . line_number (line ! () ) ; $ (rows . offset ($ offset ) ; ) ? $ (rows . verticalbar ($ verticalbar ) ; ) * rows . index_label ($ index_label ) ; $ ({let mut label = stringify ! ($ values ) . to_string () ; if label . starts_with ("&" ) {label = label [1 .. ] . to_string () ; } $ ({let label_ : &'static str = $ label ; label = label_ . to_string () ; } ) ? rows . row (label , $ values ) ; } ) * eprintln ! ("{}" , rows . to_string_table () ) ; } } ; }
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
