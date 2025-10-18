//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    xs: Vec<i64>,
}

/// n_balls 個の区別するボールを、n_boxes 個のグループ（区別しない）に分割する分割の仕方を全列挙する。
/// ボールが0個のグループがあっても良い。
/// Vec<Vec<usize>> はグループの分割を表す
fn bell(n_balls: usize, n_boxes: usize, xs: &[i64]) -> Vec<i64> {
    struct Rec {
        n_balls: usize,
        n_boxes: usize,
        xs: Vec<i64>,
    }

    impl Rec {
        fn new(n_balls: usize, n_boxes: usize, xs: Vec<i64>) -> Self {
            Rec {
                n_balls,
                n_boxes,
                xs,
            }
        }

        fn exec(&self) -> Vec<i64> {
            let mut groups_list = vec![];
            self.exec_rec(0, &mut vec![], &mut groups_list);
            groups_list
        }

        fn exec_rec(&self, cnt: usize, groups: &mut Vec<Vec<usize>>, xor_list: &mut Vec<i64>) {
            // スターリング数の漸化式
            // S(n,k) = k S(n-1, k) + S(n-1, k-1)
            // と同じ考え方をする

            if self.n_balls == cnt {
                let xor = groups
                    .iter()
                    .map(|group| group.iter().copied().map(|i| self.xs[i]).sum::<i64>())
                    .fold(0, |acc, x| acc ^ x);
                xor_list.push(xor);
                return;
            }

            // すでにあるグループに追加
            for group_i in 0..groups.len() {
                groups[group_i].push(cnt);
                self.exec_rec(cnt + 1, groups, xor_list);
                groups[group_i].pop();
            }

            // 新しくグループを作成する
            if groups.len() < self.n_boxes {
                groups.push(vec![cnt]);
                self.exec_rec(cnt + 1, groups, xor_list);
                groups.pop();
            }
        }
    }

    Rec::new(n_balls, n_boxes, xs.to_vec()).exec()
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            xs: [i64; n],
        }
        Problem { n, xs }
    }

    fn solve(&self) -> Answer {
        let n = self.n;
        let xs = &self.xs;
        let xor_list = bell(n, n, xs);
        let ans = xor_list.iter().unique().count() as i64;
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
    // bell(12, 12, &vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
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
