use cargo_snippet::snippet;

#[snippet(prefix = "use print_arr::*;")]
pub mod print_arr {
    use ndarray::{Array2, Array3};
    use proconio::fastout;

    #[fastout]
    pub fn print_arr<T: std::fmt::Debug>(arr: &[T]) {
        for a in arr {
            print!("{:?} ", a);
        }
        println!();
    }

    #[fastout]
    pub fn print_arr2<T: std::fmt::Debug>(arr: &Array2<T>) {
        for i in 0..arr.nrows() {
            for j in 0..arr.ncols() {
                print!("{:?} ", arr[[i, j]]);
            }
            println!();
        }
    }

    #[fastout]
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
    use itertools::Itertools;
    use proconio::fastout;

    #[fastout]
    pub fn print_vec<T: std::fmt::Debug>(arr: &[T]) {
        for a in arr {
            println!("{:?}", a);
        }
    }

    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Debug>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{:?}", x)).join(" ");
        println!("{}", msg);
    }

    #[fastout]
    pub fn print_vec2<T: std::fmt::Debug>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{:?}", x)).join(" ");
            println!("{}", msg);
        }
    }

    pub fn print_bytes(bytes: &[u8]) {
        let msg = String::from_utf8(bytes.to_vec()).unwrap();
        println!("{}", msg);
    }

    #[fastout]
    pub fn print_vec_bytes(vec_bytes: &[Vec<u8>]) {
        for row in vec_bytes {
            let msg = String::from_utf8(row.to_vec()).unwrap();
            println!("{}", msg);
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
    fn test_print_vec_1line() {
        let arr = vec![1, 2, 3];
        print_vec_1line(&arr);
    }

    #[test]
    fn test_print_vec2() {
        let arr = vec![vec![1, 2, 3], vec![4, 5, 6]];
        print_vec2(&arr);
    }

    #[test]
    fn test_print_bytes() {
        let bs = b"hoge";
        print_bytes(bs);
    }

    #[test]
    fn test_print_vec_bytes() {
        let bs = vec![b"hoge".to_vec(), b"fuga".to_vec()];
        print_vec_bytes(&bs);
    }
}
