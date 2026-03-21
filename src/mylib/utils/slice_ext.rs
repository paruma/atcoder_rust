use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use get_by_i64::*;")]
pub mod get_by_i64 {
    pub trait SliceExt<T> {
        /// `i64` 型のインデックスで要素を取得する。
        ///
        /// インデックスが負または範囲外の場合は `None` を返す。
        fn get_by_i64(&self, index: i64) -> Option<&T>;

        /// `i64` 型のインデックスで要素を可変参照で取得する。
        ///
        /// インデックスが負または範囲外の場合は `None` を返す。
        fn get_mut_by_i64(&mut self, index: i64) -> Option<&mut T>;

        /// `i64` 型のインデックスで要素を取得する。範囲外の場合はパニックする。
        fn at(&self, index: i64) -> &T;

        /// `i64` 型のインデックスで要素を可変参照で取得する。範囲外の場合はパニックする。
        fn at_mut(&mut self, index: i64) -> &mut T;
    }

    impl<T> SliceExt<T> for [T] {
        fn get_by_i64(&self, index: i64) -> Option<&T> {
            usize::try_from(index).ok().and_then(|i| self.get(i))
        }
        fn get_mut_by_i64(&mut self, index: i64) -> Option<&mut T> {
            usize::try_from(index).ok().and_then(|i| self.get_mut(i))
        }
        fn at(&self, index: i64) -> &T {
            &self[index as usize]
        }
        fn at_mut(&mut self, index: i64) -> &mut T {
            &mut self[index as usize]
        }
    }
}

#[cfg(test)]
mod test {
    use super::get_by_i64::*;

    #[test]
    fn test_get_by_i64() {
        let xs = [10, 20, 30, 40, 50];
        // 有効インデックス
        assert_eq!(xs.get_by_i64(0), Some(&10));
        assert_eq!(xs.get_by_i64(4), Some(&50));
        assert_eq!(xs.get_by_i64(2), Some(&30));
        // 負インデックス
        assert_eq!(xs.get_by_i64(-1), None);
        // 範囲外
        assert_eq!(xs.get_by_i64(5), None);
    }

    #[test]
    fn test_get_mut_by_i64() {
        let mut xs = [10, 20, 30];
        // 有効インデックスで値を書き換え
        *xs.get_mut_by_i64(1).unwrap() = 99;
        assert_eq!(xs, [10, 99, 30]);
        // 負インデックス
        assert_eq!(xs.get_mut_by_i64(-1), None);
    }

    #[test]
    fn test_at() {
        let xs = [1, 2, 3, 4, 5];
        assert_eq!(xs.at(0), &1);
        assert_eq!(xs.at(4), &5);
    }

    #[test]
    fn test_at_2d() {
        let xss = [[1, 2, 3, 4, 5], [6, 7, 8, 9, 10]];
        assert_eq!(xss.at(1).at(2), &8);
    }

    #[test]
    fn test_at_mut() {
        let mut xs = [1, 2, 3, 4, 5];
        *xs.at_mut(2) = 100;
        assert_eq!(xs, [1, 2, 100, 4, 5]);
    }
}
