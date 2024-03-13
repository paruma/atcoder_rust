#[derive_readable]
struct Problem {
    a: i64,
    m: i64,
    l: i64,
    r: i64,
}

impl Problem {
    fn read() -> Problem {
        input! {
            p: Problem
        }
        p
    }
    fn solve(&self) -> Answer {
        let a = self.a;
        let m = self.m;
        let l = self.l;
        let r = self.r;

        // aからの相対位置
        let l = l - a;
        let r = r - a;

        let f = |x: i64| {
            if x < 0 {
                0
            } else {
                x / m + 1
            }
        };

        let ans = if l < 0 && r < 0 {
            let cnt1 = f(-l);
            let cnt2 = f(-r - 1);
            cnt1 - cnt2
        } else if l < 0 && r >= 0 {
            let cnt1 = f(-l);
            let cnt2 = f(r);
            cnt1 + cnt2 - 1
        } else {
            // a から l-1
            let cnt1 = f(l - 1);
            // a(=0) から r
            let cnt2 = f(r);

            cnt2 - cnt1
        };

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        let a = self.a;
        let m = self.m;
        let l = self.l;
        let r = self.r;

        //let ans = num_integer::div_floor(r - a, m) - num_integer::div_ceil(l - a, m) + 1;
        // let ans = num_integer::div_floor(r - a, m) - num_integer::div_floor(l - a - 1, m);
        // let ans = num_integer::div_ceil(r - a + 1, m) - num_integer::div_ceil(l - a, m);
        let l = l - a;
        let r = r - a;
        let ans = num_integer::div_floor(r + 1, m) - num_integer::div_ceil(l, m)
            + if (r + 1) % m == 0 { 0 } else { 1 };
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
use num_integer::Integer;
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
