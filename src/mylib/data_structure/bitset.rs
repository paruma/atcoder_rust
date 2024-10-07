use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use bitset::*;")]
pub mod bitset {
    use std::{
        fmt::{Error, Formatter},
        ops::{BitAnd, BitOr, BitXor},
    };

    use itertools::Itertools;

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct BitSet {
        bit: usize,
    }

    impl BitSet {
        #[inline]
        pub fn new(bit: usize) -> BitSet {
            BitSet { bit }
        }

        pub fn to_bit(self) -> usize {
            self.bit
        }

        /// 持っている要素を Vec<usize> で返す
        pub fn to_vec(self, len: usize) -> Vec<usize> {
            (0..len).filter(|i| (self.bit >> i) & 1 == 1).collect_vec()
        }

        /// 持っている要素を Iterator で返す
        pub fn to_iter(self, len: usize) -> impl Iterator<Item = usize> {
            (0..len).filter(move |i| (self.bit >> i) & 1 == 1)
        }

        pub fn contains(self, x: usize) -> bool {
            (self.bit >> x) & 1 == 1
        }

        pub fn count(self) -> usize {
            self.bit.count_ones() as usize
        }

        pub fn insert(self, x: usize) -> BitSet {
            BitSet::new(self.bit | (1 << x))
        }

        pub fn remove(self, x: usize) -> BitSet {
            BitSet::new(self.bit & !(1 << x))
        }

        pub fn empty() -> BitSet {
            BitSet::new(0)
        }

        pub fn universal_set(size: usize) -> BitSet {
            BitSet::new((1 << size) - 1)
        }

        pub fn complement(self, size: usize) -> BitSet {
            BitSet::new(self.bit ^ ((1 << size) - 1))
        }

        pub fn set_minus(self, other: BitSet) -> BitSet {
            BitSet::new(self.bit & !other.bit)
        }

        pub fn is_empty(self) -> bool {
            self.bit == 0
        }

        pub fn is_subset(self, other: BitSet) -> bool {
            self | other == other
        }

        pub fn all_subset(size: usize) -> impl Iterator<Item = BitSet> {
            (0..(1 << size)).map(BitSet::new)
        }

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

        // 部分集合
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
    fn test_contains() {
        let b = BitSet::new(0b101);
        assert!(b.contains(0)); // true case
        assert!(!b.contains(1)); // false case
        assert!(b.contains(2)); // true case
    }

    #[test]
    fn test_count() {
        let b = BitSet::new(0b101);
        assert_eq!(b.count(), 2);

        let b_empty = BitSet::new(0b0);
        assert_eq!(b_empty.count(), 0);
    }

    #[test]
    fn test_insert() {
        let b = BitSet::new(0b101);
        let b = b.insert(1);
        assert_eq!(b.to_bit(), 0b111);

        let b = b.insert(1);
        assert_eq!(b.to_bit(), 0b111);
    }

    #[test]
    fn test_remove() {
        let b = BitSet::new(0b111);
        let b = b.remove(1);
        assert_eq!(b.to_bit(), 0b101);

        let b = b.remove(1);
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
        assert!(b1.is_empty()); // true case

        let b2 = BitSet::new(0b101);
        assert!(!b2.is_empty()); // false case
    }

    #[test]
    fn test_is_subset() {
        let b1 = BitSet::new(0b101);
        let b2 = BitSet::new(0b111);
        let b3 = BitSet::new(0b100);

        assert!(b1.is_subset(b2)); // true case
        assert!(!b2.is_subset(b1)); // false case
        assert!(b3.is_subset(b2)); // true case
        assert!(b3.is_subset(b1)); // true case
        assert!(!b1.is_subset(b3)); // false case

        let b_empty = BitSet::new(0b0);
        assert!(b_empty.is_subset(b1));
        assert!(b_empty.is_subset(b2));
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
}
