use cargo_snippet::snippet;
#[snippet(prefix = "use matrix22::*;")]
pub mod matrix22 {
    use core::fmt::Debug;
    use std::{
        convert::Infallible,
        iter::{Product, Sum},
        marker::PhantomData,
        ops::{Add, Mul},
    };

    use ac_library::Monoid;

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Matrix22<T>
    where
        T: Clone + Copy + Debug + PartialEq,
    {
        pub raw: [[T; 2]; 2],
    }

    impl<T> Matrix22<T>
    where
        T: Clone + Copy + Debug + PartialEq,
    {
        pub fn new(a00: T, a01: T, a10: T, a11: T) -> Self {
            Self {
                raw: [[a00, a01], [a10, a11]],
            }
        }

        pub fn from_array(arr: [[T; 2]; 2]) -> Self {
            Self { raw: arr }
        }

        pub fn apply(self, x: (T, T)) -> (T, T)
        where
            T: Add<Output = T> + Mul<Output = T>,
        {
            (
                self.raw[0][0] * x.0 + self.raw[0][1] * x.1,
                self.raw[1][0] * x.0 + self.raw[1][1] * x.1,
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
            Matrix22::from_array([
                [Self::t_one(), Self::t_zero()],
                [Self::t_zero(), Self::t_one()],
            ])
        }
    }

    impl<T> Add for Matrix22<T>
    where
        T: Clone + Copy + Debug + PartialEq + Add<Output = T>,
    {
        type Output = Matrix22<T>;

        fn add(self, rhs: Self) -> Self::Output {
            Matrix22::from_array([
                [
                    self.raw[0][0] + rhs.raw[0][0],
                    self.raw[0][1] + rhs.raw[0][1],
                ],
                [
                    self.raw[1][0] + rhs.raw[1][0],
                    self.raw[1][1] + rhs.raw[1][1],
                ],
            ])
        }
    }

    impl<T> Mul for Matrix22<T>
    where
        T: Clone + Copy + Debug + PartialEq + Add<Output = T> + Mul<Output = T>,
    {
        type Output = Matrix22<T>;

        fn mul(self, rhs: Self) -> Self::Output {
            Matrix22::from_array([
                [
                    self.raw[0][0] * rhs.raw[0][0] + self.raw[0][1] * rhs.raw[1][0],
                    self.raw[0][0] * rhs.raw[0][1] + self.raw[0][1] * rhs.raw[1][1],
                ],
                [
                    self.raw[1][0] * rhs.raw[0][0] + self.raw[1][1] * rhs.raw[1][0],
                    self.raw[1][0] * rhs.raw[0][1] + self.raw[1][1] * rhs.raw[1][1],
                ],
            ])
        }
    }

    pub struct Matrix22Mul<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for Matrix22Mul<T>
    where
        T: Clone + Copy + Debug + PartialEq + Sum + Product + Add<Output = T> + Mul<Output = T>,
    {
        type S = Matrix22<T>;
        fn identity() -> Self::S {
            Matrix22::identity()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            (*a) * (*b)
        }
    }
}

#[cfg(test)]
mod tests_matrix22 {
    use ac_library::Monoid;

    use super::matrix22::{Matrix22, Matrix22Mul};

    #[test]
    fn test_new() {
        let m = Matrix22::new(4, 3, 2, 1);

        assert_eq!(m.raw[0][0], 4);
        assert_eq!(m.raw[0][1], 3);
        assert_eq!(m.raw[1][0], 2);
        assert_eq!(m.raw[1][1], 1);
    }

    #[test]
    fn test_from_array() {
        let m = Matrix22::from_array([[4, 3], [2, 1]]);

        assert_eq!(m.raw[0][0], 4);
        assert_eq!(m.raw[0][1], 3);
        assert_eq!(m.raw[1][0], 2);
        assert_eq!(m.raw[1][1], 1);
    }

    #[test]
    fn test_apply() {
        let m = Matrix22::from_array([[4, 3], [2, 1]]);

        let x = (10, 20);
        // (4 3) (10)   (100)
        // (2 1) (20) = (40)

        assert_eq!(m.apply(x), (100, 40));
    }

    #[test]
    fn test_identity() {
        assert_eq!(Matrix22::identity(), Matrix22::from_array([[1, 0], [0, 1]]));

        let m1 = Matrix22::from_array([[4, 3], [2, 1]]);
        assert_eq!(Matrix22::identity() * m1, m1);
        assert_eq!(m1 * Matrix22::identity(), m1);
    }

    #[test]
    fn test_add() {
        let m1 = Matrix22::from_array([[40, 30], [20, 10]]);
        let m2 = Matrix22::from_array([[9, 8], [7, 6]]);
        let expected = Matrix22::from_array([[49, 38], [27, 16]]);

        assert_eq!(m1 + m2, expected);
    }

    #[test]
    fn test_mul() {
        // (2 3) (6 7)   (36 41)
        // (4 5) (8 9) = (64 73)
        let m1 = Matrix22::from_array([[2, 3], [4, 5]]);
        let m2 = Matrix22::from_array([[6, 7], [8, 9]]);
        let expected = Matrix22::from_array([[36, 41], [64, 73]]);

        assert_eq!(m1 * m2, expected);
    }

    #[test]
    fn test_matrix22_mul_monoid() {
        type M = Matrix22Mul<i32>;
        {
            // (2 3) (6 7)   (36 41)
            // (4 5) (8 9) = (64 73)
            let m1 = Matrix22::from_array([[2, 3], [4, 5]]);
            let m2 = Matrix22::from_array([[6, 7], [8, 9]]);
            let expected = Matrix22::from_array([[36, 41], [64, 73]]);

            assert_eq!(M::binary_operation(&m1, &m2), expected);
        }
        {
            let expected = Matrix22::from_array([[1, 0], [0, 1]]);
            assert_eq!(M::identity(), expected);
        }
    }
}
