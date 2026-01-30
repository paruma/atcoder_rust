use crate::data_structure::segtree_lib::fenwick_tree::range_sum_fenwick_tree::range_sum_fenwick_tree::RangeSumFenwickTreeArbitrary;
use crate::math::algebra::ab_group::ab_group::{AbGroup, AdditiveAbGroup};
use cargo_snippet::snippet;

#[snippet(
    prefix = "use range_add_fenwick_tree::*;",
    include = "range_sum_fenwick_tree"
)]
#[allow(clippy::module_inception)]
pub mod range_add_fenwick_tree {
    use super::{AbGroup, AdditiveAbGroup, RangeSumFenwickTreeArbitrary};
    use std::ops::{Bound, RangeBounds};

    /// 1次元の階差数列を管理する Fenwick Tree。
    /// 内部的には階差数列を `RangeSumFenwickTreeArbitrary` で管理しています。
    #[derive(Clone)]
    pub struct RangeAddFenwickTreeArbitrary<G: AbGroup> {
        ft: RangeSumFenwickTreeArbitrary<G>,
    }

    /// i64 の加算群を用いた標準的な Range Add Fenwick Tree のエイリアス。
    pub type RangeAddFenwickTreeI64 = RangeAddFenwickTreeArbitrary<AdditiveAbGroup<i64>>;

    /// 任意の数値型 T の加算群を用いた Range Add Fenwick Tree のエイリアス。
    pub type RangeAddFenwickTree<T> = RangeAddFenwickTreeArbitrary<AdditiveAbGroup<T>>;

    impl<G: AbGroup> RangeAddFenwickTreeArbitrary<G> {
        /// サイズ `n` の Range Add Fenwick Tree を作成します。
        ///
        /// # 計算量
        /// O(n)
        pub fn new(n: usize) -> Self {
            Self {
                ft: RangeSumFenwickTreeArbitrary::new(n + 1),
            }
        }

        /// 配列のスライスから Range Add Fenwick Tree を作成します。
        ///
        /// # 計算量
        /// O(n)
        pub fn from_slice(slice: &[G::S]) -> Self {
            let n = slice.len();
            let mut diff = vec![G::zero(); n + 1];
            if n > 0 {
                diff[0] = slice[0].clone();
                for i in 1..n {
                    diff[i] = G::sub(&slice[i], &slice[i - 1]);
                }
                diff[n] = G::neg(&slice[n - 1]);
            }
            Self {
                ft: RangeSumFenwickTreeArbitrary::from_slice(&diff),
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
            let (l, r) = self.resolve_range(range);
            let n = self.ft.len() - 1;
            assert!(
                l <= r && r <= n,
                "RangeAddFenwickTreeArbitrary::range_add: invalid range. l: {}, r: {}, n: {}",
                l,
                r,
                n
            );
            self.ft.add(l, val.clone());
            self.ft.add(r, G::neg(&val));
        }

        fn resolve_range<R: RangeBounds<usize>>(&self, range: R) -> (usize, usize) {
            let n = self.ft.len() - 1;
            let l = match range.start_bound() {
                Bound::Included(&l) => l,
                Bound::Excluded(&l) => l + 1,
                Bound::Unbounded => 0,
            };
            let r = match range.end_bound() {
                Bound::Included(&r) => r + 1,
                Bound::Excluded(&r) => r,
                Bound::Unbounded => n,
            };
            (l, r)
        }

        /// `idx` 番目の要素に `val` を加算します。
        ///
        /// # 計算量
        /// O(log n)
        pub fn add(&mut self, idx: usize, val: G::S) {
            self.range_add(idx..=idx, val);
        }

        /// `idx` 番目の要素の値を `val` に設定します。
        ///
        /// # 計算量
        /// O(log n)
        pub fn set(&mut self, idx: usize, val: G::S) {
            let old = self.get(idx);
            self.add(idx, G::sub(&val, &old));
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
                "RangeAddFenwickTreeArbitrary::get: index out of bounds. idx: {}, n: {}",
                idx,
                n
            );
            self.ft.prefix_sum(idx + 1)
        }

        /// 現在の状態を `Vec<G::S>` として返します。
        ///
        /// # 計算量
        /// O(n log n)
        pub fn to_vec(&self) -> Vec<G::S> {
            let n = self.ft.len() - 1;
            (0..n).map(|i| self.get(i)).collect()
        }

        /// 保持している要素数を返します。
        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            self.ft.len() - 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::range_add_fenwick_tree::*;
    use crate::math::algebra::ab_group::ab_group::AdditiveAbGroup;
    use rand::{Rng, SeedableRng, rngs::SmallRng};

    #[test]
    fn test_range_add_fenwick_tree_basic() {
        type G = AdditiveAbGroup<i64>;
        let n = 5;
        let mut ft = RangeAddFenwickTreeArbitrary::<G>::new(n);
        assert_eq!(ft.len(), 5);

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
        let mut ft = RangeAddFenwickTreeArbitrary::<G>::from_slice(&initial);
        assert_eq!(ft.to_vec(), initial);

        ft.range_add(0..5, 1);
        assert_eq!(ft.to_vec(), vec![2, 3, 4, 5, 6]);
    }

    #[test]
    #[ignore]
    fn test_random_range_add_fenwick_tree() {
        type G = AdditiveAbGroup<i64>;
        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.random_range(1..=20);
            let mut naive_vec = vec![0; n];
            let mut ft = RangeAddFenwickTreeArbitrary::<G>::new(n);

            for _ in 0..100 {
                let op = rng.random_range(0..4);
                match op {
                    0 => {
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let val = rng.random_range(-100..=100);
                        for i in l..r {
                            naive_vec[i] += val;
                        }
                        ft.range_add(l..r, val);
                    }
                    1 => {
                        let idx = rng.random_range(0..n);
                        assert_eq!(ft.get(idx), naive_vec[idx]);
                    }
                    2 => {
                        let idx = rng.random_range(0..n);
                        let val = rng.random_range(-100..=100);
                        naive_vec[idx] += val;
                        ft.add(idx, val);
                    }
                    3 => {
                        let idx = rng.random_range(0..n);
                        let val = rng.random_range(-100..=100);
                        naive_vec[idx] = val;
                        ft.set(idx, val);
                    }
                    _ => unreachable!(),
                }
            }
            assert_eq!(ft.to_vec(), naive_vec);
        }
    }
}
