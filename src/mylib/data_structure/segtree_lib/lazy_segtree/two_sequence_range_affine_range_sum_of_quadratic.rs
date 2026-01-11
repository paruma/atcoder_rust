use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use two_sequence_range_affine_range_sum_of_quadratic::*;")]
pub mod two_sequence_range_affine_range_sum_of_quadratic {
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::marker::PhantomData;
    use std::ops::{Add, Mul, RangeBounds};

    // 区間が持つデータ
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct TwoSequenceQuadraticData<T> {
        pub sum_x: T,
        pub sum_y: T,
        pub sum_x2: T,
        pub sum_y2: T,
        pub sum_xy: T,
        pub len: i64,
    }

    impl<T> TwoSequenceQuadraticData<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        pub fn unit(x_val: T, y_val: T) -> Self {
            Self {
                sum_x: x_val,
                sum_y: y_val,
                sum_x2: x_val * x_val,
                sum_y2: y_val * y_val,
                sum_xy: x_val * y_val,
                len: 1,
            }
        }
    }

    // Monoid 定義
    pub struct TwoSequenceQuadraticDataMonoid<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for TwoSequenceQuadraticDataMonoid<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        type S = TwoSequenceQuadraticData<T>;
        fn identity() -> Self::S {
            Self::S {
                sum_x: 0.into(),
                sum_y: 0.into(),
                sum_x2: 0.into(),
                sum_y2: 0.into(),
                sum_xy: 0.into(),
                len: 0,
            }
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            Self::S {
                sum_x: a.sum_x + b.sum_x,
                sum_y: a.sum_y + b.sum_y,
                sum_x2: a.sum_x2 + b.sum_x2,
                sum_y2: a.sum_y2 + b.sum_y2,
                sum_xy: a.sum_xy + b.sum_xy,
                len: a.len + b.len,
            }
        }
    }

    // 区間に作用する affine 変換 (x -> ax+b, y -> cy+d)
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct TwoSequenceAffine<T> {
        pub a: T,
        pub b: T,
        pub c: T,
        pub d: T,
    }

    // MapMonoid 定義
    pub struct TwoSequenceRangeAffineRangeSumOfQuadratic<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for TwoSequenceRangeAffineRangeSumOfQuadratic<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        type M = TwoSequenceQuadraticDataMonoid<T>;
        type F = TwoSequenceAffine<T>;

        fn identity_map() -> Self::F {
            Self::F {
                a: 1.into(),
                b: 0.into(),
                c: 1.into(),
                d: 0.into(),
            }
        }

        fn composition(f1: &Self::F, f2: &Self::F) -> Self::F {
            // f1(f2(x)) = a1(a2*x + b2) + b1 = a1*a2*x + a1*b2 + b1
            Self::F {
                a: f1.a * f2.a,
                b: f1.a * f2.b + f1.b,
                c: f1.c * f2.c,
                d: f1.c * f2.d + f1.d,
            }
        }

        fn mapping(f: &Self::F, data: &TwoSequenceQuadraticData<T>) -> TwoSequenceQuadraticData<T> {
            let a = f.a;
            let b = f.b;
            let c = f.c;
            let d = f.d;
            let len_t: T = data.len.into();

            let new_sum_x = a * data.sum_x + b * len_t;
            let new_sum_y = c * data.sum_y + d * len_t;

            let new_sum_x2 = a * a * data.sum_x2 + (a + a) * b * data.sum_x + b * b * len_t;
            let new_sum_y2 = c * c * data.sum_y2 + (c + c) * d * data.sum_y + d * d * len_t;

            let new_sum_xy =
                a * c * data.sum_xy + a * d * data.sum_x + b * c * data.sum_y + b * d * len_t;

            TwoSequenceQuadraticData {
                sum_x: new_sum_x,
                sum_y: new_sum_y,
                sum_x2: new_sum_x2,
                sum_y2: new_sum_y2,
                sum_xy: new_sum_xy,
                len: data.len,
            }
        }
    }

    // LazySegtree をラップする構造体
    pub struct TwoSequenceRangeAffineRangeSumOfQuadraticSegtree<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        segtree: LazySegtree<TwoSequenceRangeAffineRangeSumOfQuadratic<T>>,
        len: usize,
    }

    impl<T> TwoSequenceRangeAffineRangeSumOfQuadraticSegtree<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        /// `xs` と `ys` の初期シーケンスでセグメント木を構築します。
        pub fn new(xs: &[T], ys: &[T]) -> Self {
            assert_eq!(xs.len(), ys.len(), "xs and ys must have the same length");
            let xs_ys = xs
                .iter()
                .zip(ys.iter())
                .map(|(&x, &y)| TwoSequenceQuadraticData::unit(x, y))
                .collect_vec();
            let len = xs_ys.len();
            Self {
                segtree: LazySegtree::from(xs_ys),
                len,
            }
        }

        /// 指定された区間 `range` に対して、`xs[i] ← a * xs[i] + b`, `ys[i] ← c * ys[i] + d` のアフィン変換を適用します。
        pub fn apply_range_affine(
            &mut self,
            range: impl RangeBounds<usize>,
            a: T,
            b: T,
            c: T,
            d: T,
        ) {
            self.segtree
                .apply_range(range, TwoSequenceAffine { a, b, c, d })
        }

        /// 指定された区間 `range` に対して、`xs[i] ← a * xs[i] + b` のアフィン変換を適用します。
        pub fn apply_range_affine_x(&mut self, range: impl RangeBounds<usize>, a: T, b: T) {
            self.apply_range_affine(range, a, b, 1.into(), 0.into())
        }

        /// 指定された区間 `range` に対して、`ys[i] ← c * ys[i] + d` のアフィン変換を適用します。
        pub fn apply_range_affine_y(&mut self, range: impl RangeBounds<usize>, c: T, d: T) {
            self.apply_range_affine(range, 1.into(), 0.into(), c, d)
        }

        /// 指定された区間 `range` に対して、`xs[i] ← xs[i] + b` の加算を適用します。
        pub fn apply_range_add_x(&mut self, range: impl RangeBounds<usize>, b: T) {
            self.apply_range_affine_x(range, 1.into(), b)
        }

        /// 指定された区間 `range` に対して、`xs[i] ← x` の更新を適用します。
        pub fn apply_range_update_x(&mut self, range: impl RangeBounds<usize>, x: T) {
            self.apply_range_affine_x(range, 0.into(), x)
        }

        /// 指定された区間 `range` に対して、`ys[i] ← ys[i] + d` の加算を適用します。
        pub fn apply_range_add_y(&mut self, range: impl RangeBounds<usize>, d: T) {
            self.apply_range_affine_y(range, 1.into(), d)
        }

        /// 指定された区間 `range` に対して、`ys[i] ← y` の更新を適用します。
        pub fn apply_range_update_y(&mut self, range: impl RangeBounds<usize>, y: T) {
            self.apply_range_affine_y(range, 0.into(), y)
        }

        /// 指定された区間 `range` の `sum(xs[i] * ys[i])` を計算して返します。
        pub fn range_sum_xy(&mut self, range: impl RangeBounds<usize>) -> T {
            self.segtree.prod(range).sum_xy
        }

        /// 指定された区間 `range` の `sum(xs[i] * xs[i])` を計算して返します。
        pub fn range_sum_x2(&mut self, range: impl RangeBounds<usize>) -> T {
            self.segtree.prod(range).sum_x2
        }

        /// 指定された区間 `range` の `sum(ys[i] * ys[i])` を計算して返します。
        pub fn range_sum_y2(&mut self, range: impl RangeBounds<usize>) -> T {
            self.segtree.prod(range).sum_y2
        }

        /// 指定された区間 `range` の `sum(xs[i])` を計算して返します。
        pub fn range_sum_x(&mut self, range: impl RangeBounds<usize>) -> T {
            self.segtree.prod(range).sum_x
        }

        /// 指定された区間 `range` の `sum(ys[i])` を計算して返します。
        pub fn range_sum_y(&mut self, range: impl RangeBounds<usize>) -> T {
            self.segtree.prod(range).sum_y
        }

        /// 指定されたインデックス `p` の `xs[p]` と `ys[p]` の値を更新します。
        pub fn set(&mut self, p: usize, x: T, y: T) {
            self.segtree.set(p, TwoSequenceQuadraticData::unit(x, y));
        }

        /// 指定されたインデックス `p` の `xs[p]` の値を更新します。
        pub fn set_x(&mut self, p: usize, x: T) {
            let (_, y) = self.get(p);
            self.set(p, x, y);
        }

        /// 指定されたインデックス `p` の `ys[p]` の値を更新します。
        pub fn set_y(&mut self, p: usize, y: T) {
            let (x, _) = self.get(p);
            self.set(p, x, y);
        }

        /// 指定されたインデックス `p` の `xs[p]` と `ys[p]` の値を取得します。
        pub fn get(&mut self, p: usize) -> (T, T) {
            let data = self.segtree.get(p);
            (data.sum_x, data.sum_y)
        }

        /// セグメント木の現在の状態を `(Vec<T>, Vec<T>)` として返します。
        pub fn to_vec(&mut self) -> (Vec<T>, Vec<T>) {
            (0..self.len).map(|i| self.get(i)).unzip()
        }
    }
}

