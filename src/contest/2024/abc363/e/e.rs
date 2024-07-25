//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    h: usize,
    w: usize,
    max_year: usize,
    xss: Vec<Vec<usize>>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            h: usize,
            w: usize,
            max_year: usize,
            xss: [[usize; w]; h],
        }
        Problem {
            h,
            w,
            max_year,
            xss,
        }
    }

    fn is_sea(&self, pos: Pos<i64>) -> bool {
        pos.x < 0 || self.w as i64 <= pos.x || pos.y < 0 || self.h as i64 <= pos.y
    }

    fn decode(&self, pos: Pos<i64>) -> usize {
        pos.x as usize + (pos.y as usize) * self.w
    }

    fn solve(&self) -> Answer {
        let max_year = self.max_year;
        let h = self.h;
        let w = self.w;
        let xss = &self.xss;
        let mut time_to_sink_pos: Vec<Vec<Pos<i64>>> = vec![vec![]; max_year + 1];
        for y in 0..h {
            for x in 0..w {
                let pos = Pos::new(x as i64, y as i64);
                let sink_year = *xss.at(pos);
                if sink_year <= max_year {
                    time_to_sink_pos[*xss.at(pos)].push(Pos::new(x as i64, y as i64));
                }
            }
        }

        // w * h は海を表す
        let mut uf = UnionFind::new(w * h + 1);
        let sea_id = w * h;

        let mut ans = vec![];

        for current_year in 1..=max_year {
            for &pos in &time_to_sink_pos[current_year] {
                for next in Pos::around4_pos_iter(pos) {
                    if self.is_sea(next) {
                        uf.unite(self.decode(pos), sea_id);
                    } else if xss.at(next) <= xss.at(pos) {
                        uf.unite(self.decode(pos), self.decode(next));
                    }
                }
            }
            let cnt_sinked = uf.same_count(sea_id) - 1;
            let cnt_not_sinked = w * h - cnt_sinked;
            ans.push(cnt_not_sinked);
        }

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // 解法: BFS をする (解説解法)
        let max_year = self.max_year;
        let h = self.h;
        let w = self.w;
        let xss = &self.xss;
        let mut time_to_queue: Vec<Queue<Pos<i64>>> = vec![Queue::new(); max_year + 1];
        let mut visited = vec![vec![false; w]; h];
        for y in 0..h {
            for x in 0..w {
                // 外周のみ
                if x == 0 || x == w - 1 || y == 0 || y == h - 1 {
                    let pos = Pos::new(x as i64, y as i64);
                    let sink_year = *xss.at(pos);
                    if sink_year <= max_year {
                        time_to_queue[*xss.at(pos)].push(Pos::new(x as i64, y as i64));
                        *visited.at_mut(pos) = true;
                    }
                }
            }
        }

        let mut cnt_sinked = 0;
        let mut sinked = vec![vec![false; w]; h];
        let mut ans = vec![];

        for current_year in 1..=max_year {
            // bfs
            while let Some(current) = time_to_queue[current_year].pop() {
                if !sinked.at(current) {
                    *sinked.at_mut(current) = true;
                    cnt_sinked += 1;
                }

                for next in Pos::around4_pos_iter(current) {
                    if self.is_sea(next) {
                        continue;
                    }
                    if *visited.at(next) {
                        continue;
                    }
                    *visited.at_mut(next) = true;
                    let next_sinked_year = usize::max(current_year, *xss.at(next));
                    if next_sinked_year <= max_year {
                        time_to_queue[next_sinked_year].push(next);
                    }
                }
            }

            let cnt_not_sinked = w * h - cnt_sinked;

            ans.push(cnt_not_sinked);
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
        print_vec(&self.ans);
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
use pos::*;
pub mod pos {
    use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Pos<T> {
        pub x: T,
        pub y: T,
    }
    impl<T> Pos<T> {
        pub fn new(x: T, y: T) -> Pos<T> {
            Pos { x, y }
        }
    }
    impl<T: Mul<Output = T> + Copy> Pos<T> {
        pub fn scala_mul(self, rhs: T) -> Pos<T> {
            Pos::new(self.x * rhs, self.y * rhs)
        }
    }
    impl<T: Add<Output = T> + Mul<Output = T> + Copy> Pos<T> {
        pub fn inner_product(self, rhs: Self) -> T {
            self.x * rhs.x + self.y * rhs.y
        }
        pub fn norm_square(self) -> T {
            self.inner_product(self)
        }
    }
    impl<T: Add<Output = T> + Copy> Add for Pos<T> {
        type Output = Pos<T>;
        fn add(self, rhs: Self) -> Self::Output {
            Pos::new(self.x + rhs.x, self.y + rhs.y)
        }
    }
    impl<T: Sub<Output = T> + Copy> Sub for Pos<T> {
        type Output = Pos<T>;
        fn sub(self, rhs: Self) -> Self::Output {
            Pos::new(self.x - rhs.x, self.y - rhs.y)
        }
    }
    impl<T: Neg<Output = T>> Neg for Pos<T> {
        type Output = Self;
        fn neg(self) -> Self::Output {
            Pos::new(-self.x, -self.y)
        }
    }
    impl<T: num_traits::Zero + Copy> num_traits::Zero for Pos<T> {
        fn zero() -> Self {
            Pos::new(T::zero(), T::zero())
        }
        fn is_zero(&self) -> bool {
            self.x.is_zero() && self.y.is_zero()
        }
    }
    impl<T: Add<Output = T> + Copy> AddAssign for Pos<T> {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }
    impl<T: Sub<Output = T> + Copy> SubAssign for Pos<T> {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }
    pub const DIR8_LIST: [Pos<i64>; 8] = [
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 1 },
        Pos { x: 1, y: 0 },
        Pos { x: 1, y: -1 },
        Pos { x: 0, y: -1 },
        Pos { x: -1, y: -1 },
        Pos { x: -1, y: 0 },
        Pos { x: -1, y: 1 },
    ];
    pub const DIR4_LIST: [Pos<i64>; 4] = [
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 0 },
        Pos { x: 0, y: -1 },
        Pos { x: -1, y: 0 },
    ];
    impl Pos<i64> {
        pub fn around4_pos_iter(self) -> impl Iterator<Item = Pos<i64>> {
            DIR4_LIST.iter().copied().map(move |d| d + self)
        }
        pub fn around8_pos_iter(self) -> impl Iterator<Item = Pos<i64>> {
            DIR8_LIST.iter().copied().map(move |d| d + self)
        }
    }
}
use vec_vec_at::*;
pub mod vec_vec_at {
    use super::pos::*;
    use easy_ext::ext;
    #[ext(VecVecAt)]
    impl<T> Vec<Vec<T>> {
        pub fn at(&self, pos: Pos<i64>) -> &T {
            &self[pos.y as usize][pos.x as usize]
        }
        pub fn at_mut(&mut self, pos: Pos<i64>) -> &mut T {
            &mut self[pos.y as usize][pos.x as usize]
        }
    }
}
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
