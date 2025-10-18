//#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Query {
    Add { i: usize },
    Guide,
}

impl_readable_for_enum! {
    Query{
        1 => Add{i: Usize1},
        2 => Guide,
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
            qs: [Query; q],
        }
        Problem { q, qs }
    }

    fn solve(&self) -> Answer {
        let mut ans = vec![];
        let mut que = Queue::new();

        for q in &self.qs {
            match *q {
                Query::Add { i } => {
                    que.push(i);
                }
                Query::Guide => {
                    ans.push(que.pop().unwrap());
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
    ans: Vec<usize>,
}

impl Answer {
    fn print(&self) {
        let msg = self.ans.iter().copied().map(|x| x + 1).collect_vec();
        print_vec(&msg);
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
    pub fn print_vec<T: std::fmt::Display>(arr: &[T]) {
        for a in arr {
            println!("{}", a);
        }
    }
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Display>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{}", x)).join(" ");
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
pub mod impl_readable_for_enum {
    /// 利用例
    /// ```
    /// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    /// enum Query {
    ///     Move { p: usize, h: usize },
    ///     Swap { h1: usize, h2: usize },
    ///     Output { p: usize },
    /// }
    /// impl_readable_for_enum! {
    ///     Query {
    ///         1 => Move { p: Usize1, h: Usize1 },
    ///         2 => Swap { h1: Usize1, h2: Usize1 },
    ///         3 => Output { p: Usize1 },
    ///     }
    /// }
    /// ```
    #[macro_export]
    macro_rules ! impl_readable_for_enum {($ enum_name : ident {$ ($ idx : literal => $ variant : ident $ ({$ ($ field : ident : $ ty : ty ) ,* } ) ? ) ,* $ (, ) ? } ) => {impl proconio :: source :: Readable for $ enum_name {type Output = $ enum_name ; fn read < R : std :: io :: BufRead , S : proconio :: source :: Source < R >> (source : & mut S ) -> $ enum_name {input ! {from & mut * source , t : usize } match t {$ ($ idx => {impl_readable_for_enum ! (@ read_variant source , $ enum_name , $ variant $ ({$ ($ field : $ ty ) ,* } ) ? ) } ) ,*, _ => unreachable ! () , } } } } ; (@ read_variant $ source : ident , $ enum_name : ident , $ variant : ident {$ ($ field : ident : $ ty : ty ) ,* } ) => {{input ! {from & mut *$ source , $ ($ field : $ ty ) ,* } ; $ enum_name ::$ variant {$ ($ field ) ,* } } } ; (@ read_variant $ source : ident , $ enum_name : ident , $ variant : ident ) => {{$ enum_name ::$ variant } } ; }
}
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
            self.raw.push_back(value)
        }
        pub fn pop(&mut self) -> Option<T> {
            self.raw.pop_front()
        }
        pub fn peek(&self) -> Option<&T> {
            self.raw.front()
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
