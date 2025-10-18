//#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Query {
    Push(i64),
    Pop,
    Get(usize),
}

impl Query {
    fn read() -> Query {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! { l: i64 }
                Query::Push(l)
            }
            2 => Query::Pop,
            3 => {
                input! {k: Usize1}
                Query::Get(k)
            }
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone)]
struct Problem {
    q: usize,
    qs: Vec<Query>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            q: usize,
        }
        let qs = (0..q).map(|_| Query::read()).collect_vec();
        Problem { q, qs }
    }

    fn solve(&self) -> Answer {
        // ヘビの長さとその累積和
        let mut que: VecDeque<(i64, i64)> = VecDeque::new();
        let mut offset: i64 = 0;
        let mut ans: Vec<i64> = vec![];

        for &q in &self.qs {
            match q {
                Query::Push(l) => {
                    if let Some((head_l, head_sum)) = que.back() {
                        que.push_back((l, head_sum + head_l));
                    } else {
                        que.push_back((l, 0));
                        offset = 0;
                    }
                }
                Query::Pop => {
                    let (l, _sum0) = que.pop_front().unwrap();
                    offset += l;
                }
                Query::Get(k) => {
                    let (_l, sum) = que[k];
                    ans.push(sum - offset);
                }
            }
        }
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // Deque を使わない。累積和意識
        let mut snakes: Vec<i64> = vec![];
        let mut head = 0;
        let mut prefix_sum = vec![0]; // ヘビの長さの累積和
        let mut ans: Vec<i64> = vec![];

        for &q in &self.qs {
            match q {
                Query::Push(l) => {
                    snakes.push(l);
                    prefix_sum.push(prefix_sum.last().unwrap() + l);
                }
                Query::Pop => {
                    head += 1;
                }
                Query::Get(k) => {
                    // いま列にいる [0, k) 番目のヘビの長さの総和
                    // [head, head + k) 番目のヘビの長さの総和を求めればよい
                    let sub_ans = prefix_sum[head + k] - prefix_sum[head];
                    ans.push(sub_ans);
                }
            }
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
    ans: Vec<i64>,
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
use itertools::{chain, iproduct, izip, Itertools};
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::Reverse;
use std::collections::VecDeque;
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
use mod_queue::*;
pub mod mod_queue {
    use std::collections::VecDeque;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Queue<T> {
        raw: VecDeque<T>,
    }
    impl<T> Queue<T> {
        pub fn new() -> Self {
            Queue {
                raw: VecDeque::new(),
            }
        }
        pub fn push(&mut self, value: T) {
            self.raw.push_front(value)
        }
        pub fn pop(&mut self) -> Option<T> {
            self.raw.pop_back()
        }
        pub fn peek(&self) -> Option<&T> {
            self.raw.back()
        }
        pub fn is_empty(&self) -> bool {
            self.raw.is_empty()
        }
        pub fn len(&self) -> usize {
            self.raw.len()
        }

        pub fn get(&self, k: usize) -> &T {
            &self.raw[k]
        }
    }
    impl<T> Default for Queue<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}
