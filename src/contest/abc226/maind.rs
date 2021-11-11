#![allow(clippy::let_unit_value)]
use itertools::Itertools;

use proconio::input;

//------snippet------
use pos::*;
pub mod pos {
    use std::ops::{Add, Sub};
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Pos<T> {
        pub x: T,
        pub y: T,
    }
    impl<T> Pos<T> {
        pub fn new(x: T, y: T) -> Pos<T> {
            Pos { x, y }
        }
    }
    impl<T: Add<Output = T> + Copy> Add for Pos<T> {
        type Output = Pos<T>;
        fn add(self, rhs: Self) -> Self::Output {
            Pos::new(self.x + rhs.x, self.y + rhs.y)
        }
    }
    impl<T: Sub<Output = T> + Copy> Sub for Pos<T> {
        type Output = Pos<T>;
        fn sub(self, rhs: Self) -> Self::Output {
            Pos::new(self.x - rhs.x, self.y - rhs.y)
        }
    }
    impl<T: num_traits::Zero + Copy> num_traits::Zero for Pos<T> {
        fn zero() -> Self {
            Pos::new(T::zero(), T::zero())
        }
        fn is_zero(&self) -> bool {
            self.x.is_zero() && self.y.is_zero()
        }
    }
}

//-------------------

type IPos = Pos<i64>;

fn read() -> Vec<IPos> {
    input! {
        n: usize,
        xys: [(i64, i64);n]
    }
    xys.iter().map(|(x, y)| Pos::new(*x, *y)).collect_vec()
}

fn normalize(p: IPos) -> IPos {
    if p.x == 0 {
        Pos::new(0, p.y.signum())
    } else if p.y == 0 {
        Pos::new(p.x.signum(), 0)
    } else {
        let gcd = num_integer::gcd(p.x.abs(), p.y.abs());
        Pos::new(p.x / gcd, p.y / gcd)
    }
}

fn solve(pos_list: &[IPos]) -> usize {
    let mut buf: Vec<IPos> = Vec::new();

    // ここ、最初順列ではなく組み合わせと勘違い
    for i in 0..pos_list.len() {
        for j in 0..pos_list.len() {
            if i != j {
                let src = pos_list[i];
                let dst = pos_list[j];
                let diff = dst - src;
                let norm_diff = normalize(diff);
                buf.push(norm_diff)
            }
        }
    }
    buf.iter().unique().count()
}

//fn output() {}

fn main() {
    let pos_list = read();
    let ans = solve(&pos_list);
    //output();
    println!("{}", ans);
}