#[cfg(test)]
mod test {
    use super::two_sequence_range_affine_range_sum_of_quadratic::*;
    use ac_library::ModInt998244353;

    type Mint = ModInt998244353;

    #[test]
    fn test_simple() {
        let xs = [1, 2, 3];
        let ys = [4, 5, 6];
        let mut segtree = TwoSequenceRangeAffineRangeSumOfQuadraticSegtree::<Mint>::new(
            &xs.map(Mint::new),
            &ys.map(Mint::new),
        );

        // Initial sums
        // sum_x = 1+2+3 = 6
        // sum_y = 4+5+6 = 15
        // sum_x2 = 1+4+9 = 14
        // sum_y2 = 16+25+36 = 77
        // sum_xy = 4+10+18 = 32
        assert_eq!(segtree.range_sum_x(..), 6.into());
        assert_eq!(segtree.range_sum_y(..), 15.into());
        assert_eq!(segtree.range_sum_x2(..), 14.into());
        assert_eq!(segtree.range_sum_y2(..), 77.into());
        assert_eq!(segtree.range_sum_xy(..), 32.into());

        // Apply affine to x: x_i -> 2*x_i + 1 for i in 0..2
        // x: [1, 2] -> [3, 5]
        // y: [4, 5] -> [4, 5]
        // New state for 0..2:
        // x: [3, 5], y: [4, 5]
        // sum_x = 8, sum_y = 9
        // sum_x2 = 9+25=34, sum_y2 = 16+25=41
        // sum_xy = 12+25=37
        segtree.apply_range_affine_x(0..2, 2.into(), 1.into());

        // Check sums for 0..2
        assert_eq!(segtree.range_sum_x(0..2), 8.into());
        assert_eq!(segtree.range_sum_y(0..2), 9.into());
        assert_eq!(segtree.range_sum_x2(0..2), 34.into());
        assert_eq!(segtree.range_sum_y2(0..2), 41.into());
        assert_eq!(segtree.range_sum_xy(0..2), 37.into());

        // Check full sums
        // x: [3, 5, 3], y: [4, 5, 6]
        // sum_x = 3+5+3 = 11
        // sum_y = 4+5+6 = 15
        // sum_x2 = 9+25+9 = 43
        // sum_y2 = 16+25+36 = 77
        // sum_xy = 12+25+18 = 55
        assert_eq!(segtree.range_sum_x(..), 11.into());
        assert_eq!(segtree.range_sum_y(..), 15.into());
        assert_eq!(segtree.range_sum_x2(..), 43.into());
        assert_eq!(segtree.range_sum_y2(..), 77.into());
        assert_eq!(segtree.range_sum_xy(..), 55.into());

        // Apply affine to y: y_i -> 3*y_i - 2 for i in 1..3
        // Current state: x: [3, 5, 3], y: [4, 5, 6]
        // y: [5, 6] -> [3*5-2, 3*6-2] = [13, 16]
        // New state: x: [3, 5, 3], y: [4, 13, 16]
        segtree.apply_range_affine_y(1..3, 3.into(), (-2).into());

        // Check full sums
        // sum_x = 11
        // sum_y = 4+13+16 = 33
        // sum_x2 = 43
        // sum_y2 = 16 + 169 + 256 = 441
        // sum_xy = 3*4 + 5*13 + 3*16 = 12 + 65 + 48 = 125
        assert_eq!(segtree.range_sum_x(..), 11.into());
        assert_eq!(segtree.range_sum_y(..), 33.into());
        assert_eq!(segtree.range_sum_x2(..), 43.into());
        assert_eq!(segtree.range_sum_y2(..), 441.into());
        assert_eq!(segtree.range_sum_xy(..), 125.into());

        // apply_range_add_x
        segtree.apply_range_add_x(0..1, 10.into()); // x[0]: 3 -> 13
        assert_eq!(segtree.get(0), (13.into(), 4.into()));

        // apply_range_update_x
        segtree.apply_range_update_x(1..2, 100.into()); // x[1]: 5 -> 100
        assert_eq!(segtree.get(1), (100.into(), 13.into()));

        // apply_range_add_y
        segtree.apply_range_add_y(2..3, 10.into()); // y[2]: 16 -> 26
        assert_eq!(segtree.get(2), (3.into(), 26.into()));

        // apply_range_update_y
        segtree.apply_range_update_y(0..1, 50.into()); // y[0]: 4 -> 50
        assert_eq!(segtree.get(0), (13.into(), 50.into()));

        // Check get and to_vec
        assert_eq!(segtree.get(0), (13.into(), 50.into()));
        assert_eq!(segtree.get(1), (100.into(), 13.into()));
        assert_eq!(segtree.get(2), (3.into(), 26.into()));

        let (xs_vec, ys_vec) = segtree.to_vec();
        assert_eq!(xs_vec, vec![Mint::new(13), Mint::new(100), Mint::new(3)]);
        assert_eq!(ys_vec, vec![Mint::new(50), Mint::new(13), Mint::new(26)]);
    }

