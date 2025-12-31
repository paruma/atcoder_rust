use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use offset_vec::*;")]
pub mod offset_vec {
    use std::ops::{
        Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
    };

    /// [begin, end) の範囲の添字を許容する Vec。
    /// 負の添字も使用可能。
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct OffsetVec<T> {
        data: Vec<T>,
        begin: i64,
        end: i64,
    }

    impl<T> OffsetVec<T> {
        /// 指定された範囲 [begin, end) で初期化する。
        pub fn new(begin: i64, end: i64, default: T) -> Self
        where
            T: Clone,
        {
            assert!(begin <= end);
            let size = (end - begin) as usize;
            Self {
                data: vec![default; size],
                begin,
                end,
            }
        }

        /// 指定された開始位置とデータで初期化する。
        pub fn from_vec(begin: i64, data: Vec<T>) -> Self {
            let end = begin + data.len() as i64;
            Self { data, begin, end }
        }

        pub fn begin(&self) -> i64 {
            self.begin
        }

        pub fn end(&self) -> i64 {
            self.end
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }

        pub fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        pub fn iter(&self) -> std::slice::Iter<'_, T> {
            self.data.iter()
        }

        pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
            self.data.iter_mut()
        }

        pub fn as_slice(&self) -> &[T] {
            &self.data
        }

        pub fn as_mut_slice(&mut self) -> &mut [T] {
            &mut self.data
        }

        pub fn iter_with_index(&self) -> impl Iterator<Item = (i64, &T)> {
            let begin = self.begin;
            self.data
                .iter()
                .enumerate()
                .map(move |(i, v)| (begin + i as i64, v))
        }

        pub fn is_within(&self, index: i64) -> bool {
            self.begin <= index && index < self.end
        }

        fn check_range(&self, index: i64) {
            assert!(
                self.is_within(index),
                "the range is [{}, {}) but the index is {}",
                self.begin,
                self.end,
                index
            );
        }

        fn to_raw_index(&self, index: i64) -> usize {
            (index - self.begin) as usize
        }
    }

    impl<T> Index<i64> for OffsetVec<T> {
        type Output = T;
        fn index(&self, index: i64) -> &Self::Output {
            self.check_range(index);
            &self.data[self.to_raw_index(index)]
        }
    }

    impl<T> IndexMut<i64> for OffsetVec<T> {
        fn index_mut(&mut self, index: i64) -> &mut Self::Output {
            self.check_range(index);
            let raw_index = self.to_raw_index(index);
            &mut self.data[raw_index]
        }
    }

    // Slice indexing for i64
    impl<T> Index<Range<i64>> for OffsetVec<T> {
        type Output = [T];
        fn index(&self, range: Range<i64>) -> &Self::Output {
            &self.data[self.to_raw_index(range.start)..self.to_raw_index(range.end)]
        }
    }
    impl<T> IndexMut<Range<i64>> for OffsetVec<T> {
        fn index_mut(&mut self, range: Range<i64>) -> &mut Self::Output {
            let s = self.to_raw_index(range.start);
            let e = self.to_raw_index(range.end);
            &mut self.data[s..e]
        }
    }
    impl<T> Index<RangeInclusive<i64>> for OffsetVec<T> {
        type Output = [T];
        fn index(&self, range: RangeInclusive<i64>) -> &Self::Output {
            &self.data[self.to_raw_index(*range.start())..=self.to_raw_index(*range.end())]
        }
    }
    impl<T> IndexMut<RangeInclusive<i64>> for OffsetVec<T> {
        fn index_mut(&mut self, range: RangeInclusive<i64>) -> &mut Self::Output {
            let s = self.to_raw_index(*range.start());
            let e = self.to_raw_index(*range.end());
            &mut self.data[s..=e]
        }
    }
    impl<T> Index<RangeFrom<i64>> for OffsetVec<T> {
        type Output = [T];
        fn index(&self, range: RangeFrom<i64>) -> &Self::Output {
            &self.data[self.to_raw_index(range.start)..]
        }
    }
    impl<T> IndexMut<RangeFrom<i64>> for OffsetVec<T> {
        fn index_mut(&mut self, range: RangeFrom<i64>) -> &mut Self::Output {
            let s = self.to_raw_index(range.start);
            &mut self.data[s..]
        }
    }
    impl<T> Index<RangeTo<i64>> for OffsetVec<T> {
        type Output = [T];
        fn index(&self, range: RangeTo<i64>) -> &Self::Output {
            &self.data[..self.to_raw_index(range.end)]
        }
    }
    impl<T> IndexMut<RangeTo<i64>> for OffsetVec<T> {
        fn index_mut(&mut self, range: RangeTo<i64>) -> &mut Self::Output {
            let e = self.to_raw_index(range.end);
            &mut self.data[..e]
        }
    }
    impl<T> Index<RangeToInclusive<i64>> for OffsetVec<T> {
        type Output = [T];
        fn index(&self, range: RangeToInclusive<i64>) -> &Self::Output {
            &self.data[..=self.to_raw_index(range.end)]
        }
    }
    impl<T> IndexMut<RangeToInclusive<i64>> for OffsetVec<T> {
        fn index_mut(&mut self, range: RangeToInclusive<i64>) -> &mut Self::Output {
            let e = self.to_raw_index(range.end);
            &mut self.data[..=e]
        }
    }

    impl<T> Index<RangeFull> for OffsetVec<T> {
        type Output = [T];
        fn index(&self, _: RangeFull) -> &Self::Output {
            &self.data[..]
        }
    }
    impl<T> IndexMut<RangeFull> for OffsetVec<T> {
        fn index_mut(&mut self, _: RangeFull) -> &mut Self::Output {
            &mut self.data[..]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::offset_vec::*;

    #[test]
    fn test_offset_vec_basic() {
        let mut v = OffsetVec::new(-5, 5, 0);
        assert_eq!(v.begin(), -5);
        assert_eq!(v.end(), 5);
        assert_eq!(v.len(), 10);
        v[-5] = 10;
        v[0] = 20;
        v[4] = 30;
        assert_eq!(v[-5], 10);
        assert_eq!(v[0], 20);
        assert_eq!(v[4], 30);
    }

    #[test]
    #[should_panic]
    fn test_offset_vec_out_of_bounds_lower() {
        let v = OffsetVec::new(-5, 5, 0);
        let _ = v[-6];
    }

    #[test]
    #[should_panic]
    fn test_offset_vec_out_of_bounds_upper() {
        let v = OffsetVec::new(-5, 5, 0);
        let _ = v[5];
    }

    #[test]
    fn test_from_vec() {
        let v = OffsetVec::from_vec(10, vec![1, 2, 3]);
        assert_eq!(v.begin(), 10);
        assert_eq!(v.end(), 13);
        assert_eq!(v[10], 1);
        assert_eq!(v[11], 2);
        assert_eq!(v[12], 3);
    }

    #[test]
    fn test_iter_with_index() {
        let v = OffsetVec::from_vec(-2, vec![10, 20, 30]);
        let items: Vec<_> = v.iter_with_index().collect();
        assert_eq!(items, vec![(-2, &10), (-1, &20), (0, &30)]);
    }

    #[test]
    fn test_is_empty() {
        let v = OffsetVec::new(0, 3, 1);
        assert!(!v.is_empty());
        let empty: OffsetVec<i32> = OffsetVec::new(0, 0, 0);
        assert!(empty.is_empty());
    }

    #[test]
    fn test_is_within() {
        let v = OffsetVec::new(-5, 5, 0);
        assert!(v.is_within(-5));
        assert!(v.is_within(0));
        assert!(v.is_within(4));
        assert!(!v.is_within(-6));
        assert!(!v.is_within(5));
    }

    #[test]
    fn test_iter() {
        let mut v = OffsetVec::new(0, 3, 1);
        for x in v.iter() {
            assert_eq!(*x, 1);
        }
        for x in v.iter_mut() {
            *x += 1;
        }
        for x in v.iter() {
            assert_eq!(*x, 2);
        }
    }

    #[test]
    fn test_as_slice() {
        let mut v = OffsetVec::from_vec(-2, vec![30, 10, 20]);
        assert_eq!(v.as_slice(), &[30, 10, 20]);
        v.as_mut_slice().sort();
        assert_eq!(v.as_slice(), &[10, 20, 30]);
        assert_eq!(v[-2], 10);
    }

    #[test]
    fn test_slice() {
        let v = OffsetVec::from_vec(-2, vec![10, 20, 30, 40, 50]);
        assert_eq!(&v[-1..2], &[20, 30, 40]);
        assert_eq!(&v[-1..=1], &[20, 30, 40]);
        assert_eq!(&v[0..=1], &[30, 40]);
        assert_eq!(&v[..0], &[10, 20]);
        assert_eq!(&v[..=0], &[10, 20, 30]);
        assert_eq!(&v[1..], &[40, 50]);
        assert_eq!(&v[..], &[10, 20, 30, 40, 50]);
        assert_eq!(&v[..=2], &[10, 20, 30, 40, 50]);
    }

    #[test]
    fn test_slice_mut() {
        let mut v = OffsetVec::from_vec(-2, vec![10, 20, 30, 40, 50]);
        v[-1..2].fill(0);
        assert_eq!(&v[..], &[10, 0, 0, 0, 50]);
        v[-2..=-1].fill(1);
        assert_eq!(&v[..], &[1, 1, 0, 0, 50]);
        v[..1].fill(2);
        assert_eq!(&v[..], &[2, 2, 2, 0, 50]);
        v[..=0].fill(3);
        assert_eq!(&v[..], &[3, 3, 3, 0, 50]);
        v[2..].fill(4);
        assert_eq!(&v[..], &[3, 3, 3, 0, 4]);
        v[..].fill(9);
        assert_eq!(&v[..], &[9, 9, 9, 9, 9]);
    }
}
