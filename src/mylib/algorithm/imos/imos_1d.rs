use cargo_snippet::snippet;

#[snippet(prefix = "use imos_1d::*;")]
#[allow(clippy::module_inception)]
/// 1次元いもす法を扱うモジュール
///
/// # Examples
/// ```
/// use mylib::imos::imos_1d::imos_1d::*;
///
/// // [0, 5) の範囲で考える。1回差分を取るので end は 1つ余分に確保する
/// let mut imos = Imos1D::new(0, 5 + 1);
///
/// // [0, 2) に 1 を加算
/// imos.add(0, 1);
/// imos.add(2, -1);
///
/// // [3, 5) に 2 を加算
/// imos.add(3, 2);
/// imos.add(5, -2);
///
/// imos.summation();
///
/// assert_eq!(imos.get(0), 1);
/// assert_eq!(imos.get(1), 1);
/// assert_eq!(imos.get(2), 0);
/// assert_eq!(imos.get(3), 2);
/// assert_eq!(imos.get(4), 2);
/// ```
pub mod imos_1d {

    /// 1次元いもす法のための構造体
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Imos1D {
        raw: Vec<i64>,
        begin: i64,
        end: i64,
    }

    impl Imos1D {
        /// `[begin, end)` の半開区間を対象とする `Imos1D` を生成する。
        ///
        /// `summation` を複数回適用する場合（例：アフィン変換や二次関数）、
        /// その回数分だけ `end` を広めに確保する必要がある。
        ///
        /// # Panics
        /// `begin >= end` の場合、デバッグビルドではパニックする。
        pub fn new(begin: i64, end: i64) -> Self {
            debug_assert!(begin < end);
            let len = (end - begin) as usize;
            let raw = vec![0; len];
            Self { raw, begin, end }
        }

        fn is_within(&self, i: i64) -> bool {
            (self.begin..self.end).contains(&i)
        }

        /// `i` 番目の要素の値を取得する。
        ///
        /// `summation` 実行前は差分が、実行後は累積和が返される。
        ///
        /// # Panics
        /// `i` が `[begin, end)` の範囲外の場合、デバッグビルドではパニックする。
        pub fn get(&self, i: i64) -> i64 {
            if cfg!(debug_assertions) && !self.is_within(i) {
                panic!(
                    "index out of bounds: the range is [{}, {}) but the index is {}",
                    self.begin, self.end, i
                );
            }
            self.raw[(i - self.begin) as usize]
        }

        /// `i` 番目の要素に `val` を加算する。
        ///
        /// 区間 `[l, r)` に値を加算したい場合は、`imos.add(l, val)` と `imos.add(r, -val)` を呼び出す。
        ///
        /// # Panics
        /// `i` が `[begin, end)` の範囲外の場合、デバッグビルドではパニックする。
        pub fn add(&mut self, i: i64, val: i64) {
            if cfg!(debug_assertions) && !self.is_within(i) {
                panic!(
                    "index out of bounds: the range is [{}, {}) but the index is {}",
                    self.begin, self.end, i
                );
            }
            self.raw[(i - self.begin) as usize] += val;
        }

        /// 差分の累積和を計算する。
        ///
        /// この操作により、各要素は区間加算を反映した最終的な値を持つようになる。
        pub fn summation(&mut self) {
            for i in 1..self.raw.len() {
                self.raw[i] += self.raw[i - 1]
            }
        }

        /// 累積和から差分を計算する（デバッグ用）。
        pub fn difference(&mut self) {
            for i in (1..self.raw.len()).rev() {
                self.raw[i] -= self.raw[i - 1];
            }
        }
    }
}
#[cfg(test)]
mod test_imos_1d {

    use itertools::Itertools;

    use super::imos_1d::*;
    #[test]
    fn test_imos_1d_const_func() {
        // [-2, 5) の範囲で考える。1回差分を取るので end は 1つ余分に確保する
        let mut imos = Imos1D::new(-2, 5 + 1);

        // [-2, 0) で 1 を足す
        imos.add(-2, 1);
        imos.add(0, -1);

        // [3, 5) で 2 を足す
        imos.add(3, 2);
        imos.add(5, -2);

        // [1, 3) で 4 を足す
        imos.add(1, 4);
        imos.add(3, -4);

        imos.summation();

        // 1 1 0 0 0 0 0
        // 0 0 0 0 0 2 2
        // 0 0 0 4 4 0 0
        // -------------
        // 1 1 0 4 4 2 2

        let actual = (-2..5).map(|x| imos.get(x)).collect_vec();
        let expected = vec![1, 1, 0, 4, 4, 2, 2];
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_imos_1d_affine() {
        let mut imos = Imos1D::new(0, 9 + 2); // 差分の回数だけ広めに範囲を取っておく。

        // [3, 7) で x+3 を足す
        let f = |x: i64| x + 3;
        imos.add(3, f(3));
        imos.add(4, f(4) - 2 * f(3));
        imos.add(7, -f(7));
        imos.add(8, -(f(8) - 2 * f(7)));

        // [5, 9) で 2x+1 を足す
        let g = |x: i64| 2 * x + 1;
        imos.add(5, g(5));
        imos.add(6, g(6) - 2 * g(5));
        imos.add(9, -g(9));
        imos.add(10, -(g(10) - 2 * g(9)));

        imos.summation();
        imos.summation();

        let actual = (0..11).map(|x| imos.get(x)).collect_vec();
        // 0 0 0 6 7  8  9  0  0 0 0
        // 0 0 0 0 0 11 13 15 17 0 0
        // -------------------------
        // 0 0 0 6 7 19 22 15 17 0 0
        let expected = vec![0, 0, 0, 6, 7, 19, 22, 15, 17, 0, 0];
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_imos_1d_quadratic() {
        let mut imos = Imos1D::new(0, 5 + 3); // 差分の回数だけ広めに範囲を取っておく。

        // [1, 5) で x^2 を足す
        let f = |x: i64| if x >= 1 { x * x } else { 0 };
        for t in 0..3 {
            let begin = 1;
            imos.add(
                begin + t,
                f(begin + t) - 3 * f(begin + t - 1) + 3 * f(begin + t - 2),
            )
        }
        let f = |x: i64| if x >= 5 { -x * x } else { 0 };
        for t in 0..3 {
            let end = 5;
            imos.add(
                end + t,
                f(end + t) - 3 * f(end + t - 1) + 3 * f(end + t - 2),
            )
        }

        imos.summation();
        imos.summation();
        imos.summation();

        let actual = (0..5).map(|x| imos.get(x)).collect_vec();

        let expected = vec![0, 1, 4, 9, 16];
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_imos_1d_difference() {
        let mut imos = Imos1D::new(0, 5);
        for i in 0..5 {
            imos.add(i, i * i);
        }
        imos.difference();
        // 0 1 4 9 16
        // ↓ 差分
        // 0 1 3 5 7

        let actual = (0..5).map(|x| imos.get(x)).collect_vec();

        let expected = vec![0, 1, 3, 5, 7];
        assert_eq!(actual, expected)
    }
}
