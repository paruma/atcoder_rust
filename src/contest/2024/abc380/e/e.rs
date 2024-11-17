#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Query {
    Bucket { x: usize, color: usize },
    Count { color: usize },
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
        }

        let qs = (0..nq)
            .map(|_| {
                input! {
                    t: usize
                }

                if t == 1 {
                    input! {
                        x: Usize1,
                        color: Usize1
                    }
                    Query::Bucket { x, color }
                } else {
                    input! {
                        color: Usize1
                    }
                    Query::Count { color }
                }
            })
            .collect_vec();

        Problem { n, nq, qs }
    }

    fn solve(&self) -> Answer {
        let n = self.n;
        let mut ans = vec![];

        let mut uf1 = SimpleUnionFind::new(self.n);
        let mut uf2 = MonoidUnionFind::<RangeMinMaxMonoid>::new(
            &(0..self.n)
                .map(|i| RangeMinMax::unit(i as i64))
                .collect_vec(),
        );

        for &q in &self.qs {
            match q {
                Query::Bucket { x, color } => {
                    uf1.change_color(x, color);
                    let left_most = uf2.same_prod(x).min as usize;

                    if left_most != 0 {
                        // 左を見る
                        if color == uf1.get_color(left_most - 1) {
                            uf1.unite(x, left_most - 1);
                            uf2.unite(x, left_most - 1);
                        }
                    }

                    let right_most = uf2.same_prod(x).max as usize;

                    if right_most != n - 1 {
                        // 右を見る
                        if color == uf1.get_color(right_most + 1) {
                            uf1.unite(x, right_most + 1);
                            uf2.unite(x, right_most + 1);
                        }
                    }
                    //
                }
                Query::Count { color } => {
                    let sub_ans = uf1.get_color_cnt(color);
                    ans.push(sub_ans)
                }
            }
        }

        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        let n = self.n;
        let mut ans = vec![];
        let mut cell_colors = (0..n).collect_vec();
        for &q in &self.qs {
            match q {
                Query::Bucket { x, color } => {
                    let old_color = cell_colors[x];
                    cell_colors[x] = color;

                    for i in x + 1..n {
                        if cell_colors[i] == old_color {
                            cell_colors[i] = color;
                        } else {
                            break;
                        }
                    }

                    for i in (0..x).rev() {
                        if cell_colors[i] == old_color {
                            cell_colors[i] = color;
                        } else {
                            break;
                        }
                    }
                }
                Query::Count { color } => {
                    let sub_ans = cell_colors.iter().copied().filter(|&x| x == color).count();
                    ans.push(sub_ans)
                }
            }
        }
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<usize>,
}

impl Answer {
    fn print(&self) {
        print_vec(&self.ans);
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
        let n = rng.gen_range(1..=5);
        let nq = 5;

        let qs = (0..nq)
            .map(|_| {
                let t = rng.gen_range(0..2);
                if t == 1 {
                    Query::Bucket {
                        x: rng.gen_range(0..n),
                        color: rng.gen_range(0..n),
                    }
                } else {
                    Query::Count {
                        color: rng.gen_range(0..n),
                    }
                }
            })
            .collect_vec();
        let p = Problem { n, nq, qs };
        //println!("{:?}", &p);
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

use ac_library::Monoid;
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
use std::convert::Infallible;

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
        color: usize,
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
        fn root(count: usize, color: usize) -> Node {
            Node::Root(RootInfo { count, color })
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
    pub struct SimpleUnionFind {
        nodes: Vec<Node>,
        cnt_groups: usize,
        color_cnt: Vec<usize>,
    }
    impl SimpleUnionFind {
        pub fn new(n: usize) -> SimpleUnionFind {
            let nodes = (0..n).map(|i| Node::root(1, i)).collect_vec();
            let color_cnt = vec![1; n];
            SimpleUnionFind {
                nodes,
                cnt_groups: n,
                color_cnt,
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

        pub fn change_color(&mut self, x: usize, color: usize) {
            let x_root = self.root(x);
            let x_count = self.nodes[x_root].as_root().count;
            let old_color = self.nodes[x_root].as_root().color;

            self.color_cnt[old_color] -= x_count;
            self.color_cnt[color] += x_count;
            self.nodes[x_root] = Node::root(x_count, color);
        }

        pub fn get_color(&mut self, x: usize) -> usize {
            let x_root = self.root(x);
            self.nodes[x_root].as_root().color
        }

        pub fn get_color_cnt(&mut self, color: usize) -> usize {
            self.color_cnt[color]
        }
        pub fn unite(&mut self, x: usize, y: usize) -> bool {
            if self.same(x, y) {
                return false;
            }
            // 2つの色は同じ想定
            let color = {
                let x_root = self.root(x);
                self.nodes[x_root].as_root().color
            };
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
            self.nodes[larger_root] = Node::root(count_sum, color);
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

use monoid_union_find::*;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RangeMinMax {
    pub min: i64,
    pub max: i64,
}
impl RangeMinMax {
    pub fn unit(x: i64) -> Self {
        Self { min: x, max: x }
    }
}
pub struct RangeMinMaxMonoid(Infallible);
impl Monoid for RangeMinMaxMonoid {
    type S = RangeMinMax;
    fn identity() -> Self::S {
        RangeMinMax {
            min: i64::MAX,
            max: i64::MIN,
        }
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        RangeMinMax {
            min: i64::min(a.min, b.min),
            max: i64::max(a.max, b.max),
        }
    }
}
