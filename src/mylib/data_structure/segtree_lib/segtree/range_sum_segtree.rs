use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_sum_segtree::*;")]
pub mod range_sum_segtree {
    use ac_library::{Monoid, Segtree};
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::iter::Sum;
    use std::marker::PhantomData;
    use std::ops::{Add, RangeBounds};

    fn zero<T: Sum>() -> T {
        std::iter::empty::<T>().sum()
    }

    /// 汎用的な加算モノイド。
    /// `std::ops::Add` と `std::iter::Sum` を実装している型に対応。
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct GeneralAdditive<T>(Infallible, PhantomData<fn() -> T>);

    impl<T> Monoid for GeneralAdditive<T>
    where
        T: Sum + Add<Output = T> + Copy,
    {
        type S = T;
        #[inline]
        fn identity() -> Self::S {
            zero()
        }
        #[inline]
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
    }

    /// ACL の Segtree を使用した区間和セグメント木。
    /// 数値型 T に対して点更新・区間和取得を行う。
    #[derive(Clone)]
    pub struct RangeSumSegtree<T>
    where
        T: Sum + Add<Output = T> + Copy,
    {
        segtree: Segtree<GeneralAdditive<T>>,
        len: usize,
    }

    impl<T> RangeSumSegtree<T>
    where
        T: Sum + Add<Output = T> + Copy,
    {
        /// 単位元で初期化されたセグメント木を構築する
        pub fn new(n: usize) -> Self {
            Self {
                segtree: Segtree::<GeneralAdditive<T>>::new(n),
                len: n,
            }
        }

        /// 配列からセグメント木を構築する
        pub fn from_slice(xs: &[T]) -> Self {
            let len = xs.len();
            Self {
                segtree: Segtree::<GeneralAdditive<T>>::from(xs.to_vec()),
                len,
            }
        }

        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            self.len
        }

        /// p 番目の要素を x に更新する
        pub fn set(&mut self, p: usize, x: T) {
            self.segtree.set(p, x);
        }

        /// p 番目の要素を取得する
        pub fn get(&self, p: usize) -> T {
            self.segtree.get(p)
        }

        /// 指定した範囲の和を取得する
        pub fn range_sum<R>(&self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range)
        }

        /// 全要素の和を取得する
        pub fn all_sum(&self) -> T {
            self.segtree.all_prod()
        }

        /// p 番目の要素に x を加算する
        pub fn add(&mut self, p: usize, x: T) {
            let current = self.get(p);
            self.set(p, current + x);
        }

        /// セグメント木上の二分探索。
        /// [l, r) の和 s について f(&s) が true となる最大の r を返す。
        pub fn max_right<F>(&self, l: usize, f: F) -> usize
        where
            F: Fn(&T) -> bool,
        {
            self.segtree.max_right(l, f)
        }

        /// セグメント木上の二分探索。
        /// [l, r) の和 s について f(&s) が true となる最小の l を返す。
        pub fn min_left<F>(&self, r: usize, f: F) -> usize
        where
            F: Fn(&T) -> bool,
        {
            self.segtree.min_left(r, f)
        }

        /// 現在の状態を Vec として返す
        pub fn to_vec(&self) -> Vec<T> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::range_sum_segtree::*;
    use ac_library::ModInt998244353;

    #[test]
    fn test_new() {
        let n = 10;
        let mut seg = RangeSumSegtree::<i64>::new(n);
        assert_eq!(seg.len(), 10);
        assert_eq!(seg.all_sum(), 0);
        seg.add(0, 5);
        assert_eq!(seg.all_sum(), 5);
    }

    #[test]
    fn test_range_sum_segtree_primitive() {
        let mut seg = RangeSumSegtree::from_slice(&[1, 2, 3, 4, 5]);
        assert_eq!(seg.range_sum(0..3), 6); // 1 + 2 + 3
        seg.add(2, 10);
        assert_eq!(seg.get(2), 13);
        assert_eq!(seg.range_sum(0..3), 16);
        assert_eq!(seg.to_vec(), vec![1, 2, 13, 4, 5]);
    }

    #[test]
    fn test_range_sum_segtree_modint() {
        type Mint = ModInt998244353;
        let xs = vec![Mint::new(1), Mint::new(2), Mint::new(3)];
        let mut seg = RangeSumSegtree::from_slice(&xs);
        assert_eq!(seg.range_sum(0..2), Mint::new(3));
        seg.add(1, Mint::new(10));
        assert_eq!(seg.get(1), Mint::new(12));
    }

    #[test]
    fn test_max_right_min_left() {
        let seg = RangeSumSegtree::from_slice(&[1, 2, 3, 4, 5]);
        // [1, 2, 3, 4, 5]
        assert_eq!(seg.max_right(0, |&s| s <= 6), 3);
        assert_eq!(seg.min_left(5, |&s| s <= 10), 3);
    }

    #[ignore]
    #[test]
    fn test_random_add_sum() {
        use rand::Rng;

        let mut rng = rand::rng();

        for _ in 0..100 {
            let n = rng.random_range(1..=30);
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
            let mut segtree = RangeSumSegtree::<i64>::from_slice(&naive_vec);

            for _ in 0..100 {
                let op_type = rng.random_range(0..5);

                match op_type {
                    0 => {
                        let p = rng.random_range(0..n);
                        let x = rng.random_range(-100..=100);
                        naive_vec[p] = x;
                        segtree.set(p, x);
                    }
                    1 => {
                        let p = rng.random_range(0..n);
                        let x = rng.random_range(-50..=50);
                        naive_vec[p] += x;
                        segtree.add(p, x);
                    }
                    2 => {
                        let p = rng.random_range(0..n);
                        assert_eq!(segtree.get(p), naive_vec[p]);
                    }
                    3 => {
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);

                        let expected_sum: i64 = naive_vec[l..r].iter().sum();
                        assert_eq!(segtree.range_sum(l..r), expected_sum);
                    }
                    4 => {
                        let expected_sum: i64 = naive_vec.iter().sum();
                        assert_eq!(segtree.all_sum(), expected_sum);
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
