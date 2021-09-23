#![allow(clippy::let_unit_value)]
use std::cmp::{max, min};

use itertools::Itertools;
use ndarray::{Array, Array3};
use proconio::input;

//------snippet------
use tropical::Trop::{self, *};
pub mod tropical {
    use std::{cmp::Ordering, ops::Add};
    use Trop::*;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Trop {
        Inf,
        Fin(i64),
    }
    impl Trop {
        pub fn get_fin(self) -> i64 {
            match self {
                Fin(val) => val,
                Inf => panic!("called `Trop::get_fin()` on a `Fin` value"),
            }
        }
        pub fn get_fin_or(self, default: i64) -> i64 {
            match self {
                Fin(val) => val,
                Inf => default,
            }
        }
        pub fn is_fin(self) -> bool {
            matches!(self, Fin(_))
        }
        pub fn is_inf(self) -> bool {
            matches!(self, Inf)
        }
        pub fn to_option(self) -> Option<i64> {
            match self {
                Inf => None,
                Fin(a) => Some(a),
            }
        }
    }
    impl Add for Trop {
        type Output = Trop;
        fn add(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (Inf, Inf) => Inf,
                (Inf, Fin(_)) => Inf,
                (Fin(_), Inf) => Inf,
                (Fin(a), Fin(b)) => Fin(a + b),
            }
        }
    }
    impl PartialOrd for Trop {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match (self, other) {
                (Inf, Inf) => Some(Ordering::Equal),
                (Inf, Fin(_)) => Some(Ordering::Greater),
                (Fin(_), Inf) => Some(Ordering::Less),
                (Fin(a), Fin(b)) => PartialOrd::partial_cmp(a, b),
            }
        }
    }
    impl Ord for Trop {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }
}

//use print_arr::*;
pub mod print_arr {
    use ndarray::{Array2, Array3};
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
//-------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Lb {
    a: i64,
    b: i64,
}

fn read() -> (usize, i64, i64, Vec<Lb>) {
    input! {
        //from OnceSource::from(""),
        n: usize,
        x:i64, y:i64,
        lb_info: [(i64, i64); n],
    }
    let lbs = lb_info
        .iter()
        .map(|(a, b)| Lb { a: *a, b: *b })
        .collect_vec();
    (n, x, y, lbs)
}

fn solve(n: usize, xx: i64, yy: i64, lbs: &[Lb]) -> Option<i64> {
    let xx = xx as usize;
    let yy = yy as usize;
    let mut dp: Array3<Trop> = Array::from_shape_fn((n + 1, xx + 1, yy + 1), |_| Inf);

    // usize: 引き算に注意
    dp[[0, 0, 0]] = Fin(0);

    for (i, lb) in lbs.iter().enumerate() {
        for x in 0..=xx {
            for y in 0..=yy {
                let prev_x = max(x as i64 - lb.a, 0) as usize;
                let prev_y = max(y as i64 - lb.b, 0) as usize;
                dp[[i + 1, x, y]] = min(dp[[i, x, y]], dp[[i, prev_x, prev_y]] + Fin(1));
            }
        }
    }

    dp[[n, xx, yy]].to_option()
}

fn main() {
    let (n, x, y, lbs) = read();
    let ans = solve(n, x, y, &lbs);
    println!("{}", ans.unwrap_or(-1));
}
