//#[derive_readable]
struct Problem {
    n: usize,
    l: i64,
    r: i64,
    xs: Vec<i64>,
}

struct Nim {
    l: i64,
    r: i64,
}
impl Nim {
    fn grundy(&self, x: i64) -> i64 {
        let l = self.l;
        let r = self.r;
        let modulo = l + r;
        (x % modulo) / l
    }
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            l: i64,
            r: i64,
            xs: [i64; n],
        }
        Problem { n, l, r, xs }
    }

    fn solve(&self) -> Answer {
        let nim = Nim { l: self.l, r: self.r };
        let ans = self.xs.iter().copied().map(|x| nim.grundy(x)).fold(0, |acc, x| acc ^ x) != 0;
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: bool,
}

impl Answer {
    fn print(&self) {
        // 先手が勝つ場合true
        let msg = if self.ans { "First" } else { "Second" };
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
    fn test_problem() {
        let nim = Nim { l: 2, r: 4 };
        assert_eq!(nim.grundy(0), 0);
        assert_eq!(nim.grundy(1), 0);
        assert_eq!(nim.grundy(2), 1);
        assert_eq!(nim.grundy(3), 1);
        assert_eq!(nim.grundy(4), 2);
        assert_eq!(nim.grundy(5), 2);
        assert_eq!(nim.grundy(6), 0);
        assert_eq!(nim.grundy(7), 0);
        assert_eq!(nim.grundy(8), 1);
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
