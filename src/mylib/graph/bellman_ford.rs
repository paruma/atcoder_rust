use cargo_snippet::snippet;

// use bellman_ford::*; だと色々 import されて名前空間がぐちゃぐちゃになりそうなので prefix には指定していない。
#[allow(clippy::module_inception)]
#[snippet(include = "mod_ext_int")]
pub mod bellman_ford {

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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    /// 最短路問題の解。双対問題を考える場合は非有界と非許容が逆になるので注意
    pub enum Solution {
        /// 非有界 (負閉路を経由することでいくらでもパスを短くできる。-∞ 扱い)
        Unbounded,
        /// 有界
        Bounded(i64),
        /// 非許容 (連結でなくて到達不可能。 +∞ 扱い)
        Infeasible,
    }

    impl Solution {
        /// 双対問題の解に変換する (非有界と非許容が入れ替わる)
        pub fn dual(&self) -> Self {
            match self {
                Solution::Unbounded => Solution::Infeasible,
                Solution::Bounded(x) => Solution::Bounded(*x),
                Solution::Infeasible => Solution::Unbounded,
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Edge {
        from: usize,
        to: usize,
        cost: i64,
    }
    impl Edge {
        pub fn new(from: usize, to: usize, cost: i64) -> Self {
            Self { from, to, cost }
        }
    }
    pub fn bellman_ford(edges: &[Edge], n_vertex: usize, start: usize) -> Vec<Solution> {
        let mut dist = vec![INF; n_vertex];
        dist[start] = fin(0);

        // Bellman-Ford 本体
        for _ in 0..n_vertex {
            for edge in edges {
                chmin!(dist[edge.to], dist[edge.from] + fin(edge.cost));
            }
        }

        // 隣接リストを構築
        let mut adj = vec![vec![]; n_vertex];
        for edge in edges {
            adj[edge.from].push(edge.to);
        }

        // dist をさらに更新できる（＝負閉路に関与する）頂点を列挙
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        let mut is_unbounded = vec![false; n_vertex];
        let mut visited = vec![false; n_vertex];

        for edge in edges {
            if dist[edge.to] > dist[edge.from] + fin(edge.cost) {
                if !visited[edge.to] {
                    visited[edge.to] = true;
                    queue.push_back(edge.to);
                }
            }
        }

        // 負閉路の影響を BFS で伝播
        while let Some(u) = queue.pop_front() {
            is_unbounded[u] = true;
            for &v in &adj[u] {
                if !visited[v] {
                    visited[v] = true;
                    queue.push_back(v);
                }
            }
        }

        // 結果構築
        dist.into_iter()
            .zip(is_unbounded)
            .map(|(d, unbounded)| {
                if unbounded {
                    Solution::Unbounded
                } else if d.is_inf() {
                    Solution::Infeasible
                } else {
                    Solution::Bounded(d.get_fin())
                }
            })
            .collect()
    }

    use mod_ext_int::*;
    #[allow(dead_code)]
    mod mod_ext_int {
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
}

#[cfg(test)]
mod tests {
    use super::bellman_ford::*;

    #[test]
    fn test_basic_case() {
        let edges = vec![Edge::new(0, 1, 3), Edge::new(1, 2, 4), Edge::new(0, 2, 10)];
        let res = bellman_ford(&edges, 3, 0);
        assert_eq!(
            res,
            vec![
                Solution::Bounded(0),
                Solution::Bounded(3),
                Solution::Bounded(7)
            ]
        );
    }

    #[test]
    fn test_infeasible() {
        let edges = vec![Edge::new(0, 1, 1)];
        let res = bellman_ford(&edges, 3, 0);
        assert_eq!(
            res,
            vec![
                Solution::Bounded(0),
                Solution::Bounded(1),
                Solution::Infeasible
            ]
        );
    }

    #[test]
    fn test_negative_cycle() {
        let edges = vec![
            Edge::new(0, 1, 1),
            Edge::new(1, 2, 1),
            Edge::new(2, 0, -3), // total cost = -1 (negative cycle)
        ];
        let res = bellman_ford(&edges, 3, 0);
        assert_eq!(
            res,
            vec![
                Solution::Unbounded,
                Solution::Unbounded,
                Solution::Unbounded
            ]
        );
    }

    #[test]
    fn test_negative_cycle_affects_subgraph() {
        let edges = vec![
            Edge::new(0, 1, 1),
            Edge::new(1, 2, -1),
            Edge::new(2, 1, -1), // negative cycle between 1 and 2
            Edge::new(2, 3, 5),
        ];
        let res = bellman_ford(&edges, 4, 0);
        assert_eq!(
            res,
            vec![
                Solution::Bounded(0),
                Solution::Unbounded,
                Solution::Unbounded,
                Solution::Unbounded
            ]
        );
    }

    #[test]
    fn test_negative_cycle_affects_subgraph2() {
        //     3 ⇆ 4 → 5
        //     ↑
        // 0 → 1 → 2
        //
        // 6 ⇆ 7
        //
        // 8 → 9
        let edges = vec![
            Edge::new(0, 1, 1),
            Edge::new(1, 2, 1),
            Edge::new(1, 3, 1),
            Edge::new(3, 4, 1),
            Edge::new(4, 3, -10),
            Edge::new(4, 5, 1),
            Edge::new(6, 7, -1),
            Edge::new(7, 6, -1),
            Edge::new(8, 9, 1),
        ];
        let res = bellman_ford(&edges, 10, 0);
        assert_eq!(
            res,
            vec![
                Solution::Bounded(0), // 0
                Solution::Bounded(1), // 1
                Solution::Bounded(2), // 2
                Solution::Unbounded,  // 3
                Solution::Unbounded,  // 4
                Solution::Unbounded,  // 5
                Solution::Infeasible, // 6
                Solution::Infeasible, // 7
                Solution::Infeasible, // 8
                Solution::Infeasible, // 9
            ]
        );
    }

    #[test]
    fn test_dual_mapping() {
        assert_eq!(Solution::Bounded(42).dual(), Solution::Bounded(42));
        assert_eq!(Solution::Unbounded.dual(), Solution::Infeasible);
        assert_eq!(Solution::Infeasible.dual(), Solution::Unbounded);
    }
}
