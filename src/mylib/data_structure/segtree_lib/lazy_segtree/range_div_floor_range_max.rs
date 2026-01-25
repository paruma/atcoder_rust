use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_div_floor_range_max::*;")]
pub mod range_div_floor_range_max {
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::ops::RangeBounds;
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeMax(Infallible);
    impl Monoid for RangeMax {
        type S = i64;
        fn identity() -> i64 {
            i64::MIN
        }
        fn binary_operation(a: &i64, b: &i64) -> i64 {
            i64::max(*a, *b)
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeDivFloorRangeMax(Infallible);
    impl MapMonoid for RangeDivFloorRangeMax {
        type M = RangeMax;
        type F = i64; // 正の値のみを想定

        fn identity_map() -> i64 {
            1
        }
        fn composition(a: &i64, b: &i64) -> i64 {
            a.saturating_mul(*b)
        }

        fn mapping(f: &i64, x: &i64) -> i64 {
            if *x == i64::MIN {
                i64::MIN
            } else {
                x.div_euclid(*f)
            }
        }
    }

    /// 区間切り捨て除算と区間最大値取得を行う遅延セグメント木。
    ///
    /// 以下の操作をサポートします。
    /// - **区間切り捨て除算**: 区間 `[l, r)` の各要素 `a_i` を `floor(a_i / x)` に更新します。
    /// - **区間最大値取得**: 区間 `[l, r)` の要素の最大値を取得します。
    ///
    /// # 制約
    /// - 更新に使う値 `x` （割る数）は正の整数である必要があります。
    /// - セグメント木に乗せるデータは正負どちらでも問題ありません。
    #[derive(Clone)]
    pub struct RangeDivFloorRangeMaxSegtree {
        segtree: LazySegtree<RangeDivFloorRangeMax>,
        len: usize,
    }

    impl RangeDivFloorRangeMaxSegtree {
        pub fn new(n: usize) -> Self {
            let xs = vec![0; n];
            Self::from_slice(&xs)
        }

        pub fn from_slice(xs: &[i64]) -> RangeDivFloorRangeMaxSegtree {
            let len = xs.len();
            RangeDivFloorRangeMaxSegtree {
                segtree: LazySegtree::from(xs.to_vec()),
                len,
            }
        }

        pub fn set(&mut self, p: usize, x: i64) {
            self.segtree.set(p, x);
        }

        pub fn get(&mut self, p: usize) -> i64 {
            self.segtree.get(p)
        }

        pub fn range_max<R>(&mut self, range: R) -> i64
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range)
        }

        pub fn all_max(&self) -> i64 {
            self.segtree.all_prod()
        }

        /// A[p] <- A[p] / x  を計算する
        pub fn apply_div_floor(&mut self, p: usize, x: i64) {
            self.segtree.apply(p, x)
        }

        /// p in range に対して A[p] <- A[p] / x  を計算する
        pub fn apply_range_div_floor<R>(&mut self, range: R, x: i64)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, x)
        }

        /// 左端 `l` を固定し、区間 `[l, r)` での最大値が述語 `g` を満たすような最大の `r` を返します。
        ///
        /// # 計算量
        /// O(log N)
        pub fn max_right<G>(&mut self, l: usize, g: G) -> usize
        where
            G: Fn(i64) -> bool,
        {
            self.segtree.max_right(l, g)
        }

        /// 右端 `r` を固定し、区間 `[l, r)` での最大値が述語 `g` を満たすような最小の `l` を返します。
        ///
        /// # 計算量
        /// O(log N)
        pub fn min_left<G>(&mut self, r: usize, g: G) -> usize
        where
            G: Fn(i64) -> bool,
        {
            self.segtree.min_left(r, g)
        }

        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            self.len
        }
        pub fn to_vec(&mut self) -> Vec<i64> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}

#[cfg(test)]
mod test_range_div_floor_range_max {
    use super::range_div_floor_range_max::RangeDivFloorRangeMaxSegtree;

