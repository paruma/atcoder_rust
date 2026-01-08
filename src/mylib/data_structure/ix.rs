use cargo_snippet::snippet;

/// C-like enum に対して `Ix` を実装するマクロです。
/// `range` メソッド内で `unsafe` な変換を行っているため、
/// 安全性のために対象 enum には `#[repr(usize)]` を付与することを推奨します。
#[macro_export]
macro_rules! impl_ix_for_enum {
    ($t:ty) => {
        impl Ix for $t {
            fn range((l, r): (Self, Self)) -> impl Iterator<Item = Self> {
                (l as usize..=r as usize).map(|i| unsafe {
                    let ptr = &i as *const usize;
                    ptr.cast::<Self>().read()
                })
            }

            fn range_size((l, r): (Self, Self)) -> usize {
                if l > r {
                    0
                } else {
                    (r as usize) - (l as usize) + 1
                }
            }

            fn to_index((l, r): (Self, Self), i: Self) -> usize {
                if !Self::in_range((l, r), i) {
                    panic!("index out of bounds");
                }
                (i as usize) - (l as usize)
            }

            fn from_index((l, r): (Self, Self), index: usize) -> Self {
                if index >= Self::range_size((l, r)) {
                    panic!("index out of range");
                }
                let val = l as usize + index;
                unsafe {
                    let ptr = &val as *const usize;
                    ptr.cast::<Self>().read()
                }
            }

            fn in_range((l, r): (Self, Self), i: Self) -> bool {
                l <= i && i <= r
            }
        }
    };
}

#[snippet(prefix = "use ix::*;")]
#[allow(clippy::module_inception)]
pub mod ix {
    use std::ops::{Index, IndexMut};

    /// Haskell の `Ix` 型クラスに相当するトレイトです。
    /// 連続する値の範囲を定義し、その範囲内の値を整数インデックスにマッピングするために使用されます。
    pub trait Ix: PartialOrd + Copy {
        /// 範囲内の全ての要素を順番に返すイテレータを返します。
        fn range(bounds: (Self, Self)) -> impl Iterator<Item = Self>;

        /// 指定された範囲に含まれる要素の数を返します。
        fn range_size(bounds: (Self, Self)) -> usize;

        /// 指定された範囲内における、値 `i` の 0 始まりのインデックスを返します。
        /// `i` が範囲外の場合はパニックします。
        fn to_index(bounds: (Self, Self), i: Self) -> usize;

        /// 指定された範囲内のインデックスから、元の値を復元します。
        fn from_index(bounds: (Self, Self), index: usize) -> Self;

        /// 値 `i` が指定された範囲内に含まれるかを判定します。
        fn in_range(bounds: (Self, Self), i: Self) -> bool;
    }

    macro_rules! impl_ix_for_integer {
        ($($t:ty),*) => {
            $(
                impl Ix for $t {
                    fn range((l, r): (Self, Self)) -> impl Iterator<Item = Self> {
                        l..=r
                    }

                    fn range_size((l, r): (Self, Self)) -> usize {
                        if l > r {
                            0
                        } else {
                            (l.abs_diff(r) as usize) + 1
                        }
                    }

                    fn to_index((l, r): (Self, Self), i: Self) -> usize {
                        if !Self::in_range((l, r), i) {
                            panic!("index out of bounds: {:?} is not in {:?}", i, (l, r));
                        }
                        (l.abs_diff(i) as usize)
                    }

                    fn from_index((l, r): (Self, Self), index: usize) -> Self {
                        if index >= Self::range_size((l, r)) {
                            panic!("index out of range: {} for bounds {:?}", index, (l, r));
                        }
                        l + index as Self
                    }

                    fn in_range((l, r): (Self, Self), i: Self) -> bool {
                        l <= i && i <= r
                    }
                }
            )*
        };
    }

    impl_ix_for_integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

    impl Ix for bool {
        fn range((l, r): (Self, Self)) -> impl Iterator<Item = Self> {
            (l as u8..=r as u8).map(|i| i != 0)
        }

        fn range_size((l, r): (Self, Self)) -> usize {
            #[allow(clippy::bool_comparison)]
            if l > r {
                0
            } else {
                (r as usize) - (l as usize) + 1
            }
        }

