use cargo_snippet::snippet;

use crate::mylib::data_structure::queue0::mod_queue::Queue;

#[snippet(include = "mod_queue")]
pub fn topo_sort(adj: &[Vec<usize>]) -> Vec<usize> {
    let n_vertex = adj.len();
    let mut in_deg = vec![0; n_vertex];
    for neighbors in adj {
        for &next in neighbors {
            in_deg[next] += 1;
        }
    }

    let mut open: Queue<usize> = Queue::new();
    for (v, &deg) in in_deg.iter().enumerate() {
        if deg == 0 {
            open.push(v);
        }
    }

    let mut ans = vec![];

    while let Some(current) = open.pop() {
        ans.push(current);
        for &next in &adj[current] {
            in_deg[next] -= 1;
            if in_deg[next] == 0 {
                open.push(next);
            }
        }
    }
    ans
}

#[snippet(include = "topo_sort")]
pub fn has_cycle_directed_by_topo_sort(adj: &[Vec<usize>]) -> bool {
    let topo_sorted = topo_sort(adj); // 戻り値にループの部分は入ってこない。
    topo_sorted.len() != adj.len()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::mylib::graph::{
        graph::make_adj_from_directed, topo_sort::has_cycle_directed_by_topo_sort,
    };

    use super::topo_sort;

    #[test]
    fn test_topo_sort() {
        //
        // 0 → 1
        // ↓ ↗
        // 2
        // 3 → 4
        let n_vertex = 5;
        let edges = [(0, 1), (0, 2), (2, 1), (3, 4)]
            .into_iter()
            .map(|(from, to)| (from, to))
            .collect_vec();
        let adj = make_adj_from_directed(n_vertex, &edges);

        let sorted = topo_sort(&adj);

        // ソートされているか確認
        for &(from, to) in &edges {
            let from_pos = sorted.iter().position(|&x| x == from).unwrap();
            let to_pos = sorted.iter().position(|&x| x == to).unwrap();
            assert!(from_pos <= to_pos);
        }
    }

    #[test]
    fn test_has_cycle_directed() {
        {
            // 0 → 1 → 2
            //     ↑   ↓
            //     4 → 3
            let n_vertex = 5;
            let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 1)];
            let adj = make_adj_from_directed(n_vertex, &edges);
            assert!(has_cycle_directed_by_topo_sort(&adj));
        }
        {
            // 0 ← 1 → 2
            //     ↑   ↓
            //     4 → 3
            let n_vertex = 5;
            let edges = vec![(1, 0), (1, 2), (2, 3), (3, 4), (4, 1)];
            let adj = make_adj_from_directed(n_vertex, &edges);
            assert!(has_cycle_directed_by_topo_sort(&adj));
        }
        {
            // 0 → 1 → 2 → 5
            //     ↓   ↓   ↑
            //     4 → 3 → 6
            let n_vertex = 7;
            let edges = vec![
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 4),
                (1, 4),
                (2, 5),
                (3, 6),
                (6, 5),
            ];
            let adj = make_adj_from_directed(n_vertex, &edges);
            assert!(!has_cycle_directed_by_topo_sort(&adj));
        }
    }
}
