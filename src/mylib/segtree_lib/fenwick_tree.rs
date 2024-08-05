use ac_library::FenwickTree;
use cargo_snippet::snippet;

#[snippet]
pub fn fenwick_tree_to_vec<T>(fenwick_tree: &ac_library::FenwickTree<T>, len: usize) -> Vec<T>
where
    T: Clone + std::ops::AddAssign<T> + std::ops::Sub<Output = T>,
{
    (0..len).map(|i| fenwick_tree.sum(i..=i)).collect()
}

#[snippet]
pub fn vec_to_fenwick_tree<T>(xs: &[T], e: T) -> FenwickTree<T>
where
    T: Clone + std::ops::AddAssign<T>,
{
    let mut fenwick_tree = FenwickTree::new(xs.len(), e);
    for (i, x) in xs.iter().enumerate() {
        fenwick_tree.add(i, x.clone());
    }
    fenwick_tree
}

#[cfg(test)]
mod test_fenwick_tree {

    use crate::mylib::segtree_lib::fenwick_tree::fenwick_tree_to_vec;

    use super::vec_to_fenwick_tree;

    #[test]
    fn test_segtree_to_vec() {
        let fenwick_tree = vec_to_fenwick_tree(&[1, 2, 3], 0);
        assert_eq!(fenwick_tree_to_vec(&fenwick_tree, 3), vec![1, 2, 3]);
    }
}
