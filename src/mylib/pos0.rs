use cargo_snippet::snippet;

#[snippet(prefix = "use pos::*;")]
pub mod pos {
    use std::ops::{Add, Sub};

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

#[cfg(test)]
mod test {

    use num::Zero;

    use super::pos::*;
    #[test]
    fn pos_test() {
        // type UPos = Pos<usize>;
        let p1: Pos<usize> = Pos::new(2, 3);
        let p2: Pos<usize> = Pos::new(4, 7);

        let p3 = p1 + p2;

        assert_eq!(p3, Pos::new(6, 10));

        let p4 = p2 - p1;

        assert_eq!(p4, Pos::new(2, 4));

        let p5: Pos<usize> = Pos::new(0, 0);
        assert_eq!(p5, Pos::zero());
        assert!(p5.is_zero());
    }
}
