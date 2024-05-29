#![allow(dead_code)]
use std::{cmp::Reverse, collections::BinaryHeap};

/*
単一始点最短路問題

- 負の重みがあるグラフ: ベルマン・フォード法
- 負の重みがない: ダイクストラ法
- DAG: トポロジカルソートをしてDP
- 重みが1: BFS

全点対間最短路問題

- ワーシャルフロイド法
*/
use crate::mylib::ext_int::mod_ext_int::{ExtInt, ExtInt::Fin, ExtInt::Inf};

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
struct Edge {
    from: usize,
    to: usize,
    cost: i64,
}
impl Edge {
    fn new(from: usize, to: usize, cost: i64) -> Self {
        Self { from, to, cost }
    }
}
fn bellman_ford(edges: &[Edge], n_vertex: usize, start: usize) -> Option<Vec<ExtInt>> {
    let mut dist = vec![Inf; n_vertex];
    dist[start] = Fin(0);

    for n_iter in 0..n_vertex {
        let mut updates = false;
        for edge in edges {
            if chmin!(dist[edge.to], dist[edge.from] + Fin(edge.cost)) {
                updates = true
            }
        }
        if !updates {
            break;
        }
        if n_iter == n_vertex - 1 {
            // 始点からたどり着ける負閉路が存在する
            return None;
        }
    }

    Some(dist)
}

fn dijkstra(adj: &[Vec<Edge>], start: usize) -> Vec<ExtInt> {
    let n_vertex = adj.len();
    let mut pq: BinaryHeap<(Reverse<ExtInt>, usize)> = BinaryHeap::new();
    let mut dist = vec![Inf; n_vertex];
    dist[start] = Fin(0);
    pq.push((Reverse(Fin(0)), start));

    while let Some((Reverse(d), current)) = pq.pop() {
        if dist[current] < d {
            continue;
        }
        for e in &adj[current] {
            if chmin!(dist[e.to], dist[e.from] + Fin(e.cost)) {
                pq.push((Reverse(dist[e.to]), e.to));
            }
        }
    }
    dist
}

fn warshall_floyd(edges: &[Edge], n_vertex: usize) -> Vec<Vec<ExtInt>> {
    let mut dist = vec![vec![Inf; n_vertex]; n_vertex];

    for e in edges {
        dist[e.from][e.to] = Fin(e.cost)
    }
    for v in 0..n_vertex {
        dist[v][v] = Fin(0);
    }

    for k in 0..n_vertex {
        for from in 0..n_vertex {
            for to in 0..n_vertex {
                // from → (0..k の頂点を0回以上通る) → to というパスでの最短路を計算
                chmin!(dist[from][to], dist[from][k] + dist[k][to]);
            }
        }
    }

    dist
}

