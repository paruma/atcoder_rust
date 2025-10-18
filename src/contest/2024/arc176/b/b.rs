#[derive(Debug)]
struct Problem {
    test_cases: Vec<TestCase>,
}

#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct TestCase {
    n: i64,
    m: i64,
    k: i64,
}

impl TestCase {
    fn solve(&self) -> i64 {
        let n = self.n;
        let m = self.m;
        let k = self.k;

        if n >= k && m == k + 1 {
            return 0;
        }

        let mid1 = if n < k { n } else { (n - k) % (m - k) + k };

        [2, 4, 8, 6][(mid1 - 1) as usize % 4]
    }
    fn solve_naive(&self) -> i64 {
        ((1 << self.n) % ((1 << self.m) - (1 << self.k))) % 10
    }
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            test_cases: [TestCase; n],
        }
        Problem { test_cases }
    }
    fn solve(&self) -> Answer {
        let ans = self.test_cases.iter().map(|x| x.solve()).collect_vec();
        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        let ans = self
            .test_cases
            .iter()
            .map(|x| x.solve_naive())
            .collect_vec();
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<i64>,
}

impl Answer {
    fn print(&self) {
        print_vec(&self.ans);
        //println!("{}", self.ans);
    }
}

fn main() {
    Problem::read().solve().print();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }

    fn check(p: &TestCase) {
        assert_eq!(p.solve(), p.solve_naive());
    }

    fn make_random_problem() -> TestCase {
        let mut rng = SmallRng::from_os_rng();
        let n = rng.random_range(1..=15);
        let k = rng.random_range(1..=14);
        let m = rng.random_range((k + 1)..=15);
        let p = TestCase { n, m, k };
        println!("{:?}", &p);
        p
    }

    #[test]
    fn test_with_naive() {
        // 手動でテストを作るのもOK
        for _ in 0..100000 {
            let p = make_random_problem();
            check(&p);
        }
    }
}

// ====== import ======
#[allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet};

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
