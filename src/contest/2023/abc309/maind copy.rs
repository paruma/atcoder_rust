use std::collections::VecDeque;

use itertools::Itertools;
#[allow(unused_imports)]
use myio::*;
pub mod myio {
    use std::io;

    pub fn read_line() -> String {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        buffer.trim().to_string()
    }

    pub fn read_vec_i64() -> Vec<i64> {
        let buf = read_line();
        buf.trim()
            .split(' ')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    }

    pub fn read_vec_str() -> Vec<String> {
        let buf = read_line();
        buf.trim()
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    pub fn read_i64_1() -> i64 {
        let buf = read_line();
        buf.parse::<i64>().unwrap()
    }

    pub fn read_i64_2() -> (i64, i64) {
        let ns = read_vec_i64();
        (ns[0], ns[1])
    }

    pub fn read_i64_3() -> (i64, i64, i64) {
        let ns = read_vec_i64();
        (ns[0], ns[1], ns[2])
    }

    pub fn read_i64_4() -> (i64, i64, i64, i64) {
        let ns = read_vec_i64();
        (ns[0], ns[1], ns[2], ns[3])
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Edge {
    a: usize,
    b: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Problem {
    nv1: usize,
    nv2: usize,
    ne: usize,
    edges: Vec<Edge>,
}

struct Answer {
    ans: i64,
}

impl Problem {
    fn read() -> Problem {
        let (nv1, nv2, ne) = read_i64_3();
        let nv1 = nv1 as usize;
        let nv2 = nv2 as usize;
        let ne = ne as usize;
        let edges = (0..ne)
            .map(|_| {
                let (a, b) = read_i64_2();
                let a = a as usize - 1;
                let b = b as usize - 1;
                Edge { a, b }
            })
            .collect_vec();
        Problem {
            nv1,
            nv2,
            ne,
            edges,
        }
    }

    fn solve(self) -> Answer {
        // 隣接リスト
        let mut adj1: Vec<Vec<usize>> = vec![vec![]; self.nv1 + self.nv1];
        let mut adj2: Vec<Vec<usize>> = vec![vec![]; self.nv1 + self.nv2];
        let is_group1 = |i: usize| i < self.nv1;
        let is_group2 = |i: usize| self.nv1 <= i && i < self.nv1 + self.nv2;
        for edge in &self.edges {
            if is_group1(edge.a) {
                assert!(is_group1(edge.b));
                adj1[edge.a].push(edge.b);
                adj1[edge.b].push(edge.a);
            } else if is_group2(edge.a) {
                assert!(is_group2(edge.b));
                adj2[edge.a].push(edge.b);
                adj2[edge.b].push(edge.a);
            } else {
                panic!();
            }
        }
        let calc_dist = |from: usize, adj: &Vec<Vec<usize>>| {
            let mut visited = vec![false; adj.len()];
            let mut open: VecDeque<usize> = VecDeque::new();
            let mut dist = vec![-1_i64; adj.len()];

            open.push_front(from);
            visited[from] = true;
            dist[from] = 0;
            while !open.is_empty() {
                let current_idx = open.pop_back().unwrap();
                for &next_idx in &adj[current_idx] {
                    if !visited[next_idx] {
                        visited[next_idx] = true;
                        dist[next_idx] = dist[current_idx] + 1;
                        open.push_front(next_idx);
                    }
                }
            }
            dist
        };

        let dist1 = calc_dist(0, &adj1);
        let dist2 = calc_dist(self.nv1 + self.nv2 - 1, &adj2);

        let ans = *dist1.iter().max().unwrap() + *dist2.iter().max().unwrap() + 1;

        Answer { ans }
    }
}

impl Answer {
    fn print(self) {
        println!("{}", self.ans)
    }
}

fn main() {
    Problem::read().solve().print();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
