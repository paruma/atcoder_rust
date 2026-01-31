use cargo_snippet::snippet;
#[snippet(prefix = "use matrix::*;")]
#[allow(clippy::module_inception)]
pub mod matrix {

    use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Sub, SubAssign};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Matrix<T, const R: usize, const C: usize> {
        pub data: [[T; C]; R],
    }

    use std::iter::{Product, Sum};

    fn t_zero<T>() -> T
    where
        T: Sum,
    {
        std::iter::empty().sum()
    }

    fn t_one<T>() -> T
    where
        T: Product,
    {
        std::iter::empty().product()
    }

    impl<T, const R: usize, const C: usize> Matrix<T, R, C>
    where
        T: Copy + Sum + Product + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        /// 指定された値で埋められた新しい行列を作成します。
        pub fn new(initial_value: T) -> Self {
            Self {
                data: [[initial_value; C]; R],
            }
        }

        /// 配列から行列を作成します。
        pub fn from_array(data: [[T; C]; R]) -> Self {
            Self { data }
        }

        /// スカラ倍 (Matrix * T)
        pub fn scalar_mul(self, rhs: T) -> Self {
            let mut result = Self::new(t_zero());
            for i in 0..R {
                for j in 0..C {
                    result.data[i][j] = self.data[i][j] * rhs;
                }
            }
            result
        }

        /// ベクトルを行列に適用します (行列-ベクトル積)。
        /// `self`はR行C列の行列、`x`はC要素の列ベクトルです。
        /// 結果はR要素の列ベクトルになります。
        pub fn apply(self, x: [T; C]) -> [T; R] {
            let mut result = [t_zero(); R];
            for i in 0..R {
                for j in 0..C {
                    result[i] = result[i] + self.data[i][j] * x[j];
                }
            }
            result
        }
    }

    impl<T, const R: usize, const C: usize> Index<(usize, usize)> for Matrix<T, R, C> {
        type Output = T;
        // index = (row, col)
        fn index(&self, index: (usize, usize)) -> &Self::Output {
            &self.data[index.0][index.1]
        }
    }

    impl<T, const R: usize, const C: usize> IndexMut<(usize, usize)> for Matrix<T, R, C> {
        fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
            &mut self.data[index.0][index.1]
        }
    }

    // 行列の加算
    impl<T, const R: usize, const C: usize> Add for Matrix<T, R, C>
    where
        T: Copy + Sum + Product + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            let mut result = Self::new(t_zero());
            for i in 0..R {
                for j in 0..C {
                    result.data[i][j] = self.data[i][j] + rhs.data[i][j];
                }
            }
            result
        }
    }

    // 行列の加算代入
    impl<T, const R: usize, const C: usize> AddAssign for Matrix<T, R, C>
    where
        T: Copy + Sum + Product + AddAssign + Sub<Output = T> + Mul<Output = T>,
    {
        fn add_assign(&mut self, rhs: Self) {
            for i in 0..R {
                for j in 0..C {
                    self.data[i][j] += rhs.data[i][j];
                }
            }
        }
    }

    // 行列の減算
    impl<T, const R: usize, const C: usize> Sub for Matrix<T, R, C>
    where
        T: Copy + Sum + Product + Sub<Output = T> + Add<Output = T> + Mul<Output = T>,
    {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            let mut result = Self::new(t_zero());
            for i in 0..R {
                for j in 0..C {
                    result.data[i][j] = self.data[i][j] - rhs.data[i][j];
                }
            }
            result
        }
    }

    // 行列の減算代入
    impl<T, const R: usize, const C: usize> SubAssign for Matrix<T, R, C>
    where
        T: Copy + Sum + Product + SubAssign + Add<Output = T> + Mul<Output = T>,
    {
        fn sub_assign(&mut self, rhs: Self) {
            for i in 0..R {
                for j in 0..C {
                    self.data[i][j] -= rhs.data[i][j];
                }
            }
        }
    }

    // 行列の乗算 (Matrix * Matrix)
    impl<T, const R: usize, const C: usize, const K: usize> Mul<Matrix<T, C, K>> for Matrix<T, R, C>
    where
        T: Copy + Sum + Product + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        type Output = Matrix<T, R, K>;
        fn mul(self, rhs: Matrix<T, C, K>) -> Self::Output {
            let mut result = Matrix::<T, R, K>::new(t_zero());
            for i in 0..R {
                for j in 0..K {
                    for l in 0..C {
                        result.data[i][j] = result.data[i][j] + self.data[i][l] * rhs.data[l][j];
                    }
                }
            }
            result
        }
    }

    // 整数倍 (Matrix * i64)
    impl<T, const R: usize, const C: usize> Mul<i64> for Matrix<T, R, C>
    where
        T: Copy
            + Sum
            + Product
            + Mul<i64, Output = T>
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>,
    {
        type Output = Self;
        fn mul(self, rhs: i64) -> Self::Output {
            let mut result = Self::new(t_zero());
            for i in 0..R {
                for j in 0..C {
                    result.data[i][j] = self.data[i][j] * rhs;
                }
            }
            result
        }
    }

    // 行列の累乗 (正方行列のみ)
    impl<T, const N: usize> Matrix<T, N, N>
    where
        T: Copy + Sum + Product + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        pub fn pow(self, mut n: u64) -> Self {
            let mut res = Matrix::<T, N, N>::identity();
            let mut base = self;
            while n > 0 {
                if n % 2 == 1 {
                    res = res * base;
                }
                base = base * base;
                n /= 2;
            }
            res
        }

        /// 単位行列を作成します。正方行列の場合のみ有効です。
        pub fn identity() -> Self {
            let mut matrix = Self::new(t_zero());
            for i in 0..N {
                matrix.data[i][i] = t_one();
            }
            matrix
        }
    }

    impl<T> Matrix<T, 2, 2>
    where
        T: Copy + Sum + Product + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        /// 2x2行列の行列式を計算します。
        pub fn det(self) -> T {
            self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
        }
    }

    impl<T> Matrix<T, 3, 3>
    where
        T: Copy + Sum + Product + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        /// 3x3行列の行列式を計算します。
        pub fn det(self) -> T {
            let a = self.data[0][0];
            let b = self.data[0][1];
            let c = self.data[0][2];
            let d = self.data[1][0];
            let e = self.data[1][1];
            let f = self.data[1][2];
            let g = self.data[2][0];
            let h = self.data[2][1];
            let i = self.data[2][2];

            a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)
        }
    }

    impl<T> Matrix<T, 4, 4>
    where
        T: Copy + Sum + Product + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        /// 4x4行列の行列式を計算します。
        pub fn det(self) -> T {
            let m = self.data;

            let m11 = Matrix::<T, 3, 3>::from_array([
                [m[1][1], m[1][2], m[1][3]],
                [m[2][1], m[2][2], m[2][3]],
                [m[3][1], m[3][2], m[3][3]],
            ]);
            let m12 = Matrix::<T, 3, 3>::from_array([
                [m[1][0], m[1][2], m[1][3]],
                [m[2][0], m[2][2], m[2][3]],
                [m[3][0], m[3][2], m[3][3]],
            ]);
            let m13 = Matrix::<T, 3, 3>::from_array([
                [m[1][0], m[1][1], m[1][3]],
                [m[2][0], m[2][1], m[2][3]],
                [m[3][0], m[3][1], m[3][3]],
            ]);
            let m14 = Matrix::<T, 3, 3>::from_array([
                [m[1][0], m[1][1], m[1][2]],
                [m[2][0], m[2][1], m[2][2]],
                [m[3][0], m[3][1], m[3][2]],
            ]);

            m[0][0] * m11.det() - m[0][1] * m12.det() + m[0][2] * m13.det() - m[0][3] * m14.det()
        }
    }

    pub type Matrix22<T> = Matrix<T, 2, 2>;
    pub type Matrix33<T> = Matrix<T, 3, 3>;
    pub type Matrix44<T> = Matrix<T, 4, 4>;
}
#[cfg(test)]
mod tests {
    use super::matrix::*;

