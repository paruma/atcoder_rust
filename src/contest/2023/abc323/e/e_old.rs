//#[derive_readable]
struct Problem {
    n: usize,
    x: i64,
    len_list: Vec<i64>,
}

use ac_library::ModInt998244353 as Mint;

struct Dp {
    dp: Vec<Vec<Mint>>,
}
impl Dp {
    fn new(n: usize, x: i64) -> Dp {
        Dp {
            dp: vec![vec![Mint::new(0); n]; x as usize + 1],
        }
    }

    // 時刻 time で音楽 music が再生開始する確率: dp.at(time, music)
    fn at(&self, time: i64, music: usize) -> Mint {
        if time < 0 {
            return Mint::new(0);
        }
        self.dp[time as usize][music]
    }

    fn add(&mut self, time: i64, music: usize, value: Mint) {
        self.dp[time as usize][music] += value;
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
        // 配るDPによるもの (って思ったが、よく見ると普通にもらうDPをしている)
        let Problem { n, x, len_list } = self;

        // 時刻 t で音楽 i が再生開始する確率: dp.at(t, i)
        let mut dp = Dp::new(*n, *x);
        // 時刻0に1曲目を流す
        for music_i in 0..*n {
            let prob = Mint::new(*n).inv(); // 1/n
            dp.add(0, music_i, prob);
        }

        for time in 1..=*x {
            // 時刻 time - 1 で音楽が止まる確率を求める
            // 各音楽 music_i に対して、時刻 time - len_list[music_i] で開始した確率を求めて、足し合わせる
            let prob_stop: Mint = (0..*n)
                .map(|music_i| dp.at(time - len_list[music_i], music_i))
                .sum();

            // 時刻t から各音楽を流す
            for music_i in 0..*n {
                let prob = Mint::new(*n).inv() * prob_stop;
                dp.add(time, music_i, prob);
            }
        }
        // 音楽 0 の開始時刻が x, x-1, ... , x - len_list[0] + 1 の場合、時刻 x で音楽 0 が流れることになる
        let time_range = *x - len_list[0] + 1..=*x;
        let ans = time_range.map(|t| dp.at(t, 0)).sum::<Mint>();
        let ans = ans.val() as i64;
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // メモ化再帰 (TLE)
        let Problem { n, x, len_list } = self;

        struct Rec {
            dp: Vec<Vec<Option<Mint>>>,
            n_music_inv: Mint,
            len_list: Vec<i64>, // 各音楽の長さ
        }

        impl Rec {
            fn new(len_list: &[i64], max_time: i64) -> Rec {
                let n_music = len_list.len();
                let n_music_inv = Mint::new(n_music).inv();

                Rec {
                    dp: vec![vec![None; n_music]; max_time as usize + 1],
                    n_music_inv,
                    len_list: len_list.to_vec(),
                }
            }
            fn rec(&mut self, time: i64, music_idx: usize) -> Mint {
                println!("{} {}", time, music_idx);
                if time < 0 {
                    return Mint::new(0);
                }
                if let Some(ans) = self.dp[time as usize][music_idx] {
                    return ans;
                }

                // 時刻 time で音楽が止まる確率
                let prob_stop = if time == 0 {
                    Mint::new(1)
                } else {
                    // これだと「cannot borrow `*self` as mutable because it is also borrowed as immutable」でエラーになる
                    // for (music_i, len) in self.len_list.iter().enumerate() {
                    //     sum += self.rec(time - len, music_i)
                    // }

                    // ここで各 music_i に対して音楽の数の分だけ遷移することになっている(つまり音楽の数の2乗回の遷移が発生してしまっている)
                    (0..self.len_list.len())
                        .map(|music_i| {
                            let len = self.len_list[music_i];
                            self.rec(time - len, music_i)
                        })
                        .sum::<Mint>()
                };
                let ans = prob_stop * self.n_music_inv;
                self.dp[time as usize][music_idx] = Some(ans);
                ans
            }
        }

        let mut rec = Rec::new(len_list, *x);

        // 音楽 0 の開始時刻が x, x-1, ... , x - len_list[0] + 1 の場合、時刻 x で音楽 0 が流れることになる
        let time_range = *x - len_list[0] + 1..=*x;
        let ans = time_range.map(|t| rec.rec(t, 0)).sum::<Mint>();
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
    Problem::read().solve2().print();
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_problem() {
        let x: Rational64 = Rational64::new(1, 3);
        let y: Rational64 = Rational64::new(1, 2);
        let z: Rational64 = 3.into();
        assert_eq!(z, Rational64::new(3, 1));
    }
}

// ====== import ======
#[allow(unused_imports)]
use itertools::Itertools;
use num_rational::Rational64;
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
