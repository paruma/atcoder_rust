use crate::math::algebra::min_max_monoid::min_max_monoid::TupleMin;
use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_min_segtree::*;")]
pub mod range_min_segtree {
    use super::TupleMin;
    use ac_library::{Monoid, Segtree};
    use itertools::Itertools;
    use std::ops::RangeBounds;

    /// ACL の Segtree を使用した区間最小セグメント木。
    /// 数値型 T に対して点更新・区間最小取得を行う。
    pub struct RangeMinSegtree<T>
    where
        TupleMin<T>: Monoid<S = T>,
        T: Clone,
    {
        segtree: Segtree<TupleMin<T>>,
        len: usize,
    }

    impl<T> RangeMinSegtree<T>
    where
        TupleMin<T>: Monoid<S = T>,
        T: Copy + Ord,
    {
        /// 配列からセグメント木を構築する
        pub fn new(xs: &[T]) -> Self {
            let len = xs.len();
            Self {
                segtree: Segtree::<TupleMin<T>>::from(xs.to_vec()),
                len,
            }
        }

        /// p 番目の要素を x に更新する
        pub fn set(&mut self, p: usize, x: T) {
            self.segtree.set(p, x);
        }

        /// p 番目の要素を取得する
        pub fn get(&self, p: usize) -> T {
            self.segtree.get(p)
        }

        /// range の最小値を取得する
        pub fn range_min<R>(&self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range)
        }

        /// 全要素の最小値を取得する
        pub fn all_min(&self) -> T {
            self.segtree.all_prod()
        }

        /// セグメント木上の二分探索。
        /// [l, r) の最小値 s について f(&s) が true となる最大の r を返す。
        pub fn max_right<F>(&self, l: usize, f: F) -> usize
        where
            F: Fn(&T) -> bool,
        {
            self.segtree.max_right(l, f)
        }

        /// セグメント木上の二分探索。
        /// [l, r) の最小値 s について f(&s) が true となる最小の l を返す。
        pub fn min_left<F>(&self, r: usize, f: F) -> usize
        where
            F: Fn(&T) -> bool,
        {
            self.segtree.min_left(r, f)
        }

        /// p 番目の要素を min(current, x) に更新する
        pub fn chmin(&mut self, p: usize, x: T) {
            let current = self.get(p);
            self.set(p, std::cmp::min(current, x));
        }

        /// 現在の状態を Vec として返す
        pub fn to_vec(&self) -> Vec<T> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::range_min_segtree::*;

    #[test]
    fn test_range_min_segtree() {
        let mut seg = RangeMinSegtree::new(&[3, 1, 4, 1, 5, 9, 2]);
        assert_eq!(seg.range_min(0..3), 1);
        assert_eq!(seg.range_min(2..3), 4);
        assert_eq!(seg.all_min(), 1);

        seg.set(1, 10);
        assert_eq!(seg.range_min(0..3), 3);
        assert_eq!(seg.get(1), 10);
        assert_eq!(seg.to_vec(), vec![3, 10, 4, 1, 5, 9, 2]);

        seg.chmin(0, 1);
        assert_eq!(seg.get(0), 1);
        seg.chmin(0, 5);
        assert_eq!(seg.get(0), 1);
    }

    #[test]
    fn test_range_min_segtree_tuple() {
        // (値, インデックス) のペアで最小値を管理
        let xs = vec![(10, 0), (5, 1), (5, 2), (15, 3)];
        let mut seg = RangeMinSegtree::new(&xs);

        // 辞書式順序で比較されるため (5, 1) が最小
        assert_eq!(seg.range_min(0..4), (5, 1));
        assert_eq!(seg.range_min(2..4), (5, 2));

        seg.set(1, (25, 1));
        assert_eq!(seg.all_min(), (5, 2));
    }

    #[test]
    fn test_max_right_min_left() {
        let seg = RangeMinSegtree::new(&[10, 8, 6, 4, 2]);
        assert_eq!(seg.max_right(0, |&s| s >= 5), 3);
        assert_eq!(seg.min_left(5, |&s| s >= 5), 5);
    }

    #[ignore]
    #[test]
    fn test_random_min() {
        use rand::Rng;
        let mut rng = rand::rng();
        for _ in 0..100 {
            let n = rng.random_range(1..=30);
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
            let mut segtree = RangeMinSegtree::<i64>::new(&naive_vec);
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
                        assert_eq!(segtree.get(p), naive_vec[p]);
                    }
                    2 => {
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let expected = naive_vec[l..r].iter().min().copied().unwrap_or(i64::MAX);
                        assert_eq!(segtree.range_min(l..r), expected);
                    }
                    3 => {
                        let expected = naive_vec.iter().min().copied().unwrap_or(i64::MAX);
                        assert_eq!(segtree.all_min(), expected);
                    }
                    4 => {
                        let l = rng.random_range(0..=n);
                        let target = rng.random_range(-100..=100);
                        let actual = segtree.max_right(l, |&s| s >= target);
                        let mut expected = l;
                        let mut cur_min = i64::MAX;
                        for i in l..n {
                            cur_min = std::cmp::min(cur_min, naive_vec[i]);
                            if cur_min >= target {
                                expected = i + 1;
                            } else {
                                break;
                            }
                        }
                        assert_eq!(actual, expected);
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
