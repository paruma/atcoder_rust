use crate::math::algebra::min_max_monoid::min_max_monoid::MaxMonoid;
use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_max_segtree::*;", include = "min_max_monoid")]
pub mod range_max_segtree {
    use super::MaxMonoid;
    use ac_library::{Monoid, Segtree};
    use itertools::Itertools;
    use std::ops::RangeBounds;

    /// ACL の Segtree を使用した区間最大セグメント木。
    /// 数値型 T に対して点更新・区間最大取得を行う。
    #[derive(Clone)]
    pub struct RangeMaxSegtree<T>
    where
        MaxMonoid<T>: Monoid<S = T>,
        T: Clone,
    {
        segtree: Segtree<MaxMonoid<T>>,
        len: usize,
    }

    impl<T> RangeMaxSegtree<T>
    where
        MaxMonoid<T>: Monoid<S = T>,
        T: Copy + Ord,
    {
        /// 単位元で初期化されたセグメント木を構築する
        pub fn new(n: usize) -> Self {
            Self {
                segtree: Segtree::<MaxMonoid<T>>::new(n),
                len: n,
            }
        }

        /// 配列からセグメント木を構築する
        pub fn from_slice(xs: &[T]) -> Self {
            let len = xs.len();
            Self {
                segtree: Segtree::<MaxMonoid<T>>::from(xs.to_vec()),
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

        /// range の最大値を取得する
        pub fn range_max<R>(&self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range)
        }

        /// 全要素の最大値を取得する
        pub fn all_max(&self) -> T {
            self.segtree.all_prod()
        }

        /// セグメント木上の二分探索。
        /// [l, r) の最大値 s について f(&s) が true となる最大の r を返す。
        pub fn max_right<F>(&self, l: usize, f: F) -> usize
        where
            F: Fn(&T) -> bool,
        {
            self.segtree.max_right(l, f)
        }

        /// セグメント木上の二分探索。
        /// [l, r) の最大値 s について f(&s) が true となる最小の l を返す。
        pub fn min_left<F>(&self, r: usize, f: F) -> usize
        where
            F: Fn(&T) -> bool,
        {
            self.segtree.min_left(r, f)
        }

        /// p 番目の要素を max(current, x) に更新する
        pub fn chmax(&mut self, p: usize, x: T) {
            let current = self.get(p);
            self.set(p, std::cmp::max(current, x));
        }

        /// 現在の状態を Vec として返す
        pub fn to_vec(&self) -> Vec<T> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::range_max_segtree::*;

    #[test]
    fn test_new() {
        let n = 10;
        let mut seg = RangeMaxSegtree::<i64>::new(n);
        assert_eq!(seg.len(), 10);
        assert_eq!(seg.all_max(), i64::MIN);
        seg.set(0, 5);
        assert_eq!(seg.all_max(), 5);
    }

    #[test]
    fn test_range_max_segtree() {
        let mut seg = RangeMaxSegtree::from_slice(&[3, 1, 4, 1, 5, 9, 2]);
        assert_eq!(seg.range_max(0..3), 4);
        assert_eq!(seg.range_max(4..7), 9);
        assert_eq!(seg.all_max(), 9);

        seg.set(5, 0);
        assert_eq!(seg.all_max(), 5);
        assert_eq!(seg.get(5), 0);
        assert_eq!(seg.to_vec(), vec![3, 1, 4, 1, 5, 0, 2]);

        seg.chmax(0, 5);
        assert_eq!(seg.get(0), 5);
        seg.chmax(0, 1);
        assert_eq!(seg.get(0), 5);
    }

    #[test]
    fn test_range_max_segtree_tuple() {
        // (値, インデックス) のペアで最大値を管理（同じ値ならインデックスが大きい方を優先）
        let xs = vec![(10, 0), (20, 1), (20, 2), (15, 3)];
        let mut seg = RangeMaxSegtree::from_slice(&xs);

        // 辞書式順序で比較されるため (20, 2) が最大
        assert_eq!(seg.range_max(0..4), (20, 2));
        assert_eq!(seg.range_max(0..2), (20, 1));

        seg.set(1, (25, 1));
        assert_eq!(seg.all_max(), (25, 1));
    }

    #[test]
    fn test_max_right_min_left() {
        let seg = RangeMaxSegtree::from_slice(&[2, 4, 6, 8, 10]);
        assert_eq!(seg.max_right(0, |&s| s <= 7), 3);
        assert_eq!(seg.min_left(5, |&s| s <= 7), 5);
    }

    #[ignore]
    #[test]
    fn test_random_max() {
        use rand::Rng;
        let mut rng = rand::rng();
        for _ in 0..100 {
            let n = rng.random_range(1..=30);
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(-100..=100)).collect();
            let mut segtree = RangeMaxSegtree::<i64>::from_slice(&naive_vec);
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
                        let expected = naive_vec[l..r].iter().max().copied().unwrap_or(i64::MIN);
                        assert_eq!(segtree.range_max(l..r), expected);
                    }
                    3 => {
                        let expected = naive_vec.iter().max().copied().unwrap_or(i64::MIN);
                        assert_eq!(segtree.all_max(), expected);
                    }
                    4 => {
                        let r = rng.random_range(0..=n);
                        let target = rng.random_range(-100..=100);
                        let actual = segtree.min_left(r, |&s| s <= target);
                        let mut expected = r;
                        let mut cur_max = i64::MIN;
                        for i in (0..r).rev() {
                            cur_max = std::cmp::max(cur_max, naive_vec[i]);
                            if cur_max <= target {
                                expected = i;
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
