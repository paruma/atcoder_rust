#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Query {
    x: Usize1,
    c: char,
}

#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    nq: usize,
    s: Vec<char>,
    qs: Vec<Query>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            nq: usize,
            s: Chars,
            qs: [Query; nq],
        }
        Problem { n, nq, s, qs }
    }

    fn solve(&self) -> Answer {
        let n = self.n;
        let nq = self.nq;
        let mut s = self.s.clone();
        let qs = &self.qs;

        let mut cnt_abc = s
            .iter()
            .copied()
            .tuple_windows()
            .filter(|(x1, x2, x3)| [*x1, *x2, *x3] == ['A', 'B', 'C'])
            .count();

        let mut ans = vec![];

        for q in qs {
            let before0 = if q.x <= 1 { None } else { s.get(q.x - 2..=q.x) };
            let before1 = if q.x == 0 {
                None
            } else {
                s.get(q.x - 1..=q.x + 1)
            };
            let before2 = s.get(q.x..=q.x + 2);

            let before_cnt = [before0, before1, before2]
                .iter()
                .filter(|before| **before == Some(&['A', 'B', 'C']))
                .count();

            s[q.x] = q.c;

            let after0 = if q.x <= 1 { None } else { s.get(q.x - 2..=q.x) };
            let after1 = if q.x == 0 {
                None
            } else {
                s.get(q.x - 1..=q.x + 1)
            };
            let after2 = s.get(q.x..=q.x + 2);

            let after_cnt = [after0, after1, after2]
                .iter()
                .filter(|after| **after == Some(&['A', 'B', 'C']))
                .count();

            cnt_abc = cnt_abc - before_cnt + after_cnt;
            ans.push(cnt_abc);
        }

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // リファクタリング
        let n = self.n;
        let nq = self.nq;
        let mut s = self.s.clone();
        let qs = &self.qs;

        let mut cnt_abc = s.windows(3).filter(|w| *w == ['A', 'B', 'C']).count();
        let mut ans = vec![];

        for q in qs {
            // q.x の周辺5文字を見る

            let before_cnt = s[q.x.saturating_sub(2)..(q.x + 3).min(n)]
                .windows(3)
                .filter(|w| *w == ['A', 'B', 'C'])
                .count();

            s[q.x] = q.c;

            let after_cnt = s[q.x.saturating_sub(2)..(q.x + 3).min(n)]
                .windows(3)
                .filter(|w| *w == ['A', 'B', 'C'])
                .count();

            cnt_abc -= before_cnt;
            cnt_abc += after_cnt;

            ans.push(cnt_abc);
        }

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
    ans: Vec<usize>,
}

impl Answer {
    fn print(&self) {
        print_vec(&self.ans);
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
use itertools::{Itertools, chain, iproduct, izip};
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
