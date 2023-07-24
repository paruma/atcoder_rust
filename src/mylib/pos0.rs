use cargo_snippet::snippet;

#[snippet(prefix = "use pos::*;")]
pub mod pos {
    use std::ops::{Add, Mul, Sub};

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Pos<T> {
        pub x: T,
        pub y: T,
    }

    impl<T> Pos<T> {
        pub fn new(x: T, y: T) -> Pos<T> {
            Pos { x, y }
        }
    }

    impl<T: Mul<Output = T> + Copy> Pos<T> {
        pub fn scala_mul(self, rhs: T) -> Pos<T> {
            Pos::new(self.x * rhs, self.y * rhs)
        }
    }

    impl<T: Add<Output = T> + Mul<Output = T> + Copy> Pos<T> {
        pub fn norm_square(self) -> T {
            self.x * self.x + self.y * self.y
        }
    }

    impl<T: Add<Output = T> + Copy> Add for Pos<T> {
        type Output = Pos<T>;

        fn add(self, rhs: Self) -> Self::Output {
            Pos::new(self.x + rhs.x, self.y + rhs.y)
        }
    }

    impl<T: Sub<Output = T> + Copy> Sub for Pos<T> {
        type Output = Pos<T>;

        fn sub(self, rhs: Self) -> Self::Output {
            Pos::new(self.x - rhs.x, self.y - rhs.y)
        }
    }

    impl<T: num_traits::Zero + Copy> num_traits::Zero for Pos<T> {
        fn zero() -> Self {
            Pos::new(T::zero(), T::zero())
        }

        fn is_zero(&self) -> bool {
            self.x.is_zero() && self.y.is_zero()
        }
    }
}

#[snippet(include = "pos")]
#[snippet(prefix = "use vec_vec_at::*;")]
pub mod vec_vec_at {
    use super::pos::*;
    pub trait VecVecAt<T> {
        fn at(&self, pos: Pos<i64>) -> &T;
        fn at_mut(&mut self, pos: Pos<i64>) -> &mut T;
    }

    impl<T> VecVecAt<T> for Vec<Vec<T>> {
        fn at(&self, pos: Pos<i64>) -> &T {
            &self[pos.y as usize][pos.x as usize]
        }

        fn at_mut(&mut self, pos: Pos<i64>) -> &mut T {
            &mut self[pos.y as usize][pos.x as usize]
        }
    }
}

#[cfg(test)]
mod test {

    use num::Zero;

    use super::pos::*;
    #[test]
    fn test_pos_add() {
        let p1: Pos<usize> = Pos::new(2, 3);
        let p2: Pos<usize> = Pos::new(4, 7);

        assert_eq!(p1 + p2, Pos::new(6, 10));
    }

    #[test]
    fn test_pos_sub() {
        let p1: Pos<usize> = Pos::new(2, 3);
        let p2: Pos<usize> = Pos::new(4, 7);
        assert_eq!(p2 - p1, Pos::new(2, 4));
    }

    #[test]
    fn test_pos_zero() {
        let zero: Pos<usize> = Pos::new(0, 0);
        assert_eq!(Pos::zero(), zero);
        assert!(zero.is_zero());
    }

    #[test]
    fn test_pos_scala_mul() {
        let p: Pos<usize> = Pos::new(2, 3);
        assert_eq!(p.scala_mul(4), Pos::new(8, 12));
    }

    #[test]
    fn test_pos_norm_square() {
        let p: Pos<usize> = Pos::new(2, 3);
        assert_eq!(p.norm_square(), 13);
    }

    #[test]

    fn test_vec_vec_at() {
        use super::vec_vec_at::*;

        let mut xss = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(*xss.at(Pos::new(2, 1)), 6);
        *xss.at_mut(Pos::new(2, 1)) = 60;

        assert_eq!(xss, vec![vec![1, 2, 3], vec![4, 5, 60]])
    }
}
