#![allow(dead_code)]
use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;

/*
単一始点最短路問題

- 負の重みがあるグラフ: ベルマン・フォード法
- 負の重みがない: ダイクストラ法
- DAG: トポロジカルソートをしてDP
- 重みが1: BFS

全点対間最短路問題

- ワーシャルフロイド法
*/
use crate::mylib::ext_int::mod_ext_int::{ExtInt, INF, fin};

macro_rules! chmin {
    ($a: expr_2021, $b: expr_2021) => {
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

fn dijkstra(adj: &[Vec<Edge>], start: usize) -> Vec<ExtInt> {
    let n_vertex = adj.len();
    let mut pq: BinaryHeap<(Reverse<ExtInt>, usize)> = BinaryHeap::new();
    let mut dist = vec![INF; n_vertex];
    dist[start] = fin(0);
    pq.push((Reverse(fin(0)), start));

    while let Some((Reverse(d), current)) = pq.pop() {
        if dist[current] < d {
            continue;
        }
        for e in &adj[current] {
            if chmin!(dist[e.to], dist[e.from] + fin(e.cost)) {
                pq.push((Reverse(dist[e.to]), e.to));
            }
        }
    }
    dist
}

fn warshall_floyd(edges: &[Edge], n_vertex: usize) -> Vec<Vec<ExtInt>> {
    let mut dist = vec![vec![INF; n_vertex]; n_vertex];

    for e in edges {
        chmin!(dist[e.from][e.to], fin(e.cost));
    }
    for v in 0..n_vertex {
        dist[v][v] = fin(0);
    }

    for k in 0..n_vertex {
        for from in 0..n_vertex {
            for to in 0..n_vertex {
                // from → (0..=k の頂点を0回以上通る) → to というパスでの最短路を計算
                // k を経由するかどうかで場合分けして計算
                chmin!(dist[from][to], dist[from][k] + dist[k][to]);
            }
        }
    }

    dist
}

fn prev_to_path(prev_list: &[Option<usize>], start: usize, goal: usize) -> Option<Vec<usize>> {
    let mut current = goal;
    let mut path_rev = vec![goal];
    while let Some(prev) = prev_list[current] {
        current = prev;
        path_rev.push(current);
    }
    if current == start {
        let path = path_rev.iter().copied().rev().collect_vec();
        Some(path)
    } else {
        None
    }
}

/// 始点 start から各頂点までの最小コストと、最小コストを実現する前者配列を返す
fn dijkstra_with_restore(adj: &[Vec<Edge>], start: usize) -> (Vec<ExtInt>, Vec<Option<usize>>) {
    let n_vertex = adj.len();
    let mut pq: BinaryHeap<(Reverse<ExtInt>, usize)> = BinaryHeap::new();
    let mut dist = vec![INF; n_vertex];
    let mut prev: Vec<Option<usize>> = vec![None; n_vertex];
    dist[start] = fin(0);
    pq.push((Reverse(fin(0)), start));

    while let Some((Reverse(d), current)) = pq.pop() {
        if dist[current] < d {
            continue;
        }
        for e in &adj[current] {
            if chmin!(dist[e.to], dist[e.from] + fin(e.cost)) {
                prev[e.to] = Some(e.from);
                pq.push((Reverse(dist[e.to]), e.to));
            }
        }
    }
    (dist, prev)
}

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

        assert_eq!(result, Some(vec![fin(0), fin(2), fin(0), fin(1)]));
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

        assert_eq!(result, Some(vec![fin(0), fin(1), fin(3), fin(6)]));
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

        assert_eq!(result, Some(vec![fin(0), fin(2), fin(-1), fin(0)]));
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

        assert_eq!(result, Some(vec![fin(0), fin(1), INF, INF]));
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

        let expected = vec![fin(0), fin(9), fin(6), fin(6)];

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

        let expected = vec![fin(0), fin(2), fin(5), INF];

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
            vec![fin(0), fin(9), fin(6), fin(6)],
            vec![INF, fin(0), fin(1), fin(1)],
            vec![INF, fin(3), fin(0), fin(0)],
            vec![INF, INF, INF, fin(0)],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_dijkstra_with_restore() {
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
        let (result_dist, result_prev) = dijkstra_with_restore(&adj, 0);

        let expected_dist = vec![fin(0), fin(9), fin(6), fin(6)];
        let expected_prev = vec![None, Some(2), Some(0), Some(2)];

        assert_eq!(result_dist, expected_dist);
        assert_eq!(result_prev, expected_prev);
    }

    #[test]
    fn test_dijkstra_with_restore2() {
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

        let (result_dist, result_prev) = dijkstra_with_restore(&adj, 0);

        let expected_dist = vec![fin(0), fin(2), fin(5), INF];
        let expected_prev = vec![None, Some(0), Some(1), None];

        assert_eq!(result_dist, expected_dist);
        assert_eq!(result_prev, expected_prev);
    }

    #[test]
    fn test_prev_to_path1() {
        let prev = vec![None, Some(2), Some(0), Some(2)];
        let result_path = prev_to_path(&prev, 0, 3);

        let expected_path = Some(vec![0, 2, 3]);

        assert_eq!(result_path, expected_path);
    }

    #[test]
    fn test_prev_to_path2() {
        let prev = vec![None, Some(2), Some(0), Some(2)];
        let result_path = prev_to_path(&prev, 0, 1);

        let expected_path = Some(vec![0, 2, 1]);

        assert_eq!(result_path, expected_path);
    }

    #[test]
    fn test_prev_to_path3() {
        let prev = vec![None, Some(2), Some(0), Some(2)];
        let result_path = prev_to_path(&prev, 0, 0);

        let expected_path = Some(vec![0]);

        assert_eq!(result_path, expected_path);
    }

    #[test]
    fn test_prev_to_path4() {
        let prev = vec![None, Some(0), Some(1), None];
        let result_path = prev_to_path(&prev, 0, 3);

        let expected_path = None;

        assert_eq!(result_path, expected_path);
    }
}
