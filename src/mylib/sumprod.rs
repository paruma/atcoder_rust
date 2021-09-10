mod sum {
    pub trait Sum2<A>: Sized {
        fn sum2<I: Iterator<Item = A>>(iter: I) -> Self;
    }

    impl<'a, T: num::Zero + std::ops::Add<T, Output = T> + Copy> Sum2<&'a T> for T {
        fn sum2<I: Iterator<Item = &'a T>>(iter: I) -> Self {
            iter.fold(Self::zero(), |acc, x| acc + *x)
        }
    }

    impl<T: num::Zero + std::ops::Add<T, Output = T> + Copy> Sum2<T> for T {
        fn sum2<I: Iterator<Item = T>>(iter: I) -> Self {
            iter.fold(Self::zero(), |acc, x| acc + x)
        }
    }

    pub trait IteratorExtSum2: Iterator + Sized {
        fn sum2<S>(self) -> S
        where
            Self: Sized,
            S: Sum2<Self::Item>,
        {
            Sum2::sum2(self)
        }
    }

    impl<T: Iterator> IteratorExtSum2 for T {}
}

mod product {
    pub trait Product2<A>: Sized {
        fn product2<I: Iterator<Item = A>>(iter: I) -> Self;
    }

    impl<'a, T: num::One + std::ops::Mul<T, Output = T> + Copy> Product2<&'a T> for T {
        fn product2<I: Iterator<Item = &'a T>>(iter: I) -> Self {
            iter.fold(Self::one(), |acc, x| acc * (*x))
        }
    }

    impl<T: num::One + std::ops::Mul<T, Output = T> + Copy> Product2<T> for T {
        fn product2<I: Iterator<Item = T>>(iter: I) -> Self {
            iter.fold(Self::one(), |acc, x| acc * x)
        }
    }

    pub trait IteratorExtProduct2: Iterator + Sized {
        fn product2<S>(self) -> S
        where
            Self: Sized,
            S: Product2<Self::Item>,
        {
            Product2::product2(self)
        }
    }

    impl<T: Iterator> IteratorExtProduct2 for T {}
}
#[cfg(test)]
mod tests {
    use super::product::*;
    use super::sum::*;

    #[allow(clippy::redundant_clone)]
    #[test]
    fn test_sum() {
        let xs = vec![1, 2, 3];

        let s1: i32 = xs.iter().sum();
        let s2: i32 = xs.clone().into_iter().sum();
        let s3: i32 = xs.iter().sum2();
        let s4: i32 = xs.clone().into_iter().sum2();

        assert_eq!(s1, 6);
        assert_eq!(s2, 6);
        assert_eq!(s3, 6);
        assert_eq!(s4, 6);
    }

    #[allow(clippy::redundant_clone)]
    #[test]
    fn test_prod() {
        let xs = vec![1, 2, 3, 4];

        let s1: i32 = xs.iter().product();
        let s2: i32 = xs.clone().into_iter().product();
        let s3: i32 = xs.iter().product2();
        let s4: i32 = xs.clone().into_iter().product2();

        assert_eq!(s1, 24);
        assert_eq!(s2, 24);
        assert_eq!(s3, 24);
        assert_eq!(s4, 24);
    }
}
