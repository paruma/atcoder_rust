enum Query {
    Add { p: Pos },
    Merge,
    IsSame { u: usize, v: usize },
}

fn dist(p1: Pos, p2: Pos) -> i64 {
    let d = p1 - p2;
    d.x.abs() + d.y.abs()
}
fn main() {
    input! {
        n: usize,
        nq: usize,
        xys: [(i64, i64); n],
    }
    let mut ps = xys
        .iter()
        .copied()
        .map(|(x, y)| Pos::new(x, y))
        .collect_vec();

    let qs = (0..nq)
        .map(|_| {
            input! {
                t: usize,
            }
            if t == 1 {
                input! {
                    a: i64,
                    b: i64,
                }
                Query::Add { p: Pos::new(a, b) }
            } else if t == 2 {
                Query::Merge
            } else {
                input! {
                    u: Usize1,
                    v: Usize1,
                }
                Query::IsSame { u, v }
            }
        })
        .collect_vec();

    let mut heap = BinaryHeap::<(Reverse<i64>, usize, usize)>::new();
    let mut uf = UnionFind::new(n + nq);

    for (i, j) in (0..n).tuple_combinations() {
        let d = dist(ps[i], ps[j]);
        heap.push((Reverse(d), i, j));
    }

    for q in qs {
        match q {
            Query::Add { p } => {
                ps.push(p);
                let added_idx = ps.len() - 1;
                for i in 0..ps.len() - 1 {
                    let d = dist(ps[i], p);
                    heap.push((Reverse(d), i, added_idx));
                }
            }
            Query::Merge => {
                loop {
                    let next = heap.peek();
                    if let Some(next) = next {
                        if uf.same(next.1, next.2) {
                            heap.pop();
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                if heap.is_empty() {
                    println!("-1");
                } else {
                    let Reverse(min_dist) = heap.peek().unwrap().0;
                    println!("{}", min_dist);

                    loop {
                        if heap.is_empty() {
                            break;
                        }

                        let next = *heap.peek().unwrap();
                        if uf.same(next.1, next.2) {
                            heap.pop();
                            continue;
                        }
                        if next.0 .0 != min_dist {
                            break;
                        }

                        heap.pop();
                        uf.unite(next.1, next.2);
                    }
                }
                //
            }
            Query::IsSame { u, v } => {
                let ans = uf.same(u, v);
                print_yesno(ans);
            }
        }
    }
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
use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet};

// ====== output func ======
#[allow(unused_imports)]
use print_util::*;
pub mod print_util {
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
    #[fastout]
    pub fn print_vec_chars(vec_chars: &[Vec<char>]) {
        for row in vec_chars {
            let msg = row.iter().collect::<String>();
            println!("{}", msg);
        }
    }
    pub fn print_yesno(ans: bool) {
        let msg = if ans { "Yes" } else { "No" };
        println!("{}", msg);
    }
}

// ====== snippet ======

use pos::*;
pub mod pos {
    use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Pos {
        pub x: i64,
        pub y: i64,
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
    use std::fmt::{Debug, Error, Formatter};
    impl Debug for Pos {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            f.write_fmt(format_args!("({}, {})", self.x, self.y))?;
            Ok(())
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
