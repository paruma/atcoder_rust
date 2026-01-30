use crate::data_structure::segtree_lib::fenwick_tree::range_sum_fenwick_tree::range_sum_fenwick_tree::RangeSumFenwickTree;
use cargo_snippet::snippet;

#[snippet(
    prefix = "use range_add_range_sum_fenwick_tree::*;",
    include = "range_sum_fenwick_tree"
)]
#[allow(clippy::module_inception)]
pub mod range_add_range_sum_fenwick_tree {
    use super::RangeSumFenwickTree;
    use std::iter::Sum;
    use std::ops::{Add, Bound, Mul, Neg, RangeBounds, Sub};

    /// 任意の数値型 T に対して区間加算・区間和取得が可能な Fenwick Tree (Range Add Range Sum Fenwick Tree)。
    //
    // [原理]
    // A[i] の階差数列を D[i] = A[i] - A[i-1] とすると、元の値は A[j] = Σ_{k=0}^j D[k] と表せる。
    // このとき、区間 [0, i) の総和 S(i) は以下の通り変形できる：
    // S(i) = Σ_{j=0}^{i-1} A[j]
    //      = Σ_{j=0}^{i-1} Σ_{k=0}^j D[k]
    //      = Σ_{k=0}^{i-1} (i - k) * D[k]  （各 D[k] が何回足されるかカウント）
    //      = i * Σ_{k=0}^{i-1} D[k] - Σ_{k=0}^{i-1} (k * D[k])
    //
    // したがって、
    // ft0: D[k] の合計を管理
    // ft1: k * D[k] の合計を管理
    // とすることで、S(i) = i * ft0.prefix_sum(i) - ft1.prefix_sum(i) として計算できる。
    #[derive(Clone)]
    pub struct RangeAddRangeSumFenwickTree<T>
    where
        T: Copy + Add<Output = T> + Sub<Output = T> + Mul<i64, Output = T> + Neg<Output = T> + Sum,
    {
        n: usize,
        ft0: RangeSumFenwickTree<T>,
        ft1: RangeSumFenwickTree<T>,
    }

    /// i64 の加算群を用いた標準的な Range Add Range Sum Fenwick Tree のエイリアス。
    pub type RangeAddRangeSumFenwickTreeI64 = RangeAddRangeSumFenwickTree<i64>;

