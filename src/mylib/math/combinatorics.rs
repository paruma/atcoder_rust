use cargo_snippet::snippet;

#[snippet(prefix = "use mod_combinatorics::*;")]
pub mod mod_combinatorics {
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

    ///計算量: O(n)
    pub fn frac<T>(n: T) -> T
    where
        T: std::ops::Sub<Output = T> + std::ops::Mul + num::Zero + num::One + Copy,
    {
        frac0(n, T::one())
    }

    /// 計算量: O(n)
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

    /// 計算量: O(n)
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
    use super::mod_combinatorics::*;

    #[test]
    fn test_frac() {
        use crate::mylib::math::modint_field::rf::*;
        assert_eq!(frac(5), 120);
        assert_eq!(frac(RF::new(5)), RF::new(120));
    }

    #[test]
    fn test_permutation() {
        use crate::mylib::math::modint_field::rf::*;
        assert_eq!(permutation(5, 3), 60); //5*4*3=60
        assert_eq!(permutation(RF::new(5), RF::new(3)), RF::new(60));
    }
}
