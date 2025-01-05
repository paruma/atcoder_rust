use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet(prefix = "use positional_notation::*;")]
pub mod positional_notation {

    /// 配列 xs で表された base 進数の値を評価する
    /// 例: `eval_base_n_value(&[1, 2, 3], 10) == 123`
    pub fn eval_base_n_value(xs: &[i64], base: i64) -> i64 {
        xs.iter().fold(0, |acc, &x| acc * base + x)
    }

    /// n の base 進数での表記を Vec で表す
    /// 例: `to_base_n_value(123, 10) == vec![1, 2, 3]`
    pub fn to_base_n_value(x: i64, base: i64) -> Vec<i64> {
        let mut ret = vec![];
        let mut x = x;
        while x > 0 {
            ret.push(x % base);
            x /= base;
        }
        ret.reverse();
        ret
    }
}

#[cfg(test)]
mod test_positional_notation {

    use super::positional_notation::*;

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
    }
}
