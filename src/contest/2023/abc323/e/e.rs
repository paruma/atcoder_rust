//#[derive_readable]
struct Problem {
    n: usize,
    x: i64,
    len_list: Vec<i64>,
}

use ac_library::ModInt998244353 as Mint;

struct Dp {
    dp: Vec<Mint>,
}

impl Dp {
    fn new(max_time: i64) -> Self {
        Dp {
            dp: vec![Mint::new(0); max_time as usize + 1],
        }
    }

    fn at(&self, time: i64) -> Mint {
        if time < 0 {
            return Mint::new(0);
        }
        self.dp[time as usize]
    }

    fn at_mut(&mut self, time: i64) -> &mut Mint {
        &mut self.dp[time as usize]
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
        let n = self.n;
        let x = self.x;
        let len_list = &self.len_list;  

        // dp[t]: 時刻 t から再生開始する確率
        let mut dp = Dp::new(x);

        let n_inv = Mint::new(n as i64).inv();
        // 時刻0に1曲目を流す
        *dp.at_mut(0) = 1.into();

        for time in 1..=x {
            *dp.at_mut(time) = (0..n)
                .map(|music_i| dp.at(time - len_list[music_i]))
                .sum::<Mint>()
                * n_inv;
        }
        // 音楽 0 の開始時刻が x, x-1, ... , x - len_list[0] + 1 の場合、時刻 x で音楽 0 が流れることになる
        let time_range = x - len_list[0] + 1..=x;
        let ans = time_range.map(|t| dp.at(t)).sum::<Mint>() * n_inv;
        let ans = ans.val() as i64;
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // メモ化再帰
        let n = self.n;
        let x = self.x;
        let len_list = &self.len_list;

        struct Rec {
            dp: Vec<Option<Mint>>,
            n_music_inv: Mint,
            len_list: Vec<i64>, // 各音楽の長さ
        }

        impl Rec {
            fn new(len_list: &[i64], max_time: i64) -> Rec {
                let n_music = len_list.len();
                let n_music_inv = Mint::new(n_music).inv();

                Rec {
                    dp: vec![None; max_time as usize + 1],
                    n_music_inv,
                    len_list: len_list.to_vec(),
                }
            }

            // 時刻 time で音楽が止まる確率
            fn rec(&mut self, time: i64) -> Mint {
                if time < 0 {
                    return Mint::new(0);
                }
                if let Some(ans) = self.dp[time as usize] {
                    return ans;
                }

                let ans = if time == 0 {
                    Mint::new(1)
                } else {
                    (0..self.len_list.len())
                        .map(|music_i| {
                            let len = self.len_list[music_i];
                            self.rec(time - len)
                        })
                        .sum::<Mint>()
                        * self.n_music_inv
                };
                self.dp[time as usize] = Some(ans);
                ans
            }
        }

        let mut rec = Rec::new(len_list, x);

        // 音楽 0 の開始時刻が x, x-1, ... , x - len_list[0] + 1 の場合、時刻 x で音楽 0 が流れることになる
        let time_range = x - len_list[0] + 1..=x;
        let ans = time_range.map(|t| rec.rec(t)).sum::<Mint>() * Mint::new(n).inv();
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
