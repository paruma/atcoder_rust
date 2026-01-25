use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_chmin_chmax_affine_range_min_max::*;")]
pub mod range_chmin_chmax_affine_range_min_max {
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::marker::PhantomData;
    use std::ops::{Add, Mul, RangeBounds};

    pub trait Bounded {
        fn min_value() -> Self;
        fn max_value() -> Self;
    }

    impl Bounded for i64 {
        fn min_value() -> Self {
            i64::MIN
        }
        fn max_value() -> Self {
            i64::MAX
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeMinMax<T> {
        pub min: T,
        pub max: T,
    }

    impl<T> RangeMinMax<T> {
        pub fn new(min: T, max: T) -> Self {
            Self { min, max }
        }

        pub fn unit(x: T) -> Self
        where
            T: Copy,
        {
            Self { min: x, max: x }
        }
    }

    // 範囲最小値/最大値クエリのモノイド
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeMinMaxMonoid<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for RangeMinMaxMonoid<T>
    where
        T: Copy + Ord + Bounded,
    {
        type S = RangeMinMax<T>;
        fn identity() -> Self::S {
            RangeMinMax {
                min: T::max_value(),
                max: T::min_value(),
            }
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            RangeMinMax {
                min: a.min.min(b.min),
                max: a.max.max(b.max),
            }
        }
    }

    /// ChminChmaxAffineAction は、x に対して (mul * x + add).min(chmin_value).max(chmax_value) を計算する関数を表す構造体です。
    /// これは、範囲に対する chmin (最小値更新)、chmax (最大値更新)、加算、アフィン変換を一度に適用するためのアクションを定義します。
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ChminChmaxAffineAction<T> {
        chmin_val: T,
        chmax_val: T,
        mul_val: T,
        add_val: T,
    }

    impl<T> ChminChmaxAffineAction<T>
    where
        T: Copy + Ord + Bounded + From<i64> + Add<Output = T> + Mul<Output = T>,
    {
        pub fn new_chmin(val: T) -> Self {
            Self {
                chmin_val: val,
                chmax_val: T::min_value(),
                mul_val: T::from(1),
                add_val: T::from(0),
            }
        }

        pub fn new_chmax(val: T) -> Self {
            Self {
                chmin_val: T::max_value(),
                chmax_val: val,
                mul_val: T::from(1),
                add_val: T::from(0),
            }
        }

        pub fn new_add(val: T) -> Self {
            Self {
                chmin_val: T::max_value(),
                chmax_val: T::min_value(),
                mul_val: T::from(1),
                add_val: val,
            }
        }

        pub fn new_update(val: T) -> Self {
            Self {
                chmin_val: T::max_value(),
                chmax_val: T::min_value(),
                mul_val: T::from(0),
                add_val: val,
            }
        }

        pub fn new_affine(mul: T, add: T) -> Self {
            Self {
                chmin_val: T::max_value(),
                chmax_val: T::min_value(),
                mul_val: mul,
                add_val: add,
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeChminChmaxAffineRangeMinMax<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for RangeChminChmaxAffineRangeMinMax<T>
    where
        T: Copy + Ord + From<i64> + Bounded + Add<Output = T> + Mul<Output = T> + std::fmt::Debug,
    {
        type M = RangeMinMaxMonoid<T>;
        type F = ChminChmaxAffineAction<T>;

        fn identity_map() -> Self::F {
            ChminChmaxAffineAction {
                chmin_val: T::max_value(),
                chmax_val: T::min_value(),
                mul_val: T::from(1),
                add_val: T::from(0),
            }
        }

        // f: 新しいアクション, g: 古いアクション
        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            let new_mul = g.mul_val * f.mul_val;
            let new_add = g.add_val * f.mul_val + f.add_val;

            // 合成された制約を初期値で初期化
            let mut composed_chmin_val = T::max_value();
            let mut composed_chmax_val = T::min_value();

            // gのchmin_valを処理
            if g.chmin_val != T::max_value() {
                let transformed_g_chmin = g.chmin_val * f.mul_val + f.add_val;
                if f.mul_val >= T::from(0) {
                    composed_chmin_val = composed_chmin_val.min(transformed_g_chmin);
                } else {
                    // f.mul_val < 0
                    composed_chmax_val = composed_chmax_val.max(transformed_g_chmin);
                }
            }

            // gのchmax_valを処理
            if g.chmax_val != T::min_value() {
                let transformed_g_chmax = g.chmax_val * f.mul_val + f.add_val;
                if f.mul_val >= T::from(0) {
                    composed_chmax_val = composed_chmax_val.max(transformed_g_chmax);
                } else {
                    // f.mul_val < 0
                    composed_chmin_val = composed_chmin_val.min(transformed_g_chmax);
                }
            }

            // fの制約を最も外側のクランプとして適用
            composed_chmin_val = composed_chmin_val.clamp(f.chmax_val, f.chmin_val);
            composed_chmax_val = composed_chmax_val.clamp(f.chmax_val, f.chmin_val);

            ChminChmaxAffineAction {
                chmin_val: composed_chmin_val,
                chmax_val: composed_chmax_val,
                mul_val: new_mul,
                add_val: new_add,
            }
        }

        fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
            let RangeMinMax { min, max } = *x;
            if min > max {
                return *x;
            }

            let (mut transformed_min, mut transformed_max) = if f.mul_val >= T::from(0) {
                (min * f.mul_val + f.add_val, max * f.mul_val + f.add_val)
            } else {
                (max * f.mul_val + f.add_val, min * f.mul_val + f.add_val)
            };

            // fのchmin/chmax制約を適用
            if f.chmin_val != T::max_value() {
                transformed_min = transformed_min.min(f.chmin_val);
                transformed_max = transformed_max.min(f.chmin_val);
            }
            if f.chmax_val != T::min_value() {
                transformed_min = transformed_min.max(f.chmax_val);
                transformed_max = transformed_max.max(f.chmax_val);
            }

            RangeMinMax {
                min: transformed_min,
                max: transformed_max,
            }
        }
    }

    #[derive(Clone)]
    pub struct RangeChminChmaxAffineRangeMinMaxSegtree<T>
    where
        T: Copy + Ord + From<i64> + Bounded + Add<Output = T> + Mul<Output = T> + std::fmt::Debug,
    {
        segtree: LazySegtree<RangeChminChmaxAffineRangeMinMax<T>>,
        len: usize,
    }

    impl<T> RangeChminChmaxAffineRangeMinMaxSegtree<T>
    where
        T: Copy + Ord + From<i64> + Bounded + Add<Output = T> + Mul<Output = T> + std::fmt::Debug,
    {
        pub fn new(n: usize) -> Self {
            let xs = vec![0.into(); n];
            Self::from_slice(&xs)
        }

        pub fn from_slice(xs: &[T]) -> Self {
            let len = xs.len();
            let vec = xs.iter().map(|&x| RangeMinMax::unit(x)).collect::<Vec<_>>();
            Self {
                segtree: LazySegtree::from(vec),
                len,
            }
        }

        pub fn set(&mut self, p: usize, x: T) {
            self.segtree.set(p, RangeMinMax::unit(x));
        }

        pub fn get(&mut self, p: usize) -> T {
            self.segtree.get(p).min
        }

        pub fn range_minmax<R>(&mut self, range: R) -> RangeMinMax<T>
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range)
        }

        pub fn range_min<R>(&mut self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).min
        }

