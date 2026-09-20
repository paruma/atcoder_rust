use crate::math::algebra::min_max_monoid::min_max_monoid::{BoundedAbove, BoundedBelow};
use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(
    prefix = "use range_add_range_argminmax::*;",
    include = "min_max_monoid"
)]
pub mod range_add_range_argminmax {
    use super::{BoundedAbove, BoundedBelow};
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use std::cmp::Ordering;
    use std::convert::Infallible;
    use std::iter::Sum;
    use std::marker::PhantomData;
    use std::ops::{Add, RangeBounds};

    fn zero<T: Sum>() -> T {
        std::iter::empty::<T>().sum()
    }

    #[derive(Clone, Copy, Debug)]
    struct RangeArgminmax<T> {
        min: T,
        max: T,
        argmin_left: usize,
        argmin_right: usize,
        argmax_left: usize,
        argmax_right: usize,
    }

    impl<T> RangeArgminmax<T>
    where
        T: BoundedAbove + BoundedBelow + Copy + Ord,
    {
        fn from_value(index: usize, value: T) -> Self {
            Self {
                min: value,
                max: value,
                argmin_left: index,
                argmin_right: index,
                argmax_left: index,
                argmax_right: index,
            }
        }

        fn identity() -> Self {
            Self {
                min: T::max_value(),
                max: T::min_value(),
                argmin_left: usize::MAX,
                argmin_right: usize::MIN,
                argmax_left: usize::MAX,
                argmax_right: usize::MIN,
            }
        }

        fn binary_operation(left: &Self, right: &Self) -> Self {
            let min = left.min.min(right.min);
            let max = left.max.max(right.max);
            let argmin_left = match left.min.cmp(&right.min) {
                Ordering::Less => left.argmin_left,
                Ordering::Equal => left.argmin_left.min(right.argmin_left),
                Ordering::Greater => right.argmin_left,
            };
            let argmin_right = match left.min.cmp(&right.min) {
                Ordering::Less => left.argmin_right,
                Ordering::Equal => left.argmin_right.max(right.argmin_right),
                Ordering::Greater => right.argmin_right,
            };
            let argmax_left = match left.max.cmp(&right.max) {
                Ordering::Less => right.argmax_left,
                Ordering::Equal => left.argmax_left.min(right.argmax_left),
                Ordering::Greater => left.argmax_left,
            };
            let argmax_right = match left.max.cmp(&right.max) {
                Ordering::Less => right.argmax_right,
                Ordering::Equal => left.argmax_right.max(right.argmax_right),
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
    struct AddAction<T> {
        add_value: T,
    }

    impl<T: Copy> AddAction<T> {
        fn new(add_value: T) -> Self {
            Self { add_value }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct RangeAddRangeArgminmax<T>(Infallible, PhantomData<fn() -> T>);

    impl<T> MapMonoid for RangeAddRangeArgminmax<T>
    where
        T: BoundedAbove + BoundedBelow + Copy + Ord + Add<Output = T> + Sum,
    {
        type M = RangeArgminmaxMonoid<T>;
        type F = AddAction<T>;

        fn identity_map() -> Self::F {
            AddAction::new(zero())
        }

        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            AddAction::new(g.add_value + f.add_value)
        }

        fn mapping(f: &Self::F, value: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
            if value.argmin_left == usize::MAX {
                *value
            } else {
                RangeArgminmax {
                    min: value.min + f.add_value,
                    max: value.max + f.add_value,
                    ..*value
                }
            }
        }
    }

    /// 区間加算と、区間の最小値・最大値およびそれぞれの最左・最右添字を管理する遅延セグメント木。
    #[derive(Clone)]
    pub struct RangeAddRangeArgminmaxSegtree<T>
    where
        T: BoundedAbove + BoundedBelow + Copy + Ord + Add<Output = T> + Sum,
    {
        segtree: LazySegtree<RangeAddRangeArgminmax<T>>,
        len: usize,
    }

    impl<T> RangeAddRangeArgminmaxSegtree<T>
    where
        T: BoundedAbove + BoundedBelow + Copy + Ord + Add<Output = T> + Sum,
    {
        /// 配列から遅延セグメント木を構築する。
        pub fn from_slice(xs: &[T]) -> Self {
            let len = xs.len();
            let values = xs
                .iter()
                .copied()
                .enumerate()
                .map(|(index, value)| RangeArgminmax::from_value(index, value))
                .collect_vec();
            Self {
                segtree: LazySegtree::from(values),
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
        pub fn get(&mut self, p: usize) -> T {
            self.segtree.get(p).min
        }

        /// range の最小値を取得する。
        pub fn range_min<R: RangeBounds<usize>>(&mut self, range: R) -> T {
            self.segtree.prod(range).min
        }

        /// range の最大値を取得する。
        pub fn range_max<R: RangeBounds<usize>>(&mut self, range: R) -> T {
            self.segtree.prod(range).max
        }

        /// range で最小値をとる最左の添字を取得する。
        ///
        /// 空区間では `usize::MAX` を返す。
        pub fn range_argmin_left<R: RangeBounds<usize>>(&mut self, range: R) -> usize {
            self.segtree.prod(range).argmin_left
        }

        /// range で最小値をとる最右の添字を取得する。
        ///
        /// 空区間では `usize::MIN` を返す。
        pub fn range_argmin_right<R: RangeBounds<usize>>(&mut self, range: R) -> usize {
            self.segtree.prod(range).argmin_right
        }

        /// range で最大値をとる最左の添字を取得する。
        ///
        /// 空区間では `usize::MAX` を返す。
        pub fn range_argmax_left<R: RangeBounds<usize>>(&mut self, range: R) -> usize {
            self.segtree.prod(range).argmax_left
        }

        /// range で最大値をとる最右の添字を取得する。
        ///
        /// 空区間では `usize::MIN` を返す。
        pub fn range_argmax_right<R: RangeBounds<usize>>(&mut self, range: R) -> usize {
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
        ///
        /// 空のセグメント木では `usize::MAX` を返す。
        pub fn all_argmin_left(&self) -> usize {
            self.segtree.all_prod().argmin_left
        }

        /// 全要素で最小値をとる最右の添字を取得する。
        ///
        /// 空のセグメント木では `usize::MIN` を返す。
        pub fn all_argmin_right(&self) -> usize {
            self.segtree.all_prod().argmin_right
        }

        /// 全要素で最大値をとる最左の添字を取得する。
        ///
        /// 空のセグメント木では `usize::MAX` を返す。
        pub fn all_argmax_left(&self) -> usize {
            self.segtree.all_prod().argmax_left
        }

        /// 全要素で最大値をとる最右の添字を取得する。
        ///
        /// 空のセグメント木では `usize::MIN` を返す。
        pub fn all_argmax_right(&self) -> usize {
            self.segtree.all_prod().argmax_right
        }

        /// range の各要素に x を加算する。
        pub fn range_add<R: RangeBounds<usize>>(&mut self, range: R, x: T) {
            self.segtree.apply_range(range, AddAction::new(x));
        }

        /// セグメント木上の二分探索。
        ///
        /// [l, r) の最小値 min と最大値 max に対して f(&min, &max) が true となる最大の r を返す。
        ///
        /// # 前提条件
        /// * `l <= n`
        /// * 空区間の `(min, max)` に対して `f` が `true`
        /// * `f` は単調である。
        pub fn max_right<F>(&mut self, l: usize, f: F) -> usize
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
        pub fn min_left<F>(&mut self, r: usize, f: F) -> usize
        where
            F: Fn(&T, &T) -> bool,
        {
            self.segtree.min_left(r, |value| f(&value.min, &value.max))
        }

        /// p 番目の値を `min(current, x)` に更新する。
        pub fn chmin(&mut self, p: usize, x: T) {
            let current = self.get(p);
            self.set(p, current.min(x));
        }

        /// p 番目の値を `max(current, x)` に更新する。
        pub fn chmax(&mut self, p: usize, x: T) {
            let current = self.get(p);
            self.set(p, current.max(x));
        }

        /// 現在の値を Vec として返す。
        pub fn to_vec(&mut self) -> Vec<T> {
            (0..self.len).map(|index| self.get(index)).collect_vec()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::range_add_range_argminmax::*;

    fn assert_matches(
        seg: &mut RangeAddRangeArgminmaxSegtree<i64>,
        xs: &[i64],
        left: usize,
        right: usize,
    ) {
        let range = &xs[left..right];
        let min = range.iter().copied().min().unwrap_or(i64::MAX);
        let max = range.iter().copied().max().unwrap_or(i64::MIN);
        assert_eq!(seg.range_min(left..right), min);
        assert_eq!(seg.range_max(left..right), max);
        assert_eq!(
            seg.range_argmin_left(left..right),
            range
                .iter()
                .position(|&x| x == min)
                .map(|i| left + i)
                .unwrap_or(usize::MAX)
        );
        assert_eq!(
            seg.range_argmin_right(left..right),
            range
                .iter()
                .rposition(|&x| x == min)
                .map(|i| left + i)
                .unwrap_or(usize::MIN)
        );
        assert_eq!(
            seg.range_argmax_left(left..right),
            range
                .iter()
                .position(|&x| x == max)
                .map(|i| left + i)
                .unwrap_or(usize::MAX)
        );
        assert_eq!(
            seg.range_argmax_right(left..right),
            range
                .iter()
                .rposition(|&x| x == max)
                .map(|i| left + i)
                .unwrap_or(usize::MIN)
        );
    }

    #[test]
    fn test_range_add_range_argminmax_segtree() {
        let mut seg = RangeAddRangeArgminmaxSegtree::from_slice(&[3, 1, 4, 1, 5, 9, 2]);
        assert_eq!(seg.len(), 7);
        assert_matches(&mut seg, &[3, 1, 4, 1, 5, 9, 2], 0, 7);
        seg.range_add(1..5, 10);
        assert_eq!(seg.to_vec(), vec![3, 11, 14, 11, 15, 9, 2]);
        assert_matches(&mut seg, &[3, 11, 14, 11, 15, 9, 2], 0, 7);
        assert_matches(&mut seg, &[3, 11, 14, 11, 15, 9, 2], 1, 5);
        seg.set(5, 11);
        assert_eq!(seg.all_min(), 2);
        assert_eq!(seg.all_max(), 15);
        assert_eq!(seg.all_argmin_left(), 6);
        assert_eq!(seg.all_argmin_right(), 6);
        assert_eq!(seg.all_argmax_left(), 4);
        assert_eq!(seg.all_argmax_right(), 4);
    }

    #[test]
    fn test_empty_range_and_boundary_values() {
        let mut empty = RangeAddRangeArgminmaxSegtree::<i64>::from_slice(&[]);
        assert_matches(&mut empty, &[], 0, 0);
        empty.range_add(0..0, 100);
        assert_eq!(empty.to_vec(), Vec::<i64>::new());

        let mut seg = RangeAddRangeArgminmaxSegtree::from_slice(&[i64::MIN, i64::MAX]);
        assert_eq!(seg.range_argmin_left(0..1), 0);
        assert_eq!(seg.range_argmax_left(1..2), 1);
    }

    #[test]
    fn test_max_right_and_min_left() {
        let mut seg = RangeAddRangeArgminmaxSegtree::from_slice(&[2, 4, 6, 8, 10]);
        assert_eq!(seg.max_right(0, |min, max| *min >= 2 && *max <= 7), 3);
        assert_eq!(seg.min_left(5, |min, max| *min >= 7 && *max <= 10), 3);
    }

    #[test]
    fn test_point_chmin_and_chmax() {
        let mut seg = RangeAddRangeArgminmaxSegtree::from_slice(&[3, 1, 4]);
        seg.chmin(2, 0);
        seg.chmax(1, 10);
        assert_eq!(seg.to_vec(), vec![3, 10, 0]);
        assert_eq!(seg.all_argmin_left(), 2);
        assert_eq!(seg.all_argmax_left(), 1);
    }

    #[ignore]
    #[test]
    fn test_random() {
        use rand::{Rng, SeedableRng};
        let mut rng = rand::rngs::SmallRng::seed_from_u64(42);
        for _ in 0..100 {
            let len = rng.random_range(0..=30);
            let mut xs: Vec<i64> = (0..len).map(|_| rng.random_range(-20..=20)).collect();
            let mut seg = RangeAddRangeArgminmaxSegtree::from_slice(&xs);
            for _ in 0..100 {
                let left = rng.random_range(0..=len);
                let right = rng.random_range(left..=len);
                match rng.random_range(0..3) {
                    0 if len > 0 => {
                        let p = rng.random_range(0..len);
                        let x = rng.random_range(-20..=20);
                        xs[p] = x;
                        seg.set(p, x);
                    }
                    1 => {
                        let x = rng.random_range(-20..=20);
                        for value in &mut xs[left..right] {
                            *value += x;
                        }
                        seg.range_add(left..right, x);
                    }
                    2 if len > 0 => {
                        let p = rng.random_range(0..len);
                        let x = rng.random_range(-20..=20);
                        xs[p] = xs[p].min(x);
                        seg.chmin(p, x);
                    }
                    _ => {}
                }
                assert_matches(&mut seg, &xs, left, right);
            }
            assert_eq!(seg.to_vec(), xs);
        }
    }
}
