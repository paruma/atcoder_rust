use cargo_snippet::snippet;

#[snippet(prefix = "use segtree_2d::*;")]
#[allow(clippy::module_inception)]
pub mod segtree_2d {
    use ac_library::Monoid;
    use std::ops::{Bound, Range, RangeBounds};

    #[derive(Clone)]
    pub struct Segtree2D<M: Monoid> {
        h_orig: usize, // 元のグリッドの高さ
        w_orig: usize, // 元のグリッドの幅
        h_size: usize, // 高さの2のべき乗サイズ
        w_size: usize, // 幅の2のべき乗サイズ
        // 2次元セグメントツリーのノードデータ。
        // nodes[y_idx] がY軸方向のノードに対応するX軸セグメントツリーのデータ配列を保持する。
        // nodesの外側のVecは2 * h_sizeの長さ、内側のVecは2 * w_sizeの長さになる。
        nodes: Vec<Vec<M::S>>,
    }

    impl<M: Monoid> Segtree2D<M>
    where
        M::S: Clone,
    {
        /// `h` x `w` サイズの新しい2次元セグメントツリーを作成します。
        ///
        /// # 計算量
        /// O(H_size * W_size)
        pub fn new(h: usize, w: usize) -> Self {
            let h_size = h.next_power_of_two();
            let w_size = w.next_power_of_two();
            let nodes = vec![vec![M::identity(); 2 * w_size]; 2 * h_size];
            Self {
                h_orig: h,
                w_orig: w,
                h_size,
                w_size,
                nodes,
            }
        }

        /// 点 `(y, x)` の値を `val` に更新します。
        ///
        /// # Panics
        /// `y` または `x` が範囲外の場合にパニックします。
        ///
        /// # 計算量
        /// O(log H * log W)
        pub fn set(&mut self, y: usize, x: usize, val: M::S) {
            assert!(
                y < self.h_orig,
                "y is out of bounds: {} >= {}",
                y,
                self.h_orig
            );
            assert!(
                x < self.w_orig,
                "x is out of bounds: {} >= {}",
                x,
                self.w_orig
            );

            let y_idx = y + self.h_size;
            let x_idx_in_row = x + self.w_size;

            // リーフノードを直接更新
            self.nodes[y_idx][x_idx_in_row] = val.clone();

            // この特定のY行（リーフYノード）のXセグメントツリーの親ノードを更新
            for x_p in
                std::iter::successors(Some(x_idx_in_row), |&i| (i > 1).then_some(i >> 1)).skip(1)
            {
                self.nodes[y_idx][x_p] = M::binary_operation(
                    &self.nodes[y_idx][x_p * 2],
                    &self.nodes[y_idx][x_p * 2 + 1],
                );
            } // O(log W)

            // Yセグメントツリーの親ノードを更新
            for y_p in std::iter::successors(Some(y_idx), |&i| (i > 1).then_some(i >> 1)).skip(1) {
                let y_child_left = y_p * 2;
                let y_child_right = y_p * 2 + 1;

                // 子ノードの対応するノードを結合しながら、Xセグメントツリーのパスを上にたどります。
                for x_idx in
                    std::iter::successors(Some(x_idx_in_row), |&i| (i > 1).then_some(i >> 1))
                {
                    self.nodes[y_p][x_idx] = M::binary_operation(
                        &self.nodes[y_child_left][x_idx],
                        &self.nodes[y_child_right][x_idx],
                    );
                } // O(log W)
            } // O(log H * log W)
        }

        /// 矩形範囲 `[y1, y2) × [x1, x2)` の要素の結合結果をクエリします。
        /// `y_range` はy座標の範囲、`x_range` はx座標の範囲です。
        ///
        /// # Panics
        /// 範囲が不正な場合にパニックします。
        ///
        /// # 計算量
        /// O(log H * log W)
        pub fn prod(
            &self,
            y_range: impl RangeBounds<usize>,
            x_range: impl RangeBounds<usize>,
        ) -> M::S {
            let y_range = open_range_bounds(y_range, self.h_orig);
            let x_range = open_range_bounds(x_range, self.w_orig);

            let y1 = y_range.start;
            let x1 = x_range.start;
            let y2 = y_range.end;
            let x2 = x_range.end;

            assert!(
                y1 <= y2 && y2 <= self.h_orig,
                "y range invalid: y1={}, y2={}, h_orig={}",
                y1,
                y2,
                self.h_orig
            );
            assert!(
                x1 <= x2 && x2 <= self.w_orig,
                "x range invalid: x1={}, x2={}, w_orig={}",
                x1,
                x2,
                self.w_orig
            );

            let mut sml = M::identity();
            let mut smr = M::identity();

            let mut y_cur_l = y1 + self.h_size;
            let mut y_cur_r = y2 + self.h_size;
            let x_l = x1 + self.w_size;
            let x_r = x2 + self.w_size;

            while y_cur_l < y_cur_r {
                if y_cur_l & 1 == 1 {
                    // y_cur_lが右の子ノードの場合
                    // Y軸ノードが対応するX軸セグメントツリーに対してクエリ
                    sml = M::binary_operation(&sml, &self.query_x_tree(y_cur_l, x_l, x_r));
                    y_cur_l += 1;
                }
                if y_cur_r & 1 == 1 {
                    y_cur_r -= 1;
                    // Y軸ノードが対応するX軸セグメントツリーに対してクエリ
                    smr = M::binary_operation(&self.query_x_tree(y_cur_r, x_l, x_r), &smr);
                }
                y_cur_l >>= 1;
                y_cur_r >>= 1;
            }
            M::binary_operation(&sml, &smr)
        }

        // 指定されたYノードの1次元Xセグメントツリーをクエリするためのヘルパー関数
        fn query_x_tree(&self, y_node_idx: usize, mut x_cur_l: usize, mut x_cur_r: usize) -> M::S {
            let mut sml = M::identity();
            let mut smr = M::identity();
            while x_cur_l < x_cur_r {
                if x_cur_l & 1 == 1 {
                    // x_cur_lが右の子ノードの場合
                    sml = M::binary_operation(&sml, &self.nodes[y_node_idx][x_cur_l]);
                    x_cur_l += 1;
                }
                if x_cur_r & 1 == 1 {
                    // x_cur_rが右の子ノードの場合
                    x_cur_r -= 1;
                    smr = M::binary_operation(&self.nodes[y_node_idx][x_cur_r], &smr);
                }
                x_cur_l >>= 1;
                x_cur_r >>= 1;
            }
            M::binary_operation(&sml, &smr)
        }

        /// グリッド全体の要素の結合結果をクエリします。
        /// 矩形範囲 `[0, h_orig) × [0, w_orig)` と同じです。
        ///
        /// # 計算量
        /// O(1)
        pub fn all_prod(&self) -> M::S {
            self.nodes[1][1].clone()
        }

        /// 点 `(y, x)` の値を取得します。
        /// これは `self.nodes` のリーフノードに直接アクセスするため、O(1)です。
        ///
        /// # Panics
        /// `y` または `x` が範囲外の場合にパニックします。
        ///
        /// # 計算量
        /// O(1)
        pub fn get(&self, y: usize, x: usize) -> M::S {
            assert!(
                y < self.h_orig,
                "y is out of bounds: {} >= {}",
                y,
                self.h_orig
            );
            assert!(
                x < self.w_orig,
                "x is out of bounds: {} >= {}",
                x,
                self.w_orig
            );
            self.nodes[y + self.h_size][x + self.w_size].clone()
        }

        /// グリッド全体を `Vec<Vec<M::S>>` として返します。
        ///
        /// # 計算量
        /// O(H_orig * W_orig)
        pub fn to_vec(&self) -> Vec<Vec<M::S>> {
            let mut result = vec![vec![M::identity(); self.w_orig]; self.h_orig];
            for y in 0..self.h_orig {
                for x in 0..self.w_orig {
                    result[y][x] = self.get(y, x);
                }
            }
            result
        }
    } // Closes impl<M: Monoid> Segtree2D<M>

