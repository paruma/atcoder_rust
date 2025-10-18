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

        // 連結成分が3同士
        let mut uf = UnionFind::new(n);

        for current in 0..n {
            if adj[current].len() == 3 {
                for &next in &adj[current] {
                    if adj[next].len() == 3 {
                        uf.unite(current, next);
                    }
                }
            }
        }

        let ans = uf
            .groups()
            .iter()
            .filter(|group| adj[group[0]].len() == 3)
            .map(|group| {
                // group の隣にある頂点で次数が2のもの
                let cnt_next_deg2 = group
                    .iter()
                    .copied()
                    .map(|current| {
                        adj[current]
                            .iter()
                            .copied()
                            .filter(|&next| adj[next].len() == 2)
                            .count()
                    })
                    .sum::<usize>() as i64;
                cnt_next_deg2 * (cnt_next_deg2 - 1) / 2
            })
            .sum::<i64>();
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // 木 DP

        let n = self.n;
        let adj = self.es.iter().copied().fold(vec![vec![]; n], |mut acc, e| {
            acc[e.0].push(e.1);
            acc[e.1].push(e.0);
            acc
        });

        // dp0[v] = vを根とする部分木での条件をみたす個数 (v がパスの LCA となるもの)
        // dp1[v] = vから下に下がるパスで次数列が 3 3 ... 2 となるものの個数
        fn dfs(
            n: usize,
            adj: &[Vec<usize>],
            parent: Option<usize>,
            current: usize,
            dp0: &mut Vec<i64>,
            dp1: &mut Vec<i64>,
        ) {
            let children = adj[current]
                .iter()
                .copied()
                .filter(|next| Some(*next) != parent)
                .collect_vec();

            for &next in &children {
                dfs(n, adj, Some(current), next, dp0, dp1);
            }

            let dp1_children = children
                .iter()
                .copied()
                .map(|child| dp1[child])
                .collect_vec();
            let dp1_children_sum = dp1_children.iter().copied().sum::<i64>();

            let current_deg = adj[current].len();

            dp0[current] = match current_deg {
                2 => dp1_children_sum,
                3 => {
                    let dp1_children_sq_sum =
                        dp1_children.iter().copied().map(|x| x * x).sum::<i64>();
                    (dp1_children_sum * dp1_children_sum - dp1_children_sq_sum) / 2
                }
                _ => 0,
            };

            dp1[current] = match current_deg {
                2 => 1,
                3 => dp1_children_sum,
                _ => 0,
            };
        }

        let mut dp0 = vec![0; n];
        let mut dp1 = vec![0; n];

        dfs(n, &adj, None, 0, &mut dp0, &mut dp1);

        // deg(2) - deg(2) みたいな辺を追加すると多重辺になるパターンを除く
        let mult_edges = self
            .es
            .iter()
            .copied()
            .filter(|e| adj[e.0].len() == 2 && adj[e.1].len() == 2)
            .count() as i64;

        let ans = dp0.iter().sum::<i64>() - mult_edges;
        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        // 木 DP (for ループ)
        // 再帰で書くと変なところに処理を書いてしまう可能性がある。再帰を外に出すことである程度実装がやりやすくなる？

        let n = self.n;
        let adj = self.es.iter().copied().fold(vec![vec![]; n], |mut acc, e| {
            acc[e.0].push(e.1);
            acc[e.1].push(e.0);
            acc
        });

        let mut visited = vec![false; n];

        // dp0[v] = vを根とする部分木での条件をみたす個数 (v がパスの LCA となるもの)
        // dp1[v] = vから下に下がるパスで次数列が 3 3 ... 2 となるものの個数
        let mut dp0 = vec![0; n];
        let mut dp1 = vec![0; n];

        for current in dfs_post_order(&adj, 0) {
            if visited[current] {
                continue;
            }
            visited[current] = true;

            let children = adj[current]
                .iter()
                .copied()
                .filter(|next| visited[*next])
                .collect_vec();

            let dp1_children = children
                .iter()
                .copied()
                .map(|child| dp1[child])
                .collect_vec();
            let dp1_children_sum = dp1_children.iter().copied().sum::<i64>();

            let current_deg = adj[current].len();

            dp0[current] = match current_deg {
                2 => dp1_children_sum,
                3 => {
                    let dp1_children_sq_sum =
                        dp1_children.iter().copied().map(|x| x * x).sum::<i64>();
                    (dp1_children_sum * dp1_children_sum - dp1_children_sq_sum) / 2
                }
                _ => 0,
            };

            dp1[current] = match current_deg {
                2 => 1,
                3 => dp1_children_sum,
                _ => 0,
            };
        }

        // deg(2) - deg(2) みたいな辺を追加すると多重辺になるパターンを除く
        let mult_edges = self
            .es
            .iter()
            .copied()
            .filter(|e| adj[e.0].len() == 2 && adj[e.1].len() == 2)
            .count() as i64;

        let ans = dp0.iter().sum::<i64>() - mult_edges;
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
    Problem::read().solve3().print();
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
use simple_union_find::*;
pub mod simple_union_find {
    use itertools::Itertools;
    #[derive(Clone, Debug)]
    struct RootInfo {
        count: usize,
    }
    #[derive(Clone, Debug)]
    struct NonRootInfo {
        parent: usize,
    }
    #[derive(Clone, Debug)]
    enum Node {
        Root(RootInfo),
        NonRoot(NonRootInfo),
    }
    impl Node {
        fn root(count: usize) -> Node {
            Node::Root(RootInfo { count })
        }
        fn non_root(parent: usize) -> Node {
            Node::NonRoot(NonRootInfo { parent })
        }
        fn as_root(&self) -> &RootInfo {
            match self {
                Node::Root(info) => info,
                Node::NonRoot(_) => panic!(),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UnionFind {
        nodes: Vec<Node>,
        cnt_groups: usize,
    }
    impl UnionFind {
        pub fn new(n: usize) -> UnionFind {
            let nodes = (0..n).map(|_| Node::root(1)).collect_vec();
            UnionFind {
                nodes,
                cnt_groups: n,
            }
        }
        pub fn root(&mut self, index: usize) -> usize {
            match &self.nodes[index] {
                Node::Root(_) => index,
                Node::NonRoot(info) => {
                    let root = self.root(info.parent);
                    self.nodes[index] = Node::non_root(root);
                    root
                }
            }
        }
        pub fn same_count(&mut self, index: usize) -> usize {
            let root_index = self.root(index);
            self.nodes[root_index].as_root().count
        }
        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
        }
        pub fn num_groups(&self) -> usize {
            self.cnt_groups
        }
        pub fn unite(&mut self, x: usize, y: usize) -> bool {
            if self.same(x, y) {
                return false;
            }
            self.cnt_groups -= 1;
            let (smaller_root, larger_root) = {
                let x_root = self.root(x);
                let y_root = self.root(y);
                let x_count = self.nodes[x_root].as_root().count;
                let y_count = self.nodes[y_root].as_root().count;
                if x_count < y_count {
                    (x_root, y_root)
                } else {
                    (y_root, x_root)
                }
            };
            let count_sum =
                self.nodes[smaller_root].as_root().count + self.nodes[larger_root].as_root().count;
            self.nodes[smaller_root] = Node::non_root(larger_root);
            self.nodes[larger_root] = Node::root(count_sum);
            true
        }
        pub fn groups(&mut self) -> Vec<Vec<usize>> {
            let n = self.nodes.len();
            let roots = (0..n).map(|i| self.root(i)).collect_vec();
            let group_size = (0..n).map(|i| roots[i]).fold(vec![0; n], |mut acc, x| {
                acc[x] += 1;
                acc
            });
            let result = {
                let mut result = vec![Vec::new(); n];
                for i in 0..n {
                    result[i].reserve(group_size[i]);
                }
                for i in 0..n {
                    result[roots[i]].push(i);
                }
                result
            };
            result.into_iter().filter(|x| !x.is_empty()).collect_vec()
        }
    }
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
