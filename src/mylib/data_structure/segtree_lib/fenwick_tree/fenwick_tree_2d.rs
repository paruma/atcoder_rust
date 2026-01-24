use crate::math::algebra::ab_group::ab_group::{AbGroup, AdditiveAbGroup};
use cargo_snippet::snippet;

#[snippet(prefix = "use fenwick_tree_2d::*;", include = "ab_group")]
#[allow(clippy::module_inception)]
pub mod fenwick_tree_2d {
    use super::{AbGroup, AdditiveAbGroup};
    use std::ops::{Bound, RangeBounds};

    /// 可換群 (AbGroup) を用いた汎用的な 2次元 Fenwick Tree。
    ///
    /// 0-indexed で実装されています。
    /// 矩形領域の和の取得（2次元累積和）と要素への加算を O(log H * log W) で行います。
    #[derive(Clone)]
    pub struct FenwickTree2DArbitrary<G: AbGroup> {
        h: usize,
        w: usize,
        data: Vec<Vec<G::S>>,
    }

    /// i64 の加算群を用いた標準的な 2次元 Fenwick Tree のエイリアス。
    pub type FenwickTree2DI64 = FenwickTree2DArbitrary<AdditiveAbGroup<i64>>;

    /// 任意の数値型 T の加算群を用いた 2次元 Fenwick Tree のエイリアス。
    pub type FenwickTree2D<T> = FenwickTree2DArbitrary<AdditiveAbGroup<T>>;

    impl<G: AbGroup> FenwickTree2DArbitrary<G> {
        /// H × W の 2次元 Fenwick Tree を作成します。
        /// 要素はすべて `G::zero()` で初期化されます。
        ///
        /// # 計算量
        /// O(H * W)
        pub fn new(h: usize, w: usize) -> Self {
            let mut data = Vec::with_capacity(h);
            for _ in 0..h {
                let mut row = Vec::with_capacity(w);
                for _ in 0..w {
                    row.push(G::zero());
                }
                data.push(row);
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
                "FenwickTree2D::add: out of bounds"
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
        pub fn accum(&self, mut y: usize, x: usize) -> G::S {
            assert!(
                y <= self.h && x <= self.w,
                "FenwickTree2D::accum: out of bounds"
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
                "FenwickTree2D::rect_sum: invalid y range"
            );
            assert!(
                x1 <= x2 && x2 <= self.w,
                "FenwickTree2D::rect_sum: invalid x range"
            );

            // 二次元累積和の原理 (包除原理): S(y2, x2) - S(y1, x2) - S(y2, x1) + S(y1, x1)
            let term1 = self.accum(y2, x2);
            let term2 = self.accum(y1, x2);
            let term3 = self.accum(y2, x1);
            let term4 = self.accum(y1, x1);

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
    use super::fenwick_tree_2d::*;
    use crate::math::algebra::ab_group::ab_group::AdditiveAbGroup;
    use rand::{Rng, SeedableRng, rngs::SmallRng};

    #[test]
    #[allow(clippy::useless_vec)]
    fn test_fenwick_tree_2d_basic() {
        type G = AdditiveAbGroup<i64>;
        let mut ft = FenwickTree2DArbitrary::<G>::new(3, 3);

        // [1 2 3]
        // [4 5 6]
        // [7 8 9]
        let vals = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        for y in 0..3 {
            for x in 0..3 {
                ft.add(y, x, vals[y][x]);
            }
        }

        assert_eq!(ft.accum(1, 1), 1);
        assert_eq!(ft.accum(2, 2), 1 + 2 + 4 + 5);
        assert_eq!(ft.accum(3, 3), 45);

        assert_eq!(ft.rect_sum(1..3, 1..3), 5 + 6 + 8 + 9);
        assert_eq!(ft.get(1, 1), 5);

        ft.set(1, 1, 10);
        assert_eq!(ft.get(1, 1), 10);
        assert_eq!(ft.rect_sum(1..2, 1..2), 10);
    }

    #[test]
    #[ignore]
    fn test_random_fenwick_tree_2d() {
        type G = AdditiveAbGroup<i64>;
        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..20 {
            let h = rng.random_range(1..=10);
            let w = rng.random_range(1..=10);
            let mut naive = vec![vec![0i64; w]; h];
            let mut ft = FenwickTree2DArbitrary::<G>::new(h, w);

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
