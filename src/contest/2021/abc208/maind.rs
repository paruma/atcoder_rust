#![allow(clippy::let_unit_value)]
use ndarray::{s, Array, Array3};
use proconio::input;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    src: usize,
    dst: usize,
    cost: i64,
}

fn read() -> (usize, usize, Vec<Edge>) {
    input! {n_vertex: usize, n_edge: usize,
        edge_info: [(usize, usize, i64); n_edge],
    }
    let edges = edge_info
        .iter()
        .map(|(src, dst, cost)| Edge {
            src: *src - 1,
            dst: *dst - 1,
            cost: *cost,
        })
        .collect::<Vec<_>>();
    (n_vertex, n_edge, edges)
}
// min_opt, add_optはトロピカル半環の演算:
// 動的計画法を実現する代数〜トロピカル演算でグラフの最短経路を計算する〜 - Qiita https://qiita.com/lotz/items/094bffd77b24e37bf20e

// flatten→minのイメージ
// T:ordまで一般化可能
fn min_opt(a: Option<i64>, b: Option<i64>) -> Option<i64> {
    match (a, b) {
        (None, None) => None,
        (None, Some(_)) => b,
        (Some(_), None) => a,
        (Some(ae), Some(be)) => Some(std::cmp::min(ae, be)),
    }
}
fn add_opt(a: Option<i64>, b: Option<i64>) -> Option<i64> {
    match (a, b) {
        (None, None) => None,
        (None, Some(_)) => None,
        (Some(_), None) => None,
        (Some(ae), Some(be)) => Some(ae + be),
    }
}

#[allow(dead_code)]
fn output_dp(n_vertex: usize, dp: &Array3<Option<i64>>) {
    for k in 0..=n_vertex {
        for src in 0..n_vertex {
            for dst in 0..n_vertex {
                print!("{:?}", dp[[k, src, dst]]);
            }
            println!();
        }
        println!();
    }
}
fn solve(n_vertex: usize, _n_edge: usize, edges: &[Edge]) -> i64 {
    let mut dp: Array3<Option<i64>> =
        Array::from_shape_fn((n_vertex + 1, n_vertex, n_vertex), |_| None);

    for i in 0..n_vertex {
        dp[[0, i, i]] = Some(0);
    }

    for &edge in edges {
        dp[[0, edge.src, edge.dst]] = Some(edge.cost);
    }

    // forの順番間違えてた src/dst/kってしてた
    for k in 1..=n_vertex {
        for src in 0..n_vertex {
            for dst in 0..n_vertex {
                dp[[k, src, dst]] = min_opt(
                    add_opt(dp[[k - 1, src, k - 1]], dp[[k - 1, k - 1, dst]]),
                    dp[[k - 1, src, dst]],
                )
            }
        }
    }

    //output_dp(n_vertex, &dp);

    dp.slice(s![1.., .., ..])
        .iter()
        .map(|&opt| opt.unwrap_or(0))
        .sum::<i64>()
}

//fn output() {}

fn main() {
    let (n_vertex, n_edge, edges) = read();
    let ans = solve(n_vertex, n_edge, &edges);
    //output();
    println!("{}", ans);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_name() {
        assert_eq!(std::cmp::min(Some(3), Some(2)), Some(2));
        assert_eq!(std::cmp::min(None, Some(2)), None);
    }
}