        fn to_index((l, r): (Self, Self), i: Self) -> usize {
            if !Self::in_range((l, r), i) {
                panic!("index out of bounds");
            }
            (i as usize) - (l as usize)
        }

        fn from_index((l, r): (Self, Self), index: usize) -> Self {
            if index >= Self::range_size((l, r)) {
                panic!("index out of range");
            }
            (l as usize + index) != 0
        }

        fn in_range((l, r): (Self, Self), i: Self) -> bool {
            l <= i && i <= r
        }
    }

    impl Ix for char {
        fn range((l, r): (Self, Self)) -> impl Iterator<Item = Self> {
            l..=r
        }

        fn range_size((l, r): (Self, Self)) -> usize {
            if l > r {
                0
            } else {
                (u32::from(r) - u32::from(l)) as usize + 1
            }
        }

        fn to_index((l, r): (Self, Self), i: Self) -> usize {
            if !Self::in_range((l, r), i) {
                panic!("index out of bounds: {:?} is not in {:?}", i, (l, r));
            }
            (u32::from(i) - u32::from(l)) as usize
        }

        fn from_index((l, r): (Self, Self), index: usize) -> Self {
            if index >= Self::range_size((l, r)) {
                panic!("index out of range: {} for bounds {:?}", index, (l, r));
            }
            std::char::from_u32(u32::from(l) + index as u32).unwrap()
        }

        fn in_range((l, r): (Self, Self), i: Self) -> bool {
            l <= i && i <= r
        }
    }

    impl Ix for () {
        fn range(_: (Self, Self)) -> impl Iterator<Item = Self> {
            std::iter::once(())
        }

        fn range_size(_: (Self, Self)) -> usize {
            1
        }

        fn to_index(_: (Self, Self), _: Self) -> usize {
            0
        }

        fn from_index(_: (Self, Self), index: usize) -> Self {
            if index != 0 {
                panic!("index out of range");
            }
        }

        fn in_range(_: (Self, Self), _: Self) -> bool {
            true
        }
    }

    // Implement for Pairs
    impl<A: Ix, B: Ix> Ix for (A, B) {
        fn range(((l1, l2), (u1, u2)): (Self, Self)) -> impl Iterator<Item = Self> {
            A::range((l1, u1)).flat_map(move |i1| B::range((l2, u2)).map(move |i2| (i1, i2)))
        }

        fn range_size(((l1, l2), (u1, u2)): (Self, Self)) -> usize {
            A::range_size((l1, u1)) * B::range_size((l2, u2))
        }

        fn to_index(((l1, l2), (u1, u2)): (Self, Self), (i1, i2): Self) -> usize {
            let idx1 = A::to_index((l1, u1), i1);
            let idx2 = B::to_index((l2, u2), i2);
            let stride2 = B::range_size((l2, u2));
            idx1 * stride2 + idx2
        }

        fn from_index(((l1, l2), (u1, u2)): (Self, Self), index: usize) -> Self {
            let size2 = B::range_size((l2, u2));
            let idx1 = index / size2;
            let idx2 = index % size2;
            (A::from_index((l1, u1), idx1), B::from_index((l2, u2), idx2))
        }

        fn in_range(((l1, l2), (u1, u2)): (Self, Self), (i1, i2): Self) -> bool {
            A::in_range((l1, u1), i1) && B::in_range((l2, u2), i2)
        }
    }

    // Implement for Triples
    impl<A: Ix, B: Ix, C: Ix> Ix for (A, B, C) {
        fn range(((l1, l2, l3), (u1, u2, u3)): (Self, Self)) -> impl Iterator<Item = Self> {
            A::range((l1, u1)).flat_map(move |i1| {
                B::range((l2, u2)).flat_map(move |i2| C::range((l3, u3)).map(move |i3| (i1, i2, i3)))
            })
        }

        fn range_size(((l1, l2, l3), (u1, u2, u3)): (Self, Self)) -> usize {
            A::range_size((l1, u1)) * B::range_size((l2, u2)) * C::range_size((l3, u3))
        }

