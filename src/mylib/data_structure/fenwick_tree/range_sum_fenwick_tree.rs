use crate::math::algebra::ab_group::ab_group::{AbGroup, AdditiveAbGroup};
use cargo_snippet::snippet;

#[snippet(prefix = "use range_sum_fenwick_tree::*;", include = "ab_group")]
#[allow(clippy::module_inception)]
pub mod range_sum_fenwick_tree {
    use super::{AbGroup, AdditiveAbGroup};
    use std::ops::{Bound, RangeBounds};

    /// 可換群 (AbGroup) を用いた汎用的な Fenwick Tree (Binary Indexed Tree)。
    ///
    /// 0-indexed で実装されています。
    /// 基本的な加算・区間和クエリに加え、get/set や、二分探索 (max_right / min_left) を提供します。
    #[derive(Clone)]
    pub struct RangeSumFenwickTreeArbitrary<G: AbGroup> {
        n: usize,
        pub(crate) ary: Vec<G::S>,
    }

    /// i64 の加算群を用いた標準的な Fenwick Tree のエイリアス。
    pub type RangeSumFenwickTreeI64 = RangeSumFenwickTreeArbitrary<AdditiveAbGroup<i64>>;

    /// 任意の数値型 T の加算群を用いた Fenwick Tree のエイリアス。
    pub type RangeSumFenwickTree<T> = RangeSumFenwickTreeArbitrary<AdditiveAbGroup<T>>;

    pub type FenwickTree<T> = RangeSumFenwickTree<T>;

    impl<G: AbGroup> RangeSumFenwickTreeArbitrary<G> {
        /// サイズ `n` の Fenwick Tree を作成します。
        /// 要素はすべて `G::zero()` で初期化されます。
        ///
        /// # 計算量
        /// O(n)
        pub fn new(n: usize) -> Self {
            let ary = vec![G::zero(); n];
            RangeSumFenwickTreeArbitrary { n, ary }
        }

        /// 配列スライスから Fenwick Tree を作成します。
        ///
        /// # 計算量
        /// O(n)
        pub fn from_slice(slice: &[G::S]) -> Self {
            let n = slice.len();
            let mut ary = slice.to_vec();
            for i in 0..n {
                let j = i | (i + 1);
                if j < n {
                    let val_i = ary[i].clone();
                    ary[j] = G::add(&ary[j], &val_i);
                }
            }
            RangeSumFenwickTreeArbitrary { n, ary }
        }

        /// `[0, idx)` の区間の総和を計算します。
        ///
        /// # Panics
        /// `idx > n` の場合にパニックします。
        ///
        /// # 計算量
        /// O(log n)
        pub fn prefix_sum(&self, mut idx: usize) -> G::S {
            assert!(
                idx <= self.n,
                "RangeSumFenwickTreeArbitrary::prefix_sum: index out of bounds. idx: {}, n: {}",
                idx,
                self.n
            );
            let mut sum = G::zero();
            while idx > 0 {
                sum = G::add(&sum, &self.ary[idx - 1]);
                idx &= idx - 1;
            }
            sum
        }

        /// `idx` 番目の要素に `val` を加算（群の演算を適用）します。
        ///
        /// # Panics
        /// `idx >= n` の場合にパニックします。
        ///
        /// # 計算量
        /// O(log n)
        pub fn add(&mut self, mut idx: usize, val: G::S) {
            assert!(
                idx < self.n,
                "RangeSumFenwickTreeArbitrary::add: index out of bounds. idx: {}, n: {}",
                idx,
                self.n
            );
            let n = self.n;
            idx += 1;
            while idx <= n {
                self.ary[idx - 1] = G::add(&self.ary[idx - 1], &val);
                idx += idx & idx.wrapping_neg();
            }
        }

        fn resolve_range<R: RangeBounds<usize>>(&self, range: R) -> (usize, usize) {
            let l = match range.start_bound() {
                Bound::Included(&l) => l,
                Bound::Excluded(&l) => l + 1,
                Bound::Unbounded => 0,
            };
            let r = match range.end_bound() {
                Bound::Included(&r) => r + 1,
                Bound::Excluded(&r) => r,
                Bound::Unbounded => self.n,
            };
            (l, r)
        }

