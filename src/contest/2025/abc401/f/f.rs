//#[derive_readable]
#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    u: Usize1,
    v: Usize1,
}
#[derive(Debug, Clone)]
struct Problem {
    nv1: usize,
    nv2: usize,
    es1: Vec<Edge>,
    es2: Vec<Edge>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            nv1: usize,
            es1: [Edge; nv1-1],
            nv2: usize,
            es2: [Edge; nv2-1],

        }
        Problem { nv1, nv2, es1, es2 }
    }

    fn solve(&self) -> Answer {
        let nv1 = self.nv1;
        let nv2 = self.nv2;

        let adj1 = self
            .es1
            .iter()
            .copied()
            .fold(vec![vec![]; nv1], |mut acc, e| {
                acc[e.u].push(e.v);
                acc[e.v].push(e.u);
                acc
            });
        let adj2 = self
            .es2
            .iter()
            .copied()
            .fold(vec![vec![]; nv2], |mut acc, e| {
                acc[e.u].push(e.v);
                acc[e.v].push(e.u);
                acc
            });

        let max_dist1 = DistMaxReroot {}
            .reroot(&adj1)
            .iter()
            .map(|i| *i as i64)
            .collect_vec();
        let max_dist2 = DistMaxReroot {}
            .reroot(&adj2)
            .iter()
            .map(|i| *i as i64)
            .sorted()
            .collect_vec();

        let max_dist2_cumsum = CumSum::new(&max_dist2);

        let diam1 = max_dist1.iter().copied().max().unwrap();
        let diam2 = max_dist2.iter().copied().max().unwrap();

        let max_diam = i64::max(diam1, diam2);

        let ans = (0..nv1)
            .map(|i| {
                // max_dist1[i] + 1 + max_dist2[j] < max_diam
                // max_dist2[j] < max_diam - max_dist1[i] - 1
                // だったら max_diam を使う

                let bound = max_dist2.lower_bound(&(max_diam - max_dist1[i] - 1));
                bound as i64 * max_diam
                    + (nv2 - bound) as i64 * (max_dist1[i] + 1)
                    + max_dist2_cumsum.range_sum(bound..nv2)
            })
            .sum::<i64>();
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // 頻度分布の畳み込みを使う
        let nv1 = self.nv1;
        let nv2 = self.nv2;

        let adj1 = self
            .es1
            .iter()
            .copied()
            .fold(vec![vec![]; nv1], |mut acc, e| {
                acc[e.u].push(e.v);
                acc[e.v].push(e.u);
                acc
            });
        let adj2 = self
            .es2
            .iter()
            .copied()
            .fold(vec![vec![]; nv2], |mut acc, e| {
                acc[e.u].push(e.v);
                acc[e.v].push(e.u);
                acc
            });

        let max_dist1 = DistMaxReroot {}
            .reroot(&adj1)
            .iter()
            .map(|i| *i as i64)
            .collect_vec();
        let max_dist2 = DistMaxReroot {}
            .reroot(&adj2)
            .iter()
            .map(|i| *i as i64)
            .sorted()
            .collect_vec();

        let diam1 = max_dist1.iter().copied().max().unwrap();
        let diam2 = max_dist2.iter().copied().max().unwrap();
        let max_diam = i64::max(diam1, diam2);

        let max_dist1_cnts = max_dist1
            .iter()
            .copied()
            .fold(vec![0; nv1 + 1], |mut acc, d| {
                acc[d as usize] += 1;
                acc
            });

        let max_dist2_cnts = max_dist2
            .iter()
            .copied()
            .fold(vec![0; nv2 + 1], |mut acc, d| {
                acc[d as usize] += 1;
                acc
            });

        let dist_sum_cnts = ac_library::convolution_i64(&max_dist1_cnts, &max_dist2_cnts);

        let term1 = (0..max_diam - 1)
            .map(|d| dist_sum_cnts[d as usize])
            .sum::<i64>()
            * max_diam;

        let term2 = ((max_diam - 1) as usize..dist_sum_cnts.len())
            .map(|d| dist_sum_cnts[d] * (d + 1) as i64)
            .sum::<i64>();

        let ans = term1 + term2;
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

use ac_library::{Max, Monoid};
// ====== import ======
#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, izip};
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet};
use superslice::Ext;

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

struct DistMaxReroot();
impl Reroot for DistMaxReroot {
    type M = Max<u64>;

    fn add_vertex(&self, x: &<Self::M as Monoid>::S, _v: usize) -> <Self::M as Monoid>::S {
        *x
    }

