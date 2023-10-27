//#[derive_readable]
struct Problem {
    n_toast: usize,
    n_plate: usize,
    deliciousness_list: Vec<i64>,
}

fn sq(n: i64) -> i64 {
    n * n
}

impl Problem {
    fn read() -> Problem {
        input! {
            n_toast: usize,
            n_plate: usize,
            deliciousness_list: [i64; n_toast],
        }
        Problem { n_toast, n_plate, deliciousness_list }
    }
    fn solve(&self) -> Answer {
        let deliciousness_list = self.deliciousness_list.iter().copied().sorted().collect_vec();
        let n_double = self.n_toast - self.n_plate; // 2個乗せる皿の枚数

        // 2このせ
        let sum1: i64 = (0..n_double)
            .map(|i| sq(deliciousness_list[i] + deliciousness_list[2 * n_double - i - 1]))
            .sum();

        // 1個のせ
        let sum2: i64 = (n_double * 2..self.n_toast).map(|i| sq(deliciousness_list[i])).sum();

        let ans = sum1 + sum2;
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        let n_plate = self.n_plate;
        // 美味しさ0のトーストを用意する

        let deliciousness_list = {
            let mut deliciousness_list = self.deliciousness_list.clone();
            // 2 * n_plate になるように 0 を埋める。
            deliciousness_list.resize(2 * n_plate, 0);
            deliciousness_list.sort();
            deliciousness_list
        };

        let ans: i64 = (0..self.n_plate)
            .map(|i| sq(deliciousness_list[i] + deliciousness_list[2 * n_plate - i - 1]))
            .sum();

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
        assert_eq!(1 + 1, 2);
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
