// src/mylib/segtree_lib/segtree_2d.rs
use cargo_snippet::snippet;

#[snippet(prefix = "use segtree_2d::*;")]
pub mod segtree_2d {
    use ac_library::Monoid;

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

            let mut y_idx = y + self.h_size;
            let x_idx_in_row = x + self.w_size;

            // リーフノードを直接更新
            self.nodes[y_idx][x_idx_in_row] = val.clone();

            // この特定のY行（リーフYノード）のXセグメントツリーの親ノードを更新
            // この部分は、特定の行 'y' に対応するXセグメントツリーが正しいことを保証します。
            let mut current_x_idx_for_row = x_idx_in_row;
            while current_x_idx_for_row > 1 {
                current_x_idx_for_row >>= 1;
                self.nodes[y_idx][current_x_idx_for_row] = M::binary_operation(
                    &self.nodes[y_idx][current_x_idx_for_row * 2],
                    &self.nodes[y_idx][current_x_idx_for_row * 2 + 1],
                );
            } // O(log W)

            // Yセグメントツリーの親ノードを更新
            // ここで、y_idxは行（リーフ）を指します。その親を更新する必要があります。
            while y_idx > 1 {
                y_idx >>= 1; // Move to parent Y-node (e.g., node for y_range [Y1, Y2))

                // このYノードのXセグメントツリーを更新する必要があります。
                // その子ノードのXセグメントツリーを結合する必要があります。
                // The children are y_idx*2 and y_idx*2+1.
                let y_child_left = y_idx * 2;
                let y_child_right = y_idx * 2 + 1;

                // 子ノードの対応するノードを結合しながら、Xセグメントツリーのパスを上にたどります。
                let mut x_idx_path = x + self.w_size; // Start at the X-leaf corresponding to the update
                while x_idx_path > 0 {
                    // Xセグメントツリーのリーフからルートまで（0はルートではありません）
                    self.nodes[y_idx][x_idx_path] = M::binary_operation(
                        &self.nodes[y_child_left][x_idx_path],
                        &self.nodes[y_child_right][x_idx_path],
                    );
                    x_idx_path >>= 1;
                } // O(log W)
            } // O(log H * log W)
        }

        /// 矩形範囲 `[y1, y2) × [x1, x2)` の要素の結合結果をクエリします。
        ///
        /// # Panics
        /// `y1 >= y2` や `x1 >= x2` など、範囲が不正な場合にパニックします。
        /// `y2` または `x2` が範囲外の場合にパニックします。
        ///
        /// # 計算量
        /// O(log H * log W)
        pub fn prod(&self, y1: usize, x1: usize, y2: usize, x2: usize) -> M::S {
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

            let mut res = M::identity();

            let mut y_cur_l = y1 + self.h_size;
            let mut y_cur_r = y2 + self.h_size;
            let x_l = x1 + self.w_size;
            let x_r = x2 + self.w_size;

            while y_cur_l < y_cur_r {
                if y_cur_l & 1 == 1 {
                    // y_cur_lが右の子ノードの場合
                    // Y軸ノードが対応するX軸セグメントツリーに対してクエリ
                    res = M::binary_operation(&res, &self.query_x_tree(y_cur_l, x_l, x_r));
                    y_cur_l += 1;
                }
                if y_cur_r & 1 == 1 {
                    // y_cur_rが右の子ノードの場合
                    y_cur_r -= 1;
                    // Y軸ノードが対応するX軸セグメントツリーに対してクエリ
                    res = M::binary_operation(&res, &self.query_x_tree(y_cur_r, x_l, x_r));
                }
                y_cur_l >>= 1;
                y_cur_r >>= 1;
            }
            res
        }

        // 指定されたYノードの1次元Xセグメントツリーをクエリするためのヘルパー関数
        fn query_x_tree(&self, y_node_idx: usize, mut x_cur_l: usize, mut x_cur_r: usize) -> M::S {
            let mut res_x = M::identity();
            while x_cur_l < x_cur_r {
                if x_cur_l & 1 == 1 {
                    // x_cur_lが右の子ノードの場合
                    res_x = M::binary_operation(&res_x, &self.nodes[y_node_idx][x_cur_l]);
                    x_cur_l += 1;
                }
                if x_cur_r & 1 == 1 {
                    // x_cur_rが右の子ノードの場合
                    x_cur_r -= 1;
                    res_x = M::binary_operation(&res_x, &self.nodes[y_node_idx][x_cur_r]);
                }
                x_cur_l >>= 1;
                x_cur_r >>= 1;
            }
            res_x
        }
    }
}

#[cfg(test)]
mod tests {
    use super::segtree_2d::*;
    use ac_library::Additive; // from ac-library-rs
    use rand::{Rng, SeedableRng};

    #[test]
    fn test_segtree_2d_basic() {
        let h = 3;
        let w = 3;
        let mut st = Segtree2D::<Additive<i64>>::new(h, w);

        // 初期状態
        assert_eq!(st.prod(0, 0, h, w), 0); // グリッド全体をクエリ

        st.set(0, 0, 1);
        st.set(1, 1, 2);
        st.set(2, 2, 3);
        assert_eq!(st.prod(0, 0, h, w), 6); // 1+2+3

        // 値を更新
        st.set(0, 0, 5);
        assert_eq!(st.prod(0, 0, h, w), 10); // 5+2+3

        // サブ矩形をクエリ
        assert_eq!(st.prod(0, 0, 1, 1), 5); // (0,0)のみ
        assert_eq!(st.prod(1, 1, 2, 2), 2); // (1,1)のみ
        assert_eq!(st.prod(0, 0, 2, 2), 7); // (0,0), (0,1), (1,0), (1,1) (5+0+0+2 = 7)

        assert_eq!(st.prod(0, 0, 3, 3), 10); // グリッド全体
        assert_eq!(st.prod(0, 0, 1, 3), 5); // 最初の行
        assert_eq!(st.prod(0, 1, 3, 2), 2); // 中央の列 (1,1)
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
                let op_type = rng.random_range(0..2);

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

                        let actual_sum = st.prod(y1, x1, y2, x2);
                        assert_eq!(
                            actual_sum, expected_sum,
                            "prod failed for y1={},x1={},y2={},x2={}",
                            y1, x1, y2, x2
                        );
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
