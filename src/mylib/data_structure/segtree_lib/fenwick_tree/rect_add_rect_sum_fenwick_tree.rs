use crate::data_structure::segtree_lib::fenwick_tree::rect_sum_fenwick_tree_2d::rect_sum_fenwick_tree_2d::RectSumFenwickTree2D;
use cargo_snippet::snippet;

#[snippet(
    prefix = "use rect_add_rect_sum_fenwick_tree::*;",
    include = "rect_sum_fenwick_tree_2d"
)]
#[allow(clippy::module_inception)]
pub mod rect_add_rect_sum_fenwick_tree {
    use super::RectSumFenwickTree2D;
    use std::iter::Sum;
    use std::ops::{Add, Bound, Mul, Neg, RangeBounds, Sub};

    fn zero<T: Sum>() -> T {
        std::iter::empty::<T>().sum()
    }

    /// 任意の数値型 T に対して矩形加算・矩形和取得が可能な 2次元 Fenwick Tree (Rect Add Rect Sum Fenwick Tree 2D)。
    //
    // [原理]
    // 1次元の Range Add Range Sum の拡張。
    // A[y][x] の 2次元階差を D[y][x] とすると、
    // A[y][x] = Σ_{i=0}^y Σ_{j=0}^x D[i][j]
    // 累積和 S(y, x) = Σ_{i=0}^{y-1} Σ_{j=0}^{x-1} A[i][j] は以下のように変形できる：
    // S(y, x) = Σ_{i=0}^{y-1} Σ_{j=0}^{x-1} (y - i)(x - j) D[i][j]
    //         = y*x*ΣD[i][j] - y*Σ(j*D[i][j]) - x*Σ(i*D[i][j]) + Σ(i*j*D[i][j])
    //
    // よって 4 つの 2次元 BIT で以下の値を管理すればよい：
    // bit00: Σ D[i][j]
    // bit01: Σ j * D[i][j]
    // bit10: Σ i * D[i][j]
    // bit11: Σ i * j * D[i][j]
    #[derive(Clone)]
    pub struct RectAddRectSumFenwickTree<T>
    where
        T: Copy + Add<Output = T> + Sub<Output = T> + Mul<i64, Output = T> + Neg<Output = T> + Sum,
    {
        h: usize,
        w: usize,
        bit00: RectSumFenwickTree2D<T>,
        bit01: RectSumFenwickTree2D<T>,
        bit10: RectSumFenwickTree2D<T>,
        bit11: RectSumFenwickTree2D<T>,
    }

    /// i64 の加算群を用いた標準的な 2次元矩形加算・矩形和 Fenwick Tree のエイリアス。
    pub type RectAddRectSumFenwickTreeI64 = RectAddRectSumFenwickTree<i64>;

