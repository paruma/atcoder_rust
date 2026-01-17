#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PosHeight {
    pos: Pos,
    h: usize,
}

impl Readable for PosHeight {
    type Output = PosHeight;

    fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(source: &mut S) -> Self::Output {
        input! {
            from source,
            y: Usize1,
            x: Usize1,
            h: usize,
        }
        PosHeight {
            pos: Pos::new(x as i64, y as i64),
            h,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[derive_readable]
struct Query {
    src: PosHeight,
    dst: PosHeight,
}
#[derive(Debug, Clone)]
struct Problem {
    h: usize,
    w: usize,
    building_heights: Vec<Vec<usize>>,
    nq: usize,
    qs: Vec<Query>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            h: usize,
            w: usize,
            building_heights: [[usize; w]; h],
            nq: usize,
            qs: [Query; nq],
        }
        Problem {
            h,
            w,
            building_heights,
            nq,
            qs,
        }
    }
    fn solve(&self) -> Answer {
        // 並列二分探索

        let h = self.h;
        let w = self.w;
        let building_heights = &self.building_heights;

        let max_height = building_heights.iter().flatten().copied().max().unwrap();

        let height_to_pos_list = {
            let mut height_to_pos_list = vec![vec![]; max_height + 1];
            for y in 0..h {
                for x in 0..w {
                    let pos = Pos::new(x as i64, y as i64);
                    let height = building_heights[pos];
                    height_to_pos_list[height].push(pos);
                }
            }
            height_to_pos_list
        };

        let min_height_list = {
            let mut ok_ng_list = vec![(0, max_height + 1); self.nq];

            loop {
                let mut to_update = false;
                let mut mid_to_idxes = vec![vec![]; max_height + 1];

                for i in 0..self.nq {
                    let (ok, ng) = ok_ng_list[i];
                    if ng - ok > 1 {
                        let mid = (ok + ng) / 2;
                        mid_to_idxes[mid].push(i);
                        to_update = true;
                    }
                }

                if !to_update {
                    break;
                }
                let mut uf = GridUnionFind::new(h, w);
                for height in (0..=max_height).rev() {
                    for &current in &height_to_pos_list[height] {
                        for next in current.around4_pos_iter() {
                            if building_heights.is_within(next)
                                && building_heights[next] >= building_heights[current]
                            {
                                uf.unite(current, next);
                            }
                        }
                    }
                    for &i in &mid_to_idxes[height] {
                        let q = self.qs[i];
                        let mid = height;
                        if uf.same(q.src.pos, q.dst.pos) {
                            ok_ng_list[i].0 = mid; // ok = mid;
                        } else {
                            ok_ng_list[i].1 = mid; // ng = mid;
                        }
                    }

                    // eprintln!("height: {} n_group: {}", height, uf.num_groups());
                }
            }

            ok_ng_list.iter().copied().map(|(ok, _ng)| ok).collect_vec()
        };

        let ans = (0..self.nq)
            .map(|i| {
                let q = self.qs[i];
                let min_height = min_height_list[i];
                if q.src.h >= min_height || q.dst.h >= min_height {
                    usize::abs_diff(q.src.h, min_height) + usize::abs_diff(min_height, q.dst.h)
                } else {
                    usize::abs_diff(q.src.h, q.dst.h)
                }
            })
            .collect_vec();

        Answer { ans }
    }

    fn solve_naive1(&self) -> Answer {
        // 並列二分探索をしていない

        let h = self.h;
        let w = self.w;
        let building_heights = &self.building_heights;

        let max_height = building_heights.iter().flatten().copied().max().unwrap();

        let height_to_pos_list = {
            let mut height_to_pos_list = vec![vec![]; max_height + 1];
            for y in 0..h {
                for x in 0..w {
                    let pos = Pos::new(x as i64, y as i64);
                    let height = building_heights[pos];
                    height_to_pos_list[height].push(pos);
                }
            }
            height_to_pos_list
        };

        let mut ans = vec![];

        for q in &self.qs {
            // どこまで下がればいいか求める
            let min_height = {
                //
                let mut ok = 0;
                let mut ng = max_height + 1;

                while ng - ok > 1 {
                    let mid = (ok + ng) / 2;
                    let mid_is_ok = {
                        let mut uf = GridUnionFind::new(h, w);
                        for height in (mid..=max_height).rev() {
                            for &current in &height_to_pos_list[height] {
                                for next in current.around4_pos_iter() {
                                    if building_heights.is_within(next)
                                        && building_heights[next] >= building_heights[current]
                                    {
                                        uf.unite(current, next);
                                    }
                                }
                            }
                            // eprintln!("height: {} n_group: {}", height, uf.num_groups());
                        }
                        uf.same(q.src.pos, q.dst.pos)
                    };

                    if mid_is_ok {
                        ok = mid;
                    } else {
                        ng = mid;
                    }
                }
                ok
            };

            let sub_ans = if q.src.h >= min_height || q.dst.h >= min_height {
                usize::abs_diff(q.src.h, min_height) + usize::abs_diff(min_height, q.dst.h)
            } else {
                usize::abs_diff(q.src.h, q.dst.h)
            };
            ans.push(sub_ans);
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
    pub fn print_vec<T: std::fmt::Display>(arr: &[T]) {
        for a in arr {
            println!("{}", a);
        }
    }
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Display>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{}", x)).join(" ");
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
pub struct GridUnionFind {
    pub uf: UnionFind,
    pub h: usize,
    pub w: usize,
}
impl GridUnionFind {
    pub fn new(h: usize, w: usize) -> GridUnionFind {
        GridUnionFind {
            uf: UnionFind::new(h * w),
            h,
            w,
        }
    }
    pub fn encode(&self, pos: Pos) -> usize {
        (pos.y * self.w as i64 + pos.x) as usize
    }
    pub fn decode(&self, i: usize) -> Pos {
        let y = (i / self.w) as i64;
        let x = (i % self.w) as i64;
        Pos::new(x, y)
    }
    pub fn same_count(&mut self, pos: Pos) -> usize {
        self.uf.same_count(self.encode(pos))
    }
    pub fn same(&mut self, pos1: Pos, pos2: Pos) -> bool {
        self.uf.same(self.encode(pos1), self.encode(pos2))
    }
    pub fn num_groups(&self) -> usize {
        self.uf.num_groups()
    }
    pub fn unite(&mut self, pos1: Pos, pos2: Pos) {
        self.uf.unite(self.encode(pos1), self.encode(pos2));
    }
    pub fn groups(&mut self) -> Vec<Vec<Pos>> {
        self.uf
            .groups()
            .into_iter()
            .map(|group| group.iter().copied().map(|i| self.decode(i)).collect_vec())
            .collect_vec()
    }
}
use pos::*;
pub mod pos {
    use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

    use proconio::derive_readable;
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Pos {
        pub y: i64,
        pub x: i64,
    }
    impl Pos {
        pub fn new(x: i64, y: i64) -> Pos {
            Pos { x, y }
        }
    }
    impl Pos {
        pub fn scala_mul(self, rhs: i64) -> Pos {
            Pos::new(self.x * rhs, self.y * rhs)
        }
    }
    impl Pos {
        pub fn inner_product(self, rhs: Self) -> i64 {
            self.x * rhs.x + self.y * rhs.y
        }
        pub fn norm_square(self) -> i64 {
            self.inner_product(self)
        }
    }
    impl Add for Pos {
        type Output = Pos;
        fn add(self, rhs: Self) -> Self::Output {
            Pos::new(self.x + rhs.x, self.y + rhs.y)
        }
    }
    impl Sub for Pos {
        type Output = Pos;
        fn sub(self, rhs: Self) -> Self::Output {
            Pos::new(self.x - rhs.x, self.y - rhs.y)
        }
    }
    impl Neg for Pos {
        type Output = Self;
        fn neg(self) -> Self::Output {
            Pos::new(-self.x, -self.y)
        }
    }
    impl num_traits::Zero for Pos {
        fn zero() -> Self {
            Pos::new(0, 0)
        }
        fn is_zero(&self) -> bool {
            self.x.is_zero() && self.y.is_zero()
        }
    }
    impl AddAssign for Pos {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }
    impl SubAssign for Pos {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }
    pub const DIR8_LIST: [Pos; 8] = [
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 1 },
        Pos { x: 1, y: 0 },
        Pos { x: 1, y: -1 },
        Pos { x: 0, y: -1 },
        Pos { x: -1, y: -1 },
        Pos { x: -1, y: 0 },
        Pos { x: -1, y: 1 },
    ];
    pub const DIR4_LIST: [Pos; 4] = [
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 0 },
        Pos { x: 0, y: -1 },
        Pos { x: -1, y: 0 },
    ];
    impl Pos {
        pub fn around4_pos_iter(self) -> impl Iterator<Item = Pos> {
            DIR4_LIST.iter().copied().map(move |d| self + d)
        }
        pub fn around8_pos_iter(self) -> impl Iterator<Item = Pos> {
            DIR8_LIST.iter().copied().map(move |d| self + d)
        }
    }
}
use vec_vec_at::*;
pub mod vec_vec_at {
    use super::pos::*;
    use easy_ext::ext;
    use std::ops::{Index, IndexMut};
    #[ext(ExtVecVec)]
    impl<T> Vec<Vec<T>> {
        pub fn width(&self) -> usize {
            if self.is_empty() { 0 } else { self[0].len() }
        }
        pub fn height(&self) -> usize {
            self.len()
        }
        pub fn is_within(&self, pos: Pos) -> bool {
            (0..self.width() as i64).contains(&pos.x) && (0..self.height() as i64).contains(&pos.y)
        }
    }
    impl<T> Index<Pos> for Vec<Vec<T>> {
        type Output = T;
        fn index(&self, index: Pos) -> &Self::Output {
            if cfg!(debug_assertions) && !self.is_within(index) {
                panic!(
                    "index out of bounds: the size (w, h) is ({}, {}) but the index (x, y) is ({}, {})",
                    self.width(),
                    self.height(),
                    index.x,
                    index.y
                );
            }
            &self[index.y as usize][index.x as usize]
        }
    }
    impl<T> IndexMut<Pos> for Vec<Vec<T>> {
        fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
            if cfg!(debug_assertions) && !self.is_within(index) {
                panic!(
                    "index out of bounds: the size (w, h) is ({}, {}) but the index (x, y) is ({}, {})",
                    self.width(),
                    self.height(),
                    index.x,
                    index.y
                );
            }
            &mut self[index.y as usize][index.x as usize]
        }
    }
}
