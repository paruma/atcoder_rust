#![allow(clippy::let_unit_value)]
use std::cmp::min;

use proconio::input;

//------snippet------

//-------------------

fn read() -> i64 {
    input! {
        //from OnceSource::from(""),
        n: i64
    }
    n
}

fn loglike(k: i64) -> i64 {
    if k == 0 {
        return 0;
    }
    let mut cnt = 1;
    let mut begin = 1;
    let mut end = 11;
    for _ in 0..17 {
        //オーバーフローしそうで怖い
        if begin <= k && k < end {
            return cnt;
        }
        cnt += 1;
        begin = begin * 10 + 1;
        end = end * 10 + 1;
    }
    cnt
}

// 1->1
// 2->11
// 3->111
fn explike(k: i64) -> i64 {
    let mut ans = 1;
    for _ in 0..(k - 1) {
        ans = ans * 10 + 1;
    }
    ans
}

fn solve(n: i64) -> i64 {
    //
    let mut cnt = 0;
    let mut pow10 = 1;
    //let loglike_ans = loglike(n);
    for _d in 1..=16 {
        // d=1の場合
        //let added = loglike(n);
        // d=2の場合
        //let added = (loglike(n / 10) - 1) * 10 + min(n - explike(2) * 10 + 1, 10);
        let loglike_ans = loglike(n / pow10);

        dbg!(loglike_ans);

        dbg!((loglike_ans - 1) * pow10);
        dbg!(min(n - explike(loglike_ans) * pow10 + 1, pow10));

        let added = if loglike_ans == 0 {
            0
        } else {
            (loglike_ans - 1) * pow10 + min(n - explike(loglike_ans) * pow10 + 1, pow10)
        };

        cnt += added;
        pow10 *= 10;
    }
    cnt
}

//fn output() {}

fn main() {
    let n = read();
    let ans = solve(n);
    //output();
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(loglike(1), 1);
        assert_eq!(loglike(2), 1);
        assert_eq!(loglike(9), 1);
        assert_eq!(loglike(10), 1);
        assert_eq!(loglike(11), 2);
        assert_eq!(loglike(12), 2);
        assert_eq!(loglike(110), 2);
        assert_eq!(loglike(111), 3);
        assert_eq!(loglike(112), 3);
        assert_eq!(loglike(1_000_000_000_000_000), 15); //10^15

        assert_eq!(explike(3), 111);
    }
}
