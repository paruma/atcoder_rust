use cargo_snippet::snippet;
#[snippet(prefix = "use matrix44::*;")]
pub mod matrix44 {
    use core::fmt::Debug;
    use std::{
        convert::Infallible,
        iter::{Product, Sum},
        marker::PhantomData,
        ops::{Add, Mul},
    };

    use ac_library::Monoid;

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Matrix44<T>
    where
        T: Clone + Copy + Debug + PartialEq,
    {
        pub raw: [[T; 4]; 4],
    }

    impl<T> Matrix44<T>
    where
        T: Clone + Copy + Debug + PartialEq,
    {
        pub fn from_array(arr: [[T; 4]; 4]) -> Self {
            Self { raw: arr }
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
            Matrix44::from_array([
                [
                    Self::t_one(),
                    Self::t_zero(),
                    Self::t_zero(),
                    Self::t_zero(),
                ],
                [
                    Self::t_zero(),
                    Self::t_one(),
                    Self::t_zero(),
                    Self::t_zero(),
                ],
                [
                    Self::t_zero(),
                    Self::t_zero(),
                    Self::t_one(),
                    Self::t_zero(),
                ],
                [
                    Self::t_zero(),
                    Self::t_zero(),
                    Self::t_zero(),
                    Self::t_one(),
                ],
            ])
        }
    }

    impl<T> Add for Matrix44<T>
    where
        T: Clone + Copy + Debug + PartialEq + Add<Output = T>,
    {
        type Output = Matrix44<T>;

        fn add(self, rhs: Self) -> Self::Output {
            let mut result = self.raw;
            for i in 0..4 {
                for j in 0..4 {
                    result[i][j] = result[i][j] + rhs.raw[i][j];
                }
            }
            Matrix44::from_array(result)
        }
    }

    impl<T> Mul for Matrix44<T>
    where
        T: Clone + Copy + Debug + PartialEq + Sum + Add<Output = T> + Mul<Output = T>,
    {
        type Output = Matrix44<T>;

        fn mul(self, rhs: Self) -> Self::Output {
            let mut result = [[Self::t_zero(); 4]; 4];
            for i in 0..4 {
                for j in 0..4 {
                    for k in 0..4 {
                        result[i][j] = result[i][j] + self.raw[i][k] * rhs.raw[k][j];
                    }
                }
            }
            Matrix44::from_array(result)
        }
    }

    pub struct Matrix44Mul<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for Matrix44Mul<T>
    where
        T: Clone + Copy + Debug + PartialEq + Sum + Product + Add<Output = T> + Mul<Output = T>,
    {
        type S = Matrix44<T>;
        fn identity() -> Self::S {
            Matrix44::identity()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            (*a) * (*b)
        }
    }
}

#[cfg(test)]
mod tests_matrix44 {
    use ac_library::Monoid;

    use super::matrix44::{Matrix44, Matrix44Mul};

    #[test]
    fn test_identity() {
        assert_eq!(
            Matrix44::<i32>::identity(),
            Matrix44::from_array([[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]])
        );

        let m1 = Matrix44::from_array([
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ]);
        assert_eq!(Matrix44::identity() * m1, m1);
        assert_eq!(m1 * Matrix44::identity(), m1);
    }

    #[test]
    fn test_add() {
        let m1 = Matrix44::from_array([
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ]);
        let m2 = Matrix44::from_array([
            [16, 15, 14, 13],
            [12, 11, 10, 9],
            [8, 7, 6, 5],
            [4, 3, 2, 1],
        ]);
        let expected = Matrix44::from_array([
            [17, 17, 17, 17],
            [17, 17, 17, 17],
            [17, 17, 17, 17],
            [17, 17, 17, 17],
        ]);
        assert_eq!(m1 + m2, expected);
    }

    #[test]
    fn test_mul() {
        let m1 = Matrix44::from_array([
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ]);
        let m2 = Matrix44::from_array([
            [16, 15, 14, 13],
            [12, 11, 10, 9],
            [8, 7, 6, 5],
            [4, 3, 2, 1],
        ]);
        let expected = Matrix44::from_array([
            [80, 70, 60, 50],
            [240, 214, 188, 162],
            [400, 358, 316, 274],
            [560, 502, 444, 386],
        ]);
        assert_eq!(m1 * m2, expected);
    }

    #[test]
    fn test_matrix44_mul_monoid() {
        type M = Matrix44Mul<i32>;
        {
            let m1 = Matrix44::from_array([
                [1, 2, 3, 4],
                [5, 6, 7, 8],
                [9, 10, 11, 12],
                [13, 14, 15, 16],
            ]);
            let m2 = Matrix44::from_array([
                [16, 15, 14, 13],
                [12, 11, 10, 9],
                [8, 7, 6, 5],
                [4, 3, 2, 1],
            ]);
            let expected = Matrix44::from_array([
                [80, 70, 60, 50],
                [240, 214, 188, 162],
                [400, 358, 316, 274],
                [560, 502, 444, 386],
            ]);
            assert_eq!(M::binary_operation(&m1, &m2), expected);
        }
        {
            let expected =
                Matrix44::from_array([[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]);
            assert_eq!(M::identity(), expected);
        }
    }
}
