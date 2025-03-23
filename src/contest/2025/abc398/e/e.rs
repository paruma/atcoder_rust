#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    u: Usize1,
    v: Usize1,
}

enum Response {
    YouWin,
    Operate { i: usize, j: usize },
}
trait IInteractive {
    fn select_first(&mut self);
    fn select_second(&mut self) -> Response;
    fn operate(&mut self, i: usize, j: usize) -> Response;
}

struct StdinInteractive;
impl StdinInteractive {
    fn get_response(&self) -> Response {
        input_interactive! {
            a: i64,
            b: i64,
        }
        match (a, b) {
            (-1, -1) => Response::YouWin,
            (a, b) => Response::Operate {
                i: (a - 1) as usize,
                j: (b - 1) as usize,
            },
        }
    }
}
impl IInteractive for StdinInteractive {
    fn operate(&mut self, i: usize, j: usize) -> Response {
        println_flush!("{} {}", i + 1, j + 1);
        self.get_response()
    }

    fn select_first(&mut self) {
        println_flush!("First");
    }

    fn select_second(&mut self) -> Response {
        println_flush!("Second");
        self.get_response()
    }
}

// 未実装。手元でテストしたい場合に実装する
struct TestInteractive {
    xs: Vec<i64>,
    cnt_ask: usize,
}

impl TestInteractive {
    fn new(xs: Vec<i64>) -> TestInteractive {
        TestInteractive { xs, cnt_ask: 0 }
    }
}

impl IInteractive for TestInteractive {
    fn select_first(&mut self) {
        todo!()
    }

    fn select_second(&mut self) -> Response {
        todo!()
    }

    fn operate(&mut self, i: usize, j: usize) -> Response {
        todo!()
    }
}

fn solve<T: IInteractive>(asker: &mut T, n: usize, es: &[Edge]) {
    let tree_parent = {
        let mut tree_parent = vec![n + 1; n];
        let adj = es.iter().copied().fold(vec![vec![]; n], |mut acc, e| {
            acc[e.u].push(e.v);
            acc[e.v].push(e.u);
            acc
        });
        let root = 0;
        let mut open = Queue::new();
        let mut visited = vec![false; n];
        open.push(root);
        visited[root] = true;
        tree_parent[root] = root;

        while let Some(cur) = open.pop() {
            for &to in &adj[cur] {
                if !visited[to] {
                    visited[to] = true;
                    open.push(to);
                    tree_parent[to] = cur;
                }
            }
        }
        tree_parent
    };
    let lca = Lca::new(&tree_parent);

    let mut cands = (0..n)
        .tuple_combinations()
        .filter(|&(i, j)| {
            let dist = lca.dist(i, j);
            dist >= 3 && dist % 2 == 1
        })
        .collect::<BTreeSet<_>>();

    if cands.len() % 2 == 0 {
        let res = asker.select_second();
        match res {
            Response::YouWin => {
                return;
            }
            Response::Operate { i, j } => {
                cands.remove(&(i, j));
                cands.remove(&(j, i));
            }
        }
    } else {
        asker.select_first();
    }

    loop {
        let &(i, j) = cands.iter().next().unwrap();

        cands.remove(&(i, j));
        cands.remove(&(j, i));

        let res = asker.operate(i, j);

        match res {
            Response::YouWin => {
                return;
            }
            Response::Operate { i, j } => {
                cands.remove(&(i, j));
                cands.remove(&(j, i));
            }
        }
    }
}

fn main() {
    input_interactive! {
        n: usize,
        es: [Edge; n-1],
    }
    solve(&mut StdinInteractive, n, &es);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_problem() {
        let xs = vec![0, 0, 1, 0, 0, 1, 1];
        let n = xs.len();
        // let mut asker = TestInteractive::new(xs.clone());
        // let ans = solve(&mut asker, n);
        // dbg!(asker.cnt_ask);
        // assert!(xs[ans] != xs[ans + 1]);
    }
}

use std::{
    collections::BTreeSet,
    io::{stdout, Write},
};

// ====== import ======
use proconio::input_interactive;

#[allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout,
    marker::{Bytes, Chars, Usize1},
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

#[macro_export]
macro_rules! println_flush {
    () => {
        println!();
        stdout().flush().unwrap();
    };
    ($($arg:tt)*) => {{
        println!($($arg)*);
        stdout().flush().unwrap();
    }};
}

