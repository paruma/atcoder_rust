use cargo_snippet::snippet;

#[snippet(prefix = "use coordinate_compression::*;")]
#[allow(clippy::module_inception)]
pub mod coordinate_compression {
    use itertools::Itertools;
    use superslice::Ext;

    pub struct CoordinateCompression {
        space: Vec<i64>,
    }

    impl CoordinateCompression {
        ///
        /// # 計算量
        /// O(|space|log(|space|))
        pub fn new(space: &[i64]) -> Self {
            let space = space.iter().copied().sorted().dedup().collect_vec();
            Self { space }
        }

        ///
        /// # 計算量
        /// O(log(|space|))
        pub fn compress(&self, x: i64) -> usize {
            self.space.binary_search(&x).unwrap()
        }

        /// 座標圧縮前の空間のうち x 以上である最小の値を座標圧縮したものを返す
        ///
        /// # 計算量
        /// O(log(|space|))
        pub fn compress_floor(&self, x: i64) -> usize {
            self.space.upper_bound(&x) - 1
        }

        /// 座標圧縮前の空間のうち x 以下である最大の値を座標圧縮したものを返す
        ///
        /// # 計算量
        /// O(log(|space|))
        pub fn compress_ceil(&self, x: i64) -> usize {
            self.space.lower_bound(&x)
        }

        /// # 計算量
        /// O(|xs|log(|space|))
        pub fn compress_vec(&self, xs: &[i64]) -> Vec<usize> {
            xs.iter().copied().map(|x| self.compress(x)).collect_vec()
        }

        /// # 計算量
        /// O(1)
        pub fn decompress(&self, i: usize) -> i64 {
            self.space[i]
        }

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
    }
}