    impl<T> RectAddRectSumFenwickTree<T>
    where
        T: Copy + Add<Output = T> + Sub<Output = T> + Mul<i64, Output = T> + Neg<Output = T> + Sum,
    {
        /// H × W の 2次元矩形加算・矩形和 Fenwick Tree を作成します。
        ///
        /// # 計算量
        /// O(H * W)
        pub fn new(h: usize, w: usize) -> Self {
            Self {
                h,
                w,
                bit00: RectSumFenwickTree2D::new(h + 1, w + 1),
                bit01: RectSumFenwickTree2D::new(h + 1, w + 1),
                bit10: RectSumFenwickTree2D::new(h + 1, w + 1),
                bit11: RectSumFenwickTree2D::new(h + 1, w + 1),
            }
        }

        /// 配列の 2次元スライスから Rect Add Rect Sum Fenwick Tree を作成します。
        ///
        /// # 計算量
        /// O(H * W)
        pub fn from_slice(slice: &[Vec<T>]) -> Self {
            let h = slice.len();
            let w = if h == 0 { 0 } else { slice[0].len() };
            let zero_val: T = zero();
            let mut d = vec![vec![zero_val; w + 1]; h + 1];
            let mut dx = vec![vec![zero_val; w + 1]; h + 1];
            let mut dy = vec![vec![zero_val; w + 1]; h + 1];
            let mut dxy = vec![vec![zero_val; w + 1]; h + 1];

            for i in 0..=h {
                for j in 0..=w {
                    let get_a = |y: isize, x: isize| -> T {
                        if y >= 0 && (y as usize) < h && x >= 0 && (x as usize) < w {
                            slice[y as usize][x as usize]
                        } else {
                            zero()
                        }
                    };

                    let val = get_a(i as isize, j as isize)
                        - get_a(i as isize - 1, j as isize)
                        - get_a(i as isize, j as isize - 1)
                        + get_a(i as isize - 1, j as isize - 1);

                    d[i][j] = val;
                    dx[i][j] = val * (j as i64);
                    dy[i][j] = val * (i as i64);
                    dxy[i][j] = val * (i as i64) * (j as i64);
                }
            }

            Self {
                h,
                w,
                bit00: RectSumFenwickTree2D::from_slice(&d),
                bit01: RectSumFenwickTree2D::from_slice(&dx),
                bit10: RectSumFenwickTree2D::from_slice(&dy),
                bit11: RectSumFenwickTree2D::from_slice(&dxy),
            }
        }

        /// 指定された矩形領域 `y_range` × `x_range` に `val` を加算します。
        ///
        /// # Panics
        /// 範囲が不正、または領域外の場合にパニックします。
        ///
        /// # 計算量
        /// O(log H * log W)
        pub fn rect_add<Ry, Rx>(&mut self, y_range: Ry, x_range: Rx, val: T)
        where
            Ry: RangeBounds<usize>,
            Rx: RangeBounds<usize>,
        {
            let (y1, y2) = Self::resolve_range(y_range, self.h);
            let (x1, x2) = Self::resolve_range(x_range, self.w);

            assert!(
                y1 <= y2 && y2 <= self.h,
                "RectAddRectSumFenwickTree::rect_add: invalid y range: {}..{}, h={}",
                y1,
                y2,
                self.h
            );
            assert!(
                x1 <= x2 && x2 <= self.w,
                "RectAddRectSumFenwickTree::rect_add: invalid x range: {}..{}, w={}",
                x1,
                x2,
                self.w
            );

            let mut add_internal = |y: usize, x: usize, v: T| {
                if y <= self.h && x <= self.w {
                    self.bit00.add(y, x, v);
                    self.bit01.add(y, x, v * (x as i64));
                    self.bit10.add(y, x, v * (y as i64));
                    self.bit11.add(y, x, v * (y as i64) * (x as i64));
                }
            };

            add_internal(y1, x1, val);
            add_internal(y1, x2, -val);
            add_internal(y2, x1, -val);
            add_internal(y2, x2, val);
        }

        /// `(y, x)` 番目の要素に `val` を加算します。
        ///
        /// # 計算量
        /// O(log H * log W)
        pub fn add(&mut self, y: usize, x: usize, val: T) {
            self.rect_add(y..=y, x..=x, val);
        }

        /// `(y, x)` 番目の要素の値を `val` に設定します。
        ///
        /// # 計算量
        /// O(log H * log W)
        pub fn set(&mut self, y: usize, x: usize, val: T) {
            let old = self.get(y, x);
            self.add(y, x, val - old);
        }

        /// 左上 (0,0) から右下 (y,x) までの矩形和を取得します。
        ///
        /// # 計算量
        /// O(log H * log W)
        pub fn prefix_sum(&self, y: usize, x: usize) -> T {
            let s00 = self.bit00.prefix_sum(y, x);
            let s01 = self.bit01.prefix_sum(y, x);
            let s10 = self.bit10.prefix_sum(y, x);
            let s11 = self.bit11.prefix_sum(y, x);

            // S(y, x) = y*x*s00 - y*s01 - x*s10 + s11
            s00 * (y as i64) * (x as i64) - s01 * (y as i64) - s10 * (x as i64) + s11
        }

        /// 指定された矩形領域の和を計算します。
        ///
        /// # Panics
        /// 範囲が不正、または領域外の場合にパニックします。
        ///
        /// # 計算量
        /// O(log H * log W)
        pub fn rect_sum<Ry, Rx>(&self, y_range: Ry, x_range: Rx) -> T
        where
            Ry: RangeBounds<usize>,
            Rx: RangeBounds<usize>,
        {
            let (y1, y2) = Self::resolve_range(y_range, self.h);
            let (x1, x2) = Self::resolve_range(x_range, self.w);

            assert!(
                y1 <= y2 && y2 <= self.h,
                "RectAddRectSumFenwickTree::rect_sum: invalid y range: {}..{}, h={}",
                y1,
                y2,
                self.h
            );
            assert!(
                x1 <= x2 && x2 <= self.w,
                "RectAddRectSumFenwickTree::rect_sum: invalid x range: {}..{}, w={}",
                x1,
                x2,
                self.w
            );

            self.prefix_sum(y2, x2) - self.prefix_sum(y1, x2) - self.prefix_sum(y2, x1)
                + self.prefix_sum(y1, x1)
        }

        fn resolve_range<R: RangeBounds<usize>>(range: R, n: usize) -> (usize, usize) {
            let l = match range.start_bound() {
                Bound::Included(&l) => l,
                Bound::Excluded(&l) => l + 1,
                Bound::Unbounded => 0,
            };
            let r = match range.end_bound() {
                Bound::Included(&r) => r + 1,
                Bound::Excluded(&r) => r,
                Bound::Unbounded => n,
            };
            (l, r)
        }

        /// `(y, x)` 番目の要素を取得します。
        ///
        /// # 計算量
        /// O(log H * log W)
        pub fn get(&self, y: usize, x: usize) -> T {
            self.rect_sum(y..=y, x..=x)
        }

        /// 現在の状態を `Vec<Vec<T>>` として返します。
        ///
        /// # 計算量
        /// O(H * W * log H * log W)
        pub fn to_vec(&self) -> Vec<Vec<T>> {
            (0..self.h)
                .map(|y| (0..self.w).map(|x| self.get(y, x)).collect())
                .collect()
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
    use super::rect_add_rect_sum_fenwick_tree::*;
    use rand::{Rng, SeedableRng, rngs::SmallRng};

    #[test]
    fn test_rect_add_rect_sum_2d_basic() {
        let (h, w) = (5, 5);
        let mut ft = RectAddRectSumFenwickTree::<i64>::new(h, w);
        assert_eq!(ft.len_h(), 5);
        assert_eq!(ft.len_w(), 5);

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
    fn test_rect_add_rect_sum_2d_from_slice_basic() {
        let vals = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let ft = RectAddRectSumFenwickTree::<i64>::from_slice(&vals);

        assert_eq!(ft.to_vec(), vals);
        // 矩形和の検証
        assert_eq!(ft.rect_sum(0..2, 0..2), 1 + 2 + 4 + 5);
        assert_eq!(ft.rect_sum(1..3, 1..3), 5 + 6 + 8 + 9);
        assert_eq!(ft.rect_sum(0..3, 0..3), 45);
    }

    #[test]
    fn test_modint_compatibility() {
        use ac_library::ModInt998244353 as Mint;
        let (h, w) = (5, 5);
        let mut ft = RectAddRectSumFenwickTree::<Mint>::new(h, w);
        ft.rect_add(1..3, 1..3, Mint::new(5));
        assert_eq!(ft.rect_sum(1..3, 1..3), Mint::new(20));
    }

    #[test]
    #[ignore]
    fn test_random_rect_add_rect_sum_2d() {
        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..20 {
            let h = rng.random_range(1..=10);
            let w = rng.random_range(1..=10);
            let mut naive = vec![vec![0i64; w]; h];
            let mut ft = RectAddRectSumFenwickTree::<i64>::new(h, w);

            for _ in 0..50 {
                let op = rng.random_range(0..4);
                match op {
                    0 => {
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
                    }
                    1 => {
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
                    2 => {
                        // add
                        let y = rng.random_range(0..h);
                        let x = rng.random_range(0..w);
                        let val = rng.random_range(-100..=100);
                        naive[y][x] += val;
                        ft.add(y, x, val);
                    }
                    3 => {
                        // set
                        let y = rng.random_range(0..h);
                        let x = rng.random_range(0..w);
                        let val = rng.random_range(-100..=100);
                        naive[y][x] = val;
                        ft.set(y, x, val);
                    }
                    _ => unreachable!(),
                }
            }
            assert_eq!(ft.to_vec(), naive);
        }
    }

    #[test]
    #[ignore]
    fn test_random_rect_add_rect_sum_2d_from_slice() {
        let mut rng = SmallRng::seed_from_u64(42);

        for h in 1..=8 {
            for w in 1..=8 {
                let vals: Vec<Vec<i64>> = (0..h)
                    .map(|_| (0..w).map(|_| rng.random_range(-100..=100)).collect())
                    .collect();
                let ft = RectAddRectSumFenwickTree::<i64>::from_slice(&vals);

                assert_eq!(ft.to_vec(), vals, "h={}, w={} failed", h, w);
            }
        }
    }
}
