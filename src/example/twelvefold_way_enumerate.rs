#![allow(dead_code)]

/// [n_balls] → [n_boxes] の全射全列挙
// 残りのボールの数と残りの箱の数が同じになったら、ボールが入ってない箱にボールを入れるようにすれば良い？
// fn surj(n_balls: usize, n_boxes: usize) -> Vec<Vec<usize>> {
//     todo!()
// }

/// n_balls 個の区別するボールを、n_boxes 個のグループ（区別しない）に分割する分割の仕方を全列挙する。
/// 各グループは1個以上のボールが含まれるようにする。
/// Vec<Vec<usize>> はグループの分割を表す
fn stirling_s2(n_balls: usize, n_boxes: usize) -> Vec<Vec<Vec<usize>>> {
    if n_boxes > n_balls {
        // 各グループに1つ以上のボールを含めるようにできない
        return vec![];
    }
    struct Rec {
        n_balls: usize,
        n_boxes: usize,
    }

    impl Rec {
        fn new(n_balls: usize, n_boxes: usize) -> Self {
            Rec { n_balls, n_boxes }
        }

        fn exec(&self) -> Vec<Vec<Vec<usize>>> {
            let mut groups_list = vec![];
            self.exec_rec(0, &mut vec![], &mut groups_list);
            groups_list
        }

        fn exec_rec(
            &self,
            cnt: usize,
            groups: &mut Vec<Vec<usize>>,
            groups_list: &mut Vec<Vec<Vec<usize>>>,
        ) {
            // スターリング数の漸化式
            // S(n,k) = k S(n-1, k) + S(n-1, k-1)
            // と同じ考え方をする

            let n_remain_balls = self.n_balls - cnt;
            let n_remain_boxes = self.n_boxes - groups.len();

            // 残りのボールの数 = 残りの箱の数の場合は、残りの箱に1つずつボールを入れる。
            if n_remain_balls == n_remain_boxes {
                for i in cnt..self.n_balls {
                    groups.push(vec![i]);
                }
                groups_list.push(groups.clone());
                for _ in (cnt..self.n_balls).rev() {
                    groups.pop();
                }
                return;
            }

            // すでにあるグループに追加
            for group_i in 0..groups.len() {
                groups[group_i].push(cnt);
                self.exec_rec(cnt + 1, groups, groups_list);
                groups[group_i].pop();
            }

            // 新しくグループを作成する
            if groups.len() < self.n_boxes {
                groups.push(vec![cnt]);
                self.exec_rec(cnt + 1, groups, groups_list);
                groups.pop();
            }
        }
    }

    Rec::new(n_balls, n_boxes).exec()
}

/// n_balls 個の区別するボールを、n_boxes 個のグループ（区別しない）に分割する分割の仕方を全列挙する。
/// ボールが0個のグループがあっても良い。
/// Vec<Vec<usize>> はグループの分割を表す
fn bell(n_balls: usize, n_boxes: usize) -> Vec<Vec<Vec<usize>>> {
    struct Rec {
        n_balls: usize,
        n_boxes: usize,
    }

    impl Rec {
        fn new(n_balls: usize, n_boxes: usize) -> Self {
            Rec { n_balls, n_boxes }
        }

        fn exec(&self) -> Vec<Vec<Vec<usize>>> {
            let mut groups_list = vec![];
            self.exec_rec(0, &mut vec![], &mut groups_list);
            groups_list
        }

        fn exec_rec(
            &self,
            cnt: usize,
            groups: &mut Vec<Vec<usize>>,
            groups_list: &mut Vec<Vec<Vec<usize>>>,
        ) {
            // スターリング数の漸化式
            // S(n,k) = k S(n-1, k) + S(n-1, k-1)
            // と同じ考え方をする

            if self.n_balls == cnt {
                groups_list.push(groups.clone());
                return;
            }

            // すでにあるグループに追加
            for group_i in 0..groups.len() {
                groups[group_i].push(cnt);
                self.exec_rec(cnt + 1, groups, groups_list);
                groups[group_i].pop();
            }

            // 新しくグループを作成する
            if groups.len() < self.n_boxes {
                groups.push(vec![cnt]);
                self.exec_rec(cnt + 1, groups, groups_list);
                groups.pop();
            }
        }
    }

    Rec::new(n_balls, n_boxes).exec()
}

