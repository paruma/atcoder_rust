// 解法: 木DP

//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    es: Vec<(usize, usize)>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            es: [(Usize1, Usize1); n - 1],
        }
        Problem { n, es }
    }

    fn solve(&self) -> Answer {
        // UF
        let n = self.n;
        let adj = self.es.iter().copied().fold(vec![vec![]; n], |mut acc, e| {
            acc[e.0].push(e.1);
            acc[e.1].push(e.0);
            acc
        });

        let degs = adj.iter().map(|nexts| nexts.len()).collect_vec();

        let children = make_tree_children(&adj, 0);
        let ord = dfs_post_order(&adj, 0);

        // 次数列が 23* のもの (葉から見て)
        let mut dp = vec![0; n];

        for v in ord {
            dp[v] = if degs[v] == 2 {
                1
            } else if degs[v] == 3 {
                children[v].iter().copied().map(|child| dp[child]).sum()
            } else {
                0
            };
        }

        // 単純でないグラフを含める(頂点数 2)
        let ans_include_nonsimple = (0..n)
            .map(|v| {
                // v を LCA としたときの場合の数

                // 端点が LCA の場合
                let term1 = if degs[v] == 2 {
                    children[v]
                        .iter()
                        .copied()
                        .map(|child| dp[child])
                        .sum::<i64>()
                } else {
                    0
                };

                // 端点が LCA でない場合（LCA で折り返す）

                let term2 = if degs[v] == 3 {
                    let sum = children[v]
                        .iter()
                        .copied()
                        .map(|child| dp[child])
                        .sum::<i64>();
                    let sq_sum = children[v]
                        .iter()
                        .copied()
                        .map(|child| dp[child] * dp[child])
                        .sum::<i64>();
                    (sum * sum - sq_sum) / 2
                } else {
                    0
                };
                term1 + term2
            })
            .sum::<i64>();

        // [deg 2] - [deg 2] みたいなパスを除く
        let nonsimple = self
            .es
            .iter()
            .copied()
            .filter(|&(u, v)| degs[u] == 2 && degs[v] == 2)
            .count() as i64;

        let ans = ans_include_nonsimple - nonsimple;
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
/// 根付き木の隣接リスト `adj` と根 `root` から、各頂点の子頂点リストを求めます。
/// # 計算量
/// O(V + E)
pub fn make_tree_children(adj: &[Vec<usize>], root: usize) -> Vec<Vec<usize>> {
    let n = adj.len();
    let mut children = vec![vec![]; n];
    let mut visited = vec![false; n];
    let mut queue = std::collections::VecDeque::new();
    visited[root] = true;
    queue.push_back(root);
    while let Some(v) = queue.pop_front() {
        for &u in &adj[v] {
            if !visited[u] {
                visited[u] = true;
                children[v].push(u);
                queue.push_back(u);
            }
        }
    }
    children
}
use mod_stack::*;
pub mod mod_stack {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Stack<T> {
        raw: Vec<T>,
    }
    impl<T> Stack<T> {
        pub fn new() -> Self {
            Stack { raw: Vec::new() }
        }
        pub fn push(&mut self, value: T) {
            self.raw.push(value)
        }
        pub fn pop(&mut self) -> Option<T> {
            self.raw.pop()
        }
        pub fn peek(&self) -> Option<&T> {
            self.raw.last()
        }
        pub fn is_empty(&self) -> bool {
            self.raw.is_empty()
        }
        pub fn len(&self) -> usize {
            self.raw.len()
        }
    }
    impl<T> Default for Stack<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}
/// 深さ優先探索 (DFS) を行い、帰りがけ順 (post-order) での頂点順序を返します。
/// # 計算量
/// O(V + E)
pub fn dfs_post_order(adj: &[Vec<usize>], init: usize) -> Vec<usize> {
    fn dfs(
        adj: &[Vec<usize>],
        current: usize,
        visited: &mut Vec<bool>,
        post_order: &mut Vec<usize>,
    ) {
        visited[current] = true;
        for &next in &adj[current] {
            if !visited[next] {
                dfs(adj, next, visited, post_order);
            }
        }
        post_order.push(current);
    }
    let nv = adj.len();
    let mut visited = vec![false; nv];
    let mut post_order = vec![];
    dfs(adj, init, &mut visited, &mut post_order);
    post_order
}
