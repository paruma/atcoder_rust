//#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    from: usize,
    to: usize,
    t: i64,
}

impl Edge {
    fn new(from: usize, to: usize, t: i64) -> Self {
        Self { from, to, t }
    }

    fn rev(self) -> Self {
        Self {
            from: self.to,
            to: self.from,
            t: self.t,
        }
    }

    fn read() -> Self {
        input! {
            from: Usize1,
            to: Usize1,
            t: i64,
        }

        Self { from, to, t }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Query {
    k: usize,
    bs: Vec<usize>,
}

impl Query {
    fn read() -> Self {
        input! {
            k: usize,
            bs: [Usize1; k]
        }
        Query { k, bs }
    }
}
#[derive(Debug, Clone)]
struct Problem {
    nv: usize,
    ne: usize,
    edges: Vec<Edge>,
    nq: usize,
    qs: Vec<Query>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            nv: usize,
            ne: usize,
        }
        let edges = (0..ne).map(|_| Edge::read()).collect_vec();
        input! {
            nq: usize,
        }
        let qs = (0..nq).map(|_| Query::read()).collect_vec();

        Problem {
            nv,
            ne,
            edges,
            nq,
            qs,
        }
    }
    fn solve(&self) -> Answer {
        // 想定解法 (ワーシャル・フロイド法)
        let nv = self.nv;
        let ne = self.ne;
        let edges = &self.edges;
        let qs = &self.qs;

        // 全点対間最短路問題
        let dist = {
            let mut dist = vec![vec![Inf; nv]; nv];
            for e in edges {
                // 今回の問題では多重辺が存在するので、こうするとコストが大きい辺が採用されてしまう可能性がある。
                // dist[e.u][e.v] =Fin(e.t);
                // dist[e.v][e.u] = Fin(e.t);
                chmin!(dist[e.from][e.to], Fin(e.t));
                chmin!(dist[e.to][e.from], Fin(e.t));
            }
            for v in 0..nv {
                dist[v][v] = Fin(0);
            }

            for k in 0..nv {
                for from in 0..nv {
                    for to in 0..nv {
                        // from → (0..=k の頂点を0回以上通る) → to というパスでの最短路を計算
                        chmin!(dist[from][to], dist[from][k] + dist[k][to]);
                    }
                }
            }
            dist.iter()
                .map(|row| row.iter().copied().map(|x| x.get_fin()).collect_vec())
                .collect_vec()
        };

        let ans = qs
            .iter()
            .map(|q| {
                let k = q.k;
                let bs = &q.bs;
                // 橋の順番と向きを全探索
                iproduct!(
                    bs.iter().copied().permutations(k),
                    std::iter::repeat([false, true])
                        .take(k)
                        .multi_cartesian_product()
                )
                .map(|(bps, is_rev_list)| {
                    // 0 →
                    // bps[0] の始点 → bps[0]の終点
                    // → ...
                    // → bps[k-1]の始点→ bps[k-1]の終点
                    // → n-1
                    // (bits の値に応じて始点と終点は入れ替える)

                    // izip と iproduct を間違えた
                    let srcs = izip!(&bps, &is_rev_list)
                        .map(|(&b, &is_rev)| if is_rev { edges[b].to } else { edges[b].from })
                        .collect_vec();

                    let dsts = izip!(&bps, &is_rev_list)
                        .map(|(&b, &is_rev)| if is_rev { edges[b].from } else { edges[b].to })
                        .collect_vec();

                    // dbg!(&bps);
                    // dbg!(&is_rev_list);
                    // dbg!(&srcs);
                    // dbg!(&dsts);

                    dist[0][srcs[0]]
                        + (0..k - 1)
                            .map(|i| {
                                // ここの第一項、dist[srcs[i]][dsts[i]] だとダメ
                                // 別の早いルートが採用されてしまう可能性がある
                                edges[bps[i]].t + dist[dsts[i]][srcs[i + 1]]
                            })
                            .sum::<i64>()
                        + edges[bps[k - 1]].t
                        + dist[dsts[k - 1]][nv - 1]
                })
                .min()
                .unwrap()
            })
            .collect_vec();
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // 想定解法 (ワーシャル・フロイド法)
        // solve のリファクタリング
        let nv = self.nv;
        let ne = self.ne;
        let edges = &self.edges;
        let qs = &self.qs;

        // 全点対間最短路
        let dist = {
            let mut dist = vec![vec![Inf; nv]; nv];
            for e in edges {
                // 今回の問題では多重辺が存在するので、こうするとコストが大きい辺が採用されてしまう可能性がある。
                // dist[e.u][e.v] =Fin(e.t);
                // dist[e.v][e.u] = Fin(e.t);
                chmin!(dist[e.from][e.to], Fin(e.t));
                chmin!(dist[e.to][e.from], Fin(e.t));
            }
            for v in 0..nv {
                dist[v][v] = Fin(0);
            }

            for k in 0..nv {
                for from in 0..nv {
                    for to in 0..nv {
                        // from → (0..=k の頂点を0回以上通る) → to というパスでの最短路を計算
                        // k を経由するかどうかで場合分けして計算
                        chmin!(dist[from][to], dist[from][k] + dist[k][to]);
                    }
                }
            }
            dist.iter()
                .map(|row| row.iter().copied().map(|x| x.get_fin()).collect_vec())
                .collect_vec()
        };

        let ans = qs
            .iter()
            .map(|q| {
                let k = q.k;
                let bs = &q.bs;
                // 橋の順番と向きを全探索
                iproduct!(
                    bs.iter().copied().permutations(k),
                    std::iter::repeat([false, true])
                        .take(k)
                        .multi_cartesian_product()
                )
                .map(|(permuted_bs, is_rev_list)| {
                    // permuted_edges の順番に辺（橋）を訪問する
                    let permuted_edges = izip!(&permuted_bs, is_rev_list)
                        .map(|(&b, is_rev)| {
                            let e = edges[b];
                            if is_rev {
                                e.rev()
                            } else {
                                e
                            }
                        })
                        .collect_vec();
                    // 0
                    // → permuted_edges[0] の始点 → permuted_edges[0]の終点
                    // → permuted_edges[1] の始点 → permuted_edges[1]の終点
                    // → ...
                    // → permuted_edges[k-1] の始点 → permuted_edges[k-1]の終点
                    // → n-1

                    dist[0][permuted_edges[0].from]
                        + (0..k - 1)
                            .map(|i| {
                                // ここの第一項、dist[permuted_edges[i].u][permuted_edges[i + 1].v]
                                // 別の早いルートが採用されてしまう可能性がある
                                permuted_edges[i].t
                                    + dist[permuted_edges[i].to][permuted_edges[i + 1].from]
                            })
                            .sum::<i64>()
                        + permuted_edges[k - 1].t
                        + dist[permuted_edges[k - 1].to][nv - 1]
                })
                .min()
                .unwrap()
            })
            .collect_vec();
        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        // k!全探索の代わりに巡回セールスマン問題風のbitDPをする
        // (kが小さいので bitDP してもたいして早くならない)
        let nv = self.nv;
        let ne = self.ne;
        let edges = &self.edges;
        let qs = &self.qs;

        // 全点対間最短路
        let dist = {
            let mut dist = vec![vec![Inf; nv]; nv];
            for e in edges {
                // 今回の問題では多重辺が存在するので、こうするとコストが大きい辺が採用されてしまう可能性がある。
                // dist[e.u][e.v] =Fin(e.t);
                // dist[e.v][e.u] = Fin(e.t);
                chmin!(dist[e.from][e.to], Fin(e.t));
                chmin!(dist[e.to][e.from], Fin(e.t));
            }
            for v in 0..nv {
                dist[v][v] = Fin(0);
            }

            for k in 0..nv {
                for from in 0..nv {
                    for to in 0..nv {
                        // from → (0..=k の頂点を0回以上通る) → to というパスでの最短路を計算
                        // k を経由するかどうかで場合分けして計算
                        chmin!(dist[from][to], dist[from][k] + dist[k][to]);
                    }
                }
            }
            dist.iter()
                .map(|row| row.iter().copied().map(|x| x.get_fin()).collect_vec())
                .collect_vec()
        };

        let ans = qs
            .iter()
            .map(|q| {
                let k = q.k;
                let q_edges = &q.bs;
                //頂点として、0(始点), n-1(終点), q_edges に現れる辺の始点と終点 のみを考える
                let scoped_vertices = {
                    let mut scoped_vertices = HashSet::new();
                    scoped_vertices.insert(0);
                    scoped_vertices.insert(nv - 1);

                    for &ei in q_edges {
                        let edge = edges[ei];
                        scoped_vertices.insert(edge.from);
                        scoped_vertices.insert(edge.to);
                    }

                    scoped_vertices.iter().copied().collect_vec()
                };

                let rev_scoped_vertices = scoped_vertices.iter().copied().enumerate().fold(
                    vec![usize::MAX; nv],
                    |mut acc, (i, x)| {
                        acc[x] = i;
                        acc
                    },
                );

                // dp[S][i] = 頂点 0 から集合 S に含まれる辺をすべて通って rev_scoped_vertices[i] にたどり着いたときのコストの最小値
                let mut dp = vec![vec![Inf; scoped_vertices.len()]; 1 << k];
                for i in 0..scoped_vertices.len() {
                    dp[BitSet::empty().to_bit()][i] = Fin(dist[0][scoped_vertices[i]]);
                }

                // 0..1<<k の更新順序でうまくいく。
                for visited_q_edge_set in 0..1 << k {
                    let visited_q_edge_set = BitSet::new(visited_q_edge_set);
                    for (qei, ei) in q_edges.iter().copied().enumerate() {
                        if visited_q_edge_set.contains(qei) {
                            continue;
                        }
                        let next_visited_q_edge_set = visited_q_edge_set.insert(qei);

                        let edge = edges[ei];
                        let rev_edge = edge.rev();

                        // 辺 edge を使う
                        chmin!(
                            dp[next_visited_q_edge_set.to_bit()][rev_scoped_vertices[edge.to]],
                            dp[visited_q_edge_set.to_bit()][rev_scoped_vertices[edge.from]]
                                + edge.t
                        );

                        // 辺 rev_edge を使う
                        chmin!(
                            dp[next_visited_q_edge_set.to_bit()][rev_scoped_vertices[rev_edge.to]],
                            dp[visited_q_edge_set.to_bit()][rev_scoped_vertices[rev_edge.from]]
                                + edge.t
                        );

                        // 辺 edge や 辺 rev_edge の終点から各頂点への遷移を考えて、各頂点のコストを更新する
                        for i in 0..scoped_vertices.len() {
                            let v = scoped_vertices[i];
                            chmin!(
                                dp[next_visited_q_edge_set.to_bit()][i],
                                dp[next_visited_q_edge_set.to_bit()][rev_scoped_vertices[edge.to]]
                                    + dist[edge.to][v]
                            );
                            chmin!(
                                dp[next_visited_q_edge_set.to_bit()][i],
                                dp[next_visited_q_edge_set.to_bit()]
                                    [rev_scoped_vertices[rev_edge.to]]
                                    + dist[rev_edge.to][v]
                            );
                        }
                    }
                }
                dp[BitSet::universal_set(k).to_bit()][rev_scoped_vertices[nv - 1]].get_fin()
            })
            .collect_vec();
        Answer { ans }
    }

