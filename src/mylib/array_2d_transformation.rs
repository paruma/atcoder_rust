use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use array_2d_transformation::*;")]
pub mod array_2d_transformation {

    pub fn rotate_right<T>(table: &[Vec<T>]) -> Vec<Vec<T>>
    where
        T: Default + Clone + Copy,
    {
        let h = table.len();
        let w = table[0].len();
        let mut table_after = vec![vec![T::default(); h]; w];
        for (y, row) in table.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                table_after[x][h - 1 - y] = *v;
            }
        }
        table_after
    }

    pub fn rotate_left<T>(table: &[Vec<T>]) -> Vec<Vec<T>>
    where
        T: Default + Clone + Copy,
    {
        let h = table.len();
        let w = table[0].len();
        let mut table_after = vec![vec![T::default(); h]; w];
        for (y, row) in table.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                table_after[w - 1 - x][y] = *v;
            }
        }
        table_after
    }

    pub fn rotate_180_deg<T>(table: &[Vec<T>]) -> Vec<Vec<T>>
    where
        T: Default + Clone + Copy,
    {
        let h = table.len();
        let w = table[0].len();
        let mut table_after = vec![vec![T::default(); w]; h];
        for (y, row) in table.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                table_after[h - 1 - y][w - 1 - x] = *v;
            }
        }
        table_after
    }

    pub fn transpose<T>(table: &[Vec<T>]) -> Vec<Vec<T>>
    where
        T: Default + Clone + Copy,
    {
        let h = table.len();
        let w = table[0].len();
        let mut table_after = vec![vec![T::default(); h]; w];
        for (y, row) in table.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                table_after[x][y] = *v;
            }
        }
        table_after
    }

    pub fn reflect_x_axis<T>(table: &[Vec<T>]) -> Vec<Vec<T>>
    where
        T: Default + Clone + Copy,
    {
        let h = table.len();
        let w = table[0].len();
        let mut table_after = vec![vec![T::default(); w]; h];
        for (y, row) in table.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                table_after[h - 1 - y][x] = *v;
            }
        }
        table_after
    }

    pub fn reflect_y_axis<T>(table: &[Vec<T>]) -> Vec<Vec<T>>
    where
        T: Default + Clone + Copy,
    {
        let h = table.len();
        let w = table[0].len();
        let mut table_after = vec![vec![T::default(); w]; h];
        for (y, row) in table.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                table_after[y][w - 1 - x] = *v;
            }
        }
        table_after
    }
}

#[cfg(test)]
mod test {
    use super::array_2d_transformation::*;
    #[test]
    fn test_array_2d_transformation() {
        let table = vec![vec![1, 2, 3], vec![4, 5, 6]];

        assert_eq!(
            rotate_left(&table),
            vec![vec![3, 6], vec![2, 5], vec![1, 4]]
        );

        assert_eq!(
            rotate_right(&table),
            vec![vec![4, 1], vec![5, 2], vec![6, 3]]
        );

        assert_eq!(rotate_180_deg(&table), vec![vec![6, 5, 4], vec![3, 2, 1]]);

        assert_eq!(transpose(&table), vec![vec![1, 4], vec![2, 5], vec![3, 6]]);

        assert_eq!(reflect_x_axis(&table), vec![vec![4, 5, 6], vec![1, 2, 3]]);

        assert_eq!(reflect_y_axis(&table), vec![vec![3, 2, 1], vec![6, 5, 4]]);

        // 0×h や w×0 の場合は対応していない
    }
}
