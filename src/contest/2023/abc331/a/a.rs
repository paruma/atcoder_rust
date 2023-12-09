#[derive_readable]
struct Problem {
    n_month: i64,
    n_day: i64,
    year: i64,
    month: i64,
    day: i64,
}

impl Problem {
    fn read() -> Problem {
        input! {
            p: Problem
        }
        p
    }
    fn solve(&self) -> Answer {
        let (next_day, month_carry) =
            if self.day + 1 > self.n_day { (1, 1) } else { (self.day + 1, 0) };

        let (next_month, year_carry) = if self.month + month_carry > self.n_month {
            (1, 1)
        } else {
            (self.month + month_carry, 0)
        };
        let next_year = self.year + year_carry;
        Answer { day: next_day, month: next_month, year: next_year }
    }

    fn solve2(&self) -> Answer {
        // 月末・年末で場合分けする

        let is_year_end = self.month == self.n_month && self.day == self.n_day;
        let is_month_end = self.day == self.n_day;

        if is_year_end {
            Answer { year: self.year + 1, month: 1, day: 1 }
        } else if is_month_end {
            Answer { year: self.year, month: self.month + 1, day: 1 }
        } else {
            Answer { year: self.year, month: self.month, day: self.day + 1 }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    year: i64,
    month: i64,
    day: i64,
}

impl Answer {
    fn print(&self) {
        println!("{} {} {}", self.year, self.month, self.day);
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
