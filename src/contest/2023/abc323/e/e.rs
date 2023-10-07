//#[derive_readable]
struct Problem {
    n: usize,
    x: i64,
    len_list: Vec<i64>,
}

struct Dp {
    dp: Vec<Vec<Vec<RF>>>,
    max_time: i64,
}
impl Dp {
    fn new(n: usize, x: i64) -> Dp {
        Dp { dp: vec![vec![vec![RF::zero(); n + 1]; x as usize + 1]; 2], max_time: x }
    }

    fn at(&self, time: i64, music: usize, stop: bool) -> &RF {
        let stop_idx = stop as usize;
        // 場合分け必要かも
        &self.dp[stop_idx][time as usize][music]
    }

    fn set(&mut self, time: i64, music: usize, stop: bool, value: RF) {
        let stop_idx = stop as usize;
        if time <= self.max_time {
            self.dp[stop_idx][time as usize][music] = value;
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
            let prob = RF::one() / RF::new(*n as i64);
            for t in 0..music_len - 1 {
                dp.set(t, music_i, false, prob);
            }
            dp.set(music_len - 1, music_i, true, prob);
            p_queue.push((Reverse(music_len), music_i));
        }

        while let Some((Reverse(time), prev_music_i)) = p_queue.pop() {
            for music_i in 0..*n {
                let music_len = len_list[music_i];
                let prob = RF::one() / RF::new(*n as i64) * *dp.at(time - 1, prev_music_i, false);
                for t in 0..music_len - 1 {
                    dp.set(time + t, music_i, false, prob);
                }
                dp.set(time + music_len - 1, music_i, true, prob);
                if time + music_len <= *x && !visited[(time + music_len) as usize][music_i] {
                    p_queue.push((Reverse(time + music_len), music_i));
                    visited[(time + music_len) as usize][music_i] = true;
                }
            }
        }

        let ans = *dp.at(*x, 0, false) + *dp.at(*x, 0, true);
        let ans = ans.rep();
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
use num::{One, Zero};
use rf::*;
pub mod rf {
    pub const MOD: i64 = 998_244_353;
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct RF {
        rep: i64,
    }
    impl RF {
        pub fn new(x: i64) -> RF {
            RF { rep: x.rem_euclid(MOD) }
        }
        pub fn rep(self) -> i64 {
            self.rep
        }
    }
    impl RF {
        pub fn inv(self) -> Self {
            num::pow(self, (MOD - 2) as usize)
        }
    }
    impl num_traits::Zero for RF {
        fn zero() -> Self {
            RF::new(0)
        }
        fn is_zero(&self) -> bool {
            self.rep == 0
        }
    }
    impl num_traits::One for RF {
        fn one() -> Self {
            RF::new(1)
        }
    }
    macro_rules ! bi_ops_impl {($ std_ops : ident , $ fn : ident , $ op : tt ) => {impl std :: ops ::$ std_ops for RF {type Output = Self ; fn $ fn (self , rhs : Self ) -> Self :: Output {RF :: new (self . rep $ op rhs . rep ) } } } ; }
    bi_ops_impl ! (Add , add , + );
    bi_ops_impl ! (Sub , sub , - );
    bi_ops_impl ! (Mul , mul , * );
    impl std::ops::Div for RF {
        type Output = Self;
        fn div(self, rhs: Self) -> Self::Output {
            std::ops::Mul::mul(self, rhs.inv())
        }
    }
    macro_rules ! bi_ops_assign_impl {($ std_ops_assign : ident , $ fn_assign : ident , $ op : tt ) => {impl std :: ops ::$ std_ops_assign for RF {fn $ fn_assign (& mut self , rhs : Self ) {* self = * self $ op rhs } } } ; }
    bi_ops_assign_impl ! (AddAssign , add_assign , + );
    bi_ops_assign_impl ! (SubAssign , sub_assign , - );
    bi_ops_assign_impl ! (MulAssign , mul_assign , * );
    bi_ops_assign_impl ! (DivAssign , div_assign , / );
    impl std::ops::Neg for RF {
        type Output = Self;
        fn neg(self) -> Self::Output {
            RF::new(-self.rep)
        }
    }
}
