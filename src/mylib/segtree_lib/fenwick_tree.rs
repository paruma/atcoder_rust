use cargo_snippet::snippet;

#[snippet(prefix = "use fenwick_tree::*;")]
#[allow(clippy::module_inception)]
pub mod fenwick_tree {
    use std::ops::{Bound, RangeBounds};

    // Reference: https://en.wikipedia.org/wiki/Fenwick_tree
    /// ACL の FenwickTree の拡張。
    ///
    /// get, set, to_vec などが追加されている。また sum は range_sum に名前変更している。
    #[derive(Clone, Debug)]
    pub struct FenwickTree<T> {
        n: usize,
        ary: Vec<T>,
        e: T,
    }

    impl<T: Clone + std::ops::AddAssign<T> + std::ops::Sub<Output = T> + std::fmt::Debug>
        FenwickTree<T>
    {
        /// サイズ `n` の `FenwickTree` を作成します。
        ///
        /// # 計算量
        ///
        /// O(N)
        pub fn new(n: usize, e: T) -> Self {
            FenwickTree {
                n,
                ary: vec![e.clone(); n],
                e,
            }
        }

        /// スライスから `FenwickTree` を作成します。
        ///
        /// # 計算量
        ///
        /// O(N)
        pub fn from_slice(slice: &[T], e: T) -> Self {
            let n = slice.len();
            let mut ary = slice.to_vec();
            for i in 0..n {
                let j = i | (i + 1);
                if j < n {
                    let val_i = ary[i].clone();
                    ary[j] += val_i;
                }
            }
            FenwickTree { n, ary, e }
        }
        /// `[0, idx)` の区間の累積和を計算します。
        ///
        /// # 計算量
        ///
        /// O(log N)
        pub fn accum(&self, mut idx: usize) -> T {
            assert!(
                idx <= self.n,
                "FenwickTree::accum: index out of bounds. idx: {}, n: {}",
                idx,
                self.n
            );
            let mut sum = self.e.clone();
            while idx > 0 {
                sum += self.ary[idx - 1].clone();
                idx &= idx - 1;
            }
            sum
        }
        /// `idx`番目の要素に`val`を加算します。
        ///
        /// # 計算量
        ///
        /// O(log N)
        pub fn add<U: Clone>(&mut self, mut idx: usize, val: U)
        where
            T: std::ops::AddAssign<U>,
        {
            assert!(
                idx < self.n,
                "FenwickTree::add: index out of bounds. idx: {}, n: {}",
                idx,
                self.n
            );
            let n = self.n;
            idx += 1;
            while idx <= n {
                self.ary[idx - 1] += val.clone();
                idx += idx & idx.wrapping_neg();
            }
        }
        /// `[l, r)` の区間和を計算します。
        ///
        /// # 計算量
        ///
        /// O(log N)
        pub fn range_sum<R>(&self, range: R) -> T
        where
            T: std::ops::Sub<Output = T>,
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
                "FenwickTree::range_sum: invalid range. l: {}, r: {}, n: {}",
                l,
                r,
                self.n
            );
            self.accum(r) - self.accum(l)
        }

        /// `l` を左端として、`f(sum(l..r))` が true になる最大の `r` を返します。
        ///
        /// `f` は単調である必要があります。つまり、ある `r` で `f` が false になったら、
        /// それ以降の `r' > r` でも false になる必要があります。
        /// また、`f(0)` は true である必要があります。
        ///
        /// # 前提条件
        ///
        /// 各要素は非負であり、累積和が単調増加する必要があります。
        ///
        /// # 計算量
        ///
        /// O(log N)
        pub fn max_right<F>(&self, l: usize, f: F) -> usize
        where
            T: std::ops::Sub<Output = T>,
            F: Fn(&T) -> bool,
        {
            assert!(
                l <= self.n,
                "FenwickTree::max_right: index out of bounds. l: {}, n: {}",
                l,
                self.n
            );
            assert!(
                f(&self.e),
                "FenwickTree::max_right: The predicate f(e) must be true. f(e) = f({:?}) was false.",
                self.e
            );
            let val_l = self.accum(l);
            let mut r = 0;
            let mut current_val = self.e.clone();
            let mut k = 1;
            while k <= self.n {
                k <<= 1;
            }
            k >>= 1;

            while k > 0 {
                if r + k <= self.n {
                    let mut next_val = current_val.clone();
                    next_val += self.ary[r + k - 1].clone();
                    if r + k <= l || f(&(next_val.clone() - val_l.clone())) {
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
        /// `f` は単調である必要があります。つまり、ある `l` で `f` が true になったら、
        /// それ以降の `l' > l` でも true になる必要があります。
        /// また、`f(0)` は true である必要があります。
        ///
        /// # 前提条件
        ///
        /// 各要素は非負であり、累積和が単調増加する必要があります。
        ///
        /// # 計算量
        ///
        /// O(log N)
        pub fn min_left<F>(&self, r: usize, f: F) -> usize
        where
            T: std::ops::Sub<Output = T>,
            F: Fn(&T) -> bool,
        {
            assert!(
                r <= self.n,
                "FenwickTree::min_left: index out of bounds. r: {}, n: {}",
                r,
                self.n
            );
            assert!(
                f(&self.e),
                "FenwickTree::min_left: The predicate f(e) must be true. f(e) = f({:?}) was false.",
                self.e
            );

            let val_r = self.accum(r);
            // sum(0..r) = val_r - 0 がすでに条件を満たすなら、最小の l は 0
            if f(&val_r) {
                return 0;
            }

            let mut idx = 0;
            let mut current_val = self.e.clone();
            let mut k = 1;
            while k <= self.n {
                k <<= 1;
            }
            k >>= 1;

            while k > 0 {
                if idx + k <= r {
                    let mut next_val = current_val.clone();
                    next_val += self.ary[idx + k - 1].clone();
                    // sum(idx+k .. r) = val_r - next_val
                    // f(sum) が false なら、もっと l を大きく（idx を右に）して sum を減らす必要がある
                    if !f(&(val_r.clone() - next_val.clone())) {
                        idx += k;
                        current_val = next_val;
                    }
                }
                k >>= 1;
            }
            idx + 1
        }

        /// `idx`番目の要素の値を取得します。
        ///
        /// # 計算量
        ///
        /// O(log N)
        pub fn get(&self, idx: usize) -> T
        where
            T: std::ops::Sub<Output = T>,
        {
            assert!(
                idx < self.n,
                "FenwickTree::get: index out of bounds. idx: {}, n: {}",
                idx,
                self.n
            );
            self.range_sum(idx..=idx)
        }

        /// `idx`番目の要素の値を`val`に設定します。
        ///
        /// # 計算量
        ///
        /// O(log N)
        pub fn set(&mut self, idx: usize, val: T)
        where
            T: std::ops::Sub<Output = T>,
        {
            assert!(
                idx < self.n,
                "FenwickTree::set: index out of bounds. idx: {}, n: {}",
                idx,
                self.n
            );
            let old_val = self.get(idx);
            self.add(idx, val - old_val);
        }

        /// Fenwick Treeの現在の状態を`Vec<T>`として返します。
        ///
        /// # 計算量
        ///
        /// O(N log N)
        pub fn to_vec(&self) -> Vec<T>
        where
            T: std::ops::Sub<Output = T>,
        {
            (0..self.n).map(|i| self.get(i)).collect()
        }

        /// Fenwick Treeが保持している要素の数を返します。
        ///
        /// # 計算量
        ///
        /// O(1)
        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            self.n
        }
    }
}

#[cfg(test)]
mod test_fenwick_tree {
    use super::fenwick_tree::*;
    use rand::{Rng, SeedableRng, rngs::SmallRng};

    #[ignore]
    #[test]
    fn test_random_fenwick_tree() {
        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.random_range(1..=20);
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
            let mut fenwick_tree = FenwickTree::<i64>::from_slice(&naive_vec, 0);

            for _ in 0..100 {
                let op_type = rng.random_range(0..4); // 0: add, 1: get, 2: set, 3: range_sum

                match op_type {
                    0 => {
                        // add(idx, val)
                        let idx = rng.random_range(0..n);
                        let val = rng.random_range(-50..=50);
                        naive_vec[idx] += val;
                        fenwick_tree.add(idx, val);
                    }
                    1 => {
                        // get(idx)
                        let idx = rng.random_range(0..n);
                        assert_eq!(fenwick_tree.get(idx), naive_vec[idx], "get({}) failed", idx);
                    }
                    2 => {
                        // set(idx, val)
                        let idx = rng.random_range(0..n);
                        let val = rng.random_range(-100..=100);
                        naive_vec[idx] = val;
                        fenwick_tree.set(idx, val);
                    }
                    3 => {
                        // range_sum(l..r)
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

            // 最終チェック
            assert_eq!(
                fenwick_tree.to_vec(),
                naive_vec,
                "final to_vec() check failed"
            );
        }
    }

    #[test]
    fn test_len() {
        let ft1 = FenwickTree::<i64>::new(10, 0);
        assert_eq!(ft1.len(), 10);

        let initial_vec = vec![1, 2, 3];
        let ft2 = FenwickTree::<i64>::from_slice(&initial_vec, 0);
        assert_eq!(ft2.len(), 3);

        // 空列のテスト
        let ft_empty1 = FenwickTree::<i64>::new(0, 0);
        assert_eq!(ft_empty1.len(), 0);
        let ft_empty2 = FenwickTree::<i64>::from_slice(&[], 0);
        assert_eq!(ft_empty2.len(), 0);
    }

    #[test]
    fn test_from_slice() {
        let initial_vec = vec![1, 2, 3, 4, 5];
        let ft = FenwickTree::<i64>::from_slice(&initial_vec, 0);
        assert_eq!(ft.to_vec(), initial_vec);

        let empty_vec: Vec<i64> = vec![];
        let ft_empty = FenwickTree::<i64>::from_slice(&empty_vec, 0);
        assert_eq!(ft_empty.to_vec(), empty_vec);
    }

    #[test]
    fn test_to_vec() {
        let initial_vec = vec![1, 2, 3, 4, 5];
        let mut ft = FenwickTree::<i64>::from_slice(&initial_vec, 0);

        // add操作後のto_vecのテスト
        ft.add(0, 10); // initial_vec[0] = 1 + 10 = 11
        let expected_vec_add = vec![11, 2, 3, 4, 5];
        assert_eq!(ft.to_vec(), expected_vec_add, "to_vec() failed after add");

        // set操作後のto_vecのテスト
        ft.set(2, 100); // initial_vec[2] = 100
        let expected_vec_set = vec![11, 2, 100, 4, 5];
        assert_eq!(ft.to_vec(), expected_vec_set, "to_vec() failed after set");

        // 空列のテスト
        let ft_empty1 = FenwickTree::<i64>::new(0, 0);
        assert_eq!(ft_empty1.to_vec(), vec![]);
        let ft_empty2 = FenwickTree::<i64>::from_slice(&[], 0);
        assert_eq!(ft_empty2.to_vec(), vec![]);
    }

    #[test]
    fn test_range_sum_empty() {
        let ft_empty = FenwickTree::<i64>::new(0, 0);
        assert_eq!(ft_empty.range_sum(0..0), 0);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_add_empty_tree_panics() {
        let mut ft_empty = FenwickTree::<i64>::new(0, 0);
        // addはパニックするはず
        ft_empty.add(0, 1);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_set_empty_tree_panics() {
        let mut ft_empty = FenwickTree::<i64>::new(0, 0);
        // setはパニックするはず
        ft_empty.set(0, 1);
    }

    #[ignore]
    #[test]
    fn test_random_max_right() {
        let mut rng = SmallRng::seed_from_u64(100);

        for _ in 0..100 {
            let n = rng.random_range(1..=20);
            // 非負の要素で構成する
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(0..=10)).collect();
            let mut fenwick_tree = FenwickTree::<i64>::from_slice(&naive_vec, 0);

            for _ in 0..100 {
                let op_type = rng.random_range(0..2);

                if op_type == 0 {
                    // add (非負の値を足す)
                    let idx = rng.random_range(0..n);
                    let val = rng.random_range(0..=10);
                    naive_vec[idx] += val;
                    fenwick_tree.add(idx, val);
                } else {
                    // query
                    // max_right
                    let l = rng.random_range(0..=n);
                    // f: sum < threshold
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

    #[ignore]
    #[test]
    fn test_random_min_left() {
        let mut rng = SmallRng::seed_from_u64(200);

        for _ in 0..100 {
            let n = rng.random_range(1..=20);
            // 非負の要素で構成する
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(0..=10)).collect();
            let mut fenwick_tree = FenwickTree::<i64>::from_slice(&naive_vec, 0);

            for _ in 0..100 {
                let op_type = rng.random_range(0..2);

                if op_type == 0 {
                    // add (非負の値を足す)
                    let idx = rng.random_range(0..n);
                    let val = rng.random_range(0..=10);
                    naive_vec[idx] += val;
                    fenwick_tree.add(idx, val);
                } else {
                    // query
                    // min_left
                    let r = rng.random_range(0..=n);
                    // f: sum < threshold
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
