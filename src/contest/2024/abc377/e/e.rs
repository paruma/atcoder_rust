//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    k: i64,
    ps: Vec<usize>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            k: i64,
            ps: [Usize1; n],
        }
        Problem { n, k, ps }
    }

    fn solve(&self) -> Answer {
        let n = self.n;
        let k = self.k;
        let ps = &self.ps;

        let mut uf = UnionFind::new(n);

        for (i, p) in ps.iter().copied().enumerate() {
            uf.unite(i, p);
        }
        let groups = uf.groups();
        //dbg!(&groups);

        let mut ans = vec![usize::MAX; n];

        for group in groups {
            // group 内 で 2^k をする。
            // 2^k mod |group| を求めたい。
            let group = {
                let start = group[0];
                let mut group = vec![start];

                let mut current = start;
                loop {
                    current = ps[current];
                    if current == start {
                        break;
                    }
                    group.push(current);
                }
                group
            };

            //dbg!(&group);
            let step = pow_mod(2, k, group.len() as u32) as usize;
            //dbg!(step);
            for i in 0..group.len() {
                //dbg!(group[i]);
                //dbg!(group[(i + step) % group.len()]);
                ans[group[i]] = group[(i + step) % group.len()];
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
        let ans = self.ans.iter().copied().map(|x| x + 1).collect_vec();
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

use ac_library::pow_mod;
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
