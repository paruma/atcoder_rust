use cargo_snippet::snippet;

#[allow(clippy::module_inception)]
#[snippet(prefix = "use range_mult_range_prod::*;")]
pub mod range_mult_range_prod {
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::marker::PhantomData;
    use std::ops::{Mul, RangeBounds};

    fn power<T>(base: T, exp: u64) -> T
    where
        T: Copy + Mul<Output = T> + From<i64>,
    {
        let mut res = 1.into();
        let mut base = base;
        let mut exp = exp;
        while exp > 0 {
            if exp % 2 == 1 {
                res = res * base;
            }
            base = base * base;
            exp /= 2;
        }
        res
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeProd<T> {
        pub prod: T,
        pub len: i64,
    }
    impl<T> RangeProd<T> {
        pub fn unit(x: T) -> RangeProd<T> {
            RangeProd { prod: x, len: 1 }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ValueLenProd<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for ValueLenProd<T>
    where
        T: Copy + Mul<Output = T> + From<i64>,
    {
        type S = RangeProd<T>;
        fn identity() -> RangeProd<T> {
            RangeProd {
                prod: 1.into(),
                len: 0,
            }
        }
        fn binary_operation(a: &RangeProd<T>, b: &RangeProd<T>) -> RangeProd<T> {
            RangeProd {
                prod: a.prod * b.prod,
                len: a.len + b.len,
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeMultRangeProd<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for RangeMultRangeProd<T>
    where
        T: Copy + Mul<Output = T> + From<i64>,
    {
        type M = ValueLenProd<T>;
        type F = T; // T means multiply by val

        fn identity_map() -> T {
            1.into()
        }
        fn composition(a: &T, b: &T) -> T {
            *a * *b
        }

        fn mapping(f: &T, x: &RangeProd<T>) -> RangeProd<T> {
            RangeProd {
                prod: x.prod * power(*f, x.len as u64),
                len: x.len,
            }
        }
    }

    /// 計算量が O((log N)^2) な点に注意。
    /// (作用で累乗計算をしている関係で log が1つ多い)
    #[derive(Clone)]
    pub struct RangeMultRangeProdSegtree<T>
    where
        T: Copy + Mul<Output = T> + From<i64>,
    {
        segtree: LazySegtree<RangeMultRangeProd<T>>,
        len: usize,
    }

    impl<T> RangeMultRangeProdSegtree<T>
    where
        T: Copy + Mul<Output = T> + From<i64>,
    {
        pub fn new(n: usize) -> Self {
            let xs = vec![0.into(); n];
            Self::from_slice(&xs)
        }

        pub fn from_slice(xs: &[T]) -> RangeMultRangeProdSegtree<T> {
            let xs = xs.iter().copied().map(RangeProd::unit).collect_vec();
            let len = xs.len();
            RangeMultRangeProdSegtree {
                segtree: LazySegtree::from(xs),
                len,
            }
        }

        pub fn set(&mut self, p: usize, x: T) {
            self.segtree.set(p, RangeProd::unit(x));
        }

        pub fn get(&mut self, p: usize) -> T {
            self.segtree.get(p).prod
        }

        pub fn range_prod<R>(&mut self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).prod
        }

        pub fn all_prod(&self) -> T {
            self.segtree.all_prod().prod
        }

        pub fn apply_mult(&mut self, p: usize, x: T) {
            self.segtree.apply(p, x)
        }

        pub fn apply_range_mult<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, x)
        }

        /// 左端 `l` を固定し、区間 `[l, r)` での総積が述語 `g` を満たすような最大の `r` を返します。
        ///
        /// # 計算量
        /// O(log N)
        pub fn max_right<G>(&mut self, l: usize, g: G) -> usize
        where
            G: Fn(T) -> bool,
        {
            self.segtree.max_right(l, |x| g(x.prod))
        }

        /// 右端 `r` を固定し、区間 `[l, r)` での総積が述語 `g` を満たすような最小の `l` を返します。
        ///
        /// # 計算量
        /// O(log N)
        pub fn min_left<G>(&mut self, r: usize, g: G) -> usize
        where
            G: Fn(T) -> bool,
        {
            self.segtree.min_left(r, |x| g(x.prod))
        }

        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            self.len
        }
        pub fn to_vec(&mut self) -> Vec<T> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}

#[cfg(test)]
pub mod test_range_mult_range_prod {
    use ac_library::ModInt998244353;

    use super::range_mult_range_prod::RangeMultRangeProdSegtree;

    type Mint = ModInt998244353;

    #[test]
    fn test_new_and_get() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeMultRangeProdSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.get(0), 10);
        assert_eq!(segtree.get(2), 30);
        assert_eq!(segtree.get(4), 50);
    }

    #[test]
    fn test_set() {
        let xs = vec![10, 20, 30, 40, 50];
        let mut segtree = RangeMultRangeProdSegtree::<i64>::from_slice(&xs);
        segtree.set(0, 5);
        assert_eq!(segtree.to_vec(), vec![5, 20, 30, 40, 50]);
        segtree.set(4, 45);
        assert_eq!(segtree.to_vec(), vec![5, 20, 30, 40, 45]);
    }

    #[test]
    fn test_all_prod() {
        let xs = vec![1, 2, 3, 4, 5];
        let mut segtree = RangeMultRangeProdSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.all_prod(), 120);
        segtree.set(0, 5);
        assert_eq!(segtree.all_prod(), 600);
    }

