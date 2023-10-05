#[derive_readable]
struct Problem {
    a: i64,
    b: i64,
}

impl Problem {
    fn read() -> Problem {
        input! {
            a: i64,
            b: i64,
        }
        Problem { a, b }
    }
    fn solve(&self) -> Answer {
        let ans = self.a + self.b;
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: i64,
}

impl Answer {
    #[fastout]
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

// ====== import ======
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};

// ====== snippet ======
