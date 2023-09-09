use cargo_snippet::snippet;

#[snippet(prefix = "use print_arr::*;")]
pub mod print_arr {
    use ndarray::{Array2, Array3};

    pub fn print_arr<T: std::fmt::Debug>(arr: &[T]) {
        for a in arr {
            print!("{:?} ", a);
        }
        println!();
    }

    pub fn print_arr2<T: std::fmt::Debug>(arr: &Array2<T>) {
        for i in 0..arr.nrows() {
            for j in 0..arr.ncols() {
                print!("{:?} ", arr[[i, j]]);
            }
            println!();
        }
    }

    pub fn print_arr3<T: std::fmt::Debug>(arr: &Array3<T>) {
        let shape = arr.shape();
        for i in 0..shape[0] {
            for j in 0..shape[1] {
                for k in 0..shape[2] {
                    print!("{:?} ", arr[[i, j, k]]);
                }
                println!();
            }
            println!();
        }
    }
}

#[snippet(prefix = "use print_vec::*;")]
pub mod print_vec {

    pub fn print_vec<T: std::fmt::Debug>(arr: &[T]) {
        for a in arr {
            print!("{:?} ", a);
        }
        println!();
    }

    #[allow(clippy::needless_range_loop)]
    pub fn print_vec2<T: std::fmt::Debug>(arr: &Vec<Vec<T>>) {
        let height = arr.len();
        let width = arr[0].len();
        for y in 0..height {
            for x in 0..width {
                print!("{:?} ", arr[y][x]);
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use ndarray::{Array, Array2, Array3};

    use super::print_arr::*;
    use super::print_vec::*;

    #[test]
    fn test_print_arr() {
        let arr: Vec<i64> = vec![3; 4];

        print_arr(&arr);
    }

    #[test]
    fn test_print_arr2() {
        let arr: Array2<i64> = Array::from_shape_fn((2, 3), |_| 3);

        print_arr2(&arr);
    }

    #[test]
    fn test_print_arr3() {
        let arr: Array3<i64> = Array::from_shape_fn((2, 3, 4), |_| 3);

        print_arr3(&arr);
    }

    #[test]
    fn test_print_vec() {
        let arr = vec![1, 2, 3];
        print_vec(&arr);
    }

    #[test]
    fn test_print_vec2() {
        let arr = vec![vec![1, 2, 3], vec![4, 5, 6]];

        print_vec2(&arr);
    }
}
