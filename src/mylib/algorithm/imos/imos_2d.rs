use cargo_snippet::snippet;

#[snippet(prefix = "use imos_2d::*;")]
#[allow(clippy::module_inception)]
/// 2次元いもす法を扱うモジュール
///
/// # Examples
/// ```
/// use mylib::algorithm::imos::imos_2d::imos_2d::*;
///
/// let (h, w) = (4, 5);
/// // 1回差分を取るので、各次元の end を1つ余分に確保する
/// let mut imos = Imos2D::new(0, h + 1, 0, w + 1);
///
/// // 矩形領域 [1, 3) x [2, 4) に 1 を加算
/// imos.add(1, 2, 1);
/// imos.add(1, 4, -1);
/// imos.add(3, 2, -1);
/// imos.add(3, 4, 1);
///
/// // x方向、y方向の順に累積和を計算
/// imos.summation(0, 1);
/// imos.summation(1, 0);
///
/// assert_eq!(imos.get(1, 2), 1);
/// assert_eq!(imos.get(2, 3), 1);
/// assert_eq!(imos.get(0, 0), 0);
/// ```
pub mod imos_2d {

    /// 2次元いもす法のための構造体
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Imos2D {
        raw: Vec<Vec<i64>>,
        y_begin: i64,
        y_end: i64,
        x_begin: i64,
        x_end: i64,
    }

    impl Imos2D {
        /// `[y_begin, y_end) x [x_begin, x_end)` の矩形領域を対象とする `Imos2D` を生成する。
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
            let raw = vec![vec![0; width]; height];
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
        pub fn get(&self, y: i64, x: i64) -> i64 {
            if cfg!(debug_assertions) && !self.is_within(y, x) {
                panic!(
                    "index out of bounds: the domain is [{}, {}) × [{}, {}) but the index is ({}, {})",
                    self.y_begin, self.y_end, self.x_begin, self.x_end, y, x
                );
            }
            self.raw[(y - self.y_begin) as usize][(x - self.x_begin) as usize]
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
        pub fn add(&mut self, y: i64, x: i64, val: i64) {
            if cfg!(debug_assertions) && !self.is_within(y, x) {
                panic!(
                    "index out of bounds: the domain is [{}, {}) × [{}, {}) but the index is ({}, {})",
                    self.y_begin, self.y_end, self.x_begin, self.x_end, y, x
                );
            }
            self.raw[(y - self.y_begin) as usize][(x - self.x_begin) as usize] += val;
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
                // 3時から9時の方向 (3時含む、9時含まない)
                for y in 0..height {
                    for x in 0..width {
                        let prev_y = y - d_y;
                        let prev_x = x - d_x;
                        if (0..height).contains(&prev_y) && (0..width).contains(&prev_x) {
                            self.raw[y as usize][x as usize] +=
                                self.raw[prev_y as usize][prev_x as usize]
                        }
                    }
                }
            } else {
                // 9時から3時の方向 (9時含む、3時含まない)
                for y in (0..height).rev() {
                    for x in (0..width).rev() {
                        let prev_y = y - d_y;
                        let prev_x = x - d_x;
                        if (0..height).contains(&prev_y) && (0..width).contains(&prev_x) {
                            self.raw[y as usize][x as usize] +=
                                self.raw[prev_y as usize][prev_x as usize]
                        }
                    }
                }
            }
        }

