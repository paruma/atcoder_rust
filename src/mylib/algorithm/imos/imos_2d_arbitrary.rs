use crate::math::algebra::ab_group::ab_group::AbGroup;
use cargo_snippet::snippet;

#[snippet(prefix = "use imos_2d_arbitrary::*;", include = "ab_group")]
#[allow(clippy::module_inception)]
/// 可換群 (AbGroup) を用いた汎用的な 2次元いもす法を扱うモジュール
pub mod imos_2d_arbitrary {
    use super::AbGroup;

    /// 2次元いもす法のための構造体 (汎用版)
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Imos2dArbitrary<G: AbGroup> {
        raw: Vec<Vec<G::S>>,
        y_begin: i64,
        y_end: i64,
        x_begin: i64,
        x_end: i64,
    }

    impl<G: AbGroup> Imos2dArbitrary<G> {
        /// `[y_begin, y_end) x [x_begin, x_end)` の矩形領域を対象とする `Imos2dArbitrary` を生成する。
        ///
        /// `summation` を適用する方向や回数に応じて、`end` を広めに確保する必要がある。
        ///
        /// # Panics
        /// `begin >= end` となる次元がある場合、デバッグビルドではパニックする。
        pub fn new(y_begin: i64, y_end: i64, x_begin: i64, x_end: i64) -> Self {
            debug_assert!(y_begin < y_end);
            debug_assert!(x_begin < x_end);
            let height = (y_end - y_begin) as usize;
            let width = (x_end - x_begin) as usize;
            let mut raw = Vec::with_capacity(height);
            for _ in 0..height {
                let mut row = Vec::with_capacity(width);
                for _ in 0..width {
                    row.push(G::zero());
                }
                raw.push(row);
            }
            Self {
                raw,
                y_begin,
                y_end,
                x_begin,
                x_end,
            }
        }

        fn is_within(&self, y: i64, x: i64) -> bool {
            (self.y_begin..self.y_end).contains(&y) && (self.x_begin..self.x_end).contains(&x)
        }

        /// `(y, x)` の要素の値を取得する。
        ///
        /// `summation` 実行前は差分が、実行後は累積和が返される。
        ///
        /// # Panics
        /// `(y, x)` が範囲外の場合、デバッグビルドではパニックする。
        pub fn get(&self, y: i64, x: i64) -> G::S {
            if cfg!(debug_assertions) && !self.is_within(y, x) {
                panic!(
                    "index out of bounds: the domain is [{}, {}) × [{}, {}) but the index is ({}, {})",
                    self.y_begin, self.y_end, self.x_begin, self.x_end, y, x
                );
            }
            self.raw[(y - self.y_begin) as usize][(x - self.x_begin) as usize].clone()
        }

        pub fn y_begin(&self) -> i64 {
            self.y_begin
        }

        pub fn y_end(&self) -> i64 {
            self.y_end
        }

        pub fn x_begin(&self) -> i64 {
            self.x_begin
        }

        pub fn x_end(&self) -> i64 {
            self.x_end
        }

        /// `(y, x)` の要素に `val` を加算する。
        ///
        /// 矩形領域 `[y1, y2) x [x1, x2)` に値を加算するには、4点の `add` を呼び出す。
        ///
        /// # Panics
        /// `(y, x)` が範囲外の場合、デバッグビルドではパニックする。
        pub fn add(&mut self, y: i64, x: i64, val: G::S) {
            if cfg!(debug_assertions) && !self.is_within(y, x) {
                panic!(
                    "index out of bounds: the domain is [{}, {}) × [{}, {}) but the index is ({}, {})",
                    self.y_begin, self.y_end, self.x_begin, self.x_end, y, x
                );
            }
            let row = (y - self.y_begin) as usize;
            let col = (x - self.x_begin) as usize;
            self.raw[row][col] = G::add(&self.raw[row][col], &val);
        }

