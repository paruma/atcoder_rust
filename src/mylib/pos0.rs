use cargo_snippet::snippet;

#[snippet(prefix = "use pos::*;")]
pub mod pos {
    use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
        pub fn inner_product(self, rhs: Self) -> T {
            self.x * rhs.x + self.y * rhs.y
        }
        pub fn norm_square(self) -> T {
            self.inner_product(self)
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

    impl<T: Neg<Output = T>> Neg for Pos<T> {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Pos::new(-self.x, -self.y)
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

    impl<T: Add<Output = T> + Copy> AddAssign for Pos<T> {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }

    impl<T: Sub<Output = T> + Copy> SubAssign for Pos<T> {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }

    pub const DIR8_LIST: [Pos<i64>; 8] = [
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 1 },
        Pos { x: 1, y: 0 },
        Pos { x: 1, y: -1 },
        Pos { x: 0, y: -1 },
        Pos { x: -1, y: -1 },
        Pos { x: -1, y: 0 },
        Pos { x: -1, y: 1 },
    ];

    pub const DIR4_LIST: [Pos<i64>; 4] = [
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 0 },
        Pos { x: 0, y: -1 },
        Pos { x: -1, y: 0 },
    ];

    impl Pos<i64> {
        pub fn around4_pos_iter(self) -> impl Iterator<Item = Pos<i64>> {
            DIR4_LIST.iter().copied().map(move |d| d + self)
        }

        pub fn around8_pos_iter(self) -> impl Iterator<Item = Pos<i64>> {
            DIR8_LIST.iter().copied().map(move |d| d + self)
        }
    }
}

#[snippet(prefix = "use vec_vec_at::*;")]
pub mod vec_vec_at {
    use super::pos::*;
    use easy_ext::ext;

    #[ext(VecVecAt)]
    impl<T> Vec<Vec<T>> {
        pub fn at(&self, pos: Pos<i64>) -> &T {
            &self[pos.y as usize][pos.x as usize]
        }

        pub fn at_mut(&mut self, pos: Pos<i64>) -> &mut T {
            &mut self[pos.y as usize][pos.x as usize]
        }
    }
}

#[cfg(test)]
mod tests_pos {

    use std::collections::HashSet;

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
    fn test_pos_neg() {
        let p1: Pos<i64> = Pos::new(2, -3);
        assert_eq!(-p1, Pos::new(-2, 3));
    }

    #[test]
    fn test_pos_zero() {
        let zero: Pos<usize> = Pos::new(0, 0);
        assert_eq!(Pos::zero(), zero);
        assert!(zero.is_zero());
    }

    #[test]
    fn test_pos_add_assign() {
        let p1: Pos<i64> = Pos::new(2, 3);
        let mut p2: Pos<i64> = Pos::new(4, 7);
        p2 += p1;
        assert_eq!(p2.x, 6);
        assert_eq!(p2.y, 10);
    }

    #[test]
    fn test_pos_sub_assign() {
        let p1: Pos<i64> = Pos::new(2, 3);
        let mut p2: Pos<i64> = Pos::new(4, 7);
        p2 -= p1;
        assert_eq!(p2.x, 2);
        assert_eq!(p2.y, 4);
    }

    #[test]
    fn test_pos_scala_mul() {
        let p: Pos<usize> = Pos::new(2, 3);
        assert_eq!(p.scala_mul(4), Pos::new(8, 12));
    }

    #[test]
    fn test_pos_inner_product() {
        let p1: Pos<usize> = Pos::new(2, 3);
        let p2: Pos<usize> = Pos::new(4, 5);
        assert_eq!(p1.inner_product(p2), 23);
    }

    #[test]
    fn test_pos_norm_square() {
        let p: Pos<usize> = Pos::new(2, 3);
        assert_eq!(p.norm_square(), 13);
    }

    #[test]
    fn test_around4_pos_iter() {
        let p: Pos<i64> = Pos::new(2, 3);
        let actual = p.around4_pos_iter().collect::<HashSet<Pos<i64>>>();
        let expected = HashSet::from([
            Pos::new(2, 2),
            Pos::new(3, 3),
            Pos::new(2, 4),
            Pos::new(1, 3),
        ]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_around8_pos_iter() {
        let p: Pos<i64> = Pos::new(2, 3);
        let actual = p.around8_pos_iter().collect::<HashSet<Pos<i64>>>();
        let expected = HashSet::from([
            Pos::new(2, 2),
            Pos::new(3, 2),
            Pos::new(3, 3),
            Pos::new(3, 4),
            Pos::new(2, 4),
            Pos::new(1, 4),
            Pos::new(1, 4),
            Pos::new(1, 3),
            Pos::new(1, 2),
        ]);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod tests_vec_vec_at {
    use super::pos::*;
    use super::vec_vec_at::*;

    #[test]
    fn test_vec_vec_at() {
        let mut xss = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(*xss.at(Pos::new(2, 1)), 6);
        *xss.at_mut(Pos::new(2, 1)) = 60;

        assert_eq!(xss, vec![vec![1, 2, 3], vec![4, 5, 60]])
    }
}
