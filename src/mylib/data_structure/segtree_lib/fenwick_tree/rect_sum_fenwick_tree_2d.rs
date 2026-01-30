use crate::math::algebra::ab_group::ab_group::{AbGroup, AdditiveAbGroup};
use cargo_snippet::snippet;

#[snippet(prefix = "use rect_sum_fenwick_tree_2d::*;", include = "ab_group")]
#[allow(clippy::module_inception)]
pub mod rect_sum_fenwick_tree_2d {
    use super::{AbGroup, AdditiveAbGroup};
    use std::ops::{Bound, RangeBounds};

    /// 可換群 (AbGroup) を用いた汎用的な 2次元 Fenwick Tree (Rect Sum Fenwick Tree 2D)。
    ///
    /// 0-indexed で実装されています。
    /// 矩形領域の和の取得（2次元累積和）と要素への加算を O(log H * log W) で行います。
    #[derive(Clone)]
    pub struct RectSumFenwickTree2DArbitrary<G: AbGroup> {
        h: usize,
        w: usize,
        data: Vec<Vec<G::S>>,
    }

    /// i64 の加算群を用いた標準的な 2次元 Fenwick Tree のエイリアス。
    pub type RectSumFenwickTree2DI64 = RectSumFenwickTree2DArbitrary<AdditiveAbGroup<i64>>;

    /// 任意の数値型 T の加算群を用いた 2次元 Fenwick Tree のエイリアス。
    pub type RectSumFenwickTree2D<T> = RectSumFenwickTree2DArbitrary<AdditiveAbGroup<T>>;

    impl<G: AbGroup> RectSumFenwickTree2DArbitrary<G> {
        /// H × W の 2次元 Fenwick Tree を作成します。
        /// 要素はすべて `G::zero()` で初期化されます。
        ///
        /// # 計算量
        /// O(H * W)
        pub fn new(h: usize, w: usize) -> Self {
            let data = vec![vec![G::zero(); w]; h];
            Self { h, w, data }
        }

        /// 配列の 2次元スライスから Fenwick Tree を作成します。
        ///
        /// # 計算量
        /// O(H * W)
        pub fn from_slice(slice: &[Vec<G::S>]) -> Self {
            let h = slice.len();
            let w = if h == 0 { 0 } else { slice[0].len() };
            let mut data = slice.to_vec();

            // 各行に対して 1次元 BIT の構築アルゴリズムを適用
            for i in 0..h {
                for j in 0..w {
                    let next_j = j | (j + 1);
                    if next_j < w {
                        let val = data[i][j].clone();
                        data[i][next_j] = G::add(&data[i][next_j], &val);
                    }
                }
            }

            // 各列に対して 1次元 BIT の構築アルゴリズムを適用
            for j in 0..w {
                for i in 0..h {
                    let next_i = i | (i + 1);
                    if next_i < h {
                        let val = data[i][j].clone();
                        data[next_i][j] = G::add(&data[next_i][j], &val);
                    }
                }
            }

            Self { h, w, data }
        }

        /// `(y, x)` 番目の要素に `val` を加算（群の演算を適用）します。
        ///
        /// # Panics
        /// 座標が範囲外の場合にパニックします。
        ///
        /// # 計算量
        /// O(log H * log W)
        pub fn add(&mut self, mut y: usize, x: usize, val: G::S) {
            assert!(
                y < self.h && x < self.w,
                "RectSumFenwickTree2D::add: out of bounds. (y, x) = ({}, {}), (h, w) = ({}, {})",
                y,
                x,
                self.h,
                self.w
            );
            y += 1;
            while y <= self.h {
                let mut x_idx = x + 1;
                while x_idx <= self.w {
                    self.data[y - 1][x_idx - 1] = G::add(&self.data[y - 1][x_idx - 1], &val);
                    x_idx += x_idx & x_idx.wrapping_neg();
                }
                y += y & y.wrapping_neg();
            }
        }

        /// `[0, y) × [0, x)` の矩形領域の総和を計算します。
        ///
        /// # Panics
        /// 座標が `(H, W)` を超える場合にパニックします。
        ///
        /// # 計算量
        /// O(log H * log W)
        pub fn prefix_sum(&self, mut y: usize, x: usize) -> G::S {
            assert!(
                y <= self.h && x <= self.w,
                "RectSumFenwickTree2D::prefix_sum: out of bounds. (y, x) = ({}, {}), (h, w) = ({}, {})",
                y,
                x,
                self.h,
                self.w
            );
            let mut res = G::zero();
            while y > 0 {
                let mut x_idx = x;
                while x_idx > 0 {
                    res = G::add(&res, &self.data[y - 1][x_idx - 1]);
                    x_idx &= x_idx - 1;
                }
                y &= y - 1;
            }
            res
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

            assert!(
                y1 <= y2 && y2 <= self.h,
                "RectSumFenwickTree2D::rect_sum: invalid y range. y_range = {}..{}, h = {}",
                y1,
                y2,
                self.h
            );
            assert!(
                x1 <= x2 && x2 <= self.w,
                "RectSumFenwickTree2D::rect_sum: invalid x range. x_range = {}..{}, w = {}",
                x1,
                x2,
                self.w
            );

            // 二次元累積和の原理 (包除原理): S(y2, x2) - S(y1, x2) - S(y2, x1) + S(y1, x1)
            let term1 = self.prefix_sum(y2, x2);
            let term2 = self.prefix_sum(y1, x2);
            let term3 = self.prefix_sum(y2, x1);
            let term4 = self.prefix_sum(y1, x1);

            let res = G::sub(&term1, &term2);
            let res = G::sub(&res, &term3);
            G::add(&res, &term4)
        }