        /// 指定された範囲の区間和を計算します。
        ///
        /// # Panics
        /// 範囲が不正、または `n` を超える場合にパニックします。
        ///
        /// # 計算量
        /// O(log n)
        pub fn range_sum<R>(&self, range: R) -> G::S
        where
            R: RangeBounds<usize>,
        {
            let (l, r) = self.resolve_range(range);
            assert!(
                l <= r && r <= self.n,
                "RangeSumFenwickTreeArbitrary::range_sum: invalid range. l: {}, r: {}, n: {}",
                l,
                r,
                self.n
            );
            G::sub(&self.prefix_sum(r), &self.prefix_sum(l))
        }

        /// `l` を左端として、`f(sum(l..r))` が true になる最大の `r` を返します。
        ///
        /// `f` は単調性を持つ必要があります。
        /// 具体的には、`f(sum(l..i))` が true ならば、任意の `j < i` に対して `f(sum(l..j))` も true である必要があります。
        /// また、`f(zero)` は true である必要があります。
        ///
        /// # Panics
        /// `l > n` または `f(zero)` が false の場合にパニックします。
        ///
        /// # 計算量
        /// O(log n)
        pub fn max_right<F>(&self, l: usize, mut f: F) -> usize
        where
            F: FnMut(&G::S) -> bool,
        {
            assert!(
                l <= self.n,
                "RangeSumFenwickTreeArbitrary::max_right: index out of bounds. l: {}, n: {}",
                l,
                self.n
            );
            let zero = G::zero();
            assert!(
                f(&zero),
                "RangeSumFenwickTreeArbitrary::max_right: The predicate f(zero) must be true."
            );
            let val_l = self.prefix_sum(l);
            let mut r = 0;
            let mut current_val = G::zero();
            let mut k = if self.n == 0 {
                0
            } else {
                1 << (usize::BITS - 1 - self.n.leading_zeros())
            };

            while k > 0 {
                if r + k <= self.n {
                    let next_val = G::add(&current_val, &self.ary[r + k - 1]);
                    if r + k <= l || f(&G::sub(&next_val, &val_l)) {
                        r += k;
                        current_val = next_val;
                    }
                }
                k >>= 1;
            }
            r
        }

        /// `r` を右端として、`f(sum(l..r))` が true になる最小の `l` を返します。
        ///
        /// `f` は単調性を持つ必要があります。
        /// 具体的には、`f(sum(i..r))` が true ならば、任意の `j > i` に対して `f(sum(j..r))` も true である必要があります。
        /// また、`f(zero)` は true である必要があります。
        ///
        /// # Panics
        /// `r > n` または `f(zero)` が false の場合にパニックします。
        ///
        /// # 計算量
        /// O(log n)
        pub fn min_left<F>(&self, r: usize, mut f: F) -> usize
        where
            F: FnMut(&G::S) -> bool,
        {
            assert!(
                r <= self.n,
                "RangeSumFenwickTreeArbitrary::min_left: index out of bounds. r: {}, n: {}",
                r,
                self.n
            );
            let zero = G::zero();
            assert!(
                f(&zero),
                "RangeSumFenwickTreeArbitrary::min_left: The predicate f(zero) must be true."
            );

            let val_r = self.prefix_sum(r);
            if f(&val_r) {
                return 0;
            }

            let mut idx = 0;
            let mut current_val = G::zero();
            let mut k = if self.n == 0 {
                0
            } else {
                1 << (usize::BITS - 1 - self.n.leading_zeros())
            };

            while k > 0 {
                if idx + k <= r {
                    let next_val = G::add(&current_val, &self.ary[idx + k - 1]);
                    if !f(&G::sub(&val_r, &next_val)) {
                        idx += k;
                        current_val = next_val;
                    }
                }
                k >>= 1;
            }
            idx + 1
        }

        /// `idx` 番目の要素の値を取得します。
        ///
        /// # Panics
        /// `idx >= n` の場合にパニックします。
        ///
        /// # 計算量
        /// O(log n)
        pub fn get(&self, idx: usize) -> G::S {
            assert!(
                idx < self.n,
                "RangeSumFenwickTreeArbitrary::get: index out of bounds. idx: {}, n: {}",
                idx,
                self.n
            );
            self.range_sum(idx..=idx)
        }

        /// `idx` 番目の要素の値を `val` に設定します。
        ///
        /// # Panics
        /// `idx >= n` の場合にパニックします。
        ///
        /// # 計算量
        /// O(log n)
        pub fn set(&mut self, idx: usize, val: G::S) {
            assert!(
                idx < self.n,
                "RangeSumFenwickTreeArbitrary::set: index out of bounds. idx: {}, n: {}",
                idx,
                self.n
            );
            let old_val = self.get(idx);
            self.add(idx, G::sub(&val, &old_val));
        }

