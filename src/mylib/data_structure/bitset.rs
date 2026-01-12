use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use bitset::*;")]
pub mod bitset {
    use std::{
        fmt::{Error, Formatter},
        ops::{BitAnd, BitOr, BitXor, Index, IndexMut},
    };

    use itertools::Itertools;

    /// `usize` をビットフラグとして用い、要素数 64 までの集合を管理する構造体です。
    #[derive(Clone, Copy, PartialEq, Eq)]
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
        pub fn inserted(self, x: usize) -> BitSet {
            BitSet::new(self.bit | (1 << x))
        }

        /// 要素 `x` を削除した新しい `BitSet` を返します。
        pub fn removed(self, x: usize) -> BitSet {
            BitSet::new(self.bit & !(1 << x))
        }

        /// 空集合を作成します。
        pub fn empty() -> BitSet {
            BitSet::new(0)
        }

        /// 全体集合 [0, size) を作成します。
        pub fn universal_set(size: usize) -> BitSet {
            // size = 64 のときオーバーフローするため注意
            BitSet::new((1 << size) - 1)
        }

        /// 全体集合を [0, size) としたときの補集合を返します。
        pub fn complement(self, size: usize) -> BitSet {
            // size = 64 のときオーバーフローするため注意
            BitSet::new(self.bit ^ ((1 << size) - 1))
        }

        /// 差集合 `self \ other` を返します。
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
            // size = 64 のときオーバーフローするため注意
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

