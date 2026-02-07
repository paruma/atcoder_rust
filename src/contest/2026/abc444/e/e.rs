// 問題文と制約は読みましたか？

fn solve(n: usize, d: i64, xs: &[i64]) -> i64 {
    todo!();
}
// #[fastout]
fn main() {
    input! {
        n: usize,
        d: i64,
        xs: [i64; n],
    }
    let space = xs
        .iter()
        .copied()
        .flat_map(|x| [x, x - d, x + d])
        .collect_vec();

    let cc = CoordinateCompression::new(&space);

    let mut distrib = RangeSumFenwickTreeI64::new(cc.space_size());

    let mut l = 0_usize;
    let mut sum = 0;
    for r in 0..n {
        // l を伸ばす (l<=r まで)
        // xs[l..=r] の要素のうち、開区間 (xs[r] - d, xs[r] + d) に含まれている個数 (0であってほしい)
        loop {
            let cnt = distrib.range_sum(cc.compress(xs[r] - d) + 1..cc.compress(xs[r] + d));
            if cnt > 0 {
                distrib.add(cc.compress(xs[l]), -1);
                l += 1;
            } else {
                break;
            }
        }
        let sub = r - l + 1;
        sum += sub;

        // 計算が終わったら distrib 更新
        distrib.add(cc.compress(xs[r]), 1);
    }

    println!("{}", sum);
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
        collections::{BinaryHeap, HashMap, HashSet},
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
        let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Display>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{}", x)).join(" ");
            println!("{}", msg);
        }
    }
    pub fn print_bytes(bytes: &[u8]) {
        let msg = String::from_utf8(bytes.to_vec()).unwrap();
        println!("{}", msg);
    }
    pub fn print_chars(chars: &[char]) {
        let msg = chars.iter().collect::<String>();
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec_bytes(vec_bytes: &[Vec<u8>]) {
        for row in vec_bytes {
            let msg = String::from_utf8(row.to_vec()).unwrap();
            println!("{}", msg);
        }
    }
    #[fastout]
    pub fn print_vec_chars(vec_chars: &[Vec<char>]) {
        for row in vec_chars {
            let msg = row.iter().collect::<String>();
            println!("{}", msg);
        }
    }
    pub fn print_yesno(ans: bool) {
        let msg = if ans { "Yes" } else { "No" };
        println!("{}", msg);
    }
}

// ====== snippet ======
use coordinate_compression::*;
#[allow(clippy::module_inception)]
pub mod coordinate_compression {
    use itertools::Itertools;
    use superslice::Ext;
    #[derive(Debug, Clone)]
    pub struct CoordinateCompression<T> {
        space: Vec<T>,
    }
    impl<T: Ord + Copy> CoordinateCompression<T> {
        /// # 計算量
        /// O(|space|log(|space|))
        pub fn new(space: &[T]) -> Self {
            let space = space.iter().copied().sorted().dedup().collect_vec();
            Self { space }
        }
        /// # 計算量
        /// O(log(|space|))
        pub fn compress(&self, x: T) -> usize {
            self.space.binary_search(&x).unwrap()
        }
        /// 座標圧縮前の空間のうち x 以下である最大の値を座標圧縮したものを返す
        /// # 計算量
        /// O(log(|space|))
        pub fn compress_floor(&self, x: T) -> usize {
            self.space.upper_bound(&x) - 1
        }
        /// 座標圧縮前の空間のうち x 以上である最小の値を座標圧縮したものを返す
        /// # 計算量
        /// O(log(|space|))
        pub fn compress_ceil(&self, x: T) -> usize {
            self.space.lower_bound(&x)
        }
        /// # 計算量
        /// O(|xs|log(|space|))
        pub fn compress_vec(&self, xs: &[T]) -> Vec<usize> {
            xs.iter().map(|&x| self.compress(x)).collect_vec()
        }
        /// # 計算量
        /// O(1)
        pub fn decompress(&self, i: usize) -> T {
            self.space[i]
        }
        pub fn space_size(&self) -> usize {
            self.space.len()
        }
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
