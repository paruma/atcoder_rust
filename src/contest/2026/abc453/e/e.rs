// 問題文と制約は読みましたか？
// #[fastout]
fn main() {
    input! {
        n: usize,
        lrs: [(usize, usize); n],
    }

    use ac_library::ModInt998244353 as Mint;
    let comb = Comb::<Mint>::new(n + 1);

    let mut cnts = RangeAddRangeSumFenwickTreeI64::new(n);

    for &(l, r) in &lrs {
        cnts.range_add(l..=r, 1);
    }

    let ans = (1..n - 1)
        .map(|k| {
            let a = k;
            let b = n - k;

            let a_cnt = cnts.get(a);
            let b_cnt = cnts.get(b);
            if (b_cnt as usize) < b {
                Mint::new(0)
            } else {
                comb.comb(a_cnt as usize, a)
            }
        })
        .sum::<Mint>();
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
use ab_group::*;
use range_add_range_sum_fenwick_tree::*;
use range_sum_fenwick_tree::*;
#[allow(clippy::module_inception)]
pub mod range_sum_fenwick_tree {
    use super::{AbGroup, AdditiveAbGroup};
    use std::ops::{Bound, RangeBounds};
    /// 可換群 (AbGroup) を用いた汎用的な Fenwick Tree (Binary Indexed Tree)。
    /// 0-indexed で実装されています。
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
        /// 要素はすべて `G::zero()` で初期化されます。
        /// # 計算量
        /// O(n)
        pub fn new(n: usize) -> Self {
            let ary = vec![G::zero(); n];
            RangeSumFenwickTreeArbitrary { n, ary }
        }
        /// 配列スライスから Fenwick Tree を作成します。
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
        /// # Panics
        /// `idx > n` の場合にパニックします。
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
        /// # Panics
        /// `idx >= n` の場合にパニックします。
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
        /// # Panics
        /// 範囲が不正、または `n` を超える場合にパニックします。
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
        /// `f` は単調性を持つ必要があります。
        /// 具体的には、`f(sum(l..i))` が true ならば、任意の `j < i` に対して `f(sum(l..j))` も true である必要があります。
        /// また、`f(zero)` は true である必要があります。
        /// # Panics
        /// `l > n` または `f(zero)` が false の場合にパニックします。
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
        /// `f` は単調性を持つ必要があります。
        /// 具体的には、`f(sum(i..r))` が true ならば、任意の `j > i` に対して `f(sum(j..r))` も true である必要があります。
        /// また、`f(zero)` は true である必要があります。
        /// # Panics
        /// `r > n` または `f(zero)` が false の場合にパニックします。
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
        /// # Panics
        /// `idx >= n` の場合にパニックします。
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
        /// # Panics
        /// `idx >= n` の場合にパニックします。
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
pub mod range_add_range_sum_fenwick_tree {
    use super::RangeSumFenwickTree;
    use std::iter::Sum;
    use std::ops::{Add, Bound, Mul, Neg, RangeBounds, Sub};
    fn zero<T: Sum>() -> T {
        std::iter::empty::<T>().sum()
    }
    /// 任意の数値型 T に対して区間加算・区間和取得が可能な Fenwick Tree (Range Add Range Sum Fenwick Tree)。
    #[derive(Clone)]
    pub struct RangeAddRangeSumFenwickTree<T>
    where
        T: Copy + Add<Output = T> + Sub<Output = T> + Mul<i64, Output = T> + Neg<Output = T> + Sum,
    {
        n: usize,
        ft0: RangeSumFenwickTree<T>,
        ft1: RangeSumFenwickTree<T>,
    }
    /// i64 の加算群を用いた標準的な Range Add Range Sum Fenwick Tree のエイリアス。
    pub type RangeAddRangeSumFenwickTreeI64 = RangeAddRangeSumFenwickTree<i64>;
    impl<T> RangeAddRangeSumFenwickTree<T>
    where
        T: Copy + Add<Output = T> + Sub<Output = T> + Mul<i64, Output = T> + Neg<Output = T> + Sum,
    {
        /// サイズ `n` の Range Add Range Sum Fenwick Tree を作成します。
        /// # 計算量
        /// O(n)
        pub fn new(n: usize) -> Self {
            RangeAddRangeSumFenwickTree {
                n,
                ft0: RangeSumFenwickTree::new(n + 1),
                ft1: RangeSumFenwickTree::new(n + 1),
            }
        }
        /// 配列のスライスから Range Add Range Sum Fenwick Tree を作成します。
        /// # 計算量
        /// O(n)
        pub fn from_slice(slice: &[T]) -> Self {
            let n = slice.len();
            let mut d = vec![zero(); n + 1];
            let mut di = vec![zero(); n + 1];
            if n > 0 {
                d[0] = slice[0];
                for i in 1..n {
                    let val = slice[i] - slice[i - 1];
                    d[i] = val;
                    di[i] = val * (i as i64);
                }
                d[n] = -slice[n - 1];
                di[n] = d[n] * (n as i64);
            }
            Self {
                n,
                ft0: RangeSumFenwickTree::from_slice(&d),
                ft1: RangeSumFenwickTree::from_slice(&di),
            }
        }
        /// 指定された範囲 `range` に `val` を加算します。
        /// # 計算量
        /// O(log n)
        pub fn range_add<R>(&mut self, range: R, val: T)
        where
            R: RangeBounds<usize>,
        {
            let (l, r) = self.resolve_range(range);
            assert!(
                l <= r && r <= self.n,
                "RangeAddRangeSumFenwickTree::range_add: invalid range. l: {}, r: {}, n: {}",
                l,
                r,
                self.n
            );
            self.ft0.add(l, val);
            self.ft0.add(r, -val);
            let l_val = val * (l as i64);
            let r_val = (-val) * (r as i64);
            self.ft1.add(l, l_val);
            self.ft1.add(r, r_val);
        }
        /// `idx` 番目の要素に `val` を加算します。
        /// # 計算量
        /// O(log n)
        pub fn add(&mut self, idx: usize, val: T) {
            self.range_add(idx..=idx, val);
        }
        /// `idx` 番目の要素の値を `val` に設定します。
        /// # 計算量
        /// O(log n)
        pub fn set(&mut self, idx: usize, val: T) {
            let old = self.get(idx);
            self.add(idx, val - old);
        }
        /// `[0, idx)` の区間和を計算します。
        /// # 計算量
        /// O(log n)
        pub fn prefix_sum(&self, idx: usize) -> T {
            let sum0 = self.ft0.prefix_sum(idx);
            let sum1 = self.ft1.prefix_sum(idx);
            sum0 * (idx as i64) - sum1
        }
        /// 指定された範囲 `range` の区間和を計算します。
        /// # Panics
        /// 範囲が不正な場合にパニックします。
        /// # 計算量
        /// O(log n)
        pub fn range_sum<R>(&self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            let (l, r) = self.resolve_range(range);
            assert!(
                l <= r && r <= self.n,
                "RangeAddRangeSumFenwickTree::range_sum: invalid range. l: {}, r: {}, n: {}",
                l,
                r,
                self.n
            );
            self.prefix_sum(r) - self.prefix_sum(l)
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
        /// `p` 番目の要素を取得します。
        /// # 計算量
        /// O(log n)
        pub fn get(&self, p: usize) -> T {
            self.range_sum(p..=p)
        }
        /// `l` を左端として、`f(sum(l..r))` が true になる最大の `r` を返します。
        /// `f` は単調性を持つ必要があります。具体的には元の配列の要素がすべて非負である必要があります。
        /// また、`f(0)` は true である必要があります。
        /// # Panics
        /// `l > n` または `f(0)` が false の場合にパニックします。
        /// # 計算量
        /// O(log n)
        pub fn max_right<F>(&self, l: usize, mut f: F) -> usize
        where
            F: FnMut(&T) -> bool,
        {
            assert!(
                l <= self.n,
                "RangeAddRangeSumFenwickTree::max_right: index out of bounds. l: {}, n: {}",
                l,
                self.n
            );
            assert!(
                f(&zero()),
                "RangeAddRangeSumFenwickTree::max_right: The predicate f(0) must be true."
            );
            let val_l = self.prefix_sum(l);
            let mut r = 0;
            let mut sum0: T = zero();
            let mut sum1: T = zero();
            let mut k = if self.n + 1 == 0 {
                0
            } else {
                1 << (usize::BITS - 1 - (self.n + 1).leading_zeros())
            };
            while k > 0 {
                if r + k <= self.n {
                    let next_sum0 = sum0 + self.ft0.ary[r + k - 1];
                    let next_sum1 = sum1 + self.ft1.ary[r + k - 1];
                    let total_sum = next_sum0 * ((r + k) as i64) - next_sum1;
                    let current_range_sum = total_sum - val_l;
                    if r + k <= l || f(&current_range_sum) {
                        r += k;
                        sum0 = next_sum0;
                        sum1 = next_sum1;
                    }
                }
                k >>= 1;
            }
            r
        }
        /// `r` を右端として、`f(sum(l..r))` が true になる最小の `l` を返します。
        /// `f` は単調性を持つ必要があります。具体的には元の配列の要素がすべて非負である必要があります。
        /// また、`f(0)` は true である必要があります。
        /// # Panics
        /// `r > n` または `f(0)` が false の場合にパニックします。
        /// # 計算量
        /// O(log n)
        pub fn min_left<F>(&self, r: usize, mut f: F) -> usize
        where
            F: FnMut(&T) -> bool,
        {
            assert!(
                r <= self.n,
                "RangeAddRangeSumFenwickTree::min_left: index out of bounds. r: {}, n: {}",
                r,
                self.n
            );
            assert!(
                f(&zero()),
                "RangeAddRangeSumFenwickTree::min_left: The predicate f(0) must be true."
            );
            let val_r = self.prefix_sum(r);
            if f(&val_r) {
                return 0;
            }
            let mut idx = 0;
            let mut sum0: T = zero();
            let mut sum1: T = zero();
            let mut k = if self.n + 1 == 0 {
                0
            } else {
                1 << (usize::BITS - 1 - (self.n + 1).leading_zeros())
            };
            while k > 0 {
                if idx + k <= r {
                    let next_sum0 = sum0 + self.ft0.ary[idx + k - 1];
                    let next_sum1 = sum1 + self.ft1.ary[idx + k - 1];
                    let total_sum = next_sum0 * ((idx + k) as i64) - next_sum1;
                    let current_range_sum = val_r - total_sum;
                    if !f(&current_range_sum) {
                        idx += k;
                        sum0 = next_sum0;
                        sum1 = next_sum1;
                    }
                }
                k >>= 1;
            }
            idx + 1
        }
        /// 現在の状態を `Vec<T>` として返します。
        /// # 計算量
        /// O(n log n)
        pub fn to_vec(&self) -> Vec<T> {
            (0..self.n).map(|i| self.get(i)).collect()
        }
        /// 保持している要素数を返します。
        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            self.n
        }
    }
}
use mod_combinatorics::*;
pub mod mod_combinatorics {
    use ac_library::modint::ModIntBase;
    #[derive(Clone, Debug)]
    pub struct Comb<Mint: ModIntBase> {
        fac: Vec<Mint>,
        invfac: Vec<Mint>,
    }
    impl<Mint: ModIntBase> Comb<Mint> {
        /// 階乗とその逆元を `max_val` まで前計算する。
        /// 計算量: O(max_val)
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
            assert!(
                n < self.fac.len(),
                "index out of range (n={}, max_val={})",
                n,
                self.fac.len() - 1
            );
            if n < k {
                0.into()
            } else {
                self.fac[n] * self.invfac[k] * self.invfac[n - k]
            }
        }
        pub fn perm(&self, n: usize, k: usize) -> Mint {
            assert!(
                n < self.fac.len(),
                "index out of range (n={}, max_val={})",
                n,
                self.fac.len() - 1
            );
            if n < k {
                0.into()
            } else {
                self.fac[n] * self.invfac[n - k]
            }
        }
        pub fn factorial(&self, n: usize) -> Mint {
            assert!(
                n < self.fac.len(),
                "index out of range (n={}, max_val={})",
                n,
                self.fac.len() - 1
            );
            self.fac[n]
        }
        pub fn inv_factorial(&self, n: usize) -> Mint {
            assert!(
                n < self.invfac.len(),
                "index out of range (n={}, max_val={})",
                n,
                self.invfac.len() - 1
            );
            self.invfac[n]
        }
    }
}
