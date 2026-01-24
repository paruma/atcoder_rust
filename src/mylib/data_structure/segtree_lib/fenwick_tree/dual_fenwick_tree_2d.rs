use crate::data_structure::segtree_lib::fenwick_tree::fenwick_tree_2d::fenwick_tree_2d::FenwickTree2DArbitrary;
use crate::math::algebra::ab_group::ab_group::{AbGroup, AdditiveAbGroup};
use cargo_snippet::snippet;

#[snippet(prefix = "use dual_fenwick_tree_2d::*;", include = "fenwick_tree_2d")]
#[allow(clippy::module_inception)]
pub mod dual_fenwick_tree_2d {
    use super::{AbGroup, AdditiveAbGroup, FenwickTree2DArbitrary};
    use std::ops::{Bound, RangeBounds};

    /// 矩形加算・一点取得が可能な 2次元 Fenwick Tree (Dual Fenwick Tree 2D)。
    ///
    /// 内部的には 2次元の階差数列を `FenwickTree2DArbitrary` で管理しています。
    #[derive(Clone)]
    pub struct DualFenwickTree2DArbitrary<G: AbGroup> {
        ft: FenwickTree2DArbitrary<G>,
    }

    /// i64 の加算群を用いた標準的な 2次元双対 Fenwick Tree のエイリアス。
    pub type DualFenwickTree2DI64 = DualFenwickTree2DArbitrary<AdditiveAbGroup<i64>>;

    /// 任意の数値型 T の加算群を用いた 2次元双対 Fenwick Tree のエイリアス。
    pub type DualFenwickTree2D<T> = DualFenwickTree2DArbitrary<AdditiveAbGroup<T>>;

    impl<G: AbGroup> DualFenwickTree2DArbitrary<G> {
        /// H × W の 2次元双対 Fenwick Tree を作成します。
        /// 要素はすべて `G::zero()` で初期化されます。
        ///
        /// # 計算量
        /// O(H * W)
        pub fn new(h: usize, w: usize) -> Self {
            Self {
                ft: FenwickTree2DArbitrary::new(h + 1, w + 1),
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
            let h = self.ft.len_h() - 1;
            let w = self.ft.len_w() - 1;

            let y1 = match y_range.start_bound() {
                Bound::Included(&y) => y,
                Bound::Excluded(&y) => y + 1,
                Bound::Unbounded => 0,
            };
            let y2 = match y_range.end_bound() {
                Bound::Included(&y) => y + 1,
                Bound::Excluded(&y) => y,
                Bound::Unbounded => h,
            };
            let x1 = match x_range.start_bound() {
                Bound::Included(&x) => x,
                Bound::Excluded(&x) => x + 1,
                Bound::Unbounded => 0,
            };
            let x2 = match x_range.end_bound() {
                Bound::Included(&x) => x + 1,
                Bound::Excluded(&x) => x,
                Bound::Unbounded => w,
            };

            assert!(
                y1 <= y2 && y2 <= h,
                "DualFenwickTree2D::rect_add: invalid y range"
            );
            assert!(
                x1 <= x2 && x2 <= w,
                "DualFenwickTree2D::rect_add: invalid x range"
            );

            // 2次元いもす法の原理: 4点への加算
            self.ft.add(y1, x1, val.clone());
            self.ft.add(y1, x2, G::neg(&val));
            self.ft.add(y2, x1, G::neg(&val));
            self.ft.add(y2, x2, val);
        }

        /// `(y, x)` 番目の要素の値を取得します。
        ///
        /// # Panics
        /// 座標が範囲外の場合にパニックします。
        ///
        /// # 計算量
        /// O(log H * log W)
        pub fn get(&self, y: usize, x: usize) -> G::S {
            let h = self.ft.len_h() - 1;
            let w = self.ft.len_w() - 1;
            assert!(y < h && x < w, "DualFenwickTree2D::get: out of bounds");
            // 階差の 2次元累積和が元の値になる
            self.ft.accum(y + 1, x + 1)
        }

        /// 現在の状態を `Vec<Vec<G::S>>` として返します。
        ///
        /// # 計算量
        /// O(H * W * log H * log W)
        pub fn to_vec(&self) -> Vec<Vec<G::S>> {
            let h = self.ft.len_h() - 1;
            let w = self.ft.len_w() - 1;
            (0..h)
                .map(|y| (0..w).map(|x| self.get(y, x)).collect())
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::dual_fenwick_tree_2d::*;
    use crate::math::algebra::ab_group::ab_group::AdditiveAbGroup;
    use rand::{Rng, SeedableRng, rngs::SmallRng};

    #[test]
    fn test_dual_fenwick_tree_2d_basic() {
        type G = AdditiveAbGroup<i64>;
        let (h, w) = (4, 4);
        let mut ft = DualFenwickTree2DArbitrary::<G>::new(h, w);

        // [1, 3) x [1, 3) に 5 を加算
        ft.rect_add(1..3, 1..3, 5);
        assert_eq!(ft.get(1, 1), 5i64);
        assert_eq!(ft.get(1, 2), 5i64);
        assert_eq!(ft.get(2, 1), 5i64);
        assert_eq!(ft.get(2, 2), 5i64);
        assert_eq!(ft.get(0, 0), 0i64);
        assert_eq!(ft.get(3, 3), 0i64);

        // [0, 2) x [0, 2) に 10 を加算
        ft.rect_add(0..2, 0..2, 10);
        assert_eq!(ft.get(0, 0), 10i64);
        assert_eq!(ft.get(1, 1), 15i64);
        assert_eq!(ft.get(2, 2), 5i64);
    }

    #[test]
    #[ignore]
    fn test_random_dual_fenwick_tree_2d() {
        type G = AdditiveAbGroup<i64>;
        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..20 {
            let h = rng.random_range(1..=10);
            let w = rng.random_range(1..=10);
            let mut naive = vec![vec![0i64; w]; h];
            let mut ft = DualFenwickTree2DArbitrary::<G>::new(h, w);

            for _ in 0..50 {
                let op = rng.random_range(0..2);
                if op == 0 {
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

                    let y = rng.random_range(0..h);
                    let x = rng.random_range(0..w);
                    assert_eq!(ft.get(y, x), naive[y][x]);
                } else {
                    let y = rng.random_range(0..h);
                    let x = rng.random_range(0..w);
                    assert_eq!(ft.get(y, x), naive[y][x]);
                }
            }
            assert_eq!(ft.to_vec(), naive);
        }
    }
}
