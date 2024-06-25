use crate::mylib::{bitset::bitset::BitSet, ext_int::mod_ext_int::ExtInt};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    from: usize,
    to: usize,
    cost: i64,
}

#[allow(dead_code)]
fn solve(n_vertex: usize, edges: &[Edge]) -> ExtInt {
    // 巡回セールスマン問題
    // https://judge.u-aizu.ac.jp/onlinejudge/description.jsp
    // ハミルトン閉路が存在するなら、どの頂点を始点にしても良い→ここでは頂点0を始点にする。
    // dp[available_set][v]: (始点を除く)訪問する頂点の集合が available_set、最後に訪問する点をvとしたときの最短路

    // 計算量は愚直に行うとO(n!) になるが、
    // bit DP をすることで O(n^2 2^n) 程度になる
    struct Rec {
        n_vertex: usize,
        adj_matrix: Vec<Vec<ExtInt>>,
    }

    impl Rec {
        fn new(n_vertex: usize, adj_matrix: &[Vec<ExtInt>]) -> Self {
            Rec {
                n_vertex,
                adj_matrix: adj_matrix.to_vec(),
            }
        }

        // 0からスタートして available_set を訪問して to までたどり着く最短経路の手数を求める
        fn rec(&self, available_set: BitSet, to: usize, dp: &mut [Vec<Option<ExtInt>>]) -> ExtInt {
            if let Some(ans) = dp[available_set.to_bit()][to] {
                return ans;
            }

            let ans = if available_set.is_empty() && to == 0 {
                ExtInt::Fin(0)
            } else if !available_set.contains(to) {
                // to に訪問できないので to にたどり着くことはできない。
                ExtInt::Inf
            } else {
                // to の直前の頂点 from で場合分け。
                // 0 → from → to という経路を考える
                (0..self.n_vertex)
                    .map(|from| {
                        // 第1項が 0 → from
                        // 第2項が from → to
                        self.rec(available_set.remove(to), from, dp) + self.adj_matrix[from][to]
                    })
                    .min()
                    .unwrap()
            };
            dp[available_set.to_bit()][to] = Some(ans);
            ans
        }
    }
    let adj_matrix = {
        let mut adj_matrix = vec![vec![ExtInt::Inf; n_vertex]; n_vertex];
        for e in edges {
            adj_matrix[e.from][e.to] = ExtInt::Fin(e.cost);
        }
        adj_matrix
    };
    let mut dp: Vec<Vec<Option<ExtInt>>> = vec![vec![None; n_vertex]; 1 << n_vertex];

    // 頂点0からスタートして、すべての頂点を辿って頂点0までたどり着く経路の最短手数を考える。
    // (どこからスタートしても最短ハミルトン閉路は存在する)
    Rec::new(n_vertex, &adj_matrix).rec(BitSet::universal_set(n_vertex), 0, &mut dp)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {
        let n_vertex = 4;
        let edges = [
            (0, 1, 2),
            (1, 2, 3),
            (1, 3, 9),
            (2, 0, 1),
            (2, 3, 6),
            (3, 2, 4),
        ]
        .map(|(from, to, cost)| Edge { from, to, cost });

        assert_eq!(solve(n_vertex, &edges), ExtInt::Fin(16));
    }

    #[test]
    fn test2() {
        /*
        |     1
        | [0] → [1]
        | 1↓  ↙1
        | [2]
        |
         */
        let n_vertex = 4;
        let edges =
            [(0, 1, 1), (1, 2, 1), (0, 2, 1)].map(|(from, to, cost)| Edge { from, to, cost });

        assert_eq!(solve(n_vertex, &edges), ExtInt::Inf);
    }
}

//---------snippet---------