    #[test]
    fn test_range_prod() {
        let xs = vec![1, 2, 3, 4, 5];
        let mut segtree = RangeMultRangeProdSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.range_prod(1..4), 24); // 2 * 3 * 4
        segtree.set(2, 10);
        assert_eq!(segtree.range_prod(1..4), 80); // 2 * 10 * 4
    }

    #[test]
    fn test_apply_mult() {
        let xs = vec![1, 2, 3, 4, 5];
        let mut segtree = RangeMultRangeProdSegtree::<i64>::from_slice(&xs);
        segtree.apply_mult(1, 5);
        assert_eq!(segtree.to_vec(), vec![1, 10, 3, 4, 5]); // 2 * 5
        segtree.apply_mult(1, 3);
        assert_eq!(segtree.to_vec(), vec![1, 30, 3, 4, 5]); // 10 * 3
    }

    #[test]
    fn test_apply_range_mult() {
        let xs = vec![1, 2, 3, 4, 5];
        let mut segtree = RangeMultRangeProdSegtree::<i64>::from_slice(&xs);
        segtree.apply_range_mult(1..4, 5);
        assert_eq!(segtree.to_vec(), vec![1, 10, 15, 20, 5]); // 2*5, 3*5, 4*5
        segtree.apply_range_mult(0..3, 2);
        assert_eq!(segtree.to_vec(), vec![2, 20, 30, 20, 5]); // 1*2, 10*2, 15*2
    }

    #[test]
    fn test_to_vec() {
        let xs = vec![0, 1, 2, 3, 4, 5];
        let mut segtree = RangeMultRangeProdSegtree::<i64>::from_slice(&xs);
        assert_eq!(segtree.to_vec(), vec![0, 1, 2, 3, 4, 5]);
        segtree.apply_range_mult(1..4, 10);
        assert_eq!(segtree.to_vec(), vec![0, 10, 20, 30, 4, 5]);
    }

    #[test]
    fn test_with_zeros() {
        let xs = vec![
            Mint::new(1),
            Mint::new(2),
            Mint::new(0),
            Mint::new(4),
            Mint::new(5),
        ];
        let mut segtree = RangeMultRangeProdSegtree::<Mint>::from_slice(&xs);

        // Initial state
        assert_eq!(segtree.all_prod(), Mint::new(0));
        assert_eq!(segtree.range_prod(0..2), Mint::new(2)); // 1 * 2
        assert_eq!(segtree.range_prod(0..3), Mint::new(0)); // 1 * 2 * 0
        assert_eq!(segtree.range_prod(2..5), Mint::new(0)); // 0 * 4 * 5

        // Multiply a non-zero range to include zero
        segtree.apply_range_mult(0..2, Mint::new(0));
        assert_eq!(
            segtree.to_vec(),
            vec![
                Mint::new(0),
                Mint::new(0),
                Mint::new(0),
                Mint::new(4),
                Mint::new(5)
            ]
        );
        assert_eq!(segtree.all_prod(), Mint::new(0));
        assert_eq!(segtree.range_prod(0..2), Mint::new(0));
        assert_eq!(segtree.range_prod(3..5), Mint::new(20));

        // Multiply a zero to a non-zero value
        segtree.apply_mult(2, Mint::new(3));
        assert_eq!(
            segtree.to_vec(),
            vec![
                Mint::new(0),
                Mint::new(0),
                Mint::new(0),
                Mint::new(4),
                Mint::new(5)
            ]
        ); // 0 * 3 = 0
        assert_eq!(segtree.all_prod(), Mint::new(0));
        assert_eq!(segtree.range_prod(2..5), Mint::new(0)); // 0 * 4 * 5

        // Multiply a range with zero to a non-zero value
        segtree.apply_range_mult(0..2, Mint::new(1));
        assert_eq!(
            segtree.to_vec(),
            vec![
                Mint::new(0),
                Mint::new(0),
                Mint::new(0),
                Mint::new(4),
                Mint::new(5)
            ]
        );
        assert_eq!(segtree.all_prod(), Mint::new(0));
    }

    #[test]
    fn test_modint() {
        let xs = vec![Mint::new(1), Mint::new(2), Mint::new(3)];
        let mut segtree = RangeMultRangeProdSegtree::<Mint>::from_slice(&xs);
        segtree.apply_range_mult(0..3, Mint::new(10));
        assert_eq!(
            segtree.to_vec(),
            vec![Mint::new(10), Mint::new(20), Mint::new(30)]
        );
        assert_eq!(segtree.range_prod(0..2), Mint::new(200)); // 10 * 20
    }

    #[test]
    fn test_max_right_min_left() {
        let xs = vec![2, 2, 2, 2, 2];
        let mut segtree = RangeMultRangeProdSegtree::<i64>::from_slice(&xs);
        // max_right: [0, r) で積が 8 以下の最大の r (2*2*2=8)
        assert_eq!(segtree.max_right(0, |p| p <= 8), 3);
        // min_left: [l, 5) で積が 4 以下の最小の l (2*2=4)
        assert_eq!(segtree.min_left(5, |p| p <= 4), 3);
    }

    #[ignore]
    #[test]
    fn test_random_update() {
        use rand::{Rng, SeedableRng, rngs::SmallRng};

        let mut rng = SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let n = rng.random_range(1..=20);
            let mut naive_vec: Vec<Mint> = (0..n)
                .map(|_| Mint::new(rng.random_range(1..=100)))
                .collect();
            let mut segtree = RangeMultRangeProdSegtree::<Mint>::from_slice(&naive_vec);

            for _ in 0..100 {
                let op_type = rng.random_range(0..6);

                match op_type {
                    0 => {
                        // set(p, x)
                        let p = rng.random_range(0..n);
                        let x = Mint::new(rng.random_range(1..=100));
                        naive_vec[p] = x;
                        segtree.set(p, x);
                    }
                    1 => {
                        // apply_range_mult(range, x)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);
                        let x = Mint::new(rng.random_range(1..=100));

                        for i in l..r {
                            naive_vec[i] *= x;
                        }
                        segtree.apply_range_mult(l..r, x);
                    }
                    2 => {
                        // get(p)
                        let p = rng.random_range(0..n);
                        assert_eq!(segtree.get(p), naive_vec[p], "get({}) failed", p);
                    }
                    3 => {
                        // range_prod(range)
                        let l = rng.random_range(0..=n);
                        let r = rng.random_range(l..=n);

                        let expected_prod: Mint = naive_vec[l..r].iter().copied().product();
                        assert_eq!(
                            segtree.range_prod(l..r),
                            expected_prod,
                            "range_prod({}..{}) failed",
                            l,
                            r
                        );
                    }
                    4 => {
                        // all_prod()
                        let expected_prod: Mint = naive_vec.iter().copied().product();
                        assert_eq!(segtree.all_prod(), expected_prod, "all_prod() failed");
                    }
                    5 => {
                        // apply_mult(p, x)
                        let p = rng.random_range(0..n);
                        let x = Mint::new(rng.random_range(1..=100));
                        naive_vec[p] *= x;
                        segtree.apply_mult(p, x);
                    }
                    _ => unreachable!(),
                }
            }
            assert_eq!(segtree.to_vec(), naive_vec, "final to_vec() check failed");
        }
    }
}
