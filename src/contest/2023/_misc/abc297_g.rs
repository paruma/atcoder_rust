//#[derive_readable]
struct Problem {
    n: usize,
    l: i64,
    r: i64,
    xs: Vec<i64>,
}

fn mex(xs: &[i64]) -> i64 {
    let xs_set = xs.iter().copied().collect::<HashSet<_>>();
    (0..).find(|x| !xs_set.contains(x)).unwrap()
}

struct Nim {
    l: i64,
    r: i64,
}
impl Nim {
    fn new(l: i64, r: i64) -> Nim {
        Nim { l, r }
    }

    fn grundy(&self, x: i64) -> i64 {
        let l = self.l;
        let r = self.r;
        let modulo = l + r;
        (x % modulo) / l
    }

    fn grundy_naive(&self, x: i64, memo: &mut HashMap<i64, i64>) -> i64 {
        if let Some(ans) = memo.get(&x) {
            return *ans;
        }
        let next_grundy = (self.l..=self.r)
            .map(|i| x - i)
            .filter(|next| *next >= 0)
            .map(|next| self.grundy_naive(next, memo))
            .collect_vec();
        let ans = mex(&next_grundy);

        memo.insert(x, ans);
        ans
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
        let nim = Nim::new(self.l, self.r);
        let ans = self
            .xs
            .iter()
            .copied()
            .map(|x| nim.grundy(x))
            .fold(0, |acc, x| acc ^ x)
            != 0;
        Answer { ans }
    }

    fn solve_naive(&self) -> Answer {
        let nim = Nim::new(self.l, self.r);
        let mut memo = HashMap::new();
        let ans = self
            .xs
            .iter()
            .copied()
            .map(|x| nim.grundy_naive(x, &mut memo))
            .fold(0, |acc, x| acc ^ x)
            != 0;
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
        let nim = Nim::new(2, 4);
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

    #[test]
    fn test_problem_naive() {
        let nim = Nim::new(2, 4);
        let mut memo = HashMap::new();
        assert_eq!(nim.grundy_naive(0, &mut memo), 0);
        assert_eq!(nim.grundy_naive(1, &mut memo), 0);
        assert_eq!(nim.grundy_naive(2, &mut memo), 1);
        assert_eq!(nim.grundy_naive(3, &mut memo), 1);
        assert_eq!(nim.grundy_naive(4, &mut memo), 2);
        assert_eq!(nim.grundy_naive(5, &mut memo), 2);
        assert_eq!(nim.grundy_naive(6, &mut memo), 0);
        assert_eq!(nim.grundy_naive(7, &mut memo), 0);
        assert_eq!(nim.grundy_naive(8, &mut memo), 1);
    }
}

use std::collections::{HashMap, HashSet};

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