    #[ignore]
    #[test]
    fn test_random() {
        use rand::{Rng, SeedableRng};

        let mut rng = rand::rngs::SmallRng::seed_from_u64(42);

        for _ in 0..20 {
            let n = rng.random_range(1..=20);
            let mut naive_xs: Vec<i64> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
            let mut naive_ys: Vec<i64> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
            let mut segtree =
                TwoSequenceRangeAffineRangeSumOfQuadraticSegtree::<i64>::new(&naive_xs, &naive_ys);

            for _ in 0..50 {
                let op_type = rng.random_range(0..16);

                match op_type {
                    0 => {
                        // set(p, x, y)
                        let p = rng.random_range(0..n);
                        let x = rng.random_range(-100..=100);
                        let y = rng.random_range(-100..=100);
                        naive_xs[p] = x;
                        naive_ys[p] = y;
                        segtree.set(p, x, y);
                    }
                    1 => {
                        // set_x(p, x)
                        let p = rng.random_range(0..n);
                        let x = rng.random_range(-100..=100);
                        naive_xs[p] = x;
                        segtree.set_x(p, x);
                    }
                    2 => {
                        // set_y(p, y)
                        let p = rng.random_range(0..n);
                        let y = rng.random_range(-100..=100);
                        naive_ys[p] = y;
                        segtree.set_y(p, y);
                    }
                    3 => {
                        // apply_range_affine
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let a = rng.random_range(-2..=2);
                        let b = rng.random_range(-50..=50);
                        let c = rng.random_range(-2..=2);
                        let d = rng.random_range(-50..=50);
                        for i in l..r {
                            naive_xs[i] = naive_xs[i] * a + b;
                            naive_ys[i] = naive_ys[i] * c + d;
                        }
                        segtree.apply_range_affine(l..r, a, b, c, d);
                    }
                    4 => {
                        // apply_range_affine_x
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let a = rng.random_range(-2..=2);
                        let b = rng.random_range(-50..=50);
                        for i in l..r {
                            naive_xs[i] = naive_xs[i] * a + b;
                        }
                        segtree.apply_range_affine_x(l..r, a, b);
                    }
                    5 => {
                        // apply_range_affine_y
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let c = rng.random_range(-2..=2);
                        let d = rng.random_range(-50..=50);
                        for i in l..r {
                            naive_ys[i] = naive_ys[i] * c + d;
                        }
                        segtree.apply_range_affine_y(l..r, c, d);
                    }
                    6 => {
                        // get(p)
                        let p = rng.random_range(0..n);
                        assert_eq!(
                            segtree.get(p),
                            (naive_xs[p], naive_ys[p]),
                            "get({}) failed",
                            p
                        );
                    }
                    7 => {
                        // range_sum_x(range)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let expected: i64 = naive_xs[l..r].iter().sum();
                        assert_eq!(
                            segtree.range_sum_x(l..r),
                            expected,
                            "range_sum_x({}..{}) failed",
                            l,
                            r
                        );
                    }
                    8 => {
                        // range_sum_y(range)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let expected: i64 = naive_ys[l..r].iter().sum();
                        assert_eq!(
                            segtree.range_sum_y(l..r),
                            expected,
                            "range_sum_y({}..{}) failed",
                            l,
                            r
                        );
                    }
                    9 => {
                        // range_sum_xy(range)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let expected: i64 = naive_xs[l..r]
                            .iter()
                            .zip(naive_ys[l..r].iter())
                            .map(|(&x, &y)| x * y)
                            .sum();
                        assert_eq!(
                            segtree.range_sum_xy(l..r),
                            expected,
                            "range_sum_xy({}..{}) failed",
                            l,
                            r
                        );
                    }
                    10 => {
                        // range_sum_x2(range)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let expected: i64 = naive_xs[l..r].iter().map(|&x| x * x).sum();
                        assert_eq!(
                            segtree.range_sum_x2(l..r),
                            expected,
                            "range_sum_x2({}..{}) failed",
                            l,
                            r
                        );
                    }
                    11 => {
                        // range_sum_y2(range)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let expected: i64 = naive_ys[l..r].iter().map(|&y| y * y).sum();
                        assert_eq!(
                            segtree.range_sum_y2(l..r),
                            expected,
                            "range_sum_y2({}..{}) failed",
                            l,
                            r
                        );
                    }
                    12 => {
                        // apply_range_add_x
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let b = rng.random_range(-50..=50);
                        for i in l..r {
                            naive_xs[i] += b;
                        }
                        segtree.apply_range_add_x(l..r, b);
                    }
                    13 => {
                        // apply_range_update_x
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let x = rng.random_range(-100..=100);
                        for i in l..r {
                            naive_xs[i] = x;
                        }
                        segtree.apply_range_update_x(l..r, x);
                    }
                    14 => {
                        // apply_range_add_y
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let d = rng.random_range(-50..=50);
                        for i in l..r {
                            naive_ys[i] += d;
                        }
                        segtree.apply_range_add_y(l..r, d);
                    }
                    15 => {
                        // apply_range_update_y
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let y = rng.random_range(-100..=100);
                        for i in l..r {
                            naive_ys[i] = y;
                        }
                        segtree.apply_range_update_y(l..r, y);
                    }
                    _ => unreachable!(),
                }
            }

            // Final check
            let (final_xs, final_ys) = segtree.to_vec();
            assert_eq!(final_xs, naive_xs, "final to_vec() xs check failed");
            assert_eq!(final_ys, naive_ys, "final to_vec() ys check failed");
        }
    }
}