    #[test]
    fn test_new() {
        let m = Matrix::<i32, 2, 3>::new(0);
        assert_eq!(m.data, [[0, 0, 0], [0, 0, 0]]);
    }

    #[test]
    fn test_from_array() {
        let m = Matrix::<i32, 2, 2>::from_array([[1, 2], [3, 4]]);
        assert_eq!(m.data, [[1, 2], [3, 4]]);
    }

    #[test]
    fn test_identity() {
        let m = Matrix::<i32, 3, 3>::identity();
        assert_eq!(m.data, [[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
    }

    #[test]
    fn test_index_access() {
        let mut m = Matrix::<i32, 2, 2>::from_array([[1, 2], [3, 4]]);
        assert_eq!(m[(0, 0)], 1);
        assert_eq!(m[(0, 1)], 2);
        assert_eq!(m[(1, 0)], 3);
        assert_eq!(m[(1, 1)], 4);

        m[(0, 0)] = 10;
        assert_eq!(m[(0, 0)], 10);
    }

    #[test]
    fn test_add() {
        let m1 = Matrix::<i32, 2, 2>::from_array([[1, 2], [3, 4]]);
        let m2 = Matrix::<i32, 2, 2>::from_array([[5, 6], [7, 8]]);
        let m3 = m1 + m2;
        assert_eq!(m3.data, [[6, 8], [10, 12]]);
    }

    #[test]
    fn test_add_assign() {
        let mut m1 = Matrix::<i32, 2, 2>::from_array([[1, 2], [3, 4]]);
        let m2 = Matrix::<i32, 2, 2>::from_array([[5, 6], [7, 8]]);
        m1 += m2;
        assert_eq!(m1.data, [[6, 8], [10, 12]]);
    }

    #[test]
    fn test_sub() {
        let m1 = Matrix::<i32, 2, 2>::from_array([[5, 6], [7, 8]]);
        let m2 = Matrix::<i32, 2, 2>::from_array([[1, 2], [3, 4]]);
        let m3 = m1 - m2;
        assert_eq!(m3.data, [[4, 4], [4, 4]]);
    }

    #[test]
    fn test_sub_assign() {
        let mut m1 = Matrix::<i32, 2, 2>::from_array([[5, 6], [7, 8]]);
        let m2 = Matrix::<i32, 2, 2>::from_array([[1, 2], [3, 4]]);
        m1 -= m2;
        assert_eq!(m1.data, [[4, 4], [4, 4]]);
    }

    #[test]
    fn test_mul_matrix() {
        let m1 = Matrix::<i32, 2, 2>::from_array([[1, 2], [3, 4]]);
        let m2 = Matrix::<i32, 2, 2>::from_array([[5, 6], [7, 8]]);
        // |1 2|   |5 6|   |1*5+2*7  1*6+2*8|   |5+14 6+16|   |19 22|
        // |3 4| * |7 8| = |3*5+4*7  3*6+4*8| = |15+28 18+32| = |43 50|
        let m3 = m1 * m2;
        assert_eq!(m3.data, [[19, 22], [43, 50]]);

        let m4 = Matrix::<i32, 2, 3>::from_array([[1, 2, 3], [4, 5, 6]]);
        let m5 = Matrix::<i32, 3, 2>::from_array([[7, 8], [9, 10], [11, 12]]);
        // |1 2 3|   |7  8 |   |1*7+2*9+3*11  1*8+2*10+3*12|   |7+18+33  8+20+36|   |58  64|
        // |4 5 6| * |9  10| = |4*7+5*9+6*11  4*8+5*10+6*12| = |28+45+66 32+50+72| = |139 154|
        //           |11 12|
        let m6 = m4 * m5;
        assert_eq!(m6.data, [[58, 64], [139, 154]]);
    }

    #[test]
    fn test_mul_integer_matrix() {
        let m = Matrix::<i64, 2, 2>::from_array([[1, 2], [3, 4]]);
        let m_scaled = m * 2i64;
        assert_eq!(m_scaled.data, [[2, 4], [6, 8]]);
    }

    #[test]
    fn test_scalar_mul() {
        let m = Matrix::<i32, 2, 2>::from_array([[1, 2], [3, 4]]);
        let m_scaled = m.scalar_mul(2);
        assert_eq!(m_scaled.data, [[2, 4], [6, 8]]);
    }

    #[test]
    fn test_mul_scalar_mint_matrix() {
        use ac_library::ModInt998244353 as Mint;
        let m = Matrix::<Mint, 2, 2>::from_array([
            [Mint::new(1), Mint::new(2)],
            [Mint::new(3), Mint::new(4)],
        ]);
        let m_scaled = m * 2i64;
        assert_eq!(
            m_scaled.data,
            [[Mint::new(2), Mint::new(4)], [Mint::new(6), Mint::new(8)]]
        );
    }

    #[test]
    fn test_apply() {
        let m = Matrix::<i32, 2, 3>::from_array([[1, 2, 3], [4, 5, 6]]);
        let x = [7, 8, 9];
        let result = m.apply(x);
        assert_eq!(result, [50, 122]);
    }

    #[test]
    fn test_pow() {
        let m = Matrix::<i32, 2, 2>::from_array([[1, 1], [1, 0]]); // Fibonacci matrix
        let m_pow_2 = m.pow(2);
        assert_eq!(m_pow_2.data, [[2, 1], [1, 1]]); // F(2) = 1, F(1) = 1, F(0) = 0. (1,1) (1,0) -> (2,1) (1,1)

        let m_pow_3 = m.pow(3);
        assert_eq!(m_pow_3.data, [[3, 2], [2, 1]]);

        let m_pow_5 = m.pow(5);
        assert_eq!(m_pow_5.data, [[8, 5], [5, 3]]); // F(5)=5, F(6)=8
    }

    #[test]
    fn test_pow_identity() {
        let m = Matrix::<i32, 2, 2>::identity();
        let m_pow_10 = m.pow(10);
        assert_eq!(m_pow_10.data, [[1, 0], [0, 1]]);
    }

    #[test]
    fn test_pow_zero() {
        let m = Matrix::<i32, 2, 2>::from_array([[1, 2], [3, 4]]);
        let m_pow_0 = m.pow(0);
        assert_eq!(m_pow_0.data, [[1, 0], [0, 1]]); // Any matrix to the power of 0 is identity
    }

    #[test]
    fn test_modint_matrix() {
        use ac_library::ModInt998244353 as Mint;
        let m = Matrix::<Mint, 3, 3>::identity();
        assert_eq!(
            m.data,
            [
                [Mint::new(1), Mint::new(0), Mint::new(0)],
                [Mint::new(0), Mint::new(1), Mint::new(0)],
                [Mint::new(0), Mint::new(0), Mint::new(1)]
            ]
        );
    }

    #[test]
    fn test_matrix22_usage() {
        let m = Matrix22::<i32>::identity();
        assert_eq!(m.data, [[1, 0], [0, 1]]);

        let m2 = Matrix22::from_array([[1, 2], [3, 4]]);
        assert_eq!(m2.data, [[1, 2], [3, 4]]);
    }

    #[test]
    fn test_det_2x2() {
        let m = Matrix22::<i32>::from_array([[1, 2], [3, 4]]);
        assert_eq!(m.det(), -2);

        let m2 = Matrix22::<i32>::from_array([[10, 5], [2, 3]]);
        assert_eq!(m2.det(), 20);
    }

    #[test]
    fn test_det_3x3() {
        let m = Matrix33::<i32>::from_array([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        assert_eq!(m.det(), 0);

        let m2 = Matrix33::<i32>::from_array([[3, 0, 2], [2, 0, -2], [0, 1, 1]]);
        assert_eq!(m2.det(), 10);
    }

    #[test]
    fn test_det_4x4() {
        let m = Matrix44::<i32>::from_array([
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ]);
        assert_eq!(m.det(), 0);

        let m2 =
            Matrix44::<i32>::from_array([[3, 2, 0, 1], [4, 0, 1, 2], [3, 0, 2, 1], [9, 2, 3, 1]]);
        assert_eq!(m2.det(), 24);
    }

    #[test]
    #[ignore]
    fn test_random_det() {
        use rand::Rng;
        let mut rng = rand::rng();

        // Naive determinant calculation using permutations for testing
        fn naive_det<const N: usize>(m: &Matrix<i32, N, N>) -> i32 {
            use itertools::Itertools;

            let indices: Vec<usize> = (0..N).collect();
            let mut det = 0;

            for p in indices.into_iter().permutations(N) {
                let mut term = 1;
                for i in 0..N {
                    term *= m.data[i][p[i]];
                }

                let mut inversions = 0;
                for i in 0..N {
                    for j in i + 1..N {
                        if p[i] > p[j] {
                            inversions += 1;
                        }
                    }
                }

                if inversions % 2 == 1 {
                    det -= term;
                } else {
                    det += term;
                }
            }
            det
        }

        // Test for 2x2
        for _ in 0..100 {
            let data = [
                [rng.random_range(-10..=10), rng.random_range(-10..=10)],
                [rng.random_range(-10..=10), rng.random_range(-10..=10)],
            ];
            let m = Matrix22::<i32>::from_array(data);
            assert_eq!(m.det(), naive_det(&m));
        }

        // Test for 3x3
        for _ in 0..100 {
            let data = [
                [
                    rng.random_range(-5..=5),
                    rng.random_range(-5..=5),
                    rng.random_range(-5..=5),
                ],
                [
                    rng.random_range(-5..=5),
                    rng.random_range(-5..=5),
                    rng.random_range(-5..=5),
                ],
                [
                    rng.random_range(-5..=5),
                    rng.random_range(-5..=5),
                    rng.random_range(-5..=5),
                ],
            ];
            let m = Matrix33::<i32>::from_array(data);
            assert_eq!(m.det(), naive_det(&m));
        }

        // Test for 4x4
        for _ in 0..20 {
            // Fewer iterations because naive is slow
            let data = [
                [
                    rng.random_range(-3..=3),
                    rng.random_range(-3..=3),
                    rng.random_range(-3..=3),
                    rng.random_range(-3..=3),
                ],
                [
                    rng.random_range(-3..=3),
                    rng.random_range(-3..=3),
                    rng.random_range(-3..=3),
                    rng.random_range(-3..=3),
                ],
                [
                    rng.random_range(-3..=3),
                    rng.random_range(-3..=3),
                    rng.random_range(-3..=3),
                    rng.random_range(-3..=3),
                ],
                [
                    rng.random_range(-3..=3),
                    rng.random_range(-3..=3),
                    rng.random_range(-3..=3),
                    rng.random_range(-3..=3),
                ],
            ];
            let m = Matrix44::<i32>::from_array(data);
            assert_eq!(m.det(), naive_det(&m));
        }
    }
}
