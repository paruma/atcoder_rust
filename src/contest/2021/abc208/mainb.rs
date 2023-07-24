#![allow(clippy::let_unit_value)]
use proconio::input;

fn read() -> i64 {
    input! {p:i64}
    p
}

pub mod math_tools {
    use num::Integer;
    pub fn divisor(n: i64) -> Vec<i64> {
        assert!(n >= 1);
        let mut retval: Vec<i64> = Vec::new();
        for i in 1..=num_integer::sqrt(n) {
            if n.is_multiple_of(&i) {
                retval.push(i);
                if i * i != n {
                    retval.push(n / i);
                }
            }
        }
        retval
    }
    fn frac0<T>(n: T, acc: T) -> T
    where
        T: std::ops::Sub<Output = T> + std::ops::Mul + num::Zero + num::One + Copy,
    {
        if n.is_zero() {
            acc
        } else {
            frac0(n - T::one(), n * acc)
        }
    }
    pub fn frac<T>(n: T) -> T
    where
        T: std::ops::Sub<Output = T> + std::ops::Mul + num::Zero + num::One + Copy,
    {
        frac0(n, T::one())
    }
    pub fn permutation<T>(n: T, k: T) -> T
    where
        T: std::ops::Sub<Output = T>
            + std::ops::Mul
            + std::ops::Div<Output = T>
            + num::Zero
            + num::One
            + Copy,
    {
        frac(n) / frac(n - k)
    }
    pub fn comb<T>(n: T, k: T) -> T
    where
        T: std::ops::Sub<Output = T>
            + std::ops::Mul
            + std::ops::Div<Output = T>
            + num::Zero
            + num::One
            + Copy,
    {
        frac(n) / frac(n - k) / frac(k)
    }
}

fn solve(p: i64) -> i64 {
    use math_tools::frac;
    let coins = (1_i64..=10_i64).map(frac).rev().collect::<Vec<_>>();

    let mut p = p;
    let mut n_coins = 0;
    for coin in coins {
        n_coins += p / coin;
        p %= coin;
    }
    n_coins
}

//fn output() {}

fn main() {
    let p = read();
    let ans = solve(p);
    println!("{}", ans);
}
