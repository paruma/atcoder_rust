use crate::math::algebra::min_max_monoid::min_max_monoid::{BoundedAbove, BoundedBelow};
use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_argminmax_segtree::*;", include = "min_max_monoid")]
pub mod range_argminmax_segtree {
    use super::{BoundedAbove, BoundedBelow};
    use ac_library::{Monoid, Segtree};
    use itertools::Itertools;
    use std::cmp::Ordering;
    use std::convert::Infallible;
    use std::marker::PhantomData;
    use std::ops::RangeBounds;

    #[derive(Clone, Copy, Debug)]
    struct RangeArgminmax<T> {
        min: T,
        max: T,
        argmin_left: Option<usize>,
        argmin_right: Option<usize>,
        argmax_left: Option<usize>,
        argmax_right: Option<usize>,
    }

    impl<T> RangeArgminmax<T>
    where
        T: BoundedAbove + BoundedBelow + Copy + Ord,
    {
        fn from_value(index: usize, value: T) -> Self {
            Self {
                min: value,
                max: value,
                argmin_left: Some(index),
                argmin_right: Some(index),
                argmax_left: Some(index),
                argmax_right: Some(index),
            }
        }

        fn identity() -> Self {
            Self {
                min: T::max_value(),
                max: T::min_value(),
                argmin_left: None,
                argmin_right: None,
                argmax_left: None,
                argmax_right: None,
            }
        }

        fn binary_operation(left: &Self, right: &Self) -> Self {
            let min = std::cmp::min(left.min, right.min);
            let max = std::cmp::max(left.max, right.max);

            let argmin_left = match left.min.cmp(&right.min) {
                Ordering::Less => left.argmin_left,
                Ordering::Equal => left.argmin_left.or(right.argmin_left),
                Ordering::Greater => right.argmin_left,
            };
            let argmin_right = match left.min.cmp(&right.min) {
                Ordering::Less => left.argmin_right,
                Ordering::Equal => right.argmin_right.or(left.argmin_right),
                Ordering::Greater => right.argmin_right,
            };
            let argmax_left = match left.max.cmp(&right.max) {
                Ordering::Less => right.argmax_left,
                Ordering::Equal => left.argmax_left.or(right.argmax_left),
                Ordering::Greater => left.argmax_left,
            };
            let argmax_right = match left.max.cmp(&right.max) {
                Ordering::Less => right.argmax_right,
                Ordering::Equal => right.argmax_right.or(left.argmax_right),
                Ordering::Greater => left.argmax_right,
            };

            Self {
                min,
                max,
                argmin_left,
                argmin_right,
                argmax_left,
                argmax_right,
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct RangeArgminmaxMonoid<T>(Infallible, PhantomData<fn() -> T>);

    impl<T> Monoid for RangeArgminmaxMonoid<T>
    where
        T: BoundedAbove + BoundedBelow + Copy + Ord,
    {
        type S = RangeArgminmax<T>;

        fn identity() -> Self::S {
            RangeArgminmax::identity()
        }

        fn binary_operation(left: &Self::S, right: &Self::S) -> Self::S {
            RangeArgminmax::binary_operation(left, right)
        }
    }

    /// 区間の最小値・最大値と、それぞれの最左・最右添字を管理するセグメント木。
    ///
    /// 値だけを点更新し、添字は構築時の位置に固定する。
    #[derive(Clone)]
    pub struct RangeArgminmaxSegtree<T>
    where
        T: BoundedAbove + BoundedBelow + Copy + Ord,
    {
        segtree: Segtree<RangeArgminmaxMonoid<T>>,
        len: usize,
    }

    impl<T> RangeArgminmaxSegtree<T>
    where
        T: BoundedAbove + BoundedBelow + Copy + Ord,
    {
        /// 配列からセグメント木を構築する。
        pub fn from_slice(xs: &[T]) -> Self {
            let len = xs.len();
            let values = xs
                .iter()
                .copied()
                .enumerate()
                .map(|(index, value)| RangeArgminmax::from_value(index, value))
                .collect_vec();
            Self {
                segtree: Segtree::<RangeArgminmaxMonoid<T>>::from(values),
                len,
            }
        }

        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            self.len
        }

        /// p 番目の値を x に更新する。
        pub fn set(&mut self, p: usize, x: T) {
            self.segtree.set(p, RangeArgminmax::from_value(p, x));
        }

        /// p 番目の値を取得する。
        pub fn get(&self, p: usize) -> T {
            self.segtree.get(p).min
        }

        /// range の最小値を取得する。
        pub fn range_min<R>(&self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).min
        }

        /// range の最大値を取得する。
        pub fn range_max<R>(&self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).max
        }

        /// range で最小値をとる最左の添字を取得する。
        pub fn range_argmin_left<R>(&self, range: R) -> Option<usize>
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).argmin_left
        }

        /// range で最小値をとる最右の添字を取得する。
        pub fn range_argmin_right<R>(&self, range: R) -> Option<usize>
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).argmin_right
        }

        /// range で最大値をとる最左の添字を取得する。
        pub fn range_argmax_left<R>(&self, range: R) -> Option<usize>
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).argmax_left
        }

        /// range で最大値をとる最右の添字を取得する。
        pub fn range_argmax_right<R>(&self, range: R) -> Option<usize>
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).argmax_right
        }

        /// 全要素の最小値を取得する。
        pub fn all_min(&self) -> T {
            self.segtree.all_prod().min
        }

        /// 全要素の最大値を取得する。
        pub fn all_max(&self) -> T {
            self.segtree.all_prod().max
        }

        /// 全要素で最小値をとる最左の添字を取得する。
        pub fn all_argmin_left(&self) -> Option<usize> {
            self.segtree.all_prod().argmin_left
        }

        /// 全要素で最小値をとる最右の添字を取得する。
        pub fn all_argmin_right(&self) -> Option<usize> {
            self.segtree.all_prod().argmin_right
        }

        /// 全要素で最大値をとる最左の添字を取得する。
        pub fn all_argmax_left(&self) -> Option<usize> {
            self.segtree.all_prod().argmax_left
        }

        /// 全要素で最大値をとる最右の添字を取得する。
        pub fn all_argmax_right(&self) -> Option<usize> {
            self.segtree.all_prod().argmax_right
        }

        /// セグメント木上の二分探索。
        ///
        /// [l, r) の最小値 min と最大値 max に対して f(&min, &max) が true となる最大の r を返す。
        ///
        /// # 前提条件
        /// * `l <= n`
        /// * 空区間の `(min, max)` に対して `f` が `true`
        /// * `f` は単調である。
        pub fn max_right<F>(&self, l: usize, f: F) -> usize
        where
            F: Fn(&T, &T) -> bool,
        {
            self.segtree.max_right(l, |value| f(&value.min, &value.max))
        }

        /// セグメント木上の二分探索。
        ///
        /// [l, r) の最小値 min と最大値 max に対して f(&min, &max) が true となる最小の l を返す。
        ///
        /// # 前提条件
        /// * `r <= n`
        /// * 空区間の `(min, max)` に対して `f` が `true`
        /// * `f` は単調である。
        pub fn min_left<F>(&self, r: usize, f: F) -> usize
        where
            F: Fn(&T, &T) -> bool,
        {
            self.segtree.min_left(r, |value| f(&value.min, &value.max))
        }

        /// p 番目の値を `min(current, x)` に更新する。
        pub fn chmin(&mut self, p: usize, x: T) {
            self.set(p, std::cmp::min(self.get(p), x));
        }

        /// p 番目の値を `max(current, x)` に更新する。
        pub fn chmax(&mut self, p: usize, x: T) {
            self.set(p, std::cmp::max(self.get(p), x));
        }

        /// 現在の値を Vec として返す。
        pub fn to_vec(&self) -> Vec<T> {
            (0..self.len).map(|index| self.get(index)).collect_vec()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::range_argminmax_segtree::*;

    #[test]
    fn test_range_argminmax_segtree() {
        let mut seg = RangeArgminmaxSegtree::from_slice(&[3, 1, 4, 1, 5, 9, 2]);

        assert_eq!(seg.len(), 7);
        assert_eq!(seg.range_min(0..7), 1);
        assert_eq!(seg.range_max(0..7), 9);
        assert_eq!(seg.range_argmin_left(0..7), Some(1));
        assert_eq!(seg.range_argmin_right(0..7), Some(3));
        assert_eq!(seg.range_argmax_left(0..7), Some(5));
        assert_eq!(seg.range_argmax_right(0..7), Some(5));
        assert_eq!(seg.range_min(2..5), 1);
        assert_eq!(seg.range_max(2..5), 5);
        assert_eq!(seg.range_argmin_left(2..5), Some(3));
        assert_eq!(seg.range_argmin_right(2..5), Some(3));
        assert_eq!(seg.range_argmax_left(2..5), Some(4));
        assert_eq!(seg.range_argmax_right(2..5), Some(4));

        seg.set(1, 9);
        assert_eq!(seg.get(1), 9);
        assert_eq!(seg.all_min(), 1);
        assert_eq!(seg.all_max(), 9);
        assert_eq!(seg.all_argmin_left(), Some(3));
        assert_eq!(seg.all_argmin_right(), Some(3));
        assert_eq!(seg.all_argmax_left(), Some(1));
        assert_eq!(seg.all_argmax_right(), Some(5));

        seg.chmin(5, 0);
        seg.chmin(0, 5);
        seg.chmax(3, 10);
        seg.chmax(3, 1);
        assert_eq!(seg.to_vec(), vec![3, 9, 4, 10, 5, 0, 2]);
        assert_eq!(seg.all_min(), 0);
        assert_eq!(seg.all_argmin_left(), Some(5));
        assert_eq!(seg.all_argmin_right(), Some(5));
        assert_eq!(seg.all_max(), 10);
        assert_eq!(seg.all_argmax_left(), Some(3));
        assert_eq!(seg.all_argmax_right(), Some(3));
    }

    #[test]
    fn test_empty_range_and_empty_segtree() {
        let seg = RangeArgminmaxSegtree::<i64>::from_slice(&[]);
        assert_eq!(seg.len(), 0);
        assert_eq!(seg.range_min(0..0), i64::MAX);
        assert_eq!(seg.range_max(0..0), i64::MIN);
        assert_eq!(seg.range_argmin_left(0..0), None);
        assert_eq!(seg.range_argmin_right(0..0), None);
        assert_eq!(seg.range_argmax_left(0..0), None);
        assert_eq!(seg.range_argmax_right(0..0), None);
        assert_eq!(seg.all_min(), i64::MAX);
        assert_eq!(seg.all_max(), i64::MIN);
        assert_eq!(seg.all_argmin_left(), None);
        assert_eq!(seg.all_argmin_right(), None);
        assert_eq!(seg.all_argmax_left(), None);
        assert_eq!(seg.all_argmax_right(), None);
        assert_eq!(seg.to_vec(), Vec::<i64>::new());
    }

    #[test]
    fn test_max_right_and_min_left() {
        let seg = RangeArgminmaxSegtree::from_slice(&[2, 4, 6, 8, 10]);
        assert_eq!(seg.max_right(0, |min, max| *min >= 2 && *max <= 7), 3);
        assert_eq!(seg.min_left(5, |min, max| *min >= 7 && *max <= 10), 3);
    }

    #[ignore]
    #[test]
    fn test_random() {
        use rand::{Rng, SeedableRng};

        let mut rng = rand::rngs::SmallRng::seed_from_u64(42);
        for _ in 0..100 {
            let len = rng.random_range(0..=30);
            let mut values: Vec<i64> = (0..len).map(|_| rng.random_range(-3..=3)).collect();
            let mut seg = RangeArgminmaxSegtree::from_slice(&values);

            for _ in 0..100 {
                match rng.random_range(0..3) {
                    0 => {
                        if len > 0 {
                            let index = rng.random_range(0..len);
                            let value = rng.random_range(-3..=3);
                            values[index] = value;
                            seg.set(index, value);
                        }
                    }
                    1 => {
                        if len > 0 {
                            let index = rng.random_range(0..len);
                            let value = rng.random_range(-3..=3);
                            values[index] = values[index].min(value);
                            seg.chmin(index, value);
                        }
                    }
                    2 => {
                        if len > 0 {
                            let index = rng.random_range(0..len);
                            let value = rng.random_range(-3..=3);
                            values[index] = values[index].max(value);
                            seg.chmax(index, value);
                        }
                    }
                    _ => unreachable!(),
                }

                let left = rng.random_range(0..=len);
                let right = rng.random_range(left..=len);
                let range = &values[left..right];
                let min = range.iter().min().copied().unwrap_or(i64::MAX);
                let max = range.iter().max().copied().unwrap_or(i64::MIN);
                let argmin_left = range
                    .iter()
                    .position(|&value| value == min)
                    .map(|i| left + i);
                let argmin_right = range
                    .iter()
                    .rposition(|&value| value == min)
                    .map(|i| left + i);
                let argmax_left = range
                    .iter()
                    .position(|&value| value == max)
                    .map(|i| left + i);
                let argmax_right = range
                    .iter()
                    .rposition(|&value| value == max)
                    .map(|i| left + i);

                assert_eq!(seg.range_min(left..right), min);
                assert_eq!(seg.range_max(left..right), max);
                assert_eq!(seg.range_argmin_left(left..right), argmin_left);
                assert_eq!(seg.range_argmin_right(left..right), argmin_right);
                assert_eq!(seg.range_argmax_left(left..right), argmax_left);
                assert_eq!(seg.range_argmax_right(left..right), argmax_right);
            }
        }
    }
}
