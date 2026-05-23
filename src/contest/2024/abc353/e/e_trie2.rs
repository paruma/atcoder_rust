#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    strs: Vec<Vec<u8>>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            strs: [Bytes; n]
        }
        Problem { n, strs }
    }
    fn solve(&self) -> Answer {
        // 解法
        // trie 木を構築する
        let mut trie = TrieCore::new();
        let mut vals: Vec<i64> = vec![0];
        let mut ans = 0;

        for str in &self.strs {
            trie.insert(str);
            vals.resize(trie.num_nodes(), 0);
            for &node in &trie.trace_nodes(str)[1..] {
                ans += vals[node];
                vals[node] += 1;
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
    ans: i64,
}

impl Answer {
    fn print(&self) {
        println!("{}", self.ans);
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

use hashbag::HashBag;
// ====== import ======
#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, izip};
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
use trie::*;
#[allow(clippy::module_inception)]
/// トライ木（接頭辞木）の実装。
/// 数列（文字列）の集合を管理し、共通接頭辞の検索やノードのパス取得を効率的に行う。
pub mod trie {
    use std::collections::BTreeMap;
    /// トライ木の実装。
    /// 各ノードは `BTreeMap` を用いて、次の文字に対する遷移先ノード ID を保持する。
    #[derive(Clone, Debug)]
    pub struct TrieCore<T> {
        children_list: Vec<BTreeMap<T, usize>>,
    }
    impl<T: Ord + Copy> TrieCore<T> {
        /// 空のトライ木を構築する。
        /// # 計算量
        /// O(1)
        pub fn new() -> Self {
            Self {
                children_list: vec![BTreeMap::new()],
            }
        }
        /// 指定したノード `node` が持つ子ノードへの遷移情報を取得する。
        /// # 計算量
        /// O(1)
        pub fn children(&self, node: usize) -> &BTreeMap<T, usize> {
            &self.children_list[node]
        }
        /// 数列 `xs` をトライ木に挿入する。
        /// # 計算量
        /// O(|xs| log Σ) （Σ はアルファベットサイズ、ここでは文字の種類数）
        pub fn insert(&mut self, xs: &[T]) {
            let mut cur_node = 0;
            for &x in xs {
                if !self.children_list[cur_node].contains_key(&x) {
                    let new_node = self.children_list.len();
                    self.children_list[cur_node].insert(x, new_node);
                    self.children_list.push(BTreeMap::new());
                }
                cur_node = self.children_list[cur_node][&x];
            }
        }
        /// 指定したノード `cur` から、文字 `x` による遷移先のノード ID を取得する。
        /// 遷移先が存在しない場合は `None` を返す。
        /// # 計算量
        /// O(log Σ)
        pub fn next(&self, cur: usize, x: T) -> Option<usize> {
            self.children_list[cur].get(&x).copied()
        }
        /// 数列 `xs` に対応する終端ノード ID を取得する。
        /// `xs` がトライ木に含まれない場合は `None` を返す。
        /// # 計算量
        /// O(|xs| log Σ)
        pub fn find_node(&self, xs: &[T]) -> Option<usize> {
            let mut cur_node = 0;
            for &x in xs {
                let next_node = self.next(cur_node, x)?;
                cur_node = next_node;
            }
            Some(cur_node)
        }
        /// 数列 `xs` を辿る際に通過するノード ID のリストを返す。
        /// 途中で遷移できなくなった場合は、そこまでのノード ID リストを返す。
        /// # 計算量
        /// O(|xs| log Σ)
        pub fn trace_nodes(&self, xs: &[T]) -> Vec<usize> {
            let mut cur_node = 0;
            let mut path = vec![cur_node];
            for &x in xs {
                let Some(next_node) = self.next(cur_node, x) else {
                    break;
                };
                cur_node = next_node;
                path.push(cur_node);
            }
            path
        }
        /// トライ木に含まれるノードの総数を返す。
        /// # 計算量
        /// O(1)
        pub fn num_nodes(&self) -> usize {
            self.children_list.len()
        }
    }
    impl<T: Ord + Copy> Default for TrieCore<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}
