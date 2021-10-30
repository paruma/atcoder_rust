#![allow(clippy::let_unit_value)]
use proconio::input;

//------snippet------
use scan_vec::*;
pub mod scan_vec {
    pub fn scanl<A, B, F>(vec: &[A], init: B, mut f: F) -> Vec<B>
    where
        F: FnMut(&mut B, &A) -> B,
        B: Copy,
    {
        let mut ret: Vec<B> = Vec::new();
        let mut acc = init;
        ret.push(acc);
        for x in vec {
            acc = f(&mut acc, &x);
            ret.push(acc);
        }
        ret
    }
    pub fn scanr<A, B, F>(vec: &[A], init: B, f: F) -> Vec<B>
    where
        F: FnMut(&mut B, &A) -> B,
        A: Clone,
        B: Copy,
    {
        let vec2 = vec.to_vec().into_iter().rev().collect::<Vec<A>>();
        let vec3 = scanl(&vec2, init, f);
        vec3.to_vec().into_iter().rev().collect::<Vec<B>>()
    }
    pub fn cumsum<T>(vec: &[T]) -> Vec<T>
    where
        T: std::ops::Add + num::Zero + Copy,
    {
        scanl(vec, T::zero(), |acc, x| *acc + *x)
    }
    pub struct CumSum<T>
    where
        T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + num::Zero + Copy,
    {
        cumsum: Vec<T>,
    }
    impl<T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + num::Zero + Copy> CumSum<T> {
        pub fn make(vec: &[T]) -> CumSum<T> {
            CumSum {
                cumsum: cumsum(vec),
            }
        }
        pub fn partial_sum(&self, begin: usize, end: usize) -> T {
            self.cumsum[end] - self.cumsum[begin]
        }
    }
}

pub fn bin_search<F>(mut ok: i64, mut ng: i64, mut p: F) -> i64
where
    F: FnMut(i64) -> bool,
{
    while num::abs(ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        if p(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}
pub fn lower_bound<T>(xs: &[T], key: T) -> i64
where
    T: Ord,
{
    let pred = |i: i64| key <= xs[i as usize];
    bin_search(xs.len() as i64, -1 as i64, pred)
}
//-------------------

fn read() -> (usize, Vec<i64>, i64) {
    input! {
        //from OnceSource::from(""),
        n: usize,
        a: [i64; n],
        x: i64
    }
    let ret: (usize, Vec<i64>, i64) = (n, a, x);
    ret
}

#[allow(clippy::many_single_char_names)]
fn solve(n: usize, a: &[i64], x: i64) -> i64 {
    let sum_a: i64 = a.iter().sum();
    let cumsum_a = cumsum(a);
    let y = x + 1; //y以上になる最小のkを求める

    let nloops = y / sum_a;

    let i = lower_bound(&cumsum_a, y % sum_a);

    // 答えは1オリジン
    nloops * (n as i64) + i
}

fn main() {
    let (n, a, x) = read();
    let ans = solve(n, &a, x);
    //output(ans);
    println!("{}", ans);
}
