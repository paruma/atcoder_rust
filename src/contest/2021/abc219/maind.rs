#![allow(clippy::let_unit_value)]
use std::cmp::{max, min};

use itertools::Itertools;
use ndarray::{Array, Array2};
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
//-------------------

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

fn solve(_n: usize, xx: i64, yy: i64, lbs: &[Lb]) -> Option<i64> {
    let xx = xx as usize;
    let yy = yy as usize;
    let mut dp: Array2<Trop> = Array::from_shape_fn((xx + 1, yy + 1), |_| Inf);
    let mut dp_prev: Array2<Trop> = Array::from_shape_fn((xx + 1, yy + 1), |_| Inf);

    //let is_within = |x: i64, y: i64| 0 <= x && x <= (xx as i64) && 0 <= y && y <= (yy as i64);
    //let is_within = |x: usize, y: usize| x <= xx && y <= yy;

    //

    // usize: 引き算に注意
    dp_prev[[0, 0]] = Fin(0);
    /*
        for x in 0..=xx {
            for y in 0..=yy {
                if (x, y) == (0, 0) {
                    continue;
                }
                for lb in lbs {
                    let prev_x = max(x as i64 - lb.a, 0);
                    let prev_y = max(y as i64 - lb.b, 0);
                    if is_within(prev_x, prev_y) {
                        let prev_x = prev_x as usize;
                        let prev_y = prev_y as usize;
                        dp[[x, y]] = min(dp[[prev_x, prev_y]] + Fin(1), dp[[x, y]])
                    }
                }
            }
        }
    */

    for lb in lbs {
        for x in 0..=xx {
            for y in 0..=yy {
                let prev_x = max(x as i64 - lb.a, 0) as usize;
                let prev_y = max(y as i64 - lb.b, 0) as usize;
                dp[[x, y]] = min(dp_prev[[x, y]], dp_prev[[prev_x, prev_y]] + Fin(1));
            }
        }

        dp_prev = dp.clone();
    }

    //dbg!(dp.clone());
    /*
        for y in 0..=yy {
            for x in 0..=xx {
                print!("{:?} ", dp[[x, y]])
            }
            println!();
        }
    */

    match dp[[xx, yy]] {
        Inf => None,
        Fin(a) => Some(a),
    }
}
// 3次元配列作るのが正解だった。
//fn output() {}

fn main() {
    let (n, x, y, lbs) = read();
    let ans = solve(n, x, y, &lbs);
    //output();
    println!("{}", ans.unwrap_or(-1));
}
