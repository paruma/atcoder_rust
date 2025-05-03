macro_rules! chmin {
    ($a: expr, $b: expr) => {
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    };
}

#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Query {
    begin: Usize1,
    end: usize,
    sum: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    from: usize,
    to: usize,
    cost: i64,
}

impl Edge {
    fn new(from: usize, to: usize, cost: i64) -> Self {
        Edge { from, to, cost }
    }
}

fn solve(n: usize, m: usize, qs: &[Query]) -> Option<i64> {
    let mut uf = PotentializedUnionFind::new(n + 1);

    for q in qs {
        let result = uf.unite(q.begin, q.end, q.sum);
        if result.is_inconsistent() {
            return None;
        }
    }

    let groups = uf.groups();
    // dbg!(&groups);
    // dbg!(&uf);

    Some(-123)
}

fn solve2(n: usize, m: usize, qs: &[Query]) -> Option<i64> {
    // sn - s0 の最小値

    let nv = n + 1;

    let mut es = vec![];
    for i in 0..n {
        es.push(Edge::new(i + 1, i, -1));
    }

    for q in qs {
        es.push(Edge::new(q.begin, q.end, q.sum));
        es.push(Edge::new(q.end, q.begin, -q.sum));
    }
    let tmp = bellman_ford(&es, nv, nv - 1);
    if let Some(tmp) = tmp {
        tmp[0].to_option().map(|x| -x)
    } else {
        None
    }
}
fn main() {
    input! {
        n: usize,
        m: usize,
        qs: [Query; m],
    }

    let ans: Option<i64> = solve2(n, m, &qs);
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("{}", -1);
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
        /// 親のポテンシャル - 自分のポテンシャル
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
        /// root のポテンシャル - 自分のポテンシャル
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
fn bellman_ford(edges: &[Edge], n_vertex: usize, start: usize) -> Option<Vec<ExtInt>> {
    let mut dist = vec![INF; n_vertex];
    dist[start] = fin(0);

    for n_iter in 0..n_vertex {
        let mut updated = false;
        for edge in edges {
            if chmin!(dist[edge.to], dist[edge.from] + fin(edge.cost)) {
                updated = true
            }
        }
        if !updated {
            break;
        }
        if n_iter == n_vertex - 1 {
            // 始点からたどり着ける負閉路が存在する
            return None;
        }
    }

    Some(dist)
}
use mod_ext_int::*;
pub mod mod_ext_int {
    use ac_library::Monoid;
    use std::{
        cmp::Ordering,
        convert::Infallible,
        fmt,
        ops::{Add, AddAssign, Sub, SubAssign},
    };
    pub const INF: ExtInt = ExtInt::INF;
    pub fn fin(x: i64) -> ExtInt {
        ExtInt::fin(x)
    }
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ExtInt(i64);
    impl ExtInt {
        pub const INF: Self = Self(i64::MAX);
        pub fn fin(x: i64) -> Self {
            Self(x)
        }
        pub fn get_fin(self) -> i64 {
            if self.is_fin() {
                self.0
            } else {
                panic!("called `ExtInt::get_fin()` on a infinity")
            }
        }
        pub fn get_fin_or(self, default: i64) -> i64 {
            if self.is_fin() {
                self.0
            } else {
                default
            }
        }
        #[inline]
        pub fn is_fin(self) -> bool {
            self.0 != i64::MAX
        }
        pub fn is_inf(self) -> bool {
            self.0 == i64::MAX
        }
        pub fn to_option(self) -> Option<i64> {
            if self.is_fin() {
                Some(self.0)
            } else {
                None
            }
        }
        pub fn from_option(opt: Option<i64>) -> ExtInt {
            match opt {
                Some(a) => Self(a),
                None => Self::INF,
            }
        }
        pub fn times(self, t: i64) -> Self {
            match t.cmp(&0) {
                Ordering::Less => panic!("t must be non-negative."),
                Ordering::Equal => Self(0),
                Ordering::Greater => {
                    if self.is_fin() {
                        Self(self.0 * t)
                    } else {
                        Self::INF
                    }
                }
            }
        }
    }
    impl Add for ExtInt {
        type Output = ExtInt;
        fn add(self, rhs: Self) -> Self::Output {
            if self.is_inf() || rhs.is_inf() {
                Self::INF
            } else {
                Self::fin(self.0 + rhs.0)
            }
        }
    }
    impl AddAssign for ExtInt {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }
    impl Add<i64> for ExtInt {
        type Output = ExtInt;
        fn add(self, rhs: i64) -> Self::Output {
            if self.is_inf() {
                Self::INF
            } else {
                Self::fin(self.0 + rhs)
            }
        }
    }
    impl AddAssign<i64> for ExtInt {
        fn add_assign(&mut self, rhs: i64) {
            *self = *self + rhs;
        }
    }
    impl Sub<i64> for ExtInt {
        type Output = ExtInt;
        fn sub(self, rhs: i64) -> Self::Output {
            if self.is_inf() {
                Self::INF
            } else {
                Self::fin(self.0 - rhs)
            }
        }
    }
    impl SubAssign<i64> for ExtInt {
        fn sub_assign(&mut self, rhs: i64) {
            *self = *self - rhs;
        }
    }
    impl std::iter::Sum for ExtInt {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            let mut s = 0;
            for x in iter {
                if x.is_inf() {
                    return Self::INF;
                }
                s += x.0;
            }
            Self::fin(s)
        }
    }
    impl fmt::Display for ExtInt {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.is_inf() {
                write!(f, "+∞")
            } else {
                write!(f, "{}", self.0)
            }
        }
    }
    impl fmt::Debug for ExtInt {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.is_inf() {
                write!(f, "+∞")
            } else {
                write!(f, "{}", self.0)
            }
        }
    }
    pub struct ExtIntAdditive(Infallible);
    impl Monoid for ExtIntAdditive {
        type S = ExtInt;
        fn identity() -> Self::S {
            ExtInt::fin(0)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
    }
    pub struct ExtIntMin(Infallible);
    impl Monoid for ExtIntMin {
        type S = ExtInt;
        fn identity() -> Self::S {
            ExtInt::INF
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a.min(b)
        }
    }
}