    impl<T> RangeAddRangeSumFenwickTree<T>
    where
        T: Copy + Add<Output = T> + Sub<Output = T> + Mul<i64, Output = T> + Neg<Output = T> + Sum,
    {
        /// サイズ `n` の Range Add Range Sum Fenwick Tree を作成します。
        ///
        /// # 計算量
        /// O(n)
        pub fn new(n: usize) -> Self {
            RangeAddRangeSumFenwickTree {
                n,
                ft0: RangeSumFenwickTree::new(n + 1),
                ft1: RangeSumFenwickTree::new(n + 1),
            }
        }

        /// 配列のスライスから Range Add Range Sum Fenwick Tree を作成します。
        ///
        /// # 計算量
        /// O(n)
        pub fn from_slice(slice: &[T]) -> Self {
            let n = slice.len();
            let mut d = vec![std::iter::empty::<T>().sum(); n + 1];
            let mut di = vec![std::iter::empty::<T>().sum(); n + 1];
            if n > 0 {
                d[0] = slice[0];
                // di[0] = d[0] * 0 = 0
                for i in 1..n {
                    let val = slice[i] - slice[i - 1];
                    d[i] = val;
                    di[i] = val * (i as i64);
                }
                d[n] = -slice[n - 1];
                di[n] = d[n] * (n as i64);
            }
            Self {
                n,
                ft0: RangeSumFenwickTree::from_slice(&d),
                ft1: RangeSumFenwickTree::from_slice(&di),
            }
        }

        /// 指定された範囲 `range` に `val` を加算します。
        ///
        /// # 計算量
        /// O(log n)
        pub fn range_add<R>(&mut self, range: R, val: T)
        where
            R: RangeBounds<usize>,
        {
            let (l, r) = self.resolve_range(range);
            assert!(
                l <= r && r <= self.n,
                "RangeAddRangeSumFenwickTree::range_add: invalid range. l: {}, r: {}, n: {}",
                l,
                r,
                self.n
            );

            // ft0: d[i]
            self.ft0.add(l, val);
            self.ft0.add(r, -val);

            // ft1: d[i] * i
            let l_val = val * (l as i64);
            let r_val = (-val) * (r as i64);
            self.ft1.add(l, l_val);
            self.ft1.add(r, r_val);
        }

        /// `idx` 番目の要素に `val` を加算します。
        ///
        /// # 計算量
        /// O(log n)
        pub fn add(&mut self, idx: usize, val: T) {
            self.range_add(idx..=idx, val);
        }

        /// `idx` 番目の要素の値を `val` に設定します。
        ///
        /// # 計算量
        /// O(log n)
        pub fn set(&mut self, idx: usize, val: T) {
            let old = self.get(idx);
            self.add(idx, val - old);
        }

        /// `[0, idx)` の区間和を計算します。
        ///
        /// # 計算量
        /// O(log n)
        pub fn prefix_sum(&self, idx: usize) -> T {
            // S(idx) = idx * ΣD[k] - Σ(k * D[k])
            let sum0 = self.ft0.prefix_sum(idx);
            let sum1 = self.ft1.prefix_sum(idx);
            sum0 * (idx as i64) - sum1
        }

        /// 指定された範囲 `range` の区間和を計算します。
        ///
        /// # Panics
        /// 範囲が不正な場合にパニックします。
        ///
        /// # 計算量
        /// O(log n)
        pub fn range_sum<R>(&self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            let (l, r) = self.resolve_range(range);
            assert!(
                l <= r && r <= self.n,
                "RangeAddRangeSumFenwickTree::range_sum: invalid range. l: {}, r: {}, n: {}",
                l,
                r,
                self.n
            );
            self.prefix_sum(r) - self.prefix_sum(l)
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

        /// `p` 番目の要素を取得します。
        ///
        /// # 計算量
        /// O(log n)
        pub fn get(&self, p: usize) -> T {
            self.range_sum(p..=p)
        }

        /// `l` を左端として、`f(sum(l..r))` が true になる最大の `r` を返します。
        ///
        /// `f` は単調性を持つ必要があります。具体的には元の配列の要素がすべて非負である必要があります。
        /// また、`f(0)` は true である必要があります。
        ///
        /// # Panics
        /// `l > n` または `f(0)` が false の場合にパニックします。
        ///
        /// # 計算量
        /// O(log n)
        pub fn max_right<F>(&self, l: usize, mut f: F) -> usize
        where
            F: FnMut(&T) -> bool,
        {
            assert!(
                l <= self.n,
                "RangeAddRangeSumFenwickTree::max_right: index out of bounds. l: {}, n: {}",
                l,
                self.n
            );
            assert!(
                f(&std::iter::empty::<T>().sum()),
                "RangeAddRangeSumFenwickTree::max_right: The predicate f(0) must be true."
            );

            let val_l = self.prefix_sum(l);
            let mut r = 0;
            let mut sum0: T = std::iter::empty::<T>().sum();
            let mut sum1: T = std::iter::empty::<T>().sum();
            let mut k = if self.n + 1 == 0 {
                0
            } else {
                1 << (usize::BITS - 1 - (self.n + 1).leading_zeros())
            };

            while k > 0 {
                if r + k <= self.n {
                    let next_sum0 = sum0 + self.ft0.ary[r + k - 1];
                    let next_sum1 = sum1 + self.ft1.ary[r + k - 1];

                    // sum(0..r+k) = (r+k) * next_sum0 - next_sum1
                    let total_sum = next_sum0 * ((r + k) as i64) - next_sum1;
                    let current_range_sum = total_sum - val_l;

                    if r + k <= l || f(&current_range_sum) {
                        r += k;
                        sum0 = next_sum0;
                        sum1 = next_sum1;
                    }
                }
                k >>= 1;
            }
            r
        }

        /// `r` を右端として、`f(sum(l..r))` が true になる最小の `l` を返します。
        ///
        /// `f` は単調性を持つ必要があります。具体的には元の配列の要素がすべて非負である必要があります。
        /// また、`f(0)` は true である必要があります。
        ///
        /// # Panics
        /// `r > n` または `f(0)` が false の場合にパニックします。
        ///
        /// # 計算量
        /// O(log n)
        pub fn min_left<F>(&self, r: usize, mut f: F) -> usize
        where
            F: FnMut(&T) -> bool,
        {
            assert!(
                r <= self.n,
                "RangeAddRangeSumFenwickTree::min_left: index out of bounds. r: {}, n: {}",
                r,
                self.n
            );
            assert!(
                f(&std::iter::empty::<T>().sum()),
                "RangeAddRangeSumFenwickTree::min_left: The predicate f(0) must be true."
            );

            let val_r = self.prefix_sum(r);
            if f(&val_r) {
                return 0;
            }

            let mut idx = 0;
            let mut sum0: T = std::iter::empty::<T>().sum();
            let mut sum1: T = std::iter::empty::<T>().sum();
            let mut k = if self.n + 1 == 0 {
                0
            } else {
                1 << (usize::BITS - 1 - (self.n + 1).leading_zeros())
            };

            while k > 0 {
                if idx + k <= r {
                    let next_sum0 = sum0 + self.ft0.ary[idx + k - 1];
                    let next_sum1 = sum1 + self.ft1.ary[idx + k - 1];

                    // sum(0..idx+k) = (idx+k) * next_sum0 - next_sum1
                    let total_sum = next_sum0 * ((idx + k) as i64) - next_sum1;
                    let current_range_sum = val_r - total_sum;

                    if !f(&current_range_sum) {
                        idx += k;
                        sum0 = next_sum0;
                        sum1 = next_sum1;
                    }
                }
                k >>= 1;
            }
            idx + 1
        }

        /// 現在の状態を `Vec<T>` として返します。
        ///
        /// # 計算量
        /// O(n log n)
        pub fn to_vec(&self) -> Vec<T> {
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
    use super::range_add_range_sum_fenwick_tree::*;
    use rand::{rngs::SmallRng, Rng, SeedableRng};

    #[test]
    #[allow(clippy::identity_op)]
    fn test_range_add_range_sum_basic() {
        let n = 5;
        let mut ft = RangeAddRangeSumFenwickTree::<i64>::new(n);
        assert_eq!(ft.len(), 5);

        ft.range_add(1..4, 10);
        assert_eq!(ft.range_sum(0..5), 30i64);
        assert_eq!(ft.range_sum(1..4), 30i64);
        assert_eq!(ft.range_sum(2..3), 10i64);
        assert_eq!(ft.range_sum(0..2), 10i64);

        ft.range_add(2..5, 5);
        assert_eq!(ft.range_sum(0..5), (0 + 10 + 15 + 15 + 5) as i64);
    }

    #[test]
    fn test_range_add_range_sum_from_slice_basic() {
        let vals = vec![1, 3, 6, 10, 15];
        let ft = RangeAddRangeSumFenwickTree::<i64>::from_slice(&vals);

        assert_eq!(ft.to_vec(), vals);
        assert_eq!(ft.range_sum(0..3), 1 + 3 + 6);
        assert_eq!(ft.range_sum(1..4), 3 + 6 + 10);
        assert_eq!(ft.range_sum(0..5), 1 + 3 + 6 + 10 + 15);
        // Unbounded 範囲のテスト
        assert_eq!(ft.range_sum(..), 1 + 3 + 6 + 10 + 15);
        assert_eq!(ft.range_sum(2..), 6 + 10 + 15);
        assert_eq!(ft.range_sum(..3), 1 + 3 + 6);
    }

    #[test]
    fn test_modint_compatibility() {
        use ac_library::ModInt998244353 as Mint;
        let n = 5;
        let mut ft = RangeAddRangeSumFenwickTree::<Mint>::new(n);
        ft.range_add(1..4, Mint::new(10));
        assert_eq!(ft.range_sum(1..4), Mint::new(30));
    }

    #[test]
    #[ignore]
    fn test_random_range_add_range_sum() {
        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.random_range(1..=20);
            let mut naive_vec = vec![0; n];
            let mut ft = RangeAddRangeSumFenwickTree::<i64>::new(n);

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
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let expected: i64 = naive_vec[l..r].iter().sum();
                        assert_eq!(
                            ft.range_sum(l..r),
                            expected,
                            "range_sum failed: n={}, l={}, r={}",
                            n,
                            l,
                            r
                        );
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

    #[test]
    #[ignore]
    fn test_random_range_add_range_sum_from_slice() {
        let mut rng = SmallRng::seed_from_u64(42);

        for n in 1..=20 {
            let vals: Vec<i64> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
            let ft = RangeAddRangeSumFenwickTree::<i64>::from_slice(&vals);

            assert_eq!(ft.to_vec(), vals, "n={} failed", n);
        }
    }

    #[test]
    #[ignore]
    fn test_random_max_right() {
        let mut rng = SmallRng::seed_from_u64(100);

        for _ in 0..100 {
            let n = rng.random_range(1..=20);
            // 非負の要素で構成する
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(0..=10)).collect();
            let mut ft = RangeAddRangeSumFenwickTree::<i64>::from_slice(&naive_vec);

            for _ in 0..100 {
                let op_type = rng.random_range(0..2);

                if op_type == 0 {
                    let l = rng.random_range(0..=n);
                    let r = rng.random_range(l..=n);
                    let val = rng.random_range(0..=10);
                    for i in l..r {
                        naive_vec[i] += val;
                    }
                    ft.range_add(l..r, val);
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
                        ft.max_right(l, f),
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
        let mut rng = SmallRng::seed_from_u64(200);

        for _ in 0..100 {
            let n = rng.random_range(1..=20);
            // 非負の要素で構成する
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(0..=10)).collect();
            let mut ft = RangeAddRangeSumFenwickTree::<i64>::from_slice(&naive_vec);

            for _ in 0..100 {
                let op_type = rng.random_range(0..2);

                if op_type == 0 {
                    let l = rng.random_range(0..=n);
                    let r = rng.random_range(l..=n);
                    let val = rng.random_range(0..=10);
                    for i in l..r {
                        naive_vec[i] += val;
                    }
                    ft.range_add(l..r, val);
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
                        ft.min_left(r, f),
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
