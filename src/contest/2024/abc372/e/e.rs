#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Query {
    Insert { u: usize, v: usize },
    GetKth { v: usize, k: usize },
}

impl Readable for Query {
    type Output = Query;

    fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(source: &mut S) -> Self::Output {
        input! {
            from source,
            t: usize,
            x1: usize,
            x2: usize,
        }
        if t == 1 {
            Query::Insert {
                u: x1 - 1,
                v: x2 - 1,
            }
        } else {
            Query::GetKth { v: x1 - 1, k: x2 }
        }
    }
}
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    nq: usize,
    qs: Vec<Query>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            nq: usize,
            qs: [Query; nq],
        }
        Problem { n, nq, qs }
    }

    fn solve(&self) -> Answer {
        use modified_union_find::*;

        let n = self.n;
        let qs = &self.qs;

        let mut uf = UnionFind::new(n);
        let mut ans = vec![];

        for &q in qs {
            match q {
                Query::Insert { u, v } => {
                    uf.unite(u, v);
                }
                Query::GetKth { v, k } => {
                    let set = uf.set(v);
                    ans.push(set.iter().rev().nth(k - 1).copied());
                }
            }
        }
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        use monoid_union_find::*;

        let n = self.n;
        let qs = &self.qs;

        let mut uf = MonoidUnionFind::<Top10Monoid>::new(&(0..n).map(|i| vec![i]).collect_vec());
        let mut ans = vec![];

        for &q in qs {
            match q {
                Query::Insert { u, v } => {
                    uf.unite(u, v);
                }
                Query::GetKth { v, k } => {
                    let sub_ans = uf.same_prod(v).get(k - 1).copied();
                    ans.push(sub_ans);
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
    ans: Vec<Option<usize>>,
}

impl Answer {
    fn print(&self) {
        for x in &self.ans {
            match x {
                Some(x) => {
                    println!("{}", *x + 1);
                }
                None => {
                    println!("-1");
                }
            }
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
use proconio::source::Readable;
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
pub mod modified_union_find {
    use std::collections::BTreeSet;

    use itertools::Itertools;
    #[derive(Clone, Debug)]
    struct RootInfo {
        count: usize,
        set: BTreeSet<usize>,
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
        fn root(count: usize, set: BTreeSet<usize>) -> Node {
            Node::Root(RootInfo { count, set })
        }
        fn non_root(parent: usize) -> Node {
            Node::NonRoot(NonRootInfo { parent })
        }
    }
    impl Node {
        fn as_root(&self) -> &RootInfo {
            match self {
                Node::Root(info) => info,
                Node::NonRoot(_) => panic!(),
            }
        }

        fn as_root_mut(&mut self) -> &mut RootInfo {
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
            let nodes = (0..n)
                .map(|i| Node::root(1, BTreeSet::from([i])))
                .collect_vec();
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
        pub fn set(&mut self, index: usize) -> &BTreeSet<usize> {
            let root_index = self.root(index);
            &self.nodes[root_index].as_root().set
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

            let mut set = std::mem::take(&mut self.nodes[larger_root].as_root_mut().set);
            for x in std::mem::take(&mut self.nodes[smaller_root].as_root_mut().set) {
                set.insert(x);
            }

            self.nodes[smaller_root] = Node::non_root(larger_root);
            self.nodes[larger_root] = Node::root(count_sum, set);
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

use monoid_top10::*;
#[allow(unused_variables)]
pub mod monoid_top10 {
    use ac_library::segtree::Monoid;
    use itertools::{chain, Itertools};
    use std::{cmp::Reverse, convert::Infallible};

    pub struct Top10Monoid(Infallible);
    impl Monoid for Top10Monoid {
        type S = Vec<usize>;
        fn identity() -> Self::S {
            vec![]
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            chain!(a, b)
                .sorted_by_key(|x| Reverse(**x))
                .take(10)
                .copied()
                .collect_vec()
        }
    }
}

/// 可換モノイドをのっけた Union Find
pub mod monoid_union_find {
    use ac_library::Monoid;
    use itertools::Itertools;
    #[derive(Clone, Debug)]
    struct RootInfo<S: Clone> {
        count: usize,
        prod: S,
    }
    #[derive(Clone, Debug)]
    struct NonRootInfo {
        parent: usize,
    }
    #[derive(Clone, Debug)]
    enum Node<S: Clone> {
        Root(RootInfo<S>),
        NonRoot(NonRootInfo),
    }
    impl<S: Clone> Node<S> {
        fn root(count: usize, prod: S) -> Node<S> {
            Node::Root(RootInfo { count, prod })
        }
        fn non_root(parent: usize) -> Node<S> {
            Node::NonRoot(NonRootInfo { parent })
        }
        fn as_root(&self) -> &RootInfo<S> {
            match self {
                Node::Root(info) => info,
                Node::NonRoot(_) => panic!(),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MonoidUnionFind<M: Monoid> {
        nodes: Vec<Node<M::S>>,
        cnt_groups: usize,
    }
    impl<M: Monoid> MonoidUnionFind<M> {
        pub fn new(data: &[M::S]) -> MonoidUnionFind<M> {
            let nodes = data.iter().map(|d| Node::root(1, d.clone())).collect_vec();
            MonoidUnionFind {
                nodes,
                cnt_groups: data.len(),
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
        pub fn same_prod(&mut self, index: usize) -> M::S {
            let root_index = self.root(index);
            self.nodes[root_index].as_root().prod.clone()
        }
        pub fn same_prod_ref(&mut self, index: usize) -> &M::S {
            let root_index = self.root(index);
            &self.nodes[root_index].as_root().prod
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
            let smaller_root_info = self.nodes[smaller_root].as_root();
            let larger_root_info = self.nodes[larger_root].as_root();
            let count = smaller_root_info.count + larger_root_info.count;
            let prod = M::binary_operation(&smaller_root_info.prod, &larger_root_info.prod);
            self.nodes[smaller_root] = Node::non_root(larger_root);
            self.nodes[larger_root] = Node::root(count, prod);
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
