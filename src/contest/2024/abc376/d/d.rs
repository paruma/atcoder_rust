#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    src: Usize1,
    dst: Usize1,
}

impl Edge {
    fn new(src: usize, dst: usize) -> Edge {
        Edge { src, dst }
    }
    fn rev(self) -> Self {
        Edge {
            src: self.dst,
            dst: self.src,
        }
    }
}

#[derive(Debug, Clone)]
struct Problem {
    nv: usize,
    ne: usize,
    es: Vec<Edge>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            nv: usize,
            ne: usize,
            es: [Edge; ne],
        }
        Problem { nv, ne, es }
    }

    fn solve(&self) -> Answer {
        let nv = self.nv;
        let ne = self.ne;

        let adj = self
            .es
            .iter()
            .copied()
            .fold(vec![vec![]; nv], |mut acc, e| {
                acc[e.src].push(e);
                acc
            });

        let mut visited = vec![false; nv];

        let mut min_cycle = usize::MAX;

        let mut dist: HashMap<usize, usize> = HashMap::new();
        let mut open: Queue<usize> = Queue::new();
        let start = 0;
        open.push(start);
        dist.insert(start, 0);
        visited[start] = true;

        while let Some(current) = open.pop() {
            for &e in &adj[current] {
                #[allow(clippy::map_entry)]
                if !dist.contains_key(&e.dst) {
                    visited[e.dst] = true;
                    open.push(e.dst);
                    dist.insert(e.dst, dist[&e.src] + 1);
                } else {
                    if e.dst == 0 {
                        min_cycle = min_cycle.min(dist[&e.src] + 1 - dist[&e.dst]);
                    }
                }
            }
        }

        let ans = if min_cycle == usize::MAX {
            None
        } else {
            Some(min_cycle)
        };
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        let nv = self.nv;

        let adj = self
            .es
            .iter()
            .copied()
            .fold(vec![vec![]; nv], |mut acc, e| {
                acc[e.src].push(e);
                acc
            });

        let mut visited = vec![false; nv];

        let dist = {
            let mut dist = vec![None; nv];
            let mut open: Queue<usize> = Queue::new();
            let start = 0;
            open.push(start);
            dist[start] = Some(0);
            visited[start] = true;

            while let Some(current) = open.pop() {
                for &e in &adj[current] {
                    if !visited[e.dst] {
                        visited[e.dst] = true;
                        open.push(e.dst);
                        dist[e.dst] = Some(dist[e.src].unwrap() + 1);
                    }
                }
            }
            dist
        };
        let ans = self
            .es
            .iter()
            .copied()
            .filter(|e| e.dst == 0)
            .filter_map(|e| dist[e.src])
            .min()
            .map(|x| x + 1);

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
    ans: Option<usize>,
}

impl Answer {
    fn print(&self) {
        if let Some(ans) = self.ans {
            println!("{}", ans);
        } else {
            println!("{}", -1);
        }
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
    }
    impl<T> Default for Queue<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}
