use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use two_sequence_range_affine_range_sum::*;")]
pub mod two_sequence_range_affine_range_sum {
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::marker::PhantomData;
    use std::ops::{Add, Mul, RangeBounds};

    // 区間が持つデータ (sum_x, sum_y, sum_xy, len)
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct TwoSequenceData<T> {
        pub sum_x: T,
        pub sum_y: T,
        pub sum_xy: T,
        pub len: i64,
    }

    impl<T> TwoSequenceData<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        pub fn unit(x_val: T, y_val: T) -> Self {
            Self {
                sum_x: x_val,
                sum_y: y_val,
                sum_xy: x_val * y_val,
                len: 1,
            }
        }
    }

    // TwoSequenceData の Monoid 定義
    pub struct TwoSequenceDataMonoid<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for TwoSequenceDataMonoid<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        type S = TwoSequenceData<T>;
        fn identity() -> Self::S {
            Self::S {
                sum_x: 0.into(),
                sum_y: 0.into(),
                sum_xy: 0.into(),
                len: 0,
            }
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            Self::S {
                sum_x: a.sum_x + b.sum_x,
                sum_y: a.sum_y + b.sum_y,
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

    // TwoSequenceAffine を使う MapMonoid 定義
    pub struct TwoSequenceRangeAffineRangeSum<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for TwoSequenceRangeAffineRangeSum<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        type M = TwoSequenceDataMonoid<T>;
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
            Self::F {
                a: f2.a * f1.a,
                b: f2.a * f1.b + f2.b,
                c: f2.c * f1.c,
                d: f2.c * f1.d + f2.d,
            }
        }

        fn mapping(f: &Self::F, x: &TwoSequenceData<T>) -> TwoSequenceData<T> {
            TwoSequenceData {
                sum_xy: f.a * f.c * x.sum_xy
                    + f.a * f.d * x.sum_x
                    + f.b * f.c * x.sum_y
                    + f.b * f.d * x.len.into(),
                sum_x: f.a * x.sum_x + f.b * x.len.into(),
                sum_y: f.c * x.sum_y + f.d * x.len.into(),
                len: x.len,
            }
        }
    }

    // LazySegtree をラップする構造体
    pub struct TwoSequenceRangeAffineRangeSumSegtree<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        segtree: LazySegtree<TwoSequenceRangeAffineRangeSum<T>>,
        len: usize,
    }

    impl<T> TwoSequenceRangeAffineRangeSumSegtree<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        /// `xs` と `ys` の初期シーケンスでセグメント木を構築します。
        pub fn new(xs: &[T], ys: &[T]) -> Self {
            assert_eq!(xs.len(), ys.len(), "xs and ys must have the same length");
            let xs_ys = xs
                .iter()
                .zip(ys.iter())
                .map(|(&x, &y)| TwoSequenceData::unit(x, y))
                .collect_vec();
            let len = xs_ys.len();
            Self {
                segtree: LazySegtree::from(xs_ys),
                len,
            }
        }

        /// 指定された区間 `range` に対して、`xs[i] ← a * xs[i] + b`, `ys[i] ← c * ys[i] + d`
        /// のアフィン変換を適用します。
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

        /// 指定された区間 `range` の `sum(xs[i] * ys[i])` を計算して返します。
        pub fn query_sum_xy(&mut self, range: impl RangeBounds<usize>) -> T {
            self.segtree.prod(range).sum_xy
        }

        /// 指定された区間 `range` の `sum(xs[i])` を計算して返します。
        pub fn query_sum_x(&mut self, range: impl RangeBounds<usize>) -> T {
            self.segtree.prod(range).sum_x
        }

        /// 指定された区間 `range` の `sum(ys[i])` を計算して返します。
        pub fn query_sum_y(&mut self, range: impl RangeBounds<usize>) -> T {
            self.segtree.prod(range).sum_y
        }

        /// 指定されたインデックス `p` の `xs[p]` と `ys[p]` の値を更新します。
        pub fn set(&mut self, p: usize, x: T, y: T) {
            self.segtree.set(p, TwoSequenceData::unit(x, y));
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
    use ac_library::ModInt998244353;

    use super::two_sequence_range_affine_range_sum::*;

    type Mint = ModInt998244353;

    #[test]
    fn test_two_sequence_range_affine_range_sum_simple() {
        let xs = [1, 2, 3, 4, 5];
        let ys = [5, 4, 3, 2, 1];

        let mut segtree = TwoSequenceRangeAffineRangeSumSegtree::<Mint>::new(
            &xs.map(Mint::new),
            &ys.map(Mint::new),
        );

        // 初期状態の sum(xy)
        // 1*5 + 2*4 + 3*3 + 4*2 + 5*1 = 5 + 8 + 9 + 8 + 5 = 35
        assert_eq!(segtree.query_sum_xy(..), Mint::new(35));
        assert_eq!(segtree.query_sum_xy(0..2), Mint::new(13)); // 1*5 + 2*4 = 5 + 8 = 13

        // 区間 [0, 2) に対して x = 2x + 1, y = y + 1 を適用
        // (x_old, y_old) -> (x_new, y_new)
        // (1, 5) -> (2*1+1, 5+1) = (3, 6)
        // (2, 4) -> (2*2+1, 4+1) = (5, 5)
        // (3, 3) -> (3, 3) (対象外)

        segtree.apply_range_affine(0..2, 2.into(), 1.into(), 1.into(), 1.into());

        // 変更後の sum(xy)
        // 3*6 + 5*5 + 3*3 + 4*2 + 5*1
        // = 18 + 25 + 9 + 8 + 5 = 65
        assert_eq!(segtree.query_sum_xy(..), Mint::new(65));
        assert_eq!(segtree.query_sum_xy(0..2), Mint::new(43)); // 3*6 + 5*5 = 18 + 25 = 43

        // 区間 [1, 3) に対して x = x, y = 3y - 2 を適用
        // (x_old, y_old) -> (x_new, y_new)
        // (5, 5) -> (5, 3*5-2) = (5, 13)
        // (3, 3) -> (3, 3*3-2) = (3, 7)

        segtree.apply_range_affine(1..3, 1.into(), 0.into(), 3.into(), (-2).into());

        // 変更後の sum(xy)
        // (3, 6) -> 18
        // (5, 13) -> 65
        // (3, 7) -> 21
        // (4, 2) -> 8
        // (5, 1) -> 5
        // 合計 = 18 + 65 + 21 + 8 + 5 = 117
        assert_eq!(segtree.query_sum_xy(..), Mint::new(117));
        assert_eq!(segtree.query_sum_xy(1..3), Mint::new(86)); // 5*13 + 3*7 = 65 + 21 = 86

        // set と get のテスト
        segtree.set(0, 10.into(), 20.into()); // x[0]=10, y[0]=20
        assert_eq!(segtree.get(0), (10.into(), 20.into()));
        // sum(xy) should change: 10*20 + 5*13 + 3*7 + 4*2 + 5*1 = 200 + 65 + 21 + 8 + 5 = 299
        assert_eq!(segtree.query_sum_xy(..), Mint::new(299));

        // 新しい関数のテスト
        // 初期状態:
        // x: [10, 5, 3, 4, 5]
        // y: [20, 13, 7, 2, 1]
        // sum_x = 10+5+3+4+5 = 27
        // sum_y = 20+13+7+2+1 = 43
        assert_eq!(segtree.query_sum_x(..), Mint::new(27));
        assert_eq!(segtree.query_sum_y(..), Mint::new(43));

        // apply_range_affine_x
        // x[1..3] に x = 2x + 3 を適用
        // x[1]: 5 -> 2*5+3 = 13
        // x[2]: 3 -> 2*3+3 = 9
        // x: [10, 13, 9, 4, 5]
        // y: [20, 13, 7, 2, 1]
        segtree.apply_range_affine_x(1..3, 2.into(), 3.into());
        assert_eq!(segtree.get(1), (13.into(), 13.into()));
        assert_eq!(segtree.get(2), (9.into(), 7.into()));
        // sum_x = 10+13+9+4+5 = 41
        assert_eq!(segtree.query_sum_x(..), Mint::new(41));
        // sum_xy = 10*20 + 13*13 + 9*7 + 4*2 + 5*1 = 200 + 169 + 63 + 8 + 5 = 445
        assert_eq!(segtree.query_sum_xy(..), Mint::new(445));

        // apply_range_affine_y
        // y[..2] に y = y - 5 を適用
        // y[0]: 20 -> 20-5 = 15
        // y[1]: 13 -> 13-5 = 8
        // x: [10, 13, 9, 4, 5]
        // y: [15, 8, 7, 2, 1]
        segtree.apply_range_affine_y(0..2, 1.into(), (-5).into());
        assert_eq!(segtree.get(0), (10.into(), 15.into()));
        assert_eq!(segtree.get(1), (13.into(), 8.into()));
        // sum_y = 15+8+7+2+1 = 33
        assert_eq!(segtree.query_sum_y(..), Mint::new(33));
        // sum_xy = 10*15 + 13*8 + 9*7 + 4*2 + 5*1 = 150 + 104 + 63 + 8 + 5 = 330
        assert_eq!(segtree.query_sum_xy(..), Mint::new(330));

        // set_x
        segtree.set_x(4, 100.into()); // x[4] = 100
        assert_eq!(segtree.get(4), (100.into(), 1.into()));
        // x: [10, 13, 9, 4, 100]
        // sum_x = 10+13+9+4+100 = 136
        assert_eq!(segtree.query_sum_x(..), Mint::new(136));
        // sum_xy = 10*15 + 13*8 + 9*7 + 4*2 + 100*1 = 150 + 104 + 63 + 8 + 100 = 425
        assert_eq!(segtree.query_sum_xy(..), Mint::new(425));

        // set_y
        segtree.set_y(3, 100.into()); // y[3] = 100
        assert_eq!(segtree.get(3), (4.into(), 100.into()));
        // y: [15, 8, 7, 100, 1]
        // sum_y = 15+8+7+100+1 = 131
        assert_eq!(segtree.query_sum_y(..), Mint::new(131));
        // sum_xy = 10*15 + 13*8 + 9*7 + 4*100 + 100*1 = 150 + 104 + 63 + 400 + 100 = 817
        assert_eq!(segtree.query_sum_xy(..), Mint::new(817));

        // to_vec
        let (xs_vec, ys_vec) = segtree.to_vec();
        assert_eq!(
            xs_vec,
            [10, 13, 9, 4, 100]
                .iter()
                .map(|&v| Mint::new(v))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            ys_vec,
            [15, 8, 7, 100, 1]
                .iter()
                .map(|&v| Mint::new(v))
                .collect::<Vec<_>>()
        );
    }
}