    fn add_edge(
        &self,
        x: &<Self::M as Monoid>::S,
        _v: usize,
        _ei: usize,
    ) -> <Self::M as Monoid>::S {
        x + 1
    }
}
use reroot::*;
#[allow(clippy::module_inception)]
pub mod reroot {
    use ac_library::Monoid;
    use mod_queue::*;
    /// 全方位木DPを行うためのトレイトです。
    pub trait Reroot {
        type M: Monoid;
        /// 子部分木+辺の集約結果に頂点の値を加えます。
        /// # Arguments
        /// * `x` - 頂点 `v` の各子の「部分木+辺」の集約値
        /// * `v` - 対象の頂点
        /// # Returns
        /// `x` に頂点 `v` 自身の値を加えた結果
        fn add_vertex(&self, x: &<Self::M as Monoid>::S, v: usize) -> <Self::M as Monoid>::S;
        /// 部分木の集約結果にエッジの値を加えます。
        /// # Arguments
        /// * `x` - 隣接頂点 `adj[v][ei]` を根とする部分木の集約値
        /// * `v` - エッジの始点
        /// * `ei` - 頂点 `v` の `ei` 番目のエッジ（隣接頂点 `adj[v][ei]`）
        /// # Returns
        /// `x` にエッジ `v--adj[v][ei]` に関する値を加えた結果
        fn add_edge(
            &self,
            x: &<Self::M as Monoid>::S,
            v: usize,
            ei: usize,
        ) -> <Self::M as Monoid>::S;
        /// 全方位木DPを実行し、各頂点を根としたときの値を求めます。
        /// 具体的には、頂点 `u` を根とした根付き木において、
        /// 各頂点 `v` の値 `f_u(v)` を以下で再帰的に定義します：
        /// ```text
        /// f_u(v) = add_vertex(⊕_{c ∈ ch_u(v)} add_edge(f_u(c), v, index(v, c)), v)
        /// ```
        /// ここで `ch_u(v)` は `u` を根とした時の `v` の子頂点の集合、
        /// `index(v, c)` は `adj[v]` における `c` のインデックス、
        /// ⊕ はモノイドの二項演算です。
        /// 返り値を `result` とすると、`result[u] = f_u(u)` です。
        /// # Arguments
        /// * `adj` - 木の隣接リスト
        /// # 計算量
        /// O(V) (V は頂点数)
        fn reroot(&self, adj: &[Vec<usize>]) -> Vec<<Self::M as Monoid>::S> {
            let n = adj.len();
            if n == 0 {
                return vec![];
            }
            if n == 1 {
                return vec![self.add_vertex(&Self::M::identity(), 0)];
            }
            let (children, parent, bfs_order) = {
                let mut children = vec![vec![]; n];
                let mut parent = vec![None; n];
                let mut bfs_order = Vec::with_capacity(n);
                let mut queue = Queue::new();
                let mut visited = vec![false; n];
                visited[0] = true;
                queue.push(0);
                while let Some(cur) = queue.pop() {
                    bfs_order.push(cur);
                    for (cur_to_next, &next) in adj[cur].iter().enumerate() {
                        if !visited[next] {
                            visited[next] = true;
                            let next_to_cur = adj[next]
                                .iter()
                                .position(|&back| back == cur)
                                .expect("Edge must be bidirectional");
                            children[cur].push((next, cur_to_next, next_to_cur));
                            parent[next] = Some((cur, next_to_cur, cur_to_next));
                            queue.push(next);
                        }
                    }
                }
                (children, parent, bfs_order)
            };
            let mut dp: Vec<Vec<<Self::M as Monoid>::S>> = adj
                .iter()
                .map(|next_list| vec![Self::M::identity(); next_list.len()])
                .collect();
            for &u in bfs_order.iter().rev() {
                if let Some((p, _u_to_p, p_to_u)) = parent[u] {
                    let res = children[u]
                        .iter()
                        .map(|&(_c, u_to_c, _c_to_u)| self.add_edge(&dp[u][u_to_c], u, u_to_c))
                        .fold(Self::M::identity(), |acc, val| {
                            Self::M::binary_operation(&acc, &val)
                        });
                    dp[p][p_to_u] = self.add_vertex(&res, u);
                }
            }
            for &u in &bfs_order {
                if children[u].is_empty() {
                    continue;
                }
                let edge_values: Vec<_> = dp[u]
                    .iter()
                    .enumerate()
                    .map(|(i, x)| self.add_edge(x, u, i))
                    .collect();
                let cum = CumMonoid::<Self::M>::new(&edge_values);
                for &(c, u_to_c, c_to_u) in &children[u] {
                    let res_without_c = cum.prod_without1(u_to_c);
                    dp[c][c_to_u] = self.add_vertex(&res_without_c, u);
                }
            }
            (0..n)
                .map(|u| {
                    let res = dp[u]
                        .iter()
                        .enumerate()
                        .map(|(i, x)| self.add_edge(x, u, i))
                        .fold(Self::M::identity(), |acc, val| {
                            Self::M::binary_operation(&acc, &val)
                        });
                    self.add_vertex(&res, u)
                })
                .collect()
        }
    }
    /// 累積積を効率的に計算するための構造体です。
    struct CumMonoid<M: Monoid> {
        prefix: Vec<M::S>,
        suffix: Vec<M::S>,
    }
    impl<M: Monoid> CumMonoid<M> {
        fn new(xs: &[M::S]) -> Self {
            let n = xs.len();
            let mut prefix = vec![M::identity(); n + 1];
            let mut suffix = vec![M::identity(); n + 1];
            for i in 0..n {
                prefix[i + 1] = M::binary_operation(&prefix[i], &xs[i]);
            }
            for i in (0..n).rev() {
                suffix[i] = M::binary_operation(&xs[i], &suffix[i + 1]);
            }
            Self { prefix, suffix }
        }
        /// インデックス `i` の要素を除いた全体の積を求めます。
        fn prod_without1(&self, i: usize) -> M::S {
            M::binary_operation(&self.prefix[i], &self.suffix[i + 1])
        }
    }
    mod mod_queue {
        use std::collections::VecDeque;
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct Queue<T> {
            raw: VecDeque<T>,
        }
        impl<T> Queue<T> {
            #[allow(clippy::new_without_default)]
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
        }
    }
    use ac_library::Max;
    #[derive(Clone, Copy, Debug)]
    pub struct DistMaxReroot;
    impl Reroot for DistMaxReroot {
        type M = Max<u64>;
        fn add_vertex(&self, x: &u64, _v: usize) -> u64 {
            *x
        }
        fn add_edge(&self, x: &u64, _v: usize, _ei: usize) -> u64 {
            x + 1
        }
    }
}
use cumsum::*;
#[allow(clippy::module_inception)]
pub mod cumsum {
    pub fn prefix_sum(xs: &[i64]) -> Vec<i64> {
        let mut prefix_sum = vec![0; xs.len() + 1];
        for i in 1..xs.len() + 1 {
            prefix_sum[i] = prefix_sum[i - 1] + xs[i - 1];
        }
        prefix_sum
    }
    use std::ops::{Bound, Range, RangeBounds};
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct CumSum {
        pub cumsum: Vec<i64>,
    }
    impl CumSum {
        /// # 計算量
        /// O(|xs|)
        pub fn new(xs: &[i64]) -> CumSum {
            let mut cumsum = vec![0; xs.len() + 1];
            for i in 1..xs.len() + 1 {
                cumsum[i] = cumsum[i - 1] + xs[i - 1];
            }
            CumSum { cumsum }
        }
        fn open(&self, range: impl RangeBounds<usize>) -> Range<usize> {
            use Bound::Excluded;
            use Bound::Included;
            use Bound::Unbounded;
            let begin = match range.start_bound() {
                Unbounded => 0,
                Included(&x) => x,
                Excluded(&x) => x + 1,
            };
            let end = match range.end_bound() {
                Excluded(&x) => x,
                Included(&x) => x + 1,
                Unbounded => self.cumsum.len() - 1,
            };
            begin..end
        }
        /// 区間 `[begin, end)` の要素の和を計算します。
        /// # 計算量
        /// O(1)
        pub fn range_sum(&self, range: impl RangeBounds<usize>) -> i64 {
            let range = self.open(range);
            self.cumsum[range.end] - self.cumsum[range.start]
        }
        /// 区間 `[0, end)` での和を計算します。
        /// # 計算量
        /// O(1)
        pub fn prefix_sum(&self, end: usize) -> i64 {
            self.cumsum[end]
        }
        /// 区間 `[begin, n)` の要素の和を計算します。（`n` は元の配列の長さ）
        /// # 計算量
        /// O(1)
        pub fn suffix_sum(&self, begin: usize) -> i64 {
            self.cumsum[self.cumsum.len() - 1] - self.cumsum[begin]
        }
        /// `f(sum(l..r))` が `true` となる最大の `r in [l, n]` を見つける。
        /// `n` は元の配列の長さ。
        /// `f` は単調でなければならない。
        /// `f(sum(l..i))` が `true` => `f(sum(l..j))` が `true` for all `l <= j <= i`.
        /// # Panics
        /// `l > n` の場合にパニックする。
        /// # 計算量
        /// O(log n)
        pub fn max_right<F>(&self, l: usize, mut f: F) -> usize
        where
            F: FnMut(i64) -> bool,
        {
            let n = self.cumsum.len() - 1;
            assert!(l <= n);
            assert!(f(0), "f(0) must be true");
            if f(self.range_sum(l..n)) {
                return n;
            }
            let mut ok = l;
            let mut ng = n + 1;
            while ng - ok > 1 {
                let mid = ok + (ng - ok) / 2;
                if f(self.range_sum(l..mid)) {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok
        }
        /// `f(sum(l..r))` が `true` となる最小の `l in [0, r]` を見つける。
        /// `f` は単調でなければならない。
        /// `f(sum(i..r))` が `true` => `f(sum(j..r))` が `true` for all `i <= j <= r`.
        /// `r > n` の場合にパニックする。
        /// # 計算量
        /// O(log r)
        pub fn min_left<F>(&self, r: usize, mut f: F) -> usize
        where
            F: FnMut(i64) -> bool,
        {
            let n = self.cumsum.len() - 1;
            assert!(r <= n);
            assert!(f(0), "f(0) must be true");
            if f(self.range_sum(0..r)) {
                return 0;
            }
            let mut ok = r;
            let mut ng = 0;
            while ok - ng > 1 {
                let mid = ng + (ok - ng) / 2;
                if f(self.range_sum(mid..r)) {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok
        }
    }
}