        /// `(y, x)` 番目の要素の値を取得します。
        ///
        /// # 計算量
        /// O(log H * log W)
        pub fn get(&self, y: usize, x: usize) -> G::S {
            self.rect_sum(y..=y, x..=x)
        }

        /// `(y, x)` 番目の要素の値を `val` に設定します。
        ///
        /// # 計算量
        /// O(log H * log W)
        pub fn set(&mut self, y: usize, x: usize, val: G::S) {
            let old = self.get(y, x);
            self.add(y, x, G::sub(&val, &old));
        }

        /// 2次元 Fenwick Tree の現在の状態を `Vec<Vec<G::S>>` として返します。
        ///
        /// # 計算量
        /// O(H * W * log H * log W)
        pub fn to_vec(&self) -> Vec<Vec<G::S>> {
            let mut res = vec![vec![G::zero(); self.w]; self.h];
            for y in 0..self.h {
                for x in 0..self.w {
                    res[y][x] = self.get(y, x);
                }
            }
            res
        }

        pub fn len_h(&self) -> usize {
            self.h
        }

        pub fn len_w(&self) -> usize {
            self.w
        }
    }
}

#[cfg(test)]
mod tests {
    use super::rect_sum_fenwick_tree_2d::*;
    use crate::math::algebra::ab_group::ab_group::AdditiveAbGroup;
    use rand::{Rng, SeedableRng, rngs::SmallRng};

    #[test]
    #[allow(clippy::useless_vec)]
    fn test_rect_sum_fenwick_tree_2d_basic() {
        type G = AdditiveAbGroup<i64>;
        let mut ft = RectSumFenwickTree2DArbitrary::<G>::new(3, 3);

        // [1 2 3]
        // [4 5 6]
        // [7 8 9]
        let vals = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        for y in 0..3 {
            for x in 0..3 {
                ft.add(y, x, vals[y][x]);
            }
        }

        assert_eq!(ft.prefix_sum(1, 1), 1);
        assert_eq!(ft.prefix_sum(2, 2), 1 + 2 + 4 + 5);
        assert_eq!(ft.prefix_sum(3, 3), 45);

        assert_eq!(ft.rect_sum(1..3, 1..3), 5 + 6 + 8 + 9);
        assert_eq!(ft.get(1, 1), 5);

        ft.set(1, 1, 10);
        assert_eq!(ft.get(1, 1), 10);
        assert_eq!(ft.rect_sum(1..2, 1..2), 10);
    }

    #[test]
    fn test_rect_sum_fenwick_tree_2d_from_slice() {
        type G = AdditiveAbGroup<i64>;
        let vals = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let ft = RectSumFenwickTree2DArbitrary::<G>::from_slice(&vals);

        assert_eq!(ft.len_h(), 3);
        assert_eq!(ft.len_w(), 3);
        assert_eq!(ft.prefix_sum(3, 3), 45);
        assert_eq!(ft.rect_sum(1..3, 1..3), 5 + 6 + 8 + 9);
        for y in 0..3 {
            for x in 0..3 {
                assert_eq!(ft.get(y, x), vals[y][x]);
            }
        }
    }

    #[test]
    #[ignore]
    fn test_random_rect_sum_fenwick_tree_2d() {
        type G = AdditiveAbGroup<i64>;
        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..20 {
            let h = rng.random_range(1..=10);
            let w = rng.random_range(1..=10);
            let mut naive = vec![vec![0i64; w]; h];
            let mut ft = RectSumFenwickTree2DArbitrary::<G>::new(h, w);

            for _ in 0..50 {
                let op = rng.random_range(0..3);
                match op {
                    0 => {
                        let y = rng.random_range(0..h);
                        let x = rng.random_range(0..w);
                        let val = rng.random_range(-100..=100);
                        naive[y][x] += val;
                        ft.add(y, x, val);
                    }
                    1 => {
                        let y = rng.random_range(0..h);
                        let x = rng.random_range(0..w);
                        let val = rng.random_range(-100..=100);
                        naive[y][x] = val;
                        ft.set(y, x, val);
                    }
                    2 => {
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
                    _ => unreachable!(),
                }
            }
        }
    }
}