        /// `(d_y, d_x)` 方向の差分の累積和を計算する。
        ///
        /// - `(1, 0)`: y方向の累積和
        /// - `(0, 1)`: x方向の累積和
        /// - `(1, 1)`: 右下方向の累積和
        ///
        /// # Panics
        /// `(d_y, d_x) == (0, 0)` の場合、デバッグビルドではパニックする。
        pub fn summation(&mut self, d_y: i64, d_x: i64) {
            debug_assert_ne!((d_y, d_x), (0, 0));
            let height = self.y_end - self.y_begin;
            let width = self.x_end - self.x_begin;
            if d_y > 0 || (d_y == 0 && d_x > 0) {
                for y in 0..height {
                    for x in 0..width {
                        let prev_y = y - d_y;
                        let prev_x = x - d_x;
                        if (0..height).contains(&prev_y) && (0..width).contains(&prev_x) {
                            self.raw[y as usize][x as usize] = G::add(
                                &self.raw[prev_y as usize][prev_x as usize],
                                &self.raw[y as usize][x as usize],
                            );
                        }
                    }
                }
            } else {
                for y in (0..height).rev() {
                    for x in (0..width).rev() {
                        let prev_y = y - d_y;
                        let prev_x = x - d_x;
                        if (0..height).contains(&prev_y) && (0..width).contains(&prev_x) {
                            self.raw[y as usize][x as usize] = G::add(
                                &self.raw[prev_y as usize][prev_x as usize],
                                &self.raw[y as usize][x as usize],
                            );
                        }
                    }
                }
            }
        }

        /// 別の Imos2dArbitrary オブジェクトと要素ごとに足し合わせる。
        ///
        /// 2つの Imos2dArbitrary オブジェクトの領域が異なる場合は、領域の和集合を囲う最小の長方形を新しい領域とする。
        pub fn add_imos(&self, other: &Self) -> Self {
            use std::cmp::{max, min};

            let new_y_begin = min(self.y_begin, other.y_begin);
            let new_y_end = max(self.y_end, other.y_end);
            let new_x_begin = min(self.x_begin, other.x_begin);
            let new_x_end = max(self.x_end, other.x_end);

            let mut result = Self::new(new_y_begin, new_y_end, new_x_begin, new_x_end);

            for y in 0..self.raw.len() {
                for x in 0..self.raw[y].len() {
                    let original_y = self.y_begin + y as i64;
                    let original_x = self.x_begin + x as i64;
                    result.add(original_y, original_x, self.raw[y][x].clone());
                }
            }

            for y in 0..other.raw.len() {
                for x in 0..other.raw[y].len() {
                    let original_y = other.y_begin + y as i64;
                    let original_x = other.x_begin + x as i64;
                    result.add(original_y, original_x, other.raw[y][x].clone());
                }
            }
            result
        }
    }
}

#[cfg(test)]
mod tests_imos_2d_arbitrary {
    use super::imos_2d_arbitrary::*;
    use crate::math::algebra::ab_group::ab_group::AdditiveAbGroup;
    use itertools::Itertools;

