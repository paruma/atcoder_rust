//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    s: Vec<u8>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            s: Bytes,
        }
        Problem { s }
    }
    fn solve(&self) -> Answer {
        // '('を右、')'を上に対応させて経路探索問題をする
        use ac_library::ModInt998244353 as Mint;
        let n = self.s.len();
        if n % 2 == 1 {
            return Answer { ans: 0 };
        }
        assert!(n % 2 == 0);
        let mut dp = vec![vec![Mint::new(0); n / 2 + 1]; n / 2 + 1];
        dp[0][0] = Mint::new(1);
        for y in 0..=n / 2 {
            for x in 0..=n / 2 {
                if x < y {
                    continue;
                }
                if x == 0 && y == 0 {
                    continue;
                }
                // 配列外に気をつける
                let from_left = if x == 0 || self.s[x + y - 1] == b')' {
                    Mint::new(0)
                } else {
                    dp[y][x - 1]
                };
                let from_bottom = if y == 0 || self.s[x + y - 1] == b'(' {
                    Mint::new(0)
                } else {
                    dp[y - 1][x]
                };
                dp[y][x] = from_left + from_bottom;
            }
        }
        let ans = dp[n / 2][n / 2].val() as i64;
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // 括弧列を↗↘で考える
        use ac_library::ModInt998244353 as Mint;
        struct Dp {
            dp: Vec<Vec<Mint>>,
            len: usize,
        }
        impl Dp {
            fn new(len: usize) -> Self {
                Dp {
                    dp: vec![vec![Mint::new(0); len + 1]; len + 1],
                    len,
                }
            }

            fn at(&self, x: i64, y: i64) -> Mint {
                if y < 0 || y > self.len as i64 {
                    Mint::new(0)
                } else {
                    self.dp[y as usize][x as usize]
                }
            }
            fn at_mut(&mut self, x: i64, y: i64) -> &mut Mint {
                &mut self.dp[y as usize][x as usize]
            }
        }

        let mut dp = Dp::new(self.s.len());
        *dp.at_mut(0, 0) = Mint::new(1);
        for (i, ch) in self.s.iter().enumerate() {
            // 左上から来たか、左下から来たかを0,1で表現
            let (from_upper_coef, from_bottom_coef) = match ch {
                b'(' => (0, 1),
                b')' => (1, 0),
                b'?' => (1, 1),
                _ => panic!(),
            };
            let from_upper_coef = Mint::new(from_upper_coef);
            let from_bottom_coef = Mint::new(from_bottom_coef);
            let i = i as i64;
            for y in 0..=self.s.len() as i64 {
                *dp.at_mut(i + 1, y) =
                    from_bottom_coef * dp.at(i, y - 1) + from_upper_coef * dp.at(i, y + 1);
            }
        }
        let ans = dp.at(self.s.len() as i64, 0).val() as i64;
        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        todo!();
        // let ans = 0;
        // Answer { ans }
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
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    struct WrongTestCase {
        problem: Problem,
        main_ans: Answer,
        naive_ans: Answer,
    }

    #[allow(dead_code)]
    fn check(p: &Problem) -> Option<WrongTestCase> {
        let main_ans = p.solve();
        let naive_ans = p.solve_naive();
        if main_ans != naive_ans {
            Some(WrongTestCase {
                problem: p.clone(),
                main_ans,
                naive_ans,
            })
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn make_random_problem(rng: &mut SmallRng) -> Problem {
        todo!()
        // let n = rng.random_range(1..=10);
        // let p = Problem { _a: n };
        // println!("{:?}", &p);
        // p
    }

    #[allow(unreachable_code)]
    #[test]
    fn test_with_naive() {
        let num_tests = 0;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut rng = SmallRng::seed_from_u64(42);
        // let mut rng = SmallRng::from_os_rng();
        let mut wrong_cases: Vec<WrongTestCase> = vec![];
        for _ in 0..num_tests {
            let p = make_random_problem(&mut rng);
            let result = check(&p);
            if let Some(wrong_test_case) = result {
                wrong_cases.push(wrong_test_case);
            }
            if wrong_cases.len() >= max_wrong_case {
                break;
            }
        }

        if !wrong_cases.is_empty() {
            for t in &wrong_cases {
                println!("{:?}", t.problem);
                println!("main ans : {:?}", t.main_ans);
                println!("naive ans: {:?}", t.naive_ans);
                println!();
            }
            println!("{} cases are wrong.", wrong_cases.len());
            panic!();
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
