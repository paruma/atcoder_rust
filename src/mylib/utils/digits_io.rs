use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use digits_io::*;")]
pub mod digits_io {
    use proconio::source::{Readable, Source};
    use std::io::BufRead;

    /// 0-9 で表された各桁 (Big Endian) を数値として出力する
    ///
    /// 例: `[1, 2, 3]` -> "123" を出力
    pub fn print_digits(digits: &[i64]) {
        let msg = digits
            .iter()
            .map(|&d| (d as u8 + b'0') as char)
            .collect::<String>();
        println!("{}", msg);
    }

    /// 数値を各桁（0-9）の配列として読み込むための proconio マーカー
    ///
    /// 入力: "123" -> `vec![1, 2, 3]` (Big Endian)
    pub enum Digits {}

    impl Readable for Digits {
        type Output = Vec<i64>;
        fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Vec<i64> {
            let s = <String as Readable>::read(source);
            s.bytes().map(|b| (b - b'0') as i64).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::digits_io::*;
    use proconio::input;
    use proconio::source::once::OnceSource;

    #[test]
    fn test_digits_marker() {
        let input_str = "1234567890";
        let source = OnceSource::from(input_str);
        input! {
            from source,
            ds: Digits,
        }
        assert_eq!(ds, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
    }

    #[test]
    fn test_print_digits() {
        print_digits(&[1, 2, 3, 4, 5]);
    }
}
