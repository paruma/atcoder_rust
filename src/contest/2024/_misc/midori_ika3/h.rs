//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    ss: Vec<char>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            ss: Chars,
        }
        Problem { n, ss }
    }

    fn solve(&self) -> Answer {
        let n = self.n;
        let ss = &self.ss;
        // もしかしてACLない？
        let m = 998244353;
        let mut dp = vec![vec![0; 3]; n];
        let left = 0;
        let up = 1;
        let right = 2;
        // 1文字見ておく
        match ss[0] {
            'L' => {
                dp[0][left] = 1;
            }
            'U' => {
                dp[0][up] = 1;
            }
            'R' => {
                dp[0][right] = 1;
            }
            _ => {
                dp[0][left] = 1;
                dp[0][up] = 1;
                dp[0][right] = 1;
            }
        }

        for i in 1..n {
            if ss[i] == 'L' || ss[i] == '.' {
                dp[i][left] = (dp[i - 1][left] + dp[i - 1][up]) % m;
            }
            if ss[i] == 'U' || ss[i] == '.' {
                dp[i][up] = (dp[i - 1][left] + dp[i - 1][up] + dp[i - 1][right]) % m;
            }
            if ss[i] == 'R' || ss[i] == '.' {
                dp[i][right] = (dp[i - 1][left] + dp[i - 1][up] + dp[i - 1][right]) % m;
            }
        }

        //dbg!(&dp);

        let ans = (dp[n - 1][left] + dp[n - 1][up] + dp[n - 1][right]) % m;
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
    input_interactive!{alpha: char}
    println_flush!("{}{}", alpha,alpha);

    loop{
        input_interactive!{t: char}
        if t == '!'{
            input_interactive!{ss: Chars}
            break;
        }
        input_interactive!{ss: Chars}
        println_flush!("? {}{}", ss[1],ss[0]);

    }


    //Problem::read().solve().print();
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
        // let n = rng.gen_range(1..=10);
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
        // let mut rng = SmallRng::from_entropy();
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

use proconio::input_interactive;
// ====== import ======
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet};

// ====== output func ======
#[allow(unused_imports)]
use print_vec::*;
pub mod print_vec {

    use proconio::fastout;
    #[fastout]
    pub fn print_vec<T: std::fmt::Debug>(arr: &[T]) {
        for a in arr {
            println!("{:?}", a);
        }
    }
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Debug>(arr: &[T]) {
        let msg = arr
            .iter()
            .map(|x| format!("{:?}", x))
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Debug>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row
                .iter()
                .map(|x| format!("{:?}", x))
                .collect::<Vec<String>>()
                .join(" ");
            println!("{}", msg);
        }
    }
    pub fn print_bytes(bytes: &[u8]) {
        let msg = String::from_utf8(bytes.to_vec()).unwrap();
        println!("{}", msg);
    }
    pub fn print_chars(chars: &[char]) {
        let msg = chars.iter().collect::<String>();
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
use std::io::{stdout, Write};

#[macro_export]
macro_rules! println_flush {
    () => {
        println!();
        stdout().flush().unwrap();
    };
    ($($arg:tt)*) => {{
        println!($($arg)*);
        stdout().flush().unwrap();
    }};
}