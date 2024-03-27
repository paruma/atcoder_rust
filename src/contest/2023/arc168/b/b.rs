//#[derive_readable]
struct Problem {
    n: usize,
    xs: Vec<i64>,
}

fn is_win(xs: &[i64], k: i64) -> bool {
    xs.iter()
        .copied()
        .map(|x| x % (k + 1))
        .fold(0, |acc, x| acc ^ x)
        != 0
}
fn is_win_normal(xs: &[i64]) -> bool {
    xs.iter().copied().fold(0, |acc, x| acc ^ x) != 0
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            xs: [i64; n],
        }
        Problem { n, xs }
    }

    fn solve0(&self) -> Option<ExtInt> {
        let xs = &self.xs;

        if is_win_normal(xs) {
            // 普通の Nim 勝てる場合は、k をいくらでも大きくできる
            return Some(Inf);
        }
        // 要素が奇数個ある値のmax
        let max_opt = xs
            .iter()
            .copied()
            .counts()
            .into_iter()
            .filter_map(|(x, c)| (c % 2 == 1).then_some(x))
            .max();

        max_opt.map(|max| Fin(max - 1))
    }
    fn solve(&self) -> Answer {
        Answer { ans: self.solve0() }
    }
}

use ExtInt::*;
#[derive(Clone, Debug, PartialEq, Eq)]
enum ExtInt {
    Inf,
    Fin(i64),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Option<ExtInt>,
}

impl Answer {
    fn print(&self) {
        let msg = match self.ans {
            Some(ExtInt::Fin(k)) => k,
            Some(ExtInt::Inf) => -1, // 条件を満たす k の最大値が存在しない
            None => 0,               // どの k でも勝てない
        };
        println!("{}", msg);
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
    fn test_problem2() {
        dbg!(is_win(&[1, 6, 7], 1));
        dbg!(is_win(&[1, 6, 7], 2));
        dbg!(is_win(&[1, 6, 7], 3));
        dbg!(is_win(&[1, 6, 7], 4));
        dbg!(is_win(&[1, 6, 7], 5));
        dbg!(is_win(&[1, 6, 7], 6));
        dbg!(is_win(&[1, 6, 7], 7));
        dbg!(is_win(&[1, 6, 7], 8));

        dbg!(is_win_normal(&[1, 2, 3, 7, 7]));
        dbg!(is_win(&[1, 2, 3, 7, 7], 1));
        dbg!(is_win(&[1, 2, 3, 7, 7], 2));
        dbg!(is_win(&[1, 2, 3, 7, 7], 3));
        dbg!(is_win(&[1, 2, 3, 7, 7], 4));
        dbg!(is_win(&[1, 2, 3, 7, 7], 5));
        dbg!(is_win(&[1, 2, 3, 7, 7], 6));
        dbg!(is_win(&[1, 2, 3, 7, 7], 7));
        dbg!(is_win(&[1, 2, 3, 7, 7], 8));
        dbg!();
        dbg!(is_win(&[4, 5, 6, 7], 1));
        dbg!(is_win(&[4, 5, 6, 7], 2));
        dbg!(is_win(&[4, 5, 6, 7], 3));
        dbg!(is_win(&[4, 5, 6, 7], 4));
        dbg!(is_win(&[4, 5, 6, 7], 5));
        dbg!(is_win(&[4, 5, 6, 7], 6));
        dbg!(is_win(&[4, 5, 6, 7], 7));
        dbg!(is_win(&[4, 5, 6, 7], 8));
        dbg!(is_win(&[4, 5, 6, 7], 9));
        dbg!();

        dbg!(is_win(&[2, 2, 7, 7], 1));
        dbg!(is_win(&[2, 2, 7, 7], 2));
        dbg!(is_win(&[2, 2, 7, 7], 3));
        dbg!(is_win(&[2, 2, 7, 7], 4));
        dbg!(is_win(&[2, 2, 7, 7], 5));
        dbg!(is_win(&[2, 2, 7, 7], 6));
        dbg!(is_win(&[2, 2, 7, 7], 7));
    }

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
        assert!(is_win_normal(&[1, 2, 3, 4]));
        assert!(!is_win_normal(&[1, 2, 3]));
        assert!(is_win(&[1, 2, 3], 2));
        assert!(!is_win(&[1, 2, 3], 3));
        assert!(!is_win(&[1, 2, 3], 1));
        assert!(is_win(&[3, 4, 7], 4));
        dbg!(is_win(&[3, 4, 7], 1));
        dbg!(is_win(&[3, 4, 7], 2));
        dbg!(is_win(&[3, 4, 7], 3));
        dbg!(is_win(&[3, 4, 7], 4));
        dbg!(is_win(&[3, 4, 7], 5));
        dbg!(is_win(&[3, 4, 7], 6));
        dbg!(is_win(&[3, 4, 7], 7));
        dbg!();

        dbg!(is_win(&[2, 4, 6], 1));
        dbg!(is_win(&[2, 4, 6], 2));
        dbg!(is_win(&[2, 4, 6], 3));
        dbg!(is_win(&[2, 4, 6], 4));
        dbg!(is_win(&[2, 4, 6], 5));
        dbg!(is_win(&[2, 4, 6], 6));
        dbg!(is_win(&[2, 4, 6], 7));
        dbg!();

        dbg!(is_win(&[2, 5, 7], 1));
        dbg!(is_win(&[2, 5, 7], 2));
        dbg!(is_win(&[2, 5, 7], 3));
        dbg!(is_win(&[2, 5, 7], 4));
        dbg!(is_win(&[2, 5, 7], 5));
        dbg!(is_win(&[2, 5, 7], 6));
        dbg!(is_win(&[2, 5, 7], 7));
        dbg!();
        dbg!(is_win(&[1, 6, 7], 1));
        dbg!(is_win(&[1, 6, 7], 2));
        dbg!(is_win(&[1, 6, 7], 3));
        dbg!(is_win(&[1, 6, 7], 4));
        dbg!(is_win(&[1, 6, 7], 5));
        dbg!(is_win(&[1, 6, 7], 6));
        dbg!(is_win(&[1, 6, 7], 7));
        dbg!(is_win(&[1, 6, 7], 8));
        dbg!();
        dbg!(is_win(&[2, 3, 4, 5], 1));
        dbg!(is_win(&[2, 3, 4, 5], 2));
        dbg!(is_win(&[2, 3, 4, 5], 3));
        dbg!(is_win(&[2, 3, 4, 5], 4));
        dbg!(is_win(&[2, 3, 4, 5], 5));
    }
}

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