    // [T] に対する実装
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
            &self[..][s] // スライスの Index 実装を利用
        }
    }

    impl<T> IndexMut<BitSet> for Vec<T> {
        fn index_mut(&mut self, s: BitSet) -> &mut Self::Output {
            &mut self[..][s] // スライスの IndexMut 実装を利用
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bitset::BitSet;

    #[test]
    fn test_new() {
        let b = BitSet::new(0b101);
        assert_eq!(b.to_bit(), 0b101);
    }

    #[test]
    fn test_to_vec() {
        let b = BitSet::new(0b101);
        let vec = b.to_vec(3);
        assert_eq!(vec, vec![0, 2]);
    }

    #[test]
    fn test_to_iter() {
        let b = BitSet::new(0b101);
        let elements: Vec<usize> = b.to_iter(3).collect();
        assert_eq!(elements, vec![0, 2]);
    }

    #[test]
    fn test_contains() {
        let b = BitSet::new(0b101);
        assert!(b.contains(0));
        assert!(!b.contains(1));
        assert!(b.contains(2));
    }

    #[test]
    fn test_len() {
        let b = BitSet::new(0b101);
        assert_eq!(b.len(), 2);

        let b_empty = BitSet::new(0b0);
        assert_eq!(b_empty.len(), 0);
    }

    #[test]
    fn test_min_max_element() {
        let b = BitSet::new(0b1010); // elements {1, 3}
        assert_eq!(b.min_element(), Some(1));
        assert_eq!(b.max_element(), Some(3));

        let b_empty = BitSet::empty();
        assert_eq!(b_empty.min_element(), None);
        assert_eq!(b_empty.max_element(), None);

        let b_single = BitSet::new(1 << 5);
        assert_eq!(b_single.min_element(), Some(5));
        assert_eq!(b_single.max_element(), Some(5));
    }

    #[test]
    fn test_mex_element() {
        let b = BitSet::new(0b1011); // {0, 1, 3}, missing 2
        assert_eq!(b.mex_element(), 2);

        let b_empty = BitSet::empty();
        assert_eq!(b_empty.mex_element(), 0);

        let b_full = BitSet::new(!0);
        assert_eq!(b_full.mex_element(), 64);
    }

    #[test]
    fn test_inserted() {
        let b = BitSet::new(0b101);
        let b = b.inserted(1);
        assert_eq!(b.to_bit(), 0b111);

        let b = b.inserted(1);
        assert_eq!(b.to_bit(), 0b111);
    }

    #[test]
    fn test_removed() {
        let b = BitSet::new(0b111);
        let b = b.removed(1);
        assert_eq!(b.to_bit(), 0b101);

        let b = b.removed(1);
        assert_eq!(b.to_bit(), 0b101);
    }

    #[test]
    fn test_empty() {
        let b = BitSet::empty();
        assert_eq!(b.to_bit(), 0b0);
    }

    #[test]
    fn test_universal_set() {
        let b = BitSet::universal_set(3);
        assert_eq!(b.to_bit(), 0b111);

        let b = BitSet::universal_set(0);
        assert_eq!(b.to_bit(), 0b0);
    }

    #[test]
    fn test_complement() {
        let b = BitSet::new(0b101);
        let complement = b.complement(3);
        assert_eq!(complement.to_bit(), 0b010);

        let b = BitSet::new(0b0);
        let complement = b.complement(0);
        assert_eq!(complement.to_bit(), 0b0);
    }

    #[test]
    fn test_set_minus() {
        let b1 = BitSet::new(0b111);
        let b2 = BitSet::new(0b101);
        let result = b1.set_minus(b2);
        assert_eq!(result.to_bit(), 0b010);

        let b1 = BitSet::new(0b0110);
        let b2 = BitSet::new(0b1010);
        let result = b1.set_minus(b2);
        assert_eq!(result.to_bit(), 0b0100);
    }

    #[test]
    fn test_is_empty() {
        let b1 = BitSet::new(0b0);
        assert!(b1.is_empty());

        let b2 = BitSet::new(0b101);
        assert!(!b2.is_empty());
    }

    #[test]
    fn test_is_subset() {
        let b1 = BitSet::new(0b101);
        let b2 = BitSet::new(0b111);
        let b3 = BitSet::new(0b100);

        assert!(b1.is_subset(b2));
        assert!(!b2.is_subset(b1));
        assert!(b3.is_subset(b2));
        assert!(b3.is_subset(b1));
        assert!(!b1.is_subset(b3));

        let b_empty = BitSet::new(0b0);
        assert!(b_empty.is_subset(b1));
        assert!(b_empty.is_subset(b2));
    }

    #[test]
    fn test_is_disjoint() {
        let b1 = BitSet::new(0b101);
        let b2 = BitSet::new(0b010);
        let b3 = BitSet::new(0b110);

        assert!(b1.is_disjoint(b2));
        assert!(!b1.is_disjoint(b3));
        assert!(!b2.is_disjoint(b3));
        assert!(BitSet::empty().is_disjoint(b1));
    }

    #[test]
    fn test_all_subset() {
        let subsets: Vec<BitSet> = BitSet::all_subset(3).collect();
        let expected: Vec<BitSet> = vec![
            BitSet::new(0b000),
            BitSet::new(0b001),
            BitSet::new(0b010),
            BitSet::new(0b011),
            BitSet::new(0b100),
            BitSet::new(0b101),
            BitSet::new(0b110),
            BitSet::new(0b111),
        ];
        assert_eq!(subsets, expected);

        let subsets: Vec<BitSet> = BitSet::all_subset(0).collect();
        let expected: Vec<BitSet> = vec![BitSet::new(0b0)];
        assert_eq!(subsets, expected);
    }

    #[test]
    fn test_subset_of() {
        let b = BitSet::new(0b101);
        let subsets: Vec<BitSet> = b.subsets().collect();
        let expected: Vec<BitSet> = vec![
            BitSet::new(0b101),
            BitSet::new(0b100),
            BitSet::new(0b001),
            BitSet::new(0b000),
        ];
        assert_eq!(subsets, expected);
    }

    #[test]
    fn test_supersets() {
        let b = BitSet::new(0b101); // {0, 2}
        let supersets: Vec<BitSet> = b.supersets(3).collect();
        // Supersets of {0, 2} within [0, 3): {0, 2}, {0, 1, 2}
        // Note: bit order in subsets() is descending, so 0b111 comes first
        let expected: Vec<BitSet> = vec![BitSet::new(0b111), BitSet::new(0b101)];
        assert_eq!(supersets, expected);
    }

    #[test]
    fn test_bitand() {
        let b1 = BitSet::new(0b101);
        let b2 = BitSet::new(0b011);
        let result = b1 & b2;
        assert_eq!(result.to_bit(), 0b001);
    }

    #[test]
    fn test_bitor() {
        let b1 = BitSet::new(0b101);
        let b2 = BitSet::new(0b010);
        let result = b1 | b2;
        assert_eq!(result.to_bit(), 0b111);
    }

    #[test]
    fn test_bitxor() {
        let b1 = BitSet::new(0b101);
        let b2 = BitSet::new(0b011);
        let result = b1 ^ b2;
        assert_eq!(result.to_bit(), 0b110);
    }

    #[test]
    fn test_debug() {
        let b1 = BitSet::new(0b100);
        assert_eq!(format!("{:?}", b1), "0b100");
    }

    #[test]
    fn test_index() {
        let vec = Vec::from([4, 5, 6, 7]);
        let s = BitSet::new(0b010);
        assert_eq!(vec[s], 6);
    }

    #[test]
    fn test_index_mut() {
        let mut vec = Vec::from([4, 5, 6, 7]);
        let s = BitSet::new(0b010);
        vec[s] = 60;
        assert_eq!(vec, Vec::from([4, 5, 60, 7]));
    }
}
