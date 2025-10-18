//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    adj: Vec<usize>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            adj: [Usize1; n],
        }
        Problem { n, adj }
    }
    fn solve(&self) -> Answer {
        // Functional Graph の性質を使った解法
        let n = self.n;
        let adj = &self.adj;

        let mut visited = vec![false; n];
        let mut uf = UnionFind::new(n);

        for init_v in 0..n {
            if visited[init_v] {
                continue;
            }

            let mut current = init_v;

            let mut cnt_visited = HashMap::<usize, usize>::new();

            // 2*n 回回す
            // (Functional Graph の性質からn回回せば閉路に入る。もうn回回せばその閉路で1周できる。)
            for _ in 0..2 * n {
                let next = adj[current];
                if visited[next] {
                    break;
                }
                if cnt_visited.contains_key(&next) {
                    uf.unite(current, next);
                }
                if cnt_visited.get(&next) == Some(&2) {
                    break;
                }
                //cnt_visited.insert(next);
                *cnt_visited.entry(next).or_insert(0) += 1;
                current = next;
            }
            visited[init_v] = true;

            for v in cnt_visited.keys() {
                visited[*v] = true;
            }
        }

        let groups = uf.groups();
        // 各頂点から group の index がほしい。
        let vertex_to_group_idx = {
            //
            let mut retval = vec![usize::MAX; n];
            for (group_i, group) in groups.iter().enumerate() {
                for v in group {
                    retval[*v] = group_i;
                }
            }
            retval
        };
        // dbg!(uf.groups());
        // dbg!(&vertex_to_group_idx);

        let adj_groups = {
            let mut adj_groups = vec![HashSet::<usize>::new(); groups.len()];
            for from in 0..n {
                let to = adj[from];
                let from_group_idx = vertex_to_group_idx[from];
                let to_group_idx = vertex_to_group_idx[to];
                if from_group_idx != to_group_idx {
                    adj_groups[from_group_idx].insert(to_group_idx);
                }
            }

            adj_groups
                .iter()
                .map(|xs| xs.iter().copied().collect_vec())
                .collect_vec()
        };

        // dbg!(&adj_groups);

        // トポソ
        let topo_sorted_group_idx = topo_sort(&adj_groups);
        //dbg!(&topo_sorted_group_idx);

        // DP
        let mut dp = vec![0_i64; groups.len()];

        for &g_i in topo_sorted_group_idx.iter().rev() {
            dp[g_i] = adj_groups[g_i]
                .iter()
                .copied()
                .map(|g_j| dp[g_j])
                .sum::<i64>()
                + groups[g_i].len() as i64
        }

        // dbg!(&dp);

        // DP の sum
        let ans = (0..groups.len())
            .map(|i| groups[i].len() as i64 * dp[i])
            .sum::<i64>();
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // SCC を使った解法
        let n = self.n;
        let mut scc_graph = SccGraph::new(self.n);
        for from in 0..n {
            let to = self.adj[from];
            scc_graph.add_edge(from, to);
        }

        let scc = scc_graph.scc();
        let to_scc_idx = {
            let mut to_scc_idx = vec![0; n];
            for (i, component) in scc.iter().enumerate() {
                for v in component {
                    to_scc_idx[*v] = i;
                }
            }
            to_scc_idx
        };

        let scc_adj = {
            let mut scc_adj = vec![HashSet::<usize>::new(); scc.len()];
            for from in 0..n {
                let to = self.adj[from];
                let from_scc_idx = to_scc_idx[from];
                let to_scc_idx = to_scc_idx[to];
                if from_scc_idx != to_scc_idx {
                    scc_adj[from_scc_idx].insert(to_scc_idx);
                }
            }
            scc_adj
                .iter()
                .map(|s| s.iter().copied().collect_vec())
                .collect_vec()
        };

        let mut dp = vec![0; scc.len()];
        for i in (0..scc.len()).rev() {
            dp[i] =
                scc_adj[i].iter().copied().map(|next| dp[next]).sum::<i64>() + scc[i].len() as i64;
        }
        let ans = (0..scc.len())
            .map(|i| dp[i] * scc[i].len() as i64)
            .sum::<i64>();
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

use ac_library::SccGraph;
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
use simple_union_find::*;
pub mod simple_union_find {
    use itertools::Itertools;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct RootInfo {
        count: usize,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct NonRootInfo {
        parent_index: usize,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Node {
        Root(RootInfo),
        NonRoot(NonRootInfo),
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct RootAndIndex {
        info: RootInfo,
        index: usize,
    }
    #[derive(Clone, Debug)]
    pub struct UnionFind {
        nodes: Vec<Node>,
        cnt_groups: usize,
    }
    impl UnionFind {
        pub fn new(n: usize) -> UnionFind {
            UnionFind {
                nodes: vec![Node::Root(RootInfo { count: 1 }); n],
                cnt_groups: n,
            }
        }
        fn root_node(&mut self, index: usize) -> RootAndIndex {
            match self.nodes[index] {
                Node::Root(info) => RootAndIndex { info, index },
                Node::NonRoot(info) => {
                    let root_and_index = self.root_node(info.parent_index);
                    self.nodes[index] = Node::NonRoot(NonRootInfo {
                        parent_index: root_and_index.index,
                    });
                    root_and_index
                }
            }
        }
        pub fn root(&mut self, index: usize) -> usize {
            self.root_node(index).index
        }
        pub fn same_count(&mut self, index: usize) -> usize {
            self.root_node(index).info.count
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
            let x_root_node = self.root_node(x);
            let y_root_node = self.root_node(y);
            let (smaller_root, larger_root) = if x_root_node.info.count <= y_root_node.info.count {
                (x_root_node, y_root_node)
            } else {
                (y_root_node, x_root_node)
            };
            self.nodes[smaller_root.index] = Node::NonRoot(NonRootInfo {
                parent_index: larger_root.index,
            });
            self.nodes[larger_root.index] = Node::Root(RootInfo {
                count: smaller_root.info.count + larger_root.info.count,
            });
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
pub fn topo_sort(adj: &Vec<Vec<usize>>) -> Vec<usize> {
    let n_vertex = adj.len();
    let mut in_deg = vec![0; n_vertex];
    for current in 0..n_vertex {
        for &next in &adj[current] {
            in_deg[next] += 1;
        }
    }
    let mut open: Queue<usize> = Queue::new();
    for v in 0..n_vertex {
        if in_deg[v] == 0 {
            open.push(v);
        }
    }
    let mut ans = vec![];
    while let Some(current) = open.pop() {
        ans.push(current);
        for &next in &adj[current] {
            in_deg[next] -= 1;
            if in_deg[next] == 0 {
                open.push(next);
            }
        }
    }
    ans
}