// テストを書く

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_bellman_ford1() {
        // 負閉路なし
        //         [1]
        //     2 ↗  ↑|  ↘ 4
        //    ↗     ||    ↘
        //  [0]    3||-2    [3]
        //    ↘     ||    ↗
        //     6 ↘  |↓  ↗ 1
        //          [2]
        let edges = [
            (0, 1, 2),
            (0, 2, 6),
            (1, 2, -2),
            (2, 1, 3),
            (1, 3, 4),
            (2, 3, 1),
        ]
        .map(|(from, to, cost)| Edge::new(from, to, cost));

        let result = bellman_ford(&edges, 4, 0);

        assert_eq!(result, Some(vec![Fin(0), Fin(2), Fin(0), Fin(1)]));
    }

    #[test]
    fn test_bellman_ford2() {
        // 負閉路なし(パスの長さがN-1)
        //    100
        //  -------
        //  |  1  ↓  2     3
        // [0] → [1] → [2] → [3]
        let edges = [(0, 1, 100), (2, 3, 3), (1, 2, 2), (0, 1, 1)]
            .map(|(from, to, cost)| Edge::new(from, to, cost));

        let result = bellman_ford(&edges, 4, 0);

        assert_eq!(result, Some(vec![Fin(0), Fin(1), Fin(3), Fin(6)]));
    }

    #[test]
    fn test_bellman_ford3() {
        // 負閉路なし（長さ0の閉路あり）
        //         [1]
        //     2 ↗  ↑|  ↘ 4
        //    ↗     ||    ↘
        //  [0]    3||-3    [3]
        //    ↘     ||    ↗
        //     6 ↘  |↓  ↗ 1
        //          [2]
        let edges = [
            (0, 1, 2),
            (0, 2, 6),
            (1, 2, -3),
            (2, 1, 3),
            (1, 3, 4),
            (2, 3, 1),
        ]
        .map(|(from, to, cost)| Edge::new(from, to, cost));

        let result = bellman_ford(&edges, 4, 0);

        assert_eq!(result, Some(vec![Fin(0), Fin(2), Fin(-1), Fin(0)]));
    }

    #[test]
    fn test_bellman_ford4() {
        // 始点から到達可能な負閉路あり
        //         [1]
        //     2 ↗  ↑|  ↘ 4
        //    ↗     ||    ↘
        //  [0]    3||-4    [3]
        //    ↘     ||    ↗
        //     6 ↘  |↓  ↗ 1
        //          [2]
        let edges = [
            (0, 1, 2),
            (0, 2, 6),
            (1, 2, -4),
            (2, 1, 3),
            (1, 3, 4),
            (2, 3, 1),
        ]
        .map(|(from, to, cost)| Edge::new(from, to, cost));

        let result = bellman_ford(&edges, 4, 0);

        assert_eq!(result, None);
    }

    #[test]
    fn test_bellman_ford5() {
        // 始点から到達可能ではない負閉路あり
        //
        //     1    1      1
        // [0] → [1] ← [2] ⇆ [3]
        //                 -2
        let edges = [(0, 1, 1), (2, 1, 1), (2, 3, 1), (3, 2, 1)]
            .map(|(from, to, cost)| Edge::new(from, to, cost));

        let result = bellman_ford(&edges, 4, 0);

        assert_eq!(result, Some(vec![Fin(0), Fin(1), Inf, Inf]));
    }

    #[test]
    fn test_dijkstra() {
        // 頂点0 からすべての頂点に行ける
        //         [1]
        //    10 ↗  ↑|  ↘ 4
        //    ↗     ||    ↘
        //  [0]    3||1     [3]
        //    ↘     ||    ↗
        //     6 ↘  |↓  ↗ 0
        //          [2]
        let edges = [
            (0, 1, 10),
            (0, 2, 6),
            (1, 2, 1),
            (2, 1, 3),
            (1, 3, 4),
            (2, 3, 0),
        ]
        .map(|(from, to, cost)| Edge::new(from, to, cost));

        let n_vertex = 4;

        let adj: Vec<Vec<Edge>> = edges.iter().fold(vec![vec![]; n_vertex], |mut acc, e| {
            acc[e.from].push(*e);
            acc
        });
        let result_bellman_ford = bellman_ford(&edges, 4, 0);
        let result_dijkstra = dijkstra(&adj, 0);

        let expected = vec![Fin(0), Fin(9), Fin(6), Fin(6)];

        assert_eq!(result_bellman_ford, Some(expected.clone()));
        assert_eq!(result_dijkstra, expected);
    }

    #[test]
    fn test_dijkstra2() {
        // 頂点0から行けない頂点がある
        //     2     3     4
        // [0] → [1] → [2] ← [3]
        //
        let edges =
            [(0, 1, 2), (1, 2, 3), (3, 2, 4)].map(|(from, to, cost)| Edge::new(from, to, cost));

        let n_vertex = 4;

        let adj: Vec<Vec<Edge>> = edges.iter().fold(vec![vec![]; n_vertex], |mut acc, e| {
            acc[e.from].push(*e);
            acc
        });
        let result_bellman_ford = bellman_ford(&edges, 4, 0);
        let result_dijkstra = dijkstra(&adj, 0);

        let expected = vec![Fin(0), Fin(2), Fin(5), Inf];

        assert_eq!(result_bellman_ford, Some(expected.clone()));
        assert_eq!(result_dijkstra, expected);
    }

    #[test]
    fn test_warshall_floyd() {
        //         [1]
        //    10 ↗  ↑|  ↘ 4
        //    ↗     ||    ↘
        //  [0]    3||1     [3]
        //    ↘     ||    ↗
        //     6 ↘  |↓  ↗ 0
        //          [2]
        let edges = [
            (0, 1, 10),
            (0, 2, 6),
            (1, 2, 1),
            (2, 1, 3),
            (1, 3, 4),
            (2, 3, 0),
        ]
        .map(|(from, to, cost)| Edge::new(from, to, cost));
        let result = warshall_floyd(&edges, 4);
        let expected = vec![
            vec![Fin(0), Fin(9), Fin(6), Fin(6)],
            vec![Inf, Fin(0), Fin(1), Fin(1)],
            vec![Inf, Fin(3), Fin(0), Fin(0)],
            vec![Inf, Inf, Inf, Fin(0)],
        ];
        assert_eq!(result, expected);
    }
}
