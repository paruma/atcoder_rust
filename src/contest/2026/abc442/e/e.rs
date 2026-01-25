// #[fastout]
/// x軸正の向きを0度として、反時計回りを正とする偏角で順序を決める。
/// (0, 0) は未考慮。
pub fn argcmp((x0, y0): (i64, i64), (x1, y1): (i64, i64)) -> Ordering {
    ((y0, x0) < (0, 0))
        .cmp(&((y1, x1) < (0, 0)))
        .then_with(|| (x1 * y0).cmp(&(x0 * y1)))
}
fn main() {
    input! {
        n: usize,
        q: usize,
        ps: [PosXY; n],
        abs: [(Usize1, Usize1); q],
    }

    let ps = ps
        .iter()
        .copied()
        .map(|p| Pos::new(p.x, -p.y).normalize())
        .collect_vec();

    let sorted = ps
        .iter()
        .copied()
        .enumerate()
        .sorted_by(|(_i, pi), (_j, pj)| argcmp((pi.x, pi.y), (pj.x, pj.y)))
        .map(|(i, _)| i)
        .collect_vec();

    let rank = {
        let mut rank = vec![usize::MAX; n];
        let mut counter = 0;

        for i in 0..n {
            if i == 0 {
                rank[sorted[i]] = 0;
            } else if i >= 1 && ps[sorted[i]] == ps[sorted[i - 1]] {
                rank[sorted[i]] = counter;
            } else {
                counter += 1;
                rank[sorted[i]] = counter;
            }
        }
        rank
    };

    let rank_max = rank.iter().copied().max().unwrap();

    let mut cnts = FenwickTreeI64::new((rank_max + 1) * 2);

    for &r in &rank {
        cnts.add(r, 1);
        cnts.add(r + (rank_max + 1), 1);
    }

    // dbg!(cnts.to_vec());
    // dbg!(&rank);

    let ans: Vec<i64> = abs
        .iter()
        .copied()
        .map(|(a, b)| {
            let rank_a = rank[a];
            let rank_b = rank[b];
            if rank_a > rank_b {
                cnts.range_sum(rank_a..=(rank_b + (rank_max + 1)))
            } else {
                cnts.range_sum(rank_a..=rank_b)
            }
        })
        .collect_vec();
    print_vec(&ans);
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
use std::cmp::Ordering;
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
use pos::*;
#[allow(clippy::module_inception)]
pub mod pos {
    use std::io::BufRead;
    use std::iter::Sum;
    use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Pos {
        pub x: i64,
        pub y: i64,
    }
    impl Pos {
        pub fn new(x: i64, y: i64) -> Pos {
            Pos { x, y }
        }
        pub fn scala_mul(self, rhs: i64) -> Pos {
            Pos::new(self.x * rhs, self.y * rhs)
        }
        pub fn inner_product(self, rhs: Self) -> i64 {
            self.x * rhs.x + self.y * rhs.y
        }
        pub fn outer_product(self, rhs: Self) -> i64 {
            self.x * rhs.y - self.y * rhs.x
        }
        pub fn norm_square(self) -> i64 {
            self.inner_product(self)
        }
        pub fn l1_norm(self) -> i64 {
            self.x.abs() + self.y.abs()
        }
        pub fn linf_norm(self) -> i64 {
            self.x.abs().max(self.y.abs())
        }
        pub fn dist_square(self, rhs: Self) -> i64 {
            (self - rhs).norm_square()
        }
        pub fn l1_dist(self, rhs: Self) -> i64 {
            (self - rhs).l1_norm()
        }
        pub fn linf_dist(self, rhs: Self) -> i64 {
            (self - rhs).linf_norm()
        }
        pub fn normalize(self) -> Pos {
            if self.x == 0 && self.y == 0 {
                return self;
            }
            let g = num::integer::gcd(self.x.abs(), self.y.abs());
            Pos::new(self.x / g, self.y / g)
        }
        pub fn rotate90(self) -> Pos {
            Pos::new(-self.y, self.x)
        }
        pub fn rotate270(self) -> Pos {
            Pos::new(self.y, -self.x)
        }
        /// グリッドの幅 `width` を指定して、座標 `(x, y)` を 1次元インデックス `y * width + x` に変換する。
        pub fn to_index_1d(self, width: usize) -> usize {
            assert!(
                self.x >= 0 && self.y >= 0,
                "Pos::to_index_1d: x と y は 0 以上である必要があります。pos: ({}, {})",
                self.x,
                self.y
            );
            assert!(
                (self.x as usize) < width,
                "Pos::to_index_1d: x は width 未満である必要があります。x: {}, width: {}",
                self.x,
                width
            );
            (self.y as usize) * width + (self.x as usize)
        }
        /// 1次元インデックスとグリッドの幅 `width` から、座標 `(x, y)` を復元する。
        pub fn from_index_1d(index: usize, width: usize) -> Pos {
            Pos::new((index % width) as i64, (index / width) as i64)
        }
        pub fn around4_pos_iter(self) -> impl Iterator<Item = Pos> {
            DIR4_LIST.iter().copied().map(move |d| self + d)
        }
        pub fn around8_pos_iter(self) -> impl Iterator<Item = Pos> {
            DIR8_LIST.iter().copied().map(move |d| self + d)
        }
    }
    impl Add for Pos {
        type Output = Pos;
        fn add(self, rhs: Self) -> Self::Output {
            Pos::new(self.x + rhs.x, self.y + rhs.y)
        }
    }
    impl Sub for Pos {
        type Output = Pos;
        fn sub(self, rhs: Self) -> Self::Output {
            Pos::new(self.x - rhs.x, self.y - rhs.y)
        }
    }
    impl Neg for Pos {
        type Output = Self;
        fn neg(self) -> Self::Output {
            Pos::new(-self.x, -self.y)
        }
    }
    impl Sum for Pos {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Pos::new(0, 0), |acc, x| acc + x)
        }
    }
    impl<'a> Sum<&'a Pos> for Pos {
        fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
            iter.fold(Pos::new(0, 0), |a, b| a + *b)
        }
    }
    impl num_traits::Zero for Pos {
        fn zero() -> Self {
            Pos::new(0, 0)
        }
        fn is_zero(&self) -> bool {
            self.x.is_zero() && self.y.is_zero()
        }
    }
    impl AddAssign for Pos {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }
    impl SubAssign for Pos {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }
    impl Mul<i64> for Pos {
        type Output = Pos;
        fn mul(self, rhs: i64) -> Self::Output {
            Pos::new(self.x * rhs, self.y * rhs)
        }
    }
    impl MulAssign<i64> for Pos {
        fn mul_assign(&mut self, rhs: i64) {
            *self = *self * rhs
        }
    }
    use std::fmt::{Debug, Error, Formatter};
    impl Debug for Pos {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            f.write_fmt(format_args!("({}, {})", self.x, self.y))?;
            Ok(())
        }
    }
    use proconio::source::{Readable, Source};
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum PosXY {}
    impl Readable for PosXY {
        type Output = Pos;
        fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Pos {
            let x = i64::read(source);
            let y = i64::read(source);
            Pos::new(x, y)
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum PosYX {}
    impl Readable for PosYX {
        type Output = Pos;
        fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Pos {
            let y = i64::read(source);
            let x = i64::read(source);
            Pos::new(x, y)
        }
    }
    /// 1-indexed で与えられた座標(YX)
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum PosYX1 {}
    impl Readable for PosYX1 {
        type Output = Pos;
        fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Pos {
            let y = i64::read(source) - 1;
            let x = i64::read(source) - 1;
            Pos::new(x, y)
        }
    }
    pub const DIR8_LIST: [Pos; 8] = [
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 1 },
        Pos { x: 1, y: 0 },
        Pos { x: 1, y: -1 },
        Pos { x: 0, y: -1 },
        Pos { x: -1, y: -1 },
        Pos { x: -1, y: 0 },
        Pos { x: -1, y: 1 },
    ];
    pub const DIR4_LIST: [Pos; 4] = [
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 0 },
        Pos { x: 0, y: -1 },
        Pos { x: -1, y: 0 },
    ];
}
use ab_group::*;
use fenwick_tree::*;
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
pub mod fenwick_tree {
    use super::{AbGroup, AdditiveAbGroup};
    use std::ops::{Bound, RangeBounds};
    /// 可換群 (AbGroup) を用いた汎用的な Fenwick Tree (Binary Indexed Tree)。
    /// 0-indexed で実装されています。
    /// 基本的な加算・区間和クエリに加え、get/set や、二分探索 (max_right / min_left) を提供します。
    #[derive(Clone)]
    pub struct FenwickTreeArbitrary<G: AbGroup> {
        n: usize,
        ary: Vec<G::S>,
    }
    /// i64 の加算群を用いた標準的な Fenwick Tree のエイリアス。
    pub type FenwickTreeI64 = FenwickTreeArbitrary<AdditiveAbGroup<i64>>;
    /// 任意の数値型 T の加算群を用いた Fenwick Tree のエイリアス。
    pub type FenwickTree<T> = FenwickTreeArbitrary<AdditiveAbGroup<T>>;
    impl<G: AbGroup> FenwickTreeArbitrary<G> {
        /// サイズ `n` の Fenwick Tree を作成します。
        /// 要素はすべて `G::zero()` で初期化されます。
        /// # 計算量
        /// O(n)
        pub fn new(n: usize) -> Self {
            let mut ary = Vec::with_capacity(n);
            for _ in 0..n {
                ary.push(G::zero());
            }
            FenwickTreeArbitrary { n, ary }
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
            FenwickTreeArbitrary { n, ary }
        }
        /// `[0, idx)` の区間の総和を計算します。
        /// # Panics
        /// `idx > n` の場合にパニックします。
        /// # 計算量
        /// O(log n)
        pub fn accum(&self, mut idx: usize) -> G::S {
            assert!(
                idx <= self.n,
                "FenwickTreeArbitrary::accum: index out of bounds. idx: {}, n: {}",
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
                "FenwickTreeArbitrary::add: index out of bounds. idx: {}, n: {}",
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
        /// 指定された範囲の区間和を計算します。
        /// # Panics
        /// 範囲が不正、または `n` を超える場合にパニックします。
        /// # 計算量
        /// O(log n)
        pub fn range_sum<R>(&self, range: R) -> G::S
        where
            R: RangeBounds<usize>,
        {
            let r = match range.end_bound() {
                Bound::Included(r) => r + 1,
                Bound::Excluded(r) => *r,
                Bound::Unbounded => self.n,
            };
            let l = match range.start_bound() {
                Bound::Included(l) => *l,
                Bound::Excluded(l) => l + 1,
                Bound::Unbounded => return self.accum(r),
            };
            assert!(
                l <= r && r <= self.n,
                "FenwickTreeArbitrary::range_sum: invalid range. l: {}, r: {}, n: {}",
                l,
                r,
                self.n
            );
            G::sub(&self.accum(r), &self.accum(l))
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
                "FenwickTreeArbitrary::max_right: index out of bounds. l: {}, n: {}",
                l,
                self.n
            );
            let zero = G::zero();
            assert!(
                f(&zero),
                "FenwickTreeArbitrary::max_right: The predicate f(zero) must be true."
            );
            let val_l = self.accum(l);
            let mut r = 0;
            let mut current_val = G::zero();
            let mut k = 1;
            while k <= self.n {
                k <<= 1;
            }
            k >>= 1;
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
                "FenwickTreeArbitrary::min_left: index out of bounds. r: {}, n: {}",
                r,
                self.n
            );
            let zero = G::zero();
            assert!(
                f(&zero),
                "FenwickTreeArbitrary::min_left: The predicate f(zero) must be true."
            );
            let val_r = self.accum(r);
            if f(&val_r) {
                return 0;
            }
            let mut idx = 0;
            let mut current_val = G::zero();
            let mut k = 1;
            while k <= self.n {
                k <<= 1;
            }
            k >>= 1;
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
                "FenwickTreeArbitrary::get: index out of bounds. idx: {}, n: {}",
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
                "FenwickTreeArbitrary::set: index out of bounds. idx: {}, n: {}",
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
