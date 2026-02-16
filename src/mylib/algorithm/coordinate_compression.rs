use cargo_snippet::snippet;

#[snippet(prefix = "use coordinate_compression::*;")]
#[allow(clippy::module_inception)]
pub mod coordinate_compression {
    use itertools::Itertools;
    use superslice::Ext;

    #[derive(Debug, Clone)]
    pub struct CoordinateCompression<T> {
        space: Vec<T>,
    }

    impl<T: Ord + Copy> CoordinateCompression<T> {
        /// 与えられた要素から座標圧縮空間を構築する。
        ///
        /// # 計算量
        /// O(N log N) (N = |space|)
        pub fn new(space: &[T]) -> Self {
            let space = space.iter().copied().sorted().dedup().collect_vec();
            Self { space }
        }

        /// 与えられた値を座標圧縮したインデックスを返す。
        /// 値が空間に存在しない場合はパニックする。
        ///
        /// # 計算量
        /// O(log N) (N = space_size)
        pub fn compress(&self, x: T) -> usize {
            self.space.binary_search(&x).unwrap()
        }

        /// 座標圧縮前の空間のうち x 以下である最大の値を座標圧縮したものを返す
        ///
        /// # 計算量
        /// O(log N) (N = space_size)
        pub fn compress_floor(&self, x: T) -> usize {
            self.space.upper_bound(&x) - 1
        }

        /// 座標圧縮前の空間のうち x 以上である最小の値を座標圧縮したものを返す
        ///
        /// # 計算量
        /// O(log N) (N = space_size)
        pub fn compress_ceil(&self, x: T) -> usize {
            self.space.lower_bound(&x)
        }

        /// 与えられた各要素を座標圧縮した結果を返す。
        ///
        /// # 計算量
        /// O(M log N) (M = |xs|, N = space_size)
        pub fn compress_vec(&self, xs: &[T]) -> Vec<usize> {
            xs.iter().map(|&x| self.compress(x)).collect_vec()
        }

        /// 指定された範囲内の値に対応する座標圧縮後のインデックス範囲を [begin, end) で返す。
        ///
        /// # 計算量
        /// O(log N) (N = space_size)
        pub fn compress_range(
            &self,
            range: impl std::ops::RangeBounds<T>,
        ) -> std::ops::Range<usize> {
            use std::ops::Bound::*;
            let begin = match range.start_bound() {
                Unbounded => 0,
                Included(&x) => self.space.lower_bound(&x),
                Excluded(&x) => self.space.upper_bound(&x),
            };
            let end = match range.end_bound() {
                Unbounded => self.space.len(),
                Included(&x) => self.space.upper_bound(&x),
                Excluded(&x) => self.space.lower_bound(&x),
            };
            begin..end
        }

        /// 座標圧縮されたインデックスから元の値を復元する。
        ///
        /// # 計算量
        /// O(1)
        pub fn decompress(&self, i: usize) -> T {
            self.space[i]
        }

        /// 座標圧縮後の空間の大きさ（要素数）を返す。
        pub fn space_size(&self) -> usize {
            self.space.len()
        }
    }
}

#[cfg(test)]
mod test {
    use itertools::{Itertools, chain};

    use super::coordinate_compression::*;
    #[test]
    fn test_coordinate_compression() {
        // 圧縮前: 1, 5, 8, 10, 11, 12, 32, 99
        // 圧縮後: 0, 1, 2,  3,  4,  5,  6,  7
        let xs1 = [1, 10, 5, 32, 99, 8, 10];
        let xs2 = [10, 11, 12];
        let cc = CoordinateCompression::new(&chain!(xs1, xs2).collect_vec());
        let compressed_xs1 = cc.compress_vec(&xs1);
        assert_eq!(compressed_xs1, [0, 3, 1, 6, 7, 2, 3]);
        assert_eq!(cc.decompress(3), 10);
        assert_eq!(cc.space_size(), 8);

        // 圧縮前: 1, 5, 8, 10, 11, 12, 32, 99
        // 圧縮後: 0, 1, 2,  3,  4,  5,  6,  7
        assert_eq!(cc.compress(1), 0);
        assert_eq!(cc.compress(5), 1);
        assert_eq!(cc.compress(8), 2);
        assert_eq!(cc.compress(10), 3);
        assert_eq!(cc.compress(11), 4);
        assert_eq!(cc.compress(12), 5);
        assert_eq!(cc.compress(32), 6);
        assert_eq!(cc.compress(99), 7);

        // 圧縮前: 1, 5, 8, 10, 11, 12, 32, 99
        // 圧縮後: 0, 1, 2,  3,  4,  5,  6,  7
        assert_eq!(cc.compress_floor(9), 2);
        assert_eq!(cc.compress_floor(10), 3);
        assert_eq!(cc.compress_floor(11), 4);
        assert_eq!(cc.compress_floor(12), 5);
        assert_eq!(cc.compress_floor(13), 5);

        // 圧縮前: 1, 5, 8, 10, 11, 12, 32, 99
        // 圧縮後: 0, 1, 2,  3,  4,  5,  6,  7
        assert_eq!(cc.compress_ceil(9), 3);
        assert_eq!(cc.compress_ceil(10), 3);
        assert_eq!(cc.compress_ceil(11), 4);
        assert_eq!(cc.compress_ceil(12), 5);
        assert_eq!(cc.compress_ceil(13), 6);

        // compress_range
        // space: 1, 5, 8, 10, 11, 12, 32, 99
        assert_eq!(cc.compress_range(4..11), 1..4); // values: 5, 8, 10
        assert_eq!(cc.compress_range(5..=11), 1..5); // values: 5, 8, 10, 11
        assert_eq!(cc.compress_range(..10), 0..3); // values: 1, 5, 8
        assert_eq!(cc.compress_range(32..), 6..8); // values: 32, 99
        assert_eq!(cc.compress_range(..), 0..8);

        use std::ops::Bound::*;
        assert_eq!(cc.compress_range((Excluded(10), Included(32))), 4..7); // values: 11, 12, 32
        assert_eq!(cc.compress_range((Included(10), Excluded(12))), 3..5); // values: 10, 11
    }
}
