// 問題文と制約は読みましたか？
use ac_library::ModInt998244353 as Mint;

// #[fastout]
fn main() {
    input! {
        xs: Chars,
    }

    let xs = xs
        .iter()
        .copied()
        .map(|ch| ch as usize - '0' as usize)
        .collect_vec();
    let n = xs.len();

    // dp[i][smaller][桁わ mod 3][3 を含むか][種類] = 場合の数
    let mut dp = vec![
        vec![
            vec![
                vec![
                    vec![
                        // a
                        Mint::new(0); 1024
                    ];
                    2
                ];
                3
            ];
            2
        ];
        n + 1
    ];

    dp[0][0][0][0][0] = Mint::new(1);

    for i in 0..n {
        for rem in 0..3 {
            for contains3 in [0, 1] {
                for kind in 0..1024 {
                    let kind = BitSet::new(kind);

                    let dp_false = dp[i][0][rem][contains3][kind];
                    let dp_true = dp[i][1][rem][contains3][kind];

                    let begin = if i == 0 { 1 } else { 0 };

                    for x in begin..10 {
                        dp[i + 1][1][(rem + x) % 3][(contains3 == 1 || x == 3) as usize]
                            [kind.inserted(x)] += dp_true;
                    }

                    for x in begin..xs[i] {
                        dp[i + 1][1][(rem + x) % 3][(contains3 == 1 || x == 3) as usize]
                            [kind.inserted(x)] += dp_false;
                    }

                    dp[i + 1][0][(rem + xs[i]) % 3][(contains3 == 1 || xs[i] == 3) as usize]
                        [kind.inserted(xs[i])] += dp_false;
                }
            }
        }

        if i != 0 {
            for x in 1..10 {
                // 0...0 から新しく生やす(dp_false, dp_true を使わない)
                dp[i + 1][1][x % 3][(x == 3) as usize][BitSet::empty().inserted(x)] += Mint::new(1);
            }
        }
    }

    // println!("{:?}", dp);

    // 3の倍数、3は含まれない、3種類ではない
    let cnt1 = [0, 1]
        .iter()
        .map(|&smaller| {
            BitSet::all_subset(10)
                .filter(|set| set.len() != 3)
                .map(|set| dp[n][smaller][0][0][set])
                .sum::<Mint>()
        })
        .sum::<Mint>();

    // 3の倍数ではない、3が含まれる、3種類ではない
    let cnt2 = [0, 1]
        .iter()
        .map(|&smaller| {
            [1, 2]
                .iter()
                .copied()
                .map(|rem| {
                    BitSet::all_subset(10)
                        .filter(|set| set.len() != 3)
                        .map(|set| dp[n][smaller][rem][1][set])
                        .sum::<Mint>()
                })
                .sum::<Mint>()
        })
        .sum::<Mint>();

    // 3の倍数ではない、3は含まれない、ちょうど3種類
    let cnt3 = [0, 1]
        .iter()
        .map(|&smaller| {
            [1, 2]
                .iter()
                .copied()
                .map(|rem| {
                    BitSet::all_subset(10)
                        .filter(|set| set.len() == 3)
                        .map(|set| dp[n][smaller][rem][0][set])
                        .sum::<Mint>()
                })
                .sum::<Mint>()
        })
        .sum::<Mint>();

    let ans = cnt1 + cnt2 + cnt3;
    println!("{}", ans);
}

