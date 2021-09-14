#[allow(dead_code)]
mod math_tools {
    use num::Integer;

    pub fn divisor(n: i64) -> Vec<i64> {
        assert!(n >= 1);
        let mut retval: Vec<i64> = Vec::new();
        for i in 1..=num_integer::sqrt(n) {
            if n.is_multiple_of(&i) {
                retval.push(i);
                if i * i != n {
                    retval.push(n / i);
                }
            }
        }

        retval
    }

    fn frac0<T>(n: T, acc: T) -> T
    where
        T: std::ops::Sub<Output = T> + std::ops::Mul + num::Zero + num::One + Copy,
    {
        if n.is_zero() {
            acc
        } else {
            frac0(n - T::one(), n * acc)
        }
    }

    pub fn frac<T>(n: T) -> T
    where
        T: std::ops::Sub<Output = T> + std::ops::Mul + num::Zero + num::One + Copy,
    {
        frac0(n, T::one())
    }

    pub fn permutation<T>(n: T, k: T) -> T
    where
        T: std::ops::Sub<Output = T>
            + std::ops::Mul
            + std::ops::Div<Output = T>
            + num::Zero
            + num::One
            + Copy,
    {
        // n!/(n-k)!
        frac(n) / frac(n - k)
    }

    pub fn comb<T>(n: T, k: T) -> T
    where
        T: std::ops::Sub<Output = T>
            + std::ops::Mul
            + std::ops::Div<Output = T>
            + num::Zero
            + num::One
            + Copy,
    {
        // n!/(k!(n-k)!)
        frac(n) / frac(n - k) / frac(k)
    }
}

#[cfg(test)]
mod tests {
    use super::math_tools::*;

    #[test]
    fn test_divisor() {
        let test = |n: i64, ans: &[i64]| {
            let mut divisor = divisor(n);
            divisor.sort_unstable();
            assert_eq!(divisor, ans);
        };
        test(1, &[1]);
        test(2, &[1, 2]);
        test(16, &[1, 2, 4, 8, 16]);
        test(12, &[1, 2, 3, 4, 6, 12]);
    }

    #[test]
    fn test_frac() {
        use crate::mylib::modint_field::rf::*;
        assert_eq!(frac(5), 120);
        assert_eq!(frac(RF::new(5)), RF::new(120));
    }

    #[test]
    fn test_permutation() {
        use crate::mylib::modint_field::rf::*;
        assert_eq!(permutation(5, 3), 60); //5*4*3=60
        assert_eq!(permutation(RF::new(5), RF::new(3)), RF::new(60));
    }
}