    fn solve_tle(&self) -> Answer {
        // TLE 解法
        // 2^k頂点倍化 ダイクストラ法
        let nv = self.nv;
        let ne = self.ne;
        let edges = &self.edges;
        let qs = &self.qs;

        let adj = edges
            .iter()
            .copied()
            .enumerate()
            .fold(vec![vec![]; nv], |mut acc, (i, e)| {
                acc[e.from].push((i, e));
                acc[e.to].push((i, e.rev()));
                acc
            });

        let ans = qs
            .iter()
            .map(|q| {
                // 頂点倍化
                let k = q.k;
                let bs = &q.bs;
                let mut pq: BinaryHeap<(Reverse<ExtInt>, usize, usize)> = BinaryHeap::new();
                let mut dist: Vec<Vec<ExtInt>> = vec![vec![Inf; 1 << k]; nv];
                let start = 0;
                dist[start][0] = Fin(0);
                pq.push((Reverse(Fin(0)), start, 0));
                let mut j_map: Vec<Option<usize>> = vec![None; ne];

                for (i, j) in bs.iter().enumerate() {
                    j_map[*j] = Some(i);
                }

                while let Some((Reverse(d), current_pos, current_bits)) = pq.pop() {
                    if dist[current_pos][current_bits] < d {
                        continue;
                    }
                    let current_bits = BitSet::new(current_bits);
                    for (i, e) in &adj[current_pos] {
                        //let j_opt = bs.iter().copied().position(|j| j == *i);
                        let j_opt = j_map[*i];

                        //dbg!(&j_map);
                        //assert_eq!(j_opt, j_opt_2);

                        let next_bits = if let Some(j) = j_opt {
                            current_bits.insert(j)
                        } else {
                            current_bits
                        };
                        //dbg!(next_bits);
                        //dbg!(k);
                        //dbg!(e);
                        if chmin!(
                            dist[e.to][next_bits.to_bit()],
                            dist[e.from][current_bits.to_bit()] + Fin(e.t)
                        ) {
                            pq.push((
                                Reverse(dist[e.to][next_bits.to_bit()]),
                                e.to,
                                next_bits.to_bit(),
                            ));
                        }
                    }
                }
                dist[nv - 1][(1 << k) - 1].get_fin()
            })
            .collect_vec();

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
        print_vec(&self.ans);
    }
}

