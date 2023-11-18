use cargo_snippet::snippet;

#[snippet(prefix = "use imos_1d::*;")]
pub mod imos_1d {
    // TODO: Usize1 にしたいかも

    // [begin, end) で value、それ以外で0を取る関数
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct RangeFunc {
        begin: usize,
        end: usize,
        value: i64,
    }

    impl RangeFunc {
        pub fn new(begin: usize, end: usize, value: i64) -> Self {
            Self { begin, end, value }
        }
    }

    /// sum value * 1_{[begin, end)} を Vec<i64> として計算する
    /// [0, space) の範囲で考える
    /// 各 begin, end は [0, space) に入っていてほしい
    pub fn calc_imos_1d(range_func_list: &[RangeFunc], space: usize) -> Vec<i64> {
        let mut imos = vec![0; space];
        for &range_func in range_func_list {
            imos[range_func.begin] += range_func.value;
            imos[range_func.end] -= range_func.value;
        }

        for i in 1..space {
            imos[i] += imos[i - 1];
        }
        imos
    }
}

#[snippet(prefix = "use imos_2d::*;")]
pub mod imos_2d {

    // [x_begin, x_end) * [y_begin, y_end) で value、それ以外で0を取る関数
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct RectFunc {
        x_begin: usize,
        x_end: usize,
        y_begin: usize,
        y_end: usize,
        value: i64,
    }

    impl RectFunc {
        pub fn new(x_begin: usize, x_end: usize, y_begin: usize, y_end: usize, value: i64) -> Self {
            Self { x_begin, x_end, y_begin, y_end, value }
        }
    }

    #[allow(clippy::needless_range_loop)]
    pub fn calc_imos_2d(
        rect_func_list: &[RectFunc],
        x_space: usize,
        y_space: usize,
    ) -> Vec<Vec<i64>> {
        let mut imos = vec![vec![0; x_space]; y_space];
        for &rect_func in rect_func_list {
            imos[rect_func.y_begin][rect_func.x_begin] += rect_func.value;
            imos[rect_func.y_begin][rect_func.x_end] -= rect_func.value;
            imos[rect_func.y_end][rect_func.x_begin] -= rect_func.value;
            imos[rect_func.y_end][rect_func.x_end] += rect_func.value;
        }
        for x in 1..x_space {
            for y in 0..y_space {
                imos[y][x] += imos[y][x - 1];
            }
        }
        for y in 1..y_space {
            for x in 0..x_space {
                imos[y][x] += imos[y - 1][x];
            }
        }
        imos
    }
}

#[cfg(test)]
mod test {

    use super::imos_1d::*;
    #[test]
    fn test_imos_1d() {
        // 0 1 2 3 4 5
        // ------------
        //   2 2 2 2     [1, 5), value: 2
        // 3 3           [0, 2), value: 3
        // ------------
        // 3 5 2 2 2 0

        let range_func_list = [RangeFunc::new(1, 5, 2), RangeFunc::new(0, 2, 3)];
        let imos = calc_imos_1d(&range_func_list, 6);

        assert_eq!(imos, [3, 5, 2, 2, 2, 0]);
    }

    use super::imos_2d::*;
    #[test]
    fn test_imos_2d() {
        /*
        1 1 1 1 0 0       0 0 0 0 0 0       0 0 0 0 0 0       1 1 1 1 0 0
        1 1 1 1 0 0       0 0 0 0 0 0       0 0 0 0 0 0       1 1 1 1 0 0
        1 1 1 1 0 0   +   0 0 1 1 1 1   +   0 0 0 0 0 0   =   1 1 2 2 1 1
        1 1 1 1 0 0       0 0 1 1 1 1       0 1 1 0 0 0       1 2 3 2 1 1
        0 0 0 0 0 0       0 0 1 1 1 1       0 1 1 0 0 0       0 1 2 1 1 1
        0 0 0 0 0 0       0 0 1 1 1 1       0 0 0 0 0 0       0 0 1 1 1 1

        [0, 4) * [0 ,4)   [2, 6) * [2, 6)   [1, 3) * [3, 5)
         */

        let range_func_list = [
            RectFunc::new(0, 4, 0, 4, 1),
            RectFunc::new(2, 6, 2, 6, 1),
            RectFunc::new(1, 3, 3, 5, 1),
        ];
        // end も スペースに入るようにちょっと大きめに設定
        let imos = calc_imos_2d(&range_func_list, 7, 7);

        assert_eq!(
            imos,
            [
                [1, 1, 1, 1, 0, 0, 0],
                [1, 1, 1, 1, 0, 0, 0],
                [1, 1, 2, 2, 1, 1, 0],
                [1, 2, 3, 2, 1, 1, 0],
                [0, 1, 2, 1, 1, 1, 0],
                [0, 0, 1, 1, 1, 1, 0],
                [0, 0, 0, 0, 0, 0, 0],
            ]
        );
    }
}