use lca_euler_tour::*;
#[allow(clippy::module_inception)]
pub mod lca_euler_tour {
    use ac_library::{Monoid, Segtree};
    use itertools::Itertools;
    use std::convert::Infallible;
    pub struct MinI64Usize(Infallible);
    impl Monoid for MinI64Usize {
        type S = (i64, usize);
        fn identity() -> Self::S {
            (i64::MAX, usize::MAX)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            std::cmp::min(*a, *b)
        }
    }
    pub struct Lca {
        dist: Vec<i64>,
        euler_tour_dist: Segtree<MinI64Usize>,
        euler_tour_in_time: Vec<usize>,
        #[allow(dead_code)]
        euler_tour_out_time: Vec<usize>,
    }
    impl Lca {
        /// tree_parent[i]: i の 親 を表す。根の場合は tree_parent[i] == i
        /// 計算量: O(nv log(nv)) (nv は頂点の数とする)
        pub fn new(tree_parent: &[usize]) -> Self {
            let nv = tree_parent.len();
            let tree_children = tree_parent.iter().copied().enumerate().fold(
                vec![vec![]; nv],
                |mut acc, (child, parent)| {
                    if child != parent {
                        acc[parent].push(child);
                    }
                    acc
                },
            );
            let root = (0..nv).find(|&v| tree_parent[v] == v).unwrap();
            let dist = {
                fn dfs(dist: &mut [i64], current: usize, tree_children: &[Vec<usize>]) {
                    for &child in &tree_children[current] {
                        dist[child] = dist[current] + 1;
                        dfs(dist, child, tree_children);
                    }
                }
                let mut dist = vec![0; nv];
                dfs(&mut dist, root, &tree_children);
                dist
            };
            let (euler_tour, euler_tour_in_time, euler_tour_out_time) = {
                fn dfs(
                    tour: &mut Vec<usize>,
                    in_time: &mut [usize],
                    out_time: &mut [usize],
                    current: usize,
                    tree_children: &[Vec<usize>],
                ) {
                    in_time[current] = in_time[current].min(tour.len());
                    out_time[current] = out_time[current].max(tour.len());
                    tour.push(current);
                    for &child in &tree_children[current] {
                        dfs(tour, in_time, out_time, child, tree_children);
                        in_time[current] = in_time[current].min(tour.len());
                        out_time[current] = out_time[current].max(tour.len());
                        tour.push(current);
                    }
                }
                let mut tour = vec![];
                let mut in_time = vec![usize::MAX; nv];
                let mut out_time = vec![usize::MIN; nv];
                dfs(&mut tour, &mut in_time, &mut out_time, root, &tree_children);
                (tour, in_time, out_time)
            };
            let euler_tour_dist = Segtree::<MinI64Usize>::from(
                euler_tour
                    .iter()
                    .copied()
                    .map(|v| (dist[v], v))
                    .collect_vec(),
            );
            Lca {
                dist,
                euler_tour_dist,
                euler_tour_in_time,
                euler_tour_out_time,
            }
        }
        /// u と v の LCA を求める
        /// 計算量 O(log(頂点の数))
        pub fn lca(&self, u: usize, v: usize) -> usize {
            let (time_min, time_max) = {
                use std::cmp::{max, min};
                let t1 = self.euler_tour_in_time[u];
                let t2 = self.euler_tour_in_time[v];
                (min(t1, t2), max(t1, t2))
            };
            self.euler_tour_dist.prod(time_min..=time_max).1
        }
        /// 計算量: O(log(頂点の数))
        pub fn dist(&self, u: usize, v: usize) -> i64 {
            self.dist[u] + self.dist[v] - 2 * self.dist[self.lca(u, v)]
        }
        /// パス u-v 上に点 a があるかどうか
        /// 計算量: O(log(頂点の数))
        pub fn is_path_on(&self, u: usize, v: usize, a: usize) -> bool {
            self.dist(u, a) + self.dist(a, v) == self.dist(u, v)
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
            self.raw.push_back(value)
        }
        pub fn pop(&mut self) -> Option<T> {
            self.raw.pop_front()
        }
        pub fn peek(&self) -> Option<&T> {
            self.raw.front()
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
