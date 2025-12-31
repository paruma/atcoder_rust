use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use euler_tour::*;")]
pub mod euler_tour {

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum InOut {
        In(usize),
        Out(usize),
    }

    // 辺ベースのオイラーツアー
    pub struct EulerTour {
        pub tour: Vec<InOut>,
        pub in_time: Vec<usize>,  // in_time[v]: 頂点 v に初めて入る時刻
        pub out_time: Vec<usize>, // out_time[v]: 頂点 v から最後に出る時刻
    }

    impl EulerTour {
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

            let tour = {
                fn dfs(tour: &mut Vec<InOut>, current: usize, tree_children: &[Vec<usize>]) {
                    tour.push(InOut::In(current));

                    for &child in &tree_children[current] {
                        dfs(tour, child, tree_children);
                    }

                    tour.push(InOut::Out(current));
                }
                let mut tour = Vec::with_capacity(2 * nv);
                dfs(&mut tour, root, &tree_children);
                tour
            };

            let (in_time, out_time) = {
                let mut in_time = vec![0; nv];
                let mut out_time = vec![0; nv];
                for (time, edge) in tour.iter().copied().enumerate() {
                    match edge {
                        InOut::In(v) => {
                            in_time[v] = time;
                        }
                        InOut::Out(v) => {
                            out_time[v] = time;
                        }
                    }
                }

                (in_time, out_time)
            };

            EulerTour {
                tour,
                in_time,
                out_time,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use ac_library::Additive;
    use ac_library::Segtree;
    use itertools::Itertools;

    use crate::graph::euler_tour::euler_tour::EulerTour;
    use crate::graph::euler_tour::euler_tour::InOut::*;

    #[test]
    fn test_euler_tour() {
        // 0
        // ├ 1
        // │ ├ 3
        // │ │ └ 6
        // │ └ 4
        // │   ├ 7
        // │   ├ 8
        // │   └ 9
        // └ 2
        //   └ 5
        //     ├ 10
        //     └ 11

        let tree_parent = vec![0, 0, 0, 1, 1, 2, 3, 4, 4, 4, 5, 5];
        let euler_tour = EulerTour::new(&tree_parent);

        let tour_expected = vec![
            In(0),
            In(1),
            In(3),
            In(6),
            Out(6),
            Out(3),
            In(4),
            In(7),
            Out(7),
            In(8),
            Out(8),
            In(9),
            Out(9),
            Out(4),
            Out(1),
            In(2),
            In(5),
            In(10),
            Out(10),
            In(11),
            Out(11),
            Out(5),
            Out(2),
            Out(0),
        ];

        let in_time_expected = vec![0, 1, 15, 2, 6, 16, 3, 7, 9, 11, 17, 19];
        let out_time_expected = vec![23, 14, 22, 5, 13, 21, 4, 8, 10, 12, 18, 20];

        assert_eq!(euler_tour.tour, tour_expected);
        assert_eq!(euler_tour.in_time, in_time_expected);
        assert_eq!(euler_tour.out_time, out_time_expected);
    }

    #[test]
    fn test_euler_tour_example() {
        // 0
        // ├ 1
        // │ ├ 3
        // │ │ └ 6
        // │ └ 4
        // │   ├ 7
        // │   ├ 8
        // │   └ 9
        // └ 2
        //   └ 5
        //     ├ 10
        //     └ 11

        let tree_parent = vec![0, 0, 0, 1, 1, 2, 3, 4, 4, 4, 5, 5];
        let euler_tour = EulerTour::new(&tree_parent);
        // 頂点の番号 + 10 をスコアとする
        let score = [10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21];

        // 部分木のスコアを求める
        let subtree_segtree = Segtree::<Additive<i64>>::from(
            euler_tour
                .tour
                .iter()
                .map(|e| match e {
                    In(v) => score[*v],
                    Out(_) => 0,
                })
                .collect_vec(),
        );

        // 1の部分木
        assert_eq!(
            subtree_segtree.prod(euler_tour.in_time[1]..euler_tour.out_time[1]),
            11 + 13 + 14 + 16 + 17 + 18 + 19
        );
        // 3の部分木
        assert_eq!(
            subtree_segtree.prod(euler_tour.in_time[3]..euler_tour.out_time[3]),
            13 + 16
        );
        // 6の部分木
        assert_eq!(
            subtree_segtree.prod(euler_tour.in_time[6]..euler_tour.out_time[6]),
            16
        );
    }
}