        /// 別の Imos2D オブジェクトと要素ごとに足し合わせる。
        ///
        /// 2つの Imos2D オブジェクトの領域が異なる場合は、領域の和集合を囲う最小の長方形を新しい領域とする。
        pub fn add_imos(&self, other: &Self) -> Self {
            use std::cmp::{max, min};

            // 新しい領域を計算
            let new_y_begin = min(self.y_begin, other.y_begin);
            let new_y_end = max(self.y_end, other.y_end);
            let new_x_begin = min(self.x_begin, other.x_begin);
            let new_x_end = max(self.x_end, other.x_end);

            // 新しい領域で Imos2D を生成
            let mut result = Self::new(new_y_begin, new_y_end, new_x_begin, new_x_end);

            // 1つ目の Imos2D の値を新しい領域にコピー
            for y in 0..self.raw.len() {
                for x in 0..self.raw[y].len() {
                    if self.raw[y][x] != 0 {
                        let original_y = self.y_begin + y as i64;
                        let original_x = self.x_begin + x as i64;
                        result.add(original_y, original_x, self.raw[y][x]);
                    }
                }
            }

            // 2つ目の Imos2D の値を新しい領域にコピー
            for y in 0..other.raw.len() {
                for x in 0..other.raw[y].len() {
                    if other.raw[y][x] != 0 {
                        let original_y = other.y_begin + y as i64;
                        let original_x = other.x_begin + x as i64;
                        result.add(original_y, original_x, other.raw[y][x]);
                    }
                }
            }
            result
        }
    }
}

#[cfg(test)]
mod test_imos_2d {

    use itertools::Itertools;

    use super::imos_2d::*;
    #[test]
    fn test_imos_2d_rect_const_func() {
        let height = 4;
        let width = 5;
        // 1回差分を取るので end は 1つ余分に確保する
        let mut imos = Imos2D::new(0, height + 1, 0, width + 1);

        // [1,3) × [2,4) の領域で 1 を足す

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
    fn test_imos_2d_triangle_const_func() {
        let height = 5;
        let width = 5;
        // 差分の数だけ余分に確保する
        let mut imos = Imos2D::new(0, height + 2, 0, width + 2);

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
    fn test_imos_2d_chebyshev() {
        let height = 5;
        let width = 5;
        // 差分の数だけ余分に確保する
        let mut imos = Imos2D::new(0, height + 3, -1, width + 2);

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
    fn test_imos_2d_line() {
        let height = 5;
        let width = 5;
        // 差分の数だけ余分に確保する
        let mut imos = Imos2D::new(-1, height, 0, width + 1);

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
    fn test_imos_2d_add_imos_same_regions() {
        let height = 3;
        let width = 3;

        let mut imos1 = Imos2D::new(0, height + 1, 0, width + 1);
        imos1.add(0, 0, 1); // Add 1 at (0,0)
        imos1.add(0, 1, -1);
        imos1.add(1, 0, -1);
        imos1.add(1, 1, 1);

        let mut imos2 = Imos2D::new(0, height + 1, 0, width + 1);
        imos2.add(1, 1, 2); // Add 2 at (1,1)
        imos2.add(1, 2, -2);
        imos2.add(2, 1, -2);
        imos2.add(2, 2, 2);

        let mut imos_sum = imos1.add_imos(&imos2);
        imos_sum.summation(0, 1);
        imos_sum.summation(1, 0);

        let actual = (0..height)
            .map(|y| (0..width).map(|x| imos_sum.get(y, x)).collect_vec())
            .collect_vec();

        // Expected result:
        // imos1:
        // 1 0 0
        // 0 0 0
        // 0 0 0
        //
        // imos2:
        // 0 0 0
        // 0 2 0
        // 0 0 0
        //
        // imos_sum (after summation):
        // 1 0 0
        // 0 2 0
        // 0 0 0
        let expected = vec![vec![1, 0, 0], vec![0, 2, 0], vec![0, 0, 0]];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_imos_2d_add_imos_different_regions() {
        // imos1: [0, 3) x [0, 3), value 1 in [1, 2) x [1, 2)
        let mut imos1 = Imos2D::new(0, 3, 0, 3);
        imos1.add(1, 1, 1);
        imos1.add(1, 2, -1);
        imos1.add(2, 1, -1);
        imos1.add(2, 2, 1);

        // imos2: [2, 5) x [2, 5), value 2 in [3, 4) x [3, 4)
        let mut imos2 = Imos2D::new(2, 5, 2, 5);
        imos2.add(3, 3, 2);
        imos2.add(3, 4, -2);
        imos2.add(4, 3, -2);
        imos2.add(4, 4, 2);

        // The combined region should be [0, 5) x [0, 5)
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
}
