use cargo_snippet::snippet;
#[snippet(prefix = "use matrix33::*;")]
pub mod matrix33 {
    use core::fmt::Debug;
    use std::{
        convert::Infallible,
        iter::{Product, Sum},
        marker::PhantomData,
        ops::{Add, Mul},
    };

    use ac_library::Monoid;

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Matrix33<T>
    where
        T: Clone + Copy + Debug + PartialEq,
    {
        pub raw: [[T; 3]; 3],
    }

    impl<T> Matrix33<T>
    where
        T: Clone + Copy + Debug + PartialEq,
    {
        pub fn new(a00: T, a01: T, a02: T, a10: T, a11: T, a12: T, a20: T, a21: T, a22: T) -> Self {
            Self {
                raw: [[a00, a01, a02], [a10, a11, a12], [a20, a21, a22]],
            }
        }

        pub fn from_array(arr: [[T; 3]; 3]) -> Self {
            Self { raw: arr }
        }

        pub fn apply(self, x: (T, T, T)) -> (T, T, T)
        where
            T: Add<Output = T> + Mul<Output = T>,
        {
            (
                self.raw[0][0] * x.0 + self.raw[0][1] * x.1 + self.raw[0][2] * x.2,
                self.raw[1][0] * x.0 + self.raw[1][1] * x.1 + self.raw[1][2] * x.2,
                self.raw[2][0] * x.0 + self.raw[2][1] * x.1 + self.raw[2][2] * x.2,
            )
        }

        fn t_zero() -> T
        where
            T: Sum,
        {
            std::iter::empty().sum()
        }

        fn t_one() -> T
        where
            T: Product,
        {
            std::iter::empty().product()
        }

        pub fn identity() -> Self
        where
            T: Sum + Product,
        {
            Matrix33::from_array([
                [Self::t_one(), Self::t_zero(), Self::t_zero()],
                [Self::t_zero(), Self::t_one(), Self::t_zero()],
                [Self::t_zero(), Self::t_zero(), Self::t_one()],
            ])
        }
    }

    impl<T> Add for Matrix33<T>
    where
        T: Clone + Copy + Debug + PartialEq + Add<Output = T>,
    {
        type Output = Matrix33<T>;

        fn add(self, rhs: Self) -> Self::Output {
            Matrix33::from_array([
                [
                    self.raw[0][0] + rhs.raw[0][0],
                    self.raw[0][1] + rhs.raw[0][1],
                    self.raw[0][2] + rhs.raw[0][2],
                ],
                [
                    self.raw[1][0] + rhs.raw[1][0],
                    self.raw[1][1] + rhs.raw[1][1],
                    self.raw[1][2] + rhs.raw[1][2],
                ],
                [
                    self.raw[2][0] + rhs.raw[2][0],
                    self.raw[2][1] + rhs.raw[2][1],
                    self.raw[2][2] + rhs.raw[2][2],
                ],
            ])
        }
    }

    impl<T> Mul for Matrix33<T>
    where
        T: Clone + Copy + Debug + PartialEq + Sum + Add<Output = T> + Mul<Output = T>,
    {
        type Output = Matrix33<T>;

        fn mul(self, rhs: Self) -> Self::Output {
            let mut result = [[Self::t_zero(); 3]; 3];
            for i in 0..3 {
                for j in 0..3 {
                    for k in 0..3 {
                        result[i][j] = result[i][j] + self.raw[i][k] * rhs.raw[k][j];
                    }
                }
            }
            Matrix33::from_array(result)
        }
    }

    pub struct Matrix33Mul<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for Matrix33Mul<T>
    where
        T: Clone + Copy + Debug + PartialEq + Sum + Product + Add<Output = T> + Mul<Output = T>,
    {
        type S = Matrix33<T>;
        fn identity() -> Self::S {
            Matrix33::identity()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            (*a) * (*b)
        }
    }
}

#[cfg(test)]
mod tests_matrix33 {
    use ac_library::Monoid;

    use super::matrix33::{Matrix33, Matrix33Mul};

    #[test]
    fn test_new() {
        let m = Matrix33::new(1, 2, 3, 4, 5, 6, 7, 8, 9);
        assert_eq!(m.raw[0], [1, 2, 3]);
        assert_eq!(m.raw[1], [4, 5, 6]);
        assert_eq!(m.raw[2], [7, 8, 9]);
    }

    #[test]
    fn test_from_array() {
        let m = Matrix33::from_array([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        assert_eq!(m.raw[0], [1, 2, 3]);
        assert_eq!(m.raw[1], [4, 5, 6]);
        assert_eq!(m.raw[2], [7, 8, 9]);
    }

    #[test]
    fn test_apply() {
        let m = Matrix33::from_array([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        let x = (10, 20, 30);
        assert_eq!(m.apply(x), (140, 320, 500));
    }

    #[test]
    fn test_identity() {
        assert_eq!(
            Matrix33::<i32>::identity(),
            Matrix33::from_array([[1, 0, 0], [0, 1, 0], [0, 0, 1]])
        );

        let m1 = Matrix33::from_array([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        assert_eq!(Matrix33::identity() * m1, m1);
        assert_eq!(m1 * Matrix33::identity(), m1);
    }

    #[test]
    fn test_add() {
        let m1 = Matrix33::from_array([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        let m2 = Matrix33::from_array([[10, 20, 30], [40, 50, 60], [70, 80, 90]]);
        let expected = Matrix33::from_array([[11, 22, 33], [44, 55, 66], [77, 88, 99]]);
        assert_eq!(m1 + m2, expected);
    }

    #[test]
    fn test_mul() {
        let m1 = Matrix33::from_array([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        let m2 = Matrix33::from_array([[9, 8, 7], [6, 5, 4], [3, 2, 1]]);
        let expected = Matrix33::from_array([[30, 24, 18], [84, 69, 54], [138, 114, 90]]);
        assert_eq!(m1 * m2, expected);
    }

    #[test]
    fn test_matrix33_mul_monoid() {
        type M = Matrix33Mul<i32>;
        {
            let m1 = Matrix33::from_array([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
            let m2 = Matrix33::from_array([[9, 8, 7], [6, 5, 4], [3, 2, 1]]);
            let expected = Matrix33::from_array([[30, 24, 18], [84, 69, 54], [138, 114, 90]]);
            assert_eq!(M::binary_operation(&m1, &m2), expected);
        }
        {
            let expected = Matrix33::from_array([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
            assert_eq!(M::identity(), expected);
        }
    }
}
