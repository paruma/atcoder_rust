// #[fastout]
fn main() {
    input! {
        n: usize,
        abs: [(usize, Usize1); n],
        m: usize,
        ss: [Chars; m],
    }
    // exists[len][place][ch] = any[ s in ss | s.len() == len, s[place] = ch ]
    let lc = Bounds::new('a', 'z');
    let mut exists = vec![vec![IxVec::new(lc, false); 10]; 11];
    for s in &ss {
        for (i, ch) in s.iter().copied().enumerate() {
            exists[s.len()][i][ch] = true;
        }
    }

    let ans = ss
        .iter()
        .map(|s| {
            if s.len() != n {
                return false;
            }

            s.iter().copied().enumerate().all(|(i, ch)| {
                let (a, b) = abs[i];
                exists[a][b][ch]
            })
        })
        .collect_vec();

    for p in ans {
        print_yesno(p);
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
use ix::*;
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
    macro_rules ! impl_ix_for_integer {($ ($ t : ty ) ,* ) => {$ (impl Ix for $ t {fn range ((l , r ) : (Self , Self ) ) -> impl Iterator < Item = Self > {l ..= r } fn range_size ((l , r ) : (Self , Self ) ) -> usize {if l > r {0 } else {(l . abs_diff (r ) as usize ) + 1 } } fn to_index ((l , r ) : (Self , Self ) , i : Self ) -> usize {if ! Self :: in_range ((l , r ) , i ) {panic ! ("index out of bounds: {:?} is not in {:?}" , i , (l , r ) ) ; } (l . abs_diff (i ) as usize ) } fn from_index ((l , r ) : (Self , Self ) , index : usize ) -> Self {if index >= Self :: range_size ((l , r ) ) {panic ! ("index out of range: {} for bounds {:?}" , index , (l , r ) ) ; } l + index as Self } fn in_range ((l , r ) : (Self , Self ) , i : Self ) -> bool {l <= i && i <= r } } ) * } ; }
    impl_ix_for_integer!(
        u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
    );
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
    impl<A: Ix, B: Ix, C: Ix> Ix for (A, B, C) {
        fn range(((l1, l2, l3), (u1, u2, u3)): (Self, Self)) -> impl Iterator<Item = Self> {
            A::range((l1, u1)).flat_map(move |i1| {
                B::range((l2, u2))
                    .flat_map(move |i2| C::range((l3, u3)).map(move |i3| (i1, i2, i3)))
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
        fn to_index(
            ((l1, l2, l3, l4), (u1, u2, u3, u4)): (Self, Self),
            (i1, i2, i3, i4): Self,
        ) -> usize {
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
        /// 指定されたインデックスの要素への参照を返します。
        /// インデックスが範囲外の場合は `None` を返します。
        pub fn get(&self, index: I) -> Option<&T> {
            if self.bounds.in_range(index) {
                let i = self.bounds.to_index(index);
                Some(&self.data[i])
            } else {
                None
            }
        }
        /// 指定されたインデックスの要素へのミュータブル参照を返します。
        /// インデックスが範囲外の場合は `None` を返します。
        pub fn get_mut(&mut self, index: I) -> Option<&mut T> {
            if self.bounds.in_range(index) {
                let i = self.bounds.to_index(index);
                Some(&mut self.data[i])
            } else {
                None
            }
        }
        /// 指定されたインデックスが有効な範囲内にあるかを返します。
        pub fn contains_index(&self, index: I) -> bool {
            self.bounds.in_range(index)
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
