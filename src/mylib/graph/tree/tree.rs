use cargo_snippet::snippet;

#[snippet]
/// 根付き木の隣接リスト `adj` と根 `root` から、各頂点の親頂点を求めます。
///
/// # 計算量
/// O(V + E)
pub fn make_tree_parent(adj: &[Vec<usize>], root: usize) -> Vec<Option<usize>> {
    let n = adj.len();
    let mut parent = vec![None; n];
    let mut visited = vec![false; n];
    let mut queue = std::collections::VecDeque::new();

    visited[root] = true;
    queue.push_back(root);

    while let Some(v) = queue.pop_front() {
        for &u in &adj[v] {
            if !visited[u] {
                visited[u] = true;
                parent[u] = Some(v);
                queue.push_back(u);
            }
        }
    }

    parent
}

#[snippet]
/// 根付き木の隣接リスト `adj` と根 `root` から、各頂点の子頂点リストを求めます。
///
/// # 計算量
/// O(V + E)
pub fn make_tree_children(adj: &[Vec<usize>], root: usize) -> Vec<Vec<usize>> {
    let n = adj.len();
    let mut children = vec![vec![]; n];
    let mut visited = vec![false; n];
    let mut queue = std::collections::VecDeque::new();

    visited[root] = true;
    queue.push_back(root);

    while let Some(v) = queue.pop_front() {
        for &u in &adj[v] {
            if !visited[u] {
                visited[u] = true;
                children[v].push(u);
                queue.push_back(u);
            }
        }
    }

    children
}

#[snippet]
/// コスト付き根付き木の隣接リスト `adj` と根 `root` から、各頂点の親頂点とそこへのコストを求めます。
///
/// # Returns
/// `parent[v]` は根なら `None`、それ以外は `Some((親頂点, 親辺のコスト))`。
///
/// # 計算量
/// $O(V + E)$
pub fn make_tree_parent_weighted<T: Clone>(
    adj: &[Vec<(usize, T)>],
    root: usize,
) -> Vec<Option<(usize, T)>> {
    let n = adj.len();
    let mut parent = vec![None; n];
    let mut visited = vec![false; n];
    let mut queue = std::collections::VecDeque::new();

    visited[root] = true;
    queue.push_back(root);

    while let Some(v) = queue.pop_front() {
        for (u, cost) in &adj[v] {
            let u = *u;
            if !visited[u] {
                visited[u] = true;
                parent[u] = Some((v, cost.clone()));
                queue.push_back(u);
            }
        }
    }

    parent
}

#[snippet]
/// コスト付き根付き木の隣接リスト `adj` と根 `root` から、各頂点の子頂点リストを求めます。
///
/// # Returns
/// `children[v]` は `v` の子頂点とそこへのコストのリスト `(子頂点, コスト)`。
///
/// # 計算量
/// $O(V + E)$
pub fn make_tree_children_weighted<T: Clone>(
    adj: &[Vec<(usize, T)>],
    root: usize,
) -> Vec<Vec<(usize, T)>> {
    let n = adj.len();
    let mut children = vec![vec![]; n];
    let mut visited = vec![false; n];
    let mut queue = std::collections::VecDeque::new();

    visited[root] = true;
    queue.push_back(root);

    while let Some(v) = queue.pop_front() {
        for e in &adj[v] {
            let u = e.0;
            if !visited[u] {
                visited[u] = true;
                children[v].push(e.clone());
                queue.push_back(u);
            }
        }
    }

    children
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_make_tree_parent() {
        // 木構造:
        //     0
        //    / \
        //   1   2
        //  / \
        // 3   4
        let adj = vec![
            vec![1, 2],    // 0
            vec![0, 3, 4], // 1
            vec![0],       // 2
            vec![1],       // 3
            vec![1],       // 4
        ];
        let root = 0;
        let expected = vec![
            None,    // 0
            Some(0), // 1
            Some(0), // 2
            Some(1), // 3
            Some(1), // 4
        ];
        let result = make_tree_parent(&adj, root);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_make_tree_children() {
        // 木構造:
        //     0
        //    / \
        //   1   2
        //  / \
        // 3   4
        let adj = vec![
            vec![1, 2],    // 0
            vec![0, 3, 4], // 1
            vec![0],       // 2
            vec![1],       // 3
            vec![1],       // 4
        ];
        let root = 0;
        let expected = vec![
            vec![1, 2], // 0
            vec![3, 4], // 1
            vec![],     // 2
            vec![],     // 3
            vec![],     // 4
        ];
        let result = make_tree_children(&adj, root);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_make_tree_parent_weighted() {
        // 木構造（コスト付き）:
        //       0
        //     10/ \20
        //      1   2
        //    5/ \8
        //    3   4
        let adj = vec![
            vec![(1, 10i64), (2, 20i64)],           // 0
            vec![(0, 10i64), (3, 5i64), (4, 8i64)], // 1
            vec![(0, 20i64)],                       // 2
            vec![(1, 5i64)],                        // 3
            vec![(1, 8i64)],                        // 4
        ];
        let root = 0;
        let expected = vec![
            None,             // 0
            Some((0, 10i64)), // 1
            Some((0, 20i64)), // 2
            Some((1, 5i64)),  // 3
            Some((1, 8i64)),  // 4
        ];
        let result = make_tree_parent_weighted(&adj, root);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_make_tree_children_weighted() {
        // 木構造（コスト付き）:
        //       0
        //     10/ \20
        //      1   2
        //    5/ \8
        //    3   4
        let adj = vec![
            vec![(1, 10i64), (2, 20i64)],           // 0
            vec![(0, 10i64), (3, 5i64), (4, 8i64)], // 1
            vec![(0, 20i64)],                       // 2
            vec![(1, 5i64)],                        // 3
            vec![(1, 8i64)],                        // 4
        ];
        let root = 0;
        let expected = vec![
            vec![(1, 10i64), (2, 20i64)], // 0
            vec![(3, 5i64), (4, 8i64)],   // 1
            vec![],                       // 2
            vec![],                       // 3
            vec![],                       // 4
        ];
        let result = make_tree_children_weighted(&adj, root);
        assert_eq!(result, expected);
    }
}