///  n_balls 個の区別しないボールを、n_boxes 個のグループ（区別しない）に分割する。
/// ボールが含まれないグループがあってもよい。
/// 自然数 n_balls を n_boxes 個の 0 以上の整数の和で表す方法と考えても良い。
// fn partition(n_balls: usize, n_boxes: usize) -> Vec<Vec<usize>> {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn sort_groups_list(groups_list: &[Vec<Vec<usize>>]) -> Vec<Vec<Vec<usize>>> {
        let mut groups_list = groups_list.to_vec();
        for groups in groups_list.iter_mut() {
            for group in groups.iter_mut() {
                group.sort();
            }
            groups.sort();
        }
        groups_list
    }

    #[test]
    fn test_stirling_s2() {
        {
            let actual = stirling_s2(4, 2);
            let expected = vec![
                vec![vec![0, 1, 2], vec![3]],
                vec![vec![0, 1, 3], vec![2]],
                vec![vec![0, 1], vec![2, 3]],
                vec![vec![0, 2, 3], vec![1]],
                vec![vec![0, 2], vec![1, 3]],
                vec![vec![0, 3], vec![1, 2]],
                vec![vec![0], vec![1, 2, 3]],
            ];
            assert_eq!(sort_groups_list(&expected), sort_groups_list(&actual));
        }
        {
            let actual = stirling_s2(3, 3);
            let expected = vec![vec![vec![0], vec![1], vec![2]]];
            assert_eq!(sort_groups_list(&expected), sort_groups_list(&actual));
        }
        {
            let actual = stirling_s2(4, 0);
            let expected = vec![];
            assert_eq!(sort_groups_list(&expected), sort_groups_list(&actual));
        }
        {
            let actual = stirling_s2(2, 3);
            let expected = vec![];
            assert_eq!(sort_groups_list(&expected), sort_groups_list(&actual));
        }
        {
            let actual = stirling_s2(0, 0);
            let expected = vec![vec![]];
            assert_eq!(sort_groups_list(&expected), sort_groups_list(&actual));
        }
    }
    #[test]
    fn test_bell() {
        {
            let actual = bell(4, 2);
            let expected = vec![
                vec![vec![0, 1, 2, 3]],
                vec![vec![0, 1, 2], vec![3]],
                vec![vec![0, 1, 3], vec![2]],
                vec![vec![0, 1], vec![2, 3]],
                vec![vec![0, 2, 3], vec![1]],
                vec![vec![0, 2], vec![1, 3]],
                vec![vec![0, 3], vec![1, 2]],
                vec![vec![0], vec![1, 2, 3]],
            ];
            assert_eq!(sort_groups_list(&expected), sort_groups_list(&actual));
        }
        {
            let actual = bell(3, 4);
            let expected = vec![
                vec![vec![0, 1, 2]],
                vec![vec![0, 1], vec![2]],
                vec![vec![0, 2], vec![1]],
                vec![vec![0], vec![1, 2]],
                vec![vec![0], vec![1], vec![2]],
            ];
            assert_eq!(sort_groups_list(&expected), sort_groups_list(&actual));
        }
        {
            let actual = bell(4, 0);
            let expected = vec![];
            assert_eq!(sort_groups_list(&expected), sort_groups_list(&actual));
        }
        {
            let actual = bell(0, 0);
            let expected = vec![vec![]];
            assert_eq!(sort_groups_list(&expected), sort_groups_list(&actual));
        }
        {
            let actual = bell(0, 2);
            let expected = vec![vec![]];
            assert_eq!(sort_groups_list(&expected), sort_groups_list(&actual));
        }
    }
}