    #[test]
    fn test_new_and_get() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeDivFloorRangeMaxSegtree::from_slice(&xs);
        assert_eq!(segtree.get(0), 10);
        assert_eq!(segtree.get(2), 30);
        assert_eq!(segtree.get(4), 50);
    }

    #[test]
    fn test_range_max() {
        let xs = vec![10, 50, 30, 40, 20];
        let mut segtree = RangeDivFloorRangeMaxSegtree::from_slice(&xs);
        assert_eq!(segtree.range_max(1..4), 50);
        assert_eq!(segtree.range_max(2..5), 40);
    }

    #[test]
    fn test_apply_div_floor() {
        let xs = vec![10, 20, 30];
        let mut segtree = RangeDivFloorRangeMaxSegtree::from_slice(&xs);
        segtree.apply_div_floor(1, 2);
        assert_eq!(segtree.get(1), 10);
        segtree.apply_div_floor(2, 3);
        assert_eq!(segtree.get(2), 10);
    }

    #[test]
    fn test_apply_range_div_floor() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeDivFloorRangeMaxSegtree::from_slice(&xs);
        segtree.apply_range_div_floor(1..4, 2);
        // [10, 10, 15, 20, 50]
        assert_eq!(segtree.to_vec(), vec![10, 10, 15, 20, 50]);
        assert_eq!(segtree.range_max(1..4), 20);

        segtree.apply_range_div_floor(0..5, 5);
        // [2, 2, 3, 4, 10]
        assert_eq!(segtree.to_vec(), vec![2, 2, 3, 4, 10]);
    }

    #[test]
    fn test_negative_values() {
        // -5 / 2 = -3 (floor)
        let xs = vec![-10, -5, 0, 5, 10];
        let mut segtree = RangeDivFloorRangeMaxSegtree::from_slice(&xs);
        segtree.apply_range_div_floor(0..5, 2);
        assert_eq!(segtree.to_vec(), vec![-5, -3, 0, 2, 5]);
        assert_eq!(segtree.range_max(0..5), 5);

        // -3 / 2 = -2 (floor)
        segtree.apply_range_div_floor(0..5, 2);
        assert_eq!(segtree.to_vec(), vec![-3, -2, 0, 1, 2]);
    }

    #[test]
    fn test_composition_overflow() {
        let xs = vec![100, 200];
        let mut segtree = RangeDivFloorRangeMaxSegtree::from_slice(&xs);

        // 10^12 * 10^12 will overflow i64.
        // With saturating_mul, it becomes i64::MAX.
        // x / i64::MAX should be 0 (for |x| < i64::MAX).

        let large_val = 1_000_000_000_000i64; // 10^12

        // Apply large_val twice. Composition will be large_val * large_val -> saturates to i64::MAX
        segtree.apply_range_div_floor(0..2, large_val);
        segtree.apply_range_div_floor(0..2, large_val);

        // 100 / i64::MAX = 0
        assert_eq!(segtree.to_vec(), vec![0, 0]);
    }

    #[test]
    fn test_max_right_min_left() {
        let xs = vec![1, 10, 5, 20, 3];
        let mut segtree = RangeDivFloorRangeMaxSegtree::from_slice(&xs);
        // max_right: [0, r) で max が 10 以下の最大の r -> [0, 3) max=10, [0, 4) max=20
        assert_eq!(segtree.max_right(0, |m| m <= 10), 3);
        // min_left: [l, 5) で max が 10 以下の最小の l -> [4, 5) max=3, [3, 5) max=20
        assert_eq!(segtree.min_left(5, |m| m <= 10), 4);
    }

    #[ignore]
    #[test]
    fn test_random_div_max() {
        use rand::{Rng, SeedableRng, rngs::SmallRng};

        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.random_range(1..=20);
            // 負の数も含めてテスト
            let mut naive_vec: Vec<i64> = (0..n).map(|_| rng.random_range(-1000..=1000)).collect();
            let mut segtree = RangeDivFloorRangeMaxSegtree::from_slice(&naive_vec);

            for _ in 0..100 {
                let op_type = rng.random_range(0..5);

                match op_type {
                    0 => {
                        // set(p, x)
                        let p = rng.random_range(0..n);
                        let x = rng.random_range(-1000..=1000);
                        naive_vec[p] = x;
                        segtree.set(p, x);
                    }
                    1 => {
                        // apply_range_div_floor(range, x)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        if l == r {
                            continue;
                        }
                        // x は 1 以上にする (0除算回避)
                        // 稀に巨大な数を使ってオーバーフロー(合成の飽和)を誘発させる
                        let x = if rng.random_bool(0.1) {
                            rng.random_range(1..=1_000_000_000_000_000_000)
                        } else {
                            rng.random_range(1..=10)
                        };

                        for i in l..r {
                            naive_vec[i] = naive_vec[i].div_euclid(x);
                        }
                        segtree.apply_range_div_floor(l..r, x);
                    }
                    2 => {
                        // get(p)
                        let p = rng.random_range(0..n);
                        assert_eq!(segtree.get(p), naive_vec[p]);
                    }
                    3 => {
                        // range_max(range)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let expected = naive_vec[l..r].iter().copied().max().unwrap_or(i64::MIN);
                        assert_eq!(segtree.range_max(l..r), expected);
                    }
                    4 => {
                        // all_max()
                        let expected = naive_vec.iter().copied().max().unwrap_or(i64::MIN);
                        assert_eq!(segtree.all_max(), expected);
                    }
                    _ => unreachable!(),
                }
            }
            assert_eq!(segtree.to_vec(), naive_vec);
        }
    }
}
