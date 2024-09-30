#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    u: Usize1,
    v: Usize1,
    w: i64,
}

impl Edge {
    fn rev(&self) -> Self {
        Edge {
            u: self.v,
            v: self.u,
            w: -self.w,
        }
    }
}

#[derive(Debug, Clone)]
struct Problem {
    n_v: usize,
    n_e: usize,
    es: Vec<Edge>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n_v: usize,
            n_e: usize,
            es: [Edge; n_e],
        }
        Problem { n_v, n_e, es }
    }

    fn solve(&self) -> Answer {
        let nv = self.n_v;

        let mut uf = PotentializedUnionFind::new(nv);

        for e in &self.es {
            uf.unite(e.u, e.v, e.w);
        }
        let ans = (0..nv).map(|i| uf.potential_from_root(i)).collect_vec();
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        let nv = self.n_v;

        let mut adj = vec![vec![]; nv];

        for &e in &self.es {
            adj[e.u].push(e);
            adj[e.v].push(e.rev());
        }

        let n_vertex = adj.len();
        let mut open: Queue<usize> = Queue::new();
        let mut visited = vec![false; n_vertex];

        let mut ans = vec![i64::MAX; nv];

        for v in 0..nv {
            if visited[v] {
                continue;
            }
            open.push(v);
            visited[v] = true;
            ans[v] = 0;

            while let Some(current) = open.pop() {
                for &e in &adj[current] {
                    if !visited[e.v] {
                        visited[e.v] = true;
                        open.push(e.v);
                        ans[e.v] = ans[e.u] + e.w;
                    }
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
        print_vec_1line(&self.ans);
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
        // let n = rng.gen_range(1..=10);
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
use potentialized_union_find::*;
pub mod potentialized_union_find {
    use itertools::Itertools;
    #[derive(Clone, Debug)]
    struct RootInfo {
        count: usize,
    }
    #[derive(Clone, Debug)]
    struct NonRootInfo {
        parent: usize,
        potential_diff: i64,
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
        fn non_root(parent: usize, potential_diff: i64) -> Node {
            Node::NonRoot(NonRootInfo {
                parent,
                potential_diff,
            })
        }
        fn as_root(&self) -> &RootInfo {
            match self {
                Node::Root(info) => info,
                Node::NonRoot(_) => panic!(),
            }
        }
    }
    #[derive(Clone, Debug)]
    struct ToRoot {
        root_index: usize,
        potential_diff: i64,
    }
    #[derive(Clone, Debug)]
    pub struct PotentializedUnionFind {
        nodes: Vec<Node>,
        cnt_groups: usize,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UnionResult {
        Consistent { updated: bool },
        Inconsistent,
    }
    impl UnionResult {
        pub fn updated(&self) -> bool {
            match self {
                UnionResult::Consistent { updated } => *updated,
                UnionResult::Inconsistent => false,
            }
        }
        pub fn is_consistent(&self) -> bool {
            matches!(self, UnionResult::Consistent { .. })
        }
        pub fn is_inconsistent(&self) -> bool {
            matches!(self, UnionResult::Inconsistent { .. })
        }
    }
    impl PotentializedUnionFind {
        pub fn new(n: usize) -> PotentializedUnionFind {
            PotentializedUnionFind {
                nodes: vec![Node::Root(RootInfo { count: 1 }); n],
                cnt_groups: n,
            }
        }
        fn root_node(&mut self, index: usize) -> ToRoot {
            match &self.nodes[index] {
                Node::Root(_) => ToRoot {
                    root_index: index,
                    potential_diff: 0,
                },
                Node::NonRoot(my_info) => {
                    let to_parent_potential_diff = my_info.potential_diff;
                    let parent_to_root = self.root_node(my_info.parent);
                    let new_potential_diff =
                        to_parent_potential_diff + parent_to_root.potential_diff;
                    self.nodes[index] =
                        Node::non_root(parent_to_root.root_index, new_potential_diff);
                    ToRoot {
                        root_index: parent_to_root.root_index,
                        potential_diff: new_potential_diff,
                    }
                }
            }
        }
        pub fn root(&mut self, index: usize) -> usize {
            self.root_node(index).root_index
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

        pub fn potential_from_root(&mut self, x: usize) -> i64 {
            match &self.nodes[x] {
                Node::Root(root_info) => 0,
                Node::NonRoot(non_root_info) => {
                    let root: usize = self.root(x);
                    self.diff(root, x).unwrap()
                }
            }
        }

        /// diff = dst のポテンシャル - src のポテンシャル となるように統合する
        pub fn unite(&mut self, src: usize, dst: usize, diff: i64) -> UnionResult {
            if self.same(src, dst) {
                if self.diff(src, dst) == Some(diff) {
                    return UnionResult::Consistent { updated: false };
                } else {
                    return UnionResult::Inconsistent;
                }
            }
            self.cnt_groups -= 1;
            let src_root_node = self.root_node(src);
            let dst_root_node = self.root_node(dst);
            let root_diff = -src_root_node.potential_diff + diff + dst_root_node.potential_diff;
            let (src_root_node, dst_root_node, root_diff) = {
                let src_cnt = self.nodes[src_root_node.root_index].as_root().count;
                let dst_cnt = self.nodes[dst_root_node.root_index].as_root().count;
                if src_cnt <= dst_cnt {
                    (src_root_node, dst_root_node, root_diff)
                } else {
                    (dst_root_node, src_root_node, -root_diff)
                }
            };
            let count_sum = self.nodes[src_root_node.root_index].as_root().count
                + self.nodes[dst_root_node.root_index].as_root().count;
            self.nodes[src_root_node.root_index] =
                Node::non_root(dst_root_node.root_index, root_diff);
            self.nodes[dst_root_node.root_index] = Node::root(count_sum);
            UnionResult::Consistent { updated: true }
        }
        /// dst のポテンシャル - src のポテンシャル を求める
        pub fn diff(&mut self, src: usize, dst: usize) -> Option<i64> {
            if self.same(src, dst) {
                Some(self.root_node(src).potential_diff - self.root_node(dst).potential_diff)
            } else {
                None
            }
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
