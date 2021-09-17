// Rustで競プロするときのプラクティス [2018秋] - ベインのブログ https://vain0x.github.io/blog/2018-10-07/rust-procon/
// 現代のAtCoderではitertoolsにcollect_vecがあるので、それを使えば良い

trait IteratorExt: Iterator + Sized {
    fn vec(self) -> Vec<Self::Item> {
        self.collect()
    }
}

impl<T: Iterator> IteratorExt for T {}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;
    #[test]
    fn iter_vec_test() {
        let xs = (0..4).map(|i| i + 1).vec();
        let ans = vec![1, 2, 3, 4];
        assert_eq!(xs, ans);
    }

    #[test]
    fn iter_vec_test2() {
        let xs = vec![1, 2, 3, 4];
        let ys = vec![1, 2, 3, 4];

        // into_iterの方にしか対応していないのか
        // 基本的には使わない方針で

        xs.into_iter().vec();
        ys.into_iter().collect_vec();
        // xs.iter().vec();コンパイルエラー
    }
}
