//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    source: Vec<u8>,
    target: Vec<u8>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            source: Bytes,
            target: Bytes,
        }
        Problem { n, source, target }
    }
    #[allow(clippy::redundant_clone)]
    fn solve(&self) -> Answer {
        let n = self.n;
        let mut source = self.source.clone();
        let mut target = self.target.clone();
        source.push(b'.');
        source.push(b'.');
        target.push(b'.');
        target.push(b'.');
        let source = source;
        let target = target;

        let mut open: Queue<Vec<u8>> = Queue::new();
        let mut visited: HashSet<Vec<u8>> = HashSet::new();
        let mut dp: HashMap<Vec<u8>, i64> = HashMap::new();
        open.push(source.clone());
        visited.insert(source.clone());
        dp.insert(source.clone(), 0);

        while let Some(current) = open.pop() {
            let empty_pos = current.iter().copied().position(|ch| ch == b'.').unwrap();
            for i in 0..n + 1 {
                if current[i] == b'.' || current[i + 1] == b'.' {
                    continue;
                }
                // i と i + 1 を空きスペースに入れる
                let next = {
                    let mut next = current.clone();
                    // swap (i, i + 1), (empty_pos, empty_pos + 1)
                    next[empty_pos] = next[i];
                    next[empty_pos + 1] = next[i + 1];
                    next[i] = b'.';
                    next[i + 1] = b'.';
                    next
                };

                if visited.contains(&next) {
                    continue;
                }

                visited.insert(next.clone());
                dp.insert(next.clone(), dp[&current] + 1);
                open.push(next.clone());
            }
        }

        let ans = dp.get(&target).copied();

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
    ans: Option<i64>,
}

impl Answer {
    fn print(&self) {
        let msg = self.ans.unwrap_or(-1);
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
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};

    #[test]
    fn test_problem() {
        let n = 5;
        let impossible_list = iproduct!((0..n).powerset(), (0..n).powerset())
            .filter(|(s, t)| s.len() == t.len())
            .map(|(s, t)| {
                let mut source = vec![b'W'; n];
                let mut target = vec![b'W'; n];
                for i in s {
                    source[i] = b'B';
                }

                for i in t {
                    target[i] = b'B';
                }
                (source, target)
            })
            .filter(|(source, target)| {
                Problem {
                    n,
                    source: source.clone(),
                    target: target.clone(),
                }
                .solve()
                .ans
                .is_none()
            })
            .collect_vec();
        for (s, t) in &impossible_list {
            let s = String::from_utf8(s.clone()).unwrap();
            let t = String::from_utf8(t.clone()).unwrap();

            println!("source={}, target={}", s, t);
        }
        println!("len={}", impossible_list.len());

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
    }
    impl<T> Default for Queue<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}