    #[test]
    fn test_imos_2d_arbitrary_rect_const_func() {
        type G = AdditiveAbGroup<i64>;
        let height = 4;
        let width = 5;
        let mut imos = Imos2dArbitrary::<G>::new(0, height + 1, 0, width + 1);

        imos.add(1, 2, 1);
        imos.add(1, 4, -1);
        imos.add(3, 2, -1);
        imos.add(3, 4, 1);

        imos.summation(0, 1);
        imos.summation(1, 0);
        let actual = (0..height)
            .map(|y| (0..width).map(|x| imos.get(y, x)).collect_vec())
            .collect_vec();
        let expected = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 1, 1, 0],
            vec![0, 0, 1, 1, 0],
            vec![0, 0, 0, 0, 0],
        ];
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_imos_2d_arbitrary_triangle_const_func() {
        type G = AdditiveAbGroup<i64>;
        let height = 5;
        let width = 5;
        let mut imos = Imos2dArbitrary::<G>::new(0, height + 2, 0, width + 2);

        imos.add(0, 0, 1);
        imos.add(0, 1, -1);
        imos.add(5, 0, -1);
        imos.add(6, 1, 1);
        imos.add(5, 6, 1);
        imos.add(6, 6, -1);

        imos.summation(0, 1);
        imos.summation(1, 0);
        imos.summation(1, 1);

        let actual = (0..height)
            .map(|y| (0..width).map(|x| imos.get(y, x)).collect_vec())
            .collect_vec();
        let expected = vec![
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 1, 0, 0],
            vec![1, 1, 1, 1, 0],
            vec![1, 1, 1, 1, 1],
        ];
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_imos_2d_arbitrary_chebyshev() {
        type G = AdditiveAbGroup<i64>;
        let height = 5;
        let width = 5;
        let mut imos = Imos2dArbitrary::<G>::new(0, height + 3, -1, width + 2);

        imos.add(0, 0, 2);
        imos.add(1, -1, -2);
        imos.add(2, 0, 3);
        imos.add(1, 1, -3);

        imos.add(0, 5, -2);
        imos.add(1, 6, 2);
        imos.add(2, 5, -3);
        imos.add(1, 4, 3);

        imos.add(5, 0, -3);
        imos.add(6, -1, 2);
        imos.add(7, 0, -2);
        imos.add(6, 1, 3);

        imos.add(5, 5, 3);
        imos.add(6, 6, -2);
        imos.add(7, 5, 2);
        imos.add(6, 4, -3);

        imos.summation(0, 1);
        imos.summation(1, 0);
        imos.summation(1, 1);
        imos.summation(1, -1);

        let actual = (0..height)
            .map(|y| (0..width).map(|x| imos.get(y, x)).collect_vec())
            .collect_vec();
        let expected = vec![
            vec![2, 2, 2, 2, 2],
            vec![2, 1, 1, 1, 2],
            vec![2, 1, 0, 1, 2],
            vec![2, 1, 1, 1, 2],
            vec![2, 2, 2, 2, 2],
        ];
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_imos_2d_arbitrary_line() {
        type G = AdditiveAbGroup<i64>;
        let height = 5;
        let width = 5;
        let mut imos = Imos2dArbitrary::<G>::new(-1, height, 0, width + 1);

        imos.add(4, 0, 1);
        imos.add(-1, 5, -1);

        imos.summation(-1, 1);

        let actual = (0..height)
            .map(|y| (0..width).map(|x| imos.get(y, x)).collect_vec())
            .collect_vec();
        let expected = vec![
            vec![0, 0, 0, 0, 1],
            vec![0, 0, 0, 1, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
        ];
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_imos_2d_arbitrary_add_imos_same_regions() {
        type G = AdditiveAbGroup<i64>;
        let height = 3;
        let width = 3;

        let mut imos1 = Imos2dArbitrary::<G>::new(0, height + 1, 0, width + 1);
        imos1.add(0, 0, 1);
        imos1.add(0, 1, -1);
        imos1.add(1, 0, -1);
        imos1.add(1, 1, 1);

        let mut imos2 = Imos2dArbitrary::<G>::new(0, height + 1, 0, width + 1);
        imos2.add(1, 1, 2);
        imos2.add(1, 2, -2);
        imos2.add(2, 1, -2);
        imos2.add(2, 2, 2);

        let mut imos_sum = imos1.add_imos(&imos2);
        imos_sum.summation(0, 1);
        imos_sum.summation(1, 0);

        let actual = (0..height)
            .map(|y| (0..width).map(|x| imos_sum.get(y, x)).collect_vec())
            .collect_vec();
        let expected = vec![vec![1, 0, 0], vec![0, 2, 0], vec![0, 0, 0]];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_imos_2d_arbitrary_add_imos_different_regions() {
        type G = AdditiveAbGroup<i64>;
        let mut imos1 = Imos2dArbitrary::<G>::new(0, 3, 0, 3);
        imos1.add(1, 1, 1);
        imos1.add(1, 2, -1);
        imos1.add(2, 1, -1);
        imos1.add(2, 2, 1);

        let mut imos2 = Imos2dArbitrary::<G>::new(2, 5, 2, 5);
        imos2.add(3, 3, 2);
        imos2.add(3, 4, -2);
        imos2.add(4, 3, -2);
        imos2.add(4, 4, 2);

        let mut imos_sum = imos1.add_imos(&imos2);
        imos_sum.summation(0, 1);
        imos_sum.summation(1, 0);

        let height = 5;
        let width = 5;
        let actual = (0..height)
            .map(|y| (0..width).map(|x| imos_sum.get(y, x)).collect_vec())
            .collect_vec();

        let expected = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 2, 0],
            vec![0, 0, 0, 0, 0],
        ];
        assert_eq!(actual, expected);

        assert_eq!(imos_sum.y_begin(), 0);
        assert_eq!(imos_sum.y_end(), 5);
        assert_eq!(imos_sum.x_begin(), 0);
        assert_eq!(imos_sum.x_end(), 5);
    }

    #[test]
    #[should_panic]
    fn test_imos_2d_arbitrary_get_out_of_bounds() {
        type G = AdditiveAbGroup<i64>;
        let imos = Imos2dArbitrary::<G>::new(0, 3, 0, 3);
        imos.get(3, 0);
    }

    #[test]
    #[should_panic]
    fn test_imos_2d_arbitrary_add_out_of_bounds() {
        type G = AdditiveAbGroup<i64>;
        let mut imos = Imos2dArbitrary::<G>::new(0, 3, 0, 3);
        imos.add(0, 3, 1);
    }
}
