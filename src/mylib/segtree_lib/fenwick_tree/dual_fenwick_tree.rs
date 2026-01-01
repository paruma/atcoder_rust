use crate::ab_group::ab_group::AbGroup;
use crate::segtree_lib::fenwick_tree::fenwick_tree::fenwick_tree::FenwickTreeArbitrary;
use cargo_snippet::snippet;

#[snippet(prefix = "use dual_fenwick_tree::*;", include = "fenwick_tree")]
#[allow(clippy::module_inception)]
pub mod dual_fenwick_tree {
    use super::{AbGroup, FenwickTreeArbitrary};
    use std::ops::RangeBounds;

    /// 区間加算・一点取得が可能な Fenwick Tree (Dual Fenwick Tree)。
    ///
    /// 内部的には階差数列を `FenwickTreeArbitrary` で管理しています。
    #[derive(Clone, Debug)]
    pub struct DualFenwickTree<G: AbGroup>
    where
        G::S: std::fmt::Debug,
    {
        ft: FenwickTreeArbitrary<G>,
    }

    impl<G: AbGroup> DualFenwickTree<G>
    where
        G::S: std::fmt::Debug,
    {
        /// サイズ `n` の Dual Fenwick Tree を作成します。
        /// 要素はすべて `G::zero()` で初期化されます。
        ///
        /// # 計算量
        /// O(n)
        pub fn new(n: usize) -> Self {
            Self {
                ft: FenwickTreeArbitrary::new(n + 1),
            }
        }

        /// 配列スライスから Dual Fenwick Tree を作成します。
        ///
        /// # 計算量
        /// O(n)
        pub fn from_slice(slice: &[G::S]) -> Self {
            let n = slice.len();
            let mut diff = Vec::with_capacity(n + 1);
            if n == 0 {
                diff.push(G::zero());
            } else {
                diff.push(slice[0].clone());
                for i in 1..n {
                    diff.push(G::sub(&slice[i], &slice[i - 1]));
                }
                diff.push(G::neg(&slice[n - 1]));
            }

            Self {
                ft: FenwickTreeArbitrary::from_slice(&diff),
            }
        }

        /// 指定された範囲 `range` に `val` を加算します。
        ///
        /// # Panics
        /// 範囲が不正な場合にパニックします。
        ///
        /// # 計算量
        /// O(log n)
        pub fn range_add<R>(&mut self, range: R, val: G::S)
        where
            R: RangeBounds<usize>,
        {
            let n = self.ft.len() - 1;
            let r = match range.end_bound() {
                std::ops::Bound::Included(r) => r + 1,
                std::ops::Bound::Excluded(r) => *r,
                std::ops::Bound::Unbounded => n,
            };
            let l = match range.start_bound() {
                std::ops::Bound::Included(l) => *l,
                std::ops::Bound::Excluded(l) => l + 1,
                std::ops::Bound::Unbounded => 0,
            };
            assert!(
                l <= r && r <= n,
                "DualFenwickTree::range_add: invalid range. l: {}, r: {}, n: {}",
                l,
                r,
                n
            );

            self.ft.add(l, val.clone());
            self.ft.add(r, G::neg(&val));
        }

        /// `idx` 番目の要素の値を取得します。
        ///
        /// # Panics
        /// `idx >= n` の場合にパニックします。
        ///
        /// # 計算量
        /// O(log n)
        pub fn get(&self, idx: usize) -> G::S {
            let n = self.ft.len() - 1;
            assert!(
                idx < n,
                "DualFenwickTree::get: index out of bounds. idx: {}, n: {}",
                idx,
                n
            );
            self.ft.accum(idx + 1)
        }

        /// 現在の状態を `Vec<G::S>` として返します。
        ///
        /// # 計算量
        /// O(n log n)
        pub fn to_vec(&self) -> Vec<G::S> {
            let n = self.ft.len() - 1;
            (0..n).map(|i| self.get(i)).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::dual_fenwick_tree::*;
    use crate::ab_group::ab_group::AdditiveAbGroup;
    use rand::{Rng, SeedableRng, rngs::SmallRng};

    #[test]
    fn test_dual_fenwick_tree_basic() {
        type G = AdditiveAbGroup<i64>;
        let n = 5;
        let mut ft = DualFenwickTree::<G>::new(n);

        ft.range_add(1..4, 10);
        assert_eq!(ft.get(0), 0);
        assert_eq!(ft.get(1), 10);
        assert_eq!(ft.get(2), 10);
        assert_eq!(ft.get(3), 10);
        assert_eq!(ft.get(4), 0);

        ft.range_add(2..5, 5);
        assert_eq!(ft.to_vec(), vec![0, 10, 15, 15, 5]);
    }

    #[test]
    fn test_from_slice() {
        type G = AdditiveAbGroup<i64>;
        let initial = vec![1, 2, 3, 4, 5];
        let mut ft = DualFenwickTree::<G>::from_slice(&initial);
        assert_eq!(ft.to_vec(), initial);

        ft.range_add(0..5, 1);
        assert_eq!(ft.to_vec(), vec![2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_random_dual_fenwick_tree() {
        type G = AdditiveAbGroup<i64>;
        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.random_range(1..=20);
            let mut naive_vec = vec![0; n];
            let mut ft = DualFenwickTree::<G>::new(n);

            for _ in 0..100 {
                let op = rng.random_range(0..2);
                if op == 0 {
                    let l = rng.random_range(0..=n);
                    let r = rng.random_range(l..=n);
                    let val = rng.random_range(-100..=100);
                    for i in l..r {
                        naive_vec[i] += val;
                    }
                    ft.range_add(l..r, val);
                } else {
                    let idx = rng.random_range(0..n);
                    assert_eq!(ft.get(idx), naive_vec[idx]);
                }
            }
            assert_eq!(ft.to_vec(), naive_vec);
        }
    }
}
