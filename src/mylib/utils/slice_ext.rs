use cargo_snippet::snippet;

#[snippet(prefix = "use slice_at::*;")]
pub mod slice_at {
    pub trait SliceAt<T> {
        /// `i64` 型のインデックスで要素を取得する。範囲外の場合はパニックする。
        fn at(&self, index: i64) -> &T;

        /// `i64` 型のインデックスで要素を可変参照で取得する。範囲外の場合はパニックする。
        fn at_mut(&mut self, index: i64) -> &mut T;
    }

    impl<T> SliceAt<T> for [T] {
        fn at(&self, index: i64) -> &T {
            match usize::try_from(index) {
                Ok(i) => &self[i],
                Err(_) => panic!(
                    "index out of bounds: the len is {} but the index is {}",
                    self.len(),
                    index
                ),
            }
        }
        fn at_mut(&mut self, index: i64) -> &mut T {
            match usize::try_from(index) {
                Ok(i) => &mut self[i],
                Err(_) => panic!(
                    "index out of bounds: the len is {} but the index is {}",
                    self.len(),
                    index
                ),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::slice_at::*;

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

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 3 but the index is -1")]
    fn test_at_negative_panics() {
        let xs = [1, 2, 3];
        let _ = xs.at(-1);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 3 but the index is -1")]
    fn test_at_mut_negative_panics() {
        let mut xs = [1, 2, 3];
        let _ = xs.at_mut(-1);
    }
}
