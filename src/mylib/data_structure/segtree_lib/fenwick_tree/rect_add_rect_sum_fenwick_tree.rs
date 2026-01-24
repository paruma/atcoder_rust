use crate::data_structure::segtree_lib::fenwick_tree::fenwick_tree_2d::fenwick_tree_2d::FenwickTree2DArbitrary;
use crate::math::algebra::ab_group::ab_group::{AbGroup, AdditiveAbGroup};
use cargo_snippet::snippet;

#[snippet(
    prefix = "use rect_add_rect_sum_fenwick_tree::*;",
    include = "fenwick_tree_2d"
)]
#[allow(clippy::module_inception)]
pub mod rect_add_rect_sum_fenwick_tree {
    use super::{AbGroup, AdditiveAbGroup, FenwickTree2DArbitrary};
    use std::ops::{Bound, RangeBounds};

    /// 矩形加算・矩形和取得が可能な 2次元 Fenwick Tree。
    ///
    /// 内部的には 4 つの `FenwickTree2DArbitrary` を用いて、
    /// 2次元累積和の各項を管理しています。
    #[derive(Clone)]
    pub struct RectAddRectSumFenwickTreeArbitrary<G: AbGroup> {
        h: usize,
        w: usize,
        bit00: FenwickTree2DArbitrary<G>,
        bit01: FenwickTree2DArbitrary<G>,
        bit10: FenwickTree2DArbitrary<G>,
        bit11: FenwickTree2DArbitrary<G>,
    }

    /// i64 の加算群を用いた標準的な 2次元矩形加算・矩形和 Fenwick Tree のエイリアス。
    pub type RectAddRectSumFenwickTreeI64 =
        RectAddRectSumFenwickTreeArbitrary<AdditiveAbGroup<i64>>;

    /// 任意の数値型 T の加算群を用いた 2次元矩形加算・矩形和 Fenwick Tree のエイリアス。
    pub type RectAddRectSumFenwickTree<T> = RectAddRectSumFenwickTreeArbitrary<AdditiveAbGroup<T>>;