    // RangeBoundsを処理するためのヘルパー関数
    fn open_range_bounds(range: impl RangeBounds<usize>, len: usize) -> Range<usize> {
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
    use super::segtree_2d::*;
    use ac_library::{Additive, Monoid}; // from ac-library-rs
    use rand::{Rng, SeedableRng};

    #[test]
    fn test_segtree_2d_basic() {
        let h = 3;
        let w = 3;
        let mut st = Segtree2D::<Additive<i64>>::new(h, w);

        // 初期状態
        assert_eq!(st.prod(0..h, 0..w), 0); // グリッド全体をクエリ

        st.set(0, 0, 1);
        st.set(1, 1, 2);
        st.set(2, 2, 3);
        assert_eq!(st.prod(0..h, 0..w), 6); // 1+2+3

        // 値を更新
        st.set(0, 0, 5);
        assert_eq!(st.prod(0..h, 0..w), 10); // 5+2+3

        // サブ矩形をクエリ
        assert_eq!(st.prod(0..1, 0..1), 5); // (0,0)のみ
        assert_eq!(st.prod(1..2, 1..2), 2); // (1,1)のみ
        assert_eq!(st.prod(0..2, 0..2), 7); // (0,0), (0,1), (1,0), (1,1) (5+0+0+2 = 7)

        assert_eq!(st.prod(0..3, 0..3), 10); // グリッド全体
        assert_eq!(st.prod(0..1, 0..3), 5); // 最初の行
        assert_eq!(st.prod(0..3, 1..2), 2); // 中央の列 (1,1)

        // Verify all_prod
        assert_eq!(st.all_prod(), 10); // 5+2+3

        // Verify get
        assert_eq!(st.get(0, 0), 5);
        assert_eq!(st.get(1, 1), 2);
        assert_eq!(st.get(2, 2), 3);
        assert_eq!(st.get(0, 1), 0);
        assert_eq!(st.get(1, 0), 0);

        // Verify to_vec
        let expected_grid = vec![vec![5, 0, 0], vec![0, 2, 0], vec![0, 0, 3]];
        assert_eq!(st.to_vec(), expected_grid);
    }

    #[test]
    #[should_panic(expected = "y is out of bounds")]
    fn test_segtree_2d_set_panic_y() {
        let mut st = Segtree2D::<Additive<i64>>::new(3, 3);
        st.set(3, 0, 1);
    }

    #[test]
    #[should_panic(expected = "x is out of bounds")]
    fn test_segtree_2d_set_panic_x() {
        let mut st = Segtree2D::<Additive<i64>>::new(3, 3);
        st.set(0, 3, 1);
    }

    #[ignore]
    #[test]
    fn test_segtree_2d_random() {
        use rand::rngs::SmallRng;

        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            // 100回の試行
            let h = rng.random_range(1..=10);
            let w = rng.random_range(1..=10);
            let mut st = Segtree2D::<Additive<i64>>::new(h, w);
            let mut naive_grid = vec![vec![0i64; w]; h];

            for _ in 0..100 {
                // 100回の操作
                let op_type = rng.random_range(0..3); // 0: set, 1: prod, 2: get

                match op_type {
                    0 => {
                        // 設定
                        let y = rng.random_range(0..h);
                        let x = rng.random_range(0..w);
                        let val = rng.random_range(-100..=100);
                        st.set(y, x, val);
                        naive_grid[y][x] = val;
                    }
                    1 => {
                        // クエリ
                        let y1 = rng.random_range(0..=h);
                        let x1 = rng.random_range(0..=w);
                        let y2 = rng.random_range(y1..=h);
                        let x2 = rng.random_range(x1..=w);

                        let mut expected_sum = 0i64;
                        for r_y in y1..y2 {
                            for r_x in x1..x2 {
                                expected_sum += naive_grid[r_y][r_x];
                            }
                        }

                        let actual_sum = st.prod(y1..y2, x1..x2);
                        assert_eq!(
                            actual_sum, expected_sum,
                            "prod failed for y1={},x1={},y2={},x2={}",
                            y1, x1, y2, x2
                        );
                    }
                    2 => {
                        // get
                        let y = rng.random_range(0..h);
                        let x = rng.random_range(0..w);
                        assert_eq!(
                            st.get(y, x),
                            naive_grid[y][x],
                            "get failed for y={},x={}",
                            y,
                            x
                        );
                    }
                    _ => unreachable!(),
                }
            }
            // 最終チェック
            assert_eq!(
                st.to_vec(),
                naive_grid,
                "to_vec failed after all operations"
            );
        }
    }

    // 文字列連結による非可換モノイドのテスト
    struct StringMonoid;
    impl Monoid for StringMonoid {
        type S = String;
        fn identity() -> Self::S {
            String::new()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a.clone() + b
        }
    }

    #[test]
    fn test_segtree_2d_non_commutative() {
        let h = 4;
        let w = 4;
        let mut st = Segtree2D::<StringMonoid>::new(h, w);
        st.set(0, 0, "A".to_string());
        st.set(0, 1, "B".to_string());
        st.set(1, 0, "C".to_string());
        st.set(1, 1, "D".to_string());

        // クエリは辞書式順序である (0,0), (0,1), (1,0), (1,1) を尊重する必要がある
        assert_eq!(st.prod(0..2, 0..2), "ABCD".to_string());

        st.set(2, 2, "E".to_string());
        st.set(3, 3, "F".to_string());
        assert_eq!(st.prod(0..4, 0..4), "ABCDEF".to_string());

        // (0,0)から始まらないサブ矩形の確認
        st.set(1, 2, "G".to_string());
        st.set(2, 1, "H".to_string());
        // 順序は (1,1), (1,2), (2,1), (2,2) となるべき
        assert_eq!(st.prod(1..3, 1..3), "DGHE".to_string());
    }
}