        fn to_index(((l1, l2, l3), (u1, u2, u3)): (Self, Self), (i1, i2, i3): Self) -> usize {
            let idx1 = A::to_index((l1, u1), i1);
            let idx2 = B::to_index((l2, u2), i2);
            let idx3 = C::to_index((l3, u3), i3);
            let size2 = B::range_size((l2, u2));
            let size3 = C::range_size((l3, u3));
            (idx1 * size2 + idx2) * size3 + idx3
        }

        fn from_index(((l1, l2, l3), (u1, u2, u3)): (Self, Self), index: usize) -> Self {
            let size3 = C::range_size((l3, u3));
            let size23 = B::range_size((l2, u2)) * size3;
            let idx1 = index / size23;
            let idx2 = (index % size23) / size3;
            let idx3 = index % size3;
            (
                A::from_index((l1, u1), idx1),
                B::from_index((l2, u2), idx2),
                C::from_index((l3, u3), idx3),
            )
        }

        fn in_range(((l1, l2, l3), (u1, u2, u3)): (Self, Self), (i1, i2, i3): Self) -> bool {
            A::in_range((l1, u1), i1) && B::in_range((l2, u2), i2) && C::in_range((l3, u3), i3)
        }
    }

    // Implement for Quadruples
    impl<A: Ix, B: Ix, C: Ix, D: Ix> Ix for (A, B, C, D) {
        fn range(((l1, l2, l3, l4), (u1, u2, u3, u4)): (Self, Self)) -> impl Iterator<Item = Self> {
            A::range((l1, u1)).flat_map(move |i1| {
                B::range((l2, u2)).flat_map(move |i2| {
                    C::range((l3, u3))
                        .flat_map(move |i3| D::range((l4, u4)).map(move |i4| (i1, i2, i3, i4)))
                })
            })
        }

        fn range_size(((l1, l2, l3, l4), (u1, u2, u3, u4)): (Self, Self)) -> usize {
            A::range_size((l1, u1))
                * B::range_size((l2, u2))
                * C::range_size((l3, u3))
                * D::range_size((l4, u4))
        }

        fn to_index(((l1, l2, l3, l4), (u1, u2, u3, u4)): (Self, Self), (i1, i2, i3, i4): Self) -> usize {
            let idx1 = A::to_index((l1, u1), i1);
            let idx2 = B::to_index((l2, u2), i2);
            let idx3 = C::to_index((l3, u3), i3);
            let idx4 = D::to_index((l4, u4), i4);
            let size2 = B::range_size((l2, u2));
            let size3 = C::range_size((l3, u3));
            let size4 = D::range_size((l4, u4));
            ((idx1 * size2 + idx2) * size3 + idx3) * size4 + idx4
        }

        fn from_index(((l1, l2, l3, l4), (u1, u2, u3, u4)): (Self, Self), index: usize) -> Self {
            let size4 = D::range_size((l4, u4));
            let size34 = C::range_size((l3, u3)) * size4;
            let size234 = B::range_size((l2, u2)) * size34;
            let idx1 = index / size234;
            let idx2 = (index % size234) / size34;
            let idx3 = (index % size34) / size4;
            let idx4 = index % size4;
            (
                A::from_index((l1, u1), idx1),
                B::from_index((l2, u2), idx2),
                C::from_index((l3, u3), idx3),
                D::from_index((l4, u4), idx4),
            )
        }

        fn in_range(
            ((l1, l2, l3, l4), (u1, u2, u3, u4)): (Self, Self),
            (i1, i2, i3, i4): Self,
        ) -> bool {
            A::in_range((l1, u1), i1)
                && B::in_range((l2, u2), i2)
                && C::in_range((l3, u3), i3)
                && D::in_range((l4, u4), i4)
        }
    }

    /// 範囲を表す構造体です。
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Bounds<T> {
        pub min: T,
        pub max: T,
    }

    impl<T: Ix> Bounds<T> {
        /// 新しい範囲を作成します。
        pub fn new(min: T, max: T) -> Self {
            Self { min, max }
        }

        /// 指定された範囲に含まれる要素の数を返します。
        pub fn range_size(&self) -> usize {
            T::range_size((self.min, self.max))
        }

        /// 指定された範囲内における、値 `val` の 0 始まりのインデックスを返します。
        pub fn to_index(&self, val: T) -> usize {
            T::to_index((self.min, self.max), val)
        }