fn solve_naive(n: i64) -> i64 {
    let cnt1 = (1..=n)
        .filter(|&x| {
            let ds = to_digits_le_vec(x, 10);
            let cond1 = x % 3 == 0;
            let cond2 = ds.contains(&3);
            let cond3 = ds.iter().copied().collect::<HashSet<_>>().len() == 3;
            cond1 && !cond2 && !cond3
        })
        .count() as i64;

    let cnt2 = (1..=n)
        .filter(|&x| {
            let ds = to_digits_le_vec(x, 10);
            let cond1 = x % 3 == 0;
            let cond2 = ds.contains(&3);
            let cond3 = ds.iter().copied().collect::<HashSet<_>>().len() == 3;
            !cond1 && cond2 && !cond3
        })
        .count() as i64;

    let cnt3 = (1..=n)
        .filter(|&x| {
            let ds = to_digits_le_vec(x, 10);
            let cond1 = x % 3 == 0;
            let cond2 = ds.contains(&3);
            let cond3 = ds.iter().copied().collect::<HashSet<_>>().len() == 3;
            !cond1 && !cond2 && cond3
        })
        .count() as i64;
    dbg!(cnt1);
    dbg!(cnt2);
    dbg!(cnt3);

    cnt1 + cnt2 + cnt3
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
        dbg!(solve_naive(1013));
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
use bitset::*;
#[allow(clippy::module_inception)]
pub mod bitset {
    use itertools::Itertools;
    use std::{
        fmt::{Error, Formatter},
        ops::{BitAnd, BitOr, BitXor, Index, IndexMut},
    };
    /// `usize` をビットフラグとして用い、要素数 64 までの集合を管理する構造体です。
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct BitSet {
        bit: usize,
    }
    impl BitSet {
        /// 指定されたビット値を持つ `BitSet` を作成します。
        /// 下から i ビット目 (2^i の位) が 1 であるとき、要素 i が集合に含まれることに対応します。
        #[inline]
        pub fn new(bit: usize) -> BitSet {
            BitSet { bit }
        }
        /// 内部のビット表現を整数として返します。
        pub fn to_bit(self) -> usize {
            self.bit
        }
        /// 範囲 [0, size) で集合に含まれている要素を `Vec<usize>` で返します。
        pub fn to_vec(self, size: usize) -> Vec<usize> {
            (0..size).filter(|i| (self.bit >> i) & 1 == 1).collect_vec()
        }
        /// 範囲 [0, size) で集合に含まれている要素を列挙するイテレータを返します。
        pub fn to_iter(self, size: usize) -> impl Iterator<Item = usize> {
            (0..size).filter(move |i| (self.bit >> i) & 1 == 1)
        }
        /// 指定された要素 `x` が集合に含まれているかを判定します。
        pub fn contains(self, x: usize) -> bool {
            (self.bit >> x) & 1 == 1
        }
        /// 集合に含まれる要素の数を返します。
        pub fn len(self) -> usize {
            self.bit.count_ones() as usize
        }
        /// 集合に含まれる最小の要素を返します。集合が空の場合は `None` を返します。
        pub fn min_element(self) -> Option<usize> {
            if self.is_empty() {
                None
            } else {
                Some(self.bit.trailing_zeros() as usize)
            }
        }
        /// 集合に含まれる最大の要素を返します。集合が空の場合は `None` を返します。
        pub fn max_element(self) -> Option<usize> {
            if self.is_empty() {
                None
            } else {
                Some(usize::BITS as usize - 1 - self.bit.leading_zeros() as usize)
            }
        }
        /// 集合に含まれない最小の非負整数 (MEX) を返します。
        pub fn mex_element(self) -> usize {
            self.bit.trailing_ones() as usize
        }
        /// 要素 `x` を追加した新しい `BitSet` を返します。
        #[must_use]
        pub fn inserted(self, x: usize) -> BitSet {
            BitSet::new(self.bit | (1 << x))
        }
        /// 要素 `x` を削除した新しい `BitSet` を返します。
        #[must_use]
        pub fn removed(self, x: usize) -> BitSet {
            BitSet::new(self.bit & !(1 << x))
        }
        /// 空集合を作成します。
        pub fn empty() -> BitSet {
            BitSet::new(0)
        }
        /// 全体集合 [0, size) を作成します。
        pub fn universal_set(size: usize) -> BitSet {
            BitSet::new((1 << size) - 1)
        }
        /// 全体集合を [0, size) としたときの補集合を返します。
        #[must_use]
        pub fn complement(self, size: usize) -> BitSet {
            BitSet::new(self.bit ^ ((1 << size) - 1))
        }
        /// 差集合 `self \ other` を返します。
        #[must_use]
        pub fn set_minus(self, other: BitSet) -> BitSet {
            BitSet::new(self.bit & !other.bit)
        }
        /// 集合が空であるかを判定します。
        pub fn is_empty(self) -> bool {
            self.bit == 0
        }
        /// `self` が `other` の部分集合であるかを判定します。
        pub fn is_subset(self, other: BitSet) -> bool {
            self | other == other
        }
        /// 2つの集合が共通部分を持たない（互いに素である）かを判定します。
        pub fn is_disjoint(self, other: BitSet) -> bool {
            (self.bit & other.bit) == 0
        }
        /// 全体集合 [0, size) のすべての部分集合を列挙するイテレータを返します。
        pub fn all_subset(size: usize) -> impl Iterator<Item = BitSet> {
            (0..(1 << size)).map(BitSet::new)
        }
        /// `self` のすべての部分集合を降順に列挙するイテレータを返します。
        pub fn subsets(self) -> impl Iterator<Item = BitSet> {
            std::iter::successors(Some(self.bit), move |x| {
                if *x == 0 {
                    None
                } else {
                    Some((x - 1) & self.bit)
                }
            })
            .map(BitSet::new)
        }
        /// 全体集合 [0, size) の範囲で、self を部分集合として含むすべての集合を降順に列挙するイテレータを返します。
        pub fn supersets(self, size: usize) -> impl Iterator<Item = BitSet> {
            let complement = Self::universal_set(size).set_minus(self);
            complement.subsets().map(move |s| self | s)
        }
    }
    impl BitAnd for BitSet {
        type Output = BitSet;
        fn bitand(self, rhs: BitSet) -> BitSet {
            BitSet::new(self.bit & rhs.bit)
        }
    }
    impl BitOr for BitSet {
        type Output = BitSet;
        fn bitor(self, rhs: BitSet) -> BitSet {
            BitSet::new(self.bit | rhs.bit)
        }
    }
    impl BitXor for BitSet {
        type Output = BitSet;
        fn bitxor(self, rhs: BitSet) -> BitSet {
            BitSet::new(self.bit ^ rhs.bit)
        }
    }
    use std::fmt::Debug;
    impl Debug for BitSet {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            f.write_fmt(format_args!("{:#b}", self.bit))?;
            Ok(())
        }
    }
    impl<T> Index<BitSet> for [T] {
        type Output = T;
        fn index(&self, s: BitSet) -> &Self::Output {
            &self[s.to_bit()]
        }
    }
    impl<T> IndexMut<BitSet> for [T] {
        fn index_mut(&mut self, s: BitSet) -> &mut Self::Output {
            &mut self[s.to_bit()]
        }
    }
    impl<T> Index<BitSet> for Vec<T> {
        type Output = T;
        fn index(&self, s: BitSet) -> &Self::Output {
            &self[..][s]
        }
    }
    impl<T> IndexMut<BitSet> for Vec<T> {
        fn index_mut(&mut self, s: BitSet) -> &mut Self::Output {
            &mut self[..][s]
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
