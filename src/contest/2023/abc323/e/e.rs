//#[derive_readable]
struct Problem {
    n: usize,
    x: i64,
    len_list: Vec<i64>,
}

use ac_library::ModInt998244353 as Mint;

struct Dp {
    dp: Vec<Vec<Vec<Mint>>>,
    max_time: i64,
}
impl Dp {
    fn new(n: usize, x: i64) -> Dp {
        Dp { dp: vec![vec![vec![Mint::new(0); n + 1]; x as usize + 1]; 2], max_time: x }
    }

    fn at(&self, time: i64, music: usize, stop: bool) -> &Mint {
        let stop_idx = stop as usize;
        // 場合分け必要かも
        &self.dp[stop_idx][time as usize][music]
    }

    fn add(&mut self, time: i64, music: usize, stop: bool, value: Mint) {
        let stop_idx = stop as usize;
        if time <= self.max_time {
            self.dp[stop_idx][time as usize][music] += value;
        }
    }
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            x: i64,
            len_list: [i64;n],
        }
        Problem { n, x, len_list }
    }
    fn solve(&self) -> Answer {
        let Problem { n, x, len_list } = self;
        // 時刻 t に音楽 i が流れている確率
        // 時刻tで止まる
        // 時刻tで止まらない
        let mut dp = Dp::new(*n, *x);
        // 1曲目を流す
        let mut p_queue: BinaryHeap<(Reverse<i64>, usize)> = BinaryHeap::new();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; *n + 1]; *x as usize + 1];
        for music_i in 0..*n {
            let music_len = len_list[music_i];
            let prob = Mint::new(*n as i64).inv();
            dp.add(music_len - 1, music_i, true, prob);
            if music_len <= *x && !visited[music_len as usize][music_i] {
                p_queue.push((Reverse(music_len), music_i));
                visited[music_len as usize][music_i] = true;
            }
        }

        for time in 1..=*x {
            // 時刻 t-1 で止まっている確率 O(n) で求まる
            let prob_stop: Mint = (0..*n).map(|music_i| dp.at(time - 1, music_i, true)).sum();

            // 時刻t から各音楽を流す
            for music_i in 0..*n {
                let music_len = len_list[music_i];
                let prob = Mint::new(*n as i64).inv() * prob_stop;
                dp.add(time + music_len - 1, music_i, true, prob);
            }
        }
        //TODO: false はいらないので消す

        // 時刻がx のもの、x-1 のもの...
        let ans = *dp.at(*x, 0, false) + *dp.at(*x, 0, true);
        let ans = ans.val() as i64;
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: i64,
}

impl Answer {
    fn print(&self) {
        println!("{}", self.ans);
    }
}

fn main() {
    Problem::read().solve().print();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }
}

use std::{cmp::Reverse, collections::BinaryHeap};

// ====== import ======
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};

// ====== output func ======
#[allow(unused_imports)]
use print_vec::*;
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

#[allow(unused)]
fn print_yesno(ans: bool) {
    let msg = if ans { "Yes" } else { "No" };
    println!("{}", msg);
}

// ====== snippet ======