        pub fn range_max<R>(&mut self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).max
        }

        pub fn all_minmax(&self) -> RangeMinMax<T> {
            self.segtree.all_prod()
        }

        pub fn all_min(&self) -> T {
            self.segtree.all_prod().min
        }

        pub fn all_max(&self) -> T {
            self.segtree.all_prod().max
        }

        pub fn range_chmin<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree
                .apply_range(range, ChminChmaxAffineAction::new_chmin(x))
        }

        pub fn range_chmax<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree
                .apply_range(range, ChminChmaxAffineAction::new_chmax(x))
        }

        pub fn range_add<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree
                .apply_range(range, ChminChmaxAffineAction::new_add(x))
        }

        pub fn range_affine<R>(&mut self, range: R, mul: T, add: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree
                .apply_range(range, ChminChmaxAffineAction::new_affine(mul, add))
        }

        pub fn range_update<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree
                .apply_range(range, ChminChmaxAffineAction::new_update(x))
        }

        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            self.len
        }
        pub fn to_vec(&mut self) -> Vec<T> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}

#[cfg(test)]
pub mod test_range_chmin_chmax_affine_range_min_max {
    use super::range_chmin_chmax_affine_range_min_max::{
        RangeChminChmaxAffineRangeMinMaxSegtree, RangeMinMax,
    };
    use rand::{Rng, SeedableRng, rngs::SmallRng};

