use crate::data_structure::segtree_lib::fenwick_tree::fenwick_tree::fenwick_tree::FenwickTreeArbitrary;
use crate::math::algebra::ab_group::ab_group::{AbGroup, AdditiveAbGroup};
use cargo_snippet::snippet;

#[snippet(
    prefix = "use range_add_range_sum_fenwick_tree::*;",
    include = "fenwick_tree"
)]
#[allow(clippy::module_inception)]
pub mod range_add_range_sum_fenwick_tree {
    use super::{AbGroup, AdditiveAbGroup, FenwickTreeArbitrary};
    use std::ops::{Bound, RangeBounds};

    /// 区間加算・区間和取得が可能な Fenwick Tree。
    #[derive(Clone)]
    pub struct RangeAddRangeSumFenwickTreeArbitrary<G: AbGroup> {
        n: usize,
        ft0: FenwickTreeArbitrary<G>,
        ft1: FenwickTreeArbitrary<G>,
    }

    /// i64 の加算群を用いた標準的な Range Add Range Sum Fenwick Tree のエイリアス。
    pub type RangeAddRangeSumFenwickTreeI64 =
        RangeAddRangeSumFenwickTreeArbitrary<AdditiveAbGroup<i64>>;

    impl<G: AbGroup> RangeAddRangeSumFenwickTreeArbitrary<G>
    where
        G::S: Copy + std::ops::Mul<Output = G::S> + From<i64>,
    {
        /// サイズ `n` の Range Add Range Sum Fenwick Tree を作成します。
        ///
        /// # 計算量
        /// O(n)
        pub fn new(n: usize) -> Self {
            RangeAddRangeSumFenwickTreeArbitrary {
                n,
                ft0: FenwickTreeArbitrary::new(n + 1),
                ft1: FenwickTreeArbitrary::new(n + 1),
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
            let r = match range.end_bound() {
                Bound::Included(r) => r + 1,
                Bound::Excluded(r) => *r,
                Bound::Unbounded => self.n,
            };
            let l = match range.start_bound() {
                Bound::Included(l) => *l,
                Bound::Excluded(l) => l + 1,
                Bound::Unbounded => 0,
            };
            assert!(
                l <= r && r <= self.n,
                "RangeAddRangeSumFenwickTreeArbitrary::range_add: invalid range. l: {}, r: {}, n: {}",
                l,
                r,
                self.n
            );

            // ft0: d[i]
            self.ft0.add(l, val);
            self.ft0.add(r, G::neg(&val));

            // ft1: d[i] * i
            let l_val = val * G::S::from(l as i64);
            let r_val = G::neg(&val) * G::S::from(r as i64);
            self.ft1.add(l, l_val);
            self.ft1.add(r, r_val);
        }

        /// `[0, idx)` の区間和を計算します。
        ///
        /// # 計算量
        /// O(log n)
        pub fn accum(&self, idx: usize) -> G::S {
            let sum0 = self.ft0.accum(idx);
            let sum1 = self.ft1.accum(idx);
            // ret = sum0 * idx - sum1
            G::sub(&(sum0 * G::S::from(idx as i64)), &sum1)
        }

        /// 指定された範囲 `range` の区間和を計算します。
        ///
        /// # Panics
        /// 範囲が不正な場合にパニックします。
        ///
        /// # 計算量
        /// O(log n)
        pub fn range_sum<R>(&self, range: R) -> G::S
        where
            R: RangeBounds<usize>,
        {
            let r = match range.end_bound() {
                Bound::Included(r) => r + 1,
                Bound::Excluded(r) => *r,
                Bound::Unbounded => self.n,
            };
            let l = match range.start_bound() {
                Bound::Included(l) => *l,
                Bound::Excluded(l) => l + 1,
                Bound::Unbounded => return self.accum(r),
            };
            assert!(
                l <= r && r <= self.n,
                "RangeAddRangeSumFenwickTreeArbitrary::range_sum: invalid range. l: {}, r: {}, n: {}",
                l,
                r,
                self.n
            );
            G::sub(&self.accum(r), &self.accum(l))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::range_add_range_sum_fenwick_tree::*;
    use crate::math::algebra::ab_group::ab_group::AdditiveAbGroup;
    use rand::{Rng, SeedableRng, rngs::SmallRng};

    #[test]
    #[allow(clippy::identity_op)]
    fn test_range_add_range_sum_basic() {
        type G = AdditiveAbGroup<i64>;
        let n = 5;
        let mut ft = RangeAddRangeSumFenwickTreeArbitrary::<G>::new(n);

        ft.range_add(1..4, 10);
        assert_eq!(ft.range_sum(0..5), 30i64);
        assert_eq!(ft.range_sum(1..4), 30i64);
        assert_eq!(ft.range_sum(2..3), 10i64);
        assert_eq!(ft.range_sum(0..2), 10i64);

        ft.range_add(2..5, 5);
        assert_eq!(ft.range_sum(0..5), (0 + 10 + 15 + 15 + 5) as i64);
    }

    #[test]
    #[ignore]
    fn test_random_range_add_range_sum() {
        type G = AdditiveAbGroup<i64>;
        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.random_range(1..=20);
            let mut naive_vec = vec![0; n];
            let mut ft = RangeAddRangeSumFenwickTreeArbitrary::<G>::new(n);

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
                    let l = rng.random_range(0..=n);
                    let r = rng.random_range(l..=n);
                    let expected: i64 = naive_vec[l..r].iter().sum();
                    assert_eq!(ft.range_sum(l..r), expected);
                }
            }
        }
    }
}
