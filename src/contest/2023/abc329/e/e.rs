//#[derive_readable]

#[derive(Clone, Debug, PartialEq, Eq)]
struct Problem {
    sheet: Vec<u8>,
    stamp: Vec<u8>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            _: usize,
            _: usize,
            sheet: Bytes,
            stamp: Bytes,
        }
        Problem { sheet, stamp }
    }

    fn solve_sub(&self) -> bool {
        // 後ろから考える
        // 1つスタンプを押したらそのスタンプの両端の2点とその付近が次スタンプを押す候補になる
        let ideal_sheet = &self.sheet;
        let stamp = &self.stamp;
        let mut current_sheet = vec![b'#'; ideal_sheet.len()];
        let stamp_pos_in_ideal_sheet = (0..ideal_sheet.len() - stamp.len() + 1)
            .filter(|&begin| &ideal_sheet[begin..begin + stamp.len()] == stamp);
        use DirPos::*;
        #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
        enum DirPos {
            Left(usize),
            Right(usize),
        }
        let mut queue = Queue::new();
        let mut visited = HashSet::new();
        for pos in stamp_pos_in_ideal_sheet {
            queue.push(Left(pos));
            queue.push(Right(pos + stamp.len() - 1));
            visited.insert(Left(pos));
            visited.insert(Right(pos + stamp.len() - 1));
            current_sheet[pos..pos + stamp.len()].copy_from_slice(stamp);
        }

        while let Some(current_dir_pos) = queue.pop() {
            match current_dir_pos {
                Left(pos) => {
                    for stamp_size in 1..stamp.len() {
                        let sub_stamp = &stamp[0..stamp_size];
                        if pos < stamp_size {
                            continue;
                        }
                        if visited.contains(&Left(pos - stamp_size)) {
                            continue;
                        }
                        let sheet_range = pos - stamp_size..pos;
                        let sub_ideal_sheet = &ideal_sheet[sheet_range.clone()];
                        let sub_current_sheet = &mut current_sheet[sheet_range.clone()];
                        if (0..stamp_size).all(|i| {
                            sub_current_sheet[i] != b'#' || sub_ideal_sheet[i] == sub_stamp[i]
                        }) {
                            for i in 0..stamp_size {
                                if sub_current_sheet[i] == b'#' {
                                    sub_current_sheet[i] = sub_stamp[i];
                                }
                            }
                            queue.push(Left(pos - stamp_size));
                            visited.insert(Left(pos - stamp_size));
                            break;
                        }
                    }
                }
                Right(pos) => {
                    for stamp_size in 1..stamp.len() {
                        let sub_stamp = &stamp[stamp.len() - stamp_size..];
                        if pos + stamp_size >= ideal_sheet.len() {
                            continue;
                        }
                        if visited.contains(&Right(pos + stamp_size)) {
                            continue;
                        }
                        let sheet_range = pos + 1..pos + 1 + stamp_size;
                        let sub_ideal_sheet = &ideal_sheet[sheet_range.clone()];
                        let sub_current_sheet = &mut current_sheet[sheet_range.clone()];
                        if (0..stamp_size).all(|i| {
                            sub_current_sheet[i] != b'#' || sub_ideal_sheet[i] == sub_stamp[i]
                        }) {
                            for i in 0..stamp_size {
                                if sub_current_sheet[i] == b'#' {
                                    sub_current_sheet[i] = sub_stamp[i];
                                }
                            }
                            queue.push(Right(pos + stamp_size));
                            visited.insert(Right(pos + stamp_size));
                            break;
                        }
                    }
                }
            }
        }

        &current_sheet == ideal_sheet
    }
    fn solve(&self) -> Answer {
        let ans = self.solve_sub();

        Answer { ans }
    }
}

struct Sheet {
    sheet: Vec<u8>,
}

impl Sheet {
    fn new(sheet: Vec<u8>) -> Self {
        Self { sheet }
    }

    fn pretty_string(&self) -> String {
        self.sheet.iter().map(|c| *c as char).collect()
    }

    fn can_remove(&mut self, begin: usize, stamp: &[u8]) -> bool {
        let stamp_size = stamp.len();
        let sheet_sub = &self.sheet[begin..begin + stamp_size];
        (0..stamp_size).all(|i| sheet_sub[i] == b'#' || sheet_sub[i] == stamp[i])
    }

    fn remove(&mut self, begin: usize, stamp_size: usize) {
        self.sheet[begin..begin + stamp_size].copy_from_slice(&vec![b'#'; stamp_size]);
    }

    fn all_removed(&self) -> bool {
        self.sheet.iter().all(|c| *c == b'#')
    }
}

impl Problem {
    fn solve_sub2(&self) -> bool {
        // 後ろから考える
        // ゴール sheet からスタンプを使って取り除くことで ##### にできるか判定する。
        let stamp = &self.stamp;
        let stamp_size = stamp.len();
        let stamp_size_i64 = stamp_size as i64;
        let sheet_len = self.sheet.len();

        let mut current_sheet = Sheet::new(self.sheet.to_vec());

        let mut queue: Queue<usize> = Queue::new();
        let mut visited: HashSet<usize> = HashSet::new();
        for begin in 0..sheet_len - stamp_size + 1 {
            if current_sheet.can_remove(begin, stamp) {
                current_sheet.remove(begin, stamp_size);
                queue.push(begin);
                visited.insert(begin);
            }
        }

        while let Some(current_begin) = queue.pop() {
            let current_begin_i64 = current_begin as i64;
            for next_begin_i64 in
                current_begin_i64 - (stamp_size_i64 - 1)..current_begin_i64 + stamp_size_i64
            {
                if next_begin_i64 < 0 {
                    continue;
                }
                let next_begin = next_begin_i64 as usize;
                let next_end = next_begin + stamp_size;
                if next_end > sheet_len {
                    continue;
                }

                if visited.contains(&next_begin) {
                    continue;
                }
                // おけるなら置く
                if current_sheet.can_remove(next_begin, stamp) {
                    current_sheet.remove(next_begin, stamp_size);
                    visited.insert(next_begin); // if の外にはいけない。今は除去不可能でも今後除去可能なケースがあるから。
                    queue.push(next_begin);
                }
            }
        }

        current_sheet.all_removed()
    }
    fn solve2(&self) -> Answer {
        let ans = self.solve_sub2();

        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: bool,
}

impl Answer {
    fn print(&self) {
        print_yesno(self.ans);
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
        let main_ans = p.solve2();
        let naive_ans = p.solve();
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
        let m = 3;
        let n = rng.gen_range(m..=10);

        let sheet = (0..n).map(|_| rng.gen_range(b'a'..=b'c')).collect();
        let stamp = (0..m).map(|_| rng.gen_range(b'a'..=b'c')).collect();

        let p = Problem { sheet, stamp };

        println!("{:?}", &p);
        p
    }

    #[allow(unreachable_code)]
    #[test]
    fn test_with_naive() {
        let num_tests = 10000;
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

use std::{collections::HashSet, ops::Range};

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