        /// 指定された範囲内のインデックスから、元の値を復元します。
        pub fn from_index(&self, index: usize) -> T {
            T::from_index((self.min, self.max), index)
        }

        /// 値 `val` が指定された範囲内に含まれるかを判定します。
        pub fn in_range(&self, val: T) -> bool {
            T::in_range((self.min, self.max), val)
        }

        /// 範囲内の全ての要素を順番に返すイテレータを返します。
        pub fn range(&self) -> impl Iterator<Item = T> {
            T::range((self.min, self.max))
        }
    }

    /// `Ix` トレイトを実装した型をインデックスとして使用できるベクタラッパーです。
    /// 内部的には `Vec` を使用しており、`Ix::to_index` を用いてアクセスを変換します。
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct IxVec<I, T> {
        bounds: Bounds<I>,
        data: Vec<T>,
    }

    impl<I: Ix, T> IxVec<I, T> {
        /// 指定された範囲 `bounds` と初期値 `value` で `IxVec` を作成します。
        pub fn new(bounds: Bounds<I>, value: T) -> Self
        where
            T: Clone,
        {
            let size = bounds.range_size();
            Self {
                bounds,
                data: vec![value; size],
            }
        }

        /// 指定された範囲 `bounds` と各要素を生成する関数 `f` で `IxVec` を作成します。
        pub fn from_fn<F>(bounds: Bounds<I>, f: F) -> Self
        where
            F: FnMut(I) -> T,
        {
            let data = bounds.range().map(f).collect();
            Self { bounds, data }
        }

        /// 既存の `Vec` から `IxVec` を作成します。
        /// `data` の長さは `bounds` の範囲サイズと一致する必要があります。
        pub fn from_vec(bounds: Bounds<I>, data: Vec<T>) -> Self {
            let size = bounds.range_size();
            assert_eq!(
                data.len(),
                size,
                "IxVec::from_vec: data length {} does not match range size {}",
                data.len(),
                size
            );
            Self { bounds, data }
        }

        /// 要素数を返します。
        pub fn len(&self) -> usize {
            self.data.len()
        }

        /// 空であるかを返します。
        pub fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        /// 内部の `Vec` への参照を返します。
        pub fn as_vec(&self) -> &Vec<T> {
            &self.data
        }

        /// 内部の `Vec` を消費して返します。
        pub fn into_vec(self) -> Vec<T> {
            self.data
        }

        /// インデックスの範囲を返します。
        pub fn bounds(&self) -> Bounds<I> {
            self.bounds
        }

        /// 要素へのイテレータを返します。
        pub fn iter(&self) -> std::slice::Iter<'_, T> {
            self.data.iter()
        }

        /// 要素へのミュータブルイテレータを返します。
        pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
            self.data.iter_mut()
        }

        /// インデックスとその要素のペアへのイテレータを返します。
        pub fn iter_with_index(&self) -> impl Iterator<Item = (I, &T)> {
            self.bounds.range().zip(self.data.iter())
        }
    }

    impl<I: Ix, T> Index<I> for IxVec<I, T> {
        type Output = T;
        fn index(&self, index: I) -> &Self::Output {
            let i = self.bounds.to_index(index);
            &self.data[i]
        }
    }

    impl<I: Ix, T> IndexMut<I> for IxVec<I, T> {
        fn index_mut(&mut self, index: I) -> &mut Self::Output {
            let i = self.bounds.to_index(index);
            &mut self.data[i]
        }
    }

    impl<I: Ix, T> Index<&I> for IxVec<I, T> {
        type Output = T;
        fn index(&self, index: &I) -> &Self::Output {
            let i = self.bounds.to_index(*index);
            &self.data[i]
        }
    }

    impl<I: Ix, T> IndexMut<&I> for IxVec<I, T> {
        fn index_mut(&mut self, index: &I) -> &mut Self::Output {
            let i = self.bounds.to_index(*index);
            &mut self.data[i]
        }
    }
}

pub use ix::*;

#[cfg(test)]
mod tests {
    use super::*;

    // --- 1. Basic Types (Primitives) ---

