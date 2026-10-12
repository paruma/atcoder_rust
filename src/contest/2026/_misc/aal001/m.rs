use ac_library::Segtree;
// 問題文と制約は読みましたか？
use monoid_template::*;
#[allow(unused_variables)]
pub mod monoid_template {
    use ac_library::segtree::Monoid;
    use std::convert::Infallible;

    use crate::bitset::BitSet;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct RangeXxx {
        pub ans0: usize, // 0 入力の答え
        pub ans1: usize, // 1 入力の答え
    }
    impl RangeXxx {
        pub fn unit(x: usize) -> Self {
            Self {
                ans0: 0b_11111_11111_11111_11111_11111_11111, // 30個
                ans1: x ^ 0b_11111_11111_11111_11111_11111_11111,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeXxxMonoid(Infallible);
    impl Monoid for RangeXxxMonoid {
        type S = RangeXxx;
        fn identity() -> Self::S {
            RangeXxx {
                ans0: 0,
                ans1: 0b_11111_11111_11111_11111_11111_11111,
            }
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            let (a, b) = (b, a); // 合成順序が逆
            let a0set = BitSet::new(a.ans0);
            let a1set = BitSet::new(a.ans1);
            let b0set = BitSet::new(b.ans0);
            let b1set = BitSet::new(b.ans1);

            // a(b(0))
            let mut ans0set = BitSet::empty(); // 0 → 1 の場合に入れる
            for i in 0..30 {
                // b[i](0) = 1
                if b0set.contains(i) {
                    // a[i](1) = 1
                    if a1set.contains(i) {
                        ans0set = ans0set.inserted(i);
                    }
                } else {
                    if a0set.contains(i) {
                        ans0set = ans0set.inserted(i);
                    }
                }
            }
            // a(b(1))
            let mut ans1set = BitSet::empty(); // 1 → 1 の場合に入れる
            for i in 0..30 {
                // b[i](1) = 1
                if b1set.contains(i) {
                    // a[i](1) = 1
                    if a1set.contains(i) {
                        ans1set = ans1set.inserted(i);
                    }
                } else {
                    if a0set.contains(i) {
                        ans1set = ans1set.inserted(i);
                    }
                }
            }
            RangeXxx {
                ans0: ans0set.to_bit(),
                ans1: ans1set.to_bit(),
            }
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        nq: usize,
        xs: [usize; n],
    }

    let mut seg =
        Segtree::<RangeXxxMonoid>::from(xs.iter().copied().map(RangeXxx::unit).collect_vec());

    for _ in 0..nq {
        input! {
            t: usize
        }

        if t == 1 {
            input! {
                p: Usize1,
                x: usize,
            }
            seg.set(p, RangeXxx::unit(x))
        } else {
            input! {
                l: Usize1,
                r: Usize1,
            }
            let xl = seg.get(l).ans1 ^ 0b_11111_11111_11111_11111_11111_11111;

            if l == r {
                println!("{}", xl);
            } else {
                let xl_set = BitSet::new(xl);
                let p = seg.prod(l + 1..=r);
                let p0_set = BitSet::new(p.ans0);
                let p1_set = BitSet::new(p.ans1);
                let mut ans = BitSet::empty();
                for i in 0..30 {
                    if xl_set.contains(i) {
                        if p1_set.contains(i) {
                            ans = ans.inserted(i)
                        }
                    } else {
                        if p0_set.contains(i) {
                            ans = ans.inserted(i)
                        }
                    }
                }
                let ans = ans.to_bit();
                println!("{}", ans);
            }
        }
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
        ///
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
