use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet(prefix = "use digit::*;")]
pub mod digit {
    /// n の base 進数を Little Endian で表す
    ///
    /// 例:
    /// - `to_digits_le_vec(123, 10) == vec![3, 2, 1]`
    /// - `to_digits_le_vec(0, 10) == vec![]`
    pub fn to_digits_le_vec(mut n: i64, base: i64) -> Vec<i64> {
        assert!(n >= 0);
        assert!(base >= 2);
        if n == 0 {
            return vec![];
        }
        let mut res = vec![];
        while n > 0 {
            res.push(n % base);
            n /= base;
        }
        res
    }

    /// n の base 進数を Little Endian で生成するイテレータ
    ///
    /// 例:
    /// - `to_digits_le_iter(123, 10).collect::<Vec<_>>() == vec![3, 2, 1]`
    /// - `to_digits_le_iter(0, 10).collect::<Vec<_>>() == vec![]`
    pub fn to_digits_le_iter(n: i64, base: i64) -> impl Iterator<Item = i64> {
        assert!(n >= 0);
        assert!(base >= 2);
        DigitsLeIterator { n, base }
    }

    struct DigitsLeIterator {
        n: i64,
        base: i64,
    }

    impl Iterator for DigitsLeIterator {
        type Item = i64;
        fn next(&mut self) -> Option<Self::Item> {
            if self.n == 0 {
                return None;
            }
            let digit = self.n % self.base;
            self.n /= self.base;
            Some(digit)
        }
    }

    /// Little Endian で表された各桁から、数値を評価する
    ///
    /// 例:
    /// - `from_digits_le(&[3, 2, 1], 10) == 123`
    /// - `from_digits_le(&[], 10) == 0`
    pub fn from_digits_le(digits: &[i64], base: i64) -> i64 {
        assert!(base >= 2);
        debug_assert!(digits.iter().all(|&d| (0..base).contains(&d)));
        digits.iter().rfold(0, |acc, &d| acc * base + d)
    }

    /// x を base 進数で表した際の桁数を返す
    ///
    /// 例:
    /// - `count_digits(123, 10) == 3`
    /// - `count_digits(0, 10) == 0`
    pub fn count_digits(mut x: i64, base: i64) -> usize {
        assert!(x >= 0);
        assert!(base >= 2);
        if x == 0 {
            return 0;
        }
        let mut count = 0;
        while x > 0 {
            x /= base;
            count += 1;
        }
        count
    }

    /// 2つの数値を指定された基数で連結する。
    ///
    /// `count_digits(b, base)` が 0 (すなわち `b == 0`) の場合、`a` をそのまま返す。
    ///
    /// 例:
    /// - `concat_digits(123, 45, 10) == 12345`
    /// - `concat_digits(123, 0, 10) == 123`
    pub fn concat_digits(a: i64, b: i64, base: i64) -> i64 {
        assert!(a >= 0);
        assert!(b >= 0);
        assert!(base >= 2);
        let digits = count_digits(b, base);
        if digits == 0 {
            return a;
        }
        let mut p = 1;
        for _ in 0..digits {
            p *= base;
        }
        a * p + b
    }
}

#[cfg(test)]
mod test_digit {
    use crate::math::digit::digit::*;

    #[test]
    fn test_to_digits_le() {
        assert_eq!(to_digits_le_vec(12345, 10), vec![5, 4, 3, 2, 1]);
        assert_eq!(to_digits_le_vec(102030405, 100), vec![5, 4, 3, 2, 1]);
        assert_eq!(to_digits_le_vec(0, 10), vec![]);
    }

    #[test]
    fn test_from_digits_le() {
        assert_eq!(from_digits_le(&[5, 4, 3, 2, 1], 10), 12345);
        assert_eq!(from_digits_le(&[5, 4, 3, 2, 1], 100), 102030405);
        assert_eq!(from_digits_le(&[], 10), 0);
    }

    #[test]
    fn test_digits_le_iter() {
        assert_eq!(
            to_digits_le_iter(12345, 10).collect::<Vec<_>>(),
            vec![5, 4, 3, 2, 1]
        );
        assert_eq!(
            to_digits_le_iter(102030405, 100).collect::<Vec<_>>(),
            vec![5, 4, 3, 2, 1]
        );
        assert_eq!(to_digits_le_iter(0, 10).collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn test_count_digits() {
        assert_eq!(count_digits(123, 10), 3);
        assert_eq!(count_digits(0, 10), 0);
        assert_eq!(count_digits(102030405, 100), 5);
        assert_eq!(count_digits(10, 2), 4);
        assert_eq!(count_digits(1, 10), 1);
        assert_eq!(count_digits(9, 10), 1);
        assert_eq!(count_digits(10, 10), 2);
    }

    #[test]
    fn test_concat_digits() {
        assert_eq!(concat_digits(123, 45, 10), 12345);
        assert_eq!(concat_digits(123, 0, 10), 123);
        assert_eq!(concat_digits(0, 123, 10), 123);
        assert_eq!(concat_digits(1, 1, 2), 3); // 1, 1 (binary) -> 11 (binary) = 3
        assert_eq!(concat_digits(10, 20, 100), 1020); // 10, 20 (base 100) -> 10 * 100 + 20
    }

    #[test]
    #[should_panic]
    fn test_from_digits_le_panic() {
        from_digits_le(&[10], 10);
    }
}