    #[test]
    fn test_ix_usize() {
        let bounds: Bounds<usize> = Bounds::new(2, 5); // 2, 3, 4, 5. size = 4
        assert_eq!(bounds.range_size(), 4);
        assert!(bounds.in_range(2));
        assert!(bounds.in_range(5));
        assert!(!bounds.in_range(1));
        assert!(!bounds.in_range(6));

        assert_eq!(bounds.to_index(2), 0);
        assert_eq!(bounds.to_index(3), 1);
        assert_eq!(bounds.to_index(5), 3);

        let range: Vec<_> = bounds.range().collect();
        assert_eq!(range, vec![2, 3, 4, 5]);

        assert_eq!(bounds.from_index(0), 2);
        assert_eq!(bounds.from_index(3), 5);
    }

    #[test]
    fn test_ix_u64() {
        let bounds: Bounds<u64> = Bounds::new(100u64, 102u64);
        assert_eq!(bounds.range_size(), 3);
        assert_eq!(bounds.to_index(101), 1);
        assert_eq!(bounds.from_index(2), 102);

        // Empty range
        let empty: Bounds<u64> = Bounds::new(100, 99);
        assert_eq!(empty.range_size(), 0);
        assert!(empty.range().next().is_none());
    }

    #[test]
    fn test_ix_i32() {
        let bounds: Bounds<i32> = Bounds::new(-5, 5);
        assert_eq!(bounds.range_size(), 11);
        assert_eq!(bounds.to_index(0), 5);
    }

    #[test]
    fn test_ix_u8() {
        let bounds: Bounds<u8> = Bounds::new(0u8, 255u8);
        assert_eq!(bounds.range_size(), 256);
    }

    #[test]
    fn test_ix_i8() {
        let bounds: Bounds<i8> = Bounds::new(-10i8, 10i8); // size 21
        assert_eq!(bounds.range_size(), 21);
        assert_eq!(bounds.to_index(-10), 0);
        assert_eq!(bounds.to_index(0), 10);
        assert_eq!(bounds.to_index(10), 20);
        assert_eq!(bounds.from_index(20), 10);
    }

    #[test]
    fn test_ix_bool() {
        let bounds: Bounds<bool> = Bounds::new(false, true);
        assert_eq!(bounds.range_size(), 2);
        assert_eq!(bounds.to_index(false), 0);
        assert_eq!(bounds.to_index(true), 1);

        let range: Vec<_> = bounds.range().collect();
        assert_eq!(range, vec![false, true]);

        assert!(!bounds.from_index(0));
        assert!(bounds.from_index(1));

        // case l > r
        let empty: Bounds<bool> = Bounds::new(true, false);
        assert_eq!(empty.range_size(), 0);
    }

    #[test]
    fn test_ix_char() {
        let bounds: Bounds<char> = Bounds::new('a', 'z');
        assert_eq!(bounds.range_size(), 26);
        assert_eq!(bounds.to_index('a'), 0);
        assert_eq!(bounds.to_index('z'), 25);
        assert_eq!(bounds.from_index(1), 'b');

        // Coverage for range() and in_range()
        assert_eq!(bounds.range().count(), 26);
        assert!(bounds.in_range('m'));

        let empty: Bounds<char> = Bounds::new('b', 'a');
        assert_eq!(empty.range_size(), 0);
    }

    #[test]
    fn test_ix_unit() {
        let bounds: Bounds<()> = Bounds::new((), ());
        assert_eq!(bounds.range_size(), 1);
        assert_eq!(bounds.to_index(()), 0);
        assert!(bounds.in_range(()));
        bounds.from_index(0);
        let range: Vec<_> = bounds.range().collect();
        assert_eq!(range, vec![()]);
    }

    // --- 2. Tuple Types ---

    #[test]
    fn test_ix_pair() {
        let bounds: Bounds<(usize, usize)> = Bounds::new((1, 1), (2, 3));
        assert_eq!(bounds.range_size(), 6);

        assert_eq!(bounds.to_index((1, 1)), 0);
        assert_eq!(bounds.to_index((1, 2)), 1);
        assert_eq!(bounds.to_index((1, 3)), 2);
        assert_eq!(bounds.to_index((2, 1)), 3);
        assert_eq!(bounds.to_index((2, 2)), 4);
        assert_eq!(bounds.to_index((2, 3)), 5);

        let range: Vec<_> = bounds.range().collect();
        assert_eq!(range, vec![(1, 1), (1, 2), (1, 3), (2, 1), (2, 2), (2, 3)]);

        assert_eq!(bounds.from_index(0), (1, 1));
        assert_eq!(bounds.from_index(5), (2, 3));
    }

