//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n_v: usize,
    n_info: usize,
    infos: Vec<(usize, usize, i64)>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n_v: usize,
            n_info: usize,
            infos: [(Usize1, Usize1, i64); n_info],
        }
        Problem { n_v, n_info, infos }
    }
    fn solve(&self) -> Answer {
        // 素朴な乱択 (WA)
        use rand::seq::SliceRandom;
        let n_v = self.n_v;

        let time_keeper = TimeKeeper::new(1.98); // 問題の制限時間は2秒

        let mut uf = PotentializedUnionFind::new(n_v);

        for (a, b, diff) in &self.infos {
            // 逆にしがちなので、問題をちゃんと読んで実装する
            uf.unite(*b, *a, *diff); // a の順位 - b の順位 = diff
        }

        let roots = (0..n_v).filter(|x| uf.root(*x) == *x).collect_vec();

        let mut ans = vec![];

        let mut tmp = (0..n_v as i64).collect_vec();

        while !time_keeper.is_time_over() {
            let mut rng = SmallRng::from_os_rng();

            tmp.shuffle(&mut rng);
            let p = tmp[0..roots.len()].to_vec();
            let mut ord = vec![-1_i64; n_v];
            for i in 0..roots.len() {
                // roots[i] の順位を p[i] とする
                ord[roots[i]] = p[i];
            }

            //dbg!(&ord);

            for i in 0..n_v {
                let root = uf.root(i);
                ord[i] = ord[root] + uf.diff(root, i).unwrap();
            }

            if ord.iter().all_unique() && ord.iter().all(|x| (0..n_v as i64).contains(x)) {
                if ans.is_empty() {
                    ans = ord;
                } else {
                    for i in 0..n_v {
                        if ord[i] != ans[i] {
                            ans[i] = -1;
                        }
                    }
                }
            }
        }

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // 全探索 (TLE)
        let n_v = self.n_v;

        let mut uf = PotentializedUnionFind::new(n_v);

        for (a, b, diff) in &self.infos {
            // 逆にしがちなので、問題をちゃんと読んで実装する
            uf.unite(*b, *a, *diff); // a の順位 - b の順位 = diff
        }

        let roots = (0..n_v).filter(|x| uf.root(*x) == *x).collect_vec();

        let mut ans = vec![];

        for p in (0..n_v as i64).permutations(roots.len()) {
            let mut ord = vec![-1_i64; n_v];
            for i in 0..roots.len() {
                // roots[i] の順位を p[i] とする
                ord[roots[i]] = p[i];
            }

            //dbg!(&ord);

            for i in 0..n_v {
                let root = uf.root(i);
                ord[i] = ord[root] + uf.diff(root, i).unwrap();
            }

            if ord.iter().all_unique() && ord.iter().all(|x| (0..n_v as i64).contains(x)) {
                if ans.is_empty() {
                    ans = ord;
                } else {
                    for i in 0..n_v {
                        if ord[i] != ans[i] {
                            ans[i] = -1;
                        }
                    }
                }
            }

            if !ans.is_empty() && ans.iter().copied().all(|x| x == -1) {
                break;
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
        let ans = self
            .ans
            .iter()
            .copied()
            .map(|x| if (x == -1) { -1 } else { x + 1 })
            .collect_vec();
        print_vec_1line(&ans);
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
    fn make_random_problem() -> Problem {
        todo!()
        // let mut rng = SmallRng::from_os_rng();
        // let n = rng.random_range(1..=10);
        // let p = Problem { _a: n };
        // println!("{:?}", &p);
        // p
    }

    #[allow(unreachable_code)]
    #[test]
    fn test_with_naive() {
        return;
        let num_tests = 1000;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut wrong_cases: Vec<WrongTestCase> = vec![];
        for _ in 0..num_tests {
            let p = make_random_problem();
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
    marker::{Bytes, Usize1},
};
use rand::{rngs::SmallRng, SeedableRng};
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
use potentialized_union_find::*;
pub mod potentialized_union_find {
    use itertools::Itertools;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct RootInfo {
        count: usize,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct NonRootInfo {
        parent_index: usize,
        potential_diff: i64,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Node {
        Root(RootInfo),
        NonRoot(NonRootInfo),
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct ToRoot {
        root_info: RootInfo,
        root_index: usize,
        potential_diff: i64,
    }
    #[derive(Clone, Debug)]
    pub struct PotentializedUnionFind {
        nodes: Vec<Node>,
        cnt_groups: usize,
    }
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
            match self.nodes[index] {
                Node::Root(info) => ToRoot {
                    root_info: info,
                    root_index: index,
                    potential_diff: 0,
                },
                Node::NonRoot(current_info) => {
                    let parent_to_root = self.root_node(current_info.parent_index);
                    let potential_diff =
                        current_info.potential_diff + parent_to_root.potential_diff;
                    self.nodes[index] = Node::NonRoot(NonRootInfo {
                        parent_index: parent_to_root.root_index,
                        potential_diff,
                    });
                    ToRoot {
                        root_info: parent_to_root.root_info,
                        root_index: parent_to_root.root_index,
                        potential_diff,
                    }
                }
            }
        }
        pub fn root(&mut self, index: usize) -> usize {
            self.root_node(index).root_index
        }
        pub fn same_count(&mut self, index: usize) -> usize {
            self.root_node(index).root_info.count
        }
        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
        }
        pub fn num_groups(&self) -> usize {
            self.cnt_groups
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
            let (src_root_node, dst_root_node, root_diff) =
                if src_root_node.root_info.count <= dst_root_node.root_info.count {
                    (src_root_node, dst_root_node, root_diff)
                } else {
                    (dst_root_node, src_root_node, -root_diff)
                };
            self.nodes[src_root_node.root_index] = Node::NonRoot(NonRootInfo {
                parent_index: dst_root_node.root_index,
                potential_diff: root_diff,
            });
            self.nodes[dst_root_node.root_index] = Node::Root(RootInfo {
                count: src_root_node.root_info.count + dst_root_node.root_info.count,
            });
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

use time_keeper::*;
pub mod time_keeper {
    #[derive(Debug, Clone)]
    pub struct TimeKeeper {
        start_time: std::time::Instant,
        time_threshold_sec: f64,
    }
    impl TimeKeeper {
        /// time_threshold_sec: 制限時間(秒数)
        pub fn new(time_threshold_sec: f64) -> Self {
            TimeKeeper {
                start_time: std::time::Instant::now(),
                time_threshold_sec,
            }
        }
        #[inline]
        pub fn is_time_over(&self) -> bool {
            let elapsed_time = self.start_time.elapsed().as_nanos() as f64 * 1e-9;
            elapsed_time >= self.time_threshold_sec
        }
    }
}