fn main() {
    Problem::read().solve3().print();
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
use petgraph::visit;
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
use bitset::*;
#[allow(clippy::module_inception)]
pub mod bitset {
    use itertools::Itertools;
    use std::{
        fmt::{Error, Formatter},
        ops::{BitAnd, BitOr, BitXor},
    };
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct BitSet {
        bit: usize,
    }
    impl BitSet {
        #[inline]
        pub fn new(bit: usize) -> BitSet {
            BitSet { bit }
        }
        pub fn to_bit(self) -> usize {
            self.bit
        }
        /// 持っている要素を Vec<usize> で返す
        pub fn to_vec(self, len: usize) -> Vec<usize> {
            (0..len).filter(|i| (self.bit >> i) & 1 == 1).collect_vec()
        }
        pub fn contains(self, x: usize) -> bool {
            (self.bit >> x) & 1 == 1
        }
        pub fn count(self) -> usize {
            self.bit.count_ones() as usize
        }
        pub fn insert(self, x: usize) -> BitSet {
            BitSet::new(self.bit | (1 << x))
        }
        pub fn remove(self, x: usize) -> BitSet {
            BitSet::new(self.bit & !(1 << x))
        }
        pub fn empty() -> BitSet {
            BitSet::new(0)
        }
        pub fn universal_set(size: usize) -> BitSet {
            BitSet::new((1 << size) - 1)
        }
        pub fn complement(self, size: usize) -> BitSet {
            BitSet::new(self.bit ^ ((1 << size) - 1))
        }
        pub fn set_minus(self, other: BitSet) -> BitSet {
            BitSet::new(self.bit & !other.bit)
        }
        pub fn is_empty(self) -> bool {
            self.bit == 0
        }
        pub fn is_subset(self, other: BitSet) -> bool {
            self | other == other
        }
        pub fn all_subset(size: usize) -> impl Iterator<Item = BitSet> {
            (0..(1 << size)).map(BitSet::new)
        }
        pub fn subsets(self) -> impl Iterator<Item = BitSet> {
            std::iter::successors(Some(self.bit), move |x| {
                if *x == 0 {
                    None
                } else {
                    Some((x - 1) & self.bit)
                }
            })
            .map(BitSet::new)
        }
    }
    impl BitAnd for BitSet {
        type Output = BitSet;
        fn bitand(self, rhs: BitSet) -> BitSet {
            BitSet::new(self.bit & rhs.bit)
        }
    }
    impl BitOr for BitSet {
        type Output = BitSet;
        fn bitor(self, rhs: BitSet) -> BitSet {
            BitSet::new(self.bit | rhs.bit)
        }
    }
    impl BitXor for BitSet {
        type Output = BitSet;
        fn bitxor(self, rhs: BitSet) -> BitSet {
            BitSet::new(self.bit ^ rhs.bit)
        }
    }
    use std::fmt::Debug;
    impl Debug for BitSet {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            f.write_fmt(format_args!("{:#b}", self.bit))?;
            Ok(())
        }
    }
}
use mod_ext_int::ExtInt::{self, *};
pub mod mod_ext_int {
    use ac_library::Monoid;
    use std::{
        cmp::Ordering,
        convert::Infallible,
        iter::Sum,
        ops::{Add, AddAssign},
    };
    use ExtInt::*;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ExtInt {
        Inf,
        Fin(i64),
    }
    impl ExtInt {
        pub fn get_fin(self) -> i64 {
            match self {
                Fin(val) => val,
                Inf => panic!("called `ExtInt::get_fin()` on a `Fin` value"),
            }
        }
        pub fn get_fin_or(self, default: i64) -> i64 {
            match self {
                Fin(val) => val,
                Inf => default,
            }
        }
        pub fn is_fin(self) -> bool {
            matches!(self, Fin(_))
        }
        pub fn is_inf(self) -> bool {
            matches!(self, Inf)
        }
        pub fn to_option(self) -> Option<i64> {
            match self {
                Inf => None,
                Fin(a) => Some(a),
            }
        }
        pub fn from_option(opt: Option<i64>) -> ExtInt {
            match opt {
                Some(a) => Fin(a),
                None => Inf,
            }
        }
        pub fn times(self, t: i64) -> Self {
            match t.cmp(&0) {
                Ordering::Less => panic!("t must be non-negative."),
                Ordering::Equal => Fin(0),
                Ordering::Greater => match self {
                    Inf => Inf,
                    Fin(a) => Fin(a * t),
                },
            }
        }
    }
    impl Add for ExtInt {
        type Output = ExtInt;
        fn add(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (Inf, Inf) => Inf,
                (Inf, Fin(_)) => Inf,
                (Fin(_), Inf) => Inf,
                (Fin(a), Fin(b)) => Fin(a + b),
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
            match self {
                Inf => Inf,
                Fin(a) => Fin(a + rhs),
            }
        }
    }
    impl AddAssign<i64> for ExtInt {
        fn add_assign(&mut self, rhs: i64) {
            *self = *self + rhs;
        }
    }
    impl Sum for ExtInt {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            let mut s = 0;
            for x in iter {
                match x {
                    Inf => return Inf,
                    Fin(x) => s += x,
                }
            }
            Fin(s)
        }
    }
    impl PartialOrd for ExtInt {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match (self, other) {
                (Inf, Inf) => Some(Ordering::Equal),
                (Inf, Fin(_)) => Some(Ordering::Greater),
                (Fin(_), Inf) => Some(Ordering::Less),
                (Fin(a), Fin(b)) => PartialOrd::partial_cmp(a, b),
            }
        }
    }
    impl Ord for ExtInt {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }
    pub struct ExtIntAdditive(Infallible);
    impl Monoid for ExtIntAdditive {
        type S = ExtInt;
        fn identity() -> Self::S {
            Fin(0)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
    }
    pub struct ExtIntMin(Infallible);
    impl Monoid for ExtIntMin {
        type S = ExtInt;
        fn identity() -> Self::S {
            Inf
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a.min(b)
        }
    }
}
#[allow(clippy::module_inception)]
#[macro_use]
pub mod chminmax {
    #[allow(unused_macros)]
    #[macro_export]
    macro_rules! chmin {
        ($ a : expr , $ b : expr ) => {
            if $a > $b {
                $a = $b;
                true
            } else {
                false
            }
        };
    }
    #[allow(unused_macros)]
    #[macro_export]
    macro_rules! chmax {
        ($ a : expr , $ b : expr ) => {
            if $a < $b {
                $a = $b;
                true
            } else {
                false
            }
        };
    }
}