    #[test]
    fn test_ix_triple() {
        let bounds: Bounds<(usize, usize, usize)> = Bounds::new((0, 0, 0), (1, 1, 1));
        assert_eq!(bounds.range_size(), 8);
        assert_eq!(bounds.to_index((1, 0, 1)), 5);

        // Test from_index for various positions
        assert_eq!(bounds.from_index(0), (0, 0, 0));
        assert_eq!(bounds.from_index(5), (1, 0, 1));
        assert_eq!(bounds.from_index(7), (1, 1, 1));

        assert!(bounds.in_range((1, 1, 1)));
        assert!(!bounds.in_range((2, 0, 0)));
        assert_eq!(bounds.range().count(), 8);

        // Empty case
        let empty: Bounds<(usize, usize, usize)> = Bounds::new((0, 0, 1), (0, 0, 0));
        assert_eq!(empty.range_size(), 0);
    }

    #[test]
    fn test_ix_quadruple() {
        let bounds: Bounds<(usize, usize, usize, usize)> = Bounds::new((0, 0, 0, 0), (1, 1, 1, 1));
        assert_eq!(bounds.range_size(), 16);
        assert_eq!(bounds.to_index((1, 0, 1, 0)), 10);

        // Test from_index
        assert_eq!(bounds.from_index(0), (0, 0, 0, 0));
        assert_eq!(bounds.from_index(10), (1, 0, 1, 0));
        assert_eq!(bounds.from_index(15), (1, 1, 1, 1));

        assert!(bounds.in_range((1, 1, 1, 1)));
        assert!(!bounds.in_range((2, 0, 0, 0)));
        assert_eq!(bounds.range().count(), 16);

        // Empty case
        let empty: Bounds<(usize, usize, usize, usize)> = Bounds::new((1, 0, 0, 0), (0, 0, 0, 0));
        assert_eq!(empty.range_size(), 0);
    }

    // --- 3. Macros (Enum) ---

    #[test]
    fn test_ix_enum() {
        #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
        enum Color {
            Red,
            Green,
            Blue,
        }

        impl_ix_for_enum!(Color);

        let bounds: Bounds<Color> = Bounds::new(Color::Red, Color::Blue);
        assert_eq!(bounds.range_size(), 3);
        assert_eq!(bounds.to_index(Color::Red), 0);
        assert_eq!(bounds.to_index(Color::Green), 1);
        assert_eq!(bounds.to_index(Color::Blue), 2);

        let range: Vec<_> = bounds.range().collect();
        assert_eq!(range, vec![Color::Red, Color::Green, Color::Blue]);

        assert_eq!(bounds.from_index(1), Color::Green);
    }

    // --- 4. Structures (IxVec, Bounds) ---

    #[test]
    fn test_ix_vec() {
        let bounds: Bounds<usize> = Bounds::new(2, 5);
        let mut v: IxVec<usize, i32> = IxVec::new(bounds, 0);
        assert_eq!(v.len(), 4);
        assert!(!v.is_empty());
        assert_eq!(v[2], 0);
        assert_eq!(v[5], 0);

        v[2] = 10;
        v[3] = 20;
        assert_eq!(v[2], 10);
        assert_eq!(v[&3], 20); // Test &I access

        v[&3] = 30;
        assert_eq!(v[3], 30);

        // Coverage for as_vec()
        assert_eq!(v.as_vec().len(), 4);

        // Pair Index
        let bounds_pair: Bounds<(usize, usize)> = Bounds::new((1, 1), (2, 2));
        let mut v_pair: IxVec<(usize, usize), usize> = IxVec::from_fn(bounds_pair, |(i, j)| i + j);
        assert_eq!(v_pair[(1, 1)], 2);
        assert_eq!(v_pair[(2, 2)], 4);

        // Iterators
        for val in v_pair.iter_mut() {
            *val += 1;
        }
        assert_eq!(v_pair[(1, 1)], 3);

        let sum: usize = v_pair.iter().sum();
        assert_eq!(sum, (2 + 3 + 3 + 4) + 4); // each element + 1

        let indexed_vals: Vec<_> = v_pair.iter_with_index().collect();
        assert_eq!(indexed_vals[0], ((1, 1), &3));

        // into_vec / as_vec
        let raw_vec: Vec<usize> = v_pair.into_vec();
        assert_eq!(raw_vec.len(), 4);

        let v_from_vec: IxVec<(usize, usize), usize> = IxVec::from_vec(bounds_pair, vec![0, 1, 2, 3]);
        assert_eq!(v_from_vec[(1, 1)], 0);
        assert_eq!(v_from_vec.bounds().min, (1, 1));
    }