        /// Fenwick Tree の現在の状態を `Vec<G::S>` として返します。
        ///
        /// # 計算量
        /// O(n log n)
        pub fn to_vec(&self) -> Vec<G::S> {
            (0..self.n).map(|i| self.get(i)).collect()
        }

        /// 保持している要素数を返します。
        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            self.n
        }
    }
}

#[cfg(test)]
mod tests {
    use super::range_sum_fenwick_tree::*;
    use crate::math::algebra::ab_group::ab_group::AdditiveAbGroup;
    use rand::{Rng, SeedableRng, rngs::SmallRng};

    #[test]
    #[ignore]
    fn test_random_range_sum_fenwick_tree_arbitrary() {
        type G = AdditiveAbGroup<i64>;
        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.random_range(1..=20);
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
            let mut fenwick_tree = RangeSumFenwickTreeArbitrary::<G>::from_slice(&naive_vec);

            for _ in 0..100 {
                let op_type = rng.random_range(0..4); // 0: add, 1: get, 2: set, 3: range_sum

                match op_type {
                    0 => {
                        let idx = rng.random_range(0..n);
                        let val = rng.random_range(-50..=50);
                        naive_vec[idx] += val;
                        fenwick_tree.add(idx, val);
                    }
                    1 => {
                        let idx = rng.random_range(0..n);
                        assert_eq!(fenwick_tree.get(idx), naive_vec[idx], "get({}) failed", idx);
                    }
                    2 => {
                        let idx = rng.random_range(0..n);
                        let val = rng.random_range(-100..=100);
                        naive_vec[idx] = val;
                        fenwick_tree.set(idx, val);
                    }
                    3 => {
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let expected_sum: i64 = naive_vec[l..r].iter().sum();
                        assert_eq!(
                            fenwick_tree.range_sum(l..r),
                            expected_sum,
                            "range_sum({}..{}) failed",
                            l,
                            r
                        );
                    }
                    _ => unreachable!(),
                }
            }

            assert_eq!(
                fenwick_tree.to_vec(),
                naive_vec,
                "final to_vec() check failed"
            );
        }
    }

    #[test]
    fn test_len() {
        type G = AdditiveAbGroup<i64>;
        let ft1 = RangeSumFenwickTreeArbitrary::<G>::new(10);
        assert_eq!(ft1.len(), 10);

        let initial_vec = vec![1, 2, 3];
        let ft2 = RangeSumFenwickTreeArbitrary::<G>::from_slice(&initial_vec);
        assert_eq!(ft2.len(), 3);

        let ft_empty1 = RangeSumFenwickTreeArbitrary::<G>::new(0);
        assert_eq!(ft_empty1.len(), 0);
        let ft_empty2 = RangeSumFenwickTreeArbitrary::<G>::from_slice(&[]);
        assert_eq!(ft_empty2.len(), 0);
    }

    #[test]
    fn test_from_slice() {
        type G = AdditiveAbGroup<i64>;
        let initial_vec = vec![1, 2, 3, 4, 5];
        let ft = RangeSumFenwickTreeArbitrary::<G>::from_slice(&initial_vec);
        assert_eq!(ft.to_vec(), initial_vec);

        let empty_vec: Vec<i64> = vec![];
        let ft_empty = RangeSumFenwickTreeArbitrary::<G>::from_slice(&empty_vec);
        assert_eq!(ft_empty.to_vec(), empty_vec);
    }

    #[test]
    fn test_range_sum_empty() {
        type G = AdditiveAbGroup<i64>;
        let ft_empty = RangeSumFenwickTreeArbitrary::<G>::new(0);
        assert_eq!(ft_empty.range_sum(0..0), 0);
    }

    #[test]
    fn test_range_sum_patterns() {
        type G = AdditiveAbGroup<i64>;
        let initial_vec = vec![1, 2, 4, 8, 16];
        let ft = RangeSumFenwickTreeArbitrary::<G>::from_slice(&initial_vec);

        assert_eq!(ft.range_sum(..3), 1 + 2 + 4);
        assert_eq!(ft.range_sum(..=3), 1 + 2 + 4 + 8);
        assert_eq!(ft.range_sum(1..4), 2 + 4 + 8);
        assert_eq!(ft.range_sum(1..=4), 2 + 4 + 8 + 16);
        assert_eq!(ft.range_sum(2..), 4 + 8 + 16);
        assert_eq!(ft.range_sum(..), 1 + 2 + 4 + 8 + 16);

        use std::ops::Bound;
        // Excluded start, Excluded end
        assert_eq!(
            ft.range_sum((Bound::Excluded(1), Bound::Excluded(4))),
            4 + 8
        );
        // Included start, Included end
        assert_eq!(
            ft.range_sum((Bound::Included(1), Bound::Included(3))),
            2 + 4 + 8
        );
        // Unbounded start, Included end
        assert_eq!(
            ft.range_sum((Bound::Unbounded, Bound::Included(2))),
            1 + 2 + 4
        );
        // Excluded start, Unbounded end
        assert_eq!(ft.range_sum((Bound::Excluded(2), Bound::Unbounded)), 8 + 16);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_add_empty_tree_panics() {
        type G = AdditiveAbGroup<i64>;
        let mut ft_empty = RangeSumFenwickTreeArbitrary::<G>::new(0);
        ft_empty.add(0, 1);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_set_empty_tree_panics() {
        type G = AdditiveAbGroup<i64>;
        let mut ft_empty = RangeSumFenwickTreeArbitrary::<G>::new(0);
        ft_empty.set(0, 1);
    }

    #[test]
    fn test_xor_fenwick_tree() {
        use crate::math::algebra::ab_group::ab_group::XorAbGroup;
        let initial_vec: Vec<u64> = vec![1, 2, 4, 8, 16];
        let mut ft = RangeSumFenwickTreeArbitrary::<XorAbGroup>::from_slice(&initial_vec);

        // 1 ^ 2 ^ 4 = 7
        assert_eq!(ft.range_sum(0..3), 7);
        ft.add(1, 10); // initial_vec[1] becomes 2 ^ 10 = 8
        // 1 ^ 8 ^ 4 = 13
        assert_eq!(ft.range_sum(0..3), 13);
    }

    #[test]
    #[ignore]
    fn test_random_max_right() {
        type G = AdditiveAbGroup<i64>;
        let mut rng = SmallRng::seed_from_u64(100);

        for _ in 0..100 {
            let n = rng.random_range(1..=20);
            // 非負の要素で構成する
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(0..=10)).collect();
            let mut fenwick_tree = RangeSumFenwickTreeArbitrary::<G>::from_slice(&naive_vec);

            for _ in 0..100 {
                let op_type = rng.random_range(0..2);

                if op_type == 0 {
                    let idx = rng.random_range(0..n);
                    let val = rng.random_range(0..=10);
                    naive_vec[idx] += val;
                    fenwick_tree.add(idx, val);
                } else {
                    let l = rng.random_range(0..=n);
                    let threshold = rng.random_range(1..=200);
                    let f = |x: &i64| *x < threshold;

                    let expected_r = (l..=n)
                        .rev()
                        .find(|&r| {
                            let sum: i64 = naive_vec[l..r].iter().sum();
                            f(&sum)
                        })
                        .unwrap();

                    assert_eq!(
                        fenwick_tree.max_right(l, f),
                        expected_r,
                        "max_right failed. l={}, threshold={}, vec={:?}",
                        l,
                        threshold,
                        naive_vec
                    );
                }
            }
        }
    }

    #[test]
    #[ignore]
    fn test_random_min_left() {
        type G = AdditiveAbGroup<i64>;
        let mut rng = SmallRng::seed_from_u64(200);

        for _ in 0..100 {
            let n = rng.random_range(1..=20);
            // 非負の要素で構成する
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(0..=10)).collect();
            let mut fenwick_tree = RangeSumFenwickTreeArbitrary::<G>::from_slice(&naive_vec);

            for _ in 0..100 {
                let op_type = rng.random_range(0..2);

                if op_type == 0 {
                    let idx = rng.random_range(0..n);
                    let val = rng.random_range(0..=10);
                    naive_vec[idx] += val;
                    fenwick_tree.add(idx, val);
                } else {
                    let r = rng.random_range(0..=n);
                    let threshold = rng.random_range(1..=200);
                    let f = |x: &i64| *x < threshold;

                    let expected_l = (0..=r)
                        .find(|&l| {
                            let sum: i64 = naive_vec[l..r].iter().sum();
                            f(&sum)
                        })
                        .unwrap();

                    assert_eq!(
                        fenwick_tree.min_left(r, f),
                        expected_l,
                        "min_left failed. r={}, threshold={}, vec={:?}",
                        r,
                        threshold,
                        naive_vec
                    );
                }
            }
        }
    }
}