    #[test]
    fn test_new_and_get() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAffineRangeMinMaxSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.get(0), 10);
        assert_eq!(segtree.get(2), 30);
        assert_eq!(segtree.get(4), 50);
    }

    #[test]
    fn test_range_min_max() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAffineRangeMinMaxSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.range_minmax(1..4), RangeMinMax { min: 20, max: 40 });
        assert_eq!(segtree.range_min(1..4), 20);
        assert_eq!(segtree.range_max(1..4), 40);
    }

    #[test]
    fn test_range_chmin() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAffineRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.range_chmin(1..4, 15);
        assert_eq!(segtree.to_vec(), vec![10, 15, 15, 15, 50]);
    }

    #[test]
    fn test_range_chmax() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAffineRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.range_chmax(1..4, 35);
        assert_eq!(segtree.to_vec(), vec![10, 35, 35, 40, 50]);
    }

    #[test]
    fn test_range_add() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAffineRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.range_add(1..4, 5);
        assert_eq!(segtree.to_vec(), vec![10, 25, 35, 45, 50]);
    }

    #[test]
    fn test_range_affine() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAffineRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.range_affine(1..4, 2, 3); // x -> 2x + 3
        assert_eq!(
            segtree.to_vec(),
            vec![10, 20 * 2 + 3, 30 * 2 + 3, 40 * 2 + 3, 50]
        );

        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAffineRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.range_affine(1..4, -1, 100); // x -> -x + 100
        assert_eq!(
            segtree.to_vec(),
            vec![10, -20 + 100, -30 + 100, -40 + 100, 50]
        );
    }

    #[test]
    fn test_range_update() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAffineRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.range_update(1..4, 100);
        assert_eq!(segtree.to_vec(), vec![10, 100, 100, 100, 50]);
    }

    #[test]
    fn test_chmin_add() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAffineRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.range_chmin(1..4, 25); // [10, 20, 25, 25, 50]
        segtree.range_add(0..3, 5); // [15, 25, 30, 25, 50]
        assert_eq!(segtree.to_vec(), vec![15, 25, 30, 25, 50]);
    }

    #[test]
    fn test_add_chmin() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAffineRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.range_add(0..3, 5); // [15, 25, 35, 40, 50]
        segtree.range_chmin(1..4, 28); // [15, 25, 28, 28, 50]
        assert_eq!(segtree.to_vec(), vec![15, 25, 28, 28, 50]);
    }

    #[test]
    fn test_chmax_add() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAffineRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.range_chmax(1..4, 25); // [10, 25, 30, 40, 50]
        segtree.range_add(0..3, 5); // [15, 30, 35, 40, 50]
        assert_eq!(segtree.to_vec(), vec![15, 30, 35, 40, 50]);
    }

    #[test]
    fn test_add_chmax() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAffineRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.range_add(0..3, 5); // [15, 25, 35, 40, 50]
        segtree.range_chmax(1..4, 30); // [15, 30, 35, 40, 50]
        assert_eq!(segtree.to_vec(), vec![15, 30, 35, 40, 50]);
    }

    #[test]
    fn test_chmin_chmax_add() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAffineRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.range_chmin(0..5, 35); // [10, 20, 30, 35, 35]
        segtree.range_chmax(0..5, 15); // [15, 20, 30, 35, 35]
        segtree.range_add(0..5, 3); // [18, 23, 33, 38, 38]
        assert_eq!(segtree.to_vec(), vec![18, 23, 33, 38, 38]);
    }

    #[test]
    fn test_chmin_chmax_affine() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeChminChmaxAffineRangeMinMaxSegtree::<i64>::from_slice(&xs);
        segtree.range_chmin(0..5, 35); // [10, 20, 30, 35, 35]
        segtree.range_chmax(0..5, 15); // [15, 20, 30, 35, 35]
        segtree.range_affine(0..5, 2, 3); // x -> 2x + 3
        // Expected: [15*2+3, 20*2+3, 30*2+3, 35*2+3, 35*2+3]
        //           [33, 43, 63, 73, 73]
        assert_eq!(segtree.to_vec(), vec![33, 43, 63, 73, 73]);
    }

    #[test]
    fn test_random_chmin_chmax_affine_min_max() {
        let mut rng = SmallRng::seed_from_u64(43);

        for _ in 0..100 {
            let n = rng.random_range(1..=10);
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(-2..=2)).collect();
            let mut segtree =
                RangeChminChmaxAffineRangeMinMaxSegtree::<i64>::from_slice(&naive_vec);

            for _ in 0..100 {
                let op_type = rng.random_range(0..7); // 7つの操作: set, chmin, chmax, add, affine, range_minmax, all_minmax

                match op_type {
                    0 => {
                        // set(p, x)
                        if n > 0 {
                            let p = rng.random_range(0..n);
                            let x = rng.random_range(-2..=2);
                            naive_vec[p] = x;
                            segtree.set(p, x);
                        }
                    }
                    1 => {
                        // range_chmin(range, x)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let x = rng.random_range(-1..=1);
                        for i in l..r {
                            naive_vec[i] = naive_vec[i].min(x);
                        }
                        segtree.range_chmin(l..r, x);
                    }
                    2 => {
                        // range_chmax(range, x)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let x = rng.random_range(-1..=1);
                        for i in l..r {
                            naive_vec[i] = naive_vec[i].max(x);
                        }
                        segtree.range_chmax(l..r, x);
                    }
                    3 => {
                        // range_add(range, x)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let x = rng.random_range(-1..=1);
                        for i in l..r {
                            naive_vec[i] += x;
                        }
                        segtree.range_add(l..r, x);
                    }
                    4 => {
                        // range_affine(range, mul, add)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let mul = rng.random_range(-1..=1); // 負の乗数も許可
                        let add = rng.random_range(-1..=1);
                        for i in l..r {
                            naive_vec[i] = naive_vec[i] * mul + add;
                        }
                        segtree.range_affine(l..r, mul, add);
                    }
                    5 => {
                        // range_minmax(range)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        if l < r {
                            let expected_min = naive_vec[l..r].iter().copied().min().unwrap();
                            let expected_max = naive_vec[l..r].iter().copied().max().unwrap();
                            let result = segtree.range_minmax(l..r);
                            assert_eq!(result.min, expected_min, "range_min({}..{}) failed", l, r);
                            assert_eq!(result.max, expected_max, "range_max({}..{}) failed", l, r);
                        } else {
                            let result = segtree.range_minmax(l..r);
                            assert_eq!(result.min, i64::MAX);
                            assert_eq!(result.max, i64::MIN);
                        }
                    }
                    6 => {
                        // all_minmax
                        let expected_min = naive_vec.iter().copied().min().unwrap_or(i64::MAX);
                        let expected_max = naive_vec.iter().copied().max().unwrap_or(i64::MIN);
                        let result = segtree.all_minmax();
                        assert_eq!(result.min, expected_min, "all_min() failed");
                        assert_eq!(result.max, expected_max, "all_max() failed");
                    }
                    _ => unreachable!(),
                }
                assert_eq!(
                    segtree.all_min(),
                    naive_vec.iter().copied().min().unwrap_or(i64::MAX),
                    "all_min() failed"
                );
                assert_eq!(
                    segtree.all_max(),
                    naive_vec.iter().copied().max().unwrap_or(i64::MIN),
                    "all_max() failed"
                );
            }

            // 最終チェック
            assert_eq!(segtree.to_vec(), naive_vec, "final to_vec() check failed");
        }
    }
}
