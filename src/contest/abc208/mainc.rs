#![allow(clippy::let_unit_value)]
use proconio::input;

fn read() -> (i64, i64, Vec<i64>) {
    input! {n:i64,k:i64,a:[i64;n]}
    (n, k, a)
}

fn solve(n: i64, k: i64, a: &[i64]) -> Vec<i64> {
    let mut a_enu = a.to_vec().into_iter().enumerate().collect::<Vec<_>>();
    a_enu.sort_by_key(|(_, x)| *x);
    let rem = k % n;
    let mut ans = vec![k / n; n as usize];
    for (i, _) in a_enu.iter().take(rem as usize) {
        ans[*i] += 1;
    }
    ans
}

fn output(ans: &[i64]) {
    for e in ans {
        println!("{}", *e);
    }
}

fn main() {
    let (n, k, a) = read();
    let ans = solve(n, k, &a);
    output(&ans);
}
