//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    m: usize,
    pos_list: Vec<usize>,
    cnt_list: Vec<i128>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            m: usize,
            pos_list: [Usize1; m],
            cnt_list: [i128; m],
        }
        Problem {
            n,
            m,
            pos_list,
            cnt_list,
        }
    }

    fn solve(&self) -> Answer {
        let n = self.n;
        let m = self.m;

        let pos_cnt_list = izip!(&self.pos_list, &self.cnt_list)
            .sorted_by_key(|(pos, _)| **pos)
            .collect_vec();

        let pos_list = pos_cnt_list
            .iter()
            .copied()
            .map(|(pos, _)| *pos)
            .collect_vec();

        let init_cnts = &pos_cnt_list
            .iter()
            .copied()
            .map(|(_, cnt)| *cnt)
            .collect_vec();

        if init_cnts.iter().sum::<i128>() != n as i128 {
            return Answer { ans: None };
        }

        if pos_list[0] != 0 {
            return Answer { ans: None };
        }

        let mut require_cnts = vec![0; m];
        require_cnts[m - 1] = (n - pos_list[m - 1]) as i128;

        for i in (0..m - 1).rev() {
            //
            require_cnts[i] = i128::max(require_cnts[i + 1] - init_cnts[i + 1], 0)
                + (pos_list[i + 1] - pos_list[i]) as i128;
        }
        if require_cnts[0] > init_cnts[0] {
            return Answer { ans: None };
        }

        let ans = (0..m)
            .map(|i| {
                let diff = (pos_list.get(i + 1).copied().unwrap_or(n) - pos_list[i]) as i128;
                // require_cnts[i] + (require_cnts[i] - 1) + ... + (require_cnts[i] - diff + 1))
                // diff 個の和

                ((require_cnts[i] + (require_cnts[i] - diff + 1)) * diff) / 2 - diff
            })
            .sum::<i128>();

        let ans = Some(ans);
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        let n = self.n;
        let m = self.m;

        let pos_cnt_list = izip!(&self.pos_list, &self.cnt_list)
            .sorted_by_key(|(pos, _)| **pos)
            .collect_vec();

        let mut pos_list = pos_cnt_list.iter().map(|(pos, _)| **pos).collect_vec();
        let mut cnt_list = pos_cnt_list.iter().map(|(_, cnt)| **cnt).collect_vec();

        //番兵
        pos_list.push(n - 1);
        cnt_list.push(0);

        if cnt_list.iter().sum::<i128>() != self.n as i128 {
            return Answer { ans: None };
        }

        if pos_list[0] != 0 {
            return Answer { ans: None };
        }

        let mut ans = 0;

        let mut current_stones = cnt_list[0];

        for i in 1..=m {
            //
            let pos_diff = (pos_list[i] - pos_list[i - 1]) as i128;
            if pos_diff > current_stones {
                return Answer { ans: None };
            }

            let n_move = {
                let first = current_stones - 1;
                let last = current_stones - pos_diff;
                let n_terms = pos_diff;
                (first + last) * n_terms / 2
            };
            ans += n_move;

            current_stones -= pos_diff;
            current_stones += cnt_list[i];
        }
        let ans = Some(ans);
        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        let n = self.n;

        let mut init_cnts = vec![0; n];
        for (&pos, &cnt) in izip!(&self.pos_list, &self.cnt_list) {
            init_cnts[pos] = cnt;
        }

        let mut require_cnts = vec![0; n];
        require_cnts[n - 1] = 1;

        for i in (0..n - 1).rev() {
            require_cnts[i] = i128::max(require_cnts[i + 1] - init_cnts[i + 1], 0) + 1;
        }

        if require_cnts[0] > init_cnts[0] {
            return Answer { ans: None };
        }

        // dbg!(&init_cnts);
        // dbg!(&require_cnts);

        let ans = require_cnts.iter().copied().map(|x| x - 1).sum::<i128>();
        let ans = Some(ans);
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Option<i128>,
}

impl Answer {
    fn print(&self) {
        if let Some(ans) = self.ans {
            println!("{}", ans);
        } else {
            println!("-1");
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
    use atcoder_rust::mylib::random::random_test::generate_random_uniq_sequence;
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
        let n = rng.gen_range(2..=20_000);
        let m = rng.gen_range(1..=usize::min(n, 10));

        let pos_list = generate_random_uniq_sequence(m, || rng.gen_range(0..n));

        let cnt_list = (0..m)
            .map(|_| rng.gen_range(1..2_000_000_000))
            .collect_vec();

        let p = Problem {
            n,
            m,
            pos_list,
            cnt_list,
        };
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

use random_test::*;
pub mod random_test {
    use itertools::Itertools;
    use num::Integer;
    use num_integer::Roots;
    use petgraph::unionfind::UnionFind;
    use rand::Rng;
    use std::{collections::HashSet, hash::Hash};
    pub fn generate_random_uniq_sequence<T, F>(n: usize, mut gen: F) -> Vec<T>
    where
        T: Hash + PartialEq + Eq,
        F: FnMut() -> T,
    {
        let mut set: HashSet<T> = HashSet::new();
        while set.len() != n {
            set.insert(gen());
        }
        set.into_iter().collect_vec()
    }
    pub fn generate_random_while<T, F, P>(mut gen: F, mut pred: P) -> T
    where
        F: FnMut() -> T,
        P: FnMut(&T) -> bool,
    {
        loop {
            let x = gen();
            if pred(&x) {
                return x;
            }
        }
    }
    pub fn generate_random_tree<R>(rng: &mut R, n_vertices: usize) -> Vec<(usize, usize)>
    where
        R: Rng,
    {
        let mut edges: Vec<(usize, usize)> = Vec::new();
        let mut uf: UnionFind<usize> = UnionFind::new(n_vertices);
        while edges.len() != n_vertices - 1 {
            let x = rng.gen_range(0..n_vertices);
            let y = rng.gen_range(0..n_vertices);
            if uf.union(x, y) {
                edges.push((x, y));
            }
        }
        edges
    }
    fn is_prime(n: i64) -> bool {
        if n <= 1 {
            return false;
        }
        for i in 2..=n.sqrt() {
            if n.is_multiple_of(&i) {
                return false;
            }
        }
        true
    }
    pub fn generate_random_prime<R>(rng: &mut R, begin: i64, end: i64) -> i64
    where
        R: Rng,
    {
        let gen = || rng.gen_range(begin..end);
        generate_random_while(gen, |x| is_prime(*x))
    }
}
