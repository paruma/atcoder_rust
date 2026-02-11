use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet(prefix = "use positional_notation::*;")]
pub mod positional_notation {
    /// 数値を指定した基数の各桁に分解する（Little Endian：低い位の桁から順）
    /// 例: `to_digits_le(123, 10) == vec![3, 2, 1]`
    pub fn to_digits_le(mut n: i64, base: i64) -> Vec<i64> {
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

    /// 数値の各桁を生成するイテレータ（Little Endian：低い位から順）
    /// 例: `digits_le_iter(123, 10).collect::<Vec<_>>() == vec![3, 2, 1]`
    pub fn digits_le_iter(n: i64, base: i64) -> DigitsLeIterator {
        assert!(n >= 0);
        assert!(base >= 2);
        DigitsLeIterator { n, base }
    }

    pub struct DigitsLeIterator {
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

    /// 各桁（Little Endian）から数値を評価する
    /// 例: `from_digits_le(&[3, 2, 1], 10) == 123`
    pub fn from_digits_le(digits: &[i64], base: i64) -> i64 {
        assert!(base >= 2);
        digits.iter().rfold(0, |acc, &d| acc * base + d)
    }

    /// 数値を指定した基数の各桁に分解する（Big Endian：高い位の桁から順）
    /// 例: `to_digits_be(123, 10) == vec![1, 2, 3]`
    pub fn to_digits_be(n: i64, base: i64) -> Vec<i64> {
        let mut res = to_digits_le(n, base);
        res.reverse();
        res
    }

    /// 各桁（Big Endian）から数値を評価する（ホーナー法）
    /// 例: `from_digits_be(&[1, 2, 3], 10) == 123`
    pub fn from_digits_be(digits: &[i64], base: i64) -> i64 {
        assert!(base >= 2);
        digits.iter().fold(0, |acc, &d| acc * base + d)
    }

    /// x を base 進数で表記した際の桁数を返す。
    /// 例: `count_digits(123, 10) == 3`
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
}

#[cfg(test)]
mod test_positional_notation {
    use crate::math::positional_notation::positional_notation::*;

    #[test]
    fn test_to_digits_le() {
        assert_eq!(to_digits_le(12345, 10), vec![5, 4, 3, 2, 1]);
        assert_eq!(to_digits_le(102030405, 100), vec![5, 4, 3, 2, 1]);
        assert_eq!(to_digits_le(0, 10), vec![]);
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
            digits_le_iter(12345, 10).collect::<Vec<_>>(),
            vec![5, 4, 3, 2, 1]
        );
        assert_eq!(
            digits_le_iter(102030405, 100).collect::<Vec<_>>(),
            vec![5, 4, 3, 2, 1]
        );
        assert_eq!(digits_le_iter(0, 10).collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn test_to_digits_be() {
        assert_eq!(to_digits_be(12345, 10), vec![1, 2, 3, 4, 5]);
        assert_eq!(to_digits_be(102030405, 100), vec![1, 2, 3, 4, 5]);
        assert_eq!(to_digits_be(0, 10), vec![]);
    }

    #[test]
    fn test_from_digits_be() {
        assert_eq!(from_digits_be(&[1, 2, 3, 4, 5], 10), 12345);
        assert_eq!(from_digits_be(&[1, 2, 3, 4, 5], 100), 102030405);
        assert_eq!(from_digits_be(&[], 10), 0);
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
}