    impl<G: AbGroup> RectAddRectSumFenwickTreeArbitrary<G>
    where
        G::S: Copy + std::ops::Mul<Output = G::S> + From<i64>,
    {
        /// H × W の 2次元矩形加算・矩形和 Fenwick Tree を作成します。
        ///
        /// # 計算量
        /// O(H * W)
        pub fn new(h: usize, w: usize) -> Self {
            Self {
                h,
                w,
                bit00: FenwickTree2DArbitrary::new(h + 1, w + 1),
                bit01: FenwickTree2DArbitrary::new(h + 1, w + 1),
                bit10: FenwickTree2DArbitrary::new(h + 1, w + 1),
                bit11: FenwickTree2DArbitrary::new(h + 1, w + 1),
            }
        }

        /// 指定された矩形領域 `y_range` × `x_range` に `val` を加算します。
        ///
        /// # Panics
        /// 範囲が不正、または領域外の場合にパニックします。
        ///
        /// # 計算量
        /// O(log H * log W)
        pub fn rect_add<Ry, Rx>(&mut self, y_range: Ry, x_range: Rx, val: G::S)
        where
            Ry: RangeBounds<usize>,
            Rx: RangeBounds<usize>,
        {
            let y1 = match y_range.start_bound() {
                Bound::Included(&y) => y,
                Bound::Excluded(&y) => y + 1,
                Bound::Unbounded => 0,
            };
            let y2 = match y_range.end_bound() {
                Bound::Included(&y) => y + 1,
                Bound::Excluded(&y) => y,
                Bound::Unbounded => self.h,
            };
            let x1 = match x_range.start_bound() {
                Bound::Included(&x) => x,
                Bound::Excluded(&x) => x + 1,
                Bound::Unbounded => 0,
            };
            let x2 = match x_range.end_bound() {
                Bound::Included(&x) => x + 1,
                Bound::Excluded(&x) => x,
                Bound::Unbounded => self.w,
            };

            assert!(y1 <= y2 && y2 <= self.h);
            assert!(x1 <= x2 && x2 <= self.w);

            let add = |this: &mut Self, y: usize, x: usize, v: G::S| {
                if y <= this.h && x <= this.w {
                    this.bit00.add(y, x, v);
                    this.bit01.add(y, x, v * G::S::from(x as i64));
                    this.bit10.add(y, x, v * G::S::from(y as i64));
                    this.bit11
                        .add(y, x, v * G::S::from(y as i64) * G::S::from(x as i64));
                }
            };

            add(self, y1, x1, val);
            add(self, y1, x2, G::neg(&val));
            add(self, y2, x1, G::neg(&val));
            add(self, y2, x2, val);
        }

        /// 左上 (0,0) から右下 (y,x) までの矩形和を取得します。
        ///
        /// # 計算量
        /// O(log H * log W)
        pub fn accum(&self, y: usize, x: usize) -> G::S {
            let s00 = self.bit00.accum(y, x);
            let s01 = self.bit01.accum(y, x);
            let s10 = self.bit10.accum(y, x);
            let s11 = self.bit11.accum(y, x);

            let y_s = G::S::from(y as i64);
            let x_s = G::S::from(x as i64);

            // S(y, x) = y*x*s00 - y*s01 - x*s10 + s11
            let term1 = s00 * y_s * x_s;
            let term2 = s01 * y_s;
            let term3 = s10 * x_s;
            let term4 = s11;

            let res = G::sub(&term1, &term2);
            let res = G::sub(&res, &term3);
            G::add(&res, &term4)
        }

        /// 指定された矩形領域の和を計算します。
        ///
        /// # Panics
        /// 範囲が不正、または領域外の場合にパニックします。
        ///
        /// # 計算量
        /// O(log H * log W)
        pub fn rect_sum<Ry, Rx>(&self, y_range: Ry, x_range: Rx) -> G::S
        where
            Ry: RangeBounds<usize>,
            Rx: RangeBounds<usize>,
        {
            let y1 = match y_range.start_bound() {
                Bound::Included(&y) => y,
                Bound::Excluded(&y) => y + 1,
                Bound::Unbounded => 0,
            };
            let y2 = match y_range.end_bound() {
                Bound::Included(&y) => y + 1,
                Bound::Excluded(&y) => y,
                Bound::Unbounded => self.h,
            };
            let x1 = match x_range.start_bound() {
                Bound::Included(&x) => x,
                Bound::Excluded(&x) => x + 1,
                Bound::Unbounded => 0,
            };
            let x2 = match x_range.end_bound() {
                Bound::Included(&x) => x + 1,
                Bound::Excluded(&x) => x,
                Bound::Unbounded => self.w,
            };

            assert!(y1 <= y2 && y2 <= self.h);
            assert!(x1 <= x2 && x2 <= self.w);

            let term1 = self.accum(y2, x2);
            let term2 = self.accum(y1, x2);
            let term3 = self.accum(y2, x1);
            let term4 = self.accum(y1, x1);

            let res = G::sub(&term1, &term2);
            let res = G::sub(&res, &term3);
            G::add(&res, &term4)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::rect_add_rect_sum_fenwick_tree::*;
    use crate::math::algebra::ab_group::ab_group::AdditiveAbGroup;
    use rand::{Rng, SeedableRng, rngs::SmallRng};

    #[test]
    fn test_rect_add_rect_sum_2d_basic() {
        type G = AdditiveAbGroup<i64>;
        let (h, w) = (5, 5);
        let mut ft = RectAddRectSumFenwickTreeArbitrary::<G>::new(h, w);

        // [1, 3) x [1, 3) に 5 を加算
        ft.rect_add(1..3, 1..3, 5);
        // 矩形和: (2x2) * 5 = 20
        assert_eq!(ft.rect_sum(1..3, 1..3), 20);
        assert_eq!(ft.rect_sum(0..5, 0..5), 20);
        assert_eq!(ft.rect_sum(1..2, 1..2), 5);

        // [0, 2) x [0, 2) に 10 を加算
        ft.rect_add(0..2, 0..2, 10);
        // 重なり部分 [1, 2) x [1, 2) は 5 + 10 = 15
        assert_eq!(ft.rect_sum(1..2, 1..2), 15);
        // 全体の和: 20 (前の加算) + 40 (今回の加算) = 60
        assert_eq!(ft.rect_sum(0..5, 0..5), 60);
    }

    #[test]
    #[ignore]
    fn test_random_rect_add_rect_sum_2d() {
        type G = AdditiveAbGroup<i64>;
        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..20 {
            let h = rng.random_range(1..=10);
            let w = rng.random_range(1..=10);
            let mut naive = vec![vec![0i64; w]; h];
            let mut ft = RectAddRectSumFenwickTreeArbitrary::<G>::new(h, w);

            for _ in 0..50 {
                let op = rng.random_range(0..2);
                if op == 0 {
                    // rect_add
                    let y1 = rng.random_range(0..=h);
                    let y2 = rng.random_range(y1..=h);
                    let x1 = rng.random_range(0..=w);
                    let x2 = rng.random_range(x1..=w);
                    let val = rng.random_range(-100..=100);

                    for y in y1..y2 {
                        for x in x1..x2 {
                            naive[y][x] += val;
                        }
                    }
                    ft.rect_add(y1..y2, x1..x2, val);
                } else {
                    // rect_sum
                    let y1 = rng.random_range(0..=h);
                    let y2 = rng.random_range(y1..=h);
                    let x1 = rng.random_range(0..=w);
                    let x2 = rng.random_range(x1..=w);
                    let mut expected = 0;
                    for y in y1..y2 {
                        for x in x1..x2 {
                            expected += naive[y][x];
                        }
                    }
                    assert_eq!(ft.rect_sum(y1..y2, x1..x2), expected);
                }
            }
        }
    }
}
