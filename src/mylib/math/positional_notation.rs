use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet(prefix = "use positional_notation::*;")]
pub mod positional_notation {

    /// 配列 xs で表された base 進数の値を評価する。
    ///
    /// 例: `eval_base_n_value(&[1, 2, 3], 10) == 123`
    pub fn eval_base_n_value(xs: &[i64], base: i64) -> i64 {
        xs.iter().fold(0, |acc, &x| acc * base + x)
    }

    /// x の base 進数での表記を Vec で表す。
    ///
    /// 例
    /// * `to_base_n_value(123, 10) == vec![1, 2, 3]`
    /// * `to_base_n_value(0, 10) == vec![]`
    pub fn to_base_n_value(x: i64, base: i64) -> Vec<i64> {
        assert!(x >= 0);
        assert!(base >= 2);
        let mut ret = vec![];
        let mut x = x;
        while x > 0 {
            ret.push(x % base);
            x /= base;
        }
        ret.reverse();
        ret
    }

    /// x を base 進数で表記した際の桁数を返す。
    ///
    /// 例
    /// * `count_digits(123, 10) == 3`
    /// * `count_digits(0, 10) == 0`
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

    /// `to_base_n_value_iter` 関数が返すイテレータ。
    /// 数値を指定された基数で表した際の各桁を順に生成する。
    pub struct BaseNIterator {
        n: i64,
        base: i64,
        current_power: i64,
    }

    impl Iterator for BaseNIterator {
        type Item = i64;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current_power == 0 {
                return None; // power が 0 の場合 (または初期の x が 0 で power も 0 の場合) 終了する
            }

            let digit = self.n / self.current_power;
            self.n %= self.current_power;
            self.current_power /= self.base;

            Some(digit)
        }
    }

    /// x の base 進数での表記をイテレータで返す。
    ///
    /// 例
    /// - `to_base_n_value_iter(123, 10).collect() == vec![1, 2, 3]`
    /// - `to_base_n_value_iter(0, 10).collect() == vec![]`
    pub fn to_base_n_value_iter(x: i64, base: i64) -> BaseNIterator {
        assert!(x >= 0);
        assert!(base >= 2);
        if x == 0 {
            // x が 0 の場合、current_power を 0 に初期化することで、next() がすぐに None を返すようにする。
            return BaseNIterator {
                n: 0,
                base,
                current_power: 0,
            };
        }

        let mut current_power = 1;
        // x / current_power >= base は current_power * base <= x と同等
        // current_power * base がオーバーフローするのを避けるために、x / current_power を使う
        while x / current_power >= base {
            current_power *= base;
        }

        BaseNIterator {
            n: x,
            base,
            current_power,
        }
    }
}

#[cfg(test)]
mod test_positional_notation {

    use crate::mylib::math::positional_notation::positional_notation::{
        count_digits, eval_base_n_value, to_base_n_value, to_base_n_value_iter,
    };

    #[test]
    fn test_eval_base_n_value() {
        {
            let xs = vec![1, 2, 3, 4, 5];
            let base = 10;
            let ans = eval_base_n_value(&xs, base);
            assert_eq!(ans, 12345);
        }
        {
            let xs = vec![1, 2, 3, 4, 5];
            let base = 100;
            let ans = eval_base_n_value(&xs, base);
            assert_eq!(ans, 102030405);
        }
    }

    #[test]
    fn test_to_base_n_value() {
        {
            let x = 12345;
            let base = 10;
            let ans = to_base_n_value(x, base);
            assert_eq!(ans, vec![1, 2, 3, 4, 5]);
        }
        {
            let x = 102030405;
            let base = 100;
            let ans = to_base_n_value(x, base);
            assert_eq!(ans, vec![1, 2, 3, 4, 5]);
        }
        {
            let x = 0;
            let base = 10;
            let ans = to_base_n_value(x, base);
            assert_eq!(ans, vec![]);
        }
    }

    #[test]
    fn test_count_digits() {
        {
            let x = 123;
            let base = 10;
            let ans = count_digits(x, base);
            assert_eq!(ans, 3);
        }
        {
            let x = 0;
            let base = 10;
            let ans = count_digits(x, base);
            assert_eq!(ans, 0);
        }
        {
            let x = 102030405;
            let base = 100;
            let ans = count_digits(x, base);
            assert_eq!(ans, 5);
        }
        {
            let x = 10;
            let base = 2; // 10 (10進数) は 1010 (2進数) であり、4桁
            let ans = count_digits(x, base);
            assert_eq!(ans, 4);
        }
        {
            let x = 1;
            let base = 10;
            let ans = count_digits(x, base);
            assert_eq!(ans, 1);
        }
        {
            let x = 9;
            let base = 10;
            let ans = count_digits(x, base);
            assert_eq!(ans, 1);
        }
        {
            let x = 10;
            let base = 10;
            let ans = count_digits(x, base);
            assert_eq!(ans, 2);
        }
    }

    #[test]
    fn test_to_base_n_value_iter() {
        {
            let x = 12345;
            let base = 10;
            let ans: Vec<i64> = to_base_n_value_iter(x, base).collect();
            assert_eq!(ans, vec![1, 2, 3, 4, 5]);
        }
        {
            let x = 0;
            let base = 10;
            let ans: Vec<i64> = to_base_n_value_iter(x, base).collect();
            assert_eq!(ans, vec![]);
        }
        {
            let x = 102030405;
            let base = 100;
            let ans: Vec<i64> = to_base_n_value_iter(x, base).collect();
            assert_eq!(ans, vec![1, 2, 3, 4, 5]);
        }
    }
}