    #[test]
    fn test_bounds() {
        let bounds: Bounds<usize> = Bounds::new(2, 5);
        assert_eq!(bounds.range_size(), 4);
        assert_eq!(bounds.to_index(2), 0);
        assert_eq!(bounds.to_index(5), 3);
        assert!(bounds.in_range(3));
        assert!(!bounds.in_range(6));

        let vec: Vec<_> = bounds.range().collect();
        assert_eq!(vec, vec![2, 3, 4, 5]);

        let pair_bounds: Bounds<(usize, usize)> = Bounds::new((1, 1), (2, 2));
        assert_eq!(pair_bounds.range_size(), 4);
        assert_eq!(pair_bounds.to_index((1, 1)), 0);
        assert!(pair_bounds.in_range((2, 2)));
    }

    #[test]
    fn test_from_index() {
        // usize
        let b1: Bounds<usize> = Bounds::new(2, 5);
        assert_eq!(b1.from_index(0), 2);
        assert_eq!(b1.from_index(3), 5);

        // i8
        let b2: Bounds<i8> = Bounds::new(-2, 2);
        assert_eq!(b2.from_index(0), -2);
        assert_eq!(b2.from_index(2), 0);
        assert_eq!(b2.from_index(4), 2);

        // bool
        let b3: Bounds<bool> = Bounds::new(false, true);
        assert!(!b3.from_index(0));
        assert!(b3.from_index(1));

        // char
        let b4: Bounds<char> = Bounds::new('a', 'c');
        assert_eq!(b4.from_index(0), 'a');
        assert_eq!(b4.from_index(2), 'c');

        // Pair
        let b5: Bounds<(usize, usize)> = Bounds::new((1, 1), (2, 2));
        // index 0: (1,1), 1: (1,2), 2: (2,1), 3: (2,2)
        assert_eq!(b5.from_index(0), (1, 1));
        assert_eq!(b5.from_index(1), (1, 2));
        assert_eq!(b5.from_index(2), (2, 1));
        assert_eq!(b5.from_index(3), (2, 2));
    }

    // --- 5. Panic Tests (Error paths) ---

    #[test]
    #[should_panic]
    fn test_ix_out_of_bounds() {
        let bounds: Bounds<usize> = Bounds::new(2, 5);
        bounds.to_index(6);
    }

    #[test]
    #[should_panic]
    fn test_ix_from_index_out_of_range() {
        let bounds: Bounds<usize> = Bounds::new(2, 5);
        bounds.from_index(4);
    }

    #[test]
    #[should_panic]
    fn test_ix_bool_out_of_bounds() {
        let bounds: Bounds<bool> = Bounds::new(false, false);
        bounds.to_index(true);
    }

    #[test]
    #[should_panic]
    fn test_ix_bool_from_index_out_of_range() {
        let bounds: Bounds<bool> = Bounds::new(false, false);
        bounds.from_index(1);
    }

    #[test]
    #[should_panic]
    fn test_ix_char_out_of_bounds() {
        let bounds: Bounds<char> = Bounds::new('a', 'b');
        bounds.to_index('c');
    }

    #[test]
    #[should_panic]
    fn test_ix_char_from_index_out_of_range() {
        let bounds: Bounds<char> = Bounds::new('a', 'b');
        bounds.from_index(2);
    }

    #[test]
    #[should_panic]
    fn test_ix_unit_from_index_out_of_range() {
        let bounds: Bounds<()> = Bounds::new((), ());
        bounds.from_index(1);
    }

    #[test]
    #[should_panic]
    fn test_ix_vec_from_vec_wrong_size() {
        IxVec::from_vec(Bounds::new(0, 2), vec![0, 1]);
    }
}
