use cargo_snippet::snippet;

#[snippet(prefix = "use dynamic_matrix::*;")]
pub mod dynamic_matrix {
    use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Sub, SubAssign};
    use std::iter::{Product, Sum};

    // Helper functions for zero and one, similar to matrix.rs
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

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct DynamicMatrix<T> {
        pub rows: usize,
        pub cols: usize,
        pub data: Vec<Vec<T>>,
    }

    impl<T> DynamicMatrix<T>
    where
        T: Copy + Sum + Product + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        /// 指定されたサイズと初期値で新しい動的行列を作成します。
        pub fn new(rows: usize, cols: usize, initial_value: T) -> Self {
            let data = vec![vec![initial_value; cols]; rows];
            Self { rows, cols, data }
        }

        /// 単位行列を作成します。正方行列の場合のみ有効です。
        pub fn identity(size: usize) -> Self {
            let mut matrix = Self::new(size, size, t_zero());
            for i in 0..size {
                matrix.data[i][i] = t_one();
            }
            matrix
        }

        /// Vec<Vec<T>>から行列を作成します。
        pub fn from_vec(data: Vec<Vec<T>>) -> Self {
            assert!(!data.is_empty(), "Matrix cannot be empty");
            let rows = data.len();
            let cols = data[0].len();
            for row in &data {
                assert_eq!(row.len(), cols, "All rows must have the same number of columns");
            }
            Self { rows, cols, data }
        }
    }

    // Indexing
    impl<T> Index<(usize, usize)> for DynamicMatrix<T> {
        type Output = T;
        fn index(&self, index: (usize, usize)) -> &Self::Output {
            &self.data[index.0][index.1]
        }
    }

    impl<T> IndexMut<(usize, usize)> for DynamicMatrix<T> {
        fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
            &mut self.data[index.0][index.1]
        }
    }

    // Add
    impl<T> Add for DynamicMatrix<T>
    where
        T: Copy + Sum + Product + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            assert_eq!(self.rows, rhs.rows, "Matrices must have the same number of rows for addition.");
            assert_eq!(self.cols, rhs.cols, "Matrices must have the same number of columns for addition.");

            let mut result = Self::new(self.rows, self.cols, t_zero());
            for i in 0..self.rows {
                for j in 0..self.cols {
                    result.data[i][j] = self.data[i][j] + rhs.data[i][j];
                }
            }
            result
        }
    }

    // AddAssign
    impl<T> AddAssign for DynamicMatrix<T>
    where
        T: Copy + Sum + Product + AddAssign + Sub<Output = T> + Mul<Output = T>,
    {
        fn add_assign(&mut self, rhs: Self) {
            assert_eq!(self.rows, rhs.rows, "Matrices must have the same number of rows for addition.");
            assert_eq!(self.cols, rhs.cols, "Matrices must have the same number of columns for addition.");

            for i in 0..self.rows {
                for j in 0..self.cols {
                    self.data[i][j] += rhs.data[i][j];
                }
            }
        }
    }

    // Sub
    impl<T> Sub for DynamicMatrix<T>
    where
        T: Copy + Sum + Product + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            assert_eq!(self.rows, rhs.rows, "Matrices must have the same number of rows for subtraction.");
            assert_eq!(self.cols, rhs.cols, "Matrices must have the same number of columns for subtraction.");

            let mut result = Self::new(self.rows, self.cols, t_zero());
            for i in 0..self.rows {
                for j in 0..self.cols {
                    result.data[i][j] = self.data[i][j] - rhs.data[i][j];
                }
            }
            result
        }
    }

    // SubAssign
    impl<T> SubAssign for DynamicMatrix<T>
    where
        T: Copy + Sum + Product + SubAssign + Add<Output = T> + Mul<Output = T>,
    {
        fn sub_assign(&mut self, rhs: Self) {
            assert_eq!(self.rows, rhs.rows, "Matrices must have the same number of rows for subtraction.");
            assert_eq!(self.cols, rhs.cols, "Matrices must have the same number of columns for subtraction.");

            for i in 0..self.rows {
                for j in 0..self.cols {
                    self.data[i][j] -= rhs.data[i][j];
                }
            }
        }
    }

    // Mul (Matrix * Matrix)
    impl<T> Mul for DynamicMatrix<T>
    where
        T: Copy + Sum + Product + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            assert_eq!(self.cols, rhs.rows, "The number of columns of the left matrix must equal the number of rows of the right matrix for multiplication.");

            let mut result = Self::new(self.rows, rhs.cols, t_zero());
            for i in 0..self.rows {
                for j in 0..rhs.cols {
                    for k in 0..self.cols {
                        result.data[i][j] = result.data[i][j] + self.data[i][k] * rhs.data[k][j];
                    }
                }
            }
            result
        }
    }

    // Mul (Matrix * Scalar)
    impl<T> Mul<T> for DynamicMatrix<T>
    where
        T: Copy + Sum + Product + Mul<Output = T> + Add<Output = T> + Sub<Output = T>,
    {
        type Output = Self;
        fn mul(self, rhs: T) -> Self::Output {
            let mut result = Self::new(self.rows, self.cols, t_zero());
            for i in 0..self.rows {
                for j in 0..self.cols {
                    result.data[i][j] = self.data[i][j] * rhs;
                }
            }
            result
        }
    }

    // Pow (Matrix, exponent)
    impl<T> DynamicMatrix<T>
    where
        T: Copy + Sum + Product + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        pub fn pow(self, mut n: u64) -> Self {
            assert_eq!(self.rows, self.cols, "Matrix must be square for exponentiation.");
            let mut res = DynamicMatrix::identity(self.rows);
            let mut base = self;
            while n > 0 {
                if n % 2 == 1 {
                    res = res * base.clone();
                }
                base = base.clone() * base.clone();
                n /= 2;
            }
            res
        }
    }

    // Apply (Matrix, vector)
    impl<T> DynamicMatrix<T>
    where
        T: Copy + Sum + Product + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        pub fn apply(&self, x: Vec<T>) -> Vec<T> {
            assert_eq!(self.cols, x.len(), "The number of columns of the matrix must equal the length of the vector for application.");

            let mut result = vec![t_zero(); self.rows];
            for i in 0..self.rows {
                for j in 0..self.cols {
                    result[i] = result[i] + self.data[i][j] * x[j];
                }
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::dynamic_matrix::*;
    use ac_library::ModInt998244353 as Mint;

    #[test]
    fn test_new() {
        let m = DynamicMatrix::<i32>::new(2, 3, 0);
        assert_eq!(m.rows, 2);
        assert_eq!(m.cols, 3);
        assert_eq!(m.data, [[0, 0, 0], [0, 0, 0]]);
    }

    #[test]
    fn test_identity() {
        let m = DynamicMatrix::<i32>::identity(3);
        assert_eq!(m.rows, 3);
        assert_eq!(m.cols, 3);
        assert_eq!(m.data, [[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
    }

    #[test]
    #[should_panic(expected = "Matrix must be square for exponentiation.")]
    fn test_identity_non_square_panic() {
        let _ = DynamicMatrix::<i32>::new(2, 3, 0).pow(1);
    }

    #[test]
    fn test_from_vec() {
        let m = DynamicMatrix::<i32>::from_vec(vec![vec![1, 2], vec![3, 4]]);
        assert_eq!(m.rows, 2);
        assert_eq!(m.cols, 2);
        assert_eq!(m.data, [[1, 2], [3, 4]]);
    }

    #[test]
    #[should_panic(expected = "Matrix cannot be empty")]
    fn test_from_vec_empty_panic() {
        let _ = DynamicMatrix::<i32>::from_vec(vec![]);
    }

    #[test]
    #[should_panic(expected = "All rows must have the same number of columns")]
    fn test_from_vec_ragged_panic() {
        let _ = DynamicMatrix::<i32>::from_vec(vec![vec![1, 2], vec![3]]);
    }

    #[test]
    fn test_index_access() {
        let mut m = DynamicMatrix::<i32>::new(2, 2, 0);
        m[(0, 0)] = 1;
        m[(0, 1)] = 2;
        m[(1, 0)] = 3;
        m[(1, 1)] = 4;
        assert_eq!(m[(0, 0)], 1);
        assert_eq!(m[(1, 1)], 4);
    }

    #[test]
    fn test_add() {
        let m1 = DynamicMatrix::<i32>::from_vec(vec![vec![1, 2], vec![3, 4]]);
        let m2 = DynamicMatrix::<i32>::from_vec(vec![vec![5, 6], vec![7, 8]]);
        let m3 = m1 + m2;
        assert_eq!(m3.data, [[6, 8], [10, 12]]);
    }

    #[test]
    #[should_panic(expected = "Matrices must have the same number of rows for addition.")]
    fn test_add_mismatched_rows_panic() {
        let m1 = DynamicMatrix::<i32>::new(2, 2, 0);
        let m2 = DynamicMatrix::<i32>::new(3, 2, 0);
        let _ = m1 + m2;
    }

    #[test]
    fn test_mul_matrix() {
        let m1 = DynamicMatrix::<i32>::from_vec(vec![vec![1, 2], vec![3, 4]]);
        let m2 = DynamicMatrix::<i32>::from_vec(vec![vec![5, 6], vec![7, 8]]);
        let m3 = m1 * m2;
        assert_eq!(m3.data, [[19, 22], [43, 50]]);
    }

    #[test]
    #[should_panic(expected = "The number of columns of the left matrix must equal the number of rows of the right matrix for multiplication.")]
    fn test_mul_matrix_mismatched_dims_panic() {
        let m1 = DynamicMatrix::<i32>::new(2, 3, 0);
        let m2 = DynamicMatrix::<i32>::new(2, 2, 0);
        let _ = m1 * m2;
    }

    #[test]
    fn test_mul_scalar() {
        let m = DynamicMatrix::<i32>::from_vec(vec![vec![1, 2], vec![3, 4]]);
        let m_scaled = m * 2;
        assert_eq!(m_scaled.data, [[2, 4], [6, 8]]);
    }

    #[test]
    fn test_pow() {
        let m = DynamicMatrix::<i32>::from_vec(vec![vec![1, 1], vec![1, 0]]); // Fibonacci matrix
        let m_pow_2 = m.pow(2);
        assert_eq!(m_pow_2.data, [[2, 1], [1, 1]]);
    }

    #[test]
    fn test_apply() {
        let m = DynamicMatrix::<i32>::from_vec(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        let x = vec![7, 8, 9];
        let result = m.apply(x);
        assert_eq!(result, vec![50, 122]);
    }

    #[test]
    #[should_panic(expected = "The number of columns of the matrix must equal the length of the vector for application.")]
    fn test_apply_mismatched_dims_panic() {
        let m = DynamicMatrix::<i32>::new(2, 3, 0);
        let x = vec![1, 2];
        let _ = m.apply(x);
    }

    #[test]
    fn test_modint_dynamic_matrix() {
        let m = DynamicMatrix::<Mint>::identity(3);
        assert_eq!(m.data, vec![vec![Mint::new(1), Mint::new(0), Mint::new(0)],
                                vec![Mint::new(0), Mint::new(1), Mint::new(0)],
                                vec![Mint::new(0), Mint::new(0), Mint::new(1)]]);
    }
}