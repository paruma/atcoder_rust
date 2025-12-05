use cargo_snippet::snippet;

#[snippet(prefix = "use rect_add_imos::*;")]
#[allow(clippy::module_inception)]
pub mod rect_add_imos {
    use std::ops::{Bound, RangeBounds};

    /// 2次元いもす法（差分配列）を用いて、2次元配列に対する長方形領域への加算クエリを効率的に処理するデータ構造。
    ///
    /// 各長方形領域への加算操作はO(1)で、最終的な配列を構築するのにO(H*W)かかる。
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct RectAddImos {
        h: usize,
        w: usize,
        diff: Vec<Vec<i64>>,
    }

    impl RectAddImos {
        /// サイズ `h` × `w` の `RectAddImos` インスタンスを作成する。
        ///
        /// # 計算量
        /// O(h * w)
        pub fn new(h: usize, w: usize) -> Self {
            Self {
                h,
                w,
                diff: vec![vec![0; w + 1]; h + 1],
            }
        }

        /// 指定された `y_range` × `x_range` の長方形領域に `val` を加算する。
        ///
        /// # Panics
        /// 範囲が不正な場合にパニックする。
        ///
        /// # 計算量
        /// O(1)
        pub fn rect_add(
            &mut self,
            y_range: impl RangeBounds<usize>,
            x_range: impl RangeBounds<usize>,
            val: i64,
        ) {
            let y_range = open_range_bounds(y_range, self.h);
            let x_range = open_range_bounds(x_range, self.w);
            let y1 = y_range.start;
            let y2 = y_range.end;
            let x1 = x_range.start;
            let x2 = x_range.end;

            assert!(y1 <= y2 && y2 <= self.h);
            assert!(x1 <= x2 && x2 <= self.w);

            self.diff[y1][x1] += val;
            self.diff[y1][x2] -= val;
            self.diff[y2][x1] -= val;
            self.diff[y2][x2] += val;
        }

        /// 差分配列から最終的な2次元配列を構築する。
        ///
        /// # 計算量
        /// O(h * w)
        pub fn to_vec(mut self) -> Vec<Vec<i64>> {
            if self.h == 0 {
                return Vec::new();
            }
            if self.w == 0 {
                return vec![vec![]; self.h];
            }

            // 横方向に累積和
            for y in 0..=self.h {
                for x in 1..=self.w {
                    self.diff[y][x] += self.diff[y][x - 1];
                }
            }

            // 縦方向に累積和
            for x in 0..=self.w {
                for y in 1..=self.h {
                    self.diff[y][x] += self.diff[y - 1][x];
                }
            }

            self.diff.truncate(self.h);
            for y in 0..self.h {
                self.diff[y].truncate(self.w);
            }
            self.diff
        }
    }

    fn open_range_bounds(range: impl RangeBounds<usize>, len: usize) -> std::ops::Range<usize> {
        let l = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x + 1,
        };
        let r = match range.end_bound() {
            Bound::Unbounded => len,
            Bound::Included(&x) => x + 1,
            Bound::Excluded(&x) => x,
        };
        l..r
    }
}

#[cfg(test)]
mod tests {
    use super::rect_add_imos::*;
    use rand::{Rng, SeedableRng};

    #[test]
    fn test_rect_add_imos_basic() {
        let (h, w) = (5, 5);
        let mut imos = RectAddImos::new(h, w);

        // 初期状態はすべてゼロ
        assert_eq!(imos.clone().to_vec(), vec![vec![0; w]; h]);

        // [1, 1] to [3, 3] (exclusive) に 5 を加算
        // Expected:
        // 0 0 0 0 0
        // 0 5 5 0 0
        // 0 5 5 0 0
        // 0 0 0 0 0
        // 0 0 0 0 0
        imos.rect_add(1..3, 1..3, 5);
        let expected1 = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 5, 5, 0, 0],
            vec![0, 5, 5, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];
        assert_eq!(imos.clone().to_vec(), expected1);

        // [0, 0] to [2, 2] (exclusive) に -3 を加算
        // Expected:
        // -3 -3 0 0 0
        // -3 2 2 0 0
        // 0 5 5 0 0
        // 0 0 0 0 0
        // 0 0 0 0 0
        imos.rect_add(0..2, 0..2, -3);
        let expected2 = vec![
            vec![-3, -3, 0, 0, 0],
            vec![-3, 2, 5, 0, 0],
            vec![0, 5, 5, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];
        assert_eq!(imos.clone().to_vec(), expected2);
    }

    #[test]
    fn test_empty() {
        let mut imos = RectAddImos::new(0, 0);
        assert_eq!(imos.clone().to_vec(), Vec::<Vec<i64>>::new());
        imos.rect_add(0..0, 0..0, 10);
        assert_eq!(imos.to_vec(), Vec::<Vec<i64>>::new());
    }

    #[ignore]
    #[test]
    fn test_random_rect_add_imos() {
        let mut rng = rand::rngs::SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let h = rng.random_range(1..=50);
            let w = rng.random_range(1..=50);
            let mut naive_vec = vec![vec![0i64; w]; h];
            let mut imos = RectAddImos::new(h, w);

            for _ in 0..100 {
                let y1 = rng.random_range(0..=h);
                let y2 = rng.random_range(y1..=h);
                let x1 = rng.random_range(0..=w);
                let x2 = rng.random_range(x1..=w);
                let val = rng.random_range(-1000..=1000);

                // ナイーブな実装
                for y in y1..y2 {
                    for x in x1..x2 {
                        naive_vec[y][x] += val;
                    }
                }
                // imos法
                imos.rect_add(y1..y2, x1..x2, val);
            }
            assert_eq!(imos.to_vec(), naive_vec);
        }
    }
}
